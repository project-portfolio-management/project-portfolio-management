//! The jobs the service can run for itself (NC-5), opt-in. Until now these were tasks for a cron or a
//! Kubernetes `CronJob`; a deployment without either can switch them on here.
//!
//! * **Off by default.** `PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_JOBS` names the jobs to run (comma-separated;
//!   unset or empty runs none), and they run only when the scheduler ticker is on
//!   (`PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULER_MINUTES`, see [`crate::scheduler`]). An unknown name is logged and
//!   ignored.
//! * **One run across replicas.** A job is claimed in a short transaction that takes a PostgreSQL advisory lock
//!   for the job and writes a `scheduled_job_runs` row; a replica that finds a row started within the job's
//!   interval skips it. The lock guards the claim, not the run, so a long job does not hold a connection.
//! * **Each run is recorded.** The row, an audit entry at the start and another at the end, and a count in
//!   `scheduled_job_runs_total{job,outcome}`.
//! * **Retention is a dry run** unless `PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_RETENTION_APPLY` is `true`, `yes` or
//!   `1`: nothing is deleted by switching the scheduler on.

use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::models::audit_logs::Model as AuditModel;

/// The environment variable naming the jobs to run.
pub const JOBS_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_JOBS";
/// The environment variable that lets the scheduled retention job delete.
pub const RETENTION_APPLY_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULED_RETENTION_APPLY";

/// The identity a scheduled run is audited as.
pub const ACTOR: &str = "system:scheduler";

/// A job the service can schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    /// Raise an escalation for each red tolerance that has none.
    ToleranceEscalations,
    /// Tell people about delivery exceptions past their window.
    ExceptionBreaches,
    /// Remove records past their retention period (a dry run unless told to apply).
    RetentionPurge,
    /// Open an exception for each threshold now crossed.
    ThresholdSweep,
    /// Send the weekly portfolio-exceptions digest.
    ExceptionsDigest,
}

/// Every job.
pub const ALL: &[Job] = &[
    Job::ToleranceEscalations,
    Job::ExceptionBreaches,
    Job::RetentionPurge,
    Job::ThresholdSweep,
    Job::ExceptionsDigest,
];

impl Job {
    /// The name used in configuration, metrics and the audit trail.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ToleranceEscalations => "tolerance_escalations",
            Self::ExceptionBreaches => "exception_breaches",
            Self::RetentionPurge => "retention_purge",
            Self::ThresholdSweep => "threshold_sweep",
            Self::ExceptionsDigest => "exceptions_digest",
        }
    }

    /// How long after a run started the job may run again, in minutes.
    #[must_use]
    pub const fn interval_minutes(self) -> i64 {
        match self {
            Self::ExceptionBreaches => 5,
            Self::ThresholdSweep => 60,
            Self::ToleranceEscalations | Self::RetentionPurge => 24 * 60,
            Self::ExceptionsDigest => 7 * 24 * 60,
        }
    }
}

/// The jobs named in `raw`, and the names that are not jobs. Pure.
#[must_use]
pub fn parse_enabled(raw: &str) -> (Vec<Job>, Vec<String>) {
    let mut jobs = Vec::new();
    let mut unknown = Vec::new();
    for name in raw.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        match ALL.iter().find(|j| j.name() == name) {
            Some(j) if !jobs.contains(j) => jobs.push(*j),
            Some(_) => {}
            None => unknown.push(name.to_string()),
        }
    }
    (jobs, unknown)
}

/// The jobs switched on in the environment.
#[must_use]
pub fn enabled() -> Vec<Job> {
    let raw = std::env::var(JOBS_ENV).unwrap_or_default();
    let (jobs, unknown) = parse_enabled(&raw);
    for name in unknown {
        tracing::warn!(job = %name, "{JOBS_ENV} names a job that does not exist; ignored");
    }
    jobs
}

