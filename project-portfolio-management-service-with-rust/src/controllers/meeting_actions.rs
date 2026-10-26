//! Action items from minutes to tasks (BR-3). The service runs no model: a person or an external agent proposes tasks
//! from a held meeting's minutes, the service verifies every proposal and creates the chosen ones once. Pure rules in
//! [`crate::meeting_actions`].
//!
//! * `POST /api/plans/{pid}/meetings/{mid}/task-proposals/preview`: judge the proposals; nothing is written;
//! * `POST …/task-proposals/commit`: create them all in one transaction, or none. Send an `Idempotency-Key` (BR-1) so a
//!   retry returns the first answer; the table also refuses a second task for the same outcome.

use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{conflict, db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::meeting_actions::{self as rules, Meeting, ProposalIn, Verdict};
use crate::models::_entities::tasks;
use crate::models::audit_logs::Model as AuditModel;

/// Both routes take this body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ProposalsBody {
    /// The proposed tasks (at most 20).
    items: Vec<ProposalIn>,
}

/// What the rules need to know about the meeting, read once.
struct Facts {
    meeting: crate::models::_entities::meetings::Model,
    kinds: Vec<String>,
    taken: Vec<usize>,
    people: Vec<String>,
}

async fn column(
    db: &impl ConnectionTrait,
    sql: &str,
    pid: Uuid,
) -> Result<Vec<sea_orm::QueryResult>> {
    db.query_all_raw(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        sql,
        [pid.into()],
    ))
    .await
    .map_err(db_err)
}

async fn facts(
    db: &impl ConnectionTrait,
    ctx: &AppContext,
    plan_pid: Uuid,
    mid: &str,
) -> Result<Facts> {
    let meeting = super::meetings::find(ctx, plan_pid, mid).await?;
    if meeting.status != "held" || meeting.minutes.as_deref().is_none_or(str::is_empty) {
        return Err(conflict(
            "only a held meeting with minutes has outcomes to propose tasks for",
        ));
    }
    let kinds = column(
        db,
        "SELECT outcome_kind AS v FROM meeting_items WHERE meeting_pid = $1 AND part = 'outcome' ORDER BY id",
        meeting.pid,
    )
    .await?
    .iter()
    .map(|r| r.try_get::<String>("", "v").unwrap_or_default())
    .collect();
    let taken = column(
        db,
        "SELECT outcome_no AS v FROM meeting_tasks WHERE meeting_pid = $1",
        meeting.pid,
    )
    .await?
    .iter()
    .filter_map(|r| r.try_get::<i32>("", "v").ok())
    .filter_map(|n| usize::try_from(n).ok())
    .collect();
    // The plan's people: whoever is assigned a live task of the plan or holds a RACI letter on it.
    let people = column(
        db,
        "SELECT assignee_ref AS v FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL AND assignee_ref IS NOT NULL \
         UNION SELECT person_ref FROM raci_assignments WHERE plan_pid = $1 AND deleted_at IS NULL",
        plan_pid,
    )
    .await?
    .iter()
    .filter_map(|r| r.try_get::<String>("", "v").ok())
    .collect();
    Ok(Facts {
        meeting,
        kinds,
        taken,
        people,
    })
}

fn check_shape(items: &[ProposalIn]) -> Result<()> {
    if items.is_empty() || items.len() > rules::MAX_PROPOSALS {
        return Err(unprocessable(&format!(
            "send between 1 and {} proposals",
            rules::MAX_PROPOSALS
        )));
    }
    Ok(())
}

fn verdicts(items: &[ProposalIn], f: &Facts) -> Vec<Verdict> {
    let meeting = Meeting {
        minutes: f.meeting.minutes.as_deref().unwrap_or_default(),
        on: f.meeting.held_on.unwrap_or(f.meeting.scheduled_on),
        outcome_kinds: &f.kinds,
        taken: &f.taken,
    };
    let mut claimed: Vec<usize> = vec![];
    items
        .iter()
        .map(|p| {
            let v = rules::judge(p, &meeting, &f.people);
            // Two proposals for one outcome in the same request: the first keeps it.
            if !matches!(v, Verdict::Dropped(_)) && claimed.contains(&p.outcome) {
                return Verdict::Dropped("outcome_proposed_twice");
            }
            if !matches!(v, Verdict::Dropped(_)) {
                claimed.push(p.outcome);
            }
            v
        })
        .collect()
}

