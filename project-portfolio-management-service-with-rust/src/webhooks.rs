//! Outbound webhook delivery — the family's `WebhookSink` contract
//! (`agents/share/event-bus.md` §12, repo `tasks.md` EV-3, this crate's
//! own `spec/13-tasks.md` T-28m). Portfolio is the first adopter; other
//! registries copy this shape when a consumer asks for it.
//!
//! ## What this is, and is not
//!
//! This is a **best-effort notification** fan-out, not a second durable
//! bus. The relay's primary sink ([`crate::relay::LoggingSink`] /
//! [`crate::relay::FluvioSink`]) is the at-least-once path the family
//! already depends on: a send failure there leaves the outbox row
//! unpublished so [`crate::relay::drain_once`] retries it. Webhook
//! delivery must never gate that — an operator's slow or unreachable
//! receiver would otherwise stall the entire outbox forever. So
//! [`WebhookSink::send`] never returns an error: each matching target
//! gets its own bounded retry-with-backoff on a spawned task, and a
//! target still failing once that is exhausted is recorded in the
//! [`crate::models::webhook_deliveries`] delivery log and moved past,
//! never re-blocking the row it fanned out from. Compose it with the
//! primary sink via [`crate::relay::CompositeSink`].
//!
//! ## Configuration
//!
//! `PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS` (inline JSON) or
//! `PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS_FILE` (a path to a JSON file;
//! takes precedence when both are set — same precedence rule as the MAC
//! key sourcing, `compliance/mac.rs`) — a JSON array of targets:
//!
//! ```json
//! [
//!   { "url": "https://ops.example.com/hook" },
//!   { "url": "https://audit.example.com/hook", "kinds": ["created", "merged"] }
//! ]
//! ```
//!
//! `kinds` omitted or `null` means every kind. This mirrors the ABAC
//! policy loader's env/file convention
//! (`agents/share/authorization-attributes.md` §5): unset ⇒ no targets
//! (the feature is off); malformed JSON ⇒ logged and treated as no
//! targets, never a boot failure — an optional feature's config typo
//! must not take the service down.
//!
//! Every target is additionally required to be `https://`, or `http://`
//! to a **loopback** host (`127.0.0.1` / `::1` / `localhost`) for local
//! dev/test — the same SEC-V1/SEC-B11 posture the PASETO key fetch and
//! the link-graph presence probe already use. A target that fails this
//! check is dropped at load time with a warning, not silently sent to
//! in plaintext.
//!
//! ## Signing
//!
//! Every delivery carries two headers:
//!
//! - `X-Mxi-Event-Id: <uuid>` — the envelope's dedup key, so a receiver
//!   can deduplicate without parsing the body (`agents/share/event-bus.md`
//!   §6: consumers must be idempotent, keyed on `event_id`).
//! - `X-Mxi-Signature: <tag>` — [`crate::compliance::mac::tag`] under
//!   [`crate::compliance::mac::Domain::Webhook`], i.e. the shared
//!   `integrity-mac` crate's `"<scheme>.<key id>:<hex>"` format,
//!   HMAC-SHA256 over the **exact request body bytes** (below), keyed by
//!   this service's `webhook`-domain subkey.
//!
//! **The pre-image is the exact bytes of the HTTP request body** — the
//! compact `serde_json::to_vec` serialization of the outbox envelope,
//! signed *before* being sent and never re-serialized afterwards, so
//! signer and sender always agree on what was signed.
//!
//! A receiver never holds this service's **root** MAC key — only the
//! `webhook`-domain subkey, which an operator derives offline (HKDF-SHA256
//! over the root key, `info` string `mxi/<service>/webhook/d1` — the
//! shared `integrity_mac` crate's scheme tag `d1`, see
//! `integrity_mac::KeyConfig::info` and the shared crate's module docs,
//! "Domain separation") and hands to the receiver as *their* configured
//! verification secret. The receiver computes HMAC-SHA256(subkey, raw
//! body bytes) and compares the hex after `d1.<key id>:`; a mismatch or
//! an unrecognized key id is a forged or corrupted delivery. Publishing
//! this pre-image format here is what makes the signature checkable at
//! all outside this codebase (repo `tasks.md` EV-3's own condition).
//!
//! No MAC key configured (`compliance::mac::is_enabled()` false) ⇒
//! [`crate::relay::spawn`] refuses to start webhook delivery at all
//! (logged `error`) when targets are configured, rather than silently
//! sending unsigned deliveries — an unsigned webhook is not a smaller
//! version of this feature, it is a different, unauthenticated one. This
//! is the "refuses to start" half of this crate's own T-28m acceptance
//! criterion.
//!
//! ## Format (NC-11)
//!
//! A target may add `"format": "cloudevents_structured"` or `"cloudevents_binary"` to receive a `CloudEvents` 1.0
//! event instead of the bare envelope ([`render`]). The signature is unchanged: HMAC over the exact request body
//! bytes, whatever the format. The default is `native`.
//!
//! ## Retry policy
//!
//! A `5xx` response, or a transport-level failure (no response at all —
//! connection refused, timeout, TLS failure), is **retried**: the
//! receiver or its infrastructure is presumed to be having a transient
//! problem. A `4xx` response is **not retried**: it is the receiver
//! rejecting this exact request, and resending it unchanged would just
//! repeat the rejection. [`MAX_ATTEMPTS`] attempts, exponential backoff
//! starting at [`INITIAL_BACKOFF`] and doubling.

