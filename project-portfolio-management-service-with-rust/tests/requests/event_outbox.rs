//! DB-gated integration tests for the durable event bus Phase-2
//! transactional outbox (`agents/share/event-bus.md` §3, §5).
//!
//! These boot the real app against the `test` environment config, so
//! they require a reachable PostgreSQL instance and are therefore
//! `#[ignore]`d (run with `cargo test -- --ignored`). They exercise the
//! exact write path the `outbox` transport takes in the CRUD handlers:
//! one transaction spanning the `plans` insert **and** the outbox's
//! insert, so the two commit or roll back together, and the relay that
//! sends what was written. The outbox is a record (`axa-outbox` 0.2) that
//! refuses `UPDATE` and `DELETE`, so these tests bring an event to each
//! state the way a relay does, never by editing it.
//!
//! The pure envelope→event mapping and the transport-string parsing are
//! pinned DB-free in `src/models/event_outbox.rs` and `src/streaming.rs`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::event_outbox;
use project_portfolio_management::models::plans::Model as PlanModel;
use project_portfolio_management::relay::{self, EventSink, SinkError};
use project_portfolio_management::streaming::{ENTITY, Envelope, EventKind, SCHEMA_VERSION};
use project_portfolio_management_matcher::Plan;
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

/// The outbox events for a record, oldest first: `(event_id, topic, kind, state)`.
pub(crate) async fn outbox_rows(
    db: project_portfolio_management::db::Counted<&sqlx::PgPool>,
    pid: Uuid,
) -> Vec<(Uuid, String, String, String)> {
    // While owed, an event's state is `pending`, `claimed` or `failed`; after, its latest attempt (`delivered`).
    sqlx::query_as(
        "SELECT e.event_id, e.topic, e.kind, COALESCE(o.state, (SELECT a.outcome FROM axa_outbox_attempts a \
         WHERE a.event_id = e.event_id ORDER BY a.id DESC LIMIT 1)) FROM axa_outbox_events e \
         LEFT JOIN axa_outbox_owed o ON o.event_id = e.event_id WHERE e.key = $1 ORDER BY e.id",
    )
    .bind(pid.to_string())
    .fetch_all(db)
    .await
    .expect("load outbox rows")
}

