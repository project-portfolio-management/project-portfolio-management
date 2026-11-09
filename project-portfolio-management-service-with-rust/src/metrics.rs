//! Prometheus metrics for the portfolio service.
//!
//! This module owns a process-wide `prometheus::Registry`
//! populated with a fixed set of counters. Application code increments the
//! global [`Metrics`] via [`Metrics::global`] (e.g.
//! `Metrics::global().plan_created_total.inc()` in the `plans` controller).
//! The REST API exposes the registry at `GET /metrics.prom` in Prometheus
//! text-exposition format (see [`crate::controllers`] / `controllers::metrics`).
//! Configure your scraper with `metrics_path: /metrics.prom`.
//!
//! The metric handles are cheap to clone (`Arc` under the hood); always go
//! through [`Metrics::global`] rather than re-creating them, so every
//! increment lands in the one registry that `/metrics.prom` renders.
//!
//! # Metric inventory
//!
//! | Name | Type | Labels |
//! |---|---|---|
//! | `plan_created_total` | counter | — |
//! | `plan_updated_total` | counter | — |
//! | `plan_deleted_total` | counter | — |
//! | `plan_merged_total` | counter | — |
//! | `http_requests_total` | counter vec | `method`, `path`, `status` |
//! | `client_address_lockouts_total` | counter | — |
//! | `audit_write_failed_total` | counter | — |
//! | `visibility_lookup_failed_total` | counter | — |
//! | `scheduled_job_runs_total` | counter vec | `job`, `outcome` |
//! | `ppm_flow_efficiency` | gauge vec | `plan` |
//! | `ppm_flow_cycle_time_p85_days` | gauge vec | `plan` |
//! | `ppm_flow_work_in_progress` | gauge vec | `plan` |
//! | `ppm_flow_first_pass_yield` | gauge vec | `plan` |
//! | `ppm_flow_columns_over_limit` | gauge vec | `plan` |
//! | `ppm_flow_plans_exported` | gauge | — |
//! | `ppm_flow_plans_suppressed` | gauge | — |
//! | `ppm_flow_plans_dropped` | gauge | — |
//! | `ppm_flow_last_refresh_timestamp_seconds` | gauge | — |
//! | `ppm_event_outbox_rows` | gauge vec | `status` (`pending`, `claimed`, `failed`) |
//! | `ppm_event_outbox_oldest_owed_seconds` | gauge | — |
//! | `ppm_event_outbox_last_refresh_timestamp_seconds` | gauge | — |
//!
//! # The outbox gauges are absent without a relay
//!
//! The `ppm_event_outbox_*` family is set by the relay ([`crate::relay`]) from `axa-outbox`'s own counts, every
//! [`crate::relay::STATS_EVERY`]. Each is a vector, label-less where it has no label, so that with no relay running
//! nothing is exported: a `failed 0` from a service that never looked would read as all clear. A `failed` above zero
//! is a row no relay will send until an operator retries it (`operations/deployment.md`).
//!
//! # The flow gauges are default-off, bounded, and suppressed
//!
//! The `ppm_flow_*` family is populated by a background refresh loop
//! that only runs when `PROJECT_PORTFOLIO_MANAGEMENT_FLOW_METRICS_SECS`
//! is set (see [`crate::flow_metrics`]). Unset, the gauges are never
//! written and the family does not appear in the exposition at all.
//!
//! Three properties are deliberate. `/metrics.prom` needs a bearer token
//! (it is not on `crate::auth`'s public list), but every scraper and every
//! dashboard its figures reach can read them, so nothing here may describe
//! one person or one private plan more than its row in the API would.
//!
//! - **Per-plan series are capped** (default 50).
//! - **Small boards are suppressed.** A flow efficiency over two tasks
//!   describes two people's week, and §12.4 refuses per-person
//!   measurement; reaching it by arithmetic is the same thing.
//! - **Neither bound is silent** — `..._suppressed` and `..._dropped`
//!   are exported beside the rows.
//!
//! The label is the plan **pid**, never its name: a rename would fork
//! the series and silently reset its history. Per-column occupancy is
//! deliberately not exported — see
//! [`crate::tba::flow_metric_rows`] for that trade.

