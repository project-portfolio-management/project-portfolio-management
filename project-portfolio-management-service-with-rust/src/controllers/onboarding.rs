//! Onboarding and offboarding: checklists, the access record and close-out
//! readiness. Rules in [`crate::onboarding`]; handover of work is in
//! [`super::handover`].
//!
//! * **Cases** are one person joining or leaving a plan, or a plan being taken in
//!   or closed out; each starts from the kind's checklist (built-in, or a custom
//!   template) with due dates from the effective date.
//! * **Access records** are append-only: granted or revoked, by whom and when. This
//!   service records access; it cannot change an identity provider's accounts.
//! * `GET …/closeout` says what stands between a plan and being closed.
//!
//! Anything about **a person** (who joins or leaves, who holds what access) needs
//! the permission to manage people, as everywhere else; a plan's own intake and
//! close-out checklists do not name anyone and follow the plan.

use super::{conflict, forbidden};
use super::{db_err, unprocessable};
use chrono::{NaiveDate, Utc};
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::portfolio_resources::may_manage_people;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    access_records, allocations, onboarding_cases, onboarding_items, onboarding_templates, risks,
    tasks,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::onboarding::{self as rules, Kind, Seed};

fn need_people(caller: &MaybeAuthUser) -> Result<()> {
    if may_manage_people(caller) {
        Ok(())
    } else {
        Err(forbidden(
            "this is about people: it needs the permission to manage people",
        ))
    }
}

fn valid_person(r: &str) -> bool {
    super::governance::valid_ref(r, &["person", "worker"])
}

// ───────────────────────── templates ─────────────────────────

