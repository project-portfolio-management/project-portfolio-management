//! Action items from minutes to tasks (BR-3). The service runs no model: a person or an external agent proposes tasks
//! from a held meeting's minutes, the service verifies every proposal and creates the chosen ones once. Pure rules in
//! [`crate::meeting_actions`].
//!
//! * `POST /api/plans/{pid}/meetings/{mid}/task-proposals/preview`: judge the proposals; nothing is written;
//! * `POST …/task-proposals/commit`: create them all in one transaction, or none. Send an `Idempotency-Key` (BR-1) so a
//!   retry returns the first answer; the table also refuses a second task for the same outcome.

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{conflict, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::meeting_actions::{self as rules, Meeting, ProposalIn, Verdict};
use crate::models::audit_logs::Model as AuditModel;

/// Both routes take this body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ProposalsBody {
    /// The proposed tasks (at most 20).
    items: Vec<ProposalIn>,
}

/// What the rules need to know about the meeting, read once.
struct Facts {
    /// The meeting row (must be held and have minutes).
    meeting: crate::models::_entities::meetings::Model,
    /// Outcome kinds of the meeting's outcome items, in order; an outcome's index is its position here.
    kinds: Vec<String>,
    /// Outcome numbers that already produced a task, so the same outcome is not proposed twice across requests.
    taken: Vec<usize>,
    /// People known to the plan (task assignees and RACI holders) as references; proposed assignees must be among them.
    people: Vec<String>,
}

/// Runs a single-parameter SQL query (bound to `pid`) and returns its one column, `None` where it is null; used for the
/// small lookups the proposal rules need.
async fn column<T>(ctx: &AppContext, sql: &'static str, pid: Uuid) -> Result<Vec<Option<T>>>
where
    T: Send + Unpin + for<'r> sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
{
    Ok(sqlx::query_scalar(sql)
        .bind(pid)
        .fetch_all(crate::db::pool(ctx))
        .await?)
}

/// Reads the meeting and the facts the proposal rules judge against. Refuses (409) unless the meeting was held and has minutes.
async fn facts(ctx: &AppContext, plan_pid: Uuid, mid: &str) -> Result<Facts> {
    let meeting = super::meetings::find(ctx, plan_pid, mid).await?;
    if meeting.status != "held" || meeting.minutes.as_deref().is_none_or(str::is_empty) {
        return Err(conflict(
            "only a held meeting with minutes has outcomes to propose tasks for",
        ));
    }
    let kinds = column::<String>(
        ctx,
        "SELECT outcome_kind AS v FROM meeting_items WHERE meeting_pid = $1 AND part = 'outcome' ORDER BY id",
        meeting.pid,
    )
    .await?
    .into_iter()
    .map(Option::unwrap_or_default)
    .collect();
    let taken = column::<i32>(
        ctx,
        "SELECT outcome_no AS v FROM meeting_tasks WHERE meeting_pid = $1",
        meeting.pid,
    )
    .await?
    .into_iter()
    .flatten()
    .filter_map(|n| usize::try_from(n).ok())
    .collect();
    // The plan's people: whoever is assigned a live task of the plan or holds a RACI letter on it.
    let people = column::<String>(
        ctx,
        "SELECT assignee_ref AS v FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL AND assignee_ref IS NOT NULL \
         UNION SELECT person_ref FROM raci_assignments WHERE plan_pid = $1 AND deleted_at IS NULL",
        plan_pid,
    )
    .await?
    .into_iter()
    .flatten()
    .collect();
    Ok(Facts {
        meeting,
        kinds,
        taken,
        people,
    })
}

/// Rejects an empty proposal list or one over the per-request maximum.
fn check_shape(items: &[ProposalIn]) -> Result<()> {
    if items.is_empty() || items.len() > rules::MAX_PROPOSALS {
        return Err(unprocessable(&format!(
            "send between 1 and {} proposals",
            rules::MAX_PROPOSALS
        )));
    }
    Ok(())
}

/// Judges each proposal against the meeting and the plan's people; a second proposal for an outcome already claimed in this request is dropped.
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

/// Renders one verdict for the response, keyed by the proposal's index in the request.
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
    let f = facts(&ctx, plan.pid, &mid).await?;
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
    let f = facts(&ctx, plan.pid, &mid).await?;
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
    let now: chrono::DateTime<chrono::FixedOffset> = now.into();
    let mut tx = crate::db::begin(&ctx).await?;
    let mut created = vec![];
    for (p, verdict) in b.items.iter().zip(&v) {
        let Verdict::Ready(assignee) = verdict else {
            continue;
        };
        let task_pid: Uuid = sqlx::query_scalar(
            "INSERT INTO tasks
                 (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points, due_date,
                  status_changed_at, flow_type, done_at, deleted_at)
             VALUES ($1, $2, NULL, $3, $4, $5, $6, $7, $8, $9, NULL, NULL, NULL)
             RETURNING pid",
        )
        .bind(Uuid::new_v4())
        .bind(plan.pid)
        .bind(p.title.trim())
        .bind(format!(
            "Created from the minutes of the meeting \"{}\".",
            f.meeting.title
        ))
        .bind(&initial)
        .bind(assignee)
        .bind(p.points)
        .bind(p.due_on)
        .bind(now)
        .fetch_one(tx.conn())
        .await?;
        // The task's creation in the transition log, as `crate::tba::insert_transition_on` writes it.
        sqlx::query(
            "INSERT INTO task_transitions
                 (pid, task_pid, plan_pid, from_status, to_status, at, actor_ref, assignee_ref, backfilled)
             VALUES ($1, $2, $3, NULL, $4, $5, $6, $7, false)",
        )
        .bind(Uuid::new_v4())
        .bind(task_pid)
        .bind(plan.pid)
        .bind(&initial)
        .bind(now)
        .bind(caller.actor())
        .bind(assignee)
        .execute(tx.conn())
        .await?;
        let outcome_no = i32::try_from(p.outcome).unwrap_or(i32::MAX);
        sqlx::query(
            "INSERT INTO meeting_tasks (meeting_pid, outcome_no, task_pid) VALUES ($1, $2, $3)",
        )
        .bind(f.meeting.pid)
        .bind(outcome_no)
        .bind(task_pid)
        .execute(tx.conn())
        .await
        .map_err(|e| {
            if e.to_string().contains("meeting_tasks_outcome_once") {
                conflict("an outcome of this meeting already has a task")
            } else {
                Error::from(e)
            }
        })?;
        created.push(json!({ "index": created.len(), "outcome": p.outcome, "task_pid": task_pid }));
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "meeting_tasks_created",
        caller.actor(),
        Some(json!({ "meeting": f.meeting.pid, "tasks": created })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
