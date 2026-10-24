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

use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::Deserialize;
use serde_json::{Map, Value};
use uuid::Uuid;

use super::governance::find_item;
use super::{conflict, db_err, unprocessable};
use crate::auth::MaybeAuthUser;
use crate::frameworks::Family;
use crate::inherit::{Source, resolve};
use crate::models::_entities::{framework_versions, frameworks, plan_frameworks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::plans::Model as PlanModel;

/// A submitted record split into the common fields and the family's own.
struct Parsed {
    name: String,
    description: Option<String>,
    plan_pid: Option<String>,
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

async fn using(ctx: &AppContext, pid: Uuid) -> Result<u64> {
    plan_frameworks::Entity::find()
        .filter(plan_frameworks::Column::FrameworkPid.eq(pid))
        .count(&ctx.db)
        .await
        .map_err(db_err)
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
        loco_rs::controller::ErrorDetail::new(
            "forbidden",
            "changing an organisation-wide framework needs access=admin or framework=edit"
                .to_string(),
        ),
    ))
}

/// Append `version` of `row` to the history.
async fn record_version<C: ConnectionTrait>(
    db: &C,
    row: &frameworks::Model,
    by: Option<&str>,
) -> Result<()> {
    framework_versions::ActiveModel {
        framework_pid: ActiveValue::set(row.pid),
        version: ActiveValue::set(row.version),
        name: ActiveValue::set(row.name.clone()),
        description: ActiveValue::set(row.description.clone()),
        body: ActiveValue::set(row.body.clone()),
        created_by: ActiveValue::set(by.map(str::to_string)),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(db_err)?;
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
    let row = frameworks::Entity::find()
        .filter(frameworks::Column::Pid.eq(pid))
        .filter(frameworks::Column::Family.eq(family.token()))
        .filter(frameworks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
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

async fn link_of<C: ConnectionTrait>(
    db: &C,
    plan: Uuid,
    family: Family,
) -> Result<Option<plan_frameworks::Model>> {
    plan_frameworks::Entity::find()
        .filter(plan_frameworks::Column::PlanPid.eq(plan))
        .filter(plan_frameworks::Column::Family.eq(family.token()))
        .one(db)
        .await
        .map_err(db_err)
}

/// Point `plan` at `target` for `family`, replacing any earlier link.
async fn set_link<C: ConnectionTrait>(
    db: &C,
    plan: Uuid,
    family: Family,
    target: Uuid,
    version: i32,
    by: Option<&str>,
) -> Result<()> {
    if let Some(existing) = link_of(db, plan, family).await? {
        let mut a: plan_frameworks::ActiveModel = existing.into();
        a.framework_pid = ActiveValue::set(target);
        a.version = ActiveValue::set(version);
        a.set_by = ActiveValue::set(by.map(str::to_string));
        a.update(db).await.map_err(db_err)?;
    } else {
        plan_frameworks::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan),
            family: ActiveValue::set(family.token().to_string()),
            framework_pid: ActiveValue::set(target),
            version: ActiveValue::set(version),
            set_by: ActiveValue::set(by.map(str::to_string)),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(db_err)?;
    }
    Ok(())
}

fn name_taken(e: &sea_orm::DbErr, family: Family) -> Error {
    if e.to_string().contains("frameworks_name_once") {
        return conflict(&format!(
            "a {} with that name already exists for this owner",
            family.noun()
        ));
    }
    db_err(e.clone())
}

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
    require_shared_right(&caller, owner)?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = frameworks::ActiveModel {
        version: ActiveValue::set(1),
        pid: ActiveValue::set(Uuid::new_v4()),
        family: ActiveValue::set(family.token().to_string()),
        name: ActiveValue::set(p.name),
        description: ActiveValue::set(p.description),
        body: ActiveValue::set(p.body),
        owner_plan_pid: ActiveValue::set(owner),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| name_taken(&e, family))?;
    record_version(&txn, &row, caller.actor()).await?;
    if let Some(plan) = owner {
        set_link(&txn, plan, family, row.pid, row.version, caller.actor()).await?;
    }
    AuditModel::record(
        &txn,
        row.pid,
        "framework_created",
        caller.actor(),
        Some(serde_json::json!({ "family": family.token(), "name": row.name, "owner_plan_pid": owner })),
    ).await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(view(&row, u64::from(owner.is_some())))
}

/// `GET /api/<family>?plan=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    #[serde(default)]
    plan: Option<String>,
}