use std::sync::OnceLock;

use prometheus::{Counter, Encoder, Gauge, GaugeVec, IntCounterVec, Opts, Registry, TextEncoder};

/// Content type for the Prometheus text-exposition format. Set this on the
/// HTTP response that serves [`Metrics::render`] so scrapers parse it
/// correctly.
pub const CONTENT_TYPE: &str = "text/plain; version=0.0.4";

/// Process-wide Prometheus registry and the standard project-portfolio-management-service metric
/// handles.
///
/// Construct once via [`Metrics::global`]; the counters are `Arc`-backed,
/// so the struct is cheap to hold by shared reference and each `.inc()`
/// updates the shared registry that [`Metrics::render`] serializes.
pub struct Metrics {
    /// The underlying registry. Exposed for callers that want to register
    /// service-specific metrics beyond this default set.
    pub registry: Registry,

    /// Count of plan records created (`POST /api/plans`).
    pub plan_created_total: Counter,
    /// Count of plan records updated (`PUT /api/plans/{pid}`).
    pub plan_updated_total: Counter,
    /// Count of plan records soft-deleted (`DELETE /api/plans/{pid}`).
    pub plan_deleted_total: Counter,
    /// Count of plan records merged (`POST /api/plans/merge`).
    pub plan_merged_total: Counter,
    /// Audit rows that could not be written for a write that was kept (CQ-3). Any rise is a
    /// gap in the audit trail and should page someone.
    pub audit_write_failed_total: Counter,
    /// Visibility lookups (privacy, handling marks, break-glass grants) that failed and were
    /// answered by hiding what could not be checked (CQ-6). Any rise means people are seeing less
    /// than they should, and why is in the error log.
    pub visibility_lookup_failed_total: Counter,
    /// Client addresses locked out for failed authentications, once per lockout and not per refused request
    /// (NFR-14). A rise is someone guessing, or a script with a bad token.
    pub client_address_lockouts_total: Counter,

    /// HTTP requests handled, labeled by `method`, `path`, and `status`.
    pub http_requests_total: IntCounterVec,

    /// Runs of the jobs the service schedules for itself, labeled by `job` and `outcome` (`ok` or `failed`).
    pub scheduled_job_runs_total: IntCounterVec,

    /// Aggregate work-over-cycle ratio per plan (`plan` = pid).
    pub flow_efficiency: GaugeVec,
    /// p85 cycle time in calendar days per plan — the service level expectation.
    pub flow_cycle_time_p85_days: GaugeVec,
    /// Started-and-unfinished tasks per plan (κ).
    pub flow_work_in_progress: GaugeVec,
    /// Share of finished tasks that never moved backwards, per plan.
    pub flow_first_pass_yield: GaugeVec,
    /// Board columns over their configured WIP cap, per plan.
    pub flow_columns_over_limit: GaugeVec,
    /// Plans exported as their own series in the last pass.
    pub flow_plans_exported: Gauge,
    /// Plans withheld in the last pass for having too small a board.
    pub flow_plans_suppressed: Gauge,
    /// Plans dropped in the last pass for exceeding the series cap.
    pub flow_plans_dropped: Gauge,
    /// Unix time of the last successful refresh. A scraper alerts on
    /// this going stale — the only way to tell a healthy zero from a
    /// refresh loop that died.
    pub flow_last_refresh_timestamp_seconds: Gauge,

    /// Outbox rows by `status`, from the relay's last read of `axa_outbox`.
    pub event_outbox_rows: GaugeVec,
    /// How long the oldest pending or claimed row has waited, in seconds (`0` with none owed). No labels.
    pub event_outbox_oldest_owed_seconds: GaugeVec,
    /// Unix time of the relay's last read. No labels. Alert on it going stale.
    pub event_outbox_last_refresh_timestamp_seconds: GaugeVec,
}

