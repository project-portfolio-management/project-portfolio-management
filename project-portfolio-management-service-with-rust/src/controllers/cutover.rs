//! Go / no-go for safety-critical cutovers (EX-6). Rules in [`crate::cutover`].
//!
//! A decision starts as a **draft** with its stop criteria, is **published**, and only then **opened**: from
//! the moment it is published its criteria cannot be added to, edited or removed (the database refuses), so a
//! criterion cannot be moved to suit the answer; a changed criterion is a **new version**. While open, each
//! criterion is marked met or not (with evidence) and the configured roles co-sign. Then it is decided:
//! **go** needs every criterion met and every co-signature; **no-go** and **delay for safety** are decisions
//! in their own right and the pack shows them as such, not as slippage.
//!
//! Marking and deciding are by anyone who may write to the plan; a co-signature needs the signer's token to
//! carry the role in its `cutover_role` attribute.

#![expect(clippy::many_single_char_names)]

use super::conflict;
use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cutover as rules;
use crate::models::_entities::{
    cutover_cosignatures as cosigs, cutover_criteria as criteria, cutover_decisions as decisions,
    plans,
};
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/cutover-decisions", post(create))
        .add("/plans/{pid}/cutover-decisions", get(list))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/criteria",
            put(replace_criteria),
        )
        .add(
            "/plans/{pid}/cutover-decisions/{did}/publish",
            post(publish),
        )
        .add("/plans/{pid}/cutover-decisions/{did}/open", post(open))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/criteria/{n}/mark",
            put(mark),
        )
        .add("/plans/{pid}/cutover-decisions/{did}/cosign", post(cosign))
        .add("/plans/{pid}/cutover-decisions/{did}/decide", post(decide))
        .add(
            "/plans/{pid}/cutover-decisions/{did}/new-version",
            post(new_version),
        )
}