async fn list(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let mut query = frameworks::Entity::find()
        .filter(frameworks::Column::Family.eq(family.token()))
        .filter(frameworks::Column::DeletedAt.is_null())
        .order_by_asc(frameworks::Column::Id);
    if let Some(p) = q.plan.as_deref() {
        let plan = Uuid::parse_str(p).map_err(|_| unprocessable("plan must be a plan pid"))?;
        query = query.filter(frameworks::Column::OwnerPlanPid.eq(plan));
    }
    let rows = query.all(&ctx.db).await.map_err(db_err)?;
    let owners: Vec<Uuid> = rows.iter().filter_map(|r| r.owner_plan_pid).collect();
    let readable: HashSet<Uuid> = if owners.is_empty() {
        HashSet::new()
    } else {
        super::plan_access::readable_plans(&ctx, &caller, &owners).await
    };
    let mut counts: HashMap<Uuid, u64> = HashMap::new();
    for l in plan_frameworks::Entity::find()
        .filter(plan_frameworks::Column::Family.eq(family.token()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        *counts.entry(l.framework_pid).or_insert(0) += 1;
    }
    format::json(
        rows.iter()
            .filter(|r| r.owner_plan_pid.is_none_or(|o| readable.contains(&o)))
            .map(|r| view(r, counts.get(&r.pid).copied().unwrap_or(0)))
            .collect::<Vec<_>>(),
    )
}

async fn get_one(
    family: Family,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = find_visible(&ctx, &caller, family, &pid).await?;
    format::json(view(&row, using(&ctx, row.pid).await?))
}

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
    let mut a: frameworks::ActiveModel = row.into();
    a.name = ActiveValue::set(p.name);
    a.description = ActiveValue::set(p.description);
    a.body = ActiveValue::set(p.body);
    a.version = ActiveValue::set(previous + 1);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = a.update(&txn).await.map_err(|e| name_taken(&e, family))?;
    record_version(&txn, &row, caller.actor()).await?;
    AuditModel::record(
        &txn,
        row.pid,
        "framework_updated",
        caller.actor(),
        Some(serde_json::json!({ "family": family.token(), "version": row.version, "previous_version": previous })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(db_err)?;
    format::json(view(&row, using(&ctx, row.pid).await?))
}

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
    let mut a: frameworks::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "framework_deleted", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
    let versions = framework_versions::Entity::find()
        .filter(framework_versions::Column::FrameworkPid.eq(row.pid))
        .order_by_desc(framework_versions::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut following: HashMap<i32, u64> = HashMap::new();
    for l in plan_frameworks::Entity::find()
        .filter(plan_frameworks::Column::FrameworkPid.eq(row.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        *following.entry(l.version).or_insert(0) += 1;
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
                && framework_versions::Entity::find()
                    .filter(framework_versions::Column::FrameworkPid.eq(record.pid))
                    .filter(framework_versions::Column::Version.eq(version))
                    .one(&ctx.db)
                    .await
                    .map_err(db_err)?
                    .is_none()
            {
                return Err(unprocessable(&format!(
                    "version {version} does not exist (the current version is {})",
                    record.version
                )));
            }
            let before = link_of(&ctx.db, plan.pid, family)
                .await?
                .map(|l| (l.framework_pid, l.version));
            let txn = ctx.db.begin().await.map_err(db_err)?;
            set_link(&txn, plan.pid, family, record.pid, version, caller.actor()).await?;
            AuditModel::record(
                &txn,
                plan.pid,
                "framework_linked",
                caller.actor(),
                Some(serde_json::json!({ "family": family.token(), "framework_pid": record.pid, "version": version,
                    "before": before.map(|(f, v)| serde_json::json!({ "framework_pid": f, "version": v })) })),
            )
            .await
            .map_err(super::audit_failed)?;
            txn.commit().await.map_err(db_err)?;
        }
        None => {
            if let Some(existing) = link_of(&ctx.db, plan.pid, family).await? {
                let txn = ctx.db.begin().await.map_err(db_err)?;
                existing.delete(&txn).await.map_err(db_err)?;
                AuditModel::record(
                    &txn,
                    plan.pid,
                    "framework_unlinked",
                    caller.actor(),
                    Some(serde_json::json!({ "family": family.token() })),
                )
                .await
                .map_err(super::audit_failed)?;
                txn.commit().await.map_err(db_err)?;
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
    let ancestors = PlanModel::ancestor_pids(&ctx.db, plan)
        .await
        .map_err(Error::Model)?;
    let mut chain = vec![plan];
    chain.extend(&ancestors);
    let links: HashMap<Uuid, (Uuid, i32)> = plan_frameworks::Entity::find()
        .filter(plan_frameworks::Column::Family.eq(family.token()))
        .filter(plan_frameworks::Column::PlanPid.is_in(chain))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
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
    let mut record = match r.value {
        None => None,
        Some(w) => frameworks::Entity::find()
            .filter(frameworks::Column::Pid.eq(w))
            .filter(frameworks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?,
    };
    // Show the version the plan follows, not the record's current one.
    let mut latest_version = None;
    if let (Some(rec), Some(want)) = (record.as_mut(), pinned)
        && want != rec.version
    {
        let old = framework_versions::Entity::find()
            .filter(framework_versions::Column::FrameworkPid.eq(rec.pid))
            .filter(framework_versions::Column::Version.eq(want))
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
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
