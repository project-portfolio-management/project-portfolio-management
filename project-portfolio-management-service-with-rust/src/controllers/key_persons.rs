//! The key-person register (EX-11): single points of failure (a person or a role), who would take over and
//! how ready they are, and which people are contingent labour.
//!
//! **Resource managers only** (`access=admin` or `role=resource_manager`, as for availability): anyone else
//! gets 404, as if the register did not exist. The director pack carries **counts only**: how many single
//! points, how many have no successor or are not ready, and the contingent share of each skill pool. No
//! person reference, successor or note reaches the pack.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    key_person_entries as entries, resource_roles, skill_pool_members, skill_pools,
};
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/key-persons", get(list))
        .add("/key-persons", put(upsert))
        .add("/key-persons/contingent", put(set_contingent))
        .add("/key-persons/{pid}", delete(remove))
}

/// A resource manager, or 404.
fn managers_only(caller: &MaybeAuthUser) -> Result<()> {
    if super::portfolio_resources::may_manage_people(caller) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// JSON view of a key-person entry: subject, successor reference, readiness, linked skill pool and note.
fn view(e: &entries::Model) -> Value {
    json!({
        "pid": e.pid, "subject_kind": e.subject_kind, "subject": e.subject,
        "successor_ref": e.successor_ref, "readiness": e.readiness,
        "skill_pool_pid": e.skill_pool_pid, "note": e.note,
    })
}

/// `GET /api/key-persons`
#[debug_handler]
async fn list(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    managers_only(&caller)?;
    let rows: Vec<entries::Model> = sqlx::query_as("SELECT * FROM key_person_entries ORDER BY id")
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let contingent: Vec<String> =
        sqlx::query_scalar("SELECT person_ref FROM contingent_workers ORDER BY id")
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(
        json!({ "entries": rows.iter().map(view).collect::<Vec<_>>(), "contingent": contingent }),
    )
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `PUT /api/key-persons`: one single-point-of-failure record.
pub(crate) struct Entry {
    /// `person` or `role`.
    subject_kind: String,
    /// A person reference (for `person`) or a role name (for `role`).
    subject: String,
    #[serde(default)]
    /// Person reference of the named successor; absent means no successor named, which is not the same as a successor not being ready.
    successor_ref: Option<String>,
    /// `ready`, `partly` or `none`.
    readiness: String,
    #[serde(default)]
    /// Skill pool that could cover the subject; absent means none linked.
    skill_pool_pid: Option<Uuid>,
    #[serde(default)]
    /// Free-text note; absent means none.
    note: Option<String>,
}

/// `PUT /api/key-persons` — record (or update) a single point of failure.
#[debug_handler]
async fn upsert(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<Entry>,
) -> Result<Response> {
    managers_only(&caller)?;
    let mut problems = Vec::new();
    if !["person", "role"].contains(&b.subject_kind.as_str()) {
        problems.push("subject_kind must be person or role");
    }
    if !["ready", "partly", "none"].contains(&b.readiness.as_str()) {
        problems.push("readiness must be ready, partly or none");
    }
    let subject = b.subject.trim().to_string();
    if !(1..=200).contains(&subject.chars().count()) {
        problems.push("subject must be 1 to 200 characters");
    }
    if b.subject_kind == "person" && !super::governance::valid_ref(&subject, &["person", "worker"])
    {
        problems.push("a person subject must be a person:/worker: URN");
    }
    if let Some(s) = &b.successor_ref
        && !super::governance::valid_ref(s, &["person", "worker"])
    {
        problems.push("successor_ref must be a person:/worker: URN");
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if let Some(p) = b.skill_pool_pid
        && sqlx::query("SELECT 1 FROM skill_pools WHERE pid = $1 LIMIT 1")
            .bind(p)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .is_none()
    {
        return Err(unprocessable("skill_pool_pid is not a skill pool"));
    }
    let existing: Option<entries::Model> = sqlx::query_as(
        "SELECT * FROM key_person_entries
         WHERE subject_kind = $1 AND lower(btrim(subject)) = lower($2)
         LIMIT 1",
    )
    .bind(&b.subject_kind)
    .bind(&subject)
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    let note = b
        .note
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());
    let mut tx = crate::db::begin(&ctx).await?;
    let row: entries::Model = if let Some(e) = existing {
        sqlx::query_as(
            "UPDATE key_person_entries
             SET successor_ref = $1, readiness = $2, skill_pool_pid = $3, note = $4
             WHERE id = $5
             RETURNING *",
        )
        .bind(&b.successor_ref)
        .bind(&b.readiness)
        .bind(b.skill_pool_pid)
        .bind(note)
        .bind(e.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as(
            "INSERT INTO key_person_entries
                 (pid, subject_kind, subject, successor_ref, readiness, skill_pool_pid, note)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(&b.subject_kind)
        .bind(subject)
        .bind(&b.successor_ref)
        .bind(&b.readiness)
        .bind(b.skill_pool_pid)
        .bind(note)
        .fetch_one(tx.conn())
        .await?
    };
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "key_person_recorded",
        caller.actor(),
        Some(json!({ "kind": row.subject_kind, "readiness": row.readiness })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row))
}

/// `DELETE /api/key-persons/{pid}` — no longer a single point of failure.
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    managers_only(&caller)?;
    let pid = super::parse_pid(&pid)?;
    let row_id: i64 =
        sqlx::query_scalar("SELECT id FROM key_person_entries WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("DELETE FROM key_person_entries WHERE id = $1")
        .bind(row_id)
        .execute(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), pid, "key_person_removed", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "removed": pid }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Body of `PUT /api/key-persons/contingent`.
pub(crate) struct Contingent {
    /// Person or worker reference to mark.
    person_ref: String,
    /// True marks the person as contingent labour, false removes the mark.
    contingent: bool,
}

/// `PUT /api/key-persons/contingent` — mark a person as contingent labour, or not.
#[debug_handler]
async fn set_contingent(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<Contingent>,
) -> Result<Response> {
    managers_only(&caller)?;
    if !super::governance::valid_ref(&b.person_ref, &["person", "worker"]) {
        return Err(unprocessable("person_ref must be a person:/worker: URN"));
    }
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM contingent_workers WHERE person_ref = $1 LIMIT 1")
            .bind(&b.person_ref)
            .fetch_optional(crate::db::pool(&ctx))
            .await?;
    let mut tx = crate::db::begin(&ctx).await?;
    match (existing, b.contingent) {
        (None, true) => {
            sqlx::query("INSERT INTO contingent_workers (pid, person_ref) VALUES ($1, $2)")
                .bind(Uuid::new_v4())
                .bind(&b.person_ref)
                .execute(tx.conn())
                .await?;
        }
        (Some(id), false) => {
            sqlx::query("DELETE FROM contingent_workers WHERE id = $1")
                .bind(id)
                .execute(tx.conn())
                .await?;
        }
        _ => {}
    }
    AuditModel::record_on(
        tx.conn(),
        Uuid::new_v4(),
        "contingent_worker_set",
        caller.actor(),
        Some(json!({ "contingent": b.contingent })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({ "person_ref": b.person_ref, "contingent": b.contingent }))
}

/// The pack's section: counts, never a reference or a name. The contingent share of a skill pool is of the
/// people whose primary role is one of the pool's roles; a pool with nobody in it has no share.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext) -> Result<Value> {
    let rows: Vec<entries::Model> = sqlx::query_as("SELECT * FROM key_person_entries")
        .fetch_all(crate::db::pool(ctx))
        .await?;
    let count = |f: &dyn Fn(&entries::Model) -> bool| rows.iter().filter(|r| f(r)).count();
    let pools: Vec<skill_pools::Model> =
        sqlx::query_as("SELECT * FROM skill_pools WHERE active = true ORDER BY name")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let members: Vec<skill_pool_members::Model> =
        sqlx::query_as("SELECT * FROM skill_pool_members WHERE kind = 'role'")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let roles: Vec<resource_roles::Model> = sqlx::query_as("SELECT * FROM resource_roles")
        .fetch_all(crate::db::pool(ctx))
        .await?;
    let contingent: HashSet<String> =
        sqlx::query_scalar::<_, String>("SELECT person_ref FROM contingent_workers")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .collect();
    let pool_roles: HashMap<Uuid, HashSet<String>> =
        members.iter().fold(HashMap::new(), |mut m, r| {
            m.entry(r.pool_pid)
                .or_default()
                .insert(r.value.to_lowercase());
            m
        });
    let shares: Vec<Value> = pools
        .iter()
        .map(|p| {
            let rs = pool_roles.get(&p.pid);
            let people: Vec<&resource_roles::Model> = roles
                .iter()
                .filter(|r| rs.is_some_and(|s| s.contains(&r.role.to_lowercase())))
                .collect();
            let c = people.iter().filter(|r| contingent.contains(&r.person_ref)).count();
            json!({
                "pool": p.name, "headcount": people.len(), "contingent": c,
                "contingent_share_bp": (!people.is_empty()).then(|| (c * 10_000 + people.len() / 2) / people.len()),
            })
        })
        .collect();
    Ok(json!({
        "single_points": rows.len(),
        "people": count(&|r| r.subject_kind == "person"),
        "roles": count(&|r| r.subject_kind == "role"),
        "without_successor": count(&|r| r.successor_ref.is_none()),
        "readiness": {
            "ready": count(&|r| r.readiness == "ready"),
            "partly": count(&|r| r.readiness == "partly"),
            "none": count(&|r| r.readiness == "none"),
        },
        "contingent_share_by_pool": shares,
        "note": "Counts only. The register itself, with its names, successors and notes, is for resource managers. A pool with nobody whose primary role is one of its roles has no share.",
    }))
}