/// Outbox happy path: creating a plan writes BOTH the plan row
/// and exactly one `event_outbox` row, in one transaction. This mirrors
/// the `outbox`-transport CRUD handler path (`streaming::create_and_emit`).
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn outbox_create_writes_plan_and_one_event_atomically() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let wi = Plan::new("Outbox platform migration");

        // One transaction spanning the entity insert and the outbox insert.
        let mut txn = project_portfolio_management::db::begin(&ctx)
            .await
            .expect("begin tx");
        let model = PlanModel::create(&mut txn, &wi).await.expect("create plan");
        let env = created_envelope(&model);
        event_outbox::write(&mut txn, &env)
            .await
            .expect("enqueue outbox row");
        txn.commit().await.expect("commit tx");

        // The plan row persisted...
        let wi_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM plans")
            .fetch_one(project_portfolio_management::db::pool(&ctx))
            .await
            .expect("count plans");
        assert_eq!(wi_count, 1, "the plans row committed");

        // ...and exactly one outbox row for it, unpublished, correctly mapped.
        let rows = outbox_rows(project_portfolio_management::db::pool(&ctx), model.pid).await;
        assert_eq!(
            rows,
            vec![(
                env.event_id,
                "plan".to_string(),
                "created".to_string(),
                "pending".to_string()
            )],
            "exactly one pending outbox event for the plan, keyed by its pid"
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
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn outbox_failure_rolls_back_both_writes() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let wi = Plan::new("Rollback platform migration");

        let mut txn = project_portfolio_management::db::begin(&ctx)
            .await
            .expect("begin tx");
        let model = PlanModel::create(&mut txn, &wi).await.expect("create plan");
        let env = created_envelope(&model);
        event_outbox::write(&mut txn, &env)
            .await
            .expect("enqueue outbox row");
        // Simulate a handler error after both writes but before commit:
        // roll the whole transaction back.
        // Dropped without a commit, the transaction rolls back.
        drop(txn);

        // Neither write survived — the two are all-or-nothing.
        let wi_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM plans")
            .fetch_one(project_portfolio_management::db::pool(&ctx))
            .await
            .expect("count plans");
        assert_eq!(wi_count, 0, "the plans insert rolled back");
        assert!(
            outbox_rows(project_portfolio_management::db::pool(&ctx), model.pid)
                .await
                .is_empty(),
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

/// The relay sends each event through the sink, in the order they were written, and records it delivered; while the
/// sink is down nothing is delivered, and nothing is lost.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn the_relay_sends_each_row_in_order_and_only_marks_what_was_sent() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let mut txn = project_portfolio_management::db::begin(&ctx)
            .await
            .expect("begin tx");
        let model = PlanModel::create(&mut txn, &Plan::new("Relayed plan"))
            .await
            .expect("create plan");
        for kind in [EventKind::Created, EventKind::Updated, EventKind::Deleted] {
            let env = Envelope {
                kind,
                ..created_envelope(&model)
            };
            event_outbox::write(&mut txn, &env).await.expect("write");
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
        .expect("connect to the outbox");
        let (stop, stopped) = tokio::sync::watch::channel(false);
        let running = tokio::spawn(relay::run(outbox, sink.clone(), stopped));

        // Down: rows stay owed.
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let statuses = |rows: Vec<(Uuid, String, String, String)>| {
            rows.into_iter().map(|r| r.3).collect::<Vec<_>>()
        };
        assert!(
            statuses(outbox_rows(project_portfolio_management::db::pool(&ctx), model.pid).await)
                .iter()
                .all(|s| s != "delivered"),
            "nothing is delivered while the sink is down"
        );

        // Up: each goes out once, in order, and is marked sent.
        sink.down.store(false, std::sync::atomic::Ordering::SeqCst);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while statuses(outbox_rows(project_portfolio_management::db::pool(&ctx), model.pid).await)
            .iter()
            .any(|s| s != "delivered")
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

/// A send that hangs past the publish limit uses an attempt; out of attempts, the event is left `failed` and kept, and
/// another record's event still goes out. `task outbox_retry_failed` lists it and changes nothing on a dry run; applied,
/// it releases it as an act of a named person, kept in the record, and it is sent once the sink recovers.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn an_event_out_of_attempts_is_kept_as_failed_and_can_be_released() {
    use project_portfolio_management::tasks::outbox_retry_failed::{self, Release};
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let mut txn = project_portfolio_management::db::begin(&ctx).await.expect("begin tx");
        let stuck = PlanModel::create(&mut txn, &Plan::new("Stuck plan"))
            .await
            .expect("create plan");
        let fine = PlanModel::create(&mut txn, &Plan::new("Fine plan"))
            .await
            .expect("create plan");
        event_outbox::write(&mut txn, &created_envelope(&stuck))
            .await
            .expect("write");
        event_outbox::write(&mut txn, &created_envelope(&fine))
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
                .expect("connect to the outbox");
        let operator = axa::outbox::Outbox::connect_existing(&ctx.config.database.uri, options)
            .await
            .expect("connect to the outbox");
        let (stop, stopped) = tokio::sync::watch::channel(false);
        let running = tokio::spawn(relay::run(outbox, sink.clone(), stopped));
        let (stop_counting, stopped_too) = tokio::sync::watch::channel(false);

        let status = |pid| {
            let db = project_portfolio_management::db::pool(&ctx);
            async move {
                outbox_rows(db, pid)
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
        wait_for(fine.pid, "delivered").await;

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

        // The dry run lists it, with the code its last attempt kept, and changes nothing.
        let dry = outbox_retry_failed::run(&ctx, None)
            .await
            .expect("dry run");
        assert_eq!((dry.failed, dry.retried), (1, 0));
        assert_eq!(
            (
                dry.listed[0].topic.as_str(),
                dry.listed[0].kind.as_str(),
                dry.listed[0].attempts,
                dry.listed[0].error.as_deref()
            ),
            ("plan", "created", 1, Some("timed_out"))
        );
        assert_eq!(status(stuck.pid).await, ["failed"], "a dry run changes nothing");

        sink.stuck.store(false, std::sync::atomic::Ordering::SeqCst);
        let release = Release {
            by: "ops-test".into(),
            reason: "sink_restored".into(),
        };
        let applied = outbox_retry_failed::run(&ctx, Some(&release))
            .await
            .expect("apply");
        assert_eq!(applied.retried, 1);
        wait_for(stuck.pid, "delivered").await;
        let (by, reason): (Option<String>, Option<String>) = sqlx::query_as(
            "SELECT a.by, a.reason FROM axa_outbox_attempts a JOIN axa_outbox_events e ON e.event_id = a.event_id \
             WHERE e.key = $1 AND a.outcome = 'released'",
        )
        .bind(stuck.pid.to_string())
        .fetch_one(project_portfolio_management::db::pool(&ctx))
        .await
        .expect("the release is on record");
        assert_eq!(
            (by.as_deref(), reason.as_deref()),
            (Some("ops-test"), Some("sink_restored"))
        );

        let _ = stop.send(true);
        running.await.expect("the relay stops when told");
        let sent = sink.sent.lock().unwrap().clone();
        assert_eq!(
            sent.iter().filter(|k| **k == stuck.pid.to_string()).count(),
            1,
            "the stuck event is sent once, after the release: {sent:?}"
        );
    })
    .await;
}

/// A relay that dies mid-send leaves its events claimed. Once its lease has not been heard from for `stale_minutes`,
/// a live relay takes the claim back: an event with attempts left is `lost`, goes back, and is sent once; one with none
/// left is `abandoned`, and kept as `failed`.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn a_dead_relays_claim_is_taken_back_and_sent() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let uri = ctx.config.database.uri.clone();
        let fast = |max_attempts| axa::outbox::OutboxOptions {
            poll_interval: std::time::Duration::from_millis(50),
            max_attempts,
            ..relay::options()
        };
        let state = |pid: Uuid| {
            let db = project_portfolio_management::db::pool(&ctx);
            async move {
                outbox_rows(db, pid)
                    .await
                    .into_iter()
                    .map(|r| r.3)
                    .collect::<Vec<_>>()
            }
        };
        let wait_for = |pid: Uuid, want: &'static str| {
            let state = &state;
            async move {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
                while state(pid).await != [want] {
                    assert!(std::time::Instant::now() < deadline, "{pid} reached {want}: {:?}", state(pid).await);
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            }
        };
        // A relay whose publish hangs, killed while it holds the claim; then its lease is let go stale.
        let die_holding = |pid: Uuid| {
            let uri = uri.clone();
            let wait_for = &wait_for;
            let db = project_portfolio_management::db::pool(&ctx);
            async move {
                let sink = Arc::new(HangsOn {
                    pid: pid.to_string(),
                    stuck: std::sync::atomic::AtomicBool::new(true),
                    sent: Mutex::new(Vec::new()),
                });
                let outbox = axa::outbox::Outbox::connect_existing(&uri, fast(10))
                    .await
                    .expect("connect to the outbox");
                let (_stop, stopped) = tokio::sync::watch::channel(false);
                let dying = tokio::spawn(relay::run(outbox, sink, stopped));
                wait_for(pid, "claimed").await;
                dying.abort();
                let _ = dying.await;
                sqlx::query(
                    "UPDATE axa_outbox_leases SET heard_at = now() - interval '10 minutes' \
                     WHERE claim IN (SELECT claim FROM axa_outbox_owed WHERE state = 'claimed')",
                )
                .execute(db)
                .await
                .expect("let the dead relay's lease go stale");
            }
        };
        let recover_with = |max_attempts| {
            let uri = uri.clone();
            async move {
                let sink = Arc::new(Recording {
                    sent: Mutex::new(Vec::new()),
                    down: std::sync::atomic::AtomicBool::new(false),
                });
                let outbox = axa::outbox::Outbox::connect_existing(&uri, fast(max_attempts))
                    .await
                    .expect("connect to the outbox");
                let (stop, stopped) = tokio::sync::watch::channel(false);
                (sink.clone(), stop, tokio::spawn(relay::run(outbox, sink, stopped)))
            }
        };
        let write = |name: &'static str| {
            let ctx = ctx.clone();
            async move {
                let mut txn = project_portfolio_management::db::begin(&ctx).await.expect("begin tx");
                let model = PlanModel::create(&mut txn, &Plan::new(name)).await.expect("create plan");
                event_outbox::write(&mut txn, &created_envelope(&model)).await.expect("write");
                txn.commit().await.expect("commit");
                model
            }
        };

        // No attempts left for it under a relay allowing one: abandoned, kept as failed, never sent.
        let spent = write("Spent plan").await;
        die_holding(spent.pid).await;
        let (sink, stop, running) = recover_with(1).await;
        wait_for(spent.pid, "failed").await;
        let _ = stop.send(true);
        running.await.expect("the relay stops when told");
        let outcomes: Vec<String> = sqlx::query_scalar(
            "SELECT a.outcome FROM axa_outbox_attempts a JOIN axa_outbox_events e ON e.event_id = a.event_id \
             WHERE e.key = $1 ORDER BY a.id",
        )
        .bind(spent.pid.to_string())
        .fetch_all(project_portfolio_management::db::pool(&ctx))
        .await
        .expect("the record");
        assert_eq!(outcomes, ["claimed", "abandoned"], "an operator can tell why it failed");
        assert!(sink.sent.lock().unwrap().is_empty(), "the spent event is not sent");

        // Attempts left: lost, taken back, and sent once.
        let orphaned = write("Orphaned plan").await;
        die_holding(orphaned.pid).await;
        let (sink, stop, running) = recover_with(10).await;
        wait_for(orphaned.pid, "delivered").await;
        let _ = stop.send(true);
        running.await.expect("the relay stops when told");
        let sent = sink.sent.lock().unwrap().clone();
        assert_eq!(
            sent.iter()
                .filter(|(_, key, _)| *key == orphaned.pid.to_string())
                .count(),
            1,
            "the orphaned event is sent once: {sent:?}"
        );
        assert_eq!(state(spent.pid).await, ["failed"], "a failed event stays failed");
    })
    .await;
}
