//! Break-glass access to a private plan (RG-10). Rules in [`crate::break_glass`].
//!
//! A person with the `breakglass=eligible` attribute (or `access=admin`) opens a **private** plan for up to
//! four hours by giving a reason and naming a second person, who is told in the same transaction; if
//! that person cannot be told (they switched the notice off) **no grant is made**. While a grant lives the
//! plan is no longer hidden from the grantee (see `plan_access`); every request that gets through only
//! because of it is audited. A grant ends at its expiry or when it is revoked, and is never deleted.

use super::refuse;
use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use crate::auth::MaybeAuthUser;
use crate::break_glass as rules;
use crate::models::_entities::{
    break_glass_grants as grants, plan_access_settings as plan_access, plans,
};
use crate::models::audit_logs::Model as AuditModel;

fn view(g: &grants::Model, now: chrono::DateTime<Utc>) -> serde_json::Value {
    serde_json::json!({
        "pid": g.pid, "plan_pid": g.plan_pid, "sub": g.sub, "reason": g.reason,
        "notify_ref": g.notify_ref, "opened_at": g.created_at, "expires_at": g.expires_at,
        "revoked_at": g.revoked_at, "revoked_by": g.revoked_by,
        "live": rules::is_live(g.expires_at.into(), g.revoked_at.is_some(), now),
    })
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct OpenBody {
    plan_pid: Uuid,
    reason: String,
    #[serde(default)]
    minutes: Option<i64>,
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
    let plan = plans::Entity::find()
        .filter(plans::Column::Pid.eq(plan_pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let private = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.eq(plan.pid))
        .filter(plan_access::Column::Private.eq(true))
        .count(&ctx.db)
        .await
        .map_err(db_err)?
        > 0;
    if !private {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "that plan is not private: there is nothing to open",
        ));
    }
    let recent = grants::Entity::find()
        .filter(grants::Column::Sub.eq(sub))
        .filter(grants::Column::CreatedAt.gt(now - chrono::Duration::days(1)))
        .count(&ctx.db)
        .await
        .map_err(db_err)?;
    if recent >= rules::MAX_PER_DAY {
        return Err(refuse(
            StatusCode::TOO_MANY_REQUESTS,
            "too_many_requests",
            "you have opened the most break-glass grants allowed in a day",
        ));
    }
    let live = grants::Entity::find()
        .filter(grants::Column::PlanPid.eq(plan.pid))
        .filter(grants::Column::Sub.eq(sub))
        .filter(grants::Column::RevokedAt.is_null())
        .filter(grants::Column::ExpiresAt.gt(now))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
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
    let tx = ctx.db.begin().await.map_err(db_err)?;
    let row = grants::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        sub: ActiveValue::set(sub.clone()),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        notify_ref: ActiveValue::set(b.notify_ref.trim().to_string()),
        expires_at: ActiveValue::set(expires.into()),
        revoked_at: ActiveValue::set(None),
        revoked_by: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&tx)
    .await
    .map_err(db_err)?;
    let told = crate::models::capabilities::notify(
        &tx,
        &row.notify_ref,
        "plan",
        plan.pid,
        "break_glass",
        &rules::notice(&sub, minutes, &row.reason),
    )
    .await?;
    if told.is_none() {
        tx.rollback().await.map_err(db_err)?;
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "the person to notify has switched this notice off; choose someone who can be told",
        ));
    }
    AuditModel::record(
        &tx,
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
    tx.commit().await.map_err(db_err)?;
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
    let mut find = grants::Entity::find();
    if !super::plan_access::is_admin(caller.claims()) {
        find = find.filter(grants::Column::Sub.eq(sub));
    }
    let rows = find
        .order_by_desc(grants::Column::Id)
        .limit(500)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
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
    let row = grants::Entity::find()
        .filter(grants::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    // Someone else's grant does not exist for you (unless you administer).
    if row.sub != sub && !super::plan_access::is_admin(caller.claims()) {
        return Err(Error::NotFound);
    }
    if row.revoked_at.is_some() {
        return format::json(view(&row, Utc::now()));
    }
    let mut a: grants::ActiveModel = row.into();
    a.revoked_at = ActiveValue::set(Some(Utc::now().into()));
    a.revoked_by = ActiveValue::set(Some(sub.clone()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "break_glass_revoked", Some(&sub), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
