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
//! inside one transaction that holds an advisory lock on the person (CF-3), so two
//! simultaneous requests for the same person take turns: the second sees the
//! first's range and is refused. (A database exclusion constraint would need the
//! `btree_gist` extension, which not every deployment may install.)

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::availability::{self as rules, Range};
use crate::models::_entities::person_availabilities;
use crate::models::audit_logs::Model as AuditModel;

/// Whether the caller may read or change `person_ref`'s availability and
/// bookings: themselves, a resource manager, an admin — or anyone when
/// authentication is off.
pub(crate) fn may_manage_person(caller: &MaybeAuthUser, person_ref: &str) -> bool {
    caller.holds_or_open(|c| {
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

/// Every person's ranges in **one** query, grouped by person: a view over many
/// people must not do a query per person (the scale test pins this).
pub(crate) async fn all_ranges(
    ctx: &AppContext,
) -> Result<std::collections::HashMap<String, Vec<Range>>> {
    let mut out: std::collections::HashMap<String, Vec<Range>> = std::collections::HashMap::new();
    let rows: Vec<person_availabilities::Model> = sqlx::query_as(
        "SELECT * FROM person_availabilities WHERE deleted_at IS NULL ORDER BY from_date",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?;
    for r in rows {
        out.entry(r.person_ref).or_default().push(Range {
            from: r.from_date,
            to: r.to_date,
            max_percent: r.max_percent,
        });
    }
    Ok(out)
}

/// The caller's own `person:` reference, when their token carries a user id.
pub(crate) fn own_ref(caller: &MaybeAuthUser) -> Option<String> {
    caller
        .claims()
        .and_then(|c| Uuid::parse_str(&c.sub).ok())
        .map(|u| format!("person:{u}"))
}

/// A person's ranges, as pure [`Range`]s.
pub(crate) async fn ranges_for(ctx: &AppContext, person_ref: &str) -> Result<Vec<Range>> {
    ranges_on(&mut crate::db::pool(ctx), person_ref).await
}

/// A person's live ranges, earliest first.
async fn rows_on(
    db: &mut impl crate::db::Session,
    person_ref: &str,
) -> Result<Vec<person_availabilities::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM person_availabilities
         WHERE person_ref = $1 AND deleted_at IS NULL
         ORDER BY from_date",
    )
    .bind(person_ref)
    .fetch_all(db.ex())
    .await?)
}

/// [`ranges_for`] on the pool or a transaction.
async fn ranges_on(db: &mut impl crate::db::Session, person_ref: &str) -> Result<Vec<Range>> {
    Ok(rows_on(db, person_ref)
        .await?
        .into_iter()
        .map(|r| Range {
            from: r.from_date,
            to: r.to_date,
            max_percent: r.max_percent,
        })
        .collect())
}

/// The JSON shape returned for one availability range.
fn view(r: &person_availabilities::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": r.pid, "person_ref": r.person_ref, "from": r.from_date, "to": r.to_date,
        "max_percent": r.max_percent, "note": r.note,
    })
}

/// `POST /api/availability` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Body {
    /// Whose; defaults to the caller's own.
    #[serde(default)]
    person_ref: Option<String>,
    /// First day of the range, inclusive.
    from: NaiveDate,
    /// Last day of the range, inclusive; must not precede `from`.
    to: NaiveDate,
    /// Ceiling on the person's capacity in that range, 0 to 100 percent.
    max_percent: i32,
    /// Optional free-text reason.
    #[serde(default)]
    note: Option<String>,
}

/// The database refused an overlapping range (the exclusion constraint, CF-3): the same answer the
/// service's own check gives. Anything else is an ordinary database error.
fn overlap_or_db_err(e: sqlx::Error) -> Error {
    if e.to_string().contains("person_availabilities_no_overlap") {
        return Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "this range overlaps one already set for this person; remove or change that one first",
            ),
        );
    }
    Error::from(e)
}

/// `POST /api/availability` — set a person's capacity for a date range.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<Body>,
) -> Result<Response> {
    let person_ref = match b.person_ref.clone() {
        Some(r) => r,
        None => own_ref(&caller).ok_or_else(|| unprocessable("person_ref is required"))?,
    };
    if !super::governance::valid_ref(&person_ref, &["person", "worker"]) {
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
    if !may_manage_person(&caller, &person_ref) {
        return Err(Error::NotFound);
    }
    // Check and insert under one lock per person, so two requests cannot both pass the check.
    let mut tx = crate::db::begin(&ctx).await?;
    super::advisory_lock_on(&mut tx, "availability", &person_ref).await?;
    if rules::overlaps(&ranges_on(&mut tx, &person_ref).await?, b.from, b.to) {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new(
                "conflict",
                "this range overlaps one already set for this person; remove or change that one first",
            ),
        ));
    }
    let row: person_availabilities::Model = sqlx::query_as(
        "INSERT INTO person_availabilities (pid, person_ref, from_date, to_date, max_percent, note, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&person_ref)
    .bind(b.from)
    .bind(b.to)
    .bind(b.max_percent)
    .bind(b.note.clone())
    .bind(None::<chrono::DateTime<chrono::FixedOffset>>)
    .fetch_one(tx.conn())
    .await
    .map_err(overlap_or_db_err)?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "availability_set",
        caller.actor(),
        Some(
            serde_json::json!({ "person_ref": row.person_ref, "from": row.from_date,
            "to": row.to_date, "max_percent": row.max_percent }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await.map_err(overlap_or_db_err)?;

    format::json(view(&row))
}

/// `GET /api/availability` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
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
        None => own_ref(&caller).ok_or_else(|| unprocessable("person_ref is required"))?,
    };
    if !may_manage_person(&caller, &person) {
        return Err(Error::NotFound);
    }
    let rows = rows_on(&mut crate::db::pool(&ctx), &person).await?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

/// `POST /api/availability/{pid}/delete` (and `DELETE`) — remove a range.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let row: person_availabilities::Model = sqlx::query_as(
        "SELECT * FROM person_availabilities WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .filter(|r: &person_availabilities::Model| may_manage_person(&caller, &r.person_ref))
    .ok_or(Error::NotFound)?;
    let (row_pid, person) = (row.pid, row.person_ref.clone());
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    sqlx::query_scalar::<_, i64>(
        "UPDATE person_availabilities SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(
        chrono::Utc::now(),
    ))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "availability_removed",
        caller.actor(),
        Some(serde_json::json!({ "person_ref": person })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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
