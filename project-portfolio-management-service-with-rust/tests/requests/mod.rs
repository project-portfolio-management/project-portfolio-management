//! Request-level test suites, grouped by controller: the
//! `/api/plans` plan suite in [`plans`], the durable
//! outbox suite in [`event_outbox`], and the PPM Phase-A governance
//! suite (intake / gates / risks / budgets) in [`governance`].

mod access_review;
mod attachments;
mod audit_failure;
mod audit_protection;
mod blueprint_sync;
mod bulk;
mod canvas;
mod capabilities;
mod change_requests;
mod components;
mod cpm;
mod cpm_only;
mod custom_fields;
mod discussion;
mod effort;
mod email;
mod escalations;
mod event_outbox;
mod exceptions_digest;
mod extractors;
mod financials;
mod frameworks;
mod goals;
mod governance;
mod idempotency;
mod includes;
mod insights;
mod issues;
mod lessons;
mod live;
mod meeting_actions;
mod meetings;
mod metrics_control;
mod modules;
mod mspdi_batch;
mod mspdi_export;
mod oversight;
mod paging;
mod paste_import;
mod pdf;
mod plan_links;
mod plan_templates;
mod plans;
mod privacy_lookup_failure;
mod references;
mod resource_audit;
mod retention;
mod role_tasks;
mod scale;
mod scheduled_jobs;
mod scoring;
mod security_regression;
mod starter_templates;
mod status_evidence;
mod status_updates;
mod strategy;
mod subjects;
mod task_bulk;
mod task_order;
mod task_paging;
mod task_relations;
mod task_tree;
mod tba;
mod tolerances;
mod value_ceremony;
mod visibility;
mod workflow_phase;

use std::sync::Once;

/// Point the full-text index at a per-process temp directory, and switch
/// off the boot rebuild.
///
/// Must run before the first request that writes a record, because the
/// engine reads `PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_INDEX_PATH` once
/// into a `OnceLock` — shared across every test in this binary, not just
/// [`plans`]'s. Without it the suite would index into the crate's
/// working directory and accumulate documents across runs; the boot
/// rebuild would also race the test bodies' own writes.
pub(crate) fn isolate_search_index() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!(
            "project-portfolio-management-service-test-index-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        // `set_var` is `unsafe` in edition 2024; single-threaded setup.
        unsafe {
            std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_INDEX_PATH", &dir);
            std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_BOOT_REINDEX", "0");
        }
    });
}

/// Gives a plan a task workflow, as a plan template does. Registering one through the API needs an administrator, which a
/// test with no token cannot be; `tests/issue_configuration.rs` covers that route with real identities. `states` may leave
/// out `label`, `is_initial` and `is_terminal`.
pub async fn seed_task_workflow(
    ctx: &axa::AppContext,
    plan: &str,
    name: &str,
    states: serde_json::Value,
    transitions: serde_json::Value,
) {
    let states: Vec<serde_json::Value> = states
        .as_array()
        .expect("states")
        .iter()
        .map(|s| {
            serde_json::json!({
                "key": s["key"], "label": s.get("label").unwrap_or(&s["key"]), "category": s["category"],
                "is_initial": s.get("is_initial").unwrap_or(&serde_json::json!(false)),
                "is_terminal": s.get("is_terminal").unwrap_or(&serde_json::json!(false)),
            })
        })
        .collect();
    let def =
        serde_json::from_value(serde_json::json!({ "states": states, "transitions": transitions }))
            .expect("a workflow definition");
    let plan = uuid::Uuid::parse_str(plan).expect("plan pid");
    project_portfolio_management::controllers::workflow::register_own(ctx, plan, name, &def)
        .await
        .expect("seed a workflow");
}