use std::time::Duration;

use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::compliance::mac::{self, Domain};
use crate::models::webhook_deliveries::{DeliveryOutcome, Model as DeliveryLog};
use crate::relay::{EventSink, SinkError};

/// The env var holding the inline JSON target list.
pub const WEBHOOKS_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS";
/// The env var naming a file holding the JSON target list. Takes
/// precedence over [`WEBHOOKS_ENV`] when both are set.
pub const WEBHOOKS_FILE_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS_FILE";

/// The header carrying the envelope dedup id.
pub const EVENT_ID_HEADER: &str = "X-Mxi-Event-Id";
/// The header carrying the HMAC signature (`"<scheme>.<key id>:<hex>"`).
pub const SIGNATURE_HEADER: &str = "X-Mxi-Signature";

/// Maximum HTTP attempts per target per event (the initial attempt plus
/// retries).
pub const MAX_ATTEMPTS: u32 = 4;
/// The first retry's backoff; each subsequent retry doubles it.
pub const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

/// One configured webhook delivery target.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WebhookTarget {
    /// The URL delivered to.
    pub url: String,
    /// Event kinds this target receives (`created` / `updated` /
    /// `deleted` / `merged`). `None` (the field omitted, or explicit
    /// `null`) means every kind.
    #[serde(default)]
    pub kinds: Option<Vec<String>>,
    /// The shape of the request (NC-11). `native` (the default) sends the outbox envelope as it is.
    #[serde(default)]
    pub format: WebhookFormat,
}

/// How a delivery is shaped. The signature is the same in every format: HMAC over the exact request body bytes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookFormat {
    /// The outbox envelope as JSON, `content-type: application/json`.
    #[default]
    Native,
    /// A `CloudEvents` 1.0 JSON event (`application/cloudevents+json`) whose `data` is the envelope.
    CloudeventsStructured,
    /// A `CloudEvents` 1.0 binary-mode event: `ce-*` headers, the envelope as the body.
    CloudeventsBinary,
}

/// A request ready to send: its `content-type`, any extra headers, and the exact body bytes that are signed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
    /// The `content-type` header.
    pub content_type: &'static str,
    /// Extra headers (the `CloudEvents` `ce-*` attributes in binary mode).
    pub headers: Vec<(String, String)>,
    /// The body bytes, signed as they are.
    pub body: Vec<u8>,
}

