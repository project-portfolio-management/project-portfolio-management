//! `audit_logs` model — record and query the CRUD audit trail.

use crate::prelude::*;
use uuid::Uuid;

pub use super::_entities::audit_logs::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for super::_entities::audit_logs::ActiveModel {}

/// The time and the digests of an audit row about to be written.
///
/// `created_at` is set here rather than left to the database's default: the timestamp is part of the MAC pre-image, so
/// it must be known before the insert. The alternative, insert then stamp, leaves a window in which the row exists
/// unprotected. All three digests come from one call, so none can be stamped without the others. The two unkeyed ones
/// are written unconditionally; only the MAC depends on a configured key.
fn stamped(
    entity_pid: Uuid,
    action: &str,
    actor: Option<&str>,
    snapshot: Option<&serde_json::Value>,
) -> (
    chrono::DateTime<chrono::FixedOffset>,
    crate::compliance::audit_integrity::Digests,
) {
    let created_at: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    let digests = crate::compliance::audit_integrity::digests(
        &crate::compliance::audit_integrity::AuditInput {
            entity_pid,
            action,
            actor,
            snapshot,
            created_at_micros: created_at.timestamp_micros(),
        },
    );
    (created_at, digests)
}

impl Model {
    /// Record one audit entry, on the pool ([`crate::db::pool`]) or a transaction ([`crate::db::Tx::conn`]), so the
    /// audit row can share the change's transaction. `actor` is the caller's `sub` (user `pid`) when a verified token
    /// was presented, else `None`. Under the `outbox` transport the audit row is written in the *same* transaction as
    /// the entity mutation and its `axa_outbox` row, so the three can never disagree (`agents/share/event-bus.md` §3).
    /// (The `SeaORM` `record` went when its last callers, `retention` and `scheduled_jobs`, moved: SQ-1 batch 18.)
    ///
    /// # Errors
    ///
    /// When the insert fails: the `sqlx` error itself (`Error::Sqlx`), so a database that cannot serve still answers
    /// `503` `unavailable` with `Retry-After` and a broken constraint its `409` or `422`, as the `SeaORM` `record`'s
    /// error did, where the caller passes it on (`?`). Wrapped as a plain model error it would be a `500`.
    pub async fn record_on<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        db: E,
        entity_pid: Uuid,
        action: &str,
        actor: Option<&str>,
        snapshot: Option<serde_json::Value>,
    ) -> Result<Self> {
        let (created_at, digests) = stamped(entity_pid, action, actor, snapshot.as_ref());
        sqlx::query_as(
            "INSERT INTO audit_logs
                 (entity_pid, action, actor, snapshot, created_at, hash, hash_sha3, mac)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING *",
        )
        .bind(entity_pid)
        .bind(action)
        .bind(actor)
        .bind(snapshot)
        .bind(created_at)
        .bind(digests.sha256)
        .bind(digests.sha3)
        .bind(digests.mac)
        .fetch_one(db)
        .await
        .map_err(Error::from)
    }

    /// [`Self::record_on`] for a write that is kept even when its audit row cannot be written (CQ-3), on the pool
    /// ([`crate::db::pool`]) or a transaction ([`crate::db::Tx::conn`]). The failure is never dropped silently: it is
    /// logged at error level, with the entity and the action, and counted in `audit_write_failed_total`, which an
    /// operator can alert on. Whether such a write should be refused instead is plan §70 question 2.
    pub async fn record_or_log_on<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        db: E,
        entity_pid: Uuid,
        action: &str,
        actor: Option<&str>,
        snapshot: Option<serde_json::Value>,
    ) {
        if let Err(error) = Self::record_on(db, entity_pid, action, actor, snapshot).await {
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
    pub async fn recent(ctx: &AppContext, limit: u64) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM audit_logs ORDER BY id DESC LIMIT $1")
                .bind(i64::try_from(limit).unwrap_or(i64::MAX))
                .fetch_all(crate::db::pool(ctx))
                .await?,
        )
    }

    /// Audit entries for one plan, most-recent first.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn for_entity(ctx: &AppContext, entity_pid: Uuid) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM audit_logs WHERE entity_pid = $1 ORDER BY id DESC")
                .bind(entity_pid)
                .fetch_all(crate::db::pool(ctx))
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    /// A database that cannot serve answers `503` `unavailable` with `Retry-After: 5` (spec §8.3a, NFR-7) through
    /// `record_on`'s error too, as it did through `record`'s `SeaORM` error: a caller that passes it on with `?` (the
    /// outbox transport's audit row, the plan export's) must not turn an outage into a `500`. The pool here points at a
    /// port nothing listens on, so the insert fails the way an unreachable database does. A database that refused the
    /// statement is still the service's own `500`.
    #[tokio::test]
    async fn an_unavailable_database_through_record_on_answers_503_with_retry_after() {
        use axum::{http::StatusCode, response::IntoResponse};
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(500))
            .connect_lazy("postgres://nobody:nothing@127.0.0.1:1/none")
            .expect("a lazy pool");
        let error = super::Model::record_on(&pool, uuid::Uuid::nil(), "created", None, None)
            .await
            .expect_err("nothing answers on port 1");
        assert!(
            matches!(error, axa::Error::Sqlx(_)),
            "the sqlx error itself, not wrapped: {error:?}"
        );
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(response.headers()["retry-after"], "5");
        // The same error type, from a statement the database refused, is not an outage.
        let refused: axa::Error = sqlx::Error::RowNotFound.into();
        assert_eq!(
            refused.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    /// CQ-3: no audit write throws its error away. A call either passes the error on (`?`,
    /// `map_err`) or goes through `record_or_log_on`, which logs it and counts it.
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
                // The sqlx twin too, so moving a call to it cannot slip past this check.
                let calls = text
                    .match_indices("Model::record(")
                    .chain(text.match_indices("Model::record_on("));
                for (at, _) in calls {
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
    /// write. `record_or_log_on` (log, count, keep) is what remains for audits of something **already done with
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
            // The sqlx twin counts too, so moving a call to it cannot slip past this list.
            let found =
                text.matches("record_or_log(").count() + text.matches("record_or_log_on(").count();
            let allowed = KNOWN
                .iter()
                .find(|(f, _)| *f == name)
                .map_or(0, |(_, n)| *n);
            if found > allowed {
                faults.push(format!("{name}: {found} calls, {allowed} allowed: put the audit in the write's transaction (record_on + map_err(audit_failed))"));
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
    /// and under load the same thing starves the pool. Between `let mut tx = db::begin(&ctx)` (or `ctx.db.begin()`)
    /// and `tx.commit()` every database call goes through `tx`, so the word `ctx` may not appear there, after the
    /// line that opens it. Every file under `src/`, `let mut` included. (Until SQ-1 batch 18 this read only
    /// `src/controllers/`, without its subdirectories, for `ctx.db.begin()` alone, which no code calls now.)
    #[test]
    fn nothing_uses_the_pool_while_it_holds_a_transaction() {
        const OPENS: [&str; 3] = ["= ctx.db.begin()", "= crate::db::begin(", "= db::begin("];
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![root];
        let (mut faults, mut seen) = (Vec::new(), 0_usize);
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "rs")
                    || path.ends_with("models/audit_logs.rs")
                {
                    continue;
                }
                let text = std::fs::read_to_string(&path).expect("read file");
                for open_with in OPENS {
                    for (at, _) in text.match_indices(open_with) {
                        let Some(open) = text[..at].rfind("let ") else {
                            continue;
                        };
                        let var = text[open + 4..at].trim().trim_start_matches("mut ").trim();
                        // The transaction ends at its commit, its rollback, or when it is dropped.
                        let Some(close) = [
                            format!("{var}.commit()"),
                            format!("{var}.rollback()"),
                            format!("drop({var})"),
                        ]
                        .iter()
                        .filter_map(|end| text[at..].find(end.as_str()))
                        .min() else {
                            continue;
                        };
                        seen += 1;
                        // From the line after the one that opens it: that line names the context to open it.
                        let window = &text[at..at + close];
                        let Some((_, window)) = window.split_once('\n') else {
                            continue;
                        };
                        for (n, line) in window.lines().enumerate() {
                            let code = line.split("//").next().unwrap_or("");
                            let uses_ctx = code
                                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                                .any(|w| w == "ctx");
                            if uses_ctx {
                                let line_no = text[..at].matches('\n').count() + 2 + n;
                                faults.push(format!(
                                    "{}:{line_no}: {}",
                                    path.display(),
                                    line.trim()
                                ));
                            }
                        }
                    }
                }
            }
        }
        assert!(
            seen > 100,
            "the census found only {seen} transactions: has the way to open one changed?"
        );
        assert!(
            faults.is_empty(),
            "the pool is used while a transaction is open: {faults:#?}"
        );
    }
}
