//! Frameworks a plan can follow: ways of working and governance frameworks. One implementation for both
//! families ([`crate::frameworks::Family`]); the families differ in their fields and their URLs.
//!
//! For each family (`ways-of-working`, `governance-frameworks`):
//!
//! * `POST/GET /api/<family>`, `GET/PUT/DELETE /api/<family>/{pid}` and a POST delete twin: the records. A
//!   record is organisation-wide, or owned by one plan (`plan_pid` on create).
//! * `GET /api/<family>/{pid}/versions`: the versions of a record. An edit (`PUT`) appends a version and never
//!   changes an earlier one; a plan's link pins the version it follows, and the plan keeps it until it is moved
//!   (a `PUT` of the link again, with or without `version`). Changing an organisation-wide record needs
//!   `access=admin` or `framework=edit`.
//! * `PUT /api/plans/{pid}/<family>`: link a plan to a record (its own, an organisation-wide one, or a
//!   programme's), or unlink with null.
//! * `GET /api/plans/{pid}/<family>`: the framework in force for a plan: its own link, else the nearest
//!   ancestor's (resolved on read, never stored), else none.
//!
//! A record owned by a plan the caller cannot read is invisible to them (404, absent from lists), and an
//! inherited record from such a plan is reported as withheld rather than shown.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use serde::Deserialize;
use serde_json::{Map, Value};
use uuid::Uuid;

use super::governance::find_item;
use super::{conflict, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::frameworks::Family;
use crate::inherit::{Source, resolve};
use crate::models::_entities::{framework_versions, frameworks, plan_frameworks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;

/// A submitted record split into the common fields and the family's own.
struct Parsed {
    /// Record name, unique per family and owner.
    name: String,
    /// Optional free-text description; absent when not given.
    description: Option<String>,
    /// Public id of the owning plan; absent makes the record organisation-wide.
    plan_pid: Option<String>,
    /// The family-specific fields, stored as one JSON object.
    body: Value,
}

/// Split and check a request body.
fn parse(family: Family, input: Value) -> Result<Parsed> {
    let Value::Object(mut fields) = input else {
        return Err(unprocessable("the body must be a JSON object"));
    };
    let Some(Value::String(name)) = fields.remove("name") else {
        return Err(unprocessable("name is required"));
    };
    let take_text = |fields: &mut Map<String, Value>, key: &str| match fields.remove(key) {
        Some(Value::String(s)) => Some(s),
        _ => None,
    };
    let description = take_text(&mut fields, "description")
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty());
    let plan_pid = take_text(&mut fields, "plan_pid");
    let body = crate::frameworks::check(family, &name, description.as_deref(), fields)
        .map_err(|p| unprocessable(&p.join("; ")))?;
    Ok(Parsed {
        name: name.trim().to_string(),
        description,
        plan_pid,
        body,
    })
}

/// Render a record for the API: common columns, the family's own fields
/// flattened in, the `scope` (`plan` or `organization`) and how many plans
/// currently follow it.
fn view(r: &frameworks::Model, plans_using: u64) -> Value {
    let mut out = Map::new();
    out.insert("pid".into(), serde_json::json!(r.pid));
    out.insert("name".into(), serde_json::json!(r.name));
    out.insert("description".into(), serde_json::json!(r.description));
    if let Value::Object(body) = &r.body {
        out.extend(body.clone());
    }
    out.insert("owner_plan_pid".into(), serde_json::json!(r.owner_plan_pid));
    out.insert(
        "scope".into(),
        serde_json::json!(if r.owner_plan_pid.is_some() {
            "plan"
        } else {
            "organization"
        }),
    );
    out.insert("version".into(), serde_json::json!(r.version));
    out.insert("plans_using".into(), serde_json::json!(plans_using));
    out.insert("created_by".into(), serde_json::json!(r.created_by));
    out.insert("created_at".into(), serde_json::json!(r.created_at));
    Value::Object(out)
}

/// Count the plans currently linked to a framework record.
async fn using(ctx: &AppContext, pid: Uuid) -> Result<u64> {
    let n: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM plan_frameworks WHERE framework_pid = $1")
            .bind(pid)
            .fetch_one(crate::db::pool(ctx))
            .await?;
    Ok(u64::try_from(n).unwrap_or(0))
}

/// `403` when `owner` is none (an organisation-wide record) and the caller does not hold the right to change one.
fn require_shared_right(caller: &MaybeAuthUser, owner: Option<Uuid>) -> Result<()> {
    if owner.is_some()
        || crate::auth::area_allows(
            caller,
            crate::auth::SHARED_FRAMEWORK,
            authentication_verifier::Action::Write,
        )
    {
        return Ok(());
    }
    Err(Error::CustomError(
        axum::http::StatusCode::FORBIDDEN,
        axa::error::ErrorDetail::new(
            "forbidden",
            "changing an organisation-wide framework needs access=admin or framework=edit"
                .to_string(),
        ),
    ))
}

/// Append `version` of `row` to the history.
async fn record_version(
    tx: &mut crate::db::Tx,
    row: &frameworks::Model,
    by: Option<&str>,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO framework_versions (framework_pid, version, name, description, body, created_by)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(row.pid)
    .bind(row.version)
    .bind(&row.name)
    .bind(&row.description)
    .bind(&row.body)
    .bind(by)
    .execute(tx.conn())
    .await?;
    Ok(())
}

/// A live record of `family` the caller may see: organisation-wide, or owned by a plan they can read.
async fn find_visible(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    family: Family,
    pid: &str,
) -> Result<frameworks::Model> {
    let pid = super::parse_pid(pid)?;
    let row: frameworks::Model = sqlx::query_as(
        "SELECT * FROM frameworks WHERE pid = $1 AND family = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .bind(family.token())
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)?;
    if let Some(owner) = row.owner_plan_pid
        && !super::plan_access::readable_plans(ctx, caller, &[owner])
            .await
            .contains(&owner)
    {
        return Err(Error::NotFound);
    }
    Ok(row)
}

/// The plan's link for one family, if any. A plan has at most one link per
/// family.
async fn link_of(
    db: &mut impl crate::db::Session,
    plan: Uuid,
    family: Family,
) -> Result<Option<plan_frameworks::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plan_frameworks WHERE plan_pid = $1 AND family = $2 LIMIT 1")
            .bind(plan)
            .bind(family.token())
            .fetch_optional(db.ex())
            .await?,
    )
}

