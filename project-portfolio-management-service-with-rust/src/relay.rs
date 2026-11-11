//! Durable event bus **Phase 3** — the outbox relay, on `axa-outbox`.
//!
//! Phase 2 ([`crate::streaming`]) writes each event into `axa-outbox`'s record
//! (`axa_outbox_events` and `axa_outbox_payloads`) inside the entity mutation's
//! transaction. Phase 3 is the **relay**: [`axa::outbox::Outbox::run`] claims
//! events oldest first, hands each to an [`EventSink`], and records it
//! `delivered` only once the sink answers `Ok`. The crate keeps one record's
//! events in order (by topic and key, the record pid), takes back the claims of
//! a relay that died, and retries an event with a doubling wait. Every event and
//! every attempt is kept: the record's tables refuse `UPDATE`, `DELETE` and
//! `TRUNCATE`, and nothing is swept (owner decision, 9 November 2026). Delivery
//! is **at-least-once**, so sinks and consumers dedupe on the envelope
//! `event_id` (see `agents/share/event-bus.md` §6).
//!
//! The relay applies no policy before an event goes ([`decide::deliver_all`])
//! and writes no audit row of its own for an outcome ([`record::nothing`]): the
//! attempts table is the record of what went out and when.
//!
//! Every sink error counts as the broker being unavailable: no event is charged
//! an attempt, the relay waits longer each time, and the event it was trying
//! waits before the next try, so one bad record holds back its own key and no
//! other. The error's text goes to the log, and the attempt keeps only the code
//! `sink_unavailable`, as a broker's error can quote what it was sent. A sink
//! that panics has rejected that event, which after ten attempts is left
//! `failed` for an operator (`task outbox_retry_failed`).
//!
//! The default [`LoggingSink`] ships every event to the tracing log: it is
//! the **no-broker** dev/CI sink, and it marks every row sent that it logs.
//! `FluvioSink` (BUS-3, only compiled under this crate's `fluvio` cargo
//! feature, off by default) is the real-broker `impl EventSink`, and answers
//! `Ok` once the broker has acknowledged the record.
//!
//! Activation: the relay only runs when the transport is `outbox`
//! (`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TRANSPORT=outbox`) **and** `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY`
//! is truthy — so it is a no-op by default. With those on, `spawn`
//! additionally selects `FluvioSink` over [`LoggingSink`] when
//! `PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT` (§7) is configured — see
//! `spawn`'s and `build_sink`'s doc comments in this file for the "no
//! silent fallback" rule that governs an endpoint configured without the
//! `fluvio` feature.

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axa::outbox::{Failure, Outbox, OutboxOptions, decide, record};
use tokio::sync::watch;

/// A boxed, thread-safe error from a sink send (a bus/broker failure).
pub type SinkError = Box<dyn std::error::Error + Send + Sync>;

