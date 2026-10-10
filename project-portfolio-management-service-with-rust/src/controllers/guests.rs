//! Guest access (PL-7): grants for outside collaborators and the guard that
//! confines them. Pure rules in [`crate::guests`].
//!
//! A guest (a token with `role=guest`) can attempt only a narrow set of
//! requests; each is then checked against **live grants** on the plan. Anything
//! else is `404`, so a guest cannot enumerate plans, search, or read a report.
//! The plan record a guest reads is **masked** (no people references), and the
//! sub-resources that name people (tasks, risks, allocations) are not offered
//! to them at all. Grants are time-boxed, revocable, audited and listed per
//! plan.

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use chrono::{DateTime, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::guests::{self as rules, Attempt, Grant};
use crate::models::_entities::guest_grants;
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn roles_of(claims: &authentication_verifier::Claims) -> Vec<String> {
    claims.attrs.get("role").cloned().unwrap_or_default()
}

/// Whether the caller is a guest.
#[must_use]
pub fn caller_is_guest(caller: &MaybeAuthUser) -> bool {
    caller
        .claims()
        .is_some_and(|c| rules::is_guest(&roles_of(c)))
}

/// A guest's live grants (empty on a lookup failure: fail closed).
pub(crate) async fn grants_of(db: &DatabaseConnection, sub: &str) -> Vec<Grant> {
    let now = Utc::now();
    guest_grants::Entity::find()
        .filter(guest_grants::Column::GuestSub.eq(sub))
        .all(db)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|g| Grant {
                    live: rules::grant_live(
                        g.expires_at.map(|e| e.to_utc()),
                        g.revoked_at.is_some(),
                        now,
                    ),
                    subject_kind: g.subject_kind,
                    subject_pid: g.subject_pid,
                    access: g.access,
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Whether a guest holds a live grant covering `plan` for `need`.
pub(crate) async fn guest_covers_plan(
    db: &DatabaseConnection,
    caller: &MaybeAuthUser,
    plan: Uuid,
    need: &str,
) -> bool {
    let Some(claims) = caller.claims() else {
        return false;
    };
    rules::covers(&grants_of(db, &claims.sub).await, "plan", plan, need)
}

fn not_found() -> AxumResponse {
    (
        StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({ "error": "not_found" })),
    )
        .into_response()
}

fn query_param(query: Option<&str>, key: &str) -> Option<String> {
    query?
        .split('&')
        .find_map(|kv| kv.strip_prefix(&format!("{key}=")))
        .map(str::to_string)
}

/// The guard: non-guests pass untouched (no lookup); a guest is confined.
pub async fn guest_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> AxumResponse {
    let Ok(claims) = crate::auth::request_claims(req.headers()).await else {
        return next.run(req).await;
    };
    if !rules::is_guest(&roles_of(&claims)) {
        return next.run(req).await;
    }
    let attempt = rules::classify(req.method().as_str(), req.uri().path());
    let allowed = match attempt {
        Attempt::ReadPlan(pid) => {
            rules::covers(&grants_of(&ctx.db, &claims.sub).await, "plan", pid, "read")
        }
        // Comments and attachments name their subject in the query (or, for a
        // POST, the body); the handler re-checks it against the grants via
        // `resolve_subject`. Here a guest merely may attempt it.
        Attempt::Own | Attempt::ReadComments | Attempt::PostComment | Attempt::ReadAttachments => {
            true
        }
        Attempt::Denied => false,
    };
    if allowed {
        // For a comment/attachment read the subject is in the query: check it
        // before the handler so an ungranted subject never reaches it.
        if matches!(attempt, Attempt::ReadComments | Attempt::ReadAttachments) {
            let q = req.uri().query();
            let (kind, pid) = (
                query_param(q, "subject_kind"),
                query_param(q, "subject_pid"),
            );
            let granted = match (kind, pid.and_then(|p| Uuid::parse_str(&p).ok())) {
                (Some(k), Some(p)) => match plan_of(&ctx.db, &k, p).await {
                    Some(plan) => {
                        rules::covers(&grants_of(&ctx.db, &claims.sub).await, "plan", plan, "read")
                    }
                    None => false,
                },
                _ => false,
            };
            if !granted {
                return not_found();
            }
        }
        return next.run(req).await;
    }
    not_found()
}

/// The plan a subject belongs to (a plan is its own).
pub(crate) async fn plan_of(db: &DatabaseConnection, kind: &str, pid: Uuid) -> Option<Uuid> {
    use crate::models::_entities::{risks, tasks};
    match kind {
        "plan" => Some(pid),
        "task" => tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(pid))
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|t| t.plan_pid),
        "risk" => risks::Entity::find()
            .filter(risks::Column::Pid.eq(pid))
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|r| r.plan_pid),
        _ => None,
    }
}