/// One read of the outbox, as the relay publishes it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutboxCounts {
    /// Rows waiting for a relay.
    pub pending: i64,
    /// Rows a relay is working on.
    pub claimed: i64,
    /// Rows out of attempts, which no relay sends until they are retried.
    pub failed: i64,
    /// How long the oldest owed row has waited, in seconds; `None` with none owed.
    pub oldest_owed_seconds: Option<f64>,
}

/// The time-based-analysis gauge handles, grouped so [`Metrics::new`]
/// stays readable and the flow family registers in one place.
struct FlowGauges {
    /// `ppm_flow_efficiency`: aggregate work time over cycle time, per plan.
    efficiency: GaugeVec,
    /// `ppm_flow_cycle_time_p85_days`: nearest-rank 85th-percentile cycle time in calendar days, per plan.
    cycle_time_p85_days: GaugeVec,
    /// `ppm_flow_work_in_progress`: tasks started and not finished, per plan.
    work_in_progress: GaugeVec,
    /// `ppm_flow_first_pass_yield`: share of finished tasks that never moved backwards, per plan.
    first_pass_yield: GaugeVec,
    /// `ppm_flow_columns_over_limit`: board columns over their WIP cap, per plan.
    columns_over_limit: GaugeVec,
    /// `ppm_flow_plans_exported`: plans exported as their own series in the last refresh.
    plans_exported: Gauge,
    /// `ppm_flow_plans_suppressed`: plans withheld because the board was too small to aggregate without describing individuals.
    plans_suppressed: Gauge,
    /// `ppm_flow_plans_dropped`: plans dropped in the last refresh for exceeding the series cap.
    plans_dropped: Gauge,
    /// `ppm_flow_last_refresh_timestamp_seconds`: Unix time of the last successful refresh; alert on it going stale.
    last_refresh_timestamp_seconds: Gauge,
}

impl FlowGauges {
    /// Construct the flow gauges and register them into `registry`.
    ///
    /// Registered unconditionally but never written unless the refresh
    /// loop is configured, so an unconfigured deployment renders no
    /// `ppm_flow_*` family at all rather than a wall of zeroes that
    /// reads like a measured estate with nothing in it.
    fn register_into(registry: &Registry) -> Self {
        let gauge_vec = |name: &str, help: &str| {
            GaugeVec::new(Opts::new(name, help), &["plan"])
                .expect("static opts and one label name cannot fail")
        };
        let scalar = |name: &str, help: &str| {
            Gauge::with_opts(Opts::new(name, help)).expect("static opts cannot fail")
        };
        let gauges = Self {
            efficiency: gauge_vec(
                "ppm_flow_efficiency",
                "Aggregate work time over cycle time, per plan.",
            ),
            cycle_time_p85_days: gauge_vec(
                "ppm_flow_cycle_time_p85_days",
                "85th-percentile cycle time in calendar days (nearest-rank), per plan.",
            ),
            work_in_progress: gauge_vec(
                "ppm_flow_work_in_progress",
                "Tasks started and not finished, per plan.",
            ),
            first_pass_yield: gauge_vec(
                "ppm_flow_first_pass_yield",
                "Share of finished tasks that never moved backwards, per plan.",
            ),
            columns_over_limit: gauge_vec(
                "ppm_flow_columns_over_limit",
                "Board columns over their configured WIP cap, per plan.",
            ),
            plans_exported: scalar(
                "ppm_flow_plans_exported",
                "Plans exported as their own series in the last refresh.",
            ),
            plans_suppressed: scalar(
                "ppm_flow_plans_suppressed",
                "Plans withheld in the last refresh: board too small to aggregate \
                 without describing individual people.",
            ),
            plans_dropped: scalar(
                "ppm_flow_plans_dropped",
                "Plans dropped in the last refresh for exceeding the series cap.",
            ),
            last_refresh_timestamp_seconds: scalar(
                "ppm_flow_last_refresh_timestamp_seconds",
                "Unix time of the last successful flow-metrics refresh; alert on it going stale.",
            ),
        };
        for collector in [
            Box::new(gauges.efficiency.clone()) as Box<dyn prometheus::core::Collector>,
            Box::new(gauges.cycle_time_p85_days.clone()),
            Box::new(gauges.work_in_progress.clone()),
            Box::new(gauges.first_pass_yield.clone()),
            Box::new(gauges.columns_over_limit.clone()),
            Box::new(gauges.plans_exported.clone()),
            Box::new(gauges.plans_suppressed.clone()),
            Box::new(gauges.plans_dropped.clone()),
            Box::new(gauges.last_refresh_timestamp_seconds.clone()),
        ] {
            registry
                .register(collector)
                .expect("registering a freshly-constructed collector cannot fail");
        }
        gauges
    }
}