/// A durable-bus sink: ships one outbox event to the stream. The relay
/// calls [`EventSink::send`] once per unpublished row; a returned `Err`
/// leaves the row unpublished (it retries next tick). Delivery is
/// at-least-once, so real sinks/consumers dedupe on the envelope
/// `event_id` carried in `payload`.
#[async_trait::async_trait]
pub trait EventSink: Send + Sync {
    /// Ship one event: `entity` selects the topic (`mxi.<entity>.events`),
    /// `key` is the partition key (the record `pid`), `payload` is the full
    /// canonical envelope.
    ///
    /// # Errors
    ///
    /// When the underlying bus/broker send fails.
    async fn send(
        &self,
        entity: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError>;
}

/// A sink that fans out to a **primary** sink plus zero or more
/// **secondary** ones — e.g. [`LoggingSink`]/`FluvioSink` as primary,
/// [`crate::webhooks::WebhookSink`] as a secondary (`agents/share/event-bus.md`
/// §12, repo `tasks.md` EV-3).
///
/// The **primary**'s result governs the relay's at-least-once
/// retry, exactly as a single sink would: its failure leaves the row
/// unpublished. Every **secondary** sink is delivered to only after the
/// primary succeeds, and its own failure is **swallowed** (logged, never
/// propagated) — an optional secondary (a misconfigured or unreachable
/// webhook target) must never re-block the durable bus the primary sink
/// represents. In practice `WebhookSink::send` already never returns
/// `Err` (it fans its own delivery attempts out to spawned tasks), but
/// this stays defensive for any future secondary sink that does send
/// synchronously.
pub struct CompositeSink {
    /// The durable sink whose failure is returned to the caller.
    primary: Box<dyn EventSink>,
    /// Best-effort sinks, such as webhooks; their errors never propagate.
    secondary: Vec<Box<dyn EventSink>>,
}

impl CompositeSink {
    /// Build a composite over one primary sink and its secondaries.
    #[must_use]
    pub fn new(primary: Box<dyn EventSink>, secondary: Vec<Box<dyn EventSink>>) -> Self {
        Self { primary, secondary }
    }
}

#[async_trait::async_trait]
impl EventSink for CompositeSink {
    async fn send(
        &self,
        entity: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        self.primary.send(entity, key, payload).await?;
        for sink in &self.secondary {
            if let Err(err) = sink.send(entity, key, payload).await {
                tracing::warn!(
                    error = %err,
                    "secondary relay sink failed; outbox row is unaffected (already \
                     published by the primary sink)"
                );
            }
        }
        Ok(())
    }
}

/// The default **no-broker** sink: log each event at `INFO`. Used in dev /
/// CI and as an observability tap; the relay drains + acks exactly as it
/// would against a real broker. Never fails.
pub struct LoggingSink;

#[async_trait::async_trait]
impl EventSink for LoggingSink {
    async fn send(
        &self,
        entity: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        tracing::info!(
            topic = format!("mxi.{entity}.events"),
            key,
            payload = %payload,
            "relay: published outbox event"
        );
        Ok(())
    }
}

/// The real-broker sink (BUS-3, following BUS-1's case-service reference;
/// behind the `fluvio` cargo feature): ships each event to a
/// [Fluvio](https://www.fluvio.io/) topic, partitioned by record `pid`
/// per `agents/share/event-bus.md` §7. One producer per topic, held for
/// the sink's lifetime — `TopicProducerPool` is designed to be built once
/// and reused across sends rather than reconnected per event.
#[cfg(feature = "fluvio")]
pub struct FluvioSink {
    /// The connected producer for this sink's topic.
    producer: fluvio::TopicProducerPool,
}

#[cfg(feature = "fluvio")]
impl FluvioSink {
    /// Connect to the Fluvio cluster at `endpoint` and open a producer for
    /// `topic`.
    ///
    /// # Errors
    ///
    /// When the cluster connection or the producer handshake fails.
    pub async fn connect(endpoint: &str, topic: &str) -> Result<Self, SinkError> {
        let config = fluvio::FluvioConfig::new(endpoint);
        let client = fluvio::Fluvio::connect_with_config(&config).await?;
        let producer = client.topic_producer(topic).await?;
        Ok(Self { producer })
    }
}

#[cfg(feature = "fluvio")]
#[async_trait::async_trait]
impl EventSink for FluvioSink {
    async fn send(
        &self,
        _entity: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        // `_entity` selects the *topic*, which this sink is already
        // connected to (one FluvioSink per topic, per `spawn` below); the
        // partition key is the record `pid`, per §7.
        let bytes = serde_json::to_vec(payload)?;
        // Wait for the broker's acknowledgement: only then is the row sent.
        self.producer
            .send(key.to_string(), bytes)
            .await?
            .wait()
            .await?;
        Ok(())
    }
}

/// Whether the relay background loop should run: `PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY`
/// truthy (`1`/`true`/`yes`/`on`). Off by default.
#[must_use]
pub fn relay_enabled() -> bool {
    matches!(
        std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY")
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "on"
    )
}

/// Relay poll interval in seconds (`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY_INTERVAL_SECS`,
/// default 5, floored at 1).
#[must_use]
pub fn interval_secs() -> u64 {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(5)
        .max(1)
}

/// The setting that used to delete sent events after a number of calendar days. Events are now kept as a record, so
/// it does nothing; [`spawn`] warns when it is set, so that a deployment relying on it hears so.
pub const RETIRED_RETENTION_SETTING: &str = "PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RETENTION_DAYS";

/// Whether the retired retention setting is set, to anything.
#[must_use]
pub fn retired_retention_is_set() -> bool {
    std::env::var_os(RETIRED_RETENTION_SETTING).is_some()
}

/// Fluvio broker endpoint (`PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT`,
/// §7). Unset/blank ⇒ no broker configured, so the relay uses
/// [`LoggingSink`] — the documented no-broker dev/CI sink — even under
/// the `outbox` transport.
#[must_use]
pub fn fluvio_endpoint() -> Option<String> {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT")
        .ok()
        .filter(|s| !s.trim().is_empty())
}

/// The topic events publish to
/// (`PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TOPIC`, default `mxi.plan.events`
/// per §7's `mxi.<entity>.events` convention).
#[must_use]
pub fn event_topic() -> String {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_TOPIC")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "mxi.plan.events".to_string())
}

/// Seconds between retries while the initial Fluvio connection is down.
#[cfg(feature = "fluvio")]
const RECONNECT_BACKOFF_SECS: u64 = 5;

/// Build the relay's sink for this run.
///
/// [`FluvioSink`] when an endpoint is configured — `spawn` has already
/// refused to start if an endpoint was set without this feature compiled
/// in, so reaching a `None` endpoint here always means [`LoggingSink`] is
/// the intended sink. The initial connection is **retried indefinitely**
/// rather than falling back to [`LoggingSink`]: a fallback here would
/// silently start stamping outbox rows `published_at` without ever
/// reaching the broker the operator explicitly asked for — the same
/// silent-data-loss shape the family's artifact-store "no fallback on an
/// explicit backend choice" rule exists to prevent
/// (`agents/share/bulk-import-export.md` §12).
#[cfg(feature = "fluvio")]
async fn build_sink(endpoint: Option<&str>) -> Box<dyn EventSink> {
    let Some(endpoint) = endpoint else {
        return Box::new(LoggingSink);
    };
    let topic = event_topic();
    loop {
        match FluvioSink::connect(endpoint, &topic).await {
            Ok(sink) => {
                tracing::info!(endpoint, topic, "relay: connected FluvioSink");
                return Box::new(sink);
            }
            Err(err) => {
                tracing::warn!(endpoint, error = %err, "relay: FluvioSink connect failed; retrying");
                tokio::time::sleep(Duration::from_secs(RECONNECT_BACKOFF_SECS)).await;
            }
        }
    }
}

/// Without the `fluvio` feature, `spawn` has already refused to start
/// when an endpoint is configured, so the only reachable case is
/// [`LoggingSink`]. Stays `async` (with nothing to await) so both cfg
/// variants share one call site in `spawn`.
#[cfg(not(feature = "fluvio"))]
#[expect(clippy::unused_async)]
async fn build_sink(_endpoint: Option<&str>) -> Box<dyn EventSink> {
    Box::new(LoggingSink)
}

/// The relay's settings: poll every [`interval_secs`], and the crate's defaults for the rest (100 events a claim, ten
/// attempts, a 30-second publish limit, a claim lost after five minutes, every topic in order per key).
#[must_use]
pub fn options() -> OutboxOptions {
    OutboxOptions {
        poll_interval: Duration::from_secs(interval_secs()),
        ..OutboxOptions::default()
    }
}

/// The code an attempt keeps when a sink fails. The sink's own text goes to the log only.
pub const SINK_UNAVAILABLE: &str = "sink_unavailable";

/// Publish one claimed event through `sink`. Every sink error is an outage (see the module docs).
async fn publish(sink: Arc<dyn EventSink>, event: axa::outbox::Claimed) -> Result<(), Failure> {
    sink.send(&event.topic, &event.key, &event.payload)
        .await
        .map_err(|err| {
            tracing::warn!(error = %err, event_id = %event.event_id, topic = %event.topic, "relay: the sink failed");
            Failure::unavailable(SINK_UNAVAILABLE)
        })
}

/// Tells a running relay to stop: it finishes the publish in hand and puts the rest of its batch back.
static STOP: OnceLock<watch::Sender<bool>> = OnceLock::new();

/// Stop the relay, if one was started. Called on shutdown.
pub fn stop() {
    if let Some(stop) = STOP.get() {
        let _ = stop.send(true);
    }
}

/// Relay `outbox` through `sink` until `stopped` turns `true`.
pub async fn run(outbox: Outbox, sink: Arc<dyn EventSink>, stopped: watch::Receiver<bool>) {
    let counting = tokio::spawn(count(outbox.clone(), stopped.clone(), STATS_EVERY));
    outbox
        .run(
            decide::deliver_all,
            move |event| publish(sink.clone(), event),
            record::nothing,
            stopped,
        )
        .await;
    let _ = counting.await;
}

/// How often a running relay reads the outbox's counts into `/metrics.prom`.
pub const STATS_EVERY: Duration = Duration::from_secs(30);

/// Read the outbox's counts every `every` and publish them ([`crate::metrics::Metrics::publish_outbox`]), until
/// stopped. A failed read is logged and skipped, so the last refresh goes stale and says so.
pub async fn count(outbox: Outbox, mut stopped: watch::Receiver<bool>, every: Duration) {
    loop {
        match outbox.stats().await {
            Ok(stats) => crate::metrics::Metrics::global().publish_outbox(
                crate::metrics::OutboxCounts {
                    pending: stats.pending,
                    claimed: stats.claimed,
                    held: stats.held,
                    failed: stats.failed,
                    tampered: stats.tampered,
                    oldest_owed_seconds: stats.oldest.map(|d| d.as_secs_f64()),
                },
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(0.0, |d| d.as_secs_f64()),
            ),
            Err(err) => tracing::warn!(error = %err, "relay: cannot read the outbox's counts"),
        }
        tokio::select! {
            () = tokio::time::sleep(every) => {}
            _ = stopped.wait_for(|stop| *stop) => return,
        }
    }
}

/// Connect to the outbox's record, which the migrations made, retrying while the database is down.
async fn connect(uri: &str) -> Outbox {
    loop {
        match Outbox::connect_existing(uri, options()).await {
            Ok(outbox) => return outbox,
            Err(err) => {
                tracing::warn!(error = %err, "relay: cannot reach the outbox; retrying");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Spawn the background relay loop if (and only if) the transport is
/// `outbox` and [`relay_enabled`]. A no-op otherwise — so with the default
/// `memory` transport this never starts, keeping behaviour unchanged.
///
/// Sink selection: no `PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT` ⇒
/// [`LoggingSink`]. An endpoint configured **without** the `fluvio` cargo
/// feature is a clean refusal to start the relay at all (logged at
/// `error`) rather than a silent `LoggingSink` fallback — see
/// `build_sink`'s doc comment in this file for why a fallback here would
/// be worse than not running.
///
/// **Webhook fan-out** ([`crate::webhooks`], §12): when
/// `PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS[_FILE]` names at least one
/// target, the primary sink above is wrapped in a [`CompositeSink`] with
/// a [`crate::webhooks::WebhookSink`] secondary — unless no integrity-MAC
/// key is configured, in which case webhook delivery refuses to start
/// (logged `error`) rather than send unsigned deliveries; the primary
/// relay still starts either way.
pub fn spawn(ctx: &axa::AppContext) {
    if !crate::streaming::transport().is_outbox() || !relay_enabled() {
        return;
    }
    let endpoint = fluvio_endpoint();
    if endpoint.is_some() && !cfg!(feature = "fluvio") {
        tracing::error!(
            "PROJECT_PORTFOLIO_MANAGEMENT_FLUVIO_ENDPOINT is set but this binary was built \
             without the `fluvio` cargo feature; the relay will NOT start — rebuild with \
             `--features fluvio` (falling back to the logging sink would mark outbox rows \
             published without ever reaching a real broker)"
        );
        return;
    }
    let interval = interval_secs();
    if retired_retention_is_set() {
        tracing::warn!(
            setting = RETIRED_RETENTION_SETTING,
            "this setting no longer deletes anything: every event is kept as a record (axa-outbox 0.2); unset it"
        );
    }
    let webhook_targets = crate::webhooks::targets();
    tracing::info!(
        interval_secs = interval,
        fluvio_endpoint = endpoint.as_deref().unwrap_or("(none — LoggingSink)"),
        webhook_targets = webhook_targets.len(),
        "starting the outbox relay"
    );
    let (stop, stopped) = watch::channel(false);
    if STOP.set(stop).is_err() {
        tracing::error!("the event-outbox relay was already started; not starting a second");
        return;
    }
    let uri = ctx.config.database.uri.clone();
    let ctx = ctx.clone();
    tokio::spawn(async move {
        let outbox = connect(&uri).await;
        let primary = build_sink(endpoint.as_deref()).await;
        let sink: Box<dyn EventSink> = if webhook_targets.is_empty() {
            primary
        } else if crate::compliance::mac::is_enabled() {
            Box::new(CompositeSink::new(
                primary,
                vec![Box::new(crate::webhooks::WebhookSink::new(
                    webhook_targets,
                    ctx,
                ))],
            ))
        } else {
            tracing::error!(
                targets = webhook_targets.len(),
                "{} names webhook target(s) but no integrity-MAC key is configured \
                 ({}/{}); webhook delivery will NOT start (an unsigned webhook is a \
                 different, unauthenticated feature — see agents/share/event-bus.md §12); \
                 the primary relay sink is unaffected",
                crate::webhooks::WEBHOOKS_ENV,
                crate::compliance::mac::KEY_ENV,
                crate::compliance::mac::KEY_FILE_ENV,
            );
            primary
        };
        run(outbox, Arc::from(sink), stopped).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// A capturing sink for tests: records every `(entity, key)` sent.
    struct CapturingSink(Mutex<Vec<(String, String)>>);

    #[async_trait::async_trait]
    impl EventSink for CapturingSink {
        async fn send(
            &self,
            entity: &str,
            key: &str,
            _payload: &serde_json::Value,
        ) -> Result<(), SinkError> {
            self.0
                .lock()
                .unwrap()
                .push((entity.to_string(), key.to_string()));
            Ok(())
        }
    }

    /// The logging sink never fails (async smoke over a tiny runtime).
    #[test]
    fn logging_sink_sends_ok() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt.block_on(async {
            LoggingSink
                .send("plan", "pid-1", &serde_json::json!({"kind": "created"}))
                .await
        });
        assert!(out.is_ok());
    }

    /// A capturing sink records what it is handed (pins the send contract
    /// the DB-gated drain test relies on).
    #[test]
    fn capturing_sink_records_entity_and_key() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let sink = CapturingSink(Mutex::new(Vec::new()));
        rt.block_on(async {
            sink.send("plan", "pid-9", &serde_json::json!({}))
                .await
                .unwrap();
        });
        assert_eq!(
            sink.0.lock().unwrap().as_slice(),
            &[("plan".to_string(), "pid-9".to_string())]
        );
    }

    /// Config parsers: relay off by default; interval floors at 1 (env-independent defaults, since env may be unset).
    #[test]
    fn config_defaults_are_safe() {
        // These read process env; with the vars unset (the CI default) the
        // documented defaults apply.
        if std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY").is_err() {
            assert!(!relay_enabled());
        }
        if std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY_INTERVAL_SECS").is_err() {
            assert_eq!(interval_secs(), 5);
        }
        if std::env::var_os(RETIRED_RETENTION_SETTING).is_none() {
            assert!(!retired_retention_is_set());
        }
    }
}