/// The `CloudEvents` `type` of an event: `org.project-portfolio-management.<entity>.<kind>`.
fn ce_type(entity: &str, kind: &str) -> String {
    format!("org.project-portfolio-management.{entity}.{kind}")
}

/// Shape `payload` (the outbox envelope) for `format`. `None` when it cannot be serialized.
///
/// `CloudEvents` attributes: `specversion` `1.0`; `id` the envelope's `event_id`; `source`
/// `urn:project-portfolio-management:<entity>`; `type` as [`ce_type`]; `subject` the record's pid; `time` only
/// when the envelope carries `occurred_at` (it does not today, and a time is never invented); `datacontenttype`
/// `application/json`. Pure, so it is unit-tested without a network.
#[must_use]
pub fn render(
    format: WebhookFormat,
    entity: &str,
    payload: &serde_json::Value,
) -> Option<Rendered> {
    let text = |key: &str| payload.get(key).and_then(|v| v.as_str());
    let id = text("event_id").unwrap_or_default();
    let kind = text("kind").unwrap_or_default();
    let source = format!("urn:project-portfolio-management:{entity}");
    match format {
        WebhookFormat::Native => Some(Rendered {
            content_type: "application/json",
            headers: Vec::new(),
            body: serde_json::to_vec(payload).ok()?,
        }),
        WebhookFormat::CloudeventsStructured => {
            let mut event = serde_json::json!({
                "specversion": "1.0",
                "id": id,
                "source": source,
                "type": ce_type(entity, kind),
                "datacontenttype": "application/json",
                "data": payload,
            });
            if let Some(pid) = text("pid") {
                event["subject"] = pid.into();
            }
            if let Some(time) = text("occurred_at") {
                event["time"] = time.into();
            }
            Some(Rendered {
                content_type: "application/cloudevents+json",
                headers: Vec::new(),
                body: serde_json::to_vec(&event).ok()?,
            })
        }
        WebhookFormat::CloudeventsBinary => {
            let mut headers = vec![
                ("ce-specversion".to_string(), "1.0".to_string()),
                ("ce-id".to_string(), id.to_string()),
                ("ce-source".to_string(), source),
                ("ce-type".to_string(), ce_type(entity, kind)),
            ];
            if let Some(pid) = text("pid") {
                headers.push(("ce-subject".to_string(), pid.to_string()));
            }
            if let Some(time) = text("occurred_at") {
                headers.push(("ce-time".to_string(), time.to_string()));
            }
            Some(Rendered {
                content_type: "application/json",
                headers,
                body: serde_json::to_vec(payload).ok()?,
            })
        }
    }
}

impl WebhookTarget {
    /// Whether this target should receive an event of `kind`.
    #[must_use]
    pub fn matches_kind(&self, kind: &str) -> bool {
        self.kinds
            .as_ref()
            .is_none_or(|kinds| kinds.iter().any(|k| k == kind))
    }
}

/// Whether a URL may be delivered to (SEC-V1/SEC-B11 posture): `https`
/// to any host, or `http` only to a **loopback** host — the same rule
/// `authentication-verifier`'s key-set fetch and the link-graph presence
/// probe apply. Pure, so it is unit-tested without network access.
#[must_use]
pub fn url_is_permitted(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url.trim()) else {
        return false;
    };
    match parsed.scheme() {
        "https" => true,
        "http" => matches!(
            parsed.host_str(),
            Some("127.0.0.1" | "::1" | "[::1]" | "localhost")
        ),
        _ => false,
    }
}

/// The environment variable that turns on strict outbound targets (RG-1).
pub const OUTBOUND_STRICT_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_OUTBOUND_STRICT";

/// Whether strict outbound mode is on (`on`, `true` or `1`).
#[must_use]
pub fn outbound_strict() -> bool {
    std::env::var(OUTBOUND_STRICT_ENV)
        .is_ok_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "on" | "true" | "1"))
}

