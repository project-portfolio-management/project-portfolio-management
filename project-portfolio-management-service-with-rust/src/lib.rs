//! `project-portfolio-management-service` — an Axum registry for **plan**
//! records in one recursive collection, with CRUD + matching.
//!
//! The API DTO is `project_portfolio_management_matcher::Plan` itself: the service stores
//! it verbatim (JSONB in the `plans` table) and matches with the
//! canonical [`project_portfolio_management_matcher`] engine, so there is no separate domain
//! model to drift. Every plan lives in one `plans` table; `kind` is an
//! optional descriptive label and matching is kind-agnostic (there is
//! no kind gate).
//!
//! ## Modules
//!
//! - [`app`] — boot, routes, workers and truncate.
//! - [`controllers`] — Axum controllers: CRUD, `match`, `check-duplicates`.
//! - [`models`] — the row structs and helpers over the stored payload.
//! - [`workers`] — background workers (an axa `BackgroundWorker`).
//! - [`tasks`], [`initializers`], [`data`] — extension points.
//! - [`cpm`], [`evm`], [`tz`] and the other pure rule modules — in the `project-portfolio-management-domain` crate
//!   (`domain/`), re-exported here under their old names (root spec §8.3c).
//!
//! See `spec/index.md` for the living specification.

// The hand-written OpenAPI document is one large nested `json!` tree.
#![recursion_limit = "512"]
#![forbid(unsafe_code)]
// Production code does not panic (spec/rust-checked-arithmetic, RCA-P-1 and RCA-NONE-1): an `Option` or a `Result` is handled, never unwrapped. Tests may.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented
    )
)]
#![deny(missing_docs)]

/// The pure domain modules, which live in the `project-portfolio-management-domain` crate (`domain/`), re-exported
/// under their old names so `crate::cpm`, `project_portfolio_management::cpm` and doc links keep working.
pub use project_portfolio_management_domain::{
    access_review, announcements, approvals, artifacts, automation, availability, baselines,
    benefit_validation, break_glass, buffer_management, buffers, bulk, calendars, canvas,
    capacity_heatmap, ccpm_estimates, change_requests, collaboration, components, controls,
    cost_rates, cost_risk, cpm, critical_chain, csv_cell, currency, custom_fields, cutover,
    deescalation, delivery_confidence, delivery_model, deployment_moves, dhcw_import,
    director_pack, director_pack_text, director_summary, discussion, distribution, documents, drum,
    earned_value, effort, engineering, escalations, evm, exceptions_digest, extract, feeds, fever,
    figures, financial_year, financials, frameworks, gate_decisions, governance, guests, hammock,
    inherit, insights, issues, joint_risk, kpi, ledger, leveling, lifecycle, link_token,
    meeting_actions, meetings, merge, milestone_signoff, milestones, modules, mspdi, mspdi_batch,
    my_work, notification_prefs, okr, onboarding, openapi_more, pdf, phase, plan_access,
    plan_links, plan_roles, planner, portfolio_resources, prioritization, privacy, processes,
    product_flow, public_roadmap, query_count, raci, register_completeness, report_schedules,
    sandbox, saved_views, schedule_health, schedule_risk, scheduling_rules, scoring_models, sinks,
    start_control, status_evidence, status_updates, strategy, subjects, task_relations,
    task_rollup, thresholds, timesheets, tolerances, tpc, tracker_import, tz, validation, value,
    viability, visibility, watermelon, work_profile, working_time,
};
pub mod app;
/// `axa::prelude`, with `axa::extract`'s `Json`, `Path` and `Query` in place of Axum's, so a bad body, path or query
/// answers a problem that says which and quotes nothing the caller sent (spec §8.3a). Handlers import this, not
/// `axa::prelude`.
pub mod prelude {
    pub use axa::prelude::*;
    // Named, so they shadow the prelude's own `Json`, `Path` and `Query`, which are Axum's.
    pub use axa::extract::{Json, Path, Query};

