//! Cost rates, task costs and labour cost (PJ-2). The maths is
//! [`crate::cost_rates`]; this is the wiring.
//!
//! * `POST/GET /api/cost-rate-tables`, `POST …/{pid}/rates`,
//!   `GET /api/cost-rates/lookup` — named rate tables per person or role, each
//!   rate **effective from a date** and append-only;
//! * `POST/GET/DELETE …/tasks/{pid}/costs` — material and fixed-cost expenses with
//!   an accrual rule;
//! * `GET …/cost-plan` — the planned cost by period: labour (the work profile, PJ-5,
//!   priced at the rate in force on each day) plus accrued expenses, **per
//!   currency**;
//! * `POST …/labour-costs/record`, `GET …/labour-costs` — the cost of recorded
//!   time, **snapshotted with the rate used**, so a later rate change never
//!   rewrites an actual.
//!
//! **Rates are finance data.** Every route here is available only to a caller the
//! policy admits to the `finance` area (`access=admin` or `finance=cost_rates`,
//! see [`crate::auth::service_rules`]; or when authentication is off, the
//! deployment's posture for every write); anyone else gets `404`, and no rate or
//! cost appears in any other response.

use super::conflict;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::prelude::*;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_task as find_task;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cost_rates::{self as rules, Accrual, Rate};
use crate::models::_entities::{
    cost_rate_tables, cost_rates, task_costs, time_entries, time_entry_costs,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::work_profile as wp;

/// Whether the caller may see or change rates and the costs derived from them.
pub(crate) fn may_see_rates(caller: &MaybeAuthUser) -> bool {
    crate::auth::area_allows(caller, "finance", authentication_verifier::Action::Read)
}

/// A route a caller without the finance attribute must not know exists.
fn finance_only(caller: &MaybeAuthUser) -> Result<()> {
    if may_see_rates(caller) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// The rates that apply to a person, from their own table or their role's.
struct Applicable {
    /// Public id of the rate table the rates came from.
    table_pid: Uuid,
    /// ISO currency code of the table; every rate in it is in this currency.
    currency: String,
    /// Where the table was found: `person` (their own table) or `role` (the
    /// fallback via their primary role).
    source: &'static str,
    /// The table's rates, oldest effective date first, as the pure rules expect.
    rates: Vec<Rate>,
}

/// Load a table's rates ordered by effective date and package them with the
/// table's id, currency and provenance (`source`).
async fn rates_of(
    ctx: &AppContext,
    table: &cost_rate_tables::Model,
    source: &'static str,
) -> Result<Applicable> {
    // Only the two columns the rules use are read; the order is `SeaORM`'s (effective date, no tie-break).
    let rows: Vec<(NaiveDate, i64)> = sqlx::query_as(
        "SELECT effective_from, per_hour_minor FROM cost_rates WHERE table_pid = $1 ORDER BY effective_from",
    )
    .bind(table.pid)
    .fetch_all(crate::db::pool(ctx))
    .await?;
    Ok(Applicable {
        table_pid: table.pid,
        currency: table.currency.clone(),
        source,
        rates: rows
            .into_iter()
            .map(|(effective_from, per_hour_minor)| Rate {
                effective_from,
                per_hour_minor,
            })
            .collect(),
    })
}

/// The rate table for a person: their own named table, else their primary role's.
async fn applicable(ctx: &AppContext, person: &str, name: &str) -> Result<Option<Applicable>> {
    if let Some(t) = sqlx::query_as::<_, cost_rate_tables::Model>(
        "SELECT * FROM cost_rate_tables WHERE name = $1 AND person_ref = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(name)
    .bind(person)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    {
        return Ok(Some(rates_of(ctx, &t, "person").await?));
    }
    // Only the role is read.
    let role: Option<String> =
        sqlx::query_scalar("SELECT role FROM resource_roles WHERE person_ref = $1 LIMIT 1")
            .bind(person)
            .fetch_optional(crate::db::pool(ctx))
            .await?;
    let Some(role) = role else {
        return Ok(None);
    };
    match sqlx::query_as::<_, cost_rate_tables::Model>(
        "SELECT * FROM cost_rate_tables WHERE name = $1 AND role = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(name)
    .bind(role)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    {
        Some(t) => Ok(Some(rates_of(ctx, &t, "role").await?)),
        None => Ok(None),
    }
}

/// The live rate table named `name` whose owner column (`person_ref` or `role`, a literal from the callers) is
/// `owner`.
async fn find_live_table(
    ctx: &AppContext,
    name: &str,
    owner_column: &'static str,
    owner: &str,
) -> Result<Option<cost_rate_tables::Model>> {
    let sql = if owner_column == "role" {
        "SELECT * FROM cost_rate_tables WHERE name = $1 AND deleted_at IS NULL AND role = $2 LIMIT 1"
    } else {
        "SELECT * FROM cost_rate_tables WHERE name = $1 AND deleted_at IS NULL AND person_ref = $2 LIMIT 1"
    };
    Ok(sqlx::query_as(sql)
        .bind(name)
        .bind(owner)
        .fetch_optional(crate::db::pool(ctx))
        .await?)
}

/// What an import did with one rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RateOutcome {
    /// A table and/or a rate was written.
    Stored,
    /// The table already has this rate in force on that date.
    AlreadyThere,
    /// The table already has a different rate from that date; rates are append-only, so nothing was changed.
    DiffersFromExisting,
}

/// Store a rate for a person or a role from an import: the named table is created when it is missing; a rate
/// is written unless the table already holds this value (in force on the date) or a different one on that date.
pub(crate) async fn import_rate(
    ctx: &AppContext,
    actor: Option<&str>,
    owner: (Option<&str>, Option<&str>),
    table_name: &str,
    currency: &str,
    effective_from: NaiveDate,
    per_hour_minor: i64,
) -> Result<RateOutcome> {
    let (person_ref, role) = owner;
    let existing = match (person_ref, role) {
        (Some(p), _) => find_live_table(ctx, table_name, "person_ref", p).await?,
        (_, Some(r)) => find_live_table(ctx, table_name, "role", r).await?,
        _ => return Err(unprocessable("name exactly one of person_ref and role")),
    };
    if let Some(t) = &existing {
        // Only the two columns compared are read.
        let in_force: Option<(NaiveDate, i64)> = sqlx::query_as(
            "SELECT effective_from, per_hour_minor FROM cost_rates WHERE table_pid = $1 AND effective_from <= $2
             ORDER BY effective_from DESC LIMIT 1",
        )
        .bind(t.pid)
        .bind(effective_from)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if let Some((in_force_from, in_force_rate)) = in_force {
            return Ok(if in_force_rate == per_hour_minor {
                RateOutcome::AlreadyThere
            } else if in_force_from == effective_from {
                RateOutcome::DiffersFromExisting
            } else {
                // A different rate was in force earlier; this one starts a new period.
                add_import_rate(ctx, actor, t, effective_from, per_hour_minor).await?;
                RateOutcome::Stored
            });
        }
    }
    let table = match existing {
        Some(t) => t,
        None => create_import_table(ctx, actor, (person_ref, role), table_name, currency).await?,
    };
    add_import_rate(ctx, actor, &table, effective_from, per_hour_minor).await?;
    Ok(RateOutcome::Stored)
}

/// Create an empty rate table owned by one person or one role during an
/// MSPDI batch import, auditing the creation as `via: mspdi_batch`.
async fn create_import_table(
    ctx: &AppContext,
    actor: Option<&str>,
    owner: (Option<&str>, Option<&str>),
    table_name: &str,
    currency: &str,
) -> Result<cost_rate_tables::Model> {
    let (person_ref, role) = owner;
    let mut tx = crate::db::begin(ctx).await?;
    let row = insert_table(&mut tx, table_name, person_ref, role, currency).await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "cost_rate_table_created",
        actor,
        Some(
            serde_json::json!({ "name": row.name, "person_ref": row.person_ref,
            "role": row.role, "currency": row.currency, "via": "mspdi_batch" }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    Ok(row)
}

/// Insert a rate table with the columns `SeaORM`'s insert set, and read it back.
async fn insert_table(
    tx: &mut crate::db::Tx,
    name: &str,
    person_ref: Option<&str>,
    role: Option<&str>,
    currency: &str,
) -> Result<cost_rate_tables::Model> {
    Ok(sqlx::query_as(
        "INSERT INTO cost_rate_tables (pid, name, person_ref, role, currency, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(person_ref)
    .bind(role)
    .bind(currency)
    .fetch_one(tx.conn())
    .await?)
}

/// Append one rate with the columns `SeaORM`'s insert set, and read it back.
async fn insert_rate(
    tx: &mut crate::db::Tx,
    table_pid: Uuid,
    effective_from: NaiveDate,
    per_hour_minor: i64,
    created_by: Option<&str>,
) -> Result<cost_rates::Model> {
    Ok(sqlx::query_as(
        "INSERT INTO cost_rates (pid, table_pid, effective_from, per_hour_minor, created_by)
         VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(table_pid)
    .bind(effective_from)
    .bind(per_hour_minor)
    .bind(created_by)
    .fetch_one(tx.conn())
    .await?)
}

/// Insert a task cost with the columns `SeaORM`'s insert set, and read it back.
async fn insert_task_cost(
    tx: &mut crate::db::Tx,
    task_pid: Uuid,
    kind: &str,
    description: &str,
    amount_minor: i64,
    currency: &str,
    accrual: &str,
) -> Result<task_costs::Model> {
    Ok(sqlx::query_as(
        "INSERT INTO task_costs (pid, task_pid, kind, description, amount_minor, currency, accrual, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, NULL) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(task_pid)
    .bind(kind)
    .bind(description)
    .bind(amount_minor)
    .bind(currency)
    .bind(accrual)
    .fetch_one(tx.conn())
    .await?)
}

/// Append one rate to a table during import, in the same transaction as
/// its audit entry. Rates are append-only, so this never edits an old row.
async fn add_import_rate(
    ctx: &AppContext,
    actor: Option<&str>,
    table: &cost_rate_tables::Model,
    effective_from: NaiveDate,
    per_hour_minor: i64,
) -> Result<()> {
    let mut tx = crate::db::begin(ctx).await?;
    let row = insert_rate(&mut tx, table.pid, effective_from, per_hour_minor, actor).await?;
    AuditModel::record_on(
        tx.conn(),
        table.pid,
        "cost_rate_added",
        actor,
        Some(serde_json::json!({ "effective_from": row.effective_from, "per_hour_minor": row.per_hour_minor,
            "currency": table.currency, "via": "mspdi_batch" })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    Ok(())
}

/// Store a task's fixed cost from an import, once: a task that already has a fixed cost with this description
/// keeps it (a re-import converges). Returns whether one was written.
pub(crate) async fn import_fixed_cost(
    ctx: &AppContext,
    actor: Option<&str>,
    task_pid: Uuid,
    description: &str,
    amount_minor: i64,
    currency: &str,
    accrual: &str,
) -> Result<bool> {
    // Only whether one exists is read.
    let present: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM task_costs
         WHERE task_pid = $1 AND kind = 'fixed' AND description = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(task_pid)
    .bind(description)
    .fetch_optional(crate::db::pool(ctx))
    .await?;
    if present.is_some() {
        return Ok(false);
    }
    let mut tx = crate::db::begin(ctx).await?;
    let row = insert_task_cost(
        &mut tx,
        task_pid,
        "fixed",
        description,
        amount_minor,
        currency,
        accrual,
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "task_cost_added",
        actor,
        Some(
            serde_json::json!({ "task_pid": task_pid, "kind": "fixed", "amount_minor": amount_minor,
            "currency": currency, "accrual": accrual, "via": "mspdi_batch" }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    Ok(true)
}

// ───────────────────────── rate tables and rates ─────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `POST /api/cost-rate-tables` body.
pub(crate) struct TableBody {
    /// Table name, trimmed and at most 40 characters; `standard` is the name
    /// lookups use unless they ask for another.
    name: String,
    #[serde(default)]
    /// Reference to the person the table prices; exactly one of `person_ref`
    /// and `role` must be given.
    person_ref: Option<String>,
    #[serde(default)]
    /// Role the table prices, as the fallback for people without their own table.
    role: Option<String>,
    /// ISO currency code for every rate in the table.
    currency: String,
}

/// `POST /api/cost-rate-tables` — a named rate table for one person or one role.
#[debug_handler]
async fn create_table(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<TableBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let name = b.name.trim();
    let mut problems = Vec::new();
    if name.is_empty() || name.chars().count() > 40 {
        problems
            .push("name is required (up to 40 characters), e.g. standard or overtime".to_string());
    }
    match (&b.person_ref, &b.role) {
        (Some(p), None) if !super::governance::valid_ref(p, &["person", "worker"]) => {
            problems.push("person_ref must be a person:/worker: URN".to_string());
        }
        (None, Some(r)) if r.trim().is_empty() || r.chars().count() > 60 => {
            problems.push("role must be 1 to 60 characters".to_string());
        }
        (Some(_), None) | (None, Some(_)) => {}
        _ => problems.push("name exactly one of person_ref and role".to_string()),
    }
    let currency = b.currency.trim().to_ascii_uppercase();
    if !crate::currency::is_code(&currency) {
        problems.push("currency must be a 3-letter code, for example EUR, JPY or GBP".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    // Validation above leaves exactly one of the two owners.
    let existing = match (&b.person_ref, &b.role) {
        (Some(p), _) => find_live_table(&ctx, name, "person_ref", p).await?,
        (_, Some(r)) => find_live_table(&ctx, name, "role", r.trim()).await?,
        _ => None,
    };
    if existing.is_some() {
        return Err(conflict(
            "that person or role already has a table with that name",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row = insert_table(
        &mut tx,
        name,
        b.person_ref.as_deref(),
        b.role.as_deref().map(str::trim),
        &currency,
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "cost_rate_table_created",
        caller.actor(),
        Some(
            serde_json::json!({ "name": row.name, "person_ref": row.person_ref,
            "role": row.role, "currency": row.currency }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `GET /api/cost-rate-tables` query: optional filters on the owner.
pub(crate) struct ListTables {
    #[serde(default)]
    /// Restrict to tables owned by this person.
    person_ref: Option<String>,
    #[serde(default)]
    /// Restrict to tables owned by this role.
    role: Option<String>,
}

/// `GET /api/cost-rate-tables` — the tables with their rates, oldest first.
#[debug_handler]
async fn list_tables(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListTables>,
) -> Result<Response> {
    finance_only(&caller)?;
    let mut query =
        sqlx::QueryBuilder::new("SELECT * FROM cost_rate_tables WHERE deleted_at IS NULL");
    if let Some(p) = &q.person_ref {
        query.push(" AND person_ref = ").push_bind(p.as_str());
    }
    if let Some(r) = &q.role {
        query.push(" AND role = ").push_bind(r.as_str());
    }
    query.push(" ORDER BY id");
    let tables: Vec<cost_rate_tables::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let mut out = Vec::new();
    for t in &tables {
        let rates = rates_of(&ctx, t, "table").await?;
        out.push(serde_json::json!({
            "pid": t.pid, "name": t.name, "person_ref": t.person_ref, "role": t.role,
            "currency": t.currency,
            "rates": rates.rates.iter().map(|r| serde_json::json!({
                "effective_from": r.effective_from, "per_hour_minor": r.per_hour_minor })).collect::<Vec<_>>(),
        }));
    }
    format::json(serde_json::json!({ "tables": out }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `POST /api/cost-rate-tables/{pid}/rates` body.
pub(crate) struct RateBody {
    /// First day the rate applies; it stays in force until a later rate starts.
    effective_from: NaiveDate,
    /// Minor units of the table's currency per hour.
    per_hour_minor: i64,
}

/// `POST /api/cost-rate-tables/{pid}/rates` — add a rate, effective from a date.
/// Rates are append-only: there is no edit and no delete, and a date can carry
/// one rate (409), so what a day cost is what it was said to cost.
#[debug_handler]
async fn add_rate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<RateBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let pid = super::parse_pid(&pid)?;
    let table: cost_rate_tables::Model = sqlx::query_as(
        "SELECT * FROM cost_rate_tables WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    if b.per_hour_minor < 0 {
        return Err(unprocessable("per_hour_minor must not be negative"));
    }
    if b.effective_from < NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default()
        || chrono::Datelike::year(&b.effective_from) > 2100
    {
        return Err(unprocessable(
            "effective_from must be between 1970 and 2100",
        ));
    }
    // Only whether one exists is read.
    let taken: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM cost_rates WHERE table_pid = $1 AND effective_from = $2 LIMIT 1",
    )
    .bind(table.pid)
    .bind(b.effective_from)
    .fetch_optional(crate::db::pool(&ctx))
    .await?;
    if taken.is_some() {
        return Err(conflict(
            "that table already has a rate effective from that date; add a later date instead",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row = insert_rate(
        &mut tx,
        table.pid,
        b.effective_from,
        b.per_hour_minor,
        caller.actor(),
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        table.pid,
        "cost_rate_added",
        caller.actor(),
        Some(serde_json::json!({ "effective_from": row.effective_from, "per_hour_minor": row.per_hour_minor,
            "currency": table.currency })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid, "table_pid": table.pid,
        "effective_from": row.effective_from, "per_hour_minor": row.per_hour_minor, "currency": table.currency }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `GET /api/cost-rates/lookup` query.
pub(crate) struct LookupParams {
    /// The person whose rate is wanted.
    person_ref: String,
    /// The day the rate must be in force on.
    date: NaiveDate,
    #[serde(default)]
    /// Table name to look in; defaults to `standard`.
    table: Option<String>,
}

/// `GET /api/cost-rates/lookup` — the rate in force for a person on a date: their
/// own table's, else their role's. No rate is `null` with `no_rate`, never zero.
#[debug_handler]
async fn lookup(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<LookupParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let name = q.table.as_deref().unwrap_or("standard");
    let found = applicable(&ctx, &q.person_ref, name).await?;
    let hit = found.as_ref().and_then(|a| {
        rules::rate_on(&a.rates, q.date).map(|rate| {
            let from = a.rates.iter().filter(|r| r.effective_from <= q.date).map(|r| r.effective_from).max();
            serde_json::json!({ "per_hour_minor": rate, "currency": a.currency, "effective_from": from,
                "table_pid": a.table_pid, "source": a.source })
        })
    });
    format::json(serde_json::json!({
        "person_ref": q.person_ref, "date": q.date, "table": name,
        "rate": hit,
        "reason": if hit.is_none() { Some("no_rate") } else { None },
    }))
}

// ───────────────────────── task costs ─────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `POST` body for a task's material or fixed cost.
pub(crate) struct TaskCostBody {
    /// `material` or `fixed`.
    kind: String,
    /// What the cost is for.
    description: String,
    /// Amount in minor currency units (for example cents).
    amount_minor: i64,
    /// ISO currency code of the amount; amounts in different currencies are
    /// never summed.
    currency: String,
    /// `start`, `prorated` or `end`.
    accrual: String,
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/costs` — a material or fixed-cost expense.
#[debug_handler]
async fn add_task_cost(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<TaskCostBody>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let currency = b.currency.trim().to_ascii_uppercase();
    let mut problems = Vec::new();
    if !["material", "fixed"].contains(&b.kind.as_str()) {
        problems.push("kind must be material or fixed".to_string());
    }
    if b.description.trim().is_empty() || b.description.chars().count() > 200 {
        problems.push("description is required (up to 200 characters)".to_string());
    }
    if b.amount_minor <= 0 {
        problems.push("amount_minor must be above zero".to_string());
    }
    if !crate::currency::is_code(&currency) {
        problems.push("currency must be a 3-letter code, for example EUR, JPY or GBP".to_string());
    }
    if Accrual::parse(&b.accrual).is_none() {
        problems.push("accrual must be start, prorated or end".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row = insert_task_cost(
        &mut tx,
        task.pid,
        &b.kind,
        b.description.trim(),
        b.amount_minor,
        &currency,
        &b.accrual,
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "task_cost_added",
        caller.actor(),
        Some(serde_json::json!({ "task_pid": task.pid, "kind": row.kind, "amount_minor": row.amount_minor,
            "currency": row.currency, "accrual": row.accrual })),
    ).await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row.pid }))
}

/// `GET /api/plans/{pid}/tasks/{t_pid}/costs` — the task's expenses.
#[debug_handler]
async fn list_task_costs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let rows: Vec<task_costs::Model> = sqlx::query_as(
        "SELECT * FROM task_costs WHERE task_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(task.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(
        serde_json::json!({ "costs": rows.iter().map(|c| serde_json::json!({
        "pid": c.pid, "kind": c.kind, "description": c.description,
        "amount_minor": c.amount_minor, "currency": c.currency, "accrual": c.accrual })).collect::<Vec<_>>() }),
    )
}

/// `DELETE /api/plans/{pid}/tasks/{t_pid}/costs/{c_pid}` — soft-delete an expense.
#[debug_handler]
async fn delete_task_cost(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid, c_pid)): Path<(String, String, String)>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, plan.pid, &t_pid).await?;
    let c_pid = super::parse_pid(&c_pid)?;
    // Only the id and pid are read.
    let (id, row_pid): (i64, Uuid) = sqlx::query_as(
        "SELECT id, pid FROM task_costs WHERE pid = $1 AND task_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(c_pid)
    .bind(task.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let now = chrono::DateTime::<chrono::FixedOffset>::from(chrono::Utc::now());
    let mut tx = crate::db::begin(&ctx).await?;
    // By id, as `SeaORM`'s update of the row read above was; no row back is an error, as its was.
    sqlx::query_scalar::<_, i64>(
        "UPDATE task_costs SET deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(now)
    .bind(id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "task_cost_removed",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

// ───────────────────────── the planned cost ─────────────────────────

/// Bucket per-day money into reporting periods, one JSON object per
/// period (`minor` carries money here, reusing the minutes bucketing).
fn money_buckets(
    per_day: &[(NaiveDate, i64)],
    periods: &[crate::portfolio_resources::Period],
) -> Vec<serde_json::Value> {
    wp::bucket(per_day, periods)
        .0
        .iter()
        .map(|b| serde_json::json!({ "from": b.from, "to": b.to, "minor": b.minutes }))
        .collect()
}

/// Minor units by day.
type DayMoney = BTreeMap<NaiveDate, i64>;

/// Add `days` into the running per-day total, accumulating on equal dates.
fn add_days(into: &mut BTreeMap<NaiveDate, i64>, days: &[(NaiveDate, i64)]) {
    for (d, m) in days {
        *into.entry(*d).or_default() += m;
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `GET /api/plans/{pid}/cost-plan` query.
pub(crate) struct CostPlanParams {
    #[serde(flatten)]
    /// Reporting window and grain, shared with the assignment profile endpoint.
    window: super::assignments::ProfileParams,
    /// The rate table to price labour with (default `standard`).
    #[serde(default)]
    table: Option<String>,
}

/// `GET /api/plans/{pid}/cost-plan` — the planned cost by period, per currency.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn cost_plan(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<CostPlanParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let table = q.table.clone().unwrap_or_else(|| "standard".to_string());
    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let analysis = net.analyze(0).ok();
    let work = super::assignments::plan_work(&ctx, plan.pid).await?;
    let work_by: HashMap<Uuid, &super::assignments::TaskWork> =
        work.iter().map(|w| (w.pid, w)).collect();

    // Everything that will be placed on a day, to choose the window from.
    let mut every_day: Vec<(NaiveDate, i64)> = Vec::new();
    let mut labour: Vec<(Uuid, String, String, rules::Priced, Applicable)> = Vec::new();
    let mut unrated: Vec<serde_json::Value> = Vec::new();
    let mut applicable_cache: HashMap<String, Option<usize>> = HashMap::new();
    let mut tables: Vec<Applicable> = Vec::new();
    for w in &work {
        if w.unplaced.is_some() {
            continue;
        }
        let Some(person) = &w.assignee else { continue };
        let ix = if let Some(ix) = applicable_cache.get(person) {
            *ix
        } else {
            let found = applicable(&ctx, person, &table).await?;
            let ix = found.map(|a| {
                tables.push(a);
                tables.len() - 1
            });
            applicable_cache.insert(person.clone(), ix);
            ix
        };
        every_day.extend(w.per_day.iter().map(|(d, _)| (*d, 0)));
        match ix {
            None => unrated.push(serde_json::json!({ "pid": w.pid, "title": w.title,
                "work_minutes": w.work_minutes, "reason": "no_rate" })),
            Some(i) => {
                let a = &tables[i];
                let priced = rules::price_days(&w.per_day, |d| rules::rate_on(&a.rates, d));
                if priced.unrated_minutes > 0 {
                    unrated.push(serde_json::json!({ "pid": w.pid, "title": w.title,
                        "work_minutes": priced.unrated_minutes, "reason": "no_rate_on_some_days" }));
                }
                labour.push((
                    w.pid,
                    w.title.clone(),
                    a.currency.clone(),
                    priced,
                    Applicable {
                        table_pid: a.table_pid,
                        currency: a.currency.clone(),
                        source: a.source,
                        rates: Vec::new(),
                    },
                ));
            }
        }
    }
    // Work with an assignee but none placed, or no assignee at all, is said too.
    let unassigned: Vec<serde_json::Value> = work
        .iter()
        .filter(|w| w.assignee.is_none())
        .map(|w| serde_json::json!({ "pid": w.pid, "title": w.title, "work_minutes": w.work_minutes, "reason": "no_assignee" }))
        .collect();

    // Expenses, accrued over each task's business days.
    let costs: Vec<task_costs::Model> = sqlx::query_as(
        "SELECT * FROM task_costs WHERE deleted_at IS NULL AND task_pid = ANY($1) ORDER BY id",
    )
    .bind(net.rows.iter().map(|t| t.pid).collect::<Vec<_>>())
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut fixed: Vec<(task_costs::Model, Vec<(NaiveDate, i64)>)> = Vec::new();
    let mut unplaced: Vec<serde_json::Value> = Vec::new();
    for c in &costs {
        let ix = net.rows.iter().position(|t| t.pid == c.task_pid);
        let days = ix.and_then(|i| super::assignments::working_days_of(&net, analysis.as_ref(), i));
        let parts = days.as_ref().and_then(|d| {
            rules::accrue(
                c.amount_minor,
                d,
                Accrual::parse(&c.accrual).unwrap_or(Accrual::Prorated),
            )
        });
        match parts {
            Some(p) if !p.is_empty() => {
                every_day.extend(p.iter().map(|(d, _)| (*d, 0)));
                fixed.push((c.clone(), p));
            }
            _ => unplaced.push(
                serde_json::json!({ "cost_pid": c.pid, "task_pid": c.task_pid,
                "amount_minor": c.amount_minor, "currency": c.currency,
                "reason": if days.is_none() { "unscheduled" } else { "no_working_days" } }),
            ),
        }
    }
    if labour.is_empty() && fixed.is_empty() {
        return format::json(serde_json::json!({
            "plan_pid": plan.pid, "currencies": [], "tasks": [],
            "unrated": unrated, "unassigned": unassigned, "unplaced": unplaced,
            "reason": "no_costs",
            "note": "Nothing could be costed: no task with work has a rate, and no expense can be placed on a day.",
        }));
    }
    let refs: Vec<&(NaiveDate, i64)> = every_day.iter().collect();
    let (from, to, grain, periods) = super::assignments::window(&q.window, &refs)?;

    let mut by_currency: BTreeMap<String, (DayMoney, DayMoney)> = BTreeMap::new();
    let mut task_rows: BTreeMap<Uuid, serde_json::Value> = BTreeMap::new();
    let titles: HashMap<Uuid, &str> = net.rows.iter().map(|t| (t.pid, t.title.as_str())).collect();
    for (pid, title, currency, priced, _) in &labour {
        add_days(
            &mut by_currency.entry(currency.clone()).or_default().0,
            &priced.days,
        );
        let entry = task_rows
            .entry(*pid)
            .or_insert_with(|| serde_json::json!({ "pid": pid, "title": title, "fixed": [] }));
        entry["labour"] = serde_json::json!({ "currency": currency, "total_minor": priced.total_minor,
            "unrated_minutes": priced.unrated_minutes, "buckets": money_buckets(&priced.days, &periods) });
        let _ = work_by.get(pid);
    }
    for (c, parts) in &fixed {
        add_days(
            &mut by_currency.entry(c.currency.clone()).or_default().1,
            parts,
        );
        let entry = task_rows.entry(c.task_pid).or_insert_with(|| {
            serde_json::json!({
            "pid": c.task_pid, "title": titles.get(&c.task_pid), "fixed": [] })
        });
        if let Some(arr) = entry["fixed"].as_array_mut() {
            arr.push(
                serde_json::json!({ "cost_pid": c.pid, "kind": c.kind, "description": c.description,
                "currency": c.currency, "amount_minor": c.amount_minor, "accrual": c.accrual,
                "buckets": money_buckets(parts, &periods) }),
            );
        }
    }
    let currencies: Vec<serde_json::Value> = by_currency
        .iter()
        .map(|(cur, (lab, fix))| {
            let lab_days: Vec<(NaiveDate, i64)> = lab.iter().map(|(d, m)| (*d, *m)).collect();
            let fix_days: Vec<(NaiveDate, i64)> = fix.iter().map(|(d, m)| (*d, *m)).collect();
            let mut both = lab.clone();
            for (d, m) in fix {
                *both.entry(*d).or_default() += m;
            }
            let both_days: Vec<(NaiveDate, i64)> = both.iter().map(|(d, m)| (*d, *m)).collect();
            serde_json::json!({
                "currency": cur,
                "labour": { "total_minor": lab.values().sum::<i64>(), "buckets": money_buckets(&lab_days, &periods) },
                "fixed": { "total_minor": fix.values().sum::<i64>(), "buckets": money_buckets(&fix_days, &periods) },
                "total": { "total_minor": both.values().sum::<i64>(), "buckets": money_buckets(&both_days, &periods) },
            })
        })
        .collect();
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "from": from, "to": to,
        "period": match grain { wp::Grain::Day => "day", wp::Grain::Week => "week", wp::Grain::Month => "month" },
        "table": table,
        "currencies": currencies,
        "tasks": task_rows.into_values().collect::<Vec<_>>(),
        "unrated": unrated, "unassigned": unassigned, "unplaced": unplaced,
        "note": "Labour is each task's work (spread by its contour over its business days) priced day by day at the rate in force that day, from the assignee's own rate table or their role's; daily costs sum exactly to the rounded total. Expenses accrue over the task's business days by their rule (start, prorated, end) in whole minor units that sum to the amount. Work with no rate (or on days before the first rate) is listed under unrated and costs nothing here — unknown, never zero. Each currency stands alone and is never added to another. Derived on read.",
    }))
}

/// A plan's planned labour cost by currency, for a network already in hand (the
/// what-if sandbox prices the original and the edited copy).
pub(crate) struct LabourTotals {
    /// Total minor units per currency, each priced day by day at the rate in force.
    pub by_currency: BTreeMap<String, i64>,
    /// Minutes of work that no rate covers (unknown, never zero).
    pub unrated_minutes: i64,
}

/// Price every placed task's work for `net` with the `standard` tables.
/// Unplaced work is skipped; work with no assignee or no applicable rate
/// counts as `unrated_minutes`, never as zero cost. Rates are looked up
/// once per person.
pub(crate) async fn labour_totals(
    ctx: &AppContext,
    net: &super::cpm::Network,
    analysis: Option<&crate::cpm::CpmResult>,
) -> Result<LabourTotals> {
    let mut out = LabourTotals {
        by_currency: BTreeMap::new(),
        unrated_minutes: 0,
    };
    let mut cache: HashMap<String, Option<Applicable>> = HashMap::new();
    for w in super::assignments::work_of(net, analysis) {
        if w.unplaced.is_some() {
            continue;
        }
        let Some(person) = &w.assignee else {
            out.unrated_minutes += w.work_minutes;
            continue;
        };
        if !cache.contains_key(person) {
            let a = applicable(ctx, person, "standard").await?;
            cache.insert(person.clone(), a);
        }
        match cache.get(person).and_then(Option::as_ref) {
            None => out.unrated_minutes += w.work_minutes,
            Some(a) => {
                let priced = rules::price_days(&w.per_day, |d| rules::rate_on(&a.rates, d));
                out.unrated_minutes += priced.unrated_minutes;
                *out.by_currency.entry(a.currency.clone()).or_default() += priced.total_minor;
            }
        }
    }
    Ok(out)
}

// ───────────────────────── labour cost actuals ─────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// `POST /api/plans/{pid}/labour-costs/record` query.
pub(crate) struct RecordParams {
    #[serde(default)]
    /// Rate table name to price with; defaults to `standard`.
    table: Option<String>,
}

/// `POST /api/plans/{pid}/labour-costs/record` — price every recorded time entry
/// that has no recorded cost yet, at the rate in force the day it was worked, and
/// store the rate **with** the cost. Entries already recorded are never touched:
/// a later rate change never rewrites an actual. Entries with no rate are listed
/// and left unrecorded (cost unknown, not zero).
#[debug_handler]
async fn record_labour(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<RecordParams>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let table = q.table.unwrap_or_else(|| "standard".to_string());
    let entries: Vec<time_entries::Model> = sqlx::query_as(
        "SELECT * FROM time_entries WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    // Only the entry pids are read.
    let done: HashSet<Uuid> = sqlx::query_scalar::<_, Uuid>(
        "SELECT time_entry_pid FROM time_entry_costs WHERE plan_pid = $1",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?
    .into_iter()
    .collect();
    let mut cache: HashMap<String, Option<Applicable>> = HashMap::new();
    let (mut recorded, mut already) = (0_usize, 0_usize);
    let mut no_rate = Vec::new();
    // Price everything first (this reads); then record every cost and the audit row as one change. The
    // pool may hold a single connection, so nothing is read while the transaction is open.
    let mut costs: Vec<LabourCost> = Vec::new();
    for e in &entries {
        if done.contains(&e.pid) {
            already += 1;
            continue;
        }
        if !cache.contains_key(&e.actor_ref) {
            let a = applicable(&ctx, &e.actor_ref, &table).await?;
            cache.insert(e.actor_ref.clone(), a);
        }
        let hit = cache
            .get(&e.actor_ref)
            .and_then(Option::as_ref)
            .and_then(|a| {
                rules::rate_on(&a.rates, e.spent_on).map(|r| (r, a.table_pid, a.currency.clone()))
            });
        let Some((rate, table_pid, currency)) = hit else {
            no_rate.push(
                serde_json::json!({ "time_entry_pid": e.pid, "minutes": e.minutes,
                "spent_on": e.spent_on, "reason": "no_rate" }),
            );
            continue;
        };
        costs.push(LabourCost {
            entry: e,
            table_pid,
            per_hour_minor: rate,
            currency,
        });
        recorded += 1;
    }
    let mut tx = crate::db::begin(&ctx).await?;
    for c in costs {
        // The columns `SeaORM`'s insert set, no more.
        sqlx::query(
            "INSERT INTO time_entry_costs
                 (pid, time_entry_pid, plan_pid, person_ref, table_pid, spent_on, minutes, per_hour_minor, currency,
                  cost_minor)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(Uuid::new_v4())
        .bind(c.entry.pid)
        .bind(plan.pid)
        .bind(&c.entry.actor_ref)
        .bind(c.table_pid)
        .bind(c.entry.spent_on)
        .bind(c.entry.minutes)
        .bind(c.per_hour_minor)
        .bind(&c.currency)
        .bind(rules::labour_cost_minor(c.entry.minutes, c.per_hour_minor))
        .execute(tx.conn())
        .await?;
    }
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "labour_costs_recorded",
        caller.actor(),
        Some(
            serde_json::json!({ "recorded": recorded, "already_recorded": already,
            "no_rate": no_rate.len(), "table": table }),
        ),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "recorded": recorded, "already_recorded": already, "no_rate": no_rate,
        "note": "A recorded cost keeps the rate it was priced at. Recording again never changes one; it prices only entries that have none yet.",
    }))
}

/// One time entry priced, waiting to be recorded.
struct LabourCost<'a> {
    /// The entry priced.
    entry: &'a time_entries::Model,
    /// The table the rate came from.
    table_pid: Uuid,
    /// The rate in force on the day it was worked.
    per_hour_minor: i64,
    /// The table's currency.
    currency: String,
}

/// `GET /api/plans/{pid}/labour-costs` — the recorded labour cost, per currency
/// and never across, with the entries still unpriced counted apart.
#[debug_handler]
async fn labour_costs(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    finance_only(&caller)?;
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let snaps: Vec<time_entry_costs::Model> =
        sqlx::query_as("SELECT * FROM time_entry_costs WHERE plan_pid = $1 ORDER BY id")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let entries: Vec<time_entries::Model> =
        sqlx::query_as("SELECT * FROM time_entries WHERE plan_pid = $1 AND deleted_at IS NULL")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let priced: HashSet<Uuid> = snaps.iter().map(|s| s.time_entry_pid).collect();
    let unpriced: Vec<&time_entries::Model> = entries
        .iter()
        .filter(|e| !priced.contains(&e.pid))
        .collect();
    let mut by_cur: BTreeMap<&str, (i64, i64, usize)> = BTreeMap::new();
    for s in &snaps {
        let e = by_cur.entry(&s.currency).or_default();
        e.0 += s.cost_minor;
        e.1 += s.minutes;
        e.2 += 1;
    }
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "currencies": by_cur.iter().map(|(c, (cost, minutes, n))| serde_json::json!({
            "currency": c, "cost_minor": cost, "minutes": minutes, "entries": n })).collect::<Vec<_>>(),
        "entries": snaps.iter().map(|s| serde_json::json!({
            "time_entry_pid": s.time_entry_pid, "person_ref": s.person_ref, "spent_on": s.spent_on,
            "minutes": s.minutes, "per_hour_minor": s.per_hour_minor, "currency": s.currency,
            "cost_minor": s.cost_minor })).collect::<Vec<_>>(),
        "unpriced": { "entries": unpriced.len(), "minutes": unpriced.iter().map(|e| e.minutes).sum::<i64>(),
                      "reason": if unpriced.is_empty() { None } else { Some("not_recorded_or_no_rate") } },
        "note": "Each entry's cost carries the rate it was priced at. Entries not yet recorded, or with no rate when recorded, are counted under unpriced: their cost is unknown, not zero. Currencies are never added together.",
    }))
}

/// The cost-rate routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/cost-rate-tables", post(create_table))
        .add("/cost-rate-tables", get(list_tables))
        .add("/cost-rate-tables/{pid}/rates", post(add_rate))
        .add("/cost-rates/lookup", get(lookup))
        .add("/plans/{pid}/tasks/{t_pid}/costs", post(add_task_cost))
        .add("/plans/{pid}/tasks/{t_pid}/costs", get(list_task_costs))
        .add(
            "/plans/{pid}/tasks/{t_pid}/costs/{c_pid}",
            delete(delete_task_cost),
        )
        .add("/plans/{pid}/cost-plan", get(cost_plan))
        .add("/plans/{pid}/labour-costs/record", post(record_labour))
        .add("/plans/{pid}/labour-costs", get(labour_costs))
}