/// Whether an **operator-configured** target may be called. Always the
/// [`url_is_permitted`] rule; in `strict` mode also the stricter rule user-supplied
/// destinations already get ([`crate::sinks::destination_url_ok`]: `https` only, a
/// public address, no credentials, no `.internal` names). Pure, so it is unit-tested
/// without touching the environment.
#[must_use]
pub fn target_allowed(url: &str, strict: bool) -> bool {
    url_is_permitted(url) && (!strict || crate::sinks::destination_url_ok(url, false))
}

/// Parse a raw `PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS[_FILE]` JSON value
/// into usable targets: malformed JSON becomes an empty list (logged,
/// never a panic or a propagated error — an optional feature's config
/// typo must not block boot, same posture as the ABAC policy loader),
/// and a target failing [`url_is_permitted`] is dropped (logged) rather
/// than sent to in plaintext.
///
/// Pure and env-free, so it is fully unit-testable — [`targets`] is the
/// thin, untested-by-hand edge that supplies `raw` from the process
/// environment.
#[must_use]
pub fn parse_targets(raw: &str) -> Vec<WebhookTarget> {
    if raw.trim().is_empty() {
        return Vec::new();
    }
    let parsed: Vec<WebhookTarget> = match serde_json::from_str(raw) {
        Ok(targets) => targets,
        Err(err) => {
            tracing::warn!(
                error = %err,
                "failed to parse webhook targets; no webhook targets configured"
            );
            return Vec::new();
        }
    };
    let strict = outbound_strict();
    parsed
        .into_iter()
        .filter(|t| {
            let ok = target_allowed(&t.url, strict);
            if !ok {
                tracing::warn!(
                    url = %t.url,
                    strict,
                    "webhook target is not https:// (or http:// on loopback), or is not a public address in strict mode; dropped"
                );
            }
            ok
        })
        .collect()
}

/// Load the configured webhook targets from the environment:
/// [`WEBHOOKS_FILE_ENV`] (a path) takes precedence over [`WEBHOOKS_ENV`]
/// (inline JSON) when both are set; a file that fails to read is logged
/// and treated as "no targets configured", same as malformed JSON (see
/// [`parse_targets`]).
#[must_use]
pub fn targets() -> Vec<WebhookTarget> {
    let raw = std::env::var(WEBHOOKS_FILE_ENV).ok().map_or_else(
        || std::env::var(WEBHOOKS_ENV).ok(),
        |path| match std::fs::read_to_string(&path) {
            Ok(contents) => Some(contents),
            Err(err) => {
                tracing::warn!(
                    path,
                    error = %err,
                    "failed to read {WEBHOOKS_FILE_ENV}; no webhook targets configured"
                );
                None
            }
        },
    );
    raw.map(|raw| parse_targets(&raw)).unwrap_or_default()
}

/// Whether a failed attempt should be retried: a `5xx` is presumed
/// transient (the receiver or its infrastructure), a `4xx` is the
/// receiver's rejection of this exact request and would just repeat.
#[must_use]
pub fn should_retry_status(status: u16) -> bool {
    (500..600).contains(&status)
}

/// One delivery attempt's result, for the retry loop to act on.
enum AttemptResult {
    Delivered,
    /// Retry-worthy failure: `status` is `None` on a pure transport
    /// error (no response received at all).
    Retryable {
        status: Option<u16>,
        error: String,
    },
    /// A `4xx` — stop retrying immediately.
    Rejected {
        status: u16,
        error: String,
    },
}

/// The shared, **non-redirecting** HTTP client for webhook delivery
/// (SEC-B11 posture): disabling redirects means the only host ever
/// contacted is the operator-configured target — a receiver returning a
/// `3xx` cannot bounce the request to a different, attacker-chosen host.
/// Built once and reused.
pub(crate) fn client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("build the non-redirecting webhook-delivery client")
    })
}

