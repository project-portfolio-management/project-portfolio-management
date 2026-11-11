//! `exception_breaches` task: tell people about delivery exceptions that were not completed within the
//! window (DP-3).
//!
//! ```text
//! cargo run -- task exception_breaches
//! ```
//!
//! Schedule it every few minutes (cron, a Kubernetes `CronJob`). Each breach is notified **once**
//! (`breach_notified_at`), to the exception's named owner or, when none was named, to whoever opened it;
//! the notice goes through the ordinary notification inbox and its sinks. Breaches also show in the pack
//! and in `GET /api/exceptions?breached=true` whether or not this task runs.

use crate::prelude::*;
use chrono::Utc;

use crate::delivery_confidence as rules;
use crate::models::_entities::delivery_exceptions as exceptions;

/// The `exception_breaches` CLI task.
pub struct ExceptionBreaches;

/// Notify every open, breached, not-yet-notified exception. Returns how many were notified.
///
/// # Errors
///
/// When a database statement fails.
pub async fn run_once(ctx: &AppContext, now: chrono::DateTime<Utc>) -> Result<usize> {
    let window = rules::window_hours();
    let open: Vec<exceptions::Model> = sqlx::query_as(
        "SELECT * FROM delivery_exceptions
         WHERE status = 'open' AND breach_notified_at IS NULL
         ORDER BY opened_at",
    )
    .fetch_all(crate::db::pool(ctx))
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    let mut told = 0;
    for e in open {
        if !rules::breached(
            e.opened_at.into(),
            e.completed_at.map(Into::into),
            now,
            window,
        ) {
            continue;
        }
        let recipient = e
            .owner_ref
            .clone()
            .or_else(|| e.opened_by.as_ref().map(|s| format!("person:{s}")));
        if let Some(r) = recipient {
            // A person who switched the notice off, or a reference that is not a person, is simply not
            // told; the breach still stands and still shows.
            let _ = crate::models::capabilities::notify_on(
                &mut crate::db::pool(ctx),
                &r,
                "plan",
                e.plan_pid,
                "delivery_exception_breach",
                &format!(
                    "A delivery exception has not been completed within {window} hours: it needs a cause, the decision needed, a named owner and a decide-by date."
                ),
            )
            .await?;
        }
        // `fetch_one`: an update that finds no row is an error, as before SQ-1.
        sqlx::query_scalar::<_, i64>(
            "UPDATE delivery_exceptions SET breach_notified_at = $1 WHERE id = $2 RETURNING id",
        )
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(now))
        .bind(e.id)
        .fetch_one(crate::db::pool(ctx))
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
        told += 1;
    }
    Ok(told)
}

#[async_trait]
impl Task for ExceptionBreaches {
    /// Task metadata for the command line.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "exception_breaches".to_string(),
            detail: "Notify the owner of each delivery exception not completed within the window. Run every few minutes.".to_string(),
        }
    }

    /// Notify and print how many.
    ///
    /// # Errors
    ///
    /// When a database statement fails.
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let n = run_once(ctx, Utc::now()).await?;
        println!("exception breaches: {n} notified");
        Ok(())
    }
}
