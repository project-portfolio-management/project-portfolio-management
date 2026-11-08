//! The `sqlx` layer in `src/db.rs`: statements are counted, a transaction is one, and an audit row written on `sqlx` verifies
//! like one written on `SeaORM`.
//!
//! `#[ignore]`d: boots the app, so it needs PostgreSQL via `DATABASE_URL`. Run with
//! `cargo test --test sqlx_layer -- --ignored --test-threads=1`.

use axa::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::compliance::audit_integrity::verify;
use project_portfolio_management::models::audit_logs::Model as AuditModel;
use project_portfolio_management::{db, query_count};
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[ignore = "needs PostgreSQL via DATABASE_URL"]
#[serial]
async fn the_count_sees_a_statement_on_the_pool_and_in_a_transaction() {
    request::<App, _, _>(|_request, ctx| async move {
        // The boot's background tasks run statements of their own just after start, and the count is process-wide:
        // let them settle before either count.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let before = query_count::total();
        sqlx::query("SELECT 1")
            .execute(db::pool(&ctx))
            .await
            .unwrap();
        assert_eq!(query_count::total() - before, 1, "on the pool");

        let before = query_count::total();
        let mut tx = db::begin(&ctx).await.unwrap();
        sqlx::query("SELECT 1").execute(tx.conn()).await.unwrap();
        sqlx::query("SELECT 2").fetch_one(tx.conn()).await.unwrap();
        tx.commit().await.unwrap();
        assert_eq!(query_count::total() - before, 2, "in a transaction");
    })
    .await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DATABASE_URL"]
#[serial]
async fn an_audit_row_on_sqlx_verifies_and_leaves_with_its_transaction() {
    request::<App, _, _>(|_request, ctx| async move {
        let kept = Uuid::new_v4();
        let dropped = Uuid::new_v4();
        let snapshot = serde_json::json!({ "a": 1, "b": [true, null] });

        let row = AuditModel::record_on(
            db::pool(&ctx),
            kept,
            "tested",
            Some("someone"),
            Some(snapshot),
        )
        .await
        .unwrap();
        let report = verify(std::slice::from_ref(&row));
        assert_eq!((report.rows, report.intact), (1, 1), "{report:?}");
        assert_eq!(
            AuditModel::for_entity(&ctx.db, kept).await.unwrap(),
            vec![row]
        );

        let mut tx = db::begin(&ctx).await.unwrap();
        AuditModel::record_on(tx.conn(), dropped, "tested", None, None)
            .await
            .unwrap();
        drop(tx);
        assert!(
            AuditModel::for_entity(&ctx.db, dropped)
                .await
                .unwrap()
                .is_empty()
        );
    })
    .await;
}