/// In strict outbound mode, resolve the target's name now and refuse it if **any** address it
/// resolves to is not public (a name that points inside the network). `Ok(())` when strict mode is off,
/// the host is an address literal (already judged by [`target_allowed`]), or every address is public;
/// `Err(true)` when it must not be called, `Err(false)` when the name could not be resolved (retry).
///
/// Limit, stated: this checks and then the HTTP client resolves again to connect, so a name that
/// changes its answer between the two (DNS rebinding) is not caught. Pair strict mode with an egress
/// policy at the network edge for anything that matters.
///
/// # Errors
///
/// See above.
pub async fn check_resolution(url: &str) -> Result<(), bool> {
    if !outbound_strict() {
        return Ok(());
    }
    let Ok(parsed) = reqwest::Url::parse(url.trim()) else {
        return Err(true);
    };
    let Some(host) = parsed.host_str() else {
        return Err(true);
    };
    if host
        .trim_matches(['[', ']'])
        .parse::<std::net::IpAddr>()
        .is_ok()
    {
        return Ok(());
    }
    let port = parsed.port_or_known_default().unwrap_or(443);
    match tokio::net::lookup_host((host.to_string(), port)).await {
        Ok(addrs) => {
            let ips: Vec<std::net::IpAddr> = addrs.map(|a| a.ip()).collect();
            if crate::sinks::resolved_ok(&ips, false) {
                Ok(())
            } else {
                Err(true)
            }
        }
        Err(_) => Err(false),
    }
}

/// One HTTP attempt: send `body` (already the signed bytes) to `url`
/// with the given headers.
async fn attempt_once(
    url: &str,
    event_id: &str,
    signature: &str,
    request: &Rendered,
) -> AttemptResult {
    match check_resolution(url).await {
        Ok(()) => {}
        Err(true) => {
            return AttemptResult::Rejected {
                status: 0,
                error: "the target resolves to a non-public address (strict outbound mode)"
                    .to_string(),
            };
        }
        Err(false) => {
            return AttemptResult::Retryable {
                status: None,
                error: "the target's name could not be resolved".to_string(),
            };
        }
    }
    let mut builder = client()
        .post(url)
        .header("content-type", request.content_type)
        .header(EVENT_ID_HEADER, event_id)
        .header(SIGNATURE_HEADER, signature);
    for (name, value) in &request.headers {
        builder = builder.header(name.as_str(), value.as_str());
    }
    let result = builder.body(request.body.clone()).send().await;
    match result {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                AttemptResult::Delivered
            } else if should_retry_status(status.as_u16()) {
                AttemptResult::Retryable {
                    status: Some(status.as_u16()),
                    error: format!("HTTP {status}"),
                }
            } else {
                AttemptResult::Rejected {
                    status: status.as_u16(),
                    error: format!("HTTP {status}"),
                }
            }
        }
        Err(err) => AttemptResult::Retryable {
            status: None,
            error: err.to_string(),
        },
    }
}

/// Deliver one event to one target, retrying `5xx`/transport failures
/// with exponential backoff up to [`MAX_ATTEMPTS`], and record the final
/// outcome in the delivery log. Never panics and never propagates an
/// error — this is the fire-and-forget body a spawned task runs.
async fn deliver_with_retry(
    db: &DatabaseConnection,
    target: &WebhookTarget,
    entity: &str,
    kind: &str,
    event_id: uuid::Uuid,
    signature: &str,
    request: &Rendered,
) {
    let mut backoff = INITIAL_BACKOFF;
    let mut attempts = 0u32;
    let (delivered, status_code, error) = loop {
        attempts += 1;
        match attempt_once(&target.url, &event_id.to_string(), signature, request).await {
            AttemptResult::Delivered => break (true, None, None),
            AttemptResult::Rejected { status, error } => break (false, Some(status), Some(error)),
            AttemptResult::Retryable { status, error } => {
                if attempts >= MAX_ATTEMPTS {
                    break (false, status, Some(error));
                }
                tracing::warn!(
                    url = %target.url,
                    attempt = attempts,
                    error = %error,
                    "webhook delivery attempt failed; retrying"
                );
                tokio::time::sleep(backoff).await;
                backoff *= 2;
            }
        }
    };
    if !delivered {
        tracing::warn!(
            url = %target.url,
            attempts,
            error = ?error,
            "webhook delivery exhausted retries; recorded as failed"
        );
    }
    let outcome = DeliveryOutcome {
        event_id,
        entity: entity.to_string(),
        kind: kind.to_string(),
        url: target.url.clone(),
        attempts,
        delivered,
        status_code,
        error,
    };
    if let Err(err) = DeliveryLog::record(db, &outcome).await {
        tracing::warn!(error = %err, "failed to write webhook delivery-log row");
    }
}

