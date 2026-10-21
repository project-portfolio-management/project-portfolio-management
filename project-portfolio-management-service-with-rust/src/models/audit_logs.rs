//! `audit_logs` model — record and query the CRUD audit trail.

use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, QuerySelect};
use uuid::Uuid;

pub use super::_entities::audit_logs::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for super::_entities::audit_logs::ActiveModel {}

impl Model {
    /// Record one audit entry. `actor` is the caller's `sub` (user `pid`)
    /// when a verified token was presented, else `None`.
    ///
    /// Generic over [`ConnectionTrait`] so it runs either on the pooled
    /// `&DatabaseConnection` (the best-effort side-channel path, `memory`
    /// transport) **or** on a `&DatabaseTransaction` — under the `outbox`
    /// transport the audit row is written in the *same* transaction as the
    /// entity mutation and its `event_outbox` row, so the three can never
    /// disagree (`agents/share/event-bus.md` §3).
    ///
    /// # Errors
    ///
    /// When the insert fails.
    pub async fn record<C: ConnectionTrait>(
        db: &C,
        entity_pid: Uuid,
        action: &str,
        actor: Option<&str>,
        snapshot: Option<serde_json::Value>,
    ) -> ModelResult<Self> {
        // Set `created_at` explicitly rather than letting the database
        // default it: the timestamp is part of the MAC pre-image, so it
        // must be known before the insert. The alternative — insert, then
        // stamp — leaves a window in which the row exists unprotected.
        let created_at: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        // All three digests from one call, so none can be stamped
        // without the others. The two unkeyed ones are written
        // unconditionally; only the MAC depends on a configured key.
        let digests = crate::compliance::audit_integrity::digests(
            &crate::compliance::audit_integrity::AuditInput {
                entity_pid,
                action,
                actor,
                snapshot: snapshot.as_ref(),
                created_at_micros: created_at.timestamp_micros(),
            },
        );
        let entry = audit_logs::ActiveModel {
            entity_pid: ActiveValue::set(entity_pid),
            action: ActiveValue::set(action.to_string()),
            actor: ActiveValue::set(actor.map(ToString::to_string)),
            snapshot: ActiveValue::set(snapshot),
            created_at: ActiveValue::set(created_at),
            hash: ActiveValue::set(Some(digests.sha256)),
            hash_sha3: ActiveValue::set(Some(digests.sha3)),
            mac: ActiveValue::set(digests.mac),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(entry)
    }

    /// [`Self::record`] for a write that is kept even when its audit row cannot be written (CQ-3).
    /// The failure is never dropped silently: it is logged at error level, with the entity and the
    /// action, and counted in `audit_write_failed_total`, which an operator can alert on. Whether
    /// such a write should be refused instead is plan §70 question 2.
    pub async fn record_or_log<C: ConnectionTrait>(
        db: &C,
        entity_pid: Uuid,
        action: &str,
        actor: Option<&str>,
        snapshot: Option<serde_json::Value>,
    ) {
        if let Err(error) = Self::record(db, entity_pid, action, actor, snapshot).await {
            crate::metrics::Metrics::global()
                .audit_write_failed_total
                .inc();
            tracing::error!(%entity_pid, action, %error, "audit row not written");
        }
    }

    /// Most-recent audit entries, capped at `limit`.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn recent(db: &DatabaseConnection, limit: u64) -> ModelResult<Vec<Self>> {
        let rows = audit_logs::Entity::find()
            .order_by_desc(audit_logs::Column::Id)
            .limit(limit)
            .all(db)
            .await?;
        Ok(rows)
    }

    /// Audit entries for one plan, most-recent first.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn for_entity(db: &DatabaseConnection, entity_pid: Uuid) -> ModelResult<Vec<Self>> {
        let rows = audit_logs::Entity::find()
            .filter(audit_logs::Column::EntityPid.eq(entity_pid))
            .order_by_desc(audit_logs::Column::Id)
            .all(db)
            .await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    /// CQ-3: no audit write throws its error away. A call either passes the error on (`?`,
    /// `map_err`) or goes through `record_or_log`, which logs it and counts it.
    #[test]
    fn no_source_file_discards_an_audit_error() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![root];
        let mut offenders = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "rs") {
                    continue;
                }
                let text = std::fs::read_to_string(&path).expect("read file");
                for (at, _) in text.match_indices("Model::record(") {
                    let statement = text[at..].split(';').next().unwrap_or("");
                    let squashed: String = statement.split_whitespace().collect();
                    if squashed.ends_with(".ok()") {
                        let line = text[..at].matches('\n').count() + 1;
                        offenders.push(format!("{}:{line}", path.display()));
                    }
                }
            }
        }
        assert!(offenders.is_empty(), "audit error discarded: {offenders:?}");
    }
}
