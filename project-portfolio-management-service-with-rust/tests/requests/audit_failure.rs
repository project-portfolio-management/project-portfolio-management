//! CQ-3: an audit row that cannot be written for a kept write is logged and counted, never
//! dropped silently. The insert is made to fail by running it in a transaction PostgreSQL has
//! already aborted.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::metrics::Metrics;
use project_portfolio_management::models::audit_logs::Model as AuditModel;
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_failed_audit_write_is_counted_not_dropped() {
    request::<App, _, _>(|_request, ctx| async move {
        let failed = || Metrics::global().audit_write_failed_total.get();
        let before = failed();

        // A good write is not counted.
        AuditModel::record_or_log_on(
            project_portfolio_management::db::pool(&ctx),
            Uuid::new_v4(),
            "cq3_ok",
            None,
            None,
        )
        .await;
        assert!(
            (failed() - before).abs() < f64::EPSILON,
            "a written row is not a failure"
        );

        // An aborted transaction refuses every further statement, the audit insert included.
        let mut tx = project_portfolio_management::db::begin(&ctx)
            .await
            .expect("begin");
        assert!(sqlx::query("SELECT 1/0").execute(tx.conn()).await.is_err());
        AuditModel::record_or_log_on(tx.conn(), Uuid::new_v4(), "cq3_failed", None, None).await;
        drop(tx); // rolls back
        assert!(
            (failed() - before - 1.0).abs() < f64::EPSILON,
            "the failed audit write is counted once"
        );
    })
    .await;
}

/// NFR-6 / CQ-3: a write whose audit row cannot be written is refused, and nothing of it remains. The
/// audit insert is made to fail by a trigger that refuses one action, so the write itself is fine and only
/// its audit row is not. Announcements stand for the writes whose audit moved into their transaction.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_write_whose_audit_row_cannot_be_written_is_refused() {
    use serde_json::{Value, json};
    request::<App, _, _>(|request, ctx| async move {
        let failed = || Metrics::global().audit_write_failed_total.get();
        sqlx::raw_sql("CREATE OR REPLACE FUNCTION cq3_refuse_audit() RETURNS trigger AS $$
                 BEGIN RAISE EXCEPTION 'cq3: the audit table is unavailable'; END; $$ LANGUAGE plpgsql;
                 DROP TRIGGER IF EXISTS cq3_refuse ON audit_logs;
                 CREATE TRIGGER cq3_refuse BEFORE INSERT ON audit_logs FOR EACH ROW
                     WHEN (NEW.action = 'announcement_published') EXECUTE FUNCTION cq3_refuse_audit();").execute(project_portfolio_management::db::pool(&ctx))
            .await
            .unwrap();
        let title = format!("Audit refusal {}", Uuid::new_v4());
        let before = failed();
        let refused = request
            .post("/api/announcements")
            .json(&json!({ "title": title, "body": "this must not be kept" }))
            .await;
        let feed: Value = request.get("/api/announcements").await.json();
        let counted = failed() - before;
        // Put the table back before any assertion can stop the test.
        sqlx::raw_sql("DROP TRIGGER IF EXISTS cq3_refuse ON audit_logs; DROP FUNCTION IF EXISTS cq3_refuse_audit();").execute(project_portfolio_management::db::pool(&ctx))
            .await
            .unwrap();

        assert_eq!(refused.status_code(), 500, "{}", refused.text());
        assert_eq!(refused.json::<Value>()["code"], "audit_unavailable");
        let kept = feed["announcements"].as_array().unwrap().iter().any(|a| a["title"] == title.as_str());
        assert!(!kept, "the write was rolled back with its audit row");
        assert!((counted - 1.0).abs() < f64::EPSILON, "the refusal is counted once, got {counted}");

        // With the audit table back, the same request succeeds and is audited.
        let ok = request.post("/api/announcements").json(&json!({ "title": title, "body": "now it is kept" })).await;
        assert_eq!(ok.status_code(), 200, "{}", ok.text());
    })
    .await;
}
