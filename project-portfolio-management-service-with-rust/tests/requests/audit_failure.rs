//! CQ-3: an audit row that cannot be written for a kept write is logged and counted, never
//! dropped silently. The insert is made to fail by running it in a transaction PostgreSQL has
//! already aborted.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::metrics::Metrics;
use project_portfolio_management::models::audit_logs::Model as AuditModel;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_failed_audit_write_is_counted_not_dropped() {
    request::<App, _, _>(|_request, ctx| async move {
        let failed = || Metrics::global().audit_write_failed_total.get();
        let before = failed();

        // A good write is not counted.
        AuditModel::record_or_log(&ctx.db, Uuid::new_v4(), "cq3_ok", None, None).await;
        assert!(
            (failed() - before).abs() < f64::EPSILON,
            "a written row is not a failure"
        );

        // An aborted transaction refuses every further statement, the audit insert included.
        let txn = ctx.db.begin().await.expect("begin");
        assert!(txn.execute_unprepared("SELECT 1/0").await.is_err());
        AuditModel::record_or_log(&txn, Uuid::new_v4(), "cq3_failed", None, None).await;
        txn.rollback().await.expect("rollback");
        assert!(
            (failed() - before - 1.0).abs() < f64::EPSILON,
            "the failed audit write is counted once"
        );
    })
    .await;
}
