//! Typed custom fields (RM-1): admin-declared definitions, per-subject
//! values validated against them, and an equality search. Pure rules in
//! [`crate::custom_fields`].
//!
//! Values live in `custom_field_values`, never in the plan payload, so they
//! **cannot reach the matcher**. A definition is soft-deleted, never
//! dropped: values already stored stay readable (marked `withdrawn`).
//! Plan and task subjects reuse the discussion module's resolver, so the
//! owning plan's record-level read check applies to every read and write.

use super::conflict;
use super::{db_err, unprocessable};
use std::collections::BTreeMap;

use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, QuerySelect, TransactionTrait};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::MaybeAuthUser;
use crate::custom_fields::{self as rules, Def};
use crate::models::_entities::{custom_field_defs, custom_field_values, plans};
use crate::models::audit_logs::Model as AuditModel;

fn strings(v: &Value) -> Vec<String> {
    serde_json::from_value(v.clone()).unwrap_or_default()
}

fn def_of(row: &custom_field_defs::Model) -> Def {
    Def {
        key: row.key.clone(),
        field_type: row.field_type.clone(),
        options: strings(&row.options),
    }
}

async fn live_defs(ctx: &AppContext, applies_to: &str) -> Result<Vec<custom_field_defs::Model>> {
    custom_field_defs::Entity::find()
        .filter(custom_field_defs::Column::AppliesTo.eq(applies_to))
        .filter(custom_field_defs::Column::DeletedAt.is_null())
        .order_by_asc(custom_field_defs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// `POST /api/custom-fields` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DefPayload {
    key: String,
    label: String,
    #[serde(rename = "type")]
    field_type: String,
    applies_to: String,
    #[serde(default)]
    kinds: Vec<String>,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    options: Vec<String>,
}

/// `POST /api/custom-fields` — declare a field.
#[debug_handler]
async fn create_def(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(p): Json<DefPayload>,
) -> Result<Response> {
    let row = declare(
        &ctx,
        caller.actor(),
        &p.key,
        &p.label,
        &p.field_type,
        &p.applies_to,
        &p.kinds,
        p.required,
        &p.options,
    )
    .await?;
    format::json(row)
}

/// Declare a custom field: validated, refused when the key is taken for that subject type, audited. The route and
/// the MSPDI batch import both come through here.
#[expect(clippy::too_many_arguments)] // the definition's own fields, as the route receives them
pub(crate) async fn declare(
    ctx: &AppContext,
    actor: Option<&str>,
    key: &str,
    label: &str,
    field_type: &str,
    applies_to: &str,
    kinds: &[String],
    required: bool,
    options: &[String],
) -> Result<custom_field_defs::Model> {
    let problems = rules::def_problems(key, label, field_type, applies_to, kinds, options);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    if live_defs(ctx, applies_to)
        .await?
        .iter()
        .any(|d| d.key == key)
    {
        return Err(conflict(
            "a field with this key already exists for this subject type",
        ));
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = custom_field_defs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        key: ActiveValue::set(key.to_string()),
        label: ActiveValue::set(label.trim().to_string()),
        field_type: ActiveValue::set(field_type.to_string()),
        applies_to: ActiveValue::set(applies_to.to_string()),
        kinds: ActiveValue::set(serde_json::json!(kinds)),
        required: ActiveValue::set(required),
        options: ActiveValue::set(serde_json::json!(options)),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(&txn, row.pid, "custom_field_declared", actor, None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    Ok(row)
}

/// Whether a live definition with this key exists for the subject type.
pub(crate) async fn is_declared(ctx: &AppContext, applies_to: &str, key: &str) -> Result<bool> {
    Ok(live_defs(ctx, applies_to)
        .await?
        .iter()
        .any(|d| d.key == key))
}

/// `GET /api/custom-fields?applies_to=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListDefsParams {
    #[serde(default)]
    applies_to: Option<String>,
}

/// `GET /api/custom-fields` — live definitions.
#[debug_handler]
async fn list_defs(
    State(ctx): State<AppContext>,
    Query(params): Query<ListDefsParams>,
) -> Result<Response> {
    let mut q =
        custom_field_defs::Entity::find().filter(custom_field_defs::Column::DeletedAt.is_null());
    if let Some(a) = params.applies_to {
        q = q.filter(custom_field_defs::Column::AppliesTo.eq(a));
    }
    format::json(
        q.order_by_asc(custom_field_defs::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?,
    )
}

/// `DELETE /api/custom-fields/{pid}` — withdraw a definition. Stored values
/// are kept and stay readable.
#[debug_handler]
async fn delete_def(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = super::parse_pid(&pid)?;
    let row = custom_field_defs::Entity::find()
        .filter(custom_field_defs::Column::Pid.eq(pid))
        .filter(custom_field_defs::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut active: custom_field_defs::ActiveModel = row.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    active.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "custom_field_withdrawn", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::empty_json()
}

/// Validate `values` for a subject: each key a live definition that applies
/// to the subject's kind, each value valid for its type, a `null` clearing
/// (refused for a required field). When `require_all` (a create), every
/// required definition must be present. Returns `(key, Some(value))` to
/// set and `(key, None)` to clear.
pub(crate) async fn prepare(
    ctx: &AppContext,
    applies_to: &str,
    plan_kind: Option<&str>,
    values: &BTreeMap<String, Value>,
    require_all: bool,
) -> Result<Vec<(String, Option<Value>)>> {
    let defs = live_defs(ctx, applies_to).await?;
    let mut problems = Vec::new();
    let mut out = Vec::new();
    for (key, value) in values {
        let Some(def) = defs.iter().find(|d| &d.key == key) else {
            problems.push(format!("unknown custom field {key:?}"));
            continue;
        };
        if applies_to == "plan" && !rules::kind_applies(&strings(&def.kinds), plan_kind) {
            problems.push(format!("{key}: not applicable to a plan of this kind"));
            continue;
        }
        if value.is_null() {
            if def.required {
                problems.push(format!("{key}: is required and cannot be cleared"));
            } else {
                out.push((key.clone(), None));
            }
            continue;
        }
        match rules::validate_value(&def_of(def), value) {
            Ok(v) => out.push((key.clone(), Some(v))),
            Err(e) => problems.push(e),
        }
    }
    if require_all {
        for def in defs.iter().filter(|d| d.required) {
            let applies =
                applies_to != "plan" || rules::kind_applies(&strings(&def.kinds), plan_kind);
            if applies && values.get(&def.key).is_none_or(Value::is_null) {
                problems.push(format!("{}: is required", def.key));
            }
        }
    }
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(unprocessable(&problems.join("; ")))
    }
}

/// Persist prepared values on any connection (so task create can do it in
/// its own transaction).
pub(crate) async fn store<C: ConnectionTrait>(
    db: &C,
    subject_kind: &str,
    subject_pid: Uuid,
    prepared: &[(String, Option<Value>)],
) -> Result<()> {
    for (key, value) in prepared {
        let existing = custom_field_values::Entity::find()
            .filter(custom_field_values::Column::SubjectKind.eq(subject_kind))
            .filter(custom_field_values::Column::SubjectPid.eq(subject_pid))
            .filter(custom_field_values::Column::FieldKey.eq(key.as_str()))
            .one(db)
            .await
            .map_err(db_err)?;
        match (existing, value) {
            (Some(row), Some(v)) => {
                let mut a: custom_field_values::ActiveModel = row.into();
                a.value = ActiveValue::set(v.clone());
                a.update(db).await.map_err(db_err)?;
            }
            (Some(row), None) => {
                custom_field_values::Entity::delete_by_id(row.id)
                    .exec(db)
                    .await
                    .map_err(db_err)?;
            }
            (None, Some(v)) => {
                custom_field_values::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    subject_kind: ActiveValue::set(subject_kind.to_string()),
                    subject_pid: ActiveValue::set(subject_pid),
                    field_key: ActiveValue::set(key.clone()),
                    value: ActiveValue::set(v.clone()),
                    ..Default::default()
                }
                .insert(db)
                .await
                .map_err(db_err)?;
            }
            (None, None) => {}
        }
    }
    Ok(())
}

fn check_kind(applies_to: &str) -> Result<()> {
    if rules::APPLIES_TO.contains(&applies_to) {
        Ok(())
    } else {
        Err(unprocessable("applies_to must be plan or task"))
    }
}

async fn plan_kind(ctx: &AppContext, plan_pid: Uuid) -> Result<Option<String>> {
    Ok(plans::Entity::find()
        .filter(plans::Column::Pid.eq(plan_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .and_then(|p| p.kind))
}

/// `PUT /api/custom-fields/values` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ValuesPayload {
    applies_to: String,
    subject_pid: String,
    values: BTreeMap<String, Value>,
}

/// `PUT /api/custom-fields/values` — set (or, with `null`, clear) values.
/// Atomic: any invalid value refuses the whole call.
#[debug_handler]
async fn put_values(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(p): Json<ValuesPayload>,
) -> Result<Response> {
    check_kind(&p.applies_to)?;
    let subject = resolve_subject(&ctx, &caller, &p.applies_to, &p.subject_pid).await?;
    let kind = match (p.applies_to.as_str(), subject.plan_pid) {
        ("plan", _) => plan_kind(&ctx, subject.pid).await?,
        _ => None,
    };
    let prepared = prepare(&ctx, &p.applies_to, kind.as_deref(), &p.values, false).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    store(&txn, &p.applies_to, subject.pid, &prepared).await?;
    AuditModel::record(
        &txn,
        subject.pid,
        "custom_fields_set",
        caller.actor(),
        Some(serde_json::json!({ "keys": prepared.iter().map(|(k, _)| k).collect::<Vec<_>>() })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    read_values(&ctx, &caller, &p.applies_to, &p.subject_pid).await
}

/// `GET /api/custom-fields/values?applies_to=&subject_pid=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ValuesParams {
    applies_to: String,
    subject_pid: String,
}

async fn read_values(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    applies_to: &str,
    subject_pid: &str,
) -> Result<Response> {
    check_kind(applies_to)?;
    let subject = resolve_subject(ctx, caller, applies_to, subject_pid).await?;
    let stored = custom_field_values::Entity::find()
        .filter(custom_field_values::Column::SubjectKind.eq(applies_to))
        .filter(custom_field_values::Column::SubjectPid.eq(subject.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let all_defs = custom_field_defs::Entity::find()
        .filter(custom_field_defs::Column::AppliesTo.eq(applies_to))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let kind = if applies_to == "plan" {
        plan_kind(ctx, subject.pid).await?
    } else {
        None
    };
    let mut values = serde_json::Map::new();
    for v in &stored {
        let def = all_defs.iter().find(|d| d.key == v.field_key);
        values.insert(
            v.field_key.clone(),
            serde_json::json!({
                "value": v.value,
                "label": def.map(|d| d.label.clone()),
                // A definition withdrawn after the value was stored: the
                // value stays readable, and says so.
                "definition": if def.is_some_and(|d| d.deleted_at.is_none()) { "live" } else { "withdrawn" },
            }),
        );
    }
    let missing_required: Vec<&str> = all_defs
        .iter()
        .filter(|d| d.deleted_at.is_none() && d.required)
        .filter(|d| {
            applies_to != "plan" || rules::kind_applies(&strings(&d.kinds), kind.as_deref())
        })
        .filter(|d| !stored.iter().any(|v| v.field_key == d.key))
        .map(|d| d.key.as_str())
        .collect();
    format::json(serde_json::json!({
        "subject_pid": subject.pid,
        "values": values,
        "missing_required": missing_required,
    }))
}

/// `GET /api/custom-fields/values` — a subject's values, withdrawn
/// definitions included, and the required fields still unset.
#[debug_handler]
async fn get_values(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<ValuesParams>,
) -> Result<Response> {
    read_values(&ctx, &caller, &p.applies_to, &p.subject_pid).await
}

/// `GET /api/custom-fields/search?applies_to=&key=&value=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchParams {
    applies_to: String,
    key: String,
    value: String,
}

/// Rows scanned per search; a larger result is reported as truncated.
const SEARCH_CAP: u64 = 5_000;

/// `GET /api/custom-fields/search` — the subjects whose `key` equals
/// `value` (`multi_list`: contains it). Subjects whose owning plan the caller
/// cannot read are omitted.
#[debug_handler]
async fn search(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(p): Query<SearchParams>,
) -> Result<Response> {
    check_kind(&p.applies_to)?;
    let def = custom_field_defs::Entity::find()
        .filter(custom_field_defs::Column::AppliesTo.eq(p.applies_to.as_str()))
        .filter(custom_field_defs::Column::Key.eq(p.key.as_str()))
        .filter(custom_field_defs::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or_else(|| unprocessable("unknown custom field"))?;
    let d = def_of(&def);
    let rows = custom_field_values::Entity::find()
        .filter(custom_field_values::Column::SubjectKind.eq(p.applies_to.as_str()))
        .filter(custom_field_values::Column::FieldKey.eq(p.key.as_str()))
        .limit(SEARCH_CAP + 1)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let truncated = rows.len() as u64 > SEARCH_CAP;
    let mut pids = Vec::new();
    for row in rows
        .iter()
        .take(usize::try_from(SEARCH_CAP).unwrap_or(usize::MAX))
    {
        if rules::matches(&d, &row.value, &p.value)
            && resolve_subject(&ctx, &caller, &p.applies_to, &row.subject_pid.to_string())
                .await
                .is_ok()
        {
            pids.push(row.subject_pid);
        }
    }
    pids.sort();
    format::json(serde_json::json!({ "count": pids.len(), "pids": pids, "truncated": truncated }))
}

/// The custom-field routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/custom-fields", post(create_def))
        .add("/custom-fields", get(list_defs))
        .add("/custom-fields/values", put(put_values))
        .add("/custom-fields/values", get(get_values))
        .add("/custom-fields/search", get(search))
        .add("/custom-fields/{pid}", delete(delete_def))
}
