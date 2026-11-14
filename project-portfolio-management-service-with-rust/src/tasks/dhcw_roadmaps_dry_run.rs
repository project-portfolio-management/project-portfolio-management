//! `task dhcw_roadmaps_dry_run dir:<folder>` (RE-5 part b): what moving the DHCW Roadmap Editor's spreadsheet into
//! public product roadmaps would make, from its CSV exports, without writing anything.
//!
//! The folder holds `Roadmaps.csv` (the roadmap index, the spreadsheet's first tab) and one `<Sheet>.csv` per roadmap
//! tab. Organization codes and roadmap categories are matched against this deployment's own. The report is JSON on
//! standard output; the rules are [`crate::dhcw_import`]. There is no apply step.

use std::collections::BTreeMap;

use crate::prelude::*;

/// The task.
pub struct DhcwRoadmapsDryRun;

#[async_trait]
impl Task for DhcwRoadmapsDryRun {
    /// Task metadata (name + one-line description) for the command line.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "dhcw_roadmaps_dry_run".to_string(),
            detail: "Report what the DHCW roadmap spreadsheet's CSV exports would become, writing nothing. Args: dir:<folder with Roadmaps.csv and one <Sheet>.csv per tab>."
                .to_string(),
        }
    }

    /// Read the folder, match against this deployment's lists, and print the report.
    ///
    /// # Errors
    ///
    /// When `dir:` is missing, the folder or a file cannot be read, or a database read fails.
    async fn run(&self, ctx: &AppContext, vars: &task::Vars) -> Result<()> {
        let dir = vars
            .cli
            .get("dir")
            .ok_or_else(|| Error::string("dir:<folder> is required"))?;
        let dir = std::path::Path::new(dir);
        let index = std::fs::read_to_string(dir.join("Roadmaps.csv"))
            .map_err(|e| Error::string(&format!("{}: {e}", dir.join("Roadmaps.csv").display())))?;
        let mut tabs = BTreeMap::new();
        for entry in std::fs::read_dir(dir).map_err(|e| Error::string(&e.to_string()))? {
            let path = entry.map_err(|e| Error::string(&e.to_string()))?.path();
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().is_some_and(|x| x == "csv") && stem != "Roadmaps" {
                let text =
                    std::fs::read_to_string(&path).map_err(|e| Error::string(&e.to_string()))?;
                tabs.insert(stem.to_string(), text);
            }
        }
        let known_codes: Vec<String> =
            sqlx::query_scalar("SELECT code FROM organizations WHERE active AND code IS NOT NULL")
                .fetch_all(crate::db::pool(ctx))
                .await
                .map_err(|e| Error::string(&e.to_string()))?;
        let known_categories: Vec<String> =
            sqlx::query_scalar("SELECT name FROM roadmap_categories WHERE active")
                .fetch_all(crate::db::pool(ctx))
                .await
                .map_err(|e| Error::string(&e.to_string()))?;
        let report = crate::dhcw_import::dry_run(&crate::dhcw_import::Input {
            index,
            tabs,
            known_codes,
            known_categories,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
        Ok(())
    }
}
