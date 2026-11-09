//! DB-gated integration tests for the durable event bus Phase-2
//! transactional outbox (`agents/share/event-bus.md` §3, §5).
//!
//! These boot the real app against the `test` environment config, so
//! they require a reachable PostgreSQL instance and are therefore
//! `#[ignore]`d (run with `cargo test -- --ignored`). They exercise the
//! exact write path the `outbox` transport takes in the CRUD handlers:
//! one transaction spanning the `plans` insert **and** the
//! `axa_outbox` insert, so the two commit or roll back together, and the
//! relay that sends what was written.
//!
//! The pure envelope→event mapping and the transport-string parsing are
//! pinned DB-free in `src/models/event_outbox.rs` and `src/streaming.rs`.

use axa::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::plans;
use project_portfolio_management::models::event_outbox;
use project_portfolio_management::models::plans::Model as PlanModel;
use project_portfolio_management::relay::{self, EventSink, SinkError};
use project_portfolio_management::streaming::{ENTITY, Envelope, EventKind, SCHEMA_VERSION};
use project_portfolio_management_matcher::Plan;
use sea_orm::{EntityTrait, PaginatorTrait, TransactionTrait};
use serial_test::serial;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Build a `Created` envelope for a freshly-created plan row.
fn created_envelope(model: &PlanModel) -> Envelope {
    Envelope {
        event_id: Uuid::new_v4(),
        schema_version: SCHEMA_VERSION,
        entity: ENTITY,
        kind: EventKind::Created,
        pid: model.pid.to_string(),
        seq: 1,
        actor: None,
        name: model.name.clone(),
    }
}

/// The outbox rows for a record, oldest first: `(event_id, topic, kind, status)`.
pub(crate) async fn outbox_rows(
    db: &sea_orm::DatabaseConnection,
    pid: Uuid,
) -> Vec<(Uuid, String, String, String)> {
    sqlx::query_as(
        "SELECT event_id, topic, kind, status FROM axa_outbox WHERE key = $1 ORDER BY id",
    )
    .bind(pid.to_string())
    .fetch_all(db.get_postgres_connection_pool())
    .await
    .expect("load outbox rows")
}

/// Outbox happy path: creating a plan writes BOTH the plan row
/// and exactly one `event_outbox` row, in one transaction. This mirrors
/// the `outbox`-transport CRUD handler path (`streaming::create_and_emit`).
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with: cargo test -- --ignored"]
async fn outbox_create_writes_plan_and_one_event_atomically() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let wi = Plan::new("Outbox platform migration");

        // One transaction spanning the entity insert and the outbox insert.
        let txn = ctx.db.begin().await.expect("begin tx");
        let model = PlanModel::create(&txn, &wi).await.expect("create plan");
        let env = created_envelope(&model);
        event_outbox::write(&txn, &env)
            .await
            .expect("enqueue outbox row");
        txn.commit().await.expect("commit tx");

        // The plan row persisted...
        let wi_count = plans::Entity::find()
            .count(&ctx.db)
            .await
            .expect("count plans");
        assert_eq!(wi_count, 1, "the plans row committed");

        // ...and exactly one outbox row for it, unpublished, correctly mapped.
        let rows = outbox_rows(&ctx.db, model.pid).await;
        assert_eq!(
            rows,
            vec![(
                env.event_id,
                "plan".to_string(),
                "created".to_string(),
                "pending".to_string()
            )],
            "exactly one pending axa_outbox row for the plan, keyed by its pid"
        );
    })
    .await;
}

/// Outbox atomicity: a failure before commit rolls back BOTH the
/// plan row and its `event_outbox` row — neither is left behind.
/// This is the crash window the outbox closes (no committed change without
/// its event, and no event without its committed change).
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with: cargo test -- --ignored"]
async fn outbox_failure_rolls_back_both_writes() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let wi = Plan::new("Rollback platform migration");

        let txn = ctx.db.begin().await.expect("begin tx");
        let model = PlanModel::create(&txn, &wi).await.expect("create plan");
        let env = created_envelope(&model);
        event_outbox::write(&txn, &env)
            .await
            .expect("enqueue outbox row");
        // Simulate a handler error after both writes but before commit:
        // roll the whole transaction back.
        txn.rollback().await.expect("rollback tx");

        // Neither write survived — the two are all-or-nothing.
        let wi_count = plans::Entity::find()
            .count(&ctx.db)
            .await
            .expect("count plans");
        assert_eq!(wi_count, 0, "the plans insert rolled back");
        assert!(
            outbox_rows(&ctx.db, model.pid).await.is_empty(),
            "the event insert rolled back with it"
        );
    })
    .await;
}

