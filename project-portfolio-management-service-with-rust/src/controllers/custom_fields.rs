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
use super::unprocessable;
use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::auth::MaybeAuthUser;
use crate::custom_fields::{self as rules, Def};
use crate::models::_entities::{custom_field_defs, custom_field_values};
use crate::models::audit_logs::Model as AuditModel;

/// Decodes a stored JSON array of strings; anything malformed reads as an empty list rather than failing the request.
fn strings(v: &Value) -> Vec<String> {
    serde_json::from_value(v.clone()).unwrap_or_default()
}

/// Projects a stored definition onto the small [`Def`] the pure validation rules take.
fn def_of(row: &custom_field_defs::Model) -> Def {
    Def {
        key: row.key.clone(),
        field_type: row.field_type.clone(),
        options: strings(&row.options),
    }
}

/// Live (not soft-deleted) definitions for a subject type, oldest first so the declaration order is stable for clients.
async fn live_defs(ctx: &AppContext, applies_to: &str) -> Result<Vec<custom_field_defs::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM custom_field_defs WHERE applies_to = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(applies_to)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// `POST /api/custom-fields` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DefPayload {
    /// Field key, unique among live definitions of the same `applies_to`; validated by `rules::def_problems`.
    key: String,
    /// Human-readable label shown beside the field.
    label: String,
    /// The value type (JSON name `type`), one of the types the rules allow.
    #[serde(rename = "type")]
    field_type: String,
    /// Subject type the field attaches to: `plan` or `task`.
    applies_to: String,
    /// Plan kinds the field is limited to; empty means every kind. Only valid for plan fields.
    #[serde(default)]
    kinds: Vec<String>,
    /// Whether a subject must have a value; an unset required field is reported, not defaulted.
    #[serde(default)]
    required: bool,
    /// Allowed choices for the list-style types; empty for the others.
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
    let mut tx = crate::db::begin(ctx).await?;
    let row: custom_field_defs::Model = sqlx::query_as(
        "INSERT INTO custom_field_defs
             (pid, key, label, field_type, applies_to, kinds, required, options, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(key)
    .bind(label.trim())
    .bind(field_type)
    .bind(applies_to)
    .bind(serde_json::json!(kinds))
    .bind(required)
    .bind(serde_json::json!(options))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "custom_field_declared", actor, None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
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
    /// Optional subject-type filter (`plan` or `task`); absent lists both.
    #[serde(default)]
    applies_to: Option<String>,
}

/// `GET /api/custom-fields` — live definitions.
#[debug_handler]
async fn list_defs(
    State(ctx): State<AppContext>,
    Query(params): Query<ListDefsParams>,
) -> Result<Response> {
    let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM custom_field_defs WHERE deleted_at IS NULL",
    );
    if let Some(a) = params.applies_to {
        q.push(" AND applies_to = ").push_bind(a);
    }
    q.push(" ORDER BY id");
    let rows: Vec<custom_field_defs::Model> =
        q.build_query_as().fetch_all(crate::db::pool(&ctx)).await?;
    format::json(rows)
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
    let id: i64 = sqlx::query_scalar(
        "SELECT id FROM custom_field_defs WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE custom_field_defs SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(chrono::Utc::now())
    .bind(id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        pid,
        "custom_field_withdrawn",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
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

/// Persist prepared values in a transaction (task create and copy write them in the task's own; `PUT
/// /api/custom-fields/values` in its own) or on the pool (the MSPDI batch import, each statement on its own). Per
/// value: find the stored one, then update, delete or insert it.
pub(crate) async fn store_on(
    db: &mut impl crate::db::Session,
    subject_kind: &str,
    subject_pid: Uuid,
    prepared: &[(String, Option<Value>)],
) -> Result<()> {
    for (key, value) in prepared {
        let existing: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM custom_field_values
             WHERE subject_kind = $1 AND subject_pid = $2 AND field_key = $3
             LIMIT 1",
        )
        .bind(subject_kind)
        .bind(subject_pid)
        .bind(key.as_str())
        .fetch_optional(db.ex())
        .await?;
        match (existing, value) {
            (Some(id), Some(v)) => {
                // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
                sqlx::query_scalar::<_, i64>(
                    "UPDATE custom_field_values SET value = $1 WHERE id = $2 RETURNING id",
                )
                .bind(v)
                .bind(id)
                .fetch_one(db.ex())
                .await?;
            }
            (Some(id), None) => {
                sqlx::query("DELETE FROM custom_field_values WHERE id = $1")
                    .bind(id)
                    .execute(db.ex())
                    .await?;
            }
            (None, Some(v)) => {
                sqlx::query(
                    "INSERT INTO custom_field_values (pid, subject_kind, subject_pid, field_key, value)
                     VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(Uuid::new_v4())
                .bind(subject_kind)
                .bind(subject_pid)
                .bind(key.as_str())
                .bind(v)
                .execute(db.ex())
                .await?;
            }
            (None, None) => {}
        }
    }
    Ok(())
}

/// Rejects a subject type other than `plan` or `task` with a `422`.
fn check_kind(applies_to: &str) -> Result<()> {
    if rules::APPLIES_TO.contains(&applies_to) {
        Ok(())
    } else {
        Err(unprocessable("applies_to must be plan or task"))
    }
}

/// The kind label of a plan, or `None` when the plan does not exist or has no kind; used to decide which kind-scoped fields apply.
async fn plan_kind(ctx: &AppContext, plan_pid: Uuid) -> Result<Option<String>> {
    let kind: Option<Option<String>> =
        sqlx::query_scalar("SELECT kind FROM plans WHERE pid = $1 LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?;
    Ok(kind.flatten())
}

/// `PUT /api/custom-fields/values` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ValuesPayload {
    /// Subject type of the values: `plan` or `task`.
    applies_to: String,
    /// Public id of the plan or task the values belong to; resolved through the discussion resolver so the owning plan's read check applies.
    subject_pid: String,
    /// Field key to new value; a JSON `null` clears that key's stored value.
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
    let mut tx = crate::db::begin(&ctx).await?;
    store_on(&mut tx, &p.applies_to, subject.pid, &prepared).await?;
    AuditModel::record_on(
        tx.conn(),
        subject.pid,
        "custom_fields_set",
        caller.actor(),
        Some(serde_json::json!({ "keys": prepared.iter().map(|(k, _)| k).collect::<Vec<_>>() })),
    )
    .await
    .map_err(Error::Model)?;
    tx.commit().await?;
    read_values(&ctx, &caller, &p.applies_to, &p.subject_pid).await
}

/// `GET /api/custom-fields/values?applies_to=&subject_pid=` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ValuesParams {
    /// Subject type to read: `plan` or `task`.
    applies_to: String,
    /// Public id of the plan or task whose values are read.
    subject_pid: String,
}

/// Reads the values of one subject: its stored values (withdrawn definitions included and marked) plus the required fields still unset. The subject resolver enforces the caller's read access first.
async fn read_values(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    applies_to: &str,
    subject_pid: &str,
) -> Result<Response> {
    check_kind(applies_to)?;
    let subject = resolve_subject(ctx, caller, applies_to, subject_pid).await?;
    // No `ORDER BY` on either, as before: a value's definition is the first row with its key in the order the
    // database returns them, which matters only once a withdrawn key has been declared again.
    let stored: Vec<custom_field_values::Model> = sqlx::query_as(
        "SELECT * FROM custom_field_values WHERE subject_kind = $1 AND subject_pid = $2",
    )
    .bind(applies_to)
    .bind(subject.pid)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let all_defs: Vec<custom_field_defs::Model> =
        sqlx::query_as("SELECT * FROM custom_field_defs WHERE applies_to = $1")
            .bind(applies_to)
            .fetch_all(crate::db::pool(ctx))
            .await?;
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
    /// Subject type to search: `plan` or `task`.
    applies_to: String,
    /// Field key to match; must be a live definition.
    key: String,
    /// Value to compare with; for a `multi_list` field a match means the stored list contains it.
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
    let def: custom_field_defs::Model = sqlx::query_as(
        "SELECT * FROM custom_field_defs
         WHERE applies_to = $1 AND key = $2 AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(p.applies_to.as_str())
    .bind(p.key.as_str())
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or_else(|| unprocessable("unknown custom field"))?;
    let d = def_of(&def);
    let rows: Vec<custom_field_values::Model> = sqlx::query_as(
        "SELECT * FROM custom_field_values WHERE subject_kind = $1 AND field_key = $2 LIMIT $3",
    )
    .bind(p.applies_to.as_str())
    .bind(p.key.as_str())
    .bind(i64::try_from(SEARCH_CAP + 1).unwrap_or(i64::MAX))
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    // The query fetched one row beyond the cap precisely so truncation can be reported.
    let truncated = rows.len() as u64 > SEARCH_CAP;
    let mut pids = Vec::new();
    for row in rows
        .iter()
        .take(usize::try_from(SEARCH_CAP).unwrap_or(usize::MAX))
    {
        // Visibility check is per subject: a value on a plan the caller cannot read must not leak.
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