/// Loads the plan and one of its cutover decisions by pid; a decision of another plan is `NotFound`.
async fn load(ctx: &AppContext, pid: &str, did: &str) -> Result<(plans::Model, decisions::Model)> {
    let plan = super::governance::find_item(ctx, pid).await?;
    let did = super::parse_pid(did)?;
    let d: decisions::Model =
        sqlx::query_as("SELECT * FROM cutover_decisions WHERE pid = $1 AND plan_pid = $2 LIMIT 1")
            .bind(did)
            .bind(plan.pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    Ok((plan, d))
}

/// Returns the decision's stop criteria in position order and its co-signatures.
async fn parts(
    ctx: &AppContext,
    d: &decisions::Model,
) -> Result<(Vec<criteria::Model>, Vec<cosigs::Model>)> {
    let c: Vec<criteria::Model> =
        sqlx::query_as("SELECT * FROM cutover_criteria WHERE decision_pid = $1 ORDER BY position")
            .bind(d.pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let s: Vec<cosigs::Model> =
        sqlx::query_as("SELECT * FROM cutover_cosignatures WHERE decision_pid = $1 ORDER BY id")
            .bind(d.pid)
            .fetch_all(crate::db::pool(ctx))
            .await?;
    Ok((c, s))
}

/// JSON view of a decision with its criteria and co-signatures, including which required roles have and have not signed.
fn view(d: &decisions::Model, c: &[criteria::Model], s: &[cosigs::Model]) -> Value {
    let required = rules::required_roles();
    let signed: Vec<&str> = s.iter().map(|x| x.role.as_str()).collect();
    json!({
        "pid": d.pid, "plan_pid": d.plan_pid, "title": d.title, "version": d.version,
        "supersedes_pid": d.supersedes_pid, "status": d.status,
        "published_on": d.published_on, "opened_on": d.opened_on,
        "outcome": d.outcome, "decided_on": d.decided_on, "decision_reason": d.decision_reason,
        "criteria": c.iter().map(|x| json!({ "n": x.position, "criterion": x.criterion, "met": x.met, "evidence": x.evidence, "marked_on": x.marked_on })).collect::<Vec<_>>(),
        "cosignatures": { "required": required, "signed": s.iter().map(|x| json!({ "role": x.role, "signed_on": x.signed_on })).collect::<Vec<_>>(),
            "missing": required.iter().filter(|r| !signed.contains(&r.as_str())).collect::<Vec<_>>() },
        "go_would_be_refused_because": if d.status == "open" { rules::go_problems(&c.iter().map(|x| x.met).collect::<Vec<_>>(), &required, &signed.iter().map(|x| (*x).to_string()).collect::<Vec<_>>()) } else { vec![] },
    })
}

/// Trims the criteria and requires 1 to 30 items of 5 to 500 characters each, otherwise unprocessable.
fn clean(list: &[String]) -> Result<Vec<String>> {
    let items: Vec<String> = list.iter().map(|x| x.trim().to_string()).collect();
    if !(1..=30).contains(&items.len()) {
        return Err(unprocessable("give 1 to 30 stop criteria"));
    }
    if items
        .iter()
        .any(|x| !(5..=500).contains(&x.chars().count()))
    {
        return Err(unprocessable("each stop criterion is 5 to 500 characters"));
    }
    Ok(items)
}

/// Inserts the criteria with 1-based positions in the given order, in the caller's transaction (or on the pool).
async fn put_criteria(
    db: &mut impl crate::db::Session,
    decision: Uuid,
    items: &[String],
) -> Result<()> {
    for (i, text) in items.iter().enumerate() {
        sqlx::query(
            "INSERT INTO cutover_criteria (pid, decision_pid, position, criterion) VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4())
        .bind(decision)
        .bind(i32::try_from(i + 1).unwrap_or(i32::MAX))
        .bind(text)
        .execute(db.ex())
        .await?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `POST /api/plans/{pid}/cutover-decisions`.
pub(crate) struct New {
    /// Name of the cutover decision.
    title: String,
    /// Stop criteria text, in order; see [`clean`] for the limits.
    criteria: Vec<String>,
}

/// `POST /api/plans/{pid}/cutover-decisions` — a draft, version 1.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<New>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !(3..=200).contains(&b.title.trim().chars().count()) {
        return Err(unprocessable("title is 3 to 200 characters"));
    }
    let items = clean(&b.criteria)?;
    let mut tx = crate::db::begin(&ctx).await?;
    let d: decisions::Model = sqlx::query_as(
        "INSERT INTO cutover_decisions (pid, plan_pid, title, version, status)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(b.title.trim())
    .bind(1_i32)
    .bind("draft")
    .fetch_one(tx.conn())
    .await
    .map_err(|e| {
        if e.to_string().contains("cd_version") {
            conflict("a decision with that title exists for this plan: make a new version of it")
        } else {
            e.into()
        }
    })?;
    put_criteria(&mut tx, d.pid, &items).await?;
    AuditModel::record_on(
        tx.conn(),
        d.pid,
        "cutover_decision_drafted",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

/// `GET /api/plans/{pid}/cutover-decisions`
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows: Vec<decisions::Model> =
        sqlx::query_as("SELECT * FROM cutover_decisions WHERE plan_pid = $1 ORDER BY id DESC")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let mut out = Vec::new();
    for d in &rows {
        let (c, s) = parts(&ctx, d).await?;
        out.push(view(d, &c, &s));
    }
    format::json(json!({ "decisions": out, "required_cosigners": rules::required_roles() }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `PUT .../criteria`: the full replacement list.
pub(crate) struct Criteria {
    /// Stop criteria text, in order; replaces the draft's list.
    criteria: Vec<String>,
}

/// `PUT …/criteria` — replace the criteria of a draft.
#[debug_handler]
async fn replace_criteria(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Criteria>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "draft" {
        return Err(conflict(
            "the criteria of a published decision cannot be edited: make a new version",
        ));
    }
    let items = clean(&b.criteria)?;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("DELETE FROM cutover_criteria WHERE decision_pid = $1")
        .bind(d.pid)
        .execute(tx.conn())
        .await?;
    put_criteria(&mut tx, d.pid, &items).await?;
    AuditModel::record_on(
        tx.conn(),
        d.pid,
        "cutover_criteria_edited",
        caller.actor(),
        Some(json!({ "count": items.len() })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

/// Moves a decision from status `from` to `to` (publish or open), stamping today's date, and audits `action` in the same transaction. Refuses with a conflict when the decision is not in `from`.
async fn advance(
    ctx: &AppContext,
    d: decisions::Model,
    from: &str,
    to: &str,
    action: &str,
    caller: &MaybeAuthUser,
) -> Result<Response> {
    if d.status != from {
        return Err(conflict(&format!(
            "this decision is {}, not {from}",
            d.status
        )));
    }
    let today = crate::tz::deployment_today();
    // The date column is one of two literals, chosen by `to`.
    let sql = if to == "published" {
        "UPDATE cutover_decisions SET status = $1, published_on = $2 WHERE id = $3 RETURNING *"
    } else {
        "UPDATE cutover_decisions SET status = $1, opened_on = $2 WHERE id = $3 RETURNING *"
    };
    let mut tx = crate::db::begin(ctx).await?;
    let row: decisions::Model = sqlx::query_as(sql)
        .bind(to)
        .bind(Some(today))
        .bind(d.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), row.pid, action, caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(ctx, &row).await?;
    format::json(view(&row, &c, &s))
}

/// `POST …/publish` — freeze the criteria.
#[debug_handler]
async fn publish(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    advance(
        &ctx,
        d,
        "draft",
        "published",
        "cutover_criteria_published",
        &caller,
    )
    .await
}

/// `POST …/open` — open the decision (the criteria were published first, and stay as they are).
#[debug_handler]
async fn open(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    advance(
        &ctx,
        d,
        "published",
        "open",
        "cutover_decision_opened",
        &caller,
    )
    .await
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `PUT .../criteria/{n}/mark`.
pub(crate) struct Mark {
    /// Whether the criterion is met.
    met: bool,
    /// The evidence supporting the mark.
    evidence: String,
}

/// `PUT …/criteria/{n}/mark` — mark a criterion met or not, with evidence.
#[debug_handler]
async fn mark(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did, n)): Path<(String, String, i32)>,
    Json(b): Json<Mark>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict("criteria are marked while the decision is open"));
    }
    if b.evidence.trim().chars().count() < 5 {
        return Err(unprocessable(
            "evidence of at least 5 characters is required",
        ));
    }
    let c: criteria::Model = sqlx::query_as(
        "SELECT * FROM cutover_criteria WHERE decision_pid = $1 AND position = $2 LIMIT 1",
    )
    .bind(d.pid)
    .bind(n)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "UPDATE cutover_criteria SET met = $1, evidence = $2, marked_by = $3, marked_on = $4
         WHERE id = $5
         RETURNING id",
    )
    .bind(Some(b.met))
    .bind(Some(b.evidence.trim()))
    .bind(caller.actor())
    .bind(Some(crate::tz::deployment_today()))
    .bind(c.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        d.pid,
        "cutover_criterion_marked",
        caller.actor(),
        Some(json!({ "n": n, "met": b.met })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `POST .../cosign`.
pub(crate) struct Cosign {
    /// Role being signed for; the caller's token must carry it in `cutover_role`.
    role: String,
}

/// `POST …/cosign` — co-sign for a role; the caller's token must carry it in `cutover_role`.
#[debug_handler]
async fn cosign(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Cosign>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict(
            "co-signatures are given while the decision is open",
        ));
    }
    let role = b.role.trim().to_lowercase();
    if !rules::required_roles().contains(&role) {
        return Err(unprocessable(&format!(
            "role must be one of {:?}",
            rules::required_roles()
        )));
    }
    let holds = caller.holds_or_open(|c| {
        c.attrs
            .get("cutover_role")
            .is_some_and(|v| v.iter().any(|r| r.eq_ignore_ascii_case(&role)))
    });
    if !holds {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new(
                "forbidden",
                "only a person holding that role (the cutover_role attribute) co-signs for it",
            ),
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query(
        "INSERT INTO cutover_cosignatures (pid, decision_pid, role, signed_by, signed_on)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(d.pid)
    .bind(&role)
    .bind(caller.actor())
    .bind(crate::tz::deployment_today())
    .execute(tx.conn())
    .await
    .map_err(|e| {
        if e.to_string().contains("ccs_once") {
            conflict("that role has already co-signed")
        } else {
            e.into()
        }
    })?;
    AuditModel::record_on(
        tx.conn(),
        d.pid,
        "cutover_cosigned",
        caller.actor(),
        Some(json!({ "role": role })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(&ctx, &d).await?;
    format::json(view(&d, &c, &s))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `POST .../decide`.
pub(crate) struct Decide {
    /// `go`, `no_go` or a delay for safety; go needs every criterion met and every co-signature.
    outcome: String,
    /// Why the outcome was chosen; recorded with the decision.
    reason: String,
}

/// `POST …/decide` — go, no-go or delay for safety.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
    Json(b): Json<Decide>,
) -> Result<Response> {
    let (_, d) = load(&ctx, &pid, &did).await?;
    if d.status != "open" {
        return Err(conflict("only an open decision is decided"));
    }
    if !rules::OUTCOMES.contains(&b.outcome.as_str()) {
        return Err(unprocessable(&format!(
            "outcome must be one of {:?}",
            rules::OUTCOMES
        )));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a reason of at least 10 characters is required",
        ));
    }
    let (c, s) = parts(&ctx, &d).await?;
    if b.outcome == "go" {
        let signed: Vec<String> = s.iter().map(|x| x.role.clone()).collect();
        let problems = rules::go_problems(
            &c.iter().map(|x| x.met).collect::<Vec<_>>(),
            &rules::required_roles(),
            &signed,
        );
        if !problems.is_empty() {
            return Err(unprocessable(&format!(
                "a go is refused: {}",
                problems.join("; ")
            )));
        }
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: decisions::Model = sqlx::query_as(
        "UPDATE cutover_decisions
         SET status = $1, outcome = $2, decided_on = $3, decided_by = $4, decision_reason = $5
         WHERE id = $6
         RETURNING *",
    )
    .bind("decided")
    .bind(Some(&b.outcome))
    .bind(Some(crate::tz::deployment_today()))
    .bind(caller.actor())
    .bind(Some(b.reason.trim()))
    .bind(d.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "cutover_decided",
        caller.actor(),
        Some(json!({ "outcome": b.outcome })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, &c, &s))
}

/// `POST …/new-version` — a new draft with the same criteria, to change them. A published or open decision is
/// superseded; a decided one stays as decided (the new version supersedes it by reference).
#[debug_handler]
async fn new_version(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, did)): Path<(String, String)>,
) -> Result<Response> {
    let (plan, d) = load(&ctx, &pid, &did).await?;
    if d.status == "draft" || d.status == "superseded" {
        return Err(conflict(
            "only a published, open or decided decision has a new version",
        ));
    }
    if d.outcome.as_deref() == Some("go") {
        return Err(conflict("a decision that went ahead is final"));
    }
    let latest: decisions::Model = sqlx::query_as(
        "SELECT * FROM cutover_decisions WHERE plan_pid = $1 AND lower(title) = lower($2)
         ORDER BY version DESC
         LIMIT 1",
    )
    .bind(plan.pid)
    .bind(&d.title)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    if latest.pid != d.pid {
        return Err(conflict("there is already a newer version"));
    }
    let (old_c, _) = parts(&ctx, &d).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    if d.status != "decided" {
        sqlx::query("UPDATE cutover_decisions SET status = $1 WHERE id = $2 RETURNING id")
            .bind("superseded")
            .bind(d.id)
            .fetch_one(tx.conn())
            .await?;
    }
    let nd: decisions::Model = sqlx::query_as(
        "INSERT INTO cutover_decisions (pid, plan_pid, title, version, supersedes_pid, status)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(&d.title)
    .bind(d.version + 1)
    .bind(Some(d.pid))
    .bind("draft")
    .fetch_one(tx.conn())
    .await?;
    put_criteria(
        &mut tx,
        nd.pid,
        &old_c
            .iter()
            .map(|x| x.criterion.clone())
            .collect::<Vec<_>>(),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        nd.pid,
        "cutover_decision_new_version",
        caller.actor(),
        Some(json!({ "supersedes": d.pid })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    let (c, s) = parts(&ctx, &nd).await?;
    format::json(view(&nd, &c, &s))
}

/// The pack's section: the decisions made in the period, each as what it is. A no-go or a delay for safety is
/// a safety decision and is counted apart; it is not slippage.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(
    ctx: &AppContext,
    include: &std::collections::HashSet<Uuid>,
    period: &str,
) -> Result<Value> {
    let names: std::collections::HashMap<Uuid, String> =
        sqlx::query_as::<_, (Uuid, String)>("SELECT pid, name FROM plans WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .filter(|(pid, _)| include.contains(pid))
            .collect();
    let rows: Vec<decisions::Model> = sqlx::query_as::<_, decisions::Model>(
        "SELECT * FROM cutover_decisions WHERE status = $1 ORDER BY decided_on",
    )
    .bind("decided")
    .fetch_all(crate::db::pool(ctx))
    .await?
    .into_iter()
    .filter(|d| {
        names.contains_key(&d.plan_pid)
            && d.decided_on
                .is_some_and(|x| x.format("%Y-%m").to_string() == period)
    })
    .collect();
    let count = |o: &str| {
        rows.iter()
            .filter(|d| d.outcome.as_deref() == Some(o))
            .count()
    };
    Ok(json!({
        "decisions": rows.iter().map(|d| json!({ "plan": names[&d.plan_pid], "title": d.title, "version": d.version, "outcome": d.outcome, "decided_on": d.decided_on })).collect::<Vec<_>>(),
        "counts": { "go": count("go"), "no_go": count("no_go"), "delay_for_safety": count("delay_for_safety") },
        "note": "A cutover decided in the period. A no-go or a delay for safety is a decision taken on the stop criteria, and is not counted as slippage.",
    }))
}