/// A sink that records what it is sent, or fails while `down` is set.
struct Recording {
    sent: Mutex<Vec<(String, String, String)>>,
    down: std::sync::atomic::AtomicBool,
}

#[async_trait::async_trait]
impl EventSink for Recording {
    async fn send(
        &self,
        entity: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        if self.down.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("the broker is down".into());
        }
        self.sent.lock().unwrap().push((
            entity.to_string(),
            key.to_string(),
            payload["kind"].as_str().unwrap_or_default().to_string(),
        ));
        Ok(())
    }
}

/// The relay sends each row through the sink, in the order they were written, and marks it sent; while the sink is
/// down nothing is marked sent, and nothing is lost.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with: cargo test -- --ignored"]
async fn the_relay_sends_each_row_in_order_and_only_marks_what_was_sent() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let txn = ctx.db.begin().await.expect("begin tx");
        let model = PlanModel::create(&txn, &Plan::new("Relayed plan"))
            .await
            .expect("create plan");
        for kind in [EventKind::Created, EventKind::Updated, EventKind::Deleted] {
            let env = Envelope {
                kind,
                ..created_envelope(&model)
            };
            event_outbox::write(&txn, &env).await.expect("write");
        }
        txn.commit().await.expect("commit");

        let sink = Arc::new(Recording {
            sent: Mutex::new(Vec::new()),
            down: std::sync::atomic::AtomicBool::new(true),
        });
        let outbox = axa::outbox::Outbox::connect_existing(
            &ctx.config.database.uri,
            axa::outbox::OutboxOptions {
                poll_interval: std::time::Duration::from_millis(50),
                retry_backoff: std::time::Duration::from_millis(50),
                ..relay::options()
            },
        )
        .await
        .expect("connect to axa_outbox");
        let (stop, stopped) = tokio::sync::watch::channel(false);
        let running = tokio::spawn(relay::run(outbox, sink.clone(), stopped));

        // Down: rows stay owed.
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let statuses = |rows: Vec<(Uuid, String, String, String)>| {
            rows.into_iter().map(|r| r.3).collect::<Vec<_>>()
        };
        assert!(
            statuses(outbox_rows(&ctx.db, model.pid).await)
                .iter()
                .all(|s| s != "sent"),
            "nothing is marked sent while the sink is down"
        );

        // Up: each goes out once, in order, and is marked sent.
        sink.down.store(false, std::sync::atomic::Ordering::SeqCst);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while statuses(outbox_rows(&ctx.db, model.pid).await)
            .iter()
            .any(|s| s != "sent")
        {
            assert!(
                std::time::Instant::now() < deadline,
                "the relay sent every row"
            );
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let _ = stop.send(true);
        running.await.expect("the relay stops when told");
        let pid = model.pid.to_string();
        assert_eq!(
            *sink.sent.lock().unwrap(),
            ["created", "updated", "deleted"]
                .map(|kind| ("plan".to_string(), pid.clone(), kind.to_string()))
                .to_vec(),
        );
    })
    .await;
}

/// A sink that hangs on one record's events while `stuck` is set, and sends everything else.
struct HangsOn {
    pid: String,
    stuck: std::sync::atomic::AtomicBool,
    sent: Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl EventSink for HangsOn {
    async fn send(
        &self,
        _entity: &str,
        key: &str,
        _payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        if key == self.pid && self.stuck.load(std::sync::atomic::Ordering::SeqCst) {
            std::future::pending::<()>().await;
        }
        self.sent.lock().unwrap().push(key.to_string());
        Ok(())
    }
}

/// A send that hangs past the publish limit uses an attempt; out of attempts, the row is left `failed` and kept, and
/// another record's event still goes out. `retry_failed` (the operator's fix in `operations/deployment.md`) owes it
/// again, and it is sent once the sink recovers.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with: cargo test -- --ignored"]
async fn a_row_out_of_attempts_is_kept_as_failed_and_can_be_owed_again() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let txn = ctx.db.begin().await.expect("begin tx");
        let stuck = PlanModel::create(&txn, &Plan::new("Stuck plan"))
            .await
            .expect("create plan");
        let fine = PlanModel::create(&txn, &Plan::new("Fine plan"))
            .await
            .expect("create plan");
        event_outbox::write(&txn, &created_envelope(&stuck))
            .await
            .expect("write");
        event_outbox::write(&txn, &created_envelope(&fine))
            .await
            .expect("write");
        txn.commit().await.expect("commit");

