//! `tolerance_escalations` task: raise an escalation for each red tolerance that nobody has escalated.
//!
//! ```text
//! cargo loco task tolerance_escalations
//! ```
//!
//! Schedule it daily (cron, a Kubernetes `CronJob`). It does what the "Escalate" button on a plan's tolerance page
//! does, for every plan, as the identity `system:tolerance-escalations`. A dimension is skipped when an escalation for
//! it is open or acknowledged, or when one was raised for it in the last [`QUIET_DAYS`] days, so a resolved one on a
//! plan that stays red is not raised again at once. Plans hidden from everyone by a handling mark are not read.

use loco_rs::prelude::*;

/// Days after raising during which the same plan and dimension is not escalated again.
pub const QUIET_DAYS: i64 = 7;

/// The `tolerance_escalations` CLI task.
pub struct ToleranceEscalations;

#[async_trait]
impl Task for ToleranceEscalations {
    /// Task metadata for the loco CLI.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "tolerance_escalations".to_string(),
            detail: "Raise an escalation for each red tolerance that has none open. Run daily."
                .to_string(),
        }
    }

    /// Raise and print how many.
    ///
    /// # Errors
    ///
    /// When a database statement fails.
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let n = crate::controllers::tolerances::raise_all_needed(ctx, QUIET_DAYS).await?;
        println!("tolerance escalations: {n} raised");
        Ok(())
    }
}
