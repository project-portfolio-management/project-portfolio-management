//! The service, top to bottom: [`App::run`] reads the command line and the environment, connects what the service
//! needs, and starts it. Every step is an ordinary function call, so the whole path can be read here.

use axa::{
    AppContext, Environment, Error, Result,
    boot::{BootResult, TestApp},
    cli::{Cli, Command, DbCommand, parse_vars},
    config::{Config as ServiceConfig, Defaults, MB},
    health::Checks,
    queue::{Queue, QueueOptions},
    routes::AppRoutes,
    server,
    task::Tasks,
    worker::{BackgroundWorker, WorkerMode},
};
use axum::{
    Router as AxumRouter,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use project_portfolio_management_migration as migrations;
use std::sync::OnceLock;

#[expect(unused_imports)]
use crate::{
    auth, controllers,
    observability::{self, Telemetry, TelemetryConfig},
    tasks,
    workers::{bulk::BulkWorker, downloader::DownloadWorker},
};

/// The installed telemetry providers, so `on_shutdown` can flush what the
/// batch processor is still holding. Set once by [`App::init_logger`].
static TELEMETRY: OnceLock<Telemetry> = OnceLock::new();

/// Blanket auth-enforcement middleware: PASETO and/or Keycloak authentication then ABAC
/// authorization. Reads the enforcement posture (on unless explicitly off), the verifier,
/// and the ABAC policy per request (all cached `OnceLock`s) and delegates
/// to the pure [`auth::enforce_request`]: public paths and the disabled flag pass
/// through; otherwise a valid bearer token is required (`401`) and the
/// derived action is checked against the policy (`403`). On by default
/// (see `auth.rs`).
async fn require_auth_mw(req: Request, next: Next) -> Response {
    let decision = auth::enforce_request(
        auth::require_auth(),
        req.method(),
        req.uri().path(),
        req.headers(),
        // Per-request snapshot, so the policy watcher reaches the guard
        // too — not just the handlers.
        &auth::policy().current(),
    )
    .await;
    match decision {
        Ok(()) => next.run(req).await,
        // As a problem with the reason in `detail`: axa would send the plain text as a problem without it.
        Err(refusal) => crate::controllers::record_rejection(refusal).into_response(),
    }
}

/// A migration runner's failure as the service's: a statement's own error stays sqlx's, so a database that cannot
/// serve reads as one; the rest (a refused rollback, a database newer than this binary) keep their message.
fn migration_error(error: migrations::Error) -> Error {
    match error {
        migrations::Error::Sqlx(error) => Error::Sqlx(error),
        other => Error::Any(Box::new(other)),
    }
}

/// Port and database when nothing overrides them. A pool of one in development and tests, as a deadlock
/// from holding a connection across a second query then fails at once.
fn defaults() -> Defaults {
    Defaults {
        database: "project_portfolio_management",
        port: 1241,
        production_port: None,
        body_limit: 2 * MB,
        max_connections: 1,
        ..Defaults::default()
    }
}

/// Whether this environment was explicitly set up to run without login or rate limits: development, tests and the
/// insecure `container-dev` profile. **Any other name gets the secure defaults,** so a misspelt environment fails closed.
#[must_use]
pub fn opts_out_of_guards(environment: &Environment) -> bool {
    match environment {
        Environment::Development | Environment::Test => true,
        Environment::Other(name) => name == "container-dev",
        Environment::Production => false,
    }
}

/// The configuration, with the five things axa would do differently from this service's own middleware turned off: CORS,
/// HSTS, the content security policy and the permissions policy are `http_security`'s, and a request timeout would cut
/// the live event streams, so it needs asking for. axa's content security policy is production-only and runs first, so
/// left on it would silently replace ours and blank `/swagger-ui`, and no test on the `test` profile would see it. Its
/// permissions policy (new in axa 0.11, set by `HTTP_PERMISSIONS_POLICY`) would likewise replace ours, so one list, ours,
/// is the policy whatever the environment says.
fn service_config(environment: &Environment) -> Result<ServiceConfig> {
    renamed_settings(|name| std::env::var(name).ok()).map_err(Error::Message)?;
    let mut config = ServiceConfig::from_env(environment, &defaults())?;
    leave_headers_to_http_security(&mut config);
    if std::env::var_os("REQUEST_TIMEOUT_SECONDS").is_none() {
        config.http.timeout_seconds = 0;
    }
    // One list of trusted proxies, which axa has read as `TRUSTED_PROXIES`. The old name still works alone, and two
    // different lists refuse to start.
    if let Some(old) = crate::trusted_proxy::old_setting(|name| std::env::var(name).ok())
        .map_err(Error::Message)?
    {
        tracing::warn!(
            "{} is deprecated: set {} instead",
            crate::trusted_proxy::OLD_TRUSTED_PROXIES,
            crate::trusted_proxy::TRUSTED_PROXIES
        );
        config.http.trusted_proxies = crate::trusted_proxy::TrustedProxies::parse(&old)
            .map_or_else(
                |error| {
                    tracing::warn!(%error, "trusted proxy list ignored; trusting no proxy");
                    Vec::new()
                },
                crate::trusted_proxy::TrustedProxies::into_nets,
            );
    }
    Ok(config)
}

/// Turns off what axa would send that `http_security` sends itself (see [`service_config`]).
fn leave_headers_to_http_security(config: &mut ServiceConfig) {
    config.http.cors_origins.clear();
    config.http.hsts = false;
    config.http.csp = None;
    config.http.permissions_policy = None;
}

/// Settings axa 0.10 read under a name axa 0.11 ignores, each with the name that replaced it. Left alone, each falls back
/// without a word: to a default database in development, to the build's environment, to the default body limit or
/// header timeout. `HTTP_CSP` is not here, as this service never used axa's policy under either name.
pub const RENAMED_SETTINGS: [(&str, &str); 4] = [
    ("DATABASE_URL", "DB_URL"),
    ("LOCO_ENV", "APP_ENV"),
    ("BODY_LIMIT_BYTES", "HTTP_REQUEST_BODY_LIMIT_BYTE_COUNT"),
    (
        "HEADER_READ_TIMEOUT_SECONDS",
        "HTTP_REQUEST_HEADER_READ_TIMEOUT_SECONDS",
    ),
];

/// Refuses a boot, in every environment, where an old name in [`RENAMED_SETTINGS`] is set and its new name is not, so
/// that a deployment not yet renamed stops and says why rather than running on defaults.
///
/// # Errors
///
/// Naming each old setting that is set alone, with its new name.
pub fn renamed_settings(
    lookup: impl Fn(&str) -> Option<String>,
) -> std::result::Result<(), String> {
    let set = |name: &str| lookup(name).is_some_and(|value| !value.trim().is_empty());
    let stale: Vec<String> = RENAMED_SETTINGS
        .iter()
        .filter(|(old, new)| set(old) && !set(new))
        .map(|(old, new)| format!("{old} (set {new} instead)"))
        .collect();
    if stale.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "refusing to start: axa 0.11 no longer reads {}",
            stale.join(", ")
        ))
    }
}