        let sink = Arc::new(HangsOn {
            pid: stuck.pid.to_string(),
            stuck: std::sync::atomic::AtomicBool::new(true),
            sent: Mutex::new(Vec::new()),
        });
        let options = axa::outbox::OutboxOptions {
            poll_interval: std::time::Duration::from_millis(50),
            retry_backoff: std::time::Duration::from_millis(50),
            publish_timeout: std::time::Duration::from_millis(200),
            max_attempts: 1,
            ..relay::options()
        };
        let outbox =
            axa::outbox::Outbox::connect_existing(&ctx.config.database.uri, options.clone())
                .await
                .expect("connect to axa_outbox");
        let operator = axa::outbox::Outbox::connect_existing(&ctx.config.database.uri, options)
            .await
            .expect("connect to axa_outbox");
        let (stop, stopped) = tokio::sync::watch::channel(false);
        let running = tokio::spawn(relay::run(outbox, sink.clone(), stopped));
        let (stop_counting, stopped_too) = tokio::sync::watch::channel(false);

        let status = |pid| {
            let db = ctx.db.clone();
            async move {
                outbox_rows(&db, pid)
                    .await
                    .into_iter()
                    .map(|r| r.3)
                    .collect::<Vec<_>>()
            }
        };
        let wait_for = |pid, want: &'static str| {
            let status = &status;
            async move {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
                while status(pid).await != [want] {
                    assert!(std::time::Instant::now() < deadline, "{pid} reached {want}");
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            }
        };

        wait_for(stuck.pid, "failed").await;
        wait_for(fine.pid, "sent").await;

        // The relay's counts reach `/metrics.prom`, so an operator can be told of the failed row.
        let counting = tokio::spawn(relay::count(
            operator.clone(),
            stopped_too.clone(),
            std::time::Duration::from_millis(50),
        ));
        let failed_rows = || {
            project_portfolio_management::metrics::Metrics::global()
                .render()
                .lines()
                .find_map(|l| l.strip_prefix("ppm_event_outbox_rows{status=\"failed\"} "))
                .and_then(|v| v.parse::<f64>().ok())
        };
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while failed_rows().is_none_or(|n| n < 1.0) {
            assert!(
                std::time::Instant::now() < deadline,
                "the failed row is counted"
            );
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let _ = stop_counting.send(true);
        counting.await.expect("the count stops when told");

        sink.stuck.store(false, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(operator.retry_failed().await.expect("retry_failed"), 1);
        wait_for(stuck.pid, "sent").await;

        let _ = stop.send(true);
        running.await.expect("the relay stops when told");
        let sent = sink.sent.lock().unwrap().clone();
        assert_eq!(
            sent.iter().filter(|k| **k == stuck.pid.to_string()).count(),
            1,
            "the stuck row is sent once, after the retry: {sent:?}"
        );
    })
    .await;
}

/// `task outbox_retry_failed`: a dry run lists a failed row and changes nothing; `apply:true` puts it back to
/// pending with its attempts reset.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DATABASE_URL); run with: cargo test -- --ignored"]
async fn the_retry_task_lists_failed_rows_and_owes_them_again_only_when_applied() {
    use project_portfolio_management::tasks::outbox_retry_failed;
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let key = Uuid::new_v4();
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO axa_outbox (event_id, topic, kind, key, payload, status, attempts, error) \
             VALUES ($1, 'plan', 'created', $2, '{}'::jsonb, 'failed', 10, 'publish timed out') RETURNING id",
        )
        .bind(Uuid::new_v4())
        .bind(key.to_string())
        .fetch_one(ctx.db.get_postgres_connection_pool())
        .await
        .expect("seed a failed row");
        let row = |db: sea_orm::DatabaseConnection| async move {
            sqlx::query_as::<_, (String, i32)>("SELECT status, attempts FROM axa_outbox WHERE id = $1")
                .bind(id)
                .fetch_one(db.get_postgres_connection_pool())
                .await
                .expect("the row")
        };

        let dry = outbox_retry_failed::run(&ctx.db, &ctx.config.database.uri, false)
            .await
            .expect("dry run");
        assert!(dry.failed >= 1);
        assert_eq!(dry.retried, 0);
        let listed = dry.listed.iter().find(|r| r.id == id).expect("the row is listed");
        assert_eq!(
            (listed.topic.as_str(), listed.kind.as_str(), listed.attempts, listed.error.as_deref()),
            ("plan", "created", 10, Some("publish timed out"))
        );
        assert_eq!(row(ctx.db.clone()).await, ("failed".to_string(), 10), "a dry run changes nothing");

        let applied = outbox_retry_failed::run(&ctx.db, &ctx.config.database.uri, true)
            .await
            .expect("apply");
        assert!(applied.retried >= 1);
        assert_eq!(row(ctx.db.clone()).await, ("pending".to_string(), 0), "owed again, attempts reset");
        sqlx::query("DELETE FROM axa_outbox WHERE id = $1")
            .bind(id)
            .execute(ctx.db.get_postgres_connection_pool())
            .await
            .expect("tidy up");
    })
    .await;
}
