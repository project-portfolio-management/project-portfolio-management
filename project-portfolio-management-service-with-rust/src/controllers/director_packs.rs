//! The monthly director pack (DP-1). Rules in [`crate::director_pack`].
//!
//! A pack is **computed once** into stored text when it is drafted, then a person **issues** it; from then
//! on the database refuses any change to it. Corrections are reissues (the next version, with a change
//! note); every version stays readable. The pack is computed over plans that are neither private nor
//! carrying a handling mark, so an audience that needs no access to the live estate can never be shown
//! something the live estate would hide from them. Today it holds the sections that are built (issues and
//! change requests) and says plainly which are not.

use axum::http::{StatusCode, header};
use chrono::{DateTime, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{MaybeAuthUser, require_auth};
use crate::director_pack::{self as rules, Power};
use crate::director_summary::{self as summary, ChangeLite, IssueLite};
use crate::models::_entities::{
    change_requests, director_packs as packs, issues, plan_access_settings, plan_handling_marks,
    plans,
};
use crate::models::audit_logs::Model as AuditModel;

fn refuse(status: StatusCode, code: &str, message: &str) -> Error {
    Error::CustomError(status, ErrorDetail::new(code, message))
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

/// What the caller may do. With login off everyone is trusted, as for every other write.
pub(crate) fn caller_power(caller: &MaybeAuthUser) -> Option<Power> {
    if !require_auth() {
        return Some(Power::Issuer);
    }
    let claims = caller.claims()?;
    let attr = |k: &str| claims.attrs.get(k).cloned().unwrap_or_default();
    rules::power(
        attr("access").iter().any(|a| a == "admin"),
        &attr("directorpack"),
    )
}

pub(crate) fn caller_audiences(caller: &MaybeAuthUser) -> Vec<String> {
    let mut out = caller
        .claims()
        .and_then(|c| c.attrs.get("pack_audience").cloned())
        .unwrap_or_default();
    // A caller of an organization is an audience of the packs addressed to it (EX-3).
    out.extend(
        super::organizations::caller_organizations(caller)
            .into_iter()
            .map(|o| format!("org:{o}")),
    );
    out
}

/// The pack as a reader sees it, never with the body.
fn meta(p: &packs::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": p.pid, "period": p.period, "audience": p.audience, "version": p.version,
        "status": p.status, "cut_off": p.cut_off, "change_note": p.change_note,
        "supersedes_pid": p.supersedes_pid, "issued_at": p.issued_at, "issued_by": p.issued_by,
        "origin": p.origin, "reviewed": p.reviewed_at.is_some(),
        "covers_whole_period": rules::covers_whole_period(&p.period, DateTime::<Utc>::from(p.cut_off).date_naive()),
        "stale": rules::is_stale(&p.status, &p.period, DateTime::<Utc>::from(p.cut_off).date_naive()),
    })
}

/// Everything the issued and the draft packs share: may the caller read this one?
fn may_read(caller: &MaybeAuthUser, p: &packs::Model) -> bool {
    if !require_auth() {
        return true;
    }
    let power = caller_power(caller);
    if power.is_some() {
        return true;
    }
    p.status == "issued" && rules::may_read(None, &caller_audiences(caller), &p.audience)
}

