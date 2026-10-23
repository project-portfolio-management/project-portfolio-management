//! Live plan updates (PL-3): the in-process event hub behind
//! `GET /api/plans/{pid}/live`.
//!
//! Every mutation that matters to an open board (a task created, moved or
//! edited; a comment added) publishes one [`Event`]. The hub keeps a **bounded
//! ring** of the most recent events so a client that reconnects with
//! `Last-Event-ID` is replayed what it missed — and when the gap is larger
//! than the ring remembers, it is told to `resync` rather than silently
//! missing events. A subscriber that falls behind the **bounded** broadcast
//! channel is disconnected (told to resync), never buffered without limit.
//!
//! **Scope and limits.** The hub is per process: with several replicas an
//! event reaches only the clients connected to the replica that handled the
//! write. A shared bus (the outbox relay already exists) is the extension.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use serde_json::Value;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Events remembered for replay.
pub const RING: usize = 1_000;
/// Capacity of the live broadcast channel (a slower subscriber lags out).
pub const CHANNEL: usize = 256;

/// One live event.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// Monotonic id, the SSE `id:` (so `Last-Event-ID` resumes).
    pub id: u64,
    /// The plan it concerns.
    pub plan_pid: Uuid,
    /// The SSE `event:` name (`task_created`, `task_moved`, …).
    pub kind: String,
    /// The payload — identifiers and the changed fact, no more.
    pub data: Value,
}

/// What a reconnecting client is owed.
#[derive(Debug, Clone, PartialEq)]
pub enum Replay {
    /// These events, oldest first (possibly none).
    Events(Vec<Event>),
    /// The gap is larger than the ring remembers: refetch instead.
    Resync,
}

/// The hub.
pub struct Hub {
    inner: Mutex<Inner>,
    tx: broadcast::Sender<Event>,
}

struct Inner {
    next: u64,
    ring: VecDeque<Event>,
    cap: usize,
}

impl Hub {
    /// A hub remembering `cap` events.
    #[must_use]
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(CHANNEL);
        Self {
            inner: Mutex::new(Inner {
                next: 1,
                ring: VecDeque::new(),
                cap: cap.max(1),
            }),
            tx,
        }
    }

    /// Record and broadcast an event; returns its id.
    pub fn publish(&self, plan_pid: Uuid, kind: &str, data: Value) -> u64 {
        let mut g = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let id = g.next;
        g.next += 1;
        let event = Event {
            id,
            plan_pid,
            kind: kind.to_string(),
            data,
        };
        g.ring.push_back(event.clone());
        while g.ring.len() > g.cap {
            g.ring.pop_front();
        }
        // No subscribers is fine: `send` errors only when nobody listens.
        let _ = self.tx.send(event);
        id
    }

    /// Subscribe to events published from now on.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    /// The newest id issued so far (0 if none).
    #[must_use]
    pub fn latest(&self) -> u64 {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .next
            - 1
    }

    /// What a client that last saw `last_seen` for `plan` is owed. If the
    /// oldest remembered event is newer than `last_seen + 1`, an event may
    /// have been forgotten: `Resync`.
    #[must_use]
    pub fn replay(&self, plan: Uuid, last_seen: u64) -> Replay {
        let g = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(oldest) = g.ring.front()
            && oldest.id > last_seen + 1
        {
            return Replay::Resync;
        }
        Replay::Events(
            g.ring
                .iter()
                .filter(|e| e.id > last_seen && e.plan_pid == plan)
                .cloned()
                .collect(),
        )
    }
}

static HUB: OnceLock<Hub> = OnceLock::new();

/// The process-wide hub.
#[must_use]
pub fn hub() -> &'static Hub {
    HUB.get_or_init(|| Hub::new(RING))
}

/// Publish on the process-wide hub (best-effort, never fails the caller).
pub fn publish(plan_pid: Uuid, kind: &str, data: Value) {
    hub().publish(plan_pid, kind, data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn p(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    #[expect(clippy::many_single_char_names)] // a, b, c are just three events in a row
    fn ids_are_monotonic_and_replay_is_per_plan_and_ordered() {
        let h = Hub::new(10);
        let a = h.publish(p(1), "task_created", json!({}));
        let b = h.publish(p(2), "task_created", json!({}));
        let c = h.publish(p(1), "task_moved", json!({}));
        assert!(a < b && b < c);
        let Replay::Events(e) = h.replay(p(1), 0) else {
            panic!("events")
        };
        assert_eq!(
            e.iter().map(|x| x.id).collect::<Vec<_>>(),
            [a, c],
            "only plan 1, oldest first"
        );
        let Replay::Events(e) = h.replay(p(1), a) else {
            panic!("events")
        };
        assert_eq!(e.len(), 1, "resumes after the last seen id");
        assert_eq!(h.latest(), c);
    }

    #[test]
    fn a_gap_larger_than_the_ring_says_resync_not_silence() {
        let h = Hub::new(3);
        for _ in 0..10 {
            h.publish(p(1), "x", json!({}));
        }
        // Ring holds ids 8..=10. A client that last saw 2 missed 3..=7.
        assert_eq!(h.replay(p(1), 2), Replay::Resync);
        // A client that saw 7 missed nothing the ring forgot.
        let Replay::Events(e) = h.replay(p(1), 7) else {
            panic!("events")
        };
        assert_eq!(e.len(), 3);
        assert_eq!(h.replay(p(1), 10), Replay::Events(vec![]), "caught up");
    }

    #[tokio::test]
    async fn subscribers_receive_live_events_and_a_slow_one_lags_instead_of_growing() {
        let h = Hub::new(10);
        let mut rx = h.subscribe();
        h.publish(p(1), "task_created", json!({ "t": 1 }));
        let got = rx.recv().await.unwrap();
        assert_eq!((got.kind.as_str(), got.plan_pid), ("task_created", p(1)));
        // Overflow the bounded channel without reading.
        for _ in 0..(CHANNEL + 50) {
            h.publish(p(1), "x", json!({}));
        }
        assert!(matches!(
            rx.recv().await,
            Err(broadcast::error::RecvError::Lagged(_))
        ));
    }
}
