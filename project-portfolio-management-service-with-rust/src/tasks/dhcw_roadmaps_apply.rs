//! `task dhcw_roadmaps_apply dir:<folder> plans:<file>` (RE-5): write the DHCW Roadmap Editor's spreadsheet, from its
//! CSV exports, onto the existing `Product` plans the plans file names, as unpublished roadmaps.
//!
//! Run `dhcw_roadmaps_dry_run` on the same folder first. The plans file is CSV with the columns `Sheet` and `Plan`
//! (a plan id), one row per tab to import; a tab it does not name is left alone. The report is JSON on standard output
//! and accounts for every roadmap; the rules are [`crate::dhcw_apply`].

use std::collections::BTreeMap;

use crate::prelude::*;

/// The task.
pub struct DhcwRoadmapsApply;

#[async_trait]
impl Task for DhcwRoadmapsApply {
    /// Task metadata (name + one-line description) for the command line.
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "dhcw_roadmaps_apply".to_string(),
            detail: "Write the DHCW roadmap spreadsheet's CSV exports onto existing Product plans, unpublished. Args: dir:<folder with Roadmaps.csv and one <Sheet>.csv per tab> plans:<CSV of Sheet,Plan>."
                .to_string(),
        }
    }

    /// Read the folder and the plans file, write each roadmap, and print the report.
    ///
    /// # Errors
    ///
    /// When `dir:` or `plans:` is missing, a file cannot be read, the plans file is malformed, or the database fails.
    async fn run(&self, ctx: &AppContext, vars: &task::Vars) -> Result<()> {
        let arg = |name: &str| {
            vars.cli
                .get(name)
                .ok_or_else(|| Error::string(&format!("{name}:<…> is required")))
        };
        let dir = std::path::Path::new(arg("dir")?);
        let plans_path = arg("plans")?;
        let read = |p: &std::path::Path| {
            std::fs::read_to_string(p).map_err(|e| Error::string(&format!("{}: {e}", p.display())))
        };
        let plans = crate::dhcw_apply::plans_file(&read(std::path::Path::new(plans_path))?)
            .map_err(|e| Error::string(&e))?;
        let index = read(&dir.join("Roadmaps.csv"))?;
        let mut tabs = BTreeMap::new();
        for entry in std::fs::read_dir(dir).map_err(|e| Error::string(&e.to_string()))? {
            let path = entry.map_err(|e| Error::string(&e.to_string()))?.path();
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().is_some_and(|x| x == "csv") && stem != "Roadmaps" {
                tabs.insert(stem.to_string(), read(&path)?);
            }
        }
        let parsed = crate::dhcw_import::parse(&crate::dhcw_import::Input {
            index,
            tabs,
            ..Default::default()
        })
        .map_err(|e| Error::string(&e))?;
        let report = crate::dhcw_apply::apply(ctx, &parsed, &plans).await?;
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
        Ok(())
    }
}