/// The inputs for a pack: only plans that are live, not private and not marked. Returns the rows and how
/// many plans were left out for being private or marked.
async fn inputs(
    ctx: &AppContext,
    cut_off: DateTime<Utc>,
) -> Result<(
    Vec<IssueLite>,
    Vec<ChangeLite>,
    std::collections::HashSet<Uuid>,
    usize,
)> {
    let day = cut_off.date_naive();
    let all_plans: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let mut left_out: std::collections::HashSet<Uuid> = plan_access_settings::Entity::find()
        .filter(plan_access_settings::Column::Private.eq(true))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .map(|r| r.plan_pid)
        .collect();
    left_out.extend(
        plan_handling_marks::Entity::find()
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .into_iter()
            .map(|r| r.plan_pid),
    );
    let included: std::collections::HashSet<Uuid> = all_plans
        .iter()
        .copied()
        .filter(|p| !left_out.contains(p))
        .collect();
    let excluded = all_plans.len() - included.len();
    let issue_rows: Vec<IssueLite> = issues::Entity::find()
        .filter(issues::Column::DeletedAt.is_null())
        .order_by_asc(issues::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|i| included.contains(&i.plan_pid) && i.raised_on <= day)
        .map(|i| IssueLite {
            plan_pid: i.plan_pid.to_string(),
            title: i.title,
            severity: i.severity,
            status: i.status,
            raised_on: i.raised_on,
            resolved_on: i.resolved_on,
            due_date: i.due_date,
        })
        .collect();
    let change_rows: Vec<ChangeLite> = change_requests::Entity::find()
        .filter(change_requests::Column::DeletedAt.is_null())
        .order_by_asc(change_requests::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|c| included.contains(&c.plan_pid) && c.requested_on <= day)
        .map(|c| ChangeLite {
            plan_pid: c.plan_pid.to_string(),
            title: c.title,
            status: c.status,
            requested_on: c.requested_on,
            decided_on: c.decided_on,
        })
        .collect();
    Ok((issue_rows, change_rows, included, excluded))
}

/// One count out of an earlier pack's body.
fn count_in(body: &str, path: &[&str]) -> Option<u64> {
    let mut v: serde_json::Value = serde_json::from_str(body).ok()?;
    for k in path {
        v = v.get(k)?.clone();
    }
    v.as_u64()
}

/// Compute the body text for a pack. Called once, at drafting.
#[allow(clippy::too_many_arguments)]
async fn compute_body(
    ctx: &AppContext,
    pid: Uuid,
    period: &str,
    audience: &str,
    version: i32,
    supersedes: Option<Uuid>,
    cut_off: DateTime<Utc>,
    change_note: Option<&str>,
) -> Result<String> {
    let (issue_rows, change_rows, included_set, excluded) = inputs(ctx, cut_off).await?;
    let included = included_set.len();
    let today = cut_off.date_naive();
    let i = summary::issues(&issue_rows, today, 31);
    let c = summary::changes(&change_rows, today, 31);
    // The previous issued pack for this audience: the latest of an earlier period.
    let previous = packs::Entity::find()
        .filter(packs::Column::Audience.eq(audience))
        .filter(packs::Column::Status.eq("issued"))
        .filter(packs::Column::Period.lt(period))
        .order_by_desc(packs::Column::Period)
        .order_by_desc(packs::Column::Version)
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let prev_body = previous.as_ref().map(|p| p.body.as_str());
    let before = |path: &[&str]| prev_body.and_then(|b| count_in(b, path));
    let what_changed = vec![
        rules::changed("Open issues", before(&["issues", "open"]), i.open as u64),
        rules::changed(
            "Change requests waiting for a decision",
            before(&["change_requests", "awaiting_decision"]),
            c.awaiting_decision as u64,
        ),
    ];
    let portfolio_map = super::confidence::build_map(ctx, &included_set, period, prev_body).await?;
    let exceptions =
        super::confidence::exceptions_section(ctx, &included_set, period, cut_off).await?;
    let gate_decisions =
        super::programme_gates::gates_section(ctx, &included_set, period, cut_off).await?;
    let money = super::funding::pack_section(ctx, &included_set, cut_off).await?;
    let fever = super::fever::fever_section(ctx, &included_set, prev_body).await?;
    let evm_section = super::earned_value::portfolio_section(
        ctx,
        &included_set,
        today,
        super::earned_value::pack_ac_source(),
    )
    .await?;
    let benefits_view = super::benefit_validation::benefits_section(ctx, &included_set).await?;
    let heatmap = super::capacity_heatmap::pack_section(ctx, &included_set, cut_off).await?;
    let signoffs = super::milestone_signoffs::pack_section(ctx, &included_set).await?;
    let asks = super::governance::asks_section(ctx, period).await?;
    let register = super::register::pack_section(ctx, &included_set).await?;
    let key_persons = super::key_persons::pack_section(ctx).await?;
    let triage = super::thresholds::pack_section(ctx, &included_set, period).await?;
    let cutovers = super::cutover::pack_section(ctx, &included_set, period).await?;
    let mut body = serde_json::json!({
        "cover": {
            "pack_pid": pid, "version": version, "supersedes_pid": supersedes,
            "period": period, "audience": audience, "cut_off": cut_off,
            "change_note": change_note,
            "previous_issued_pack": previous.as_ref().map(|p| serde_json::json!({
                "pid": p.pid, "period": p.period, "version": p.version })),
        },
        "what_changed": what_changed,
        "basis": "Records dated on or before the cut-off, in the state they have when this pack is drafted: issues and change requests keep no status history, so a record closed after the cut-off but before drafting shows as closed.",
        "data_quality": {
            "plans_included": included,
            "plans_left_out_private_or_marked": excluded,
            "sections_included": ["portfolio_map", "exceptions", "gate_decisions", "funding", "buffer_fever", "earned_value", "benefits", "capacity_heatmap", "milestone_signoffs", "external_asks", "key_persons", "triage_and_thresholds", "cutover_decisions", "watermelon", "issues", "change_requests"],
            "sections_not_yet_built": [],
            "register_completeness": register,
            "note": "A section that is not built is absent from this pack, not zero or green.",
        },
        "portfolio_map": portfolio_map,
        "exceptions": exceptions,
        "gate_decisions": gate_decisions,
        "funding": money,
        "buffer_fever": fever,
        "earned_value": evm_section,
        "benefits": benefits_view,
        "capacity_heatmap": heatmap,
        "milestone_signoffs": signoffs,
        "external_asks": asks,
        "key_persons": key_persons,
        "triage_and_thresholds": triage,
        "cutover_decisions": cutovers,
        "issues": i,
        "change_requests": c,
    });
    // The watermelon measures read the stored bodies only: this one and the previous issued pack's (EX-9).
    let previous_value = prev_body.and_then(|b| serde_json::from_str::<serde_json::Value>(b).ok());
    body["watermelon"] = crate::watermelon::measures(&body, previous_value.as_ref());
    serde_json::to_string(&body).map_err(|e| Error::string(&e.to_string()))
}

