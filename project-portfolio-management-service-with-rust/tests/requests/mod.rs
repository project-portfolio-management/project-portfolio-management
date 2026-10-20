//! Request-level test suites, grouped by controller: the
//! `/api/plans` plan suite in [`plans`], the durable
//! outbox suite in [`event_outbox`], and the PPM Phase-A governance
//! suite (intake / gates / risks / budgets) in [`governance`].

mod access_review;
mod attachments;
mod audit_protection;
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
mod event_outbox;
mod financials;
mod goals;
mod governance;
mod includes;
mod insights;
mod issues;
mod live;
mod metrics_control;
mod modules;
mod mspdi_export;
mod oversight;
mod paging;
mod pdf;
mod plan_links;
mod plan_templates;
mod plans;
mod retention;
mod role_tasks;
mod scale;
mod strategy;
mod subjects;
mod task_bulk;
mod task_order;
mod task_paging;
mod task_relations;
mod task_tree;
mod tba;
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
