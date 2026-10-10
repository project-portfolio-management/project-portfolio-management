//! `retention_purge` task: remove records past their retention period (RG-4).
//!
//! ```text
//! cargo loco task retention_purge                              # dry run: counts only
//! cargo loco task retention_purge apply:true                   # delete
//! cargo loco task retention_purge apply:true notifications_days:60
//! ```
//!
//! Schedule it (cron, a Kubernetes `CronJob`, a systemd timer) once a day after the dry run has
//! shown what it would remove. See `governance/retention-schedule.md` for what is and is not covered.

use chrono::Utc;
use loco_rs::prelude::*;

use crate::retention::{self, RULES};

/// The `retention_purge` CLI task.
pub struct RetentionPurge;

#[async_trait]
impl Task for RetentionPurge {
    /// Task metadata for the loco CLI.
    fn task(&self) -> TaskInfo {
        let names: Vec<String> = RULES.iter().map(|r| format!("{}_days", r.name)).collect();
        TaskInfo {
            name: "retention_purge".to_string(),
            detail: format!(
                "Delete records past their retention period. Dry run unless apply:true. Args: [apply:true] [{}:<calendar days>].",
                names.join("|")
            ),
        }
    }

    /// Run every rule and print what it matched and deleted.
    ///
    /// # Errors
    ///
    /// When an argument is not a positive number of calendar days, an argument name is unknown, or
    /// a database statement fails.
    async fn run(&self, ctx: &AppContext, vars: &task::Vars) -> Result<()> {
        let mut apply = false;
        let mut overrides = Vec::new();
        for (key, value) in &vars.cli {
            if key == "apply" {
                apply = matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "true" | "yes" | "1"
                );
            } else if let Some(name) = key.strip_suffix("_days") {
                if !RULES.iter().any(|r| r.name == name) {
                    return Err(Error::string(&format!("unknown rule {name:?}")));
                }
                let days = retention::parse_days(name, value).map_err(|m| Error::string(&m))?;
                overrides.push((name.to_string(), days));
            } else {
                return Err(Error::string(&format!("unknown argument {key:?}")));
            }
        }
        let outcomes = retention::run(&ctx.db, Utc::now(), &overrides, apply).await?;
        println!(
            "{}",
            if apply {
                "retention purge (APPLIED)"
            } else {
                "retention purge (dry run: nothing deleted; add apply:true)"
            }
        );
        for o in &outcomes {
            println!(
                "  {:<20} {:<22} older than {:>5} calendar days: {} matched, {} deleted",
                o.rule, o.table, o.days, o.matched, o.deleted
            );
        }
        Ok(())
    }
}