/// The service: plain functions, called in order from [`App::run`].
pub struct App;

impl App {
    /// The version shown to operators: the crate version and the commit it was built from.
    #[must_use]
    pub fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    /// Connects the database, the cache, the mailer and, where jobs are queued, the queue.
    ///
    /// # Errors
    ///
    /// When any of those cannot be set up.
    pub async fn context(environment: Environment, config: ServiceConfig) -> Result<AppContext> {
        let db = axa::db::connect(&config.database).await?;
        let mut ctx = AppContext::new(environment, db, config);
        ctx.shared_store
            .insert(crate::cache::Cache::from_env(&ctx.environment)?);
        ctx.shared_store
            .insert(crate::email::Mailer::from_env(&ctx.environment)?);
        ctx.shared_store
            .insert(crate::cursor::Cursors::from_env(&ctx.environment)?);
        let cache = crate::cache::Cache::of(&ctx);
        ctx.health_checks = Checks::new().add("cache", move || {
            let cache = cache.clone();
            async move { cache.ping().await }
        });
        if ctx.config.workers == WorkerMode::BackgroundQueue {
            let mut queue =
                Queue::connect(&ctx.config.database.uri, QueueOptions::default()).await?;
            Self::connect_workers(&ctx, &mut queue);
            ctx.queue_provider = Some(std::sync::Arc::new(queue));
        }
        Ok(ctx)
    }

    /// The whole router: the routes under the standard middleware, then this service's own layers.
    ///
    /// # Errors
    ///
    /// When a route table is invalid or a startup check refuses.
    pub async fn router(ctx: &AppContext) -> Result<AxumRouter> {
        let router = Self::routes(ctx).into_app(ctx.clone())?;
        Self::after_routes(router, ctx).await
    }