/// The outbound webhook [`EventSink`]. Holds the configured targets and
/// a database handle for the delivery log.
///
/// Constructing this with targets configured but no MAC key enabled is
/// a caller error — [`crate::relay::spawn`] checks
/// [`mac::is_enabled`] first and refuses to build this sink at all in
/// that case (see the module docs' "Signing" section).
pub struct WebhookSink {
    targets: Vec<WebhookTarget>,
    db: DatabaseConnection,
}

impl WebhookSink {
    /// Build a sink over the given targets.
    #[must_use]
    pub fn new(targets: Vec<WebhookTarget>, db: DatabaseConnection) -> Self {
        Self { targets, db }
    }
}

#[async_trait::async_trait]
impl EventSink for WebhookSink {
    /// Fan out `payload` to every matching target on its own spawned
    /// task and return immediately — see the module docs for why this
    /// never blocks or fails the caller. Always returns `Ok`.
    async fn send(
        &self,
        entity: &str,
        _key: &str,
        payload: &serde_json::Value,
    ) -> Result<(), SinkError> {
        let kind = payload.get("kind").and_then(|k| k.as_str()).unwrap_or("");
        let event_id = payload
            .get("event_id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .unwrap_or_else(uuid::Uuid::nil);
        for target in &self.targets {
            if !target.matches_kind(kind) {
                continue;
            }
            let Some(request) = render(target.format, entity, payload) else {
                tracing::warn!("failed to serialize webhook payload; skipping");
                continue;
            };
            let Some(signature) = mac::tag(Domain::Webhook, &request.body) else {
                // Constructed without a key configured — refuse to send
                // unsigned deliveries rather than silently downgrade.
                tracing::error!(
                    "webhook targets are configured but no integrity-MAC key is active; \
                     skipping delivery rather than sending an unsigned request"
                );
                return Ok(());
            };
            let target = target.clone();
            let entity = entity.to_string();
            let kind = kind.to_string();
            let db = self.db.clone();
            tokio::spawn(async move {
                deliver_with_retry(&db, &target, &entity, &kind, event_id, &signature, &request)
                    .await;
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strict_mode_refuses_private_metadata_and_internal_targets() {
        // Off: the operator's own choice stands.
        assert!(target_allowed("https://10.0.0.5/hook", false));
        assert!(target_allowed("https://169.254.169.254/latest", false));
        assert!(target_allowed("https://billing.internal/hook", false));
        // On: public only.
        assert!(target_allowed("https://ops.example.com/hook", true));
        assert!(!target_allowed("https://10.0.0.5/hook", true));
        assert!(!target_allowed("https://192.168.1.9/hook", true));
        assert!(!target_allowed("https://169.254.169.254/latest", true));
        assert!(!target_allowed("https://[fd00::1]/hook", true));
        assert!(!target_allowed("https://billing.internal/hook", true));
        assert!(!target_allowed(
            "https://user:pw@ops.example.com/hook",
            true
        ));
        // Loopback over http stays a development convenience only when strict is off.
        assert!(target_allowed("http://127.0.0.1:8080/hook", false));
        assert!(!target_allowed("http://127.0.0.1:8080/hook", true));
    }

    use super::{
        WebhookFormat, WebhookTarget, parse_targets, render, should_retry_status, target_allowed,
        url_is_permitted,
    };

    #[test]
    fn https_to_any_host_is_permitted() {
        assert!(url_is_permitted("https://ops.example.com/hook"));
    }

    #[test]
    fn http_to_a_non_loopback_host_is_refused() {
        assert!(!url_is_permitted("http://ops.example.com/hook"));
    }

    #[test]
    fn http_to_loopback_is_permitted_for_dev() {
        assert!(url_is_permitted("http://127.0.0.1:8080/hook"));
        assert!(url_is_permitted("http://localhost:8080/hook"));
    }

    #[test]
    fn an_unparseable_url_is_refused() {
        assert!(!url_is_permitted("not a url"));
    }

    #[test]
    fn a_non_http_scheme_is_refused() {
        assert!(!url_is_permitted("file:///etc/passwd"));
    }

    #[test]
    fn a_5xx_is_retried_and_a_4xx_is_not() {
        // Pins this crate's own T-28m acceptance criterion directly.
        assert!(should_retry_status(500));
        assert!(should_retry_status(503));
        assert!(should_retry_status(599));
        assert!(!should_retry_status(400));
        assert!(!should_retry_status(404));
        assert!(!should_retry_status(429));
        assert!(!should_retry_status(200));
    }

    #[test]
    fn a_target_with_no_kinds_matches_every_kind() {
        let target = WebhookTarget {
            url: "https://example.com/hook".to_string(),
            kinds: None,
            format: WebhookFormat::Native,
        };
        assert!(target.matches_kind("created"));
        assert!(target.matches_kind("merged"));
    }

    #[test]
    fn a_target_with_kinds_matches_only_those() {
        let target = WebhookTarget {
            url: "https://example.com/hook".to_string(),
            kinds: Some(vec!["created".to_string(), "merged".to_string()]),
            format: WebhookFormat::Native,
        };
        assert!(target.matches_kind("created"));
        assert!(target.matches_kind("merged"));
        assert!(!target.matches_kind("updated"));
        assert!(!target.matches_kind("deleted"));
    }

    #[test]
    fn parsing_a_target_list_reads_the_kinds_filter() {
        let json = r#"[
            {"url": "https://ops.example.com/hook"},
            {"url": "https://audit.example.com/hook", "kinds": ["created", "merged"]}
        ]"#;
        let targets: Vec<WebhookTarget> = serde_json::from_str(json).unwrap();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].kinds, None);
        assert_eq!(
            targets[1].kinds,
            Some(vec!["created".to_string(), "merged".to_string()])
        );
    }

    #[test]
    fn parse_targets_drops_a_non_https_entry_and_keeps_the_rest() {
        let targets = parse_targets(
            r#"[{"url": "http://not-loopback.example.com/hook"}, {"url": "https://ok.example.com/hook"}]"#,
        );
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].url, "https://ok.example.com/hook");
    }

