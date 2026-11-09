//! `outbox_retry_failed` task: put the event outbox's `failed` events back to be sent (`axa::outbox::release`).
//!
//! ```text
//! cargo run -- task outbox_retry_failed                                       # dry run: lists them, changes nothing
//! cargo run -- task outbox_retry_failed apply:true by:<who> reason:<code>     # every failed event back, recorded
//! ```
//!
//! An event is `failed` when its sends ran out of attempts (`ppm_event_outbox_rows{status="failed"}` in
//! `/metrics.prom`). No relay touches it again until a person releases it, so fix the cause first (the broker, a sink
//! that hangs), read the dry run, then apply. Each release is kept in the outbox's record as a `released` attempt
//! naming `by` and `reason`: an id for the person and a short code (1 to 200 characters, no line breaks), such as
//! `by:ops-jane reason:broker_restored`. All are released in one transaction, so either every one goes back or none
//! does. A released event can arrive after later events for its key, and a consumer dedupes on its `event_id`, so one
//! the broker had in fact taken is harmless to send again.

use crate::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};

/// The most failed events the dry run lists; the count covers them all.
pub const LISTED: i64 = 50;

/// One failed event, as the dry run lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedRow {
    /// The event's row id in the record.
    pub id: i64,
    /// Its topic (the entity).
    pub topic: String,
    /// Its change kind.
    pub kind: String,
    /// Attempts it was charged.
    pub attempts: i32,
    /// The code the last failed attempt kept.
    pub error: Option<String>,
}

/// What one run found and did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// Failed events found.
    pub failed: i64,
    /// The first [`LISTED`] of them, oldest first.
    pub listed: Vec<FailedRow>,
    /// Events released; `0` on a dry run.
    pub retried: u64,
}

/// Who released the events and why, as the record keeps them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// An id for the person.
    pub by: String,
    /// A short code for why.
    pub reason: String,
}

/// What the arguments ask for: `None` for a dry run, or who releases and why for `apply:true` (or `yes`, `1`).
///
/// # Errors
///
/// An unknown argument, or `apply:true` without both `by` and `reason`.
pub fn applies<'a>(
    args: impl IntoIterator<Item = (&'a String, &'a String)>,
) -> Result<Option<Release>> {
    let (mut apply, mut by, mut reason) = (false, None, None);
    for (key, value) in args {
        match key.as_str() {
            "apply" => {
                apply = matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "true" | "yes" | "1"
                );
            }
            "by" => by = Some(value.trim().to_string()),
            "reason" => reason = Some(value.trim().to_string()),
            _ => return Err(Error::string(&format!("unknown argument {key:?}"))),
        }
    }
    if !apply {
        return Ok(None);
    }
    match (by, reason) {
        (Some(by), Some(reason)) if !by.is_empty() && !reason.is_empty() => {
            Ok(Some(Release { by, reason }))
        }
        _ => Err(Error::string(
            "apply:true needs by:<who> and reason:<code>, which the outbox's record keeps on each release",
        )),
    }
}

/// The failed events, oldest first.
const FAILED: &str = "SELECT owed.id, owed.event_id, e.topic, e.kind, owed.charges, owed.reason
    FROM axa_outbox_owed owed JOIN axa_outbox_events e ON e.event_id = owed.event_id
    WHERE owed.state = 'failed' ORDER BY owed.id";

/// List the failed events and, given a [`Release`], put them all back in one transaction.
///
/// # Errors
///
/// When a statement fails, or the record refuses `by` or `reason` (empty, over 200 characters, or with a line break).
pub async fn run(db: &DatabaseConnection, release: Option<&Release>) -> Result<Report> {
    let rows = db
        .query_all_raw(Statement::from_string(DatabaseBackend::Postgres, FAILED))
        .await?;
    let failed = i64::try_from(rows.len()).unwrap_or(i64::MAX);
    let mut listed = Vec::new();
    let mut events = Vec::with_capacity(rows.len());
    for row in &rows {
        events.push(row.try_get::<uuid::Uuid>("", "event_id")?);
        if i64::try_from(listed.len()).unwrap_or(i64::MAX) < LISTED {
            listed.push(FailedRow {
                id: row.try_get("", "id")?,
                topic: row.try_get("", "topic")?,
                kind: row.try_get("", "kind")?,
                attempts: row.try_get("", "charges")?,
                error: row.try_get("", "reason")?,
            });
        }
    }
    let mut retried = 0;
    if let Some(release) = release {
        let outbox_err = |e: sqlx::Error| Error::string(&format!("outbox: {e}"));
        let mut tx = crate::db::for_axa_crate(db)
            .begin()
            .await
            .map_err(outbox_err)?;
        for event in events {
            if axa::outbox::release(&mut tx, event, &release.by, &release.reason)
                .await
                .map_err(outbox_err)?
            {
                retried += 1;
            }
        }
        tx.commit().await.map_err(outbox_err)?;
    }
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
            detail: "Send the event outbox's failed events again. Dry run unless apply:true. Args: [apply:true by:<who> reason:<code>]."
                .to_string(),
        }
    }

    /// List the failed rows, and owe them again with `apply:true`.
    ///
    /// # Errors
    ///
    /// An unknown argument, a failed statement, or an outbox that cannot be reached.
    async fn run(&self, ctx: &AppContext, vars: &task::Vars) -> Result<()> {
        let release = applies(&vars.cli)?;
        let apply = release.is_some();
        let report = run(&ctx.db, release.as_ref()).await?;
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
            println!("{} released", report.retried);
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
        assert_eq!(applies(&args(&[])).unwrap(), None);
        assert_eq!(applies(&args(&[("apply", "false")])).unwrap(), None);
        assert_eq!(
            applies(&args(&[("apply", "maybe"), ("by", "ops")])).unwrap(),
            None
        );
        for yes in ["true", "TRUE", "yes", "1"] {
            assert_eq!(
                applies(&args(&[
                    ("apply", yes),
                    ("by", "ops-jane"),
                    ("reason", "broker_restored")
                ]))
                .unwrap(),
                Some(Release {
                    by: "ops-jane".into(),
                    reason: "broker_restored".into()
                }),
                "{yes}"
            );
        }
    }

    #[test]
    fn applying_names_who_and_why() {
        assert!(applies(&args(&[("apply", "true")])).is_err());
        assert!(applies(&args(&[("apply", "true"), ("by", "ops")])).is_err());
        assert!(applies(&args(&[("apply", "true"), ("reason", "fixed")])).is_err());
        assert!(
            applies(&args(&[
                ("apply", "true"),
                ("by", " "),
                ("reason", "fixed")
            ]))
            .is_err()
        );
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