    /// Runs one `db` subcommand. `axa::cli::parse` has already refused one that destroys data where it may not run.
    async fn db(ctx: &AppContext, command: DbCommand) -> Result<()> {
        match command {
            DbCommand::Migrate => migrations::up(crate::db::for_migrations(ctx))
                .await
                .map_err(migration_error)?,
            DbCommand::Down { steps } => migrations::down(crate::db::for_migrations(ctx), steps)
                .await
                .map_err(migration_error)?,
            DbCommand::Reset => migrations::fresh(crate::db::for_migrations(ctx))
                .await
                .map_err(migration_error)?,
            DbCommand::Status => {
                tracing::info!("Checking migration status");
                for migration in migrations::status(crate::db::for_migrations(ctx))
                    .await
                    .map_err(migration_error)?
                {
                    let status = if migration.applied {
                        "Applied"
                    } else {
                        "Pending"
                    };
                    tracing::info!("Migration '{}'... {status}", migration.name);
                }
            }
            DbCommand::Truncate => Self::truncate(ctx).await?,
        }
        Ok(())
    }

    /// The start of everything: one command, from the command line, against the environment's configuration.
    ///
    /// # Errors
    ///
    /// When the configuration is refused, or the command fails.
    pub async fn run(cli: Cli) -> Result<()> {
        let environment = cli
            .environment
            .as_deref()
            .map_or_else(Environment::from_env, Environment::parse);
        if matches!(cli.command, Command::Check) {
            renamed_settings(|name| std::env::var(name).ok()).map_err(Error::Message)?;
            axa::cli::check(&environment, &defaults())?;
            // axa's check knows nothing of the settings that are this service's own, and these two refuse a bad boot.
            crate::cache::Cache::from_env(&environment)?;
            crate::email::Mailer::from_env(&environment)?;
            return Ok(());
        }
        let mut config = service_config(&environment)?;
        if let Command::Start { binding, port, .. } = &cli.command {
            if let Some(binding) = binding {
                config.server.binding.clone_from(binding);
            }
            if let Some(port) = port {
                config.server.port = *port;
            }
        }
        let ctx = Self::context(environment, config).await?;
        if !Self::init_logger(&ctx)? {
            axa::logger::init(env!("CARGO_CRATE_NAME"), &ctx.config.logger);
        }
        if ctx.config.database.auto_migrate && !matches!(cli.command, Command::Db { .. }) {
            migrations::up(crate::db::for_migrations(&ctx))
                .await
                .map_err(migration_error)?;
        }
        match cli.command {
            Command::Check => Ok(()),
            Command::Start { worker, all, .. } => Self::start(&ctx, worker, all).await,
            Command::Db { command } => Self::db(&ctx, command).await,
            Command::Task { name, params } => {
                let mut tasks = Tasks::default();
                Self::register_tasks(&mut tasks);
                let Some(name) = name else {
                    for task in tasks.list() {
                        println!("{}\t{}", task.name, task.detail);
                    }
                    return Ok(());
                };
                tasks.run(&name, &ctx, &parse_vars(&params)?).await
            }
        }
    }

    /// Serves, works the queue, or both, until told to stop. Plain `start` only serves.
    async fn start(ctx: &AppContext, worker: bool, all: bool) -> Result<()> {
        let works = worker || all;
        axa::queue::warn_if_unserved(ctx, works);
        let router = if worker && !all {
            None
        } else {
            Some(Self::router(ctx).await?)
        };
        let queue = works.then(|| ctx.queue_provider.clone()).flatten();
        let ended = server::run(router, queue, &ctx.config.server).await;
        crate::relay::stop();
        Self::on_shutdown();
        ended
    }

