//! Live plan updates (PL-3): `GET /api/plans/{pid}/live`, a
//! `text/event-stream` of what changes on a plan's board. The hub, its
//! bounded replay ring and the lag policy are in [`crate::live`].
//!
//! The plan is read-checked **once, at connect** (record-level ABAC and
//! private plans: a plan the caller cannot see is `404` before any event);
//! a plan that no longer exists is `404` too. A reconnecting client sends
//! `Last-Event-ID` and is replayed what it missed, or told to `resync` when
//! the gap is larger than the ring remembers. A subscriber that falls behind
//! is told to `resync` and disconnected — never buffered without bound.
//!
//! Two optional parameters make the stream usable by clients that cannot hold
//! a connection open: `limit` closes it after that many events, and `idle_ms`
//! closes it after that long with nothing to send.

use std::time::Duration;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use loco_rs::prelude::*;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;

use super::discussion::resolve_subject;
use crate::auth::MaybeAuthUser;
use crate::live::{self, Replay};

/// `GET /api/plans/{pid}/live` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LiveParams {
    /// Close after this many events.
    #[serde(default)]
    limit: Option<usize>,
    /// Close after this many milliseconds with nothing to send.
    #[serde(default)]
    idle_ms: Option<u64>,
}

fn sse_of(e: &live::Event) -> SseEvent {
    SseEvent::default()
        .id(e.id.to_string())
        .event(e.kind.clone())
        .data(
            serde_json::json!({ "plan_pid": e.plan_pid, "kind": e.kind, "data": e.data })
                .to_string(),
        )
}

/// `GET /api/plans/{pid}/live`.
#[debug_handler]
async fn live_stream(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Query(params): Query<LiveParams>,
) -> Result<axum::response::Response> {
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let plan = subject.pid;
    let last_seen: Option<u64> = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse().ok());

    // Subscribe BEFORE replaying, so nothing published in between is lost;
    // anything both replayed and received is skipped by id.
    let mut rx = live::hub().subscribe();
    let replay = last_seen.map(|id| live::hub().replay(plan, id));
    let limit = params.limit;
    let idle = params.idle_ms.map(Duration::from_millis);

    let stream = async_stream::stream! {
        let mut sent = 0usize;
        let mut high_water = last_seen.unwrap_or(0);
        match replay {
            Some(Replay::Resync) => {
                yield Ok::<_, std::convert::Infallible>(
                    SseEvent::default().event("resync").data("{\"reason\":\"gap\"}"),
                );
            }
            Some(Replay::Events(events)) => {
                for e in events {
                    high_water = high_water.max(e.id);
                    yield Ok(sse_of(&e));
                    sent += 1;
                    if limit.is_some_and(|l| sent >= l) {
                        return;
                    }
                }
            }
            None => {}
        }
        loop {
            let next = match idle {
                Some(d) => match tokio::time::timeout(d, rx.recv()).await {
                    Ok(r) => r,
                    Err(_) => return, // idle: close, the client reconnects
                },
                None => rx.recv().await,
            };
            match next {
                Ok(e) if e.plan_pid == plan && e.id > high_water => {
                    high_water = e.id;
                    yield Ok(sse_of(&e));
                    sent += 1;
                    if limit.is_some_and(|l| sent >= l) {
                        return;
                    }
                }
                Ok(_) => {}
                Err(RecvError::Lagged(_)) => {
                    // Fell behind the bounded channel: say so and hang up.
                    yield Ok(SseEvent::default().event("resync").data("{\"reason\":\"lagged\"}"));
                    return;
                }
                Err(RecvError::Closed) => return,
            }
        }
    };
    Ok(Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response())
}

/// The live-update routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/live", get(live_stream))
}
