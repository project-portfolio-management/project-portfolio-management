//! Break-glass access to a private plan (RG-10). Rules in [`crate::break_glass`].
//!
//! A person with the `breakglass=eligible` attribute (or `access=admin`) opens a **private** plan for up to
//! four hours by giving a reason and naming a second person, who is told in the same transaction; if
//! that person cannot be told (they switched the notice off) **no grant is made**. While a grant lives the
//! plan is no longer hidden from the grantee (see `plan_access`); every request that gets through only
//! because of it is audited. A grant ends at its expiry or when it is revoked, and is never deleted.

use super::refuse;
use crate::prelude::*;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::break_glass as rules;
use crate::models::_entities::{break_glass_grants as grants, plans};
use crate::models::audit_logs::Model as AuditModel;

/// The JSON shape for one grant, including whether it is still live (unexpired and not revoked) at `now`.
fn view(g: &grants::Model, now: chrono::DateTime<Utc>) -> serde_json::Value {
    serde_json::json!({
        "pid": g.pid, "plan_pid": g.plan_pid, "sub": g.sub, "reason": g.reason,
        "notify_ref": g.notify_ref, "opened_at": g.created_at, "expires_at": g.expires_at,
        "revoked_at": g.revoked_at, "revoked_by": g.revoked_by,
        "live": rules::is_live(g.expires_at.into(), g.revoked_at.is_some(), now),
    })
}

/// `POST /api/break-glass` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct OpenBody {
    /// The private plan to open.
    plan_pid: Uuid,
    /// Why access is needed; length-checked and recorded in the audit trail.
    reason: String,
    /// Requested duration in minutes; absent uses the default, and the value is capped at the maximum.
    #[serde(default)]
    minutes: Option<i64>,
    /// Person reference told that the grant was opened, so the access is never silent.
    notify_ref: String,
}

/// What must hold before a grant may be made: the plan exists and is private, the person has not used up
/// their day, and they do not already hold a live grant on it.
async fn preconditions(
    ctx: &AppContext,
    plan_pid: Uuid,
    sub: &str,
    now: chrono::DateTime<Utc>,
) -> Result<plans::Model> {
    // The same answer for "no such plan" and "a plan you could not otherwise name" is not needed here:
    // an eligible person is told whether the plan exists, but only that, never what is in it.
    let plan: plans::Model =
        sqlx::query_as("SELECT * FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let private = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM plan_access_settings WHERE plan_pid = $1 AND private = true",
    )
    .bind(plan.pid)
    .fetch_one(crate::db::pool(ctx))
    .await?
        > 0;
    if !private {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "that plan is not private: there is nothing to open",
        ));
    }
    let recent: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM break_glass_grants WHERE sub = $1 AND created_at > $2",
    )
    .bind(sub)
    .bind(now - chrono::Duration::days(1))
    .fetch_one(crate::db::pool(ctx))
    .await?;
    if u64::try_from(recent).unwrap_or(0) >= rules::MAX_PER_DAY {
        return Err(refuse(
            StatusCode::TOO_MANY_REQUESTS,
            "too_many_requests",
            "you have opened the most break-glass grants allowed in a day",
        ));
    }
    let live: Option<grants::Model> = sqlx::query_as(
        "SELECT * FROM break_glass_grants
         WHERE plan_pid = $1 AND sub = $2 AND revoked_at IS NULL AND expires_at > $3
         LIMIT 1",
    )
    .bind(plan.pid)
    .bind(sub)
    .bind(now)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    if let Some(g) = live {
        return Err(refuse(
            StatusCode::CONFLICT,
            "conflict",
            &format!(
                "you already hold a grant for this plan until {}",
                g.expires_at
            ),
        ));
    }
    Ok(plan)
}

