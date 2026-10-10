//! `exception_breaches` task: tell people about delivery exceptions that were not completed within the
//! window (DP-3).
//!
//! ```text
//! cargo loco task exception_breaches
//! ```
//!
//! Schedule it every few minutes (cron, a Kubernetes `CronJob`). Each breach is notified **once**
//! (`breach_notified_at`), to the exception's named owner or, when none was named, to whoever opened it;
//! the notice goes through the ordinary notification inbox and its sinks. Breaches also show in the pack
//! and in `GET /api/exceptions?breached=true` whether or not this task runs.

use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;

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
    let open = exceptions::Entity::find()
        .filter(exceptions::Column::Status.eq("open"))
        .filter(exceptions::Column::BreachNotifiedAt.is_null())
        .order_by_asc(exceptions::Column::OpenedAt)
        .all(&ctx.db)
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
            let _ = crate::models::capabilities::notify(
                &ctx.db,
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
        let mut a: exceptions::ActiveModel = e.into();
        a.breach_notified_at = ActiveValue::set(Some(now.into()));
        a.update(&ctx.db)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
        told += 1;
    }
    Ok(told)
}

#[async_trait]
impl Task for ExceptionBreaches {
    /// Task metadata for the loco CLI.
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