    /// Install this crate's own logging + `OpenTelemetry` stack (repo
    /// `tasks.md` PRO-H12), returning `true` so the plain logger is
    /// **not** also installed. Ported from case-service's `App::init_logger`
    /// (itself care-pathway's, itself organization's, itself course's,
    /// itself person's) — the `Hooks::init_logger` seam is identical
    /// across crates' shapes despite the differing router layout (see
    /// `src/observability.rs`'s module docs and `after_routes` below for
    /// where this crate's shape actually diverges). Returns `false` when
    /// export is off, and [`App::run`] then installs the plain logger.
    ///
    /// # Errors
    ///
    /// Returns an error only if an OTLP exporter cannot be **built** (a
    /// malformed endpoint). A collector that is merely unreachable is not an
    /// error: the gRPC channel connects lazily, so boot is unaffected.
    pub fn init_logger(ctx: &AppContext) -> Result<bool> {
        let telemetry_config = TelemetryConfig::from_env();
        if !telemetry_config.export_enabled() {
            return Ok(false);
        }
        let logger_config = &ctx.config.logger;
        // An operator who set `RUST_LOG` or `logger.override_filter` owns
        // the filter outright; otherwise axa's module whitelist applies,
        // and the whitelist has no `opentelemetry*` entry — so a failing
        // export would be silent unless widened.
        let operator_supplied =
            std::env::var("RUST_LOG").is_ok() || logger_config.override_filter.is_some();
        let env_filter = observability::with_exporter_diagnostics(
            axa::logger::init_env_filter(
                env!("CARGO_CRATE_NAME"),
                logger_config.override_filter.as_ref(),
                &logger_config.level,
            ),
            operator_supplied,
        );
        // `logger.enable: false` means "no stdout layer" — honour it rather
        // than quietly turning logging on for anyone who enables export.
        let fmt_layer = logger_config
            .enable
            .then(|| axa::logger::init_layer(std::io::stdout, &logger_config.format, true));
        let telemetry = observability::init(&telemetry_config, env_filter, fmt_layer)
            .map_err(|error| axa::Error::Message(format!("OTLP init failed: {error}")))?;
        tracing::info!(
            service.name = %telemetry_config.service_name,
            endpoint = %telemetry_config.endpoint.as_deref().unwrap_or_default(),
            "OpenTelemetry OTLP export enabled"
        );
        let _ = TELEMETRY.set(telemetry);
        Ok(true)
    }

    /// Flush and tear down the OTLP providers on graceful shutdown, so the
    /// last batch of spans is not lost with the process.
    pub fn on_shutdown() {
        if let Some(telemetry) = TELEMETRY.get() {
            telemetry.shutdown();
        }
    }