/// `POST /api/break-glass` — open a private plan for a limited time.
#[debug_handler]
async fn open(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<OpenBody>,
) -> Result<Response> {
    if !crate::auth::area_allows(
        &caller,
        "break_glass",
        authentication_verifier::Action::Write,
    ) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "break-glass access needs breakglass=eligible or access=admin",
        ));
    }
    let Some(sub) = caller.actor().map(str::to_string) else {
        return Err(refuse(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "sign in first",
        ));
    };
    let problems = rules::problems(&b.reason, b.minutes, &b.notify_ref, &sub);
    if !problems.is_empty() {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            &problems.join("; "),
        ));
    }
    let now = Utc::now();
    let plan = preconditions(&ctx, b.plan_pid, &sub, now).await?;
    let expires = rules::expires_at(now, b.minutes);
    let minutes = b.minutes.unwrap_or(rules::DEFAULT_MINUTES);
    // The grant and the notice stand or fall together.
    let mut tx = crate::db::begin(&ctx).await?;
    let row: grants::Model = sqlx::query_as(
        "INSERT INTO break_glass_grants
             (pid, plan_pid, sub, reason, notify_ref, expires_at, revoked_at, revoked_by)
         VALUES ($1, $2, $3, $4, $5, $6, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(&sub)
    .bind(b.reason.trim())
    .bind(b.notify_ref.trim())
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(expires))
    .fetch_one(tx.conn())
    .await?;
    let told = crate::models::capabilities::notify_on(
        &mut tx,
        &row.notify_ref,
        "plan",
        plan.pid,
        "break_glass",
        &rules::notice(&sub, minutes, &row.reason),
    )
    .await?;
    if told.is_none() {
        // Dropped without a commit, the transaction rolls back: no grant, no notice.
        drop(tx);
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "the person to notify has switched this notice off; choose someone who can be told",
        ));
    }
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "break_glass_opened",
        Some(&sub),
        Some(serde_json::json!({
            "plan_pid": plan.pid, "minutes": minutes, "expires_at": expires,
            "notify_ref": row.notify_ref, "reason": row.reason,
        })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "pid": row.pid, "plan_pid": plan.pid, "expires_at": expires, "notified": true,
    }))
}

/// `GET /api/break-glass` — your grants (all of them for an administrator), newest first.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let Some(sub) = caller.actor().map(str::to_string) else {
        return Err(refuse(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "sign in first",
        ));
    };
    let mut find = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM break_glass_grants");
    if !super::plan_access::is_admin(caller.claims()) {
        find.push(" WHERE sub = ").push_bind(sub);
    }
    find.push(" ORDER BY id DESC LIMIT 500");
    let rows: Vec<grants::Model> = find
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let now = Utc::now();
    format::json(serde_json::json!({
        "grants": rows.iter().map(|g| view(g, now)).collect::<Vec<_>>(),
        "live": rows.iter().filter(|g| rules::is_live(g.expires_at.into(), g.revoked_at.is_some(), now)).count(),
    }))
}

/// `POST /api/break-glass/{pid}/revoke` — end a grant now (its holder, or an administrator).
#[debug_handler]
async fn revoke(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let Some(sub) = caller.actor().map(str::to_string) else {
        return Err(refuse(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "sign in first",
        ));
    };
    let pid = super::parse_pid(&pid)?;
    let row: grants::Model =
        sqlx::query_as("SELECT * FROM break_glass_grants WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    // Someone else's grant does not exist for you (unless you administer).
    if row.sub != sub && !super::plan_access::is_admin(caller.claims()) {
        return Err(Error::NotFound);
    }
    if row.revoked_at.is_some() {
        return format::json(view(&row, Utc::now()));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as `SeaORM`'s was.
    let row: grants::Model = sqlx::query_as(
        "UPDATE break_glass_grants SET revoked_at = $1, revoked_by = $2 WHERE id = $3 RETURNING *",
    )
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(Utc::now()))
    .bind(&sub)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "break_glass_revoked", Some(&sub), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, Utc::now()))
}

/// The break-glass routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/break-glass")
        .add("", post(open))
        .add("", get(list))
        .add("/{pid}/revoke", post(revoke))
}