async fn items_for(ctx: &AppContext, kind: Kind) -> Result<(Vec<Seed>, bool)> {
    let custom = onboarding_templates::Entity::find()
        .filter(onboarding_templates::Column::Kind.eq(kind.token()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .and_then(|t| serde_json::from_value::<Vec<Seed>>(t.items).ok());
    Ok(match custom {
        Some(items) => (items, true),
        None => (rules::builtin(kind), false),
    })
}

const KINDS: [Kind; 4] = [
    Kind::PersonJoin,
    Kind::PersonLeave,
    Kind::PlanIntake,
    Kind::PlanCloseout,
];

/// `GET /api/onboarding/templates` — the checklist for each kind (custom or built-in).
#[debug_handler]
async fn list_templates(State(ctx): State<AppContext>) -> Result<Response> {
    let mut out = Vec::new();
    for k in KINDS {
        let (items, custom) = items_for(&ctx, k).await?;
        out.push(serde_json::json!({ "kind": k.token(), "custom": custom, "items": items }));
    }
    format::json(serde_json::json!({ "templates": out }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TemplateBody {
    items: Vec<Seed>,
}

/// `PUT /api/onboarding/templates/{kind}` — replace a kind's checklist.
#[debug_handler]
async fn put_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(kind): Path<String>,
    Json(b): Json<TemplateBody>,
) -> Result<Response> {
    need_people(&caller)?;
    let kind = Kind::parse(&kind).ok_or(Error::NotFound)?;
    let problems = rules::template_problems(&b.items);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let items = serde_json::to_value(&b.items).unwrap_or_default();
    let existing = onboarding_templates::Entity::find()
        .filter(onboarding_templates::Column::Kind.eq(kind.token()))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let pid = match existing {
        Some(t) => {
            let pid = t.pid;
            let mut a: onboarding_templates::ActiveModel = t.into();
            a.items = ActiveValue::set(items);
            a.update(&txn).await.map_err(db_err)?;
            pid
        }
        None => {
            onboarding_templates::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                kind: ActiveValue::set(kind.token().to_string()),
                items: ActiveValue::set(items),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?
            .pid
        }
    };
    AuditModel::record(
        &txn,
        pid,
        "onboarding_template_set",
        caller.actor(),
        Some(serde_json::json!({ "kind": kind.token(), "items": b.items.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(serde_json::json!({ "kind": kind.token(), "custom": true, "items": b.items }))
}

/// `DELETE /api/onboarding/templates/{kind}` — go back to the built-in checklist.
#[debug_handler]
async fn delete_template(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(kind): Path<String>,
) -> Result<Response> {
    need_people(&caller)?;
    let kind = Kind::parse(&kind).ok_or(Error::NotFound)?;
    onboarding_templates::Entity::delete_many()
        .filter(onboarding_templates::Column::Kind.eq(kind.token()))
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    format::empty_json()
}

// ───────────────────────── cases ─────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CaseBody {
    kind: String,
    /// The person, for a join or leave.
    #[serde(default)]
    subject_ref: Option<String>,
    effective_date: NaiveDate,
    #[serde(default)]
    title: Option<String>,
}

/// `POST /api/plans/{pid}/onboarding/cases` — start an onboarding or offboarding.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn create_case(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CaseBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let kind = Kind::parse(&b.kind).ok_or_else(|| {
        unprocessable("kind must be person_join, person_leave, plan_intake or plan_closeout")
    })?;
    if kind.is_person() {
        need_people(&caller)?;
        if !b.subject_ref.as_deref().is_some_and(valid_person) {
            return Err(unprocessable("subject_ref must be a person:/worker: URN"));
        }
    } else if b.subject_ref.is_some() {
        return Err(unprocessable("a plan checklist has no subject_ref"));
    }
    let year = chrono::Datelike::year(&b.effective_date);
    if !(1970..=2100).contains(&year) {
        return Err(unprocessable(
            "effective_date must be between 1970 and 2100",
        ));
    }
    if b.title
        .as_deref()
        .is_some_and(|t| t.trim().is_empty() || t.chars().count() > 200)
    {
        return Err(unprocessable("title is 1 to 200 characters"));
    }
    // One open case per (kind, plan, person): starting a second is a mistake, not a feature.
    let mut dup = onboarding_cases::Entity::find()
        .filter(onboarding_cases::Column::PlanPid.eq(plan.pid))
        .filter(onboarding_cases::Column::Kind.eq(kind.token()))
        .filter(onboarding_cases::Column::Status.eq("open"));
    if let Some(s) = &b.subject_ref {
        dup = dup.filter(onboarding_cases::Column::SubjectRef.eq(s.as_str()));
    }
    if dup.one(&ctx.db).await.map_err(db_err)?.is_some() {
        return Err(conflict(
            "an open checklist of this kind already exists here; finish or cancel it first",
        ));
    }
    let title = b.title.clone().unwrap_or_else(|| match kind {
        Kind::PersonJoin => format!("Onboarding into {}", plan.name),
        Kind::PersonLeave => format!("Offboarding from {}", plan.name),
        Kind::PlanIntake => format!("Taking in {}", plan.name),
        Kind::PlanCloseout => format!("Closing out {}", plan.name),
    });
    let (seeds, _) = items_for(&ctx, kind).await?;
    // Case, checklist and audit row land together or not at all.
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let case = onboarding_cases::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        kind: ActiveValue::set(kind.token().to_string()),
        plan_pid: ActiveValue::set(plan.pid),
        subject_ref: ActiveValue::set(b.subject_ref.clone()),
        title: ActiveValue::set(title),
        effective_date: ActiveValue::set(b.effective_date),
        status: ActiveValue::set("open".to_string()),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        closed_at: ActiveValue::set(None),
        closed_reason: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| {
        if e.to_string().contains("onboarding_cases_one_open") {
            conflict(
                "an open checklist of this kind already exists here; finish or cancel it first",
            )
        } else {
            db_err(e)
        }
    })?;
    for (i, s) in seeds.iter().enumerate() {
        onboarding_items::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            case_pid: ActiveValue::set(case.pid),
            position: ActiveValue::set(i32::try_from(i).unwrap_or(0)),
            title: ActiveValue::set(s.title.clone()),
            owner_ref: ActiveValue::set(None),
            due_date: ActiveValue::set(rules::due_date(b.effective_date, s.offset_days)),
            resolution: ActiveValue::set(None),
            note: ActiveValue::set(None),
            resolved_at: ActiveValue::set(None),
            resolved_by: ActiveValue::set(None),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?;
    }
    AuditModel::record(
        &txn,
        case.pid,
        "onboarding_case_opened",
        caller.actor(),
        Some(
            serde_json::json!({ "kind": kind.token(), "plan_pid": plan.pid,
            "subject_ref": case.subject_ref, "effective_date": case.effective_date }),
        ),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    case_view(&ctx, &caller, &case).await
}

async fn items_of(ctx: &AppContext, case: Uuid) -> Result<Vec<onboarding_items::Model>> {
    onboarding_items::Entity::find()
        .filter(onboarding_items::Column::CasePid.eq(case))
        .order_by_asc(onboarding_items::Column::Position)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

fn progress_of(items: &[onboarding_items::Model]) -> rules::Progress {
    let today = crate::tz::deployment_today();
    let states: Vec<rules::ItemState<'_>> = items
        .iter()
        .map(|i| rules::ItemState {
            resolution: i.resolution.as_deref(),
            due: i.due_date,
        })
        .collect();
    rules::progress(&states, today)
}

async fn case_view(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    c: &onboarding_cases::Model,
) -> Result<Response> {
    let items = items_of(ctx, c.pid).await?;
    let see = may_manage_people(caller);
    let today = crate::tz::deployment_today();
    format::json(serde_json::json!({
        "pid": c.pid, "kind": c.kind, "plan_pid": c.plan_pid, "title": c.title,
        "subject_ref": if see { c.subject_ref.clone() } else { None },
        "effective_date": c.effective_date, "status": c.status,
        "closed_at": c.closed_at, "closed_reason": c.closed_reason,
        "progress": progress_of(&items),
        "items": items.iter().map(|i| serde_json::json!({
            "pid": i.pid, "position": i.position, "title": i.title, "owner_ref": i.owner_ref,
            "due_date": i.due_date, "resolution": i.resolution, "note": i.note,
            "resolved_at": i.resolved_at, "resolved_by": i.resolved_by,
            "overdue": i.resolution.is_none() && i.due_date.is_some_and(|d| d < today),
        })).collect::<Vec<_>>(),
        "masked": !see,
    }))
}

async fn find_case(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<onboarding_cases::Model> {
    let pid = super::parse_pid(pid)?;
    let c = onboarding_cases::Entity::find()
        .filter(onboarding_cases::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    // A case of a private plan the caller is outside (or a guest not granted) does not exist for them.
    if (super::guests::caller_is_guest(caller)
        && !super::guests::guest_covers_plan(&ctx.db, caller, c.plan_pid, "read").await)
        || super::plan_access::hidden_plans(ctx, &[c.plan_pid], caller)
            .await
            .contains(&c.plan_pid)
    {
        return Err(Error::NotFound);
    }
    // A case about a person does not exist for someone who may not see people.
    if Kind::parse(&c.kind).is_some_and(Kind::is_person) && !may_manage_people(caller) {
        return Err(Error::NotFound);
    }
    Ok(c)
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    #[serde(default)]
    status: Option<String>,
}

/// `GET /api/plans/{pid}/onboarding/cases` — the plan's checklists with their progress.
/// Cases about a person are listed (and counted) only for who may manage people.
#[debug_handler]
async fn list_cases(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut query = onboarding_cases::Entity::find()
        .filter(onboarding_cases::Column::PlanPid.eq(plan.pid))
        .order_by_desc(onboarding_cases::Column::Id);
    if let Some(s) = &q.status {
        if !["open", "completed", "cancelled"].contains(&s.as_str()) {
            return Err(unprocessable("status must be open, completed or cancelled"));
        }
        query = query.filter(onboarding_cases::Column::Status.eq(s.as_str()));
    }
    let see = may_manage_people(&caller);
    let mut out = Vec::new();
    let mut hidden = 0;
    for c in query.all(&ctx.db).await.map_err(db_err)? {
        if Kind::parse(&c.kind).is_some_and(Kind::is_person) && !see {
            hidden += 1;
            continue;
        }
        let items = items_of(&ctx, c.pid).await?;
        out.push(serde_json::json!({
            "pid": c.pid, "kind": c.kind, "title": c.title, "status": c.status,
            "subject_ref": c.subject_ref, "effective_date": c.effective_date,
            "progress": progress_of(&items),
        }));
    }
    format::json(serde_json::json!({ "cases": out, "hidden_people_cases": hidden, "masked": !see }))
}

/// `GET /api/onboarding/cases/{pid}` — one case with its items.
#[debug_handler]
async fn get_case(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let c = find_case(&ctx, &caller, &pid).await?;
    case_view(&ctx, &caller, &c).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemBody {
    /// A person URN, or "" to clear.
    #[serde(default)]
    owner_ref: Option<String>,
    #[serde(default)]
    due_date: Option<NaiveDate>,
    /// `done`, `skipped` (needs a note) or `open` (reopens).
    #[serde(default)]
    resolution: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

/// `PUT /api/onboarding/cases/{pid}/items/{item}` — assign, date, resolve or reopen an item.
#[debug_handler]
async fn put_item(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, item)): Path<(String, String)>,
    Json(b): Json<ItemBody>,
) -> Result<Response> {
    let case = find_case(&ctx, &caller, &pid).await?;
    if case.status != "open" {
        return Err(conflict("this checklist is closed"));
    }
    let item_pid = super::parse_pid(&item)?;
    let row = onboarding_items::Entity::find()
        .filter(onboarding_items::Column::Pid.eq(item_pid))
        .filter(onboarding_items::Column::CasePid.eq(case.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if b.owner_ref
        .as_deref()
        .is_some_and(|o| !o.is_empty() && !valid_person(o))
    {
        return Err(unprocessable(
            "owner_ref must be a person:/worker: URN (or empty to clear)",
        ));
    }
    if b.note.as_deref().is_some_and(|n| n.chars().count() > 500) {
        return Err(unprocessable("note is at most 500 characters"));
    }
    let was = serde_json::json!({ "owner_ref": row.owner_ref, "due_date": row.due_date,
        "resolution": row.resolution, "note": row.note });
    let mut a: onboarding_items::ActiveModel = row.clone().into();
    if let Some(o) = &b.owner_ref {
        a.owner_ref = ActiveValue::set((!o.is_empty()).then(|| o.clone()));
    }
    if let Some(d) = b.due_date {
        a.due_date = ActiveValue::set(Some(d));
    }
    if let Some(n) = &b.note {
        a.note = ActiveValue::set((!n.trim().is_empty()).then(|| n.trim().to_string()));
    }
    match b.resolution.as_deref() {
        None => {}
        Some("done") => {
            a.resolution = ActiveValue::set(Some("done".to_string()));
            a.resolved_at = ActiveValue::set(Some(Utc::now().into()));
            a.resolved_by = ActiveValue::set(caller.actor().map(str::to_string));
        }
        Some("skipped") => {
            let note = b.note.as_deref().or(row.note.as_deref()).unwrap_or("");
            if note.trim().is_empty() {
                return Err(unprocessable("skipping an item needs a note saying why"));
            }
            a.resolution = ActiveValue::set(Some("skipped".to_string()));
            a.resolved_at = ActiveValue::set(Some(Utc::now().into()));
            a.resolved_by = ActiveValue::set(caller.actor().map(str::to_string));
        }
        Some("open") => {
            a.resolution = ActiveValue::set(None);
            a.resolved_at = ActiveValue::set(None);
            a.resolved_by = ActiveValue::set(None);
        }
        Some(_) => return Err(unprocessable("resolution must be done, skipped or open")),
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let updated = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        updated.pid,
        "onboarding_item_updated",
        caller.actor(),
        Some(
            serde_json::json!({ "case_pid": case.pid, "title": updated.title, "from": was,
            "to": { "owner_ref": updated.owner_ref, "due_date": updated.due_date,
                    "resolution": updated.resolution, "note": updated.note } }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    case_view(&ctx, &caller, &case).await
}

/// `POST /api/onboarding/cases/{pid}/complete` — complete once no item is open.
#[debug_handler]
async fn complete_case(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let case = find_case(&ctx, &caller, &pid).await?;
    if case.status != "open" {
        return Err(conflict("this checklist is already closed"));
    }
    let open: Vec<String> = items_of(&ctx, case.pid)
        .await?
        .into_iter()
        .filter(|i| i.resolution.is_none())
        .map(|i| i.title)
        .collect();
    let problems = rules::completion_problems(&open);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    close(&ctx, &caller, case, "completed", None).await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CancelBody {
    reason: String,
}

/// `POST /api/onboarding/cases/{pid}/cancel` — abandon a checklist, with a reason.
#[debug_handler]
async fn cancel_case(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<CancelBody>,
) -> Result<Response> {
    let case = find_case(&ctx, &caller, &pid).await?;
    if case.status != "open" {
        return Err(conflict("this checklist is already closed"));
    }
    if b.reason.trim().is_empty() || b.reason.chars().count() > 500 {
        return Err(unprocessable("a reason is required (up to 500 characters)"));
    }
    close(
        &ctx,
        &caller,
        case,
        "cancelled",
        Some(b.reason.trim().to_string()),
    )
    .await
}

async fn close(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    case: onboarding_cases::Model,
    status: &str,
    reason: Option<String>,
) -> Result<Response> {
    let mut a: onboarding_cases::ActiveModel = case.into();
    a.status = ActiveValue::set(status.to_string());
    a.closed_at = ActiveValue::set(Some(Utc::now().into()));
    a.closed_reason = ActiveValue::set(reason.clone());
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        &format!("onboarding_case_{status}"),
        caller.actor(),
        Some(serde_json::json!({ "reason": reason })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    case_view(ctx, caller, &row).await
}

// ───────────────────────── access record ─────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AccessBody {
    person_ref: String,
    /// What access, in words ("repo write", "wiki editor").
    access: String,
    /// `granted` or `revoked`.
    action: String,
    #[serde(default)]
    recorded_on: Option<NaiveDate>,
    #[serde(default)]
    note: Option<String>,
    #[serde(default)]
    case_pid: Option<Uuid>,
}

/// `POST /api/plans/{pid}/access-records` — record access granted or revoked. Append-only.
#[debug_handler]
async fn record_access(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<AccessBody>,
) -> Result<Response> {
    need_people(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut problems = Vec::new();
    if !valid_person(&b.person_ref) {
        problems.push("person_ref must be a person:/worker: URN");
    }
    if b.access.trim().is_empty() || b.access.chars().count() > 120 {
        problems.push("access is required (up to 120 characters)");
    }
    if !["granted", "revoked"].contains(&b.action.as_str()) {
        problems.push("action must be granted or revoked");
    }
    if b.note.as_deref().is_some_and(|n| n.chars().count() > 500) {
        problems.push("note is at most 500 characters");
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if let Some(c) = b.case_pid {
        let ok = onboarding_cases::Entity::find()
            .filter(onboarding_cases::Column::Pid.eq(c))
            .filter(onboarding_cases::Column::PlanPid.eq(plan.pid))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .is_some();
        if !ok {
            return Err(unprocessable("case_pid is not a checklist of this plan"));
        }
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = access_records::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        case_pid: ActiveValue::set(b.case_pid),
        person_ref: ActiveValue::set(b.person_ref.clone()),
        access: ActiveValue::set(b.access.trim().to_string()),
        action: ActiveValue::set(b.action.clone()),
        recorded_on: ActiveValue::set(b.recorded_on.unwrap_or_else(crate::tz::deployment_today)),
        recorded_by: ActiveValue::set(caller.actor().map(str::to_string)),
        note: ActiveValue::set(b.note.clone().filter(|n| !n.trim().is_empty())),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "access_recorded",
        caller.actor(),
        Some(
            serde_json::json!({ "plan_pid": plan.pid, "person_ref": row.person_ref,
            "access": row.access, "action": row.action }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(serde_json::json!({ "pid": row.pid }))
}

async fn plan_records(ctx: &AppContext, plan: Uuid) -> Result<Vec<access_records::Model>> {
    access_records::Entity::find()
        .filter(access_records::Column::PlanPid.eq(plan))
        .order_by_asc(access_records::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

fn outstanding_of(rows: &[access_records::Model]) -> Vec<(String, String, NaiveDate)> {
    let recs: Vec<rules::Record> = rows
        .iter()
        .map(|r| rules::Record {
            id: r.id,
            person: r.person_ref.clone(),
            access: r.access.clone(),
            granted: r.action == "granted",
            on: r.recorded_on,
        })
        .collect();
    rules::outstanding(&recs)
}

/// `GET /api/plans/{pid}/access-records` — the record and what is still outstanding.
#[debug_handler]
async fn list_access(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    need_people(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = plan_records(&ctx, plan.pid).await?;
    format::json(serde_json::json!({
        "outstanding": outstanding_of(&rows).iter().map(|(p, a, on)| serde_json::json!({
            "person_ref": p, "access": a, "granted_on": on })).collect::<Vec<_>>(),
        "records": rows.iter().map(|r| serde_json::json!({
            "pid": r.pid, "person_ref": r.person_ref, "access": r.access, "action": r.action,
            "recorded_on": r.recorded_on, "recorded_by": r.recorded_by, "note": r.note,
            "case_pid": r.case_pid })).collect::<Vec<_>>(),
        "note": "This is a record, kept by hand. This service cannot grant or revoke access in an identity provider, so 'revoked' here means someone says they did it.",
    }))
}

// ───────────────────────── close-out readiness ─────────────────────────

/// `GET /api/plans/{pid}/closeout` — what stands between the plan and being closed.
/// Derived on read; no one is named.
#[debug_handler]
async fn closeout(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let def = super::workflow::in_force(&ctx, plan.pid, "task").await?;
    let open_tasks = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .iter()
        .filter(|t| !crate::workflow::is_done(&def, &t.status))
        .count();
    let open_risks = risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(plan.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .filter(risks::Column::Status.is_in(["open", "mitigating"]))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .len();
    let today = crate::tz::deployment_today();
    let active_allocations = allocations::Entity::find()
        .filter(allocations::Column::PlanPid.eq(plan.pid))
        .filter(allocations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .iter()
        .filter(|a| a.end_date.is_none_or(|e| e >= today))
        .count();
    let outstanding_access = outstanding_of(&plan_records(&ctx, plan.pid).await?).len();
    let open_cases = onboarding_cases::Entity::find()
        .filter(onboarding_cases::Column::PlanPid.eq(plan.pid))
        .filter(onboarding_cases::Column::Status.eq("open"))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .len();
    let counts = rules::Counts {
        open_tasks,
        open_risks,
        active_allocations,
        outstanding_access,
        open_cases,
    };
    let findings = rules::closeout(counts);
    format::json(serde_json::json!({
        "plan_pid": plan.pid, "ready": findings.is_empty(), "counts": counts, "findings": findings,
        "note": "Ready means nothing is left open: every task finished, every risk closed, every booking ended, every access grant revoked and every checklist closed. Derived on read.",
    }))
}

/// The onboarding routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/onboarding/templates", get(list_templates))
        .add("/onboarding/templates/{kind}", put(put_template))
        .add("/onboarding/templates/{kind}", delete(delete_template))
        .add("/plans/{pid}/onboarding/cases", post(create_case))
        .add("/plans/{pid}/onboarding/cases", get(list_cases))
        .add("/onboarding/cases/{pid}", get(get_case))
        .add("/onboarding/cases/{pid}/items/{item}", put(put_item))
        .add("/onboarding/cases/{pid}/complete", post(complete_case))
        .add("/onboarding/cases/{pid}/cancel", post(cancel_case))
        .add("/plans/{pid}/access-records", post(record_access))
        .add("/plans/{pid}/access-records", get(list_access))
        .add("/plans/{pid}/closeout", get(closeout))
}
