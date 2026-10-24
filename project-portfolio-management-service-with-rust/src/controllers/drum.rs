//! The drum and the pipeline (CC-5). Rules in [`crate::drum`].
//!
//! * `PUT /api/portfolio-drum` names the skill pool that is the drum (or clears the naming);
//! * `GET /api/portfolio-drum` says which pool is the drum now and why;
//! * `GET /api/portfolio-pipeline` proposes a staggered start for each project that needs the drum, with a
//!   capacity buffer, and explains it. **A proposal: no date is ever changed.**
//!
//! Only plans the caller may read are looked at, so a private plan's drum use is nowhere (it neither
//! appears nor holds up another project), as in the capacity heatmap.

use super::db_err;
use std::collections::{HashMap, HashSet};

use chrono::{Datelike, NaiveDate};
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::drum::{self as rules, Project, Segment};
use crate::models::_entities::{
    allocations, plans, portfolio_drum as drums, resource_roles, skill_pool_members as members,
    skill_pools as pools,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::portfolio_resources::is_placeholder;

fn buffer_days() -> (i64, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_DRUM_BUFFER_DAYS") {
        Err(_) => (7, false),
        Ok(v) => match v.trim().parse::<i64>() {
            Ok(d) if (0..=90).contains(&d) => (d, false),
            _ => (7, true),
        },
    }
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/portfolio-drum", put(set_drum))
        .add("/portfolio-drum", get(show_drum))
        .add("/portfolio-pipeline", get(pipeline))
}

#[derive(Debug, Deserialize)]
struct DrumBody {
    /// The pool to name; `null` clears the naming.
    #[serde(default)]
    skill_pool_pid: Option<Uuid>,
}