    #[cfg(test)]
    mod tests {
        /// Axum's extractors reject with text that can quote what the caller sent, and axa then sends that as a
        /// problem with no `detail`. This keeps them, and the prelude that exports them, out of `src`.
        #[test]
        fn handlers_take_axas_extractors_not_axums() {
            let forbidden = [
                "axa::prelude::*",
                "axum::extract::Path",
                "axum::extract::Query",
                "axum::extract::Json",
                ": axum::Json<",
            ];
            let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let mut stack = vec![manifest.join("src"), manifest.join("domain/src")];
            let mut offenders = Vec::new();
            while let Some(dir) = stack.pop() {
                for entry in std::fs::read_dir(&dir).expect("read src") {
                    let path = entry.expect("entry").path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.extension().is_some_and(|e| e == "rs")
                        && !path.ends_with("src/lib.rs")
                    {
                        let text = std::fs::read_to_string(&path).expect("read file");
                        for pattern in &forbidden {
                            if text.contains(pattern) {
                                offenders.push(format!("{}: {pattern}", path.display()));
                            }
                        }
                    }
                }
            }
            assert!(
                offenders.is_empty(),
                "use crate::prelude or axa::extract: {offenders:?}"
            );
        }
    }
}
pub mod asyncapi;
pub mod auth;
pub mod blueprint_sync;
pub mod cache;
/// Compliance controls: row-level integrity and keyed MACs.
pub mod compliance;
pub mod content_check;
pub mod controllers;
pub mod cursor;
pub mod data;
pub mod datetime;
pub mod db;
pub mod deployment_id;
pub mod dhcw_apply;
pub mod email;
/// The time-based-analysis flow-gauge refresh loop: default-off,
/// bounded by a series cap, and suppressed below a board-size floor.
pub mod flow_metrics;
pub mod handling_marks;
pub mod http_security;
pub mod idempotency;
pub mod initializers;
pub mod live;
pub mod metrics;
pub mod models;
/// Structured logging + real OpenTelemetry OTLP export (repo `tasks.md`
/// PRO-H12, slice 7 of 7 — the last). Installed via
/// `app::App::init_logger`; [`observability::trace_mw`] is layered in
/// `App::after_routes`.
pub mod observability;
pub mod openapi;
pub mod openapi_bodies;
pub mod openapi_schemas;
pub mod plan_templates;
/// Durable event bus Phase 3: the outbox relay loop + retention purge.
pub mod rate_limit;
pub mod relay;
pub mod response_cache;
pub mod retention;
pub mod scheduled_jobs;
/// The set-and-forget ticker: the optional scheduled-action sweep loop.
pub mod scheduler;
/// Tantivy full-text search: index schema, engine, and query surface.
pub mod search;
pub mod settings;
/// Point-in-time estate snapshots (board / CRO trends).
pub mod snapshots;
pub mod starter_templates;
pub mod streaming;
pub mod task_bulk;
pub mod tasks;
/// Time-based analysis (TBA): pure computation over the task
/// status-transition log — cycle versus lead time, flow efficiency,
/// rework and first-pass yield, the service level expectation,
/// constraint ranking, and queueing-theory flow.
pub mod tba;
pub mod trusted_proxy;
/// Header-based API versioning (`Accepts-version`) for the REST surface.
pub mod version;
/// Outbound webhook delivery — the `WebhookSink` family contract
/// (`agents/share/event-bus.md` §12, repo `tasks.md` EV-3).
pub mod webhooks;
pub mod workers;
/// Live worker-skill resolution over the worker service by
/// `EntityRef`, TTL-cached, never persisted (T-28c).
pub mod workers_client;
/// Custom workflows: configurable task and issue state vocabularies,
/// each state declaring the category every derived view computes from.
pub mod workflow;
/// The bridge to the workforce service (WPM): the pool link, its supply in and demand out (FB-1 to FB-3).
pub mod wpm_bridge;