/// `POST /api/plans/{pid}/guests` body.
#[derive(Debug, Deserialize)]
struct GrantBody {
    /// The guest's user id (their token `sub`).
    guest_sub: Uuid,
    /// `read` or `comment`.
    access: String,
    #[serde(default)]
    expires_at: Option<DateTime<Utc>>,
}

/// `POST /api/plans/{pid}/guests` — grant a guest access to this plan.
#[debug_handler]
async fn grant(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<GrantBody>,
) -> Result<Response> {
    if !rules::ACCESS.contains(&b.access.as_str()) {
        return Err(unprocessable(&format!(
            "access must be one of {:?}",
            rules::ACCESS
        )));
    }
    if b.expires_at.is_some_and(|e| e <= Utc::now()) {
        return Err(unprocessable("expires_at must be in the future"));
    }
    let caller = MaybeAuthUser(Some(claims.clone()));
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let row = guest_grants::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        guest_sub: ActiveValue::set(b.guest_sub.to_string()),
        subject_kind: ActiveValue::set("plan".to_string()),
        subject_pid: ActiveValue::set(subject.pid),
        access: ActiveValue::set(b.access.clone()),
        expires_at: ActiveValue::set(b.expires_at.map(Into::into)),
        granted_by: ActiveValue::set(Some(claims.sub.clone())),
        revoked_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        subject.pid,
        "guest_granted",
        Some(&claims.sub),
        Some(serde_json::json!({ "guest": b.guest_sub, "access": b.access, "expires_at": b.expires_at })),
    )
    .await
    .ok();
    format::json(row)
}

/// `GET /api/plans/{pid}/guests` — the plan's grants, live and not.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let rows = guest_grants::Entity::find()
        .filter(guest_grants::Column::SubjectKind.eq("plan"))
        .filter(guest_grants::Column::SubjectPid.eq(subject.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let now = Utc::now();
    format::json(
        rows.iter()
            .map(|g| {
                serde_json::json!({
                    "pid": g.pid, "guest_sub": g.guest_sub, "access": g.access,
                    "expires_at": g.expires_at, "granted_by": g.granted_by, "revoked_at": g.revoked_at,
                    "live": rules::grant_live(g.expires_at.map(|e| e.to_utc()), g.revoked_at.is_some(), now),
                })
            })
            .collect::<Vec<_>>(),
    )
}

/// `POST /api/plans/{pid}/guests/{grant}/revoke` — take the access away,
/// effective on the guest's next request.
#[debug_handler]
async fn revoke(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path((pid, grant)): Path<(String, String)>,
) -> Result<Response> {
    let caller = MaybeAuthUser(Some(claims.clone()));
    let subject = resolve_subject(&ctx, &caller, "plan", &pid).await?;
    let grant = Uuid::parse_str(&grant).map_err(|_| Error::NotFound)?;
    let row = guest_grants::Entity::find()
        .filter(guest_grants::Column::Pid.eq(grant))
        .filter(guest_grants::Column::SubjectPid.eq(subject.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if row.revoked_at.is_none() {
        let mut a: guest_grants::ActiveModel = row.into();
        a.revoked_at = ActiveValue::set(Some(Utc::now().into()));
        a.update(&ctx.db).await.map_err(db_err)?;
        AuditModel::record(
            &ctx.db,
            subject.pid,
            "guest_revoked",
            Some(&claims.sub),
            None,
        )
        .await
        .ok();
    }
    format::empty_json()
}

/// The guest routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/guests", post(grant))
        .add("/plans/{pid}/guests", get(list))
        .add("/plans/{pid}/guests/{grant}/revoke", post(revoke))
}
