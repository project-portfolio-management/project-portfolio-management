//! The audit trail is protected in the database (RG-3): a row cannot be changed, and cannot be
//! deleted except inside a transaction that the retention task has flagged.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::audit_logs::Model as AuditModel;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn audit_rows_cannot_be_changed_or_deleted_without_the_flags() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let entity = Uuid::new_v4();
        AuditModel::record(&ctx.db, entity, "test_action", Some("tester"), None)
            .await
            .expect("record");
        let filter = format!("WHERE entity_pid = '{entity}'");

        // An update is refused.
        let changed = ctx
            .db
            .execute_unprepared(&format!("UPDATE audit_logs SET action = 'x' {filter}"))
            .await;
        assert!(changed.is_err(), "an audit row was changed");
        // A delete is refused.
        let gone = ctx
            .db
            .execute_unprepared(&format!("DELETE FROM audit_logs {filter}"))
            .await;
        assert!(gone.is_err(), "an audit row was deleted");

        // Nothing changed.
        let rows = ctx
            .db
            .query_all_raw(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!("SELECT action FROM audit_logs {filter}"),
            ))
            .await
            .expect("select");
        assert_eq!(rows.len(), 1);

        // The retention task's flag, set for one transaction only, allows the delete.
        let tx = ctx.db.begin().await.expect("begin");
        tx.execute_unprepared("SELECT set_config('ppm.retention_purge', 'on', true)")
            .await
            .expect("flag");
        tx.execute_unprepared(&format!("DELETE FROM audit_logs {filter}"))
            .await
            .expect("flagged delete");
        tx.commit().await.expect("commit");

        // The flag did not leak: the next unflagged delete is refused again.
        AuditModel::record(&ctx.db, entity, "again", None, None)
            .await
            .expect("record again");
        assert!(
            ctx.db
                .execute_unprepared(&format!("DELETE FROM audit_logs {filter}"))
                .await
                .is_err()
        );
    })
    .await;
}