#[derive(Debug, Deserialize)]
struct DraftBody {
    period: String,
    audience: String,
    /// Defaults to now; not in the future and not more than a year old.
    #[serde(default)]
    cut_off: Option<DateTime<Utc>>,
}

pub(crate) fn need(caller: &MaybeAuthUser, at_least: Power) -> Result<String> {
    match caller_power(caller) {
        Some(p) if p >= at_least => Ok(caller.actor().unwrap_or("system").to_string()),
        _ => Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            if at_least == Power::Issuer {
                "issuing a director pack needs access=admin or directorpack=issuer"
            } else {
                "drafting a director pack needs access=admin or directorpack=editor or issuer"
            },
        )),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn insert_draft(
    ctx: &AppContext,
    actor: &str,
    period: &str,
    audience: &str,
    version: i32,
    supersedes: Option<Uuid>,
    cut_off: DateTime<Utc>,
    note: Option<&str>,
    event: &str,
) -> Result<packs::Model> {
    let pid = Uuid::new_v4();
    let body = compute_body(
        ctx, pid, period, audience, version, supersedes, cut_off, note,
    )
    .await?;
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let row = packs::ActiveModel {
        pid: ActiveValue::set(pid),
        period: ActiveValue::set(period.to_string()),
        audience: ActiveValue::set(audience.to_string()),
        version: ActiveValue::set(version),
        status: ActiveValue::set("draft".to_string()),
        cut_off: ActiveValue::set(cut_off.into()),
        body: ActiveValue::set(body),
        change_note: ActiveValue::set(note.map(str::to_string)),
        supersedes_pid: ActiveValue::set(supersedes),
        drafted_by: ActiveValue::set(Some(actor.to_string())),
        issued_by: ActiveValue::set(None),
        issued_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&tx)
    .await
    .map_err(|e| {
        // The unique indexes are the arbiters of a race: one draft, one version.
        if e.to_string().contains("duplicate key") {
            refuse(
                StatusCode::CONFLICT,
                "conflict",
                "a draft or that version of this pack already exists",
            )
        } else {
            db_err(&e)
        }
    })?;
    AuditModel::record(
        &tx,
        row.pid,
        event,
        Some(actor),
        Some(serde_json::json!({
            "period": period, "audience": audience, "version": version, "change_note": note,
        })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    Ok(row)
}

/// `POST /api/director-packs` — draft the first version of a period's pack for an audience.
#[debug_handler]
async fn draft(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<DraftBody>,
) -> Result<Response> {
    let actor = need(&caller, Power::Editor)?;
    if !rules::valid_period(&b.period) {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "period must be YYYY-MM",
        ));
    }
    if !super::organizations::audience_valid(&ctx, &b.audience).await? {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            &format!("audience must be one of {:?}, or org:<pid> of an active organization", rules::AUDIENCES),
        ));
    }
    let now = Utc::now();
    let cut_off = b.cut_off.unwrap_or(now);
    if cut_off > now || cut_off < now - chrono::Duration::days(400) {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "cut_off must not be in the future or more than 400 days old",
        ));
    }
    let existing = packs::Entity::find()
        .filter(packs::Column::Period.eq(b.period.as_str()))
        .filter(packs::Column::Audience.eq(b.audience.as_str()))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    if let Some(e) = existing {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            &format!(
                "this period already has a pack for that audience (version {}, {}); {}",
                e.version,
                e.status,
                if e.status == "issued" {
                    "reissue it with a change note"
                } else {
                    "issue or discard that draft"
                }
            ),
        ));
    }
    let row = insert_draft(
        &ctx,
        &actor,
        &b.period,
        &b.audience,
        1,
        None,
        cut_off,
        None,
        "director_pack_drafted",
    )
    .await?;
    let mut resp = format::json(meta(&row))?;
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