/// `PUT /api/portfolio-drum`
#[debug_handler]
async fn set_drum(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<DrumBody>,
) -> Result<Response> {
    if let Some(p) = b.skill_pool_pid {
        let pool = pools::Entity::find()
            .filter(pools::Column::Pid.eq(p))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .ok_or_else(|| unprocessable("there is no such skill pool"))?;
        if !pool.active {
            return Err(unprocessable("that skill pool is retired"));
        }
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = drums::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        skill_pool_pid: ActiveValue::set(b.skill_pool_pid),
        declared_on: ActiveValue::set(crate::tz::deployment_today()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "portfolio_drum_set",
        caller.actor(),
        Some(json!({ "skill_pool_pid": b.skill_pool_pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(json!({ "skill_pool_pid": row.skill_pool_pid, "declared_on": row.declared_on }))
}

/// The drum in force.
struct Drum {
    pool: Uuid,
    name: String,
    source: &'static str,
    /// Why, in words.
    why: String,
}

/// The constraint pool for start control (EX-8): the drum, named or the most overloaded, with its name.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn constraint_pool(
    ctx: &AppContext,
    heat: &Value,
) -> Result<Option<(Uuid, String)>> {
    Ok(resolve(ctx, heat).await?.ok().map(|d| (d.pool, d.name)))
}

/// Which pool is the drum: the one named, else the most overloaded in the heatmap, else none (with why).
async fn resolve(
    ctx: &AppContext,
    heat: &Value,
) -> Result<std::result::Result<Drum, (&'static str, String)>> {
    let named = drums::Entity::find()
        .order_by_desc(drums::Column::Id)
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .and_then(|d| d.skill_pool_pid);
    if let Some(p) = named {
        let pool = pools::Entity::find()
            .filter(pools::Column::Pid.eq(p))
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
        return Ok(match pool {
            Some(pool) if pool.active => Ok(Drum {
                pool: pool.pid,
                name: pool.name,
                source: "named",
                why: "named by the portfolio office".into(),
            }),
            _ => Err((
                "named_pool_retired",
                "the pool named as the drum is retired or gone: name another or clear the naming"
                    .into(),
            )),
        });
    }
    let all = pools::Entity::find()
        .filter(pools::Column::Active.eq(true))
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if all == 0 {
        return Ok(Err((
            "no_skill_pools",
            "no drum is named and there are no skill pools to choose from".into(),
        )));
    }
    let mut best: Option<(f64, String, Uuid)> = None;
    for p in heat["pools"].as_array().into_iter().flatten() {
        let Some(pid) = p["pool_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok()) else {
            continue;
        };
        let short: f64 = p["cells"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|c| c["gap_fte"].as_f64())
            .filter(|g| *g < 0.0)
            .map(|g| -g)
            .sum();
        if short > 0.0
            && best
                .as_ref()
                .is_none_or(|(b, n, _)| match short.total_cmp(b) {
                    std::cmp::Ordering::Greater => true,
                    std::cmp::Ordering::Equal => {
                        p["name"].as_str().unwrap_or("").to_lowercase() < n.to_lowercase()
                    }
                    std::cmp::Ordering::Less => false,
                })
        {
            best = Some((short, p["name"].as_str().unwrap_or("").to_string(), pid));
        }
    }
    Ok(match best {
        Some((short, name, pool)) => Ok(Drum {
            pool,
            name,
            source: "most_overloaded",
            why: format!(
                "no drum is named; this pool has the largest total shortfall in the heatmap window ({short:.2} FTE-months)"
            ),
        }),
        None => Err((
            "no_overloaded_pool",
            "no drum is named and no skill pool is short of supply in the window".into(),
        )),
    })
}

async fn readable_set(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<HashSet<Uuid>> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    Ok(super::plan_access::readable_plans(ctx, caller, &all).await)
}

#[derive(Debug, Deserialize)]
struct Window {
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    months: Option<u32>,
}

fn window(q: &Window) -> Result<(NaiveDate, u32)> {
    let months = q.months.unwrap_or(6);
    if !(1..=24).contains(&months) {
        return Err(unprocessable("months must be 1 to 24"));
    }
    let from = match &q.from {
        None => crate::tz::deployment_today()
            .with_day(1)
            .unwrap_or_default(),
        Some(f) => crate::capacity_heatmap::parse_month(f)
            .ok_or_else(|| unprocessable("from must be YYYY-MM"))?,
    };
    Ok((from, months))
}

/// `GET /api/portfolio-drum[?from=YYYY-MM][&months=6]`
#[debug_handler]
async fn show_drum(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<Window>,
) -> Result<Response> {
    let (from, months) = window(&q)?;
    let readable = readable_set(&ctx, &caller).await?;
    let heat = super::capacity_heatmap::heatmap(&ctx, &readable, from, months).await?;
    let history = drums::Entity::find()
        .order_by_desc(drums::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let now = match resolve(&ctx, &heat).await? {
        Ok(d) => {
            json!({ "state": "drum", "skill_pool_pid": d.pool, "name": d.name, "source": d.source, "why": d.why })
        }
        Err((code, why)) => json!({ "state": "no_drum", "reason": code, "why": why }),
    };
    format::json(json!({
        "drum": now,
        "history": history.iter().map(|h| json!({ "skill_pool_pid": h.skill_pool_pid, "declared_on": h.declared_on })).collect::<Vec<_>>(),
        "note": "The drum is the skill pool the portfolio office names, or, when none is named, the pool with the largest total shortfall in the capacity heatmap window over the plans you may read.",
    }))
}

/// `GET /api/portfolio-pipeline[?from=YYYY-MM][&months=6]`
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn pipeline(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<Window>,
) -> Result<Response> {
    let (from, months) = window(&q)?;
    let readable = readable_set(&ctx, &caller).await?;
    let heat = super::capacity_heatmap::heatmap(&ctx, &readable, from, months).await?;
    let (buffer, unusable) = buffer_days();
    let mut out = json!({
        "advisory": true, "capacity_buffer_days": buffer, "capacity_buffer_setting_unusable": unusable,
        "from": from, "months": months,
        "note": "This is a proposal: no start date has been changed. Projects that ask the drum for capacity are taken in order of when they first need it, then by name; a project is moved later only when the projects already placed leave the drum without room for it, and then to after the last drum use of an earlier project plus the capacity buffer (calendar days). Capacity is the lowest monthly supply (the organization's own people and partners' commitments) of the drum pool in the window. Demand is firm allocations of people in the drum pool's roles on plans you may read; plans you may not read are not looked at, so they neither appear nor hold up another project.",
    });
    let drum = match resolve(&ctx, &heat).await? {
        Ok(d) => d,
        Err((code, why)) => {
            out["state"] = json!("no_drum");
            out["reason"] = json!(code);
            out["why"] = json!(why);
            return format::json(out);
        }
    };
    out["drum"] = json!({ "skill_pool_pid": drum.pool, "name": drum.name, "source": drum.source, "why": drum.why });
    let supply: Vec<f64> = heat["pools"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|p| p["pool_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok()) == Some(drum.pool))
        .into_iter()
        .flat_map(|p| p["cells"].as_array().cloned().unwrap_or_default())
        .filter_map(|c| c["supply_fte"].as_f64())
        .collect();
    let Some(low) = supply.iter().copied().reduce(f64::min) else {
        out["state"] = json!("no_capacity");
        out["reason"] = json!(
            "the drum has no supply recorded in the window, so there is nothing to stagger against"
        );
        return format::json(out);
    };
    #[expect(clippy::cast_possible_truncation)] // an FTE count is tiny
    let capacity = (low * 1000.0).round() as i64;
    out["capacity_fte"] = json!(low);

    let roles: HashSet<String> = members::Entity::find()
        .filter(members::Column::PoolPid.eq(drum.pool))
        .filter(members::Column::Kind.eq("role"))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|m| m.value.to_lowercase())
        .collect();
    let role_of: HashMap<String, String> = resource_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| (r.person_ref, r.role.to_lowercase()))
        .collect();
    let last = crate::capacity_heatmap::add_months(from, months)
        .pred_opt()
        .unwrap_or(from);
    let mut by_plan: HashMap<Uuid, Vec<Segment>> = HashMap::new();
    for a in allocations::Entity::find()
        .filter(allocations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        if !readable.contains(&a.plan_pid) || a.booking == "proposed" {
            continue;
        }
        let role = a
            .role
            .as_deref()
            .filter(|r| !r.trim().is_empty())
            .map(str::to_lowercase)
            .or_else(|| {
                (!is_placeholder(&a.person_ref))
                    .then(|| role_of.get(&a.person_ref).cloned())
                    .flatten()
            });
        if !role.is_some_and(|r| roles.contains(&r)) {
            continue;
        }
        let (start, end) = (
            a.start_date.unwrap_or(from).max(from),
            a.end_date.unwrap_or(last).min(last),
        );
        if start > end {
            continue;
        }
        by_plan.entry(a.plan_pid).or_default().push(Segment {
            start,
            end,
            milli_fte: i64::from(a.percent) * 10,
        });
    }
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let projects: Vec<Project> = by_plan
        .into_iter()
        .map(|(plan, segments)| Project {
            plan,
            name: names.get(&plan).cloned().unwrap_or_default(),
            segments,
        })
        .collect();
    let placements = rules::stagger(&projects, capacity, buffer);
    out["state"] = json!("pipeline");
    out["projects"] =
        json!(placements
        .iter()
        .map(|p| json!({
            "plan_pid": p.plan, "name": p.name, "outcome": p.outcome,
            "first_drum_use": p.first_use, "last_drum_use": p.last_use,
            "peak_fte": f64::from(i32::try_from(p.peak_milli_fte).unwrap_or(i32::MAX)) / 1000.0,
            "delay_days": p.delay_days, "proposed_first_drum_use": p.proposed_first_use,
            "explanation": p.explanation,
        }))
        .collect::<Vec<_>>());
    out["staggered"] = json!(placements.iter().filter(|p| p.delay_days > 0).count());
    format::json(out)
}