    /// Malformed JSON must not block boot — the config loader falls
    /// back to "no targets" rather than propagating a parse error.
    #[test]
    fn parse_targets_on_malformed_json_returns_empty_rather_than_panicking() {
        assert!(parse_targets("not json").is_empty());
    }

    /// An empty/unset value is "no targets configured", not a parse
    /// error — the common "feature not in use" case must be silent.
    #[test]
    fn parse_targets_on_blank_input_returns_empty() {
        assert!(parse_targets("").is_empty());
        assert!(parse_targets("   ").is_empty());
    }

    fn envelope() -> serde_json::Value {
        serde_json::json!({
            "event_id": "5f1d1d5e-6d7e-4a53-9b0a-0d6a2c3f2a11",
            "schema_version": 1,
            "entity": "plan",
            "kind": "created",
            "pid": "0c3f6a1e-93a4-4b1f-8f55-3a0c7e9b2d10",
            "seq": 7,
            "actor": null,
            "name": "Data warehouse"
        })
    }

    /// The `CloudEvents` 1.0 rules a receiver relies on: the four required attributes are present and
    /// non-empty strings, `specversion` is `1.0`, `source` is a URI-reference, `type` is non-empty,
    /// attribute names are lower-case `a-z0-9`, and `time`, when present, is RFC 3339.
    fn assert_valid_ce(attrs: &std::collections::BTreeMap<String, String>) {
        for required in ["specversion", "id", "source", "type"] {
            assert!(
                attrs.get(required).is_some_and(|v| !v.is_empty()),
                "{required} is required and non-empty"
            );
        }
        assert_eq!(attrs["specversion"], "1.0");
        assert!(reqwest::Url::parse(&attrs["source"]).is_ok(), "a URI");
        for name in attrs.keys() {
            assert!(
                name.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "attribute name {name}"
            );
        }
        if let Some(t) = attrs.get("time") {
            assert!(chrono::DateTime::parse_from_rfc3339(t).is_ok(), "RFC 3339");
        }
    }

