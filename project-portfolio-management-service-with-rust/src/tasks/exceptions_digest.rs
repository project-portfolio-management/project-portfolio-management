//! `exceptions_digest` task: send the weekly portfolio-exceptions digest.
//!
//! ```text
//! PROJECT_PORTFOLIO_MANAGEMENT_DIGEST_RECIPIENTS=person:<user id>,person:<user id> cargo loco task exceptions_digest
//! ```
//!
//! Schedule it weekly (cron, a Kubernetes `CronJob`); running it again in the same week sends nothing more. With no
//! recipients configured it does nothing.

use loco_rs::prelude::*;

/// The `exceptions_digest` CLI task.
pub struct ExceptionsDigest;

#[async_trait]
impl Task for ExceptionsDigest {
    /// Task metadata for the loco CLI.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "exceptions_digest".to_string(),
            detail: "Send the weekly portfolio-exceptions digest to the configured recipients. Run weekly."
                .to_string(),
        }
    }

    /// Send and print the outcome.
    ///
    /// # Errors
    ///
    /// When a database statement fails.
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let recipients = crate::controllers::exceptions_digest::recipients();
        let sent = crate::controllers::exceptions_digest::send_weekly(ctx, &recipients).await?;
        println!(
            "exceptions digest: {} sent, {} already sent this week, {} not addressable",
            sent.sent, sent.already, sent.unaddressable
        );
        Ok(())
    }
}
