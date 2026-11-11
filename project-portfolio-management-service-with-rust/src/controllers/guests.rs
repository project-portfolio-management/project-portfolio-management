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

use super::unprocessable;
use crate::prelude::*;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::guests::{self as rules, Attempt, Grant};
use crate::models::_entities::guest_grants;
use crate::models::audit_logs::Model as AuditModel;

/// The caller's `role` attribute values; empty when none.
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
pub(crate) async fn grants_of(ctx: &AppContext, sub: &str) -> Vec<Grant> {
    let now = Utc::now();
    sqlx::query_as::<_, guest_grants::Model>("SELECT * FROM guest_grants WHERE guest_sub = $1")
        .bind(sub)
        .fetch_all(crate::db::pool(ctx))
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
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    need: &str,
) -> bool {
    let Some(claims) = caller.claims() else {
        return false;
    };
    rules::covers(&grants_of(ctx, &claims.sub).await, "plan", plan, need)
}

/// The `404` a missing record answers, used for every guest refusal, so a guest cannot tell a hidden record from a
/// missing one.
fn not_found() -> AxumResponse {
    axa::Error::NotFound.into_response()
}

/// The raw (not percent-decoded) value of one query-string key, or `None`; used by the guard before any handler runs.
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
    // A confined pack reader (the partner case, DP-1) sees the director packs and nothing live.
    if crate::director_pack::is_confined(&roles_of(&claims)) {
        if crate::director_pack::confined_allows(req.method().as_str(), req.uri().path()) {
            return next.run(req).await;
        }
        return not_found();
    }
    if !rules::is_guest(&roles_of(&claims)) {
        return next.run(req).await;
    }
    let attempt = rules::classify(req.method().as_str(), req.uri().path());
    let allowed = match attempt {
        Attempt::ReadPlan(pid) => {
            rules::covers(&grants_of(&ctx, &claims.sub).await, "plan", pid, "read")
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
                (Some(k), Some(p)) => match plan_of(&ctx, &k, p).await {
                    Some(plan) => {
                        rules::covers(&grants_of(&ctx, &claims.sub).await, "plan", plan, "read")
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
pub(crate) async fn plan_of(ctx: &AppContext, kind: &str, pid: Uuid) -> Option<Uuid> {
    // Any row, deleted or not, as before; a lookup failure is `None` (fail closed).
    let sql = match kind {
        "plan" => return Some(pid),
        "task" => "SELECT plan_pid FROM tasks WHERE pid = $1 LIMIT 1",
        "risk" => "SELECT plan_pid FROM risks WHERE pid = $1 LIMIT 1",
        _ => return None,
    };
    sqlx::query_scalar::<_, Uuid>(sql)
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
        .ok()
        .flatten()
}

/// `POST /api/plans/{pid}/guests` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct GrantBody {
    /// The guest's user id (their token `sub`).
    guest_sub: Uuid,
    /// `read` or `comment`.
    access: String,
    /// When the grant lapses; absent means it stays live until revoked.
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
    let mut tx = crate::db::begin(&ctx).await?;
    let row: guest_grants::Model = sqlx::query_as(
        "INSERT INTO guest_grants
             (pid, guest_sub, subject_kind, subject_pid, access, expires_at, granted_by, revoked_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(b.guest_sub.to_string())
    .bind("plan")
    .bind(subject.pid)
    .bind(&b.access)
    .bind(
        b.expires_at
            .map(chrono::DateTime::<chrono::FixedOffset>::from),
    )
    .bind(Some(claims.sub.as_str()))
    .bind(None::<chrono::DateTime<chrono::FixedOffset>>)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        subject.pid,
        "guest_granted",
        Some(&claims.sub),
        Some(serde_json::json!({ "guest": b.guest_sub, "access": b.access, "expires_at": b.expires_at })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    let rows: Vec<guest_grants::Model> =
        sqlx::query_as("SELECT * FROM guest_grants WHERE subject_kind = $1 AND subject_pid = $2")
            .bind("plan")
            .bind(subject.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
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
    let grant = super::parse_pid(&grant)?;
    let row: guest_grants::Model =
        sqlx::query_as("SELECT * FROM guest_grants WHERE pid = $1 AND subject_pid = $2 LIMIT 1")
            .bind(grant)
            .bind(subject.pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    if row.revoked_at.is_none() {
        let revoked_at = chrono::DateTime::<chrono::FixedOffset>::from(Utc::now());
        let mut tx = crate::db::begin(&ctx).await?;
        // `fetch_one`: an update that finds no row is an error, as before SQ-1.
        sqlx::query_scalar::<_, i64>(
            "UPDATE guest_grants SET revoked_at = $1 WHERE id = $2 RETURNING id",
        )
        .bind(Some(revoked_at))
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
        AuditModel::record_on(
            tx.conn(),
            subject.pid,
            "guest_revoked",
            Some(&claims.sub),
            None,
        )
        .await
        .map_err(super::audit_failed)?;
        tx.commit().await?;
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