/// Point `plan` at `target` for `family`, replacing any earlier link.
async fn set_link(
    tx: &mut crate::db::Tx,
    plan: Uuid,
    family: Family,
    target: Uuid,
    version: i32,
    by: Option<&str>,
) -> Result<()> {
    if let Some(existing) = link_of(tx, plan, family).await? {
        sqlx::query(
            "UPDATE plan_frameworks SET framework_pid = $1, version = $2, set_by = $3 WHERE id = $4
             RETURNING id",
        )
        .bind(target)
        .bind(version)
        .bind(by)
        .bind(existing.id)
        .fetch_one(tx.conn())
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO plan_frameworks (pid, plan_pid, family, framework_pid, version, set_by)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(Uuid::new_v4())
        .bind(plan)
        .bind(family.token())
        .bind(target)
        .bind(version)
        .bind(by)
        .execute(tx.conn())
        .await?;
    }
    Ok(())
}

/// Map an insert/update failure to `409` when it is the unique-name
/// constraint, otherwise to the generic database error.
fn name_taken(e: sqlx::Error, family: Family) -> Error {
    if e.to_string().contains("frameworks_name_once") {
        return conflict(&format!(
            "a {} with that name already exists for this owner",
            family.noun()
        ));
    }
    Error::from(e)
}

/// `POST /api/<family>` - create a record at version 1. Organisation-wide
/// records need `access=admin` or `framework=edit`; a plan-owned record
/// also links its owner plan to it. Audited, and the first version is
/// stored in the history in the same transaction.
async fn create(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(input): Json<Value>,
) -> Result<Response> {
    let p = parse(family, input)?;
    let owner = match p.plan_pid.as_deref() {
        Some(plan) => Some(find_item(&ctx, plan).await?.pid),
        None => None,
    };
    super::plan_access::require_readable_if_any(&ctx, &caller, owner).await?;
    require_shared_right(&caller, owner)?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: frameworks::Model = sqlx::query_as(
        "INSERT INTO frameworks
             (version, pid, family, name, description, body, owner_plan_pid, created_by, deleted_at)
         VALUES (1, $1, $2, $3, $4, $5, $6, $7, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(family.token())
    .bind(p.name)
    .bind(p.description)
    .bind(p.body)
    .bind(owner)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| name_taken(e, family))?;
    record_version(&mut tx, &row, caller.actor()).await?;
    if let Some(plan) = owner {
        set_link(&mut tx, plan, family, row.pid, row.version, caller.actor()).await?;
    }
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "framework_created",
        caller.actor(),
        Some(serde_json::json!({ "family": family.token(), "name": row.name, "owner_plan_pid": owner })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, u64::from(owner.is_some())))
}