/// What a run did, for the log and the audit trail.
#[derive(Debug, PartialEq, Eq)]
pub enum Ran {
    /// Another replica ran it, or it ran within its interval; this one did nothing.
    Skipped,
    /// It ran and finished; the text says what it did.
    Ok(String),
    /// It ran and failed; the text says why.
    Failed(String),
}

/// Claim `job` for this replica: take the advisory lock, and if no run of it started within its interval, record
/// one. Returns the run's pid when this replica should run it.
async fn claim(ctx: &AppContext, job: Job, now: DateTime<Utc>) -> Result<Option<Uuid>> {
    let mut txn = crate::db::begin(ctx).await?;
    // The same key as ever: `hashtext('scheduled_job:<name>')`, bound. Taken without waiting; a replica that does not
    // get it skips.
    let got: Option<Option<bool>> =
        sqlx::query_scalar("SELECT pg_try_advisory_xact_lock(hashtext($1)) AS got")
            .bind(format!("scheduled_job:{}", job.name()))
            .fetch_optional(txn.conn())
            .await?;
    if !got.flatten().unwrap_or(false) {
        return Ok(None);
    }
    let since = now - chrono::Duration::minutes(job.interval_minutes());
    let recent: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM scheduled_job_runs WHERE job = $1 AND started_at > $2 ORDER BY id DESC LIMIT 1",
    )
    .bind(job.name())
    .bind(since)
    .fetch_optional(txn.conn())
    .await?;
    if recent.is_some() {
        return Ok(None);
    }
    let pid = Uuid::new_v4();
    // The columns the insert set before SQ-1, no more (`created_at`, `updated_at` and `id` are the table's defaults).
    sqlx::query(
        "INSERT INTO scheduled_job_runs (pid, job, started_at, outcome) VALUES ($1, $2, $3, $4)",
    )
    .bind(pid)
    .bind(job.name())
    .bind(now)
    .bind("running")
    .execute(txn.conn())
    .await?;
    AuditModel::record_on(
        txn.conn(),
        pid,
        "scheduled_job_started",
        Some(ACTOR),
        Some(json!({ "job": job.name() })),
    )
    .await
    .map_err(crate::controllers::audit_failed)?;
    txn.commit().await?;
    Ok(Some(pid))
}

/// Run one job and return a one-line outcome for the run record. Retention purge only deletes when the apply setting is on, otherwise it is a dry run; a bad retention setting fails the run rather than purging by default periods.
async fn execute(ctx: &AppContext, job: Job, now: DateTime<Utc>) -> Result<String> {
    match job {
        Job::ToleranceEscalations => {
            let n = crate::controllers::tolerances::raise_all_needed(
                ctx,
                crate::tasks::tolerance_escalations::QUIET_DAYS,
            )
            .await?;
            Ok(format!("{n} escalation(s) raised"))
        }
        Job::ExceptionBreaches => {
            let n = crate::tasks::exception_breaches::run_once(ctx, now).await?;
            Ok(format!("{n} breach(es) notified"))
        }
        Job::RetentionPurge => {
            let apply = std::env::var(RETENTION_APPLY_ENV).is_ok_and(|v| {
                matches!(v.trim().to_ascii_lowercase().as_str(), "true" | "yes" | "1")
            });
            // WW-1.7: the deployment's periods; a bad setting fails the run rather than purge by the default.
            let overrides =
                crate::retention::env_overrides().map_err(|m| axa::Error::string(&m))?;
            let out = crate::retention::run(ctx, now, &overrides, apply).await?;
            let matched: u64 = out.iter().map(|o| o.matched).sum();
            let deleted: u64 = out.iter().map(|o| o.deleted).sum();
            Ok(format!(
                "{} ({matched} matched, {deleted} deleted)",
                if apply { "applied" } else { "dry run" }
            ))
        }
        Job::ThresholdSweep => {
            let v = crate::controllers::thresholds::run_sweep(ctx, Some(ACTOR), None).await?;
            let opened = v["results"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|r| r["outcome"] == "opened")
                .count();
            Ok(format!("{opened} exception(s) opened"))
        }
        Job::ExceptionsDigest => {
            let recipients = crate::controllers::exceptions_digest::recipients();
            let s = crate::controllers::exceptions_digest::send_weekly(ctx, &recipients).await?;
            Ok(format!("{} sent, {} already had it", s.sent, s.already))
        }
    }
}

