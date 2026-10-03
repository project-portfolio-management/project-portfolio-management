//! Availability over time (PJ-3): a person's capacity by date range. Pure rules
//! in [`crate::availability`].
//!
//! **Who may do what.** A person may read and set **their own** ranges; a
//! resource manager (token `role=resource_manager`) or an admin may for anyone.
//! With authentication off there is no caller to refuse, the deployment's posture
//! for every write. Another person's ranges are `404`, not `403`, so their
//! existence is not confirmed.
//!
//! Ranges for one person may not overlap (409). That is checked in the service,
//! not by a database exclusion constraint, so two simultaneous requests could both
//! pass; the second would then be read as the first match, which is the same
//! answer on every day it covers (the capacity of a day is read from the first
//! range found).

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::availability::{self as rules, Range};
use crate::models::_entities::person_availabilities;
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

/// Whether the caller may read or change `person_ref`'s availability and
/// bookings: themselves, a resource manager, an admin — or anyone when
/// authentication is off.
pub(crate) fn may_manage_person(caller: &MaybeAuthUser, person_ref: &str) -> bool {
    caller.claims().is_none_or(|c| {
        let has = |key: &str, want: &str| {
            c.attrs
                .get(key)
                .is_some_and(|v| v.iter().any(|a| a == want))
        };
        has("access", "admin")
            || has("role", "resource_manager")
            || Uuid::parse_str(&c.sub).is_ok_and(|u| person_ref == format!("person:{u}"))
    })
}

/// A person's ranges, as pure [`Range`]s.
pub(crate) async fn ranges_for(ctx: &AppContext, person_ref: &str) -> Result<Vec<Range>> {
    Ok(person_availabilities::Entity::find()
        .filter(person_availabilities::Column::PersonRef.eq(person_ref))
        .filter(person_availabilities::Column::DeletedAt.is_null())
        .order_by_asc(person_availabilities::Column::FromDate)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| Range {
            from: r.from_date,
            to: r.to_date,
            max_percent: r.max_percent,
        })
        .collect())
}

fn view(r: &person_availabilities::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": r.pid, "person_ref": r.person_ref, "from": r.from_date, "to": r.to_date,
        "max_percent": r.max_percent, "note": r.note,
    })
}

/// `POST /api/availability` body.
#[derive(Debug, Deserialize)]
struct Body {
    person_ref: String,
    from: NaiveDate,
    to: NaiveDate,
    max_percent: i32,
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/availability` — set a person's capacity for a date range.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<Body>,
) -> Result<Response> {
    if !super::governance::valid_ref(&b.person_ref, &["person", "worker"]) {
        return Err(unprocessable("person_ref must be a person:/worker: URN"));
    }
    let mut problems = rules::problems(b.from, b.to, b.max_percent);
    if b.note
        .as_deref()
        .is_some_and(|n| n.len() > crate::validation::MAX_TEXT_LEN)
    {
        problems.push("note exceeds the text cap".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if !may_manage_person(&caller, &b.person_ref) {
        return Err(Error::NotFound);
    }
    if rules::overlaps(&ranges_for(&ctx, &b.person_ref).await?, b.from, b.to) {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "this range overlaps one already set for this person; remove or change that one first",
            ),
        ));
    }
    let row = person_availabilities::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        person_ref: ActiveValue::set(b.person_ref.clone()),
        from_date: ActiveValue::set(b.from),
        to_date: ActiveValue::set(b.to),
        max_percent: ActiveValue::set(b.max_percent),
        note: ActiveValue::set(b.note.clone()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "availability_set",
        caller.actor(),
        Some(
            serde_json::json!({ "person_ref": row.person_ref, "from": row.from_date,
            "to": row.to_date, "max_percent": row.max_percent }),
        ),
    )
    .await
    .ok();
    format::json(view(&row))
}

#[derive(Debug, Deserialize)]
struct ListParams {
    /// Whose ranges; defaults to the caller's own.
    #[serde(default)]
    person_ref: Option<String>,
}

/// `GET /api/availability?person_ref=` — a person's ranges (your own by default).
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<ListParams>,
) -> Result<Response> {
    let person = match p.person_ref {
        Some(r) => r,
        None => caller
            .claims()
            .and_then(|c| Uuid::parse_str(&c.sub).ok())
            .map(|u| format!("person:{u}"))
            .ok_or_else(|| unprocessable("person_ref is required"))?,
    };
    if !may_manage_person(&caller, &person) {
        return Err(Error::NotFound);
    }
    let rows = person_availabilities::Entity::find()
        .filter(person_availabilities::Column::PersonRef.eq(person.as_str()))
        .filter(person_availabilities::Column::DeletedAt.is_null())
        .order_by_asc(person_availabilities::Column::FromDate)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

/// `POST /api/availability/{pid}/delete` (and `DELETE`) — remove a range.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let row = person_availabilities::Entity::find()
        .filter(person_availabilities::Column::Pid.eq(pid))
        .filter(person_availabilities::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .filter(|r| may_manage_person(&caller, &r.person_ref))
        .ok_or(Error::NotFound)?;
    let (row_pid, person) = (row.pid, row.person_ref.clone());
    let mut a: person_availabilities::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row_pid,
        "availability_removed",
        caller.actor(),
        Some(serde_json::json!({ "person_ref": person })),
    )
    .await
    .ok();
    format::empty_json()
}

/// The availability routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/availability", post(create))
        .add("/availability", get(list))
        .add("/availability/{pid}", delete(remove))
        .add("/availability/{pid}/delete", post(remove))
}