/// `GET /api/<family>?plan=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    #[serde(default)]
    /// Restrict the list to records owned by this plan (a plan public id).
    plan: Option<String>,
}

/// `GET /api/<family>` - the live records the caller may see. Records
/// owned by plans the caller cannot read are left out, not masked.
async fn list(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let plan = match q.plan.as_deref() {
        Some(p) => Some(Uuid::parse_str(p).map_err(|_| unprocessable("plan must be a plan pid"))?),
        None => None,
    };
    let rows: Vec<frameworks::Model> = sqlx::query_as(
        "SELECT * FROM frameworks
         WHERE family = $1 AND deleted_at IS NULL AND ($2::uuid IS NULL OR owner_plan_pid = $2)
         ORDER BY id",
    )
    .bind(family.token())
    .bind(plan)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let owners: Vec<Uuid> = rows.iter().filter_map(|r| r.owner_plan_pid).collect();
    let readable: HashSet<Uuid> = if owners.is_empty() {
        HashSet::new()
    } else {
        super::plan_access::readable_plans(&ctx, &caller, &owners).await
    };
    let mut counts: HashMap<Uuid, u64> = HashMap::new();
    for framework_pid in
        sqlx::query_scalar::<_, Uuid>("SELECT framework_pid FROM plan_frameworks WHERE family = $1")
            .bind(family.token())
            .fetch_all(crate::db::pool(&ctx))
            .await?
    {
        *counts.entry(framework_pid).or_insert(0) += 1;
    }
    format::json(
        rows.iter()
            .filter(|r| r.owner_plan_pid.is_none_or(|o| readable.contains(&o)))
            .map(|r| view(r, counts.get(&r.pid).copied().unwrap_or(0)))
            .collect::<Vec<_>>(),
    )
}

/// `GET /api/<family>/{pid}` - one record the caller may see (`404` if it
/// is hidden or deleted).
async fn get_one(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_visible(&ctx, &caller, family, &pid).await?;
    format::json(view(&row, using(&ctx, row.pid).await?))
}

