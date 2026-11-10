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

use super::unprocessable;
use super::{conflict, forbidden};
use crate::prelude::*;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::portfolio_resources::may_manage_people;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    access_records, onboarding_cases, onboarding_items, onboarding_templates,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::onboarding::{self as rules, Kind, Seed};

/// Gate for person-related onboarding data: passes only callers who may manage people, otherwise a 403 with an explanatory message.
fn need_people(caller: &MaybeAuthUser) -> Result<()> {
    if may_manage_people(caller) {
        Ok(())
    } else {
        Err(forbidden(
            "this is about people: it needs the permission to manage people",
        ))
    }
}

/// True when `r` is a well-formed `person:` or `worker:` reference (people are references, never free text).
fn valid_person(r: &str) -> bool {
    super::governance::valid_ref(r, &["person", "worker"])
}

// ───────────────────────── templates ─────────────────────────

/// Resolves the checklist for `kind`: the custom template if one is stored and parses, otherwise the built-in. The bool is true when the custom one was used.
async fn items_for(ctx: &AppContext, kind: Kind) -> Result<(Vec<Seed>, bool)> {
    let custom = template_of(ctx, kind)
        .await?
        .and_then(|t| serde_json::from_value::<Vec<Seed>>(t.items).ok());
    Ok(match custom {
        Some(items) => (items, true),
        None => (rules::builtin(kind), false),
    })
}

/// The stored custom template for `kind`, if any.
async fn template_of(ctx: &AppContext, kind: Kind) -> Result<Option<onboarding_templates::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM onboarding_templates WHERE kind = $1 LIMIT 1")
            .bind(kind.token())
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Every checklist kind, in the order the templates endpoint lists them.
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