fn shown(index: usize, v: &Verdict) -> Value {
    match v {
        Verdict::Ready(a) => json!({ "index": index, "status": "ready", "assignee": a }),
        Verdict::Dropped(why) => json!({ "index": index, "status": "dropped", "reason": why }),
    }
}

/// `POST …/task-proposals/preview`: judge each proposal against the minutes and the plan; nothing is written.
#[debug_handler]
async fn preview(
    State(ctx): State<AppContext>,
    Path((pid, mid)): Path<(String, String)>,
    Json(b): Json<ProposalsBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    check_shape(&b.items)?;
    let f = facts(&ctx.db, &ctx, plan.pid, &mid).await?;
    let v = verdicts(&b.items, &f);
    let count = |pred: fn(&Verdict) -> bool| v.iter().filter(|x| pred(x)).count();
    format::json(json!({
        "meeting": f.meeting.pid,
        "counts": {
            "ready": count(|x| matches!(x, Verdict::Ready(_))),
            "dropped": count(|x| matches!(x, Verdict::Dropped(_))),
        },
        "items": v.iter().enumerate().map(|(i, x)| shown(i, x)).collect::<Vec<_>>(),
    }))
}

/// `POST …/task-proposals/commit`: create every proposed task in one transaction, or none (audited). A proposal that
/// is not ready, or an outcome that already has a task, is `422` and nothing is written.
#[debug_handler]
async fn commit(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, mid)): Path<(String, String)>,
    Json(b): Json<ProposalsBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    check_shape(&b.items)?;
    let f = facts(&ctx.db, &ctx, plan.pid, &mid).await?;
    let v = verdicts(&b.items, &f);
    let refused: Vec<String> = v
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            Verdict::Ready(_) => None,
            Verdict::Dropped(why) => Some(format!("item {i}: {why}")),
        })
        .collect();
    if !refused.is_empty() {
        return Err(unprocessable(&refused.join("; ")));
    }
    let workflow = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let initial = workflow
        .states
        .iter()
        .find(|s| s.is_initial)
        .map_or("todo", |s| s.key.as_str())
        .to_string();
    let now = chrono::Utc::now();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let mut created = vec![];
    for (p, verdict) in b.items.iter().zip(&v) {
        let Verdict::Ready(assignee) = verdict else {
            continue;
        };
        let task = tasks::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            sprint_pid: ActiveValue::set(None),
            title: ActiveValue::set(p.title.trim().to_string()),
            description: ActiveValue::set(Some(format!(
                "Created from the minutes of the meeting \"{}\".",
                f.meeting.title
            ))),
            status: ActiveValue::set(initial.clone()),
            assignee_ref: ActiveValue::set(assignee.clone()),
            points: ActiveValue::set(p.points),
            due_date: ActiveValue::set(p.due_on),
            status_changed_at: ActiveValue::set(now.into()),
            flow_type: ActiveValue::set(None),
            done_at: ActiveValue::set(None),
            deleted_at: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?;
        crate::tba::transition_row(
            task.pid,
            plan.pid,
            None,
            initial.clone(),
            now,
            caller.actor().map(ToString::to_string),
            assignee.clone(),
        )
        .insert(&txn)
        .await
        .map_err(db_err)?;
        let outcome_no = i32::try_from(p.outcome).unwrap_or(i32::MAX);
        txn.execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "INSERT INTO meeting_tasks (meeting_pid, outcome_no, task_pid) VALUES ($1, $2, $3)",
            [f.meeting.pid.into(), outcome_no.into(), task.pid.into()],
        ))
        .await
        .map_err(|e| {
            if e.to_string().contains("meeting_tasks_outcome_once") {
                conflict("an outcome of this meeting already has a task")
            } else {
                db_err(e)
            }
        })?;
        created.push(json!({ "index": created.len(), "outcome": p.outcome, "task_pid": task.pid }));
    }
    AuditModel::record(
        &txn,
        plan.pid,
        "meeting_tasks_created",
        caller.actor(),
        Some(json!({ "meeting": f.meeting.pid, "tasks": created })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({ "meeting": f.meeting.pid, "created": created }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add(
            "/plans/{pid}/meetings/{mid}/task-proposals/preview",
            post(preview),
        )
        .add(
            "/plans/{pid}/meetings/{mid}/task-proposals/commit",
            post(commit),
        )
}