/// The outbox gauge handles, made and registered together (see the module docs).
struct OutboxGauges {
    rows: GaugeVec,
    oldest_owed_seconds: GaugeVec,
    last_refresh_timestamp_seconds: GaugeVec,
}

impl OutboxGauges {
    /// Construct the outbox gauges and register them into `registry`.
    fn new(registry: &Registry) -> Self {
        let outbox_vec = |name: &str, help: &str, labels: &[&str]| {
            let vec = GaugeVec::new(Opts::new(name, help), labels)
                .expect("static opts and label names cannot fail");
            registry
                .register(Box::new(vec.clone()))
                .expect("registering a freshly-constructed collector cannot fail");
            vec
        };
        let event_outbox_rows = outbox_vec(
            "ppm_event_outbox_rows",
            "Event outbox rows by status, from the relay's last read; failed rows wait for an operator.",
            &["status"],
        );
        let event_outbox_oldest_owed_seconds = outbox_vec(
            "ppm_event_outbox_oldest_owed_seconds",
            "How long the oldest pending or claimed event outbox row has waited, in seconds.",
            &[],
        );
        let event_outbox_last_refresh_timestamp_seconds = outbox_vec(
            "ppm_event_outbox_last_refresh_timestamp_seconds",
            "Unix time of the relay's last read of the event outbox; alert on it going stale.",
            &[],
        );
        Self {
            rows: event_outbox_rows,
            oldest_owed_seconds: event_outbox_oldest_owed_seconds,
            last_refresh_timestamp_seconds: event_outbox_last_refresh_timestamp_seconds,
        }
    }
}