pub(crate) async fn find(ctx: &AppContext, pid: &str) -> Result<packs::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    packs::Entity::find()
        .filter(packs::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)
}

/// `POST /api/director-packs/{pid}/issue` — freeze a draft. Only a person with the right can; a second
/// issue is a conflict.
#[debug_handler]
async fn issue(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let actor = need(&caller, Power::Issuer)?;
    let row = find(&ctx, &pid).await?;
    // A pack the schedule drafted is looked over by the portfolio office before anyone issues it.
    if row.status == "draft" && row.origin == "scheduled" && row.reviewed_at.is_none() {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this draft was made by the monthly schedule; the portfolio office must review it before it is issued",
        ));
    }
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let done = packs::Entity::update_many()
        .col_expr(packs::Column::Status, Expr::value("issued"))
        .col_expr(packs::Column::IssuedBy, Expr::value(actor.clone()))
        .col_expr(packs::Column::IssuedAt, Expr::value(Utc::now()))
        .filter(packs::Column::Pid.eq(row.pid))
        .filter(packs::Column::Status.eq("draft"))
        .exec(&tx)
        .await
        .map_err(|e| db_err(&e))?;
    if done.rows_affected == 0 {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "this pack is already issued",
        ));
    }
    AuditModel::record(
        &tx,
        row.pid,
        "director_pack_issued",
        Some(&actor),
        Some(serde_json::json!({ "period": row.period, "audience": row.audience, "version": row.version })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    let now = find(&ctx, &pid).await?;
    // Tell the audience's subscribers, with a link and never the body. A notice that cannot be made does
    // not undo the issue: the pack is issued either way.
    if let Err(e) = super::pack_rhythm::notify_issued(&ctx, &now).await {
        tracing::warn!(
            "director pack {} issued but its notices failed: {e}",
            now.pid
        );
    }
    format::json(meta(&now))
}

#[derive(Debug, Deserialize)]
struct ReissueBody {
    change_note: String,
}

/// `POST /api/director-packs/{pid}/reissue` — draft the next version of an issued pack, with a note.
/// The earlier version stays readable and unchanged.
#[debug_handler]
async fn reissue(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ReissueBody>,
) -> Result<Response> {
    let actor = need(&caller, Power::Editor)?;
    let row = find(&ctx, &pid).await?;
    if row.status != "issued" {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            "only an issued pack is reissued; a draft is simply issued",
        ));
    }
    if let Some(problem) = rules::note_problem(&b.change_note) {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            &problem,
        ));
    }
    let latest = packs::Entity::find()
        .filter(packs::Column::Period.eq(row.period.as_str()))
        .filter(packs::Column::Audience.eq(row.audience.as_str()))
        .order_by_desc(packs::Column::Version)
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .ok_or(Error::NotFound)?;
    if latest.pid != row.pid {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            &format!(
                "version {} already exists; reissue from the latest",
                latest.version
            ),
        ));
    }
    let new = insert_draft(
        &ctx,
        &actor,
        &row.period,
        &row.audience,
        row.version + 1,
        Some(row.pid),
        row.cut_off.into(),
        Some(b.change_note.trim()),
        "director_pack_reissue_drafted",
    )
    .await?;
    let mut resp = format::json(meta(&new))?;
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

