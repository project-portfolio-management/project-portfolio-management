//! Meetings (NC-14): an agenda before, minutes and outcomes after, linked to the plan's items, printable as PDF.
//! Pure rules and printed text in [`crate::meetings`].
//!
//! * `POST /api/plans/{pid}/meetings`: plan a meeting with an agenda;
//! * `GET /api/plans/{pid}/meetings`: the plan's meetings, newest first;
//! * `GET /api/plans/{pid}/meetings/{mid}`: one meeting with its agenda, minutes and outcomes;
//! * `POST …/{mid}/held`: record that it was held, with minutes and outcomes (once; a record is not edited);
//! * `POST …/{mid}/cancel`: cancel a planned meeting;
//! * `GET …/{mid}/pdf`: the meeting as a PDF, from what the JSON shows and no more.

use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryOrder, Statement};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::{db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::meetings::{self as rules, AgendaIn, Line, Link, OutcomeIn, Printable};
use crate::models::_entities::{meeting_items, meetings};
use crate::models::audit_logs::Model as AuditModel;

/// Where each kind of link lives: kind, table, the column that names the record, and the condition for "still there".
const LINK_TABLES: &[(&str, &str, &str, &str)] = &[
    ("task", "tasks", "title", "deleted_at IS NULL"),
    ("risk", "risks", "title", "deleted_at IS NULL"),
    ("issue", "issues", "title", "deleted_at IS NULL"),
    ("milestone", "milestones", "name", "deleted_at IS NULL"),
    (
        "change_request",
        "change_requests",
        "title",
        "deleted_at IS NULL",
    ),
    ("cutover_decision", "cutover_decisions", "title", "TRUE"),
];

/// The name of the record a link points at, when it belongs to `plan_pid` and is still there.
async fn describe(
    db: &impl ConnectionTrait,
    plan_pid: Uuid,
    link: &Link,
) -> Result<Option<String>> {
    let Some((_, table, label, live)) = LINK_TABLES.iter().find(|(k, ..)| *k == link.kind) else {
        return Ok(None);
    };
    // The table, column and condition come from the constant above, never from the request.
    let sql = format!(
        "SELECT {label}::text AS label FROM {table} WHERE pid = $1 AND plan_pid = $2 AND {live}"
    );
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [link.pid.into(), plan_pid.into()],
        ))
        .await
        .map_err(db_err)?;
    Ok(row.and_then(|r| r.try_get::<String>("", "label").ok()))
}

/// Check every link belongs to the plan; the message names the first that does not.
async fn check_links(db: &impl ConnectionTrait, plan_pid: Uuid, links: &[&Link]) -> Result<()> {
    for link in links {
        if describe(db, plan_pid, link).await?.is_none() {
            return Err(unprocessable(&format!(
                "{} {} is not a record of this plan",
                link.kind, link.pid
            )));
        }
    }
    Ok(())
}

/// `POST /api/plans/{pid}/meetings` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct PlanBody {
    /// The meeting's title.
    title: String,
    /// The date it is scheduled for.
    scheduled_on: chrono::NaiveDate,
    /// The agenda, in order.
    #[serde(default)]
    agenda: Vec<AgendaIn>,
}