/// `PUT /api/<family>/{pid}` - replace a record's name, description and
/// fields. Appends a new version; plans that pinned an earlier one keep it.
/// Audited as `framework_updated`.
async fn update(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(input): Json<Value>,
) -> Result<Response> {
    let p = parse(family, input)?;
    let row = find_visible(&ctx, &caller, family, &pid).await?;
    require_shared_right(&caller, row.owner_plan_pid)?;
    // An edit is a new version: the plans that follow this one keep the version they pinned until they move.
    let previous = row.version;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: frameworks::Model = sqlx::query_as(
        "UPDATE frameworks SET name = $1, description = $2, body = $3, version = $4 WHERE id = $5
         RETURNING *",
    )
    .bind(p.name)
    .bind(p.description)
    .bind(p.body)
    .bind(previous + 1)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await
    .map_err(|e| name_taken(e, family))?;
    record_version(&mut tx, &row, caller.actor()).await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "framework_updated",
        caller.actor(),
        Some(serde_json::json!({ "family": family.token(), "version": row.version, "previous_version": previous })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(view(&row, using(&ctx, row.pid).await?))
}

/// `DELETE /api/<family>/{pid}` (and its `POST .../delete` twin) -
/// soft-delete a record. Refused with `409` while any plan still follows it.
async fn remove(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_visible(&ctx, &caller, family, &pid).await?;
    require_shared_right(&caller, row.owner_plan_pid)?;
    let n = using(&ctx, row.pid).await?;
    if n > 0 {
        return Err(conflict(&format!(
            "{n} plan(s) follow this; unlink them first"
        )));
    }
    let pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE frameworks SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::Utc::now().fixed_offset())
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), pid, "framework_deleted", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `GET /api/<family>/{pid}/versions`: every version the record has had, newest first, with how many plans
/// follow each.
async fn list_versions(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_visible(&ctx, &caller, family, &pid).await?;
    let versions: Vec<framework_versions::Model> = sqlx::query_as(
        "SELECT * FROM framework_versions WHERE framework_pid = $1 ORDER BY version DESC",
    )
    .bind(row.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut following: HashMap<i32, u64> = HashMap::new();
    for version in
        sqlx::query_scalar::<_, i32>("SELECT version FROM plan_frameworks WHERE framework_pid = $1")
            .bind(row.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?
    {
        *following.entry(version).or_insert(0) += 1;
    }
    format::json(serde_json::json!({
        "framework_pid": row.pid,
        "current_version": row.version,
        "versions": versions.iter().map(|v| {
            let mut out = Map::new();
            out.insert("version".into(), serde_json::json!(v.version));
            out.insert("current".into(), serde_json::json!(v.version == row.version));
            out.insert("name".into(), serde_json::json!(v.name));
            out.insert("description".into(), serde_json::json!(v.description));
            if let Value::Object(body) = &v.body {
                out.extend(body.clone());
            }
            out.insert("created_by".into(), serde_json::json!(v.created_by));
            out.insert("created_at".into(), serde_json::json!(v.created_at));
            out.insert("plans_following".into(), serde_json::json!(following.get(&v.version).copied().unwrap_or(0)));
            Value::Object(out)
        }).collect::<Vec<_>>(),
    }))
}

/// `PUT /api/plans/{pid}/<family>` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct LinkBody {
    /// The record to follow, or null to follow none of its own (the plan then inherits).
    framework_pid: Option<String>,
    /// The version to follow; the record's current one when absent. Following a record again with no version
    /// moves the plan to its current version.
    #[serde(default)]
    version: Option<i32>,
}

/// `PUT /api/plans/{pid}/<family>` - link a plan to a record (or unlink
/// with a null `framework_pid`), pinning the requested version or the
/// current one. The record must be readable by the caller.
async fn link(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<LinkBody>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    match b.framework_pid {
        Some(target) => {
            let record =
                find_visible(&ctx, &caller, family, &target)
                    .await
                    .map_err(|e| match e {
                        Error::NotFound => unprocessable(&format!(
                            "framework_pid must name a {} you can read",
                            family.noun()
                        )),
                        other => other,
                    })?;
            let version = b.version.unwrap_or(record.version);
            if version != record.version
                && sqlx::query(
                    "SELECT 1 FROM framework_versions WHERE framework_pid = $1 AND version = $2 LIMIT 1",
                )
                .bind(record.pid)
                .bind(version)
                .fetch_optional(crate::db::pool(&ctx))
                .await?
                .is_none()
            {
                return Err(unprocessable(&format!(
                    "version {version} does not exist (the current version is {})",
                    record.version
                )));
            }
            let before = link_of(&mut crate::db::pool(&ctx), plan.pid, family)
                .await?
                .map(|l| (l.framework_pid, l.version));
            let mut tx = crate::db::begin(&ctx).await?;
            set_link(
                &mut tx,
                plan.pid,
                family,
                record.pid,
                version,
                caller.actor(),
            )
            .await?;
            AuditModel::record_on(
                tx.conn(),
                plan.pid,
                "framework_linked",
                caller.actor(),
                Some(serde_json::json!({ "family": family.token(), "framework_pid": record.pid, "version": version,
                    "before": before.map(|(f, v)| serde_json::json!({ "framework_pid": f, "version": v })) })),
            )
            .await
            .map_err(super::audit_failed)?;
            tx.commit().await?;
        }
        None => {
            if let Some(existing) = link_of(&mut crate::db::pool(&ctx), plan.pid, family).await? {
                let mut tx = crate::db::begin(&ctx).await?;
                sqlx::query("DELETE FROM plan_frameworks WHERE id = $1")
                    .bind(existing.id)
                    .execute(tx.conn())
                    .await?;
                AuditModel::record_on(
                    tx.conn(),
                    plan.pid,
                    "framework_unlinked",
                    caller.actor(),
                    Some(serde_json::json!({ "family": family.token() })),
                )
                .await
                .map_err(super::audit_failed)?;
                tx.commit().await?;
            }
        }
    }
    effective(&ctx, &caller, family, plan.pid).await
}

/// What is in force for a plan, before it is turned into an answer.
pub(crate) struct InForce {
    /// Where it came from.
    pub source: Source,
    /// The plan whose link it is, when it is shown.
    pub from_plan: Option<Uuid>,
    /// The record, unless it is withheld (from a plan the caller cannot read) or gone.
    pub record: Option<frameworks::Model>,
    /// The record exists but comes from a plan the caller cannot read.
    pub withheld: bool,
    /// The plan follows a record that has since been deleted.
    pub deleted: bool,
    /// The record's current version, when it differs from the one the plan follows (`record` then shows the
    /// pinned one).
    pub latest_version: Option<i32>,
}

/// Resolve the framework of `family` in force for `plan`: its own link, else the nearest ancestor's, else none.
pub(crate) async fn in_force(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    family: Family,
    plan: Uuid,
) -> Result<InForce> {
    let ancestors = PlanModel::ancestor_pids(ctx, plan).await?;
    let mut chain = vec![plan];
    chain.extend(&ancestors);
    let links: HashMap<Uuid, (Uuid, i32)> = sqlx::query_as::<_, plan_frameworks::Model>(
        "SELECT * FROM plan_frameworks WHERE family = $1 AND plan_pid = ANY($2)",
    )
    .bind(family.token())
    .bind(chain)
    .fetch_all(crate::db::pool(ctx))
    .await?
    .into_iter()
    .map(|l| (l.plan_pid, (l.framework_pid, l.version)))
    .collect();
    let ancestor_links: Vec<(Uuid, Option<Uuid>)> = ancestors
        .iter()
        .map(|p| (*p, links.get(p).map(|l| l.0)))
        .collect();
    let r = resolve(plan, links.get(&plan).map(|l| l.0), &ancestor_links);
    // The version pinned by the link that decided it (the plan's own, or the ancestor's it inherits).
    let pinned = match r.source {
        Source::Own => links.get(&plan).map(|l| l.1),
        _ => r.from_plan.and_then(|p| links.get(&p)).map(|l| l.1),
    };
    let mut record: Option<frameworks::Model> = match r.value {
        None => None,
        Some(w) => {
            sqlx::query_as("SELECT * FROM frameworks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
                .bind(w)
                .fetch_optional(crate::db::pool(ctx))
                .await?
        }
    };
    // Show the version the plan follows, not the record's current one.
    let mut latest_version = None;
    if let (Some(rec), Some(want)) = (record.as_mut(), pinned)
        && want != rec.version
    {
        let old: Option<framework_versions::Model> = sqlx::query_as(
            "SELECT * FROM framework_versions WHERE framework_pid = $1 AND version = $2 LIMIT 1",
        )
        .bind(rec.pid)
        .bind(want)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if let Some(old) = old {
            latest_version = Some(rec.version);
            rec.name = old.name;
            rec.description = old.description;
            rec.body = old.body;
            rec.version = old.version;
        }
    }
    // An inherited record that comes from (or is owned by) a plan the caller cannot read is not shown.
    let mut guarded: Vec<Uuid> = Vec::new();
    if r.source == Source::Inherited {
        guarded.extend(r.from_plan);
    }
    guarded.extend(record.as_ref().and_then(|x| x.owner_plan_pid));
    let readable = if guarded.is_empty() {
        HashSet::new()
    } else {
        super::plan_access::readable_plans(ctx, caller, &guarded).await
    };
    let withheld = guarded.iter().any(|p| !readable.contains(p));
    Ok(InForce {
        source: r.source,
        from_plan: if withheld { None } else { r.from_plan },
        deleted: r.value.is_some() && record.is_none(),
        record: if withheld { None } else { record },
        withheld,
        latest_version,
    })
}

/// The escalation path of the governance framework in force for `plan`, lowest level first; empty when there is
/// none, or it is withheld, or it declares no path.
pub(crate) async fn escalation_path(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
) -> Result<Vec<crate::frameworks::Escalation>> {
    let f = in_force(ctx, caller, Family::Governance, plan).await?;
    let mut path: Vec<crate::frameworks::Escalation> = f
        .record
        .and_then(|r| serde_json::from_value::<crate::frameworks::Governance>(r.body).ok())
        .map(|g| g.escalation_path)
        .unwrap_or_default();
    path.sort_by_key(|e| e.level);
    Ok(path)
}

/// The framework of `family` in force for `plan`, as the answer.
async fn effective(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    family: Family,
    plan: Uuid,
) -> Result<Response> {
    let f = in_force(ctx, caller, family, plan).await?;
    let reason = match (f.source, f.withheld, f.deleted) {
        (_, true, _) => Some("it comes from a plan you cannot read".to_string()),
        (Source::None, _, _) => Some(format!(
            "neither this plan nor any plan above it declares a {}",
            family.noun()
        )),
        (_, _, true) => Some("the record this plan follows has been deleted".to_string()),
        _ => None,
    };
    let plans_using = match &f.record {
        Some(x) => using(ctx, x.pid).await?,
        None => 0,
    };
    let mut out = Map::new();
    out.insert("plan_pid".into(), serde_json::json!(plan));
    out.insert(
        "source".into(),
        serde_json::json!(if f.withheld {
            Source::Inherited
        } else {
            f.source
        }),
    );
    out.insert("from_plan_pid".into(), serde_json::json!(f.from_plan));
    out.insert(
        family.key().into(),
        f.record
            .as_ref()
            .map_or(Value::Null, |x| view(x, plans_using)),
    );
    out.insert("reason".into(), serde_json::json!(reason));
    // Behind: the plan follows an older version than the record now has (CF-4); moving it is a deliberate act.
    out.insert(
        "latest_version".into(),
        serde_json::json!(f.latest_version.or(f.record.as_ref().map(|x| x.version))),
    );
    out.insert(
        "behind".into(),
        serde_json::json!(f.latest_version.is_some()),
    );
    format::json(Value::Object(out))
}

/// `GET /api/plans/{pid}/<family>` - the framework in force for the plan:
/// its own link, else the nearest ancestor's, resolved on read.
async fn get_effective(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    effective(&ctx, &caller, family, plan.pid).await
}

/// Add the eight routes of one family.
fn family_routes(routes: Routes, family: Family, collection: &str, plan_path: &str) -> Routes {
    type S = State<AppContext>;
    type U = MaybeAuthUser;
    let one = format!("/{collection}/{{pid}}");
    routes
        .add(
            &format!("/{collection}"),
            post(move |s: S, u: U, j: Json<Value>| create(family, s, u, j)),
        )
        .add(
            &format!("/{collection}"),
            get(move |s: S, u: U, q: Query<ListParams>| list(family, s, u, q)),
        )
        .add(
            &one,
            get(move |s: S, u: U, p: Path<String>| get_one(family, s, u, p)),
        )
        .add(
            &one,
            put(move |s: S, u: U, p: Path<String>, j: Json<Value>| update(family, s, u, p, j)),
        )
        .add(
            &one,
            delete(move |s: S, u: U, p: Path<String>| remove(family, s, u, p)),
        )
        .add(
            &format!("{one}/delete"),
            post(move |s: S, u: U, p: Path<String>| remove(family, s, u, p)),
        )
        .add(
            &format!("{one}/versions"),
            get(move |s: S, u: U, p: Path<String>| list_versions(family, s, u, p)),
        )
        .add(
            plan_path,
            get(move |s: S, u: U, p: Path<String>| get_effective(family, s, u, p)),
        )
        .add(
            plan_path,
            put(move |s: S, u: U, p: Path<String>, j: Json<LinkBody>| link(family, s, u, p, j)),
        )
}

/// The framework routes, for both families.
#[must_use]
pub fn routes() -> Routes {
    let routes = Routes::new().prefix("/api");
    let routes = family_routes(
        routes,
        Family::WaysOfWorking,
        "ways-of-working",
        "/plans/{pid}/ways-of-working",
    );
    family_routes(
        routes,
        Family::Governance,
        "governance-frameworks",
        "/plans/{pid}/governance-framework",
    )
}
