//! The durable event bus's outbox events, kept in `axa-outbox`'s record (`axa_outbox_events`, `axa_outbox_payloads`,
//! `axa_outbox_attempts`; `agents/share/event-bus.md` §3, §5). Every event is kept: the record refuses changes.
//!
//! [`event`] is the **pure** envelope-to-event mapping, unit-tested with no database. [`write`] puts the event on
//! the handler's transaction, so the change and its event commit or roll back together; it takes a transaction and
//! not a connection, as an event written outside one is the bug the outbox exists to prevent. [`recent`] reads the
//! last rows for the operator's `/events/recent`, and locks nothing. The relay is [`crate::relay`].
//!
//! The row's `topic` is the envelope's entity, `kind` its change kind, `key` the record's pid (which keeps one
//! record's events in order) and `payload` the whole envelope, so a consumer sees the same §4 shape as before. The
//! envelope's own `event_id` is the row's, which a consumer dedupes on.

use crate::prelude::*;
use axa::outbox::Event;
use uuid::Uuid;

use crate::streaming::{Envelope, EventKind, EventView};

/// The wire token for an [`EventKind`] (matches the envelope's lowercase serde form).
const fn kind_token(kind: EventKind) -> &'static str {
    match kind {
        EventKind::Created => "created",
        EventKind::Updated => "updated",
        EventKind::Deleted => "deleted",
        EventKind::Merged => "merged",
    }
}

/// The outbox event for an envelope: its id, its entity as the topic, its kind, its pid as the key, and the whole
/// envelope as the payload. Pure and database-free.
///
/// # Errors
///
/// When `env.pid` is not a UUID, or the envelope fails to serialize.
pub fn event(env: &Envelope) -> ModelResult<Event> {
    let pid = Uuid::parse_str(&env.pid).map_err(|e| ModelError::Any(Box::new(e)))?;
    let payload = serde_json::to_value(env).map_err(|e| ModelError::Any(Box::new(e)))?;
    Ok(Event {
        event_id: env.event_id,
        topic: env.entity.to_string(),
        kind: kind_token(env.kind).to_string(),
        key: pid.to_string(),
        payload,
    })
}

/// Write the event for `env` **on the handler's transaction**.
///
/// # Errors
///
/// As [`event`], or when the insert fails (a repeated `event_id` among them).
pub async fn write(tx: &mut crate::db::Tx, env: &Envelope) -> Result<()> {
    tx.write_outbox(&event(env)?).await?;
    Ok(())
}

/// The most recent outbox rows, **newest first**, projected to the frozen operator [`EventView`]
/// (`{kind, pid, name, seq}`). A row whose payload is not an envelope is skipped rather than failing the read.
///
/// # Errors
///
/// When the query fails.
pub async fn recent(ctx: &AppContext, limit: u64) -> Result<Vec<EventView>> {
    let payloads: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT p.payload FROM axa_outbox_events e JOIN axa_outbox_payloads p ON p.event_id = e.event_id \
         WHERE p.payload IS NOT NULL ORDER BY e.id DESC LIMIT $1",
    )
    .bind(i64::try_from(limit).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(ctx))
    .await?;
    Ok(payloads
        .into_iter()
        .filter_map(|payload| serde_json::from_value::<Envelope>(payload).ok())
        .map(|env| EventView::from(&env))
        .collect())
}

/// DB-free pins for the pure envelope-to-event mapping.
#[cfg(test)]
mod tests {
    use super::{event, kind_token};
    use crate::streaming::{ENTITY, Envelope, EventKind, SCHEMA_VERSION};
    use uuid::Uuid;

    fn an_envelope(kind: EventKind, pid: &str) -> Envelope {
        Envelope {
            event_id: Uuid::parse_str("22222222-2222-4222-8222-222222222222").unwrap(),
            schema_version: SCHEMA_VERSION,
            entity: ENTITY,
            kind,
            pid: pid.to_string(),
            seq: 7,
            actor: Some("user-1".to_string()),
            name: "Apollo platform migration".to_string(),
        }
    }

    #[test]
    fn an_envelope_becomes_one_event_and_keeps_the_whole_envelope() {
        let pid = "0c4f1e2a-0000-4000-8000-000000000000";
        let env = an_envelope(EventKind::Created, pid);
        let event = event(&env).unwrap();
        assert_eq!(event.event_id, env.event_id, "the consumer's dedupe id");
        assert_eq!(event.topic, "plan");
        assert_eq!(event.kind, "created");
        assert_eq!(event.key, pid, "one record's events stay in order");
        for (field, value) in [
            ("pid", serde_json::json!(pid)),
            ("seq", serde_json::json!(7)),
            ("name", serde_json::json!("Apollo platform migration")),
            ("entity", serde_json::json!("plan")),
            ("kind", serde_json::json!("created")),
            ("actor", serde_json::json!("user-1")),
            ("schema_version", serde_json::json!(1)),
        ] {
            assert_eq!(event.payload[field], value, "{field}");
        }
    }

    #[test]
    fn merged_and_deleted_events_keep_their_kind() {
        let pid = "0c4f1e2a-0000-4000-8000-000000000001";
        for kind in [EventKind::Merged, EventKind::Deleted] {
            let event = event(&an_envelope(kind, pid)).unwrap();
            assert_eq!(event.kind, kind_token(kind));
            assert_eq!(event.payload["kind"], kind_token(kind));
        }
    }

    #[test]
    fn a_pid_that_is_not_a_uuid_is_refused() {
        assert!(event(&an_envelope(EventKind::Created, "not-a-uuid")).is_err());
    }

    #[test]
    fn kind_tokens_match_the_envelope_serde_form() {
        assert_eq!(kind_token(EventKind::Created), "created");
        assert_eq!(kind_token(EventKind::Updated), "updated");
        assert_eq!(kind_token(EventKind::Deleted), "deleted");
        assert_eq!(kind_token(EventKind::Merged), "merged");
    }
}
