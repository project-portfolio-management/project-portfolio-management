//! `outbox_retry_failed` task: owe the event outbox's `failed` rows again (`axa::outbox::Outbox::retry_failed`).
//!
//! ```text
//! cargo run -- task outbox_retry_failed              # dry run: lists the failed rows, changes nothing
//! cargo run -- task outbox_retry_failed apply:true   # every failed row back to pending, with its attempts reset
//! ```
//!
//! A row is `failed` when its sends ran out of attempts (`ppm_event_outbox_rows{status="failed"}` in
//! `/metrics.prom`). No relay touches it again until this runs, so fix the cause first (the broker, a sink that
//! hangs), read the dry run, then apply. A retried row goes out in its record's order, and a consumer dedupes on its
//! `event_id`, so a row the broker had in fact taken is harmless to send again.

use crate::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};

/// The most failed rows the dry run lists; the count covers them all.
pub const LISTED: i64 = 50;

/// One failed row, as the dry run lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedRow {
    /// The row's id in `axa_outbox`.
    pub id: i64,
    /// Its topic (the entity).
    pub topic: String,
    /// Its change kind.
    pub kind: String,
    /// Sends tried.
    pub attempts: i32,
    /// The last send's error, as the crate kept it.
    pub error: Option<String>,
}

/// What one run found and did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Failed rows found.
    pub failed: i64,
    /// The first [`LISTED`] of them, oldest first.
    pub listed: Vec<FailedRow>,
    /// Rows put back to pending; `0` on a dry run.
    pub retried: u64,
}

/// Whether the arguments ask to apply: `apply:true` (or `yes`, `1`); anything else is a dry run.
///
/// # Errors
///
/// An argument other than `apply`.
pub fn applies<'a>(args: impl IntoIterator<Item = (&'a String, &'a String)>) -> Result<bool> {
    let mut apply = false;
    for (key, value) in args {
        if key == "apply" {
            apply = matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "true" | "yes" | "1"
            );
        } else {
            return Err(Error::string(&format!("unknown argument {key:?}")));
        }
    }
    Ok(apply)
}

/// List the failed rows and, when `apply`, owe them all again on a connection of the crate's own to `uri`.
///
/// # Errors
///
/// When a statement fails, or the outbox cannot be reached.
pub async fn run(db: &DatabaseConnection, uri: &str, apply: bool) -> Result<Report> {
    let failed: i64 = db
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT count(*) AS failed FROM axa_outbox WHERE status = 'failed'",
        ))
        .await?
        .map(|row| row.try_get("", "failed"))
        .transpose()?
        .unwrap_or(0);
    let listed = db
        .query_all_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT id, topic, kind, attempts, error FROM axa_outbox WHERE status = 'failed' ORDER BY id LIMIT $1",
            [LISTED.into()],
        ))
        .await?
        .iter()
        .map(|row| {
            Ok(FailedRow {
                id: row.try_get("", "id")?,
                topic: row.try_get("", "topic")?,
                kind: row.try_get("", "kind")?,
                attempts: row.try_get("", "attempts")?,
                error: row.try_get("", "error")?,
            })
        })
        .collect::<std::result::Result<Vec<_>, sea_orm::DbErr>>()?;
    let retried = if apply && failed > 0 {
        axa::outbox::Outbox::connect_existing(uri, crate::relay::options())
            .await
            .map_err(|e| Error::string(&format!("axa_outbox: {e}")))?
            .retry_failed()
            .await
            .map_err(|e| Error::string(&format!("axa_outbox: {e}")))?
    } else {
        0
    };
    Ok(Report {
        failed,
        listed,
        retried,
    })
}

/// The `outbox_retry_failed` CLI task.
pub struct OutboxRetryFailed;

#[async_trait]
impl Task for OutboxRetryFailed {
    /// Task metadata for the command line.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "outbox_retry_failed".to_string(),
            detail: "Send the event outbox's failed rows again. Dry run unless apply:true. Args: [apply:true]."
                .to_string(),
        }
    }

    /// List the failed rows, and owe them again with `apply:true`.
    ///
    /// # Errors
    ///
    /// An unknown argument, a failed statement, or an outbox that cannot be reached.
    async fn run(&self, ctx: &AppContext, vars: &task::Vars) -> Result<()> {
        let apply = applies(&vars.cli)?;
        let report = run(&ctx.db, &ctx.config.database.uri, apply).await?;
        println!(
            "{}: {} failed",
            if apply {
                "outbox retry (APPLIED)"
            } else {
                "outbox retry (dry run: nothing changed; add apply:true)"
            },
            report.failed
        );
        for row in &report.listed {
            println!(
                "  {:>10} {:<12} {:<8} {:>2} attempts  {}",
                row.id,
                row.topic,
                row.kind,
                row.attempts,
                row.error.as_deref().unwrap_or("")
            );
        }
        if report.failed > LISTED {
            println!("  ... and {} more", report.failed - LISTED);
        }
        if apply {
            println!("{} back to pending", report.retried);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn args(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn it_is_a_dry_run_unless_asked_to_apply() {
        assert!(!applies(&args(&[])).unwrap());
        assert!(!applies(&args(&[("apply", "false")])).unwrap());
        assert!(!applies(&args(&[("apply", "maybe")])).unwrap());
        for yes in ["true", "TRUE", "yes", "1"] {
            assert!(applies(&args(&[("apply", yes)])).unwrap(), "{yes}");
        }
    }

    #[test]
    fn an_unknown_argument_is_refused() {
        assert!(applies(&args(&[("aply", "true")])).is_err());
    }

    #[test]
    fn it_is_named_for_the_command_line() {
        assert_eq!(OutboxRetryFailed.task().name, "outbox_retry_failed");
    }
}