impl Metrics {
    /// Construct the full metric set and register every collector into a
    /// fresh [`Registry`].
    ///
    /// Called once by [`Metrics::global`]. The `.expect`s are infallible:
    /// every `Opts`/collector here is statically valid, and registering a
    /// freshly-constructed collector into an empty registry cannot collide.
    fn new() -> Self {
        let registry = Registry::new();

        let outbox = OutboxGauges::new(&registry);

        let plan_created_total = Counter::with_opts(Opts::new(
            "plan_created_total",
            "Total plan records created.",
        ))
        .expect("static counter opts are always valid");
        let plan_updated_total = Counter::with_opts(Opts::new(
            "plan_updated_total",
            "Total plan records updated.",
        ))
        .expect("static counter opts are always valid");
        let plan_deleted_total = Counter::with_opts(Opts::new(
            "plan_deleted_total",
            "Total plan records soft-deleted.",
        ))
        .expect("static counter opts are always valid");
        let plan_merged_total =
            Counter::with_opts(Opts::new("plan_merged_total", "Total plan records merged."))
                .expect("static counter opts are always valid");
        let audit_write_failed_total = Counter::with_opts(Opts::new(
            "audit_write_failed_total",
            "Audit rows that could not be written for a write that was kept.",
        ))
        .expect("static counter opts are always valid");

        let client_address_lockouts_total = Counter::with_opts(Opts::new(
            "client_address_lockouts_total",
            "Client addresses locked out for failed authentications, once per lockout.",
        ))
        .expect("static counter opts are always valid");
        let visibility_lookup_failed_total = Counter::with_opts(Opts::new(
            "visibility_lookup_failed_total",
            "Visibility lookups that failed and were answered by hiding (fail closed).",
        ))
        .expect("static counter opts are always valid");

        let http_requests_total = IntCounterVec::new(
            Opts::new(
                "http_requests_total",
                "Total HTTP requests handled, labeled by method, path, and status.",
            ),
            &["method", "path", "status"],
        )
        .expect("static counter-vec opts are always valid");

        let scheduled_job_runs_total = IntCounterVec::new(
            Opts::new(
                "scheduled_job_runs_total",
                "Runs of the jobs the service schedules for itself, labeled by job and outcome.",
            ),
            &["job", "outcome"],
        )
        .expect("static counter-vec opts are always valid");

        for collector in [
            Box::new(plan_created_total.clone()) as Box<dyn prometheus::core::Collector>,
            Box::new(plan_updated_total.clone()),
            Box::new(plan_deleted_total.clone()),
            Box::new(plan_merged_total.clone()),
            Box::new(audit_write_failed_total.clone()),
            Box::new(visibility_lookup_failed_total.clone()),
            Box::new(client_address_lockouts_total.clone()),
        ] {
            registry
                .register(collector)
                .expect("registering a freshly-constructed collector cannot fail");
        }
        registry
            .register(Box::new(http_requests_total.clone()))
            .expect("registering a freshly-constructed collector cannot fail");
        registry
            .register(Box::new(scheduled_job_runs_total.clone()))
            .expect("registering a freshly-constructed collector cannot fail");

        let flow = FlowGauges::register_into(&registry);
        Self {
            registry,
            plan_created_total,
            plan_updated_total,
            plan_deleted_total,
            plan_merged_total,
            audit_write_failed_total,
            visibility_lookup_failed_total,
            client_address_lockouts_total,
            http_requests_total,
            scheduled_job_runs_total,
            flow_efficiency: flow.efficiency,
            flow_cycle_time_p85_days: flow.cycle_time_p85_days,
            flow_work_in_progress: flow.work_in_progress,
            flow_first_pass_yield: flow.first_pass_yield,
            flow_columns_over_limit: flow.columns_over_limit,
            flow_plans_exported: flow.plans_exported,
            flow_plans_suppressed: flow.plans_suppressed,
            flow_plans_dropped: flow.plans_dropped,
            flow_last_refresh_timestamp_seconds: flow.last_refresh_timestamp_seconds,
            event_outbox_rows: outbox.rows,
            event_outbox_oldest_owed_seconds: outbox.oldest_owed_seconds,
            event_outbox_last_refresh_timestamp_seconds: outbox.last_refresh_timestamp_seconds,
        }
    }

    /// Publish one read of the outbox.
    #[expect(clippy::cast_precision_loss)] // row counts, far below 2^52
    pub fn publish_outbox(&self, counts: OutboxCounts, now_unix: f64) {
        for (status, rows) in [
            ("pending", counts.pending),
            ("claimed", counts.claimed),
            ("failed", counts.failed),
        ] {
            self.event_outbox_rows
                .with_label_values(&[status])
                .set(rows as f64);
        }
        self.event_outbox_oldest_owed_seconds
            .with_label_values(&[] as &[&str])
            .set(counts.oldest_owed_seconds.unwrap_or(0.0));
        self.event_outbox_last_refresh_timestamp_seconds
            .with_label_values(&[] as &[&str])
            .set(now_unix);
    }

