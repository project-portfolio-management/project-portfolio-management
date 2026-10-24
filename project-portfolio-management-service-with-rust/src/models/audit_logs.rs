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

    /// NFR-6 / CQ-3: a write's audit row is written in the write's own transaction and a failure refuses the
    /// write. `record_or_log` (log, count, keep) is what remains for audits of something **already done with
    /// no caller to refuse**, or that is not a change: the audit of an automation run or a scheduled action
    /// that has fired, of a background job's end, of a report run that failed, the warning that a register is
    /// incomplete, and the record that a scan refused an upload. This list can only shrink: a file's count may
    /// go down (then lower it here), never up, and a file not named here may not call it.
    #[test]
    fn the_writes_that_still_keep_a_failed_audit_only_shrink() {
        const KNOWN: &[(&str, usize)] = &[
            ("attachments.rs", 1),
            ("automation.rs", 2),
            ("bulk.rs", 2),
            ("register.rs", 1),
            ("report_schedules.rs", 1),
        ];
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controllers");
        let mut faults = Vec::new();
        for entry in std::fs::read_dir(&dir).expect("read controllers") {
            let path = entry.expect("entry").path();
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let name = path
                .file_name()
                .expect("name")
                .to_string_lossy()
                .to_string();
            let text = std::fs::read_to_string(&path).expect("read file");
            let found = text.matches("record_or_log(").count();
            let allowed = KNOWN
                .iter()
                .find(|(f, _)| *f == name)
                .map_or(0, |(_, n)| *n);
            if found > allowed {
                faults.push(format!("{name}: {found} calls, {allowed} allowed: put the audit in the write's transaction (record + map_err(audit_failed))"));
            } else if found < allowed {
                faults.push(format!(
                    "{name}: {found} calls, {allowed} listed: lower the number in KNOWN"
                ));
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// A handler that holds a transaction must not ask the pool for a second connection: with one connection
    /// (the test and development default) the second request waits for the first, which is waiting for it,
    /// and under load the same thing starves the pool. Between `let txn = ctx.db.begin()` and `txn.commit()`
    /// every database call goes through `txn`, so the word `ctx` may not appear there.
    #[test]
    fn no_controller_uses_the_pool_while_it_holds_a_transaction() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controllers");
        let mut faults = Vec::new();
        for entry in std::fs::read_dir(&dir).expect("read controllers") {
            let path = entry.expect("entry").path();
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read file");
            for (at, _) in text.match_indices("= ctx.db.begin()") {
                let Some(open) = text[..at].rfind("let ") else {
                    continue;
                };
                let var = text[open + 4..at].trim().to_string();
                // The transaction ends at its commit, its rollback, or when it is dropped.
                let Some(close) = [
                    format!("{var}.commit()"),
                    format!("{var}.rollback()"),
                    format!("drop({var})"),
                ]
                .iter()
                .filter_map(|end| text[at..].find(end))
                .min() else {
                    continue;
                };
                let window = &text[at + 16..at + close];
                for (n, line) in window.lines().enumerate() {
                    let code = line.split("//").next().unwrap_or("");
                    let uses_ctx = code
                        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                        .any(|w| w == "ctx");
                    if uses_ctx {
                        let line_no = text[..at].matches('\n').count() + 1 + n;
                        faults.push(format!("{}:{line_no}: {}", path.display(), line.trim()));
                    }
                }
            }
        }
        assert!(
            faults.is_empty(),
            "the pool is used while a transaction is open: {faults:#?}"
        );
    }
}