/// `POST /api/plans/{pid}/meetings`: plan a meeting with its agenda (audited).
#[debug_handler]
async fn plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<PlanBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let found = rules::problems_planned(&b.title, &b.agenda);
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    let links: Vec<&Link> = b.agenda.iter().filter_map(|a| a.link.as_ref()).collect();
    check_links(&ctx.db, plan.pid, &links).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = meetings::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        title: ActiveValue::set(b.title.trim().to_string()),
        scheduled_on: ActiveValue::set(b.scheduled_on),
        status: ActiveValue::set("planned".to_string()),
        minutes: ActiveValue::set(None),
        held_on: ActiveValue::set(None),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        held_by: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    for a in &b.agenda {
        insert_item(&txn, row.pid, "agenda", None, &a.text, a.link.as_ref()).await?;
    }
    AuditModel::record(
        &txn,
        plan.pid,
        "meeting_planned",
        caller.actor(),
        Some(json!({ "meeting": row.pid, "agenda_items": b.agenda.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({ "pid": row.pid, "status": row.status }))
}

async fn insert_item(
    db: &impl ConnectionTrait,
    meeting_pid: Uuid,
    part: &str,
    kind: Option<&str>,
    text: &str,
    link: Option<&Link>,
) -> Result<()> {
    meeting_items::ActiveModel {
        meeting_pid: ActiveValue::set(meeting_pid),
        part: ActiveValue::set(part.to_string()),
        outcome_kind: ActiveValue::set(kind.map(str::to_string)),
        text: ActiveValue::set(text.trim().to_string()),
        link_kind: ActiveValue::set(link.map(|l| l.kind.clone())),
        link_pid: ActiveValue::set(link.map(|l| l.pid)),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(db_err)?;
    Ok(())
}

pub(super) async fn find(ctx: &AppContext, plan_pid: Uuid, mid: &str) -> Result<meetings::Model> {
    meetings::Entity::find()
        .filter(meetings::Column::Pid.eq(super::parse_pid(mid)?))
        .filter(meetings::Column::PlanPid.eq(plan_pid))
        .filter(meetings::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// `GET /api/plans/{pid}/meetings`: the plan's meetings, newest first, without their items.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = meetings::Entity::find()
        .filter(meetings::Column::PlanPid.eq(plan.pid))
        .filter(meetings::Column::DeletedAt.is_null())
        .order_by_desc(meetings::Column::ScheduledOn)
        .order_by_desc(meetings::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let items: Vec<serde_json::Value> = rows
        .iter()
        .map(|m| {
            json!({
                "pid": m.pid, "title": m.title, "scheduled_on": m.scheduled_on,
                "status": m.status, "held_on": m.held_on,
            })
        })
        .collect();
    format::json(items)
}

/// A meeting's items: printable lines and JSON, each for the agenda and the outcomes, every link described.
struct Items {
    agenda: Vec<Line>,
    outcomes: Vec<Line>,
    agenda_json: Vec<serde_json::Value>,
    outcomes_json: Vec<serde_json::Value>,
}

async fn load_items(ctx: &AppContext, plan_pid: Uuid, meeting_pid: Uuid) -> Result<Items> {
    let rows = meeting_items::Entity::find()
        .filter(meeting_items::Column::MeetingPid.eq(meeting_pid))
        .order_by_asc(meeting_items::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut items = Items {
        agenda: vec![],
        outcomes: vec![],
        agenda_json: vec![],
        outcomes_json: vec![],
    };
    for r in rows {
        let link = match (&r.link_kind, r.link_pid) {
            (Some(kind), Some(pid)) => {
                let name = describe(
                    &ctx.db,
                    plan_pid,
                    &Link {
                        kind: kind.clone(),
                        pid,
                    },
                )
                .await?;
                Some((kind.clone(), pid, name))
            }
            _ => None,
        };
        let printed = Line {
            kind: r.outcome_kind.clone().unwrap_or_default(),
            text: r.text.clone(),
            link: link
                .as_ref()
                .map(|(k, _, n)| format!("{k}: {}", n.as_deref().unwrap_or("(no longer there)"))),
        };
        let value = json!({
            "kind": r.outcome_kind, "text": r.text,
            "link": link.as_ref().map(|(k, p, n)| json!({ "kind": k, "pid": p, "name": n })),
        });
        if r.part == "agenda" {
            items.agenda.push(printed);
            items.agenda_json.push(value);
        } else {
            items.outcomes.push(printed);
            items.outcomes_json.push(value);
        }
    }
    Ok(items)
}

/// `GET /api/plans/{pid}/meetings/{mid}`: one meeting with its agenda, minutes and outcomes, each linked record
/// named as it is now (or marked as no longer there).
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    Path((pid, mid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m = find(&ctx, plan.pid, &mid).await?;
    let items = load_items(&ctx, plan.pid, m.pid).await?;
    // Each outcome says which task was created from it (BR-3), by its position among the outcomes.
    let made = ctx
        .db
        .query_all_raw(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT outcome_no, task_pid FROM meeting_tasks WHERE meeting_pid = $1",
            [m.pid.into()],
        ))
        .await
        .map_err(db_err)?;
    let mut outcomes = items.outcomes_json;
    for (i, o) in outcomes.iter_mut().enumerate() {
        let task = made.iter().find_map(|r| {
            (r.try_get::<i32>("", "outcome_no").ok()? == i32::try_from(i).ok()?)
                .then(|| r.try_get::<Uuid>("", "task_pid").ok())
                .flatten()
        });
        o["index"] = json!(i);
        o["task_pid"] = json!(task);
    }
    format::json(json!({
        "pid": m.pid, "title": m.title, "scheduled_on": m.scheduled_on, "status": m.status,
        "held_on": m.held_on, "minutes": m.minutes,
        "agenda": items.agenda_json, "outcomes": outcomes,
    }))
}

/// `POST …/{mid}/held` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct HeldBody {
    /// What was said and agreed.
    minutes: String,
    /// The date it was held (default: the scheduled date).
    #[serde(default)]
    held_on: Option<chrono::NaiveDate>,
    /// Decisions, actions and notes, in order.
    #[serde(default)]
    outcomes: Vec<OutcomeIn>,
}

/// `POST /api/plans/{pid}/meetings/{mid}/held`: record the minutes and outcomes of a planned meeting, once (audited).
/// A meeting that is held or cancelled is a record and answers `409`.
#[debug_handler]
async fn held(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, mid)): Path<(String, String)>,
    Json(b): Json<HeldBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m = find(&ctx, plan.pid, &mid).await?;
    let found = rules::problems_held(&b.minutes, &b.outcomes);
    if !found.is_empty() {
        return Err(unprocessable(&found.join("; ")));
    }
    let links: Vec<&Link> = b.outcomes.iter().filter_map(|o| o.link.as_ref()).collect();
    check_links(&ctx.db, plan.pid, &links).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    // Claimed by a conditional update, so two callers cannot both record the same meeting.
    let claimed = meetings::Entity::update_many()
        .col_expr(
            meetings::Column::Status,
            sea_orm::sea_query::Expr::value("held"),
        )
        .col_expr(
            meetings::Column::Minutes,
            sea_orm::sea_query::Expr::value(b.minutes.trim().to_string()),
        )
        .col_expr(
            meetings::Column::HeldOn,
            sea_orm::sea_query::Expr::value(b.held_on.unwrap_or(m.scheduled_on)),
        )
        .col_expr(
            meetings::Column::HeldBy,
            sea_orm::sea_query::Expr::value(caller.actor().map(str::to_string)),
        )
        .filter(meetings::Column::Pid.eq(m.pid))
        .filter(meetings::Column::Status.eq("planned"))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    if claimed.rows_affected == 0 {
        return Err(super::conflict("this meeting is already held or cancelled"));
    }
    for o in &b.outcomes {
        insert_item(
            &txn,
            m.pid,
            "outcome",
            Some(&o.kind),
            &o.text,
            o.link.as_ref(),
        )
        .await?;
    }
    AuditModel::record(
        &txn,
        plan.pid,
        "meeting_held",
        caller.actor(),
        Some(json!({ "meeting": m.pid, "outcomes": b.outcomes.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({ "pid": m.pid, "status": "held" }))
}

/// `POST /api/plans/{pid}/meetings/{mid}/cancel`: cancel a planned meeting (audited); `409` once held or cancelled.
#[debug_handler]
async fn cancel(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, mid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m = find(&ctx, plan.pid, &mid).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let claimed = meetings::Entity::update_many()
        .col_expr(
            meetings::Column::Status,
            sea_orm::sea_query::Expr::value("cancelled"),
        )
        .filter(meetings::Column::Pid.eq(m.pid))
        .filter(meetings::Column::Status.eq("planned"))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    if claimed.rows_affected == 0 {
        return Err(super::conflict("this meeting is already held or cancelled"));
    }
    AuditModel::record(
        &txn,
        plan.pid,
        "meeting_cancelled",
        caller.actor(),
        Some(json!({ "meeting": m.pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(json!({ "pid": m.pid, "status": "cancelled" }))
}

/// `GET /api/plans/{pid}/meetings/{mid}/pdf`: the meeting as a PDF: agenda, and once held the minutes and outcomes.
/// It states the meeting's own dates and not the moment of rendering, so two renders are byte-identical.
#[debug_handler]
async fn pdf(
    State(ctx): State<AppContext>,
    Path((pid, mid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m = find(&ctx, plan.pid, &mid).await?;
    let items = load_items(&ctx, plan.pid, m.pid).await?;
    let printable = Printable {
        plan: plan.name.clone(),
        title: m.title.clone(),
        status: m.status.clone(),
        scheduled_on: m.scheduled_on.to_string(),
        held_on: m.held_on.map(|d| d.to_string()),
        agenda: items.agenda,
        minutes: m.minutes.clone(),
        outcomes: items.outcomes,
    };
    let bytes = crate::pdf::render(&format!("Meeting: {}", m.title), &rules::lines(&printable));
    Ok(super::pdf::respond(
        &format!("meeting-{}.pdf", m.pid),
        bytes,
    ))
}

/// The meeting routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/meetings", post(plan))
        .add("/plans/{pid}/meetings", get(list))
        .add("/plans/{pid}/meetings/{mid}", get(get_one))
        .add("/plans/{pid}/meetings/{mid}/held", post(held))
        .add("/plans/{pid}/meetings/{mid}/cancel", post(cancel))
        .add("/plans/{pid}/meetings/{mid}/pdf", get(pdf))
}
