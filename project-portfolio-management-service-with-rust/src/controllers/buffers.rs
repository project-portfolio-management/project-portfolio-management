//! Project, feeding and resource buffers (CC-3): `GET /api/plans/{pid}/critical-chain-buffers`. The rules are
//! [`crate::buffers`]; the schedule is [`crate::controllers::critical_chain`]'s, built with every task at its
//! aggressive estimate (CC-2) where it has one. Changes nothing.
//!
//! A buffer is a size in business days and a place on the chain, never a date a task is held to. A buffer
//! that would push the chain is reported, not absorbed. Resource buffers are readiness alerts, not time.

use std::collections::{BTreeSet, HashMap};

use loco_rs::prelude::*;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::buffers;
use crate::critical_chain::{self, Outcome};

const NOTE: &str = "This is advice; nothing has been changed. Tasks are scheduled at their aggressive estimates where they have one (the others keep their likely duration and are listed), levelled for people's time as for the critical chain. Each buffer is sized from the safety removed from its tasks (safe minus aggressive) by the rule printed, in business days of the plan's calendar. A feeding buffer that would push the critical chain is flagged with the days, not absorbed. Resource buffers are alerts that a person must be ready, not time. Only finish-to-start dependencies are followed as chain links.";

fn resource_lead_days() -> (i64, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_RESOURCE_BUFFER_DAYS") {
        Err(_) => (2, false),
        Ok(v) => match v.trim().parse::<i64>() {
            Ok(d) if (0..=30).contains(&d) => (d, false),
            _ => (2, true),
        },
    }
}

/// `GET /api/plans/{pid}/critical-chain-buffers`
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn show(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let c = super::leveling::compute_with(&ctx, plan.pid, true, 100, true).await?;
    let (_, nodes, levelled) = super::critical_chain::schedules(&c)?;
    let net = &c.net;
    let (rule, rule_unusable) = buffers::configured_rule();
    let (lead, lead_unusable) = resource_lead_days();
    let (est_rule, est_unusable) = crate::ccpm_estimates::configured_rule();
    let mut out = json!({
        "advisory": true,
        "calendar": net.cal_source,
        "buffer_rule": rule.name(), "buffer_rule_setting_unusable": rule_unusable,
        "estimate_rule": est_rule.name(), "estimate_rule_setting_unusable": est_unusable,
        "resource_buffer_lead_days": lead, "resource_buffer_setting_unusable": lead_unusable,
        "note": NOTE,
    });
    let chain = match critical_chain::find(&nodes, &net.edges, &levelled, &c.people) {
        Outcome::NoCriticalChain { reason } => {
            out["state"] = json!("no_buffers");
            out["reason"] = json!(reason);
            return format::json(out);
        }
        Outcome::Chain(ch) => ch,
    };
    let own: Vec<Uuid> = net.rows.iter().map(|t| t.pid).collect();
    let safety = |pid: &Uuid| -> Option<i64> {
        let r = c.estimates.get(pid)?;
        Some(r.safe?.days - r.aggressive?.days)
    };
    let titled = |p: &Uuid| json!({ "pid": p, "title": c.titles.get(p) });
    let using_likely: Vec<Value> = own
        .iter()
        .filter(|p| {
            c.estimates.get(*p).is_some_and(|r| {
                r.aggressive.is_none()
                    && !matches!(
                        r.state,
                        crate::ccpm_estimates::State::Milestone | crate::ccpm_estimates::State::NoDuration
                    )
            })
        })
        .map(titled)
        .collect();
    out["scheduled_at_likely_duration"] = json!(using_likely);

    let chain_pids: Vec<Uuid> = chain.tasks.iter().map(|t| t.pid).collect();
    let missing: Vec<Value> = chain_pids.iter().filter(|p| safety(p).is_none()).map(titled).collect();
    if !missing.is_empty() {
        out["state"] = json!("no_buffers");
        out["reason"] = json!("estimates_missing");
        out["chain_tasks_missing_estimates"] = json!(missing);
        return format::json(out);
    }
    let when: HashMap<Uuid, &crate::cpm::NodeResult> = levelled.nodes.iter().map(|n| (n.pid, n)).collect();
    let cal = &net.plan_cal;
    let last = when[chain_pids.last().expect("a chain has a task")];
    let project_days = buffers::size(
        rule,
        &chain_pids.iter().filter_map(&safety).collect::<Vec<_>>(),
    );
    out["state"] = json!("buffers");
    out["chain_days_at_aggressive"] = json!(chain.chain_days);
    out["crosses_paths"] = json!(chain.crosses_paths);
    out["chain"] = json!(chain_pids.iter().map(titled).collect::<Vec<_>>());
    out["project_buffer"] = json!({
        "days": project_days,
        "safety_removed_days": chain_pids.iter().filter_map(&safety).sum::<i64>(),
        "chain_finish": last.early_finish,
        "finish_with_buffer": cal.date_of(last.ef_index - 1 + project_days),
    });

    let aggressive: HashMap<Uuid, i64> = nodes
        .iter()
        .map(|n| (n.pid, n.duration.unwrap_or(0)))
        .collect();
    let feeders = buffers::feeding_chains(&chain_pids, &own, &net.edges, &aggressive);
    let mut feeding = Vec::new();
    for f in &feeders {
        let gaps: Vec<Value> = f.tasks.iter().filter(|t| safety(t).is_none()).map(&titled).collect();
        let mut v = json!({
            "joins": titled(&f.joins),
            "tasks": f.tasks.iter().map(&titled).collect::<Vec<_>>(),
            "duration_days": f.duration_days,
        });
        if gaps.is_empty() {
            let days = buffers::size(rule, &f.tasks.iter().filter_map(&safety).collect::<Vec<_>>());
            let ends = f.tasks.last().and_then(|t| when.get(t)).map_or(0, |r| r.ef_index);
            let room = when.get(&f.joins).map_or(0, |r| r.es_index) - ends;
            v["buffer_days"] = json!(days);
            v["room_before_join_days"] = json!(room.max(0));
            v["would_push_chain"] = json!(days > room);
            v["push_days"] = json!((days - room).max(0));
        } else {
            v["buffer_days"] = Value::Null;
            v["reason"] = json!("estimates_missing");
            v["tasks_missing_estimates"] = json!(gaps);
        }
        feeding.push(v);
    }
    out["feeding_buffers"] = json!(feeding);

    let people: BTreeSet<&str> = chain_pids.iter().filter_map(|p| c.people.get(p)).map(String::as_str).collect();
    let label: HashMap<&str, String> = people.iter().enumerate().map(|(i, p)| (*p, format!("person-{}", i + 1))).collect();
    let alerts: Vec<Value> = chain_pids
        .iter()
        .filter_map(|p| {
            let person = c.people.get(p)?;
            let r = when.get(p)?;
            let mut v = json!({
                "task": titled(p), "starts": r.early_start,
                "alert_on": cal.date_of(r.es_index - lead),
                "person": label[person.as_str()],
            });
            if super::availability::may_manage_person(&caller, person) {
                v["person_ref"] = json!(person);
            } else {
                v["person_ref"] = Value::Null;
                v["masked"] = json!(true);
            }
            Some(v)
        })
        .collect();
    out["resource_buffers"] = json!(alerts);
    format::json(out)
}

/// The buffer routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/critical-chain-buffers", get(show))
}