    /// Every route table of the service, and the health routes.
    #[expect(clippy::too_many_lines)]
    #[must_use]
    pub fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::plans::routes())
            .add_route(controllers::compliance::routes())
            .add_route(controllers::governance::routes())
            .add_route(controllers::visibility::routes())
            .add_route(controllers::insights::routes())
            .add_route(controllers::oversight::routes())
            .add_route(controllers::tba::routes())
            .add_route(controllers::tpc::routes())
            .add_route(controllers::financials::routes())
            .add_route(controllers::controls::routes())
            .add_route(controllers::phase::routes())
            .add_route(controllers::distribution::routes())
            .add_route(controllers::workflow::routes())
            .add_route(controllers::okr::routes())
            .add_route(controllers::effort::routes())
            .add_route(controllers::ceremony::routes())
            .add_route(controllers::value::routes())
            .add_route(controllers::engineering::routes())
            .add_route(controllers::strategy::routes())
            .add_route(controllers::collaboration::routes())
            .add_route(controllers::automation::routes())
            .add_route(controllers::prioritization::routes())
            .add_route(controllers::scoring::routes())
            .add_route(controllers::lessons::routes())
            .add_route(controllers::status_updates::routes())
            .add_route(controllers::exceptions_digest::routes())
            .add_route(controllers::saved_views::routes())
            .add_route(controllers::canvas::routes())
            .add_route(controllers::modules::routes())
            .add_route(controllers::my_work::routes())
            .add_route(controllers::notification_prefs::routes())
            .add_route(controllers::discussion::routes())
            .add_route(controllers::docs::routes())
            .add_route(controllers::components::routes())
            .add_route(controllers::custom_fields::routes())
            .add_route(controllers::watchers::routes())
            .add_route(controllers::approvals::routes())
            .add_route(controllers::task_relations::routes())
            .add_route(controllers::task_tree::routes())
            .add_route(controllers::feeds::routes())
            .add_route(controllers::plan_access::routes())
            .add_route(controllers::task_bulk::routes())
            .add_route(controllers::timesheets::routes())
            .add_route(controllers::plan_templates::routes())
            .add_route(controllers::blueprint_sync::routes())
            .add_route(controllers::meetings::routes())
            .add_route(controllers::meeting_actions::routes())
            .add_route(controllers::mspdi_batch::routes())
            .add_route(controllers::live::routes())
            .add_route(controllers::planner::routes())
            .add_route(controllers::attachments::routes())
            .add_route(controllers::bulk::routes())
            .add_route(controllers::guests::routes())
            .add_route(controllers::goals::routes())
            .add_route(controllers::plan_links::routes())
            .add_route(controllers::frameworks::routes())
            .add_route(controllers::tolerances::routes())
            .add_route(controllers::escalations::routes())
            .add_route(controllers::role_tasks::routes())
            .add_route(controllers::pdf::routes())
            .add_route(controllers::sinks::routes())
            .add_route(controllers::report_schedules::routes())
            .add_route(controllers::announcements::routes())
            .add_route(controllers::issues::routes())
            .add_route(controllers::issue_configuration::routes())
            .add_route(controllers::plan_roles::routes())
            .add_route(controllers::subjects::routes())
            .add_route(controllers::access_review::routes())
            .add_route(controllers::break_glass::routes())
            .add_route(controllers::settings::routes())
            .add_route(controllers::handling_marks::routes())
            .add_route(controllers::change_requests::routes())
            .add_route(controllers::director_summary::routes())
            .add_route(controllers::confidence::routes())
            .add_route(controllers::programme_gates::routes())
            .add_route(controllers::funding::routes())
            .add_route(controllers::fever::routes())
            .add_route(controllers::pack_rhythm::routes())
            .add_route(controllers::capacity_heatmap::routes())
            .add_route(controllers::benefit_validation::routes())
            .add_route(controllers::director_packs::routes())
            .add_route(controllers::sandbox::routes())
            .add_route(controllers::onboarding::routes())
            .add_route(controllers::handover::routes())
            .add_route(controllers::hammock::routes())
            .add_route(controllers::controlled_documents::routes())
            .add_route(controllers::processes::routes())
            .add_route(controllers::task_get::routes())
            .add_route(controllers::assignments::routes())
            .add_route(controllers::cost_rates::routes())
            .add_route(controllers::cost_risk::routes())
            .add_route(controllers::cpm::routes())
            .add_route(controllers::kpis::routes())
            .add_route(controllers::critical_chain::routes())
            .add_route(controllers::ccpm_estimates::routes())
            .add_route(controllers::buffers::routes())
            .add_route(controllers::buffer_management::routes())
            .add_route(controllers::drum::routes())
            .add_route(controllers::organizations::routes())
            .add_route(controllers::roadmap_categories::routes())
            .add_route(controllers::public_roadmaps::routes())
            .add_route(controllers::ledger::routes())
            .add_route(controllers::deescalation::routes())
            .add_route(controllers::cutover::routes())
            .add_route(controllers::deployment_moves::routes())
            .add_route(controllers::start_control::routes())
            .add_route(controllers::email_deliveries::routes())
            .add_route(controllers::webhook_deliveries::routes())
            .add_route(controllers::thresholds::routes())
            .add_route(controllers::key_persons::routes())
            .add_route(controllers::register::routes())
            .add_route(controllers::programme_profile::routes())
            .add_route(controllers::people_impact::routes())
            .add_route(controllers::milestone_signoffs::routes())
            .add_route(controllers::leveling::routes())
            .add_route(controllers::milestones::routes())
            .add_route(controllers::portfolio_resources::routes())
            .add_route(controllers::schedule_risk::routes())
            .add_route(controllers::schedule_health::routes())
            .add_route(controllers::calendars::routes())
            .add_route(controllers::mspdi_export::routes())
            .add_route(controllers::mspdi_import::routes())
            .add_route(controllers::tracker_import::routes())
            .add_route(controllers::availability::routes())
            .add_route(controllers::schedule_baselines::routes())
            .add_route(controllers::raci::routes())
            .add_route(controllers::earned_value::routes())
            .add_route(controllers::metrics::routes())
    }

    /// The guards and background loops, in order: each layer wraps the ones added before it.
    ///
    /// # Errors
    ///
    /// When [`auth::check_startup`] refuses this posture.
    pub async fn after_routes(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Seed the process-wide PASETO verifier before the app serves
        // traffic: when `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` is set the published
        // key set is fetched over HTTP once at boot (fetch failure falls
        // back to the `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS` env path — the service
        // always boots).
        auth::init().await;
        // Secure by default: fix the enforcement posture from the environment and the
        // profile, and refuse to start when it is off in
        // production or on with no token verifier (see `auth::check_startup`).
        let settings = opts_out_of_guards(&ctx.environment)
            .then(|| serde_json::json!({ "require_auth": false }));
        auth::check_startup(!opts_out_of_guards(&ctx.environment), settings.as_ref())
            .map_err(Error::Message)?;
        // Key rotation and policy edits without a restart: both loops are
        // no-ops unless their source is configured.
        auth::spawn_key_refresh();
        auth::spawn_policy_watcher();
        // The zone that defines "today": load it now, re-read it every minute (settings.rs).
        controllers::settings::spawn(ctx.clone());
        // Time-based-analysis gauges: a no-op unless
        // `PROJECT_PORTFOLIO_MANAGEMENT_FLOW_METRICS_SECS` is set.
        crate::flow_metrics::spawn(ctx);
        // Durable event bus Phase 3: start the outbox relay loop. A no-op
        // unless `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox` AND `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY`
        // are set, so the default `memory` transport never spawns it.
        crate::relay::spawn(ctx);
        // Optional estate-snapshot ticker (env-gated, default off).
        crate::snapshots::spawn(ctx.clone());
        // Set-and-forget: the optional scheduled-action sweep ticker
        // (env-gated, default off — the sweep endpoint always works).
        crate::scheduler::spawn(ctx.clone());
        // Notification sinks: send due deliveries (env-gated; 0 turns it off).
        controllers::sinks::spawn(ctx);
        // Rebuild the full-text index if it is empty while records exist
        // — the upgrade path for a deployment whose data predates the
        // index, and the recovery path for an index directory that did
        // not survive a restart. A no-op on a normal boot.
        crate::tasks::search::spawn_reindex_if_empty(ctx.clone());
        // Blanket JWT enforcement layer. Added unconditionally; enforcement is on
        // unless explicitly turned off (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false`
        // or a development profile), and the layer is a near-noop then.
        // The key store's table is the migrations'; a missing one stops the boot, not the first write.
        let keys = crate::idempotency::keys(ctx);
        keys.check()
            .await
            .map_err(|error| Error::Message(format!("axa_idempotency: {error}")))?;
        let limits = std::sync::Arc::new(
            crate::rate_limit::RateLimiter::from_env(
                opts_out_of_guards(&ctx.environment).then_some(false),
                ctx.config.http.trusted_proxies.clone(),
            )
            .map_err(Error::Message)?,
        );
        Ok(router
            // Innermost, as it compares what the handler answered, before paging can take the `ETag` away.
            .layer(axum::middleware::from_fn(
                crate::http_security::not_modified_mw,
            ))
            // Every list pages the same way (RM-12b): innermost of all, so what it
            // slices is exactly what the handler answered.
            .layer(axum::middleware::from_fn_with_state(
                ctx.clone(),
                controllers::paging::paging_mw,
            ))
            // A retried write returns the first answer (BR-1). Inside the guards below, so a replay never
            // answers a caller they would refuse.
            .layer(axum::middleware::from_fn_with_state(
                keys,
                axa::idempotency::idempotent,
            ))
            .layer(axum::middleware::from_fn(crate::idempotency::caller_mw))
            // A read whose visibility lookup failed answers 503, not a short 200 (CQ-6).
            .layer(axum::middleware::from_fn(
                controllers::plan_access::lookup_fault_mw,
            ))
            // A successful write anywhere orphans every cached response (response_cache.rs).
            .layer(axum::middleware::from_fn_with_state(
                ctx.clone(),
                crate::response_cache::epoch_mw,
            ))
            // Runs after the auth guard and versioning, so an
            // unauthenticated caller never learns which modules a plan has.
            .layer(axum::middleware::from_fn_with_state(
                ctx.clone(),
                controllers::modules::modules_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                ctx.clone(),
                controllers::plan_access::privacy_mw,
            ))
            // Outermost of the plan guards: a guest is confined before anything
            // else looks at the request.
            .layer(axum::middleware::from_fn_with_state(
                ctx.clone(),
                controllers::guests::guest_mw,
            ))
            .layer(axum::middleware::from_fn(require_auth_mw))
            .layer(axum::middleware::from_fn(
                crate::version::require_version_mw,
            ))
            // Rate limiting sits outside the auth guard so failed logins are counted
            // and refused before the guard does any work (HD-5). From the outside in: the
            // client address is recorded, a locked-out address is refused, then the limits.
            .layer(axum::middleware::from_fn_with_state(
                limits.clone(),
                crate::rate_limit::rate_limit_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                limits.clone(),
                crate::rate_limit::lockout_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                limits,
                crate::rate_limit::client_address_mw,
            ))
            // CORS is an explicit allow-list (none by default) and the security headers
            // go on every response including the 401/403/429 above (HD-4).
            .layer(axum::middleware::from_fn_with_state(
                std::sync::Arc::new(crate::http_security::CorsConfig::from_env()),
                crate::http_security::cors_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                std::sync::Arc::new(crate::http_security::SecurityConfig::from_env(
                    crate::trusted_proxy::TrustedProxies::from_nets(
                        ctx.config.http.trusted_proxies.clone(),
                    ),
                )),
                crate::http_security::security_headers_mw,
            ))
            // Outermost layer runs first, so the request span wraps
            // versioning and the auth guard too — a 401/403 is part of the
            // trace, not invisible to it. This is this crate's only
            // router-construction surface — see `src/observability.rs`'s
            // module docs.
            .layer(axum::middleware::from_fn(observability::trace_mw)))
    }

    /// The kinds of job the queue runs.
    pub fn connect_workers(ctx: &AppContext, queue: &mut Queue) {
        queue.register(DownloadWorker::build(ctx));
        queue.register(BulkWorker::build(ctx));
    }

    /// The tasks `task <name>` can run.
    pub fn register_tasks(tasks: &mut Tasks) {
        tasks.register(crate::tasks::search::SearchReindex);
        tasks.register(crate::tasks::retention_purge::RetentionPurge);
        tasks.register(crate::tasks::exception_breaches::ExceptionBreaches);
        tasks.register(crate::tasks::tolerance_escalations::ToleranceEscalations);
        tasks.register(crate::tasks::exceptions_digest::ExceptionsDigest);
        tasks.register(crate::tasks::director_pack_schedule::DirectorPackSchedule);
        tasks.register(crate::tasks::outbox_retry_failed::OutboxRetryFailed);
        // tasks-inject (do not remove)
    }

    /// Empties the tables the service owns, for `db truncate`.
    ///
    /// # Errors
    ///
    /// Propagates a database error.
    pub async fn truncate(ctx: &AppContext) -> Result<()> {
        // `DELETE FROM` each, in this order, each on its own, as before SQ-1. The names are fixed literals.
        for table in [
            "notifications",
            "automation_runs",
            "automations",
            "scheduled_actions",
            "reviews",
            "benefits",
            "objective_links",
            "objectives",
            "scenarios",
            "ideas",
            "report_definitions",
            "allocations",
            "milestones",
            "plan_dependencies",
            "budget_lines",
            "risks",
            "gate_reviews",
            "proposals",
        ] {
            Self::delete_all(ctx, table).await?;
        }
        sqlx::raw_sql("DELETE FROM axa_outbox")
            .execute(crate::db::pool(ctx))
            .await?;
        // The outbox's record refuses DELETE and TRUNCATE by design, so emptying it means making it again.
        sqlx::raw_sql(
            "DROP TABLE IF EXISTS axa_outbox_attempts, axa_outbox_owed, axa_outbox_leases, \
             axa_outbox_payloads, axa_outbox_events CASCADE",
        )
        .execute(crate::db::pool(ctx))
        .await?;
        for statement in axa::outbox::SETUP_STATEMENTS {
            sqlx::raw_sql(statement)
                .execute(crate::db::pool(ctx))
                .await?;
        }
        for table in ["merge_records", "audit_logs", "plans"] {
            Self::delete_all(ctx, table).await?;
        }
        Ok(())
    }

    /// `DELETE FROM <table>`, for [`Self::truncate`]; `table` is one of its fixed names.
    async fn delete_all(ctx: &AppContext, table: &'static str) -> Result<()> {
        sqlx::query(sqlx::AssertSqlSafe(format!("DELETE FROM \"{table}\"")))
            .execute(crate::db::pool(ctx))
            .await?;
        Ok(())
    }
}