/// `PUT /api/onboarding/templates/{kind}` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TemplateBody {
    /// Replacement checklist items, in display order.
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
    let existing = template_of(&ctx, kind).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let pid = match existing {
        Some(t) => {
            sqlx::query("UPDATE onboarding_templates SET items = $1 WHERE id = $2 RETURNING id")
                .bind(items)
                .bind(t.id)
                .fetch_one(tx.conn())
                .await?;
            t.pid
        }
        None => sqlx::query_scalar(
            "INSERT INTO onboarding_templates (pid, kind, items) VALUES ($1, $2, $3) RETURNING pid",
        )
        .bind(Uuid::new_v4())
        .bind(kind.token())
        .bind(items)
        .fetch_one(tx.conn())
        .await?,
    };
    AuditModel::record_on(
        tx.conn(),
        pid,
        "onboarding_template_set",
        caller.actor(),
        Some(serde_json::json!({ "kind": kind.token(), "items": b.items.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    sqlx::query("DELETE FROM onboarding_templates WHERE kind = $1")
        .bind(kind.token())
        .execute(crate::db::pool(&ctx))
        .await?;
    format::empty_json()
}

// ───────────────────────── cases ─────────────────────────

/// `POST /api/plans/{pid}/onboarding/cases` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CaseBody {
    /// Checklist kind token: `person_join`, `person_leave`, `plan_intake` or `plan_closeout`.
    kind: String,
    /// The person, for a join or leave.
    #[serde(default)]
    subject_ref: Option<String>,
    /// Date the join, leave, intake or closeout takes effect; must fall within 1970..=2100.
    effective_date: NaiveDate,
    /// Optional case title (non-blank, at most 200 characters); `None` lets the server derive one.
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
    let dup = sqlx::query(
        "SELECT 1 FROM onboarding_cases
         WHERE plan_pid = $1 AND kind = $2 AND status = 'open'
           AND ($3::text IS NULL OR subject_ref = $3)
         LIMIT 1",
    )
    .bind(plan.pid)
    .bind(kind.token())
    .bind(&b.subject_ref)
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    if dup.is_some() {
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
    let mut tx = crate::db::begin(&ctx).await?;
    let case: onboarding_cases::Model = sqlx::query_as(
        "INSERT INTO onboarding_cases
             (pid, kind, plan_pid, subject_ref, title, effective_date, status, created_by, closed_at,
              closed_reason)
         VALUES ($1, $2, $3, $4, $5, $6, 'open', $7, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(kind.token())
    .bind(plan.pid)
    .bind(&b.subject_ref)
    .bind(title)
    .bind(b.effective_date)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| {
        if e.to_string().contains("onboarding_cases_one_open") {
            conflict(
                "an open checklist of this kind already exists here; finish or cancel it first",
            )
        } else {
            Error::from(e)
        }
    })?;
    for (i, s) in seeds.iter().enumerate() {
        sqlx::query(
            "INSERT INTO onboarding_items
                 (pid, case_pid, position, title, owner_ref, due_date, resolution, note, resolved_at,
                  resolved_by)
             VALUES ($1, $2, $3, $4, NULL, $5, NULL, NULL, NULL, NULL)",
        )
        .bind(Uuid::new_v4())
        .bind(case.pid)
        .bind(i32::try_from(i).unwrap_or(0))
        .bind(&s.title)
        .bind(rules::due_date(b.effective_date, s.offset_days))
        .execute(tx.conn())
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
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
    tx.commit().await?;
    case_view(&ctx, &caller, &case).await
}

/// Loads a case's checklist items in their display `position` order.
async fn items_of(ctx: &AppContext, case: Uuid) -> Result<Vec<onboarding_items::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM onboarding_items WHERE case_pid = $1 ORDER BY position")
            .bind(case)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// Summarizes item resolutions into done/open/overdue progress, judging lateness against the deployment's local today.
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

/// Renders a case with its items and progress. The subject reference is hidden (and `masked` set) unless the caller may manage people.
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

/// Loads a case by pid for a caller, answering not-found (never forbidden) for cases the caller must not know exist: private plans outside their scope, guests without read grant, and person cases for callers who cannot manage people.
async fn find_case(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<onboarding_cases::Model> {
    let pid = super::parse_pid(pid)?;
    let c: onboarding_cases::Model =
        sqlx::query_as("SELECT * FROM onboarding_cases WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    // A case of a private plan the caller is outside (or a guest not granted) does not exist for them.
    if (super::guests::caller_is_guest(caller)
        && !super::guests::guest_covers_plan(ctx, caller, c.plan_pid, "read").await)
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

/// Query parameters for `GET /api/plans/{pid}/onboarding/cases`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// Optional status filter (for example `open`, `completed`, `cancelled`); `None` lists all.
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
    if let Some(s) = &q.status
        && !["open", "completed", "cancelled"].contains(&s.as_str())
    {
        return Err(unprocessable("status must be open, completed or cancelled"));
    }
    let cases: Vec<onboarding_cases::Model> = sqlx::query_as(
        "SELECT * FROM onboarding_cases
         WHERE plan_pid = $1 AND ($2::text IS NULL OR status = $2)
         ORDER BY id DESC",
    )
    .bind(plan.pid)
    .bind(&q.status)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let see = may_manage_people(&caller);
    let mut out = Vec::new();
    let mut hidden = 0;
    for c in cases {
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

/// `PUT /api/onboarding/cases/{pid}/items/{item}` body; every field is optional and only those present change the item.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ItemBody {
    /// A person URN, or "" to clear.
    #[serde(default)]
    owner_ref: Option<String>,
    /// New due date; `None` leaves it unchanged.
    #[serde(default)]
    due_date: Option<NaiveDate>,
    /// `done`, `skipped` (needs a note) or `open` (reopens).
    #[serde(default)]
    resolution: Option<String>,
    /// Free-text note on the item; required when resolving as `skipped`.
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
    let row: onboarding_items::Model =
        sqlx::query_as("SELECT * FROM onboarding_items WHERE pid = $1 AND case_pid = $2 LIMIT 1")
            .bind(item_pid)
            .bind(case.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
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
    // Only the columns the body names are written, as before.
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE onboarding_items SET ");
    let mut set = q.separated(", ");
    let mut changed = false;
    if let Some(o) = &b.owner_ref {
        set.push("owner_ref = ")
            .push_bind_unseparated((!o.is_empty()).then(|| o.clone()));
        changed = true;
    }
    if let Some(d) = b.due_date {
        set.push("due_date = ").push_bind_unseparated(Some(d));
        changed = true;
    }
    if let Some(n) = &b.note {
        set.push("note = ")
            .push_bind_unseparated((!n.trim().is_empty()).then(|| n.trim().to_string()));
        changed = true;
    }
    let resolution: Option<(Option<&str>, bool)> = match b.resolution.as_deref() {
        None => None,
        Some("done") => Some((Some("done"), true)),
        Some("skipped") => {
            let note = b.note.as_deref().or(row.note.as_deref()).unwrap_or("");
            if note.trim().is_empty() {
                return Err(unprocessable("skipping an item needs a note saying why"));
            }
            Some((Some("skipped"), true))
        }
        Some("open") => Some((None, false)),
        Some(_) => return Err(unprocessable("resolution must be done, skipped or open")),
    };
    if let Some((token, resolved)) = resolution {
        set.push("resolution = ")
            .push_bind_unseparated(token.map(str::to_string));
        set.push("resolved_at = ").push_bind_unseparated(
            resolved.then(|| chrono::DateTime::<chrono::FixedOffset>::from(Utc::now())),
        );
        set.push("resolved_by = ")
            .push_bind_unseparated(if resolved {
                caller.actor().map(str::to_string)
            } else {
                None
            });
        changed = true;
    }
    q.push(" WHERE id = ")
        .push_bind(row.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(&ctx).await?;
    let updated: onboarding_items::Model = if changed {
        q.build_query_as().fetch_one(tx.conn()).await?
    } else {
        // Nothing to write: the row is read back, as an update with no columns did.
        sqlx::query_as("SELECT * FROM onboarding_items WHERE id = $1 LIMIT 1")
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?
    };
    AuditModel::record_on(
        tx.conn(),
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
    tx.commit().await?;
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

/// `POST /api/onboarding/cases/{pid}/cancel` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CancelBody {
    /// Why the checklist is being abandoned; stored as the closed reason.
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

/// Closes a case with `status` (`completed` or `cancelled`) and an optional reason, stamping the close time and auditing `onboarding_case_<status>` in the same transaction.
async fn close(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    case: onboarding_cases::Model,
    status: &str,
    reason: Option<String>,
) -> Result<Response> {
    let mut tx = crate::db::begin(ctx).await?;
    let row: onboarding_cases::Model = sqlx::query_as(
        "UPDATE onboarding_cases SET status = $1, closed_at = $2, closed_reason = $3
         WHERE id = $4 RETURNING *",
    )
    .bind(status)
    .bind(Utc::now().fixed_offset())
    .bind(&reason)
    .bind(case.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        &format!("onboarding_case_{status}"),
        caller.actor(),
        Some(serde_json::json!({ "reason": reason })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    case_view(ctx, caller, &row).await
}

// ───────────────────────── access record ─────────────────────────

/// `POST /api/plans/{pid}/access-records` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AccessBody {
    /// Person URN whose access changed.
    person_ref: String,
    /// What access, in words ("repo write", "wiki editor").
    access: String,
    /// `granted` or `revoked`.
    action: String,
    /// Date the change took effect; `None` lets the server default it.
    #[serde(default)]
    recorded_on: Option<NaiveDate>,
    /// Optional free-text note.
    #[serde(default)]
    note: Option<String>,
    /// Optional onboarding case this record belongs to.
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
        let ok =
            sqlx::query("SELECT 1 FROM onboarding_cases WHERE pid = $1 AND plan_pid = $2 LIMIT 1")
                .bind(c)
                .bind(plan.pid)
                .fetch_optional(crate::db::pool(&ctx))
                .await?
                .is_some();
        if !ok {
            return Err(unprocessable("case_pid is not a checklist of this plan"));
        }
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: access_records::Model = sqlx::query_as(
        "INSERT INTO access_records
             (pid, plan_pid, case_pid, person_ref, access, action, recorded_on, recorded_by, note)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(b.case_pid)
    .bind(&b.person_ref)
    .bind(b.access.trim())
    .bind(&b.action)
    .bind(b.recorded_on.unwrap_or_else(crate::tz::deployment_today))
    .bind(caller.actor())
    .bind(b.note.clone().filter(|n| !n.trim().is_empty()))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
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
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid }))
}

/// Loads a plan's access records oldest first (by id), the append-only order the outstanding computation relies on.
async fn plan_records(ctx: &AppContext, plan: Uuid) -> Result<Vec<access_records::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM access_records WHERE plan_pid = $1 ORDER BY id")
            .bind(plan)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// Reduces the access history to grants that were never revoked, as (person, access, granted-on) triples.
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
    let open_tasks = sqlx::query_scalar::<_, String>(
        "SELECT status FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .iter()
    .filter(|status| !crate::workflow::is_done(&def, status))
    .count();
    let open_risks = count(
        &ctx,
        "SELECT count(*) FROM risks
         WHERE plan_pid = $1 AND deleted_at IS NULL AND status IN ('open', 'mitigating')",
        plan.pid,
    )
    .await?;
    let today = crate::tz::deployment_today();
    let active_allocations = sqlx::query_scalar::<_, Option<NaiveDate>>(
        "SELECT end_date FROM allocations WHERE plan_pid = $1 AND deleted_at IS NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .iter()
    .filter(|end| end.is_none_or(|e| e >= today))
    .count();
    let outstanding_access = outstanding_of(&plan_records(&ctx, plan.pid).await?).len();
    let open_cases = count(
        &ctx,
        "SELECT count(*) FROM onboarding_cases WHERE plan_pid = $1 AND status = 'open'",
        plan.pid,
    )
    .await?;
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

/// A `count(*)` over one plan's rows.
async fn count(ctx: &AppContext, sql: &'static str, plan: Uuid) -> Result<usize> {
    let n: i64 = sqlx::query_scalar(sql)
        .bind(plan)
        .fetch_one(crate::db::pool(ctx))
        .await?;
    Ok(usize::try_from(n).unwrap_or(0))
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