/// `GET /api/director-packs` — the packs you may read, without bodies; drafts only for those who draft.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let rows = packs::Entity::find()
        .order_by_desc(packs::Column::Period)
        .order_by_asc(packs::Column::Audience)
        .order_by_desc(packs::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let shown: Vec<serde_json::Value> = rows
        .iter()
        .filter(|p| may_read(&caller, p))
        .map(meta)
        .collect();
    format::json(serde_json::json!({ "packs": shown }))
}

/// `GET /api/director-packs/{pid}` — the pack. For an issued pack `body` is the exact text that was
/// stored, so it is byte-identical on every read.
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find(&ctx, &pid).await?;
    if !may_read(&caller, &row) {
        // The same answer as a pack that does not exist.
        return Err(Error::NotFound);
    }
    let meta_text =
        serde_json::to_string(&meta(&row)).map_err(|e| Error::string(&e.to_string()))?;
    // The stored text is spliced in, not parsed and re-serialised, so its bytes never change.
    let text = format!("{{\"pack\":{meta_text},\"body\":{}}}", row.body);
    Ok(([(header::CONTENT_TYPE, "application/json")], text).into_response())
}

/// `GET /api/director-packs/{pid}/pdf` — the same pack as a PDF. It is made from the stored JSON alone, so
/// it shows no figure the JSON does not, and it is as deterministic as the JSON is frozen: two reads of an
/// issued pack are byte-identical. The words are English (the writer is Latin-1 only); a reader of another
/// language prints the translated page in the interface.
#[debug_handler]
async fn get_pdf(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find(&ctx, &pid).await?;
    if !may_read(&caller, &row) {
        return Err(Error::NotFound);
    }
    let body: serde_json::Value = serde_json::from_str(&row.body)
        .map_err(|e| Error::string(&format!("the stored pack cannot be read: {e}")))?;
    let lines = crate::director_pack_text::lines(&meta(&row), &body);
    let title = format!(
        "Director pack {} {} v{}",
        row.period, row.audience, row.version
    );
    let pdf = crate::pdf::render(&title, &lines);
    let name = format!(
        "director-pack-{}-{}-v{}.pdf",
        row.period, row.audience, row.version
    );
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{name}\""),
            ),
        ],
        pdf,
    )
        .into_response())
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/director-packs")
        .add("", post(draft))
        .add("", get(list))
        .add("/{pid}", get(get_one))
        .add("/{pid}/pdf", get(get_pdf))
        .add("/{pid}/issue", post(issue))
        .add("/{pid}/reissue", post(reissue))
}
