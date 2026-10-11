//! The critical chain (CC-1): `GET /api/plans/{pid}/critical-chain`. The rules are [`crate::critical_chain`];
//! the levelled schedule it reads is [`crate::leveling`] with critical tasks allowed to move (a chain is
//! what is left once people's time is respected, so the finish may move). Changes nothing.
//!
//! The critical path is returned beside the chain and never blended with it. A person is named only to a
//! caller who may manage them; anyone else sees `person-N`.

use std::collections::{BTreeSet, HashMap};

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::cpm::{self, Options};
use crate::critical_chain::{self, Outcome};

const NOTE: &str = "This is advice; nothing has been changed. The critical chain is the longest chain through dependencies and resource links on a schedule levelled for people's time (critical tasks may move, so the finish may). A resource link means a task started when its person came free. The critical_path is the longest chain through dependencies alone, on the schedule as entered; the two are not blended. Only finish-to-start dependencies are followed as links, and milestones are not resource links. Each unfinished assigned task asks 100 percent of its assignee on each day it works.";

/// The schedule as entered, the nodes with the leveling moves applied as planned starts, and the schedule
/// those give.
pub(crate) fn schedules(
    c: &super::leveling::Computed,
) -> Result<(cpm::CpmResult, Vec<cpm::Node>, cpm::CpmResult)> {
    let net = &c.net;
    let cycle = || {
        Error::CustomError(
            StatusCode::UNPROCESSABLE_ENTITY,
            ErrorDetail::new("unprocessable_entity", "the dependencies contain a cycle"),
        )
    };
    let options = Options::default();
    let base = cpm::analyze_with(&net.nodes, &net.edges, &net.plan_cal, &net.own, options)
        .map_err(|cpm::CpmError::Cycle(_)| cycle())?;
    let starts: HashMap<Uuid, chrono::NaiveDate> = c
        .proposal
        .moves
        .iter()
        .map(|m| (m.pid, m.to_start))
        .collect();
    let nodes: Vec<cpm::Node> = net
        .nodes
        .iter()
        .map(|n| {
            let mut n = *n;
            if let Some(d) = starts.get(&n.pid) {
                n.planned_start = Some(*d);
            }
            n
        })
        .collect();
    let levelled = cpm::analyze_with(&nodes, &net.edges, &net.plan_cal, &net.own, options)
        .map_err(|cpm::CpmError::Cycle(_)| cycle())?;
    Ok((base, nodes, levelled))
}

/// `GET /api/plans/{pid}/critical-chain`
#[debug_handler]
async fn show(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let c = super::leveling::compute(&ctx, plan.pid, true, 100).await?;
    let (base, nodes, levelled) = schedules(&c)?;
    let net = &c.net;
    let titled = |p: &Uuid| json!({ "pid": p, "title": c.titles.get(p) });
    let critical_path: Vec<Value> = base
        .chains
        .first()
        .into_iter()
        .flatten()
        .map(titled)
        .collect();
    let on_path: BTreeSet<Uuid> = base.chains.iter().flatten().copied().collect();
    let label: HashMap<&str, String> = c
        .people
        .values()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, format!("person-{}", i + 1)))
        .collect();
    let when: HashMap<Uuid, &cpm::NodeResult> = levelled.nodes.iter().map(|n| (n.pid, n)).collect();
    let mut out = json!({
        "advisory": true,
        "calendar": net.cal_source,
        "critical_path": { "tasks": critical_path, "finish": base.project_finish, "chains": base.chains.len() },
        "levelled_finish": levelled.project_finish,
        "finish_delay_days": c.proposal.finish_delay_days,
        "over_allocated_after": c.proposal.unresolved.len(),
        "note": NOTE,
    });
    match critical_chain::find(&nodes, &net.edges, &levelled, &c.people) {
        Outcome::NoCriticalChain { reason } => {
            out["state"] = json!("no_critical_chain");
            out["reason"] = json!(reason);
        }
        Outcome::Chain(ch) => {
            out["state"] = json!("chain");
            out["chain_days"] = json!(ch.chain_days);
            out["crosses_paths"] = json!(ch.crosses_paths);
            out["tied_chains"] = json!(ch.tied_chains);
            out["tasks"] = json!(
                ch.tasks
                    .iter()
                    .map(|t| {
                        let r = when.get(&t.pid);
                        json!({
                            "pid": t.pid, "title": c.titles.get(&t.pid), "duration_days": t.duration_days,
                            "start": r.map(|r| r.early_start), "finish": r.map(|r| r.early_finish),
                            "on_critical_path": on_path.contains(&t.pid),
                        })
                    })
                    .collect::<Vec<_>>()
            );
            out["links"] = json!(
                ch.links
                    .iter()
                    .map(|l| {
                        let mut v = json!({ "from": l.from, "to": l.to, "kind": l.kind });
                        if let Some(person) = &l.person {
                            v["person"] = json!(label[person.as_str()]);
                            v["person_ref"] =
                                if super::availability::may_manage_person(&caller, person) {
                                    json!(person)
                                } else {
                                    v["masked"] = json!(true);
                                    Value::Null
                                };
                        }
                        v
                    })
                    .collect::<Vec<_>>()
            );
        }
    }
    format::json(out)
}

/// The critical chain routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/critical-chain", get(show))
}