    #[test]
    fn native_is_the_envelope_unchanged() {
        let r = render(WebhookFormat::Native, "plan", &envelope()).unwrap();
        assert_eq!(r.content_type, "application/json");
        assert!(r.headers.is_empty());
        assert_eq!(r.body, serde_json::to_vec(&envelope()).unwrap());
    }

    #[test]
    fn a_structured_cloudevent_is_valid_and_carries_the_envelope_as_data() {
        let r = render(WebhookFormat::CloudeventsStructured, "plan", &envelope()).unwrap();
        assert_eq!(r.content_type, "application/cloudevents+json");
        let event: serde_json::Value = serde_json::from_slice(&r.body).unwrap();
        let attrs = event
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| k.as_str() != "data")
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();
        assert_valid_ce(&attrs);
        assert_eq!(attrs["id"], "5f1d1d5e-6d7e-4a53-9b0a-0d6a2c3f2a11");
        assert_eq!(
            attrs["type"],
            "org.project-portfolio-management.plan.created"
        );
        assert_eq!(attrs["subject"], "0c3f6a1e-93a4-4b1f-8f55-3a0c7e9b2d10");
        assert_eq!(event["data"], envelope());
        assert!(!attrs.contains_key("time"), "a time is never invented");
    }

    #[test]
    fn a_binary_cloudevent_is_valid_with_the_envelope_as_the_body() {
        let r = render(WebhookFormat::CloudeventsBinary, "plan", &envelope()).unwrap();
        assert_eq!(r.content_type, "application/json");
        assert_eq!(r.body, serde_json::to_vec(&envelope()).unwrap());
        let attrs = r
            .headers
            .iter()
            .map(|(k, v)| {
                (
                    k.strip_prefix("ce-").expect("ce- prefix").to_string(),
                    v.clone(),
                )
            })
            .collect();
        assert_valid_ce(&attrs);
        assert_eq!(
            attrs["type"],
            "org.project-portfolio-management.plan.created"
        );
    }

    #[test]
    fn occurred_at_becomes_the_cloudevents_time_when_the_envelope_has_one() {
        let mut e = envelope();
        e["occurred_at"] = "2026-10-25T06:00:00Z".into();
        let r = render(WebhookFormat::CloudeventsBinary, "plan", &e).unwrap();
        assert!(
            r.headers
                .contains(&("ce-time".to_string(), "2026-10-25T06:00:00Z".to_string()))
        );
    }

    #[test]
    fn the_format_is_read_from_the_target_list_and_defaults_to_native() {
        let raw = r#"[{"url":"https://a.example/h"},
                      {"url":"https://b.example/h","format":"cloudevents_structured"},
                      {"url":"https://c.example/h","format":"cloudevents_binary"}]"#;
        let formats: Vec<WebhookFormat> = parse_targets(raw).iter().map(|t| t.format).collect();
        assert_eq!(
            formats,
            vec![
                WebhookFormat::Native,
                WebhookFormat::CloudeventsStructured,
                WebhookFormat::CloudeventsBinary
            ]
        );
        assert!(parse_targets(r#"[{"url":"https://a.example/h","format":"xml"}]"#).is_empty());
    }
}
