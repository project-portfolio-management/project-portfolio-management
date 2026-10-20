//! `?include=` on `GET /api/plans/{pid}` (RM-12): embed named sub-resources
//! in one response instead of one request each. Without `include` the
//! response is the bare plan, unchanged.
//!
//! Each included set is the **same live rows** its own route would return
//! (capped), and the same gates apply: a sub-resource whose **module is off**
//! for the plan is omitted (reason `module_disabled`), and a **masked** read
//! pulls nothing operational at all (reason `masked`). An omitted name is
//! *listed*, never silently missing.

use super::db_err;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::models::_entities::{budget_lines, components, milestones, risks, tasks, time_entries};

/// Rows embedded per sub-resource.
const CAP: u64 = 200;

/// Includable names, and the module each belongs to.
pub const INCLUDABLE: &[(&str, &str)] = &[
    ("milestones", "milestones"),
    ("risks", "risks"),
    ("tasks", "tasks"),
    ("components", "tasks"),
    ("time_entries", "time"),
    ("budget_lines", "financials"),
];

/// Parse `include=a,b` into known names; an unknown one is an error that
/// lists what is allowed.
///
/// # Errors
///
/// A message naming each unknown name and the allowed ones.
pub fn parse(raw: &str) -> std::result::Result<Vec<&'static str>, String> {
    let mut out: Vec<&'static str> = Vec::new();
    let mut bad: Vec<String> = Vec::new();
    for part in raw.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        match INCLUDABLE.iter().find(|(n, _)| *n == part) {
            Some((n, _)) => {
                if !out.contains(n) {
                    out.push(n);
                }
            }
            None => bad.push(part.to_string()),
        }
    }
    if !bad.is_empty() {
        let allowed: Vec<&str> = INCLUDABLE.iter().map(|(n, _)| *n).collect();
        return Err(format!("unknown include {bad:?} (allowed: {allowed:?})"));
    }
    Ok(out)
}

/// Build the `included` map and the `omitted` list for `names`.
///
/// # Errors
///
/// Propagates database errors.
pub async fn build(
    ctx: &AppContext,
    plan_pid: Uuid,
    names: &[&'static str],
    masked: bool,
) -> Result<(
    serde_json::Map<String, serde_json::Value>,
    Vec<serde_json::Value>,
)> {
    let mut included = serde_json::Map::new();
    let mut omitted = Vec::new();
    let (modules, _) = super::modules::modules_in_force(&ctx.db, plan_pid)
        .await
        .map_err(db_err)?;
    for name in names {
        if masked {
            omitted.push(serde_json::json!({ "name": name, "reason": "masked" }));
            continue;
        }
        let module = INCLUDABLE
            .iter()
            .find(|(n, _)| n == name)
            .map_or("", |(_, m)| *m);
        if !modules.contains(module) {
            omitted.push(serde_json::json!({ "name": name, "reason": "module_disabled" }));
            continue;
        }
        let value = match *name {
            "milestones" => serde_json::json!(
                milestones::Entity::find()
                    .filter(milestones::Column::PlanPid.eq(plan_pid))
                    .filter(milestones::Column::DeletedAt.is_null())
                    .order_by_asc(milestones::Column::Due)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            "risks" => serde_json::json!(
                risks::Entity::find()
                    .filter(risks::Column::PlanPid.eq(plan_pid))
                    .filter(risks::Column::DeletedAt.is_null())
                    .order_by_asc(risks::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            "tasks" => serde_json::json!(
                tasks::Entity::find()
                    .filter(tasks::Column::PlanPid.eq(plan_pid))
                    .filter(tasks::Column::DeletedAt.is_null())
                    .order_by_asc(tasks::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            "components" => serde_json::json!(
                components::Entity::find()
                    .filter(components::Column::PlanPid.eq(plan_pid))
                    .filter(components::Column::DeletedAt.is_null())
                    .order_by_asc(components::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            "time_entries" => serde_json::json!(
                time_entries::Entity::find()
                    .filter(time_entries::Column::PlanPid.eq(plan_pid))
                    .filter(time_entries::Column::DeletedAt.is_null())
                    .order_by_desc(time_entries::Column::SpentOn)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            "budget_lines" => serde_json::json!(
                budget_lines::Entity::find()
                    .filter(budget_lines::Column::PlanPid.eq(plan_pid))
                    .filter(budget_lines::Column::DeletedAt.is_null())
                    .order_by_asc(budget_lines::Column::Id)
                    .limit(CAP)
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
            ),
            _ => continue,
        };
        included.insert((*name).to_string(), value);
    }
    Ok((included, omitted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_names_parse_deduplicated_in_order() {
        assert_eq!(parse("risks, tasks,risks").unwrap(), ["risks", "tasks"]);
        assert!(parse("").unwrap().is_empty());
    }

    #[test]
    fn an_unknown_name_is_refused_listing_the_allowed_ones() {
        let err = parse("tasks,nope").unwrap_err();
        assert!(err.contains("nope") && err.contains("milestones"), "{err}");
    }

    #[test]
    fn every_includable_name_belongs_to_a_known_module() {
        let all = crate::modules::all_keys();
        for (_, m) in INCLUDABLE {
            assert!(all.contains(*m), "{m}");
        }
    }
}
