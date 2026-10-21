//! The access review (RG-9): the grants that stand today across the plans, oldest first, for
//! periodic recertification. Rules in [`crate::access_review`].
//!
//! **Administrators only** (`access=admin`; the `access_review` area is closed to the policy file).
//! Plans the caller cannot read are still left out of the answer. `?format=csv` returns a spreadsheet
//! with a column for the reviewer's decision. Read-only and audited.

use super::refuse;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use crate::access_review::{self as rules, DEFAULT_STALE_DAYS, Entry};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::access_records;
use crate::models::audit_logs::Model as AuditModel;

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    plan: Option<Uuid>,
    /// Calendar days after which a grant is flagged for review.
    #[serde(default)]
    older_than_days: Option<i64>,
    #[serde(default)]
    format: Option<String>,
}

/// `GET /api/access-review` — the grants that stand today.
#[debug_handler]
async fn review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<Params>,
) -> Result<Response> {
    if !crate::auth::area_allows(
        &caller,
        "access_review",
        authentication_verifier::Action::Write,
    ) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "the access review needs access=admin",
        ));
    }
    let stale = q.older_than_days.unwrap_or(DEFAULT_STALE_DAYS);
    if stale < 0 {
        return Err(refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
            "older_than_days cannot be negative",
        ));
    }
    let csv = match q.format.as_deref() {
        None | Some("json") => false,
        Some("csv") => true,
        Some(_) => {
            return Err(refuse(
                StatusCode::UNPROCESSABLE_ENTITY,
                "unprocessable_entity",
                "format must be json or csv",
            ));
        }
    };
    let mut find = access_records::Entity::find();
    if let Some(p) = q.plan {
        find = find.filter(access_records::Column::PlanPid.eq(p));
    }
    let rows = find
        .order_by_asc(access_records::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut plans: Vec<Uuid> = rows.iter().map(|r| r.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    let entries: Vec<Entry> = rows
        .into_iter()
        .filter(|r| readable.contains(&r.plan_pid))
        .map(|r| Entry {
            id: r.id,
            plan_pid: r.plan_pid,
            person_ref: r.person_ref,
            access: r.access,
            action: r.action,
            recorded_on: r.recorded_on,
            recorded_by: r.recorded_by,
            note: r.note,
        })
        .collect();
    let today = Utc::now().date_naive();
    let standing = rules::standing(&entries, today, stale);
    AuditModel::record_or_log(
        &ctx.db,
        Uuid::nil(),
        "access_review_read",
        caller.actor(),
        Some(serde_json::json!({ "grants": standing.len(), "older_than_days": stale })),
    )
    .await;
    if csv {
        return Ok((
            [
                (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"access-review.csv\"",
                ),
            ],
            rules::to_csv(&standing),
        )
            .into_response());
    }
    let due = standing.iter().filter(|s| s.review_due).count();
    format::json(serde_json::json!({
        "as_of": today,
        "older_than_days": stale,
        "standing": standing.len(),
        "review_due": due,
        "grants": standing.iter().map(|s| serde_json::json!({
            "plan_pid": s.entry.plan_pid, "person_ref": s.entry.person_ref,
            "access": s.entry.access, "granted_on": s.entry.recorded_on,
            "granted_by": s.entry.recorded_by, "note": s.entry.note,
            "age_calendar_days": s.age_calendar_days, "review_due": s.review_due,
        })).collect::<Vec<_>>(),
        "note": "A grant stands when the latest ledger entry for the person, plan and access is a grant. This is what people recorded, not what the identity provider enforces: compare the two. Oldest first.",
    }))
}

/// The access-review routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/access-review", get(review))
}
