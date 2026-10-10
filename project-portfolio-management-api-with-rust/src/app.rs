//! loco.rs application wiring: the [`App`] `Hooks` implementation that
//! registers routes, background workers, and the truncate/seed
//! lifecycle for `project-portfolio-management-service`.

use async_trait::async_trait;
use axum::{
    Router as AxumRouter,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    db::truncate_table,
    environment::Environment,
    task::Tasks,
};
use migration::Migrator;
use std::path::Path;
use std::sync::OnceLock;

#[allow(unused_imports)]
use crate::{
    auth, controllers,
    models::_entities::{
        allocations, audit_logs, automation_runs, automations, benefits, budget_lines,
        event_outbox, gate_reviews, ideas, merge_records, milestones, notifications,
        objective_links, objectives, plan_dependencies, plans, proposals, report_definitions,
        reviews, risks, scenarios, scheduled_actions,
    },
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
        Err((status, msg)) => (status, msg).into_response(),
    }
}

/// The loco.rs application hooks for `project-portfolio-management-service`.
///
/// Implements [`Hooks`] to register routes, background workers, and the
/// truncate/seed lifecycle; the binary entrypoint drives it via the
/// loco CLI.
pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    /// Install this crate's own logging + `OpenTelemetry` stack (repo
    /// `tasks.md` PRO-H12), returning `true` so loco does **not** also
    /// install its own. Ported from case-service's `App::init_logger`
    /// (itself care-pathway's, itself organization's, itself course's,
    /// itself person's) — the `Hooks::init_logger` seam is identical
    /// across crates' shapes despite the differing router layout (see
    /// `src/observability.rs`'s module docs and `after_routes` below for
    /// where this crate's shape actually diverges).
    ///
    /// # Errors
    ///
    /// Returns an error only if an OTLP exporter cannot be **built** (a
    /// malformed endpoint). A collector that is merely unreachable is not an
    /// error: the gRPC channel connects lazily, so boot is unaffected.
    fn init_logger(ctx: &AppContext) -> Result<bool> {
        let telemetry_config = TelemetryConfig::from_env();
        if !telemetry_config.export_enabled() {
            return Ok(false);
        }
        let logger_config = &ctx.config.logger;
        // An operator who set `RUST_LOG` or `logger.override_filter` owns
        // the filter outright; otherwise loco's module whitelist applies,
        // and the whitelist has no `opentelemetry*` entry — so a failing
        // export would be silent unless widened.
        let operator_supplied =
            std::env::var("RUST_LOG").is_ok() || logger_config.override_filter.is_some();
        let env_filter = observability::with_exporter_diagnostics(
            loco_rs::logger::init_env_filter::<Self>(
                logger_config.override_filter.as_ref(),
                &logger_config.level,
            ),
            operator_supplied,
        );
        // `logger.enable: false` means "no stdout layer" — honour it rather
        // than quietly turning logging on for anyone who enables export.
        let fmt_layer = logger_config
            .enable
            .then(|| loco_rs::logger::init_layer(std::io::stdout, &logger_config.format, true));
        let telemetry = observability::init(&telemetry_config, env_filter, fmt_layer)
            .map_err(|error| loco_rs::Error::Message(format!("OTLP init failed: {error}")))?;
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
    async fn on_shutdown(_ctx: &AppContext) {
        if let Some(telemetry) = TELEMETRY.get() {
            telemetry.shutdown();
        }
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
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
            .add_route(controllers::live::routes())
            .add_route(controllers::planner::routes())
            .add_route(controllers::attachments::routes())
            .add_route(controllers::bulk::routes())
            .add_route(controllers::guests::routes())
            .add_route(controllers::plan_links::routes())
            .add_route(controllers::pdf::routes())
            .add_route(controllers::sinks::routes())
            .add_route(controllers::report_schedules::routes())
            .add_route(controllers::announcements::routes())
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
            .add_route(controllers::leveling::routes())
            .add_route(controllers::milestones::routes())
            .add_route(controllers::portfolio_resources::routes())
            .add_route(controllers::schedule_risk::routes())
            .add_route(controllers::schedule_health::routes())
            .add_route(controllers::calendars::routes())
            .add_route(controllers::mspdi_import::routes())
            .add_route(controllers::tracker_import::routes())
            .add_route(controllers::availability::routes())
            .add_route(controllers::schedule_baselines::routes())
            .add_route(controllers::raci::routes())
            .add_route(controllers::earned_value::routes())
            .add_route(controllers::metrics::routes())
    }

    async fn after_routes(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Seed the process-wide PASETO verifier before the app serves
        // traffic: when `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` is set the published
        // key set is fetched over HTTP once at boot (fetch failure falls
        // back to the `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS` env path — the service
        // always boots).
        auth::init().await;
        // Secure by default: fix the enforcement posture from the environment and the
        // config's `settings.require_auth`, and refuse to start when it is off in
        // production or on with no token verifier (see `auth::check_startup`).
        auth::check_startup(
            ctx.environment == Environment::Production,
            ctx.config.settings.as_ref(),
        )
        .map_err(loco_rs::Error::Message)?;
        // Key rotation and policy edits without a restart: both loops are
        // no-ops unless their source is configured.
        auth::spawn_key_refresh();
        auth::spawn_policy_watcher();
        // Time-based-analysis gauges: a no-op unless
        // `PROJECT_PORTFOLIO_MANAGEMENT_FLOW_METRICS_SECS` is set.
        crate::flow_metrics::spawn(ctx);
        // Durable event bus Phase 3: start the outbox relay loop. A no-op
        // unless `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox` AND `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY`
        // are set, so the default `memory` transport never spawns it.
        crate::relay::spawn(ctx.db.clone());
        // Optional estate-snapshot ticker (env-gated, default off).
        crate::snapshots::spawn(ctx.db.clone());
        // Set-and-forget: the optional scheduled-action sweep ticker
        // (env-gated, default off — the sweep endpoint always works).
        crate::scheduler::spawn(ctx.clone());
        // Notification sinks: send due deliveries (env-gated; 0 turns it off).
        controllers::sinks::spawn(ctx);
        // Rebuild the full-text index if it is empty while records exist
        // — the upgrade path for a deployment whose data predates the
        // index, and the recovery path for an index directory that did
        // not survive a restart. A no-op on a normal boot.
        crate::tasks::search::spawn_reindex_if_empty(ctx.db.clone());
        // Blanket JWT enforcement layer. Added unconditionally; enforcement is on
        // unless explicitly turned off (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false`
        // or `settings.require_auth: false`), and the layer is a near-noop then.
        Ok(router
            // Every list pages the same way (RM-12b): innermost of all, so what it
            // slices is exactly what the handler answered.
            .layer(axum::middleware::from_fn(controllers::paging::paging_mw))
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
            // and refused before the guard does any work (HD-5).
            .layer(axum::middleware::from_fn_with_state(
                std::sync::Arc::new(crate::rate_limit::RateLimiter::from_env(
                    ctx.config
                        .settings
                        .as_ref()
                        .and_then(|s| s.get("rate_limit"))
                        .and_then(serde_json::Value::as_bool),
                )),
                crate::rate_limit::rate_limit_mw,
            ))
            // CORS is an explicit allow-list (none by default) and the security headers
            // go on every response including the 401/403/429 above (HD-4).
            .layer(axum::middleware::from_fn_with_state(
                std::sync::Arc::new(crate::http_security::CorsConfig::from_env()),
                crate::http_security::cors_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                std::sync::Arc::new(crate::http_security::SecurityConfig::from_env()),
                crate::http_security::security_headers_mw,
            ))
            // Outermost layer runs first, so the request span wraps
            // versioning and the auth guard too — a 401/403 is part of the
            // trace, not invisible to it. This is this crate's only
            // router-construction surface — see `src/observability.rs`'s
            // module docs.
            .layer(axum::middleware::from_fn(observability::trace_mw)))
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        queue.register(BulkWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(crate::tasks::search::SearchReindex);
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, notifications::Entity).await?;
        truncate_table(&ctx.db, automation_runs::Entity).await?;
        truncate_table(&ctx.db, automations::Entity).await?;
        truncate_table(&ctx.db, scheduled_actions::Entity).await?;
        truncate_table(&ctx.db, reviews::Entity).await?;
        truncate_table(&ctx.db, benefits::Entity).await?;
        truncate_table(&ctx.db, objective_links::Entity).await?;
        truncate_table(&ctx.db, objectives::Entity).await?;
        truncate_table(&ctx.db, scenarios::Entity).await?;
        truncate_table(&ctx.db, ideas::Entity).await?;
        truncate_table(&ctx.db, report_definitions::Entity).await?;
        truncate_table(&ctx.db, allocations::Entity).await?;
        truncate_table(&ctx.db, milestones::Entity).await?;
        truncate_table(&ctx.db, plan_dependencies::Entity).await?;
        truncate_table(&ctx.db, budget_lines::Entity).await?;
        truncate_table(&ctx.db, risks::Entity).await?;
        truncate_table(&ctx.db, gate_reviews::Entity).await?;
        truncate_table(&ctx.db, proposals::Entity).await?;
        truncate_table(&ctx.db, event_outbox::Entity).await?;
        truncate_table(&ctx.db, merge_records::Entity).await?;
        truncate_table(&ctx.db, audit_logs::Entity).await?;
        truncate_table(&ctx.db, plans::Entity).await?;
        Ok(())
    }
    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
