//! DB-gated proof that under the `outbox` transport, `create_and_emit`
//! writes the entity, its `axa_outbox` row, **and** its `audit_logs`
//! row in **one transaction** (`agents/share/event-bus.md` §3: audit and
//! outbox share the handler transaction, so they can never disagree).
//!
//! This drives the real `streaming::create_and_emit` path (not a
//! hand-built transaction), so it also exercises the transport switch. It
//! runs in its **own test binary** so the process-wide `transport()`
//! `OnceLock` — pinned to `outbox` here — does not leak into the
//! enforcement-off request suite (which expects the default `memory`).
//!
//! `#[ignore]`d: boots the app against Postgres. Run with
//! `cargo test --test outbox_audit -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::audit_logs;
use project_portfolio_management::streaming;
use project_portfolio_management_matcher::Plan;
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test --test outbox_audit -- --ignored"]
async fn outbox_create_and_emit_writes_entity_event_and_audit_atomically() {
    // Select the durable transport BEFORE the app boots (the transport is
    // read once into a process-wide OnceLock). `set_var` is `unsafe` in
    // edition 2024; single-threaded test setup.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT", "outbox");
    }

    request::<App, _, _>(|_request, ctx| async move {
        let actor = "11111111-1111-1111-1111-111111111111";
        let wi = Plan::new("Outbox Audit Portfolio");

        // The real handler path: entity + event + audit, one transaction.
        let model = streaming::create_and_emit(&ctx, &wi, Some(actor))
            .await
            .expect("create_and_emit under outbox transport");

        // Entity committed.
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM plans WHERE pid = $1")
                .bind(model.pid)
                .fetch_one(project_portfolio_management::db::pool(&ctx))
                .await
                .expect("count plans"),
            1,
            "the plans row committed"
        );

        // Exactly one outbox row, unpublished, for this record.
        let events: Vec<(String, String)> = sqlx::query_as(
            "SELECT e.kind, o.state FROM axa_outbox_events e \
                 JOIN axa_outbox_owed o ON o.event_id = e.event_id WHERE e.key = $1",
        )
        .bind(model.pid.to_string())
        .fetch_all(project_portfolio_management::db::pool(&ctx))
        .await
        .expect("load outbox rows");
        assert_eq!(
            events,
            vec![("created".to_string(), "pending".to_string())],
            "one pending outbox event"
        );

        // ...and the audit row rode the SAME transaction (§3) — not a
        // best-effort after-commit side channel.
        let audits = sqlx::query_as::<_, audit_logs::Model>(
            "SELECT * FROM audit_logs WHERE entity_pid = $1",
        )
        .bind(model.pid)
        .fetch_all(project_portfolio_management::db::pool(&ctx))
        .await
        .expect("load audit rows");
        assert_eq!(audits.len(), 1, "one audit_logs row, atomic with the event");
        assert_eq!(audits[0].action, "created");
        assert_eq!(audits[0].actor.as_deref(), Some(actor));
    })
    .await;
}
