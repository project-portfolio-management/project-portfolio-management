//! Meetings (NC-14): an agenda before, minutes and outcomes after, linked to the plan's items, printable as PDF.
//! Pure rules and printed text in [`crate::meetings`].
//!
//! * `POST /api/plans/{pid}/meetings`: plan a meeting with an agenda;
//! * `GET /api/plans/{pid}/meetings`: the plan's meetings, newest first;
//! * `GET /api/plans/{pid}/meetings/{mid}`: one meeting with its agenda, minutes and outcomes;
//! * `POST …/{mid}/held`: record that it was held, with minutes and outcomes (once; a record is not edited);
//! * `POST …/{mid}/cancel`: cancel a planned meeting;
//! * `GET …/{mid}/pdf`: the meeting as a PDF, from what the JSON shows and no more;
//! * `GET /api/plans/{pid}/references?kind=&pid=`: the meetings and lessons that point at one record of the plan (BR-4).

use crate::prelude::*;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::unprocessable;
use crate::auth::MaybeAuthUser;
use crate::db::Tx;
use crate::meetings::{self as rules, AgendaIn, Line, Link, OutcomeIn, Printable};
use crate::models::_entities::{lessons, meeting_items, meetings};
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
pub(super) async fn describe(
    ctx: &AppContext,
    plan_pid: Uuid,
    link: &Link,
) -> Result<Option<String>> {
    let Some((_, table, label, live)) = LINK_TABLES.iter().find(|(k, ..)| *k == link.kind) else {
        return Ok(None);
    };
    // The table, column and condition come from the constant above, never from the request.
    let sql = format!(
        "SELECT {label}::text AS label FROM {table} WHERE pid = $1 AND plan_pid = $2 AND {live} LIMIT 1"
    );
    let label: Option<Option<String>> = sqlx::query_scalar(sqlx::AssertSqlSafe(sql))
        .bind(link.pid)
        .bind(plan_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
    Ok(label.flatten())
}

/// Check every link belongs to the plan; the message names the first that does not.
async fn check_links(ctx: &AppContext, plan_pid: Uuid, links: &[&Link]) -> Result<()> {
    for link in links {
        if describe(ctx, plan_pid, link).await?.is_none() {
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
    check_links(&ctx, plan.pid, &links).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: meetings::Model = sqlx::query_as(
        "INSERT INTO meetings
             (pid, plan_pid, title, scheduled_on, status, minutes, held_on, created_by, held_by,
              deleted_at)
         VALUES ($1, $2, $3, $4, 'planned', NULL, NULL, $5, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(b.title.trim())
    .bind(b.scheduled_on)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    for a in &b.agenda {
        insert_item(&mut tx, row.pid, "agenda", None, &a.text, a.link.as_ref()).await?;
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "meeting_planned",
        caller.actor(),
        Some(json!({ "meeting": row.pid, "agenda_items": b.agenda.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "pid": row.pid, "status": row.status }))
}

/// Insert one agenda or outcome row for a meeting inside the caller's transaction. `part` is `agenda` or an outcome part; `kind` is the outcome kind (absent for agenda items); `link` optionally ties the row to another record.
async fn insert_item(
    tx: &mut Tx,
    meeting_pid: Uuid,
    part: &str,
    kind: Option<&str>,
    text: &str,
    link: Option<&Link>,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO meeting_items (meeting_pid, part, outcome_kind, text, link_kind, link_pid)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(meeting_pid)
    .bind(part)
    .bind(kind)
    .bind(text.trim())
    .bind(link.map(|l| l.kind.clone()))
    .bind(link.map(|l| l.pid))
    .execute(tx.conn())
    .await?;
    Ok(())
}

/// Load a live meeting by pid that belongs to `plan_pid`, or `NotFound`; a meeting of another plan is indistinguishable from a missing one.
pub(super) async fn find(ctx: &AppContext, plan_pid: Uuid, mid: &str) -> Result<meetings::Model> {
    sqlx::query_as(
        "SELECT * FROM meetings WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(super::parse_pid(mid)?)
    .bind(plan_pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)
}

/// `GET /api/plans/{pid}/meetings`: the plan's meetings, newest first, without their items.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows: Vec<meetings::Model> = sqlx::query_as(
        "SELECT * FROM meetings WHERE plan_pid = $1 AND deleted_at IS NULL
         ORDER BY scheduled_on DESC, id DESC",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
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
    /// Printable agenda lines, in entry order.
    agenda: Vec<Line>,
    /// Printable outcome lines, in entry order.
    outcomes: Vec<Line>,
    /// The agenda as JSON values, in entry order.
    agenda_json: Vec<serde_json::Value>,
    /// The outcomes as JSON values, in entry order.
    outcomes_json: Vec<serde_json::Value>,
}

/// Load a meeting's agenda and outcome rows in entry order, describing each linked record as it is now. A link whose record is gone is kept and shown as no longer there.
async fn load_items(ctx: &AppContext, plan_pid: Uuid, meeting_pid: Uuid) -> Result<Items> {
    let rows: Vec<meeting_items::Model> =
        sqlx::query_as("SELECT * FROM meeting_items WHERE meeting_pid = $1 ORDER BY id")
            .bind(meeting_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;
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
                    ctx,
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
    let made: Vec<(Option<i32>, Option<Uuid>)> =
        sqlx::query_as("SELECT outcome_no, task_pid FROM meeting_tasks WHERE meeting_pid = $1")
            .bind(m.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut outcomes = items.outcomes_json;
    for (i, o) in outcomes.iter_mut().enumerate() {
        let task = made.iter().find_map(|(no, task)| {
            ((*no)? == i32::try_from(i).ok()?)
                .then_some(*task)
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
    check_links(&ctx, plan.pid, &links).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    // Claimed by a conditional update, so two callers cannot both record the same meeting.
    let claimed = sqlx::query(
        "UPDATE meetings SET status = 'held', minutes = $1, held_on = $2, held_by = $3
         WHERE pid = $4 AND status = 'planned'",
    )
    .bind(b.minutes.trim())
    .bind(b.held_on.unwrap_or(m.scheduled_on))
    .bind(caller.actor())
    .bind(m.pid)
    .execute(tx.conn())
    .await?;
    if claimed.rows_affected() == 0 {
        return Err(super::conflict("this meeting is already held or cancelled"));
    }
    for o in &b.outcomes {
        insert_item(
            &mut tx,
            m.pid,
            "outcome",
            Some(&o.kind),
            &o.text,
            o.link.as_ref(),
        )
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "meeting_held",
        caller.actor(),
        Some(json!({ "meeting": m.pid, "outcomes": b.outcomes.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let mut tx = crate::db::begin(&ctx).await?;
    let claimed = sqlx::query(
        "UPDATE meetings SET status = 'cancelled' WHERE pid = $1 AND status = 'planned'",
    )
    .bind(m.pid)
    .execute(tx.conn())
    .await?;
    if claimed.rows_affected() == 0 {
        return Err(super::conflict("this meeting is already held or cancelled"));
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "meeting_cancelled",
        caller.actor(),
        Some(json!({ "meeting": m.pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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

/// One meeting item that points at the record, with its meeting.
#[derive(Debug, sqlx::FromRow)]
struct Reference {
    /// The meeting's pid.
    meeting_pid: Option<Uuid>,
    /// The meeting's title.
    meeting_title: Option<String>,
    /// The meeting's status.
    meeting_status: Option<String>,
    /// The date the meeting is scheduled for.
    scheduled_on: Option<chrono::NaiveDate>,
    /// `agenda` or `outcome`.
    part: Option<String>,
    /// The outcome's kind; none on an agenda item.
    outcome_kind: Option<String>,
    /// The item's text.
    text: Option<String>,
}

/// `GET /api/plans/{pid}/references` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReferencesQuery {
    /// `task`, `risk`, `issue`, `milestone`, `change_request` or `cutover_decision`.
    kind: String,
    /// The record's `pid`.
    pid: Uuid,
}

/// `GET /api/plans/{pid}/references?kind=&pid=`: the meetings and the lessons that point at one record of the plan.
/// The plan guard runs first (a plan the caller cannot read is a `404`), then the target must be a live record of
/// this plan, else it answers as one that does not exist; only then are references read, and `count` is the length
/// of what is listed (at most 100 meetings and 100 lessons).
#[debug_handler]
async fn references(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<ReferencesQuery>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let link = Link {
        kind: q.kind.clone(),
        pid: q.pid,
    };
    let Some(title) = describe(&ctx, plan.pid, &link).await? else {
        return Err(Error::NotFound);
    };
    let rows: Vec<Reference> = sqlx::query_as(
        "SELECT m.pid AS meeting_pid, m.title AS meeting_title, m.status AS meeting_status, \
                m.scheduled_on AS scheduled_on, i.part AS part, i.outcome_kind AS outcome_kind, i.text AS text \
         FROM meeting_items i JOIN meetings m ON m.pid = i.meeting_pid \
         WHERE m.plan_pid = $1 AND m.deleted_at IS NULL AND i.link_kind = $2 AND i.link_pid = $3 \
         ORDER BY m.scheduled_on DESC, i.id LIMIT 100",
    )
    .bind(plan.pid)
    .bind(&q.kind)
    .bind(q.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let meetings: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            json!({
                "meeting_pid": r.meeting_pid,
                "meeting_title": r.meeting_title,
                "meeting_status": r.meeting_status,
                "scheduled_on": r.scheduled_on,
                "part": r.part,
                "outcome_kind": r.outcome_kind,
                "text": r.text,
            })
        })
        .collect();
    let lessons: Vec<serde_json::Value> = sqlx::query_as::<_, lessons::Model>(
        "SELECT * FROM lessons
         WHERE plan_pid = $1 AND deleted_at IS NULL AND link_kind = $2 AND link_pid = $3
         ORDER BY id LIMIT 100",
    )
    .bind(plan.pid)
    .bind(&q.kind)
    .bind(q.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .into_iter()
        .map(|l| json!({ "lesson_pid": l.pid, "occasion": l.occasion, "category": l.category, "summary": l.summary }))
        .collect();
    format::json(json!({
        "target": { "kind": q.kind, "pid": q.pid, "title": title },
        "meetings": meetings,
        "lessons": lessons,
        "count": meetings.len() + lessons.len(),
    }))
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
        .add("/plans/{pid}/references", get(references))
}
