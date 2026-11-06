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
use axa::prelude::*;
use uuid::Uuid;

use crate::models::_entities::{budget_lines, components, milestones, risks, tasks, time_entries};

/// Rows embedded per sub-resource.
const CAP: i64 = 200;

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

/// A plan's live rows of one kind, as JSON. `sql` takes the plan as `$1` and the cap as `$2`.
async fn live<T>(ctx: &AppContext, sql: &'static str, plan_pid: Uuid) -> Result<serde_json::Value>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + serde::Serialize + Send + Unpin,
{
    let rows: Vec<T> = sqlx::query_as(sql)
        .bind(plan_pid)
        .bind(CAP)
        .fetch_all(crate::db::pool(ctx))
        .await?;
    Ok(serde_json::json!(rows))
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
            "milestones" => {
                live::<milestones::Model>(
                    ctx,
                    "SELECT * FROM milestones WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY due LIMIT $2",
                    plan_pid,
                )
                .await?
            }
            "risks" => {
                live::<risks::Model>(
                    ctx,
                    "SELECT * FROM risks WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY id LIMIT $2",
                    plan_pid,
                )
                .await?
            }
            "tasks" => {
                live::<tasks::Model>(
                    ctx,
                    "SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY id LIMIT $2",
                    plan_pid,
                )
                .await?
            }
            "components" => {
                live::<components::Model>(
                    ctx,
                    "SELECT * FROM components WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY id LIMIT $2",
                    plan_pid,
                )
                .await?
            }
            "time_entries" => {
                live::<time_entries::Model>(
                    ctx,
                    "SELECT * FROM time_entries WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY spent_on DESC LIMIT $2",
                    plan_pid,
                )
                .await?
            }
            "budget_lines" => {
                live::<budget_lines::Model>(
                    ctx,
                    "SELECT * FROM budget_lines WHERE plan_pid = $1 AND deleted_at IS NULL
                     ORDER BY id LIMIT $2",
                    plan_pid,
                )
                .await?
            }
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
