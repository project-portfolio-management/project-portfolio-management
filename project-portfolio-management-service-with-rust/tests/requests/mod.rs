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
mod roadmap_lists;
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

/// The next page's URL, from `Link: <url>; rel="next"`; `None` on the last page.
pub(crate) fn next_page(r: &axum_test::TestResponse) -> Option<String> {
    let link = r.maybe_header("link")?;
    let link = link.to_str().ok()?;
    link.strip_prefix('<')?
        .split_once(">; rel=\"next\"")
        .map(|(url, _)| url.to_string())
}

/// Every row of a list from `url` on, following its next links, and how many pages that took. Each page must be `200`.
pub(crate) async fn walk(
    request: &axum_test::TestServer,
    url: &str,
) -> (Vec<serde_json::Value>, usize) {
    let (mut rows, mut pages, mut at) = (Vec::new(), 0, Some(url.to_string()));
    while let Some(url) = at {
        let r = request.get(&url).await;
        assert_eq!(r.status_code(), 200, "{url}: {}", r.text());
        assert!(
            r.maybe_header("x-total-count").is_none(),
            "{url}: a list sends no total"
        );
        rows.extend(
            r.json::<serde_json::Value>()
                .as_array()
                .expect("array")
                .clone(),
        );
        pages += 1;
        at = next_page(&r);
        assert!(pages <= 100, "{url}: more than 100 pages");
    }
    (rows, pages)
}

/// A list at `url` pages by cursor (FR-46): two at a time it walks the same `expected` rows, in the same order, as one
/// default page, with no total; a limit over the ceiling, a made-up cursor and an offset are refused.
pub(crate) async fn assert_pages(request: &axum_test::TestServer, url: &str, expected: usize) {
    let sep = if url.contains('?') { '&' } else { '?' };
    let (whole, one) = walk(request, url).await;
    assert_eq!((whole.len(), one), (expected, 1), "{url}: one default page");
    let (in_twos, page_count) = walk(request, &format!("{url}{sep}limit=2")).await;
    assert_eq!(
        in_twos, whole,
        "{url}: the pages are the list, once each, in order"
    );
    assert_eq!(
        page_count,
        expected.div_ceil(2).max(1),
        "{url}: pages of two"
    );
    for (query, status) in [
        ("limit=100000", 422),
        ("cursor=made-up", 422),
        ("offset=1", 400),
    ] {
        let r = request.get(&format!("{url}{sep}{query}")).await;
        assert_eq!(r.status_code(), status, "{url} {query}: {}", r.text());
    }
}