/// Run `job` if this replica wins the claim: record the run, do the work, record how it ended.
///
/// # Errors
///
/// When the claim or the closing record cannot be written. A failure of the job itself is not an error: it is
/// [`Ran::Failed`], recorded and counted.
pub async fn run_job(ctx: &AppContext, job: Job, now: DateTime<Utc>) -> Result<Ran> {
    let Some(pid) = claim(ctx, job, now).await? else {
        return Ok(Ran::Skipped);
    };
    let result = execute(ctx, job, now).await;
    let (outcome, detail) = match &result {
        Ok(text) => ("ok", text.clone()),
        Err(e) => ("failed", e.to_string()),
    };
    crate::metrics::Metrics::global()
        .scheduled_job_runs_total
        .with_label_values(&[job.name(), outcome])
        .inc();
    let mut txn = crate::db::begin(ctx).await?;
    let row: Option<i64> = sqlx::query_scalar("SELECT id FROM scheduled_job_runs WHERE pid = $1")
        .bind(pid)
        .fetch_optional(txn.conn())
        .await?;
    if let Some(id) = row {
        // The three columns the update set before SQ-1, by id; no row back is an error, as its was.
        sqlx::query_scalar::<_, i64>(
            "UPDATE scheduled_job_runs SET finished_at = $1, outcome = $2, detail = $3 WHERE id = $4
             RETURNING id",
        )
        .bind(Utc::now())
        .bind(outcome)
        .bind(&detail)
        .bind(id)
        .fetch_one(txn.conn())
        .await?;
    }
    AuditModel::record_on(
        txn.conn(),
        pid,
        "scheduled_job_finished",
        Some(ACTOR),
        Some(json!({ "job": job.name(), "outcome": outcome, "detail": detail })),
    )
    .await
    .map_err(crate::controllers::audit_failed)?;
    txn.commit().await?;
    Ok(match result {
        Ok(_) => Ran::Ok(detail),
        Err(_) => Ran::Failed(detail),
    })
}

/// Run every enabled job that is due. Called by the scheduler ticker; runs nothing when no job is enabled.
pub async fn run_enabled(ctx: &AppContext) {
    for job in enabled() {
        match run_job(ctx, job, Utc::now()).await {
            Ok(Ran::Ok(text)) => tracing::info!(job = job.name(), "{text}"),
            Ok(Ran::Failed(text)) => {
                tracing::warn!(job = job.name(), "scheduled job failed: {text}");
            }
            Ok(Ran::Skipped) => {}
            Err(e) => tracing::warn!(job = job.name(), "scheduled job could not be recorded: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_enabled_by_default() {
        assert_eq!(parse_enabled(""), (vec![], vec![]));
        assert_eq!(parse_enabled(" , ,"), (vec![], vec![]));
    }

    #[test]
    fn named_jobs_are_enabled_once_and_unknown_names_are_reported() {
        let (jobs, unknown) =
            parse_enabled("retention_purge, threshold_sweep,retention_purge,backups");
        assert_eq!(jobs, vec![Job::RetentionPurge, Job::ThresholdSweep]);
        assert_eq!(unknown, vec!["backups".to_string()]);
    }

    #[test]
    fn every_job_has_a_distinct_name_that_parses_back() {
        for j in ALL {
            assert_eq!(parse_enabled(j.name()).0, vec![*j]);
            assert!(j.interval_minutes() > 0);
        }
        let mut names: Vec<_> = ALL.iter().map(|j| j.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ALL.len());
    }
}
