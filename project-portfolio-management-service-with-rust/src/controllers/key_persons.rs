//! The key-person register (EX-11): single points of failure (a person or a role), who would take over and
//! how ready they are, and which people are contingent labour.
//!
//! **Resource managers only** (`access=admin` or `role=resource_manager`, as for availability): anyone else
//! gets 404, as if the register did not exist. The director pack carries **counts only**: how many single
//! points, how many have no successor or are not ready, and the contingent share of each skill pool. No
//! person reference, successor or note reaches the pack.

use std::collections::{HashMap, HashSet};

use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    contingent_workers, key_person_entries as entries, resource_roles, skill_pool_members,
    skill_pools,
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
    let rows = entries::Entity::find()
        .order_by_asc(entries::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let contingent: Vec<String> = contingent_workers::Entity::find()
        .order_by_asc(contingent_workers::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|c| c.person_ref)
        .collect();
    format::json(
        json!({ "entries": rows.iter().map(view).collect::<Vec<_>>(), "contingent": contingent }),
    )
}

#[derive(Debug, Deserialize)]
struct Entry {
    /// `person` or `role`.
    subject_kind: String,
    /// A person reference (for `person`) or a role name (for `role`).
    subject: String,
    #[serde(default)]
    successor_ref: Option<String>,
    /// `ready`, `partly` or `none`.
    readiness: String,
    #[serde(default)]
    skill_pool_pid: Option<Uuid>,
    #[serde(default)]
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
        && skill_pools::Entity::find()
            .filter(skill_pools::Column::Pid.eq(p))
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .is_none()
    {
        return Err(unprocessable("skill_pool_pid is not a skill pool"));
    }
    let existing = entries::Entity::find()
        .filter(entries::Column::SubjectKind.eq(&b.subject_kind))
        .filter(sea_orm::sea_query::Expr::cust_with_values(
            "lower(btrim(subject)) = lower($1)",
            [subject.clone()],
        ))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let note = b
        .note
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());
    let row = if let Some(e) = existing {
        let mut a: entries::ActiveModel = e.into();
        a.successor_ref = ActiveValue::set(b.successor_ref.clone());
        a.readiness = ActiveValue::set(b.readiness.clone());
        a.skill_pool_pid = ActiveValue::set(b.skill_pool_pid);
        a.note = ActiveValue::set(note);
        a.update(&ctx.db).await.map_err(db_err)?
    } else {
        entries::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            subject_kind: ActiveValue::set(b.subject_kind.clone()),
            subject: ActiveValue::set(subject),
            successor_ref: ActiveValue::set(b.successor_ref.clone()),
            readiness: ActiveValue::set(b.readiness.clone()),
            skill_pool_pid: ActiveValue::set(b.skill_pool_pid),
            note: ActiveValue::set(note),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?
    };
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "key_person_recorded",
        caller.actor(),
        Some(json!({ "kind": row.subject_kind, "readiness": row.readiness })),
    )
    .await;
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
    let row = entries::Entity::find()
        .filter(entries::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    row.delete(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "key_person_removed", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(json!({ "removed": pid }))
}

#[derive(Debug, Deserialize)]
struct Contingent {
    person_ref: String,
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
    let existing = contingent_workers::Entity::find()
        .filter(contingent_workers::Column::PersonRef.eq(&b.person_ref))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    match (existing, b.contingent) {
        (None, true) => {
            contingent_workers::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                person_ref: ActiveValue::set(b.person_ref.clone()),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
        (Some(e), false) => {
            e.delete(&ctx.db).await.map_err(db_err)?;
        }
        _ => {}
    }
    AuditModel::record_or_log(
        &ctx.db,
        Uuid::new_v4(),
        "contingent_worker_set",
        caller.actor(),
        Some(json!({ "contingent": b.contingent })),
    )
    .await;
    format::json(json!({ "person_ref": b.person_ref, "contingent": b.contingent }))
}

/// The pack's section: counts, never a reference or a name. The contingent share of a skill pool is of the
/// people whose primary role is one of the pool's roles; a pool with nobody in it has no share.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext) -> Result<Value> {
    let rows = entries::Entity::find().all(&ctx.db).await.map_err(db_err)?;
    let count = |f: &dyn Fn(&entries::Model) -> bool| rows.iter().filter(|r| f(r)).count();
    let pools = skill_pools::Entity::find()
        .filter(skill_pools::Column::Active.eq(true))
        .order_by_asc(skill_pools::Column::Name)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let members = skill_pool_members::Entity::find()
        .filter(skill_pool_members::Column::Kind.eq("role"))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let roles: Vec<resource_roles::Model> = resource_roles::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let contingent: HashSet<String> = contingent_workers::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|c| c.person_ref)
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