impl TestApp for App {
    /// A fresh schema and the full router, as `DB reset` then `start` would give.
    async fn boot_test() -> Result<BootResult> {
        let environment = Environment::Test;
        let config = service_config(&environment)?;
        let ctx = Self::context(environment, config).await?;
        migrations::fresh(crate::db::for_migrations(&ctx))
            .await
            .map_err(migration_error)?;
        let router = Self::router(&ctx).await?;
        Ok(BootResult {
            app_context: ctx,
            router: Some(router),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> Environment {
        Environment::Other(name.to_string())
    }

    /// axa's parser is what refuses `db reset` in production, as this service no longer does: a parser of its own,
    /// written with clap, would compile just as well and refuse nothing.
    /// Each task has been asked whether it may run in production, and these all may: none loads fixtures, and each is
    /// meant to run on a schedule against real data. A new task fails this until someone decides, and one that loads
    /// fixtures says `development_only`, which `task` then refuses in production as `db reset` is refused.
    #[test]
    fn every_task_has_been_decided_on_for_production() {
        use axa::task::Task;
        let decided: [(&str, &dyn Task); 7] = [
            (
                "director_pack_schedule",
                &crate::tasks::director_pack_schedule::DirectorPackSchedule,
            ),
            (
                "exception_breaches",
                &crate::tasks::exception_breaches::ExceptionBreaches,
            ),
            (
                "exceptions_digest",
                &crate::tasks::exceptions_digest::ExceptionsDigest,
            ),
            (
                "outbox_retry_failed",
                &crate::tasks::outbox_retry_failed::OutboxRetryFailed,
            ),
            (
                "retention_purge",
                &crate::tasks::retention_purge::RetentionPurge,
            ),
            ("search_reindex", &crate::tasks::search::SearchReindex),
            (
                "tolerance_escalations",
                &crate::tasks::tolerance_escalations::ToleranceEscalations,
            ),
        ];
        let mut tasks = Tasks::default();
        App::register_tasks(&mut tasks);
        let mut registered: Vec<String> = tasks.list().into_iter().map(|t| t.name).collect();
        registered.sort();
        assert_eq!(
            registered,
            decided.map(|(name, _)| name.to_string()),
            "a task was added or removed: decide whether it is development_only, and list it here"
        );
        for (name, task) in decided {
            assert_eq!(task.task().name, name);
            assert!(!task.development_only(), "{name} is meant for production");
        }
    }

    /// `http_security` owns CORS, HSTS, the content security policy and the permissions policy, so axa's are off, and
    /// axa 0.11's new permissions policy cannot replace this service's own list from `HTTP_PERMISSIONS_POLICY`.
    #[test]
    fn axa_sends_none_of_the_headers_this_service_sets_itself() {
        let mut config = ServiceConfig::from_env(&Environment::Test, &defaults()).unwrap();
        assert!(
            config.http.permissions_policy.is_some(),
            "axa 0.11 sends one unless told not to"
        );
        leave_headers_to_http_security(&mut config);
        assert!(config.http.cors_origins.is_empty());
        assert!(!config.http.hsts);
        assert_eq!(config.http.csp, None);
        assert_eq!(config.http.permissions_policy, None);
    }

    /// An old name set alone refuses the boot, naming the new one; the new name, or both, boot; a blank old name is unset.
    #[test]
    fn a_setting_axa_no_longer_reads_refuses_the_boot() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |name: &str| {
                pairs
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, v)| (*v).to_string())
            }
        };
        assert_eq!(renamed_settings(env(&[])), Ok(()));
        assert_eq!(renamed_settings(env(&[("DB_URL", "postgres://x")])), Ok(()));
        assert_eq!(
            renamed_settings(env(&[
                ("DATABASE_URL", "postgres://x"),
                ("DB_URL", "postgres://x")
            ])),
            Ok(())
        );
        assert_eq!(renamed_settings(env(&[("DATABASE_URL", " ")])), Ok(()));
        for (old, new) in RENAMED_SETTINGS {
            let lookup = move |name: &str| (name == old).then(|| "1".to_string());
            let refusal = renamed_settings(lookup).unwrap_err();
            assert!(refusal.contains(old) && refusal.contains(new), "{refusal}");
        }
        let refusal = renamed_settings(env(&[
            ("DATABASE_URL", "postgres://x"),
            ("LOCO_ENV", "production"),
        ]))
        .unwrap_err();
        assert!(
            refusal.contains("DB_URL") && refusal.contains("APP_ENV"),
            "{refusal}"
        );
    }

    #[test]
    fn the_binary_reads_its_arguments_through_the_parser_that_refuses_data_loss() {
        let main = include_str!("bin/main.rs");
        assert!(main.contains("axa::cli::parse("));
    }

    #[test]
    fn only_the_named_profiles_may_run_without_login() {
        for environment in [Environment::Production, named("prod-eu"), named("staging")] {
            assert!(!opts_out_of_guards(&environment), "{environment}");
            assert!(
                auth::startup_verdict(!opts_out_of_guards(&environment), false, true, false)
                    .is_err()
            );
        }
        for environment in [
            Environment::Development,
            Environment::Test,
            named("container-dev"),
        ] {
            assert!(opts_out_of_guards(&environment), "{environment}");
            assert!(
                auth::startup_verdict(!opts_out_of_guards(&environment), false, true, false)
                    .is_ok()
            );
        }
    }
}