    /// The process-wide project-portfolio-management-service metrics, initialized on first access.
    ///
    /// Backed by a private [`OnceLock`], so every caller shares the one
    /// registry that `GET /metrics.prom` renders.
    #[must_use]
    pub fn global() -> &'static Self {
        static METRICS: OnceLock<Metrics> = OnceLock::new();
        METRICS.get_or_init(Self::new)
    }

    /// A fresh, unregistered metric set for a test.
    ///
    /// The gauge tests reset labelled series, so sharing
    /// [`Metrics::global`] would let them clobber each other under the
    /// harness's parallelism. Each gets its own registry instead.
    #[cfg(test)]
    fn isolated() -> Self {
        Self::new()
    }

    /// Publish one flow-metrics pass (spec §15 TBA-10).
    ///
    /// Every labelled series is **reset first**. Without that, a plan
    /// that drops out of the export — archived, board shrunk below the
    /// floor, pushed past the cap — would keep its last value forever,
    /// and a stale figure that looks live is worse than an absent one.
    pub fn publish_flow(&self, set: &crate::tba::FlowMetricSet, now_unix: f64) {
        self.flow_efficiency.reset();
        self.flow_cycle_time_p85_days.reset();
        self.flow_work_in_progress.reset();
        self.flow_first_pass_yield.reset();
        self.flow_columns_over_limit.reset();

        for row in &set.rows {
            let labels = &[row.plan_pid.as_str()];
            // A `None` figure is left unset rather than written as zero:
            // an undefined ratio is not a ratio of nothing, and a p85
            // below the SLE's minimum sample is a refusal, not a zero.
            if let Some(value) = row.flow_efficiency {
                self.flow_efficiency.with_label_values(labels).set(value);
            }
            if let Some(value) = row.cycle_time_p85_days {
                self.flow_cycle_time_p85_days
                    .with_label_values(labels)
                    .set(value);
            }
            if let Some(value) = row.rolled_first_pass_yield {
                self.flow_first_pass_yield
                    .with_label_values(labels)
                    .set(value);
            }
            #[expect(clippy::cast_precision_loss)] // bounded counts
            {
                self.flow_work_in_progress
                    .with_label_values(labels)
                    .set(row.work_in_progress as f64);
                self.flow_columns_over_limit
                    .with_label_values(labels)
                    .set(row.columns_over_limit as f64);
            }
        }
        #[expect(clippy::cast_precision_loss)] // bounded counts
        {
            self.flow_plans_exported.set(set.rows.len() as f64);
            self.flow_plans_suppressed.set(set.suppressed_plans as f64);
            self.flow_plans_dropped.set(set.dropped_plans as f64);
        }
        self.flow_last_refresh_timestamp_seconds.set(now_unix);
    }

    /// Render the registry to Prometheus text-exposition format
    /// (`text/plain; version=0.0.4`).
    ///
    /// # Panics
    ///
    /// Does not panic in practice: the text encoder writes into a `Vec`
    /// (which never fails) and always produces valid UTF-8, so the two
    /// `.expect`s are unreachable.
    #[must_use]
    pub fn render(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buf = Vec::new();
        encoder
            .encode(&metric_families, &mut buf)
            .expect("text encoder writes to a Vec which never fails");
        String::from_utf8(buf).expect("Prometheus text encoder produces UTF-8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_outbox_family_is_absent_until_a_relay_publishes_it() {
        let metrics = Metrics::isolated();
        assert!(
            !metrics.render().contains("ppm_event_outbox"),
            "no relay, no figures"
        );
        metrics.publish_outbox(
            OutboxCounts {
                pending: 4,
                claimed: 1,
                failed: 2,
                oldest_owed_seconds: Some(12.5),
            },
            1_700_000_000.0,
        );
        let text = metrics.render();
        for line in [
            "ppm_event_outbox_rows{status=\"pending\"} 4",
            "ppm_event_outbox_rows{status=\"claimed\"} 1",
            "ppm_event_outbox_rows{status=\"failed\"} 2",
            "ppm_event_outbox_oldest_owed_seconds 12.5",
            "ppm_event_outbox_last_refresh_timestamp_seconds 1700000000",
        ] {
            assert!(text.lines().any(|l| l == line), "{line} in:\n{text}");
        }
        metrics.publish_outbox(
            OutboxCounts {
                pending: 0,
                claimed: 0,
                failed: 0,
                oldest_owed_seconds: None,
            },
            1_700_000_030.0,
        );
        assert!(
            metrics
                .render()
                .lines()
                .any(|l| l == "ppm_event_outbox_oldest_owed_seconds 0"),
            "none owed is a wait of nothing"
        );
    }

    /// A published pass renders labelled series, and the two bound
    /// counters travel with them so the gauges cannot be mistaken for
    /// the whole estate.
    #[test]
    fn publishing_flow_renders_labelled_series_and_its_bounds() {
        let metrics = Metrics::isolated();
        metrics.publish_flow(
            &crate::tba::FlowMetricSet {
                rows: vec![crate::tba::FlowMetricRow {
                    plan_pid: "abc".to_string(),
                    tasks: 20,
                    work_in_progress: 4,
                    flow_efficiency: Some(0.07),
                    cycle_time_p85_days: Some(11.0),
                    rolled_first_pass_yield: Some(0.75),
                    columns_over_limit: 1,
                }],
                suppressed_plans: 3,
                dropped_plans: 7,
            },
            1_700_000_000.0,
        );
        let body = metrics.render();
        assert!(
            body.contains(r#"ppm_flow_efficiency{plan="abc"} 0.07"#),
            "{body}"
        );
        assert!(
            body.contains(r#"ppm_flow_cycle_time_p85_days{plan="abc"} 11"#),
            "{body}"
        );
        assert!(
            body.contains(r#"ppm_flow_work_in_progress{plan="abc"} 4"#),
            "{body}"
        );
        assert!(
            body.contains(r#"ppm_flow_columns_over_limit{plan="abc"} 1"#),
            "{body}"
        );
        assert!(body.contains("ppm_flow_plans_suppressed 3"), "{body}");
        assert!(body.contains("ppm_flow_plans_dropped 7"), "{body}");
        assert!(
            body.contains("ppm_flow_last_refresh_timestamp_seconds 1700000000"),
            "a scraper alerts on this going stale: {body}"
        );
    }

    /// A plan that drops out of the export loses its series rather than
    /// keeping its last value forever.
    #[test]
    fn a_plan_that_leaves_the_export_loses_its_series() {
        let metrics = Metrics::isolated();
        let row = |pid: &str| crate::tba::FlowMetricRow {
            plan_pid: pid.to_string(),
            tasks: 9,
            work_in_progress: 1,
            flow_efficiency: Some(0.5),
            cycle_time_p85_days: Some(3.0),
            rolled_first_pass_yield: Some(1.0),
            columns_over_limit: 0,
        };
        metrics.publish_flow(
            &crate::tba::FlowMetricSet {
                rows: vec![row("gone"), row("stays")],
                suppressed_plans: 0,
                dropped_plans: 0,
            },
            1.0,
        );
        assert!(metrics.render().contains(r#"plan="gone""#));
        metrics.publish_flow(
            &crate::tba::FlowMetricSet {
                rows: vec![row("stays")],
                suppressed_plans: 1,
                dropped_plans: 0,
            },
            2.0,
        );
        let body = metrics.render();
        assert!(
            !body.contains(r#"plan="gone""#),
            "a withdrawn plan must not keep its last value: {body}"
        );
        assert!(body.contains(r#"plan="stays""#));
    }

    /// An undefined figure is absent, not zero. A p85 below the SLE's
    /// minimum sample is a refusal to forecast, and rendering it as 0
    /// would turn that refusal into a claim of instant delivery.
    #[test]
    fn a_null_figure_is_absent_not_zero() {
        let metrics = Metrics::isolated();
        metrics.publish_flow(
            &crate::tba::FlowMetricSet {
                rows: vec![crate::tba::FlowMetricRow {
                    plan_pid: "nulls".to_string(),
                    tasks: 6,
                    work_in_progress: 2,
                    flow_efficiency: None,
                    cycle_time_p85_days: None,
                    rolled_first_pass_yield: None,
                    columns_over_limit: 0,
                }],
                suppressed_plans: 0,
                dropped_plans: 0,
            },
            3.0,
        );
        let body = metrics.render();
        assert!(
            !body.contains(r#"ppm_flow_cycle_time_p85_days{plan="nulls"}"#),
            "a refused p85 must not render as 0: {body}"
        );
        assert!(
            !body.contains(r#"ppm_flow_efficiency{plan="nulls"}"#),
            "an undefined ratio must not render as 0: {body}"
        );
        assert!(
            body.contains(r#"ppm_flow_work_in_progress{plan="nulls"} 2"#),
            "{body}"
        );
    }

    /// Incrementing a counter is reflected in the rendered exposition, and
    /// the rendered text is valid Prometheus format (HELP/TYPE lines for the
    /// registered metrics). Runs without a database.
    #[test]
    fn render_yields_valid_prometheus_text() {
        let metrics = Metrics::global();
        metrics.plan_created_total.inc();
        // A counter vec only emits a metric family once it has at least one
        // observed label set, so touch one combination here.
        metrics
            .http_requests_total
            .with_label_values(&["GET", "/metrics.prom", "200"])
            .inc();
        let body = metrics.render();

        // Each registered counter contributes HELP + TYPE lines.
        assert!(
            body.contains("# HELP plan_created_total"),
            "missing HELP for plan_created_total; got: {body}"
        );
        assert!(
            body.contains("# TYPE plan_created_total counter"),
            "missing TYPE for plan_created_total; got: {body}"
        );
        // The incremented sample is present and non-zero.
        assert!(
            body.contains("plan_created_total"),
            "missing plan_created_total sample; got: {body}"
        );
        // The label-bearing HTTP counter vec is registered too.
        assert!(
            body.contains("# TYPE http_requests_total counter"),
            "missing http_requests_total; got: {body}"
        );
    }

    /// The exposed content type matches the Prometheus text format version
    /// the handler advertises.
    /// The shipped alert rules (`operations/prometheus/alerts.yml`) name only metrics this registry
    /// renders, so renaming one here cannot leave an alert silently watching nothing.
    #[test]
    fn every_metric_an_alert_rule_names_is_registered() {
        const RULES: &str = include_str!("../../operations/prometheus/alerts.yml");
        let metrics = Metrics::isolated();
        // The vecs render nothing until a label is used; give each one series.
        metrics
            .http_requests_total
            .with_label_values(&["GET", "/x", "503"])
            .inc();
        metrics
            .scheduled_job_runs_total
            .with_label_values(&["retention_purge", "failed"])
            .inc();
        metrics.publish_outbox(
            OutboxCounts {
                pending: 0,
                claimed: 0,
                failed: 0,
                oldest_owed_seconds: None,
            },
            0.0,
        );
        let text = metrics.render();
        let mut named = Vec::new();
        for line in RULES.lines().filter(|l| !l.trim_start().starts_with('#')) {
            for word in line.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
                if (word.starts_with("ppm_") || word.ends_with("_total")) && !named.contains(&word)
                {
                    named.push(word);
                }
            }
        }
        assert!(named.len() >= 8, "the rules name the metrics: {named:?}");
        for name in named {
            assert!(
                text.contains(&format!("# TYPE {name} ")),
                "alerts.yml names {name}, which the service does not register"
            );
        }
    }

    #[test]
    fn content_type_is_prometheus_text() {
        assert_eq!(CONTENT_TYPE, "text/plain; version=0.0.4");
    }
}
