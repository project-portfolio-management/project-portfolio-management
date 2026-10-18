//! The retention purge (RG-4): a dry run changes nothing; applying removes only what is past its
//! period, leaves recent rows alone, and writes an audit row per table.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use chrono::Utc;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::retention;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use serial_test::serial;

async fn count(db: &sea_orm::DatabaseConnection, sql: &str) -> i64 {
    db.query_one_raw(Statement::from_string(
        DatabaseBackend::Postgres,
        sql.to_string(),
    ))
    .await
    .expect("query")
    .expect("row")
    .try_get::<i64>("", "n")
    .expect("n")
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_purge_removes_only_what_is_past_its_period() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let db = &ctx.db;
        for sql in [
            // Notifications: read 120 days ago (purge), read 10 days ago (keep), never read (keep).
            "INSERT INTO notifications (pid, recipient_ref, subject_kind, subject_pid, kind, message, read_at, created_at, updated_at) VALUES \
             (gen_random_uuid(), 'person:a', 'task', gen_random_uuid(), 'k', 'old read', now() - interval '120 days', now(), now()), \
             (gen_random_uuid(), 'person:a', 'task', gen_random_uuid(), 'k', 'new read', now() - interval '10 days', now(), now()), \
             (gen_random_uuid(), 'person:a', 'task', gen_random_uuid(), 'k', 'unread', NULL, now() - interval '400 days', now())",
            // Webhook log: one old, one new.
            "INSERT INTO webhook_deliveries (event_id, entity, kind, url, attempts, status, created_at) VALUES \
             (gen_random_uuid(), 'plan', 'created', 'https://x.example/h', 1, 'delivered', now() - interval '45 days'), \
             (gen_random_uuid(), 'plan', 'created', 'https://x.example/h', 1, 'delivered', now() - interval '2 days')",
        ] {
            db.execute_unprepared(sql).await.expect("seed");
        }

        let now = Utc::now();

        // A dry run counts and changes nothing.
        let dry = retention::run(db, now, &[], false).await.expect("dry run");
        let n = dry.iter().find(|o| o.rule == "notifications").unwrap();
        assert_eq!((n.matched, n.deleted), (1, 0));
        let w = dry.iter().find(|o| o.rule == "webhook_log").unwrap();
        assert_eq!((w.matched, w.deleted), (1, 0));
        assert_eq!(count(db, "SELECT COUNT(*) AS n FROM notifications").await, 3);

        // Applying removes only the old read notification and the old log row.
        let done = retention::run(db, now, &[], true).await.expect("apply");
        let n = done.iter().find(|o| o.rule == "notifications").unwrap();
        assert_eq!((n.matched, n.deleted), (1, 1));
        assert_eq!(count(db, "SELECT COUNT(*) AS n FROM notifications").await, 2);
        assert_eq!(
            count(db, "SELECT COUNT(*) AS n FROM notifications WHERE message = 'old read'").await,
            0
        );
        assert_eq!(count(db, "SELECT COUNT(*) AS n FROM webhook_deliveries").await, 1);

        // One audit row per table actually purged.
        assert_eq!(
            count(
                db,
                "SELECT COUNT(*) AS n FROM audit_logs WHERE action = 'retention_purged' AND actor = 'system:retention'"
            )
            .await,
            2
        );

        // A shorter period reaches the 10-day-old read notification; a second run is a no-op elsewhere.
        let shorter = retention::run(db, now, &[("notifications".to_string(), 5)], true)
            .await
            .expect("shorter");
        assert_eq!(shorter.iter().find(|o| o.rule == "notifications").unwrap().deleted, 1);
        let again = retention::run(db, now, &[], true).await.expect("again");
        assert!(again.iter().all(|o| o.deleted == 0));
    })
    .await;
}
