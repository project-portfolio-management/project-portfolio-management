//! `director_pack_schedule` task: draft the monthly director packs that are due (DP-12).
//!
//! ```text
//! cargo loco task director_pack_schedule
//! ```
//!
//! Schedule it daily (cron, a Kubernetes `CronJob`), or switch on the in-process ticker
//! (`PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULER_MINUTES`). It **only drafts**: a person reviews and a person
//! issues. `POST /api/director-pack-schedules/sweep` does the same by hand.

use loco_rs::prelude::*;

/// The `director_pack_schedule` CLI task.
pub struct DirectorPackSchedule;

#[async_trait]
impl Task for DirectorPackSchedule {
    /// Task metadata for the loco CLI.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "director_pack_schedule".to_string(),
            detail: "Draft the director packs that are due on the monthly schedule. Never issues. Run daily.".to_string(),
        }
    }

    /// Draft and print how many.
    ///
    /// # Errors
    ///
    /// When a database statement fails.
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let r =
            crate::controllers::pack_rhythm::run_due(ctx, crate::tz::deployment_today()).await?;
        println!(
            "director pack schedule: {} drafted, {} skipped, 0 issued",
            r.drafted.len(),
            r.skipped
        );
        Ok(())
    }
}
