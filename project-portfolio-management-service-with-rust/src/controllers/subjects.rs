//! Data-subject requests (RG-6): export everything held about one person reference, and replace
//! the reference with a pseudonym. Rules and the policy statement are in [`crate::subjects`].
//!
//! **Administrators only** (`access=admin`; the `subjects` area is closed to the policy file, like
//! `finance`). Every call is audited without writing the reference itself. Pseudonymising is a
//! **preview unless `?apply=true`**, runs in one transaction and is all or nothing.

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::audit_logs::Model as AuditModel;
use crate::subjects::{self as rules, COLUMNS};

/// Most rows returned per table in an export.
const EXPORT_ROWS_PER_TABLE: i64 = 1_000;

/// The environment variable holding the secret salt for pseudonyms.
pub const SALT_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_PSEUDONYM_SALT";

/// 403 error returned when the caller lacks write access to the `subjects` area (administrators).
fn forbidden() -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden", "data-subject requests need access=admin"),
    )
}

/// Whether the caller may write in the `subjects` area, which gates every data-subject request.
fn allowed(caller: &MaybeAuthUser) -> bool {
    crate::auth::area_allows(caller, "subjects", authentication_verifier::Action::Write)
}

/// Push `"<column>" IN ($n, $n+1, …)` onto `q`, one bound parameter per form of the subject. `column` is one of the
/// fixed names in `subjects::COLUMNS`, checked by the caller.
fn push_in_list(q: &mut sqlx::QueryBuilder<sqlx::Postgres>, column: &str, forms: &[String]) {
    q.push(format!("\"{column}\" IN ("));
    let mut list = q.separated(", ");
    for form in forms {
        list.push_bind(form.clone());
    }
    list.push_unseparated(")");
}

/// Whether a JSON key looks like a credential or a digest, which an export leaves out.
fn withheld(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.contains("hash") || k.contains("secret") || k.contains("password") || k.contains("token")
}

/// Drop credential-like and digest-like keys from an exported row (see [`withheld`]) so an export never carries secrets.
fn clean(mut row: serde_json::Value) -> serde_json::Value {
    if let Some(map) = row.as_object_mut() {
        map.retain(|k, _| !withheld(k));
    }
    row
}

/// `GET /api/admin/subjects/{subject}` — everything held about this person reference.
#[debug_handler]
async fn export(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(subject): Path<String>,
) -> Result<Response> {
    if !allowed(&caller) {
        return Err(forbidden());
    }
    let forms = rules::forms(&subject);
    if forms.is_empty() {
        return Err(unprocessable("the subject reference is empty"));
    }
    let mut tables = Vec::new();
    let mut total: i64 = 0;
    for (table, column) in COLUMNS {
        // Only the fixed names in `subjects::COLUMNS` reach the SQL (CQ-12); every form is a bound parameter.
        let mut q = sqlx::QueryBuilder::<sqlx::Postgres>::new(format!(
            "SELECT row_to_json(t)::text AS j FROM \"{table}\" t WHERE "
        ));
        push_in_list(&mut q, column, &forms);
        q.push(format!(" LIMIT {}", EXPORT_ROWS_PER_TABLE + 1));
        let found: Vec<Option<String>> = q
            .build_query_scalar()
            .fetch_all(crate::db::pool(&ctx))
            .await?;
        if found.is_empty() {
            continue;
        }
        let truncated = i64::try_from(found.len()).unwrap_or(0) > EXPORT_ROWS_PER_TABLE;
        let rows: Vec<serde_json::Value> = found
            .into_iter()
            .take(usize::try_from(EXPORT_ROWS_PER_TABLE).unwrap_or(1000))
            .flatten()
            .filter_map(|j| serde_json::from_str(&j).ok())
            .map(clean)
            .collect();
        total += i64::try_from(rows.len()).unwrap_or(0);
        tables.push(serde_json::json!({
            "table": table, "column": column, "rows": rows, "truncated": truncated,
        }));
    }
    AuditModel::record_on(
        crate::db::pool(&ctx),
        Uuid::nil(),
        "subject_exported",
        caller.actor(),
        Some(serde_json::json!({ "tables": tables.len(), "rows": total })),
    )
    .await
    .map_err(super::audit_failed)?;
    format::json(serde_json::json!({
        "subject_forms_searched": forms.len(),
        "tables": tables,
        "total_rows": total,
        "not_covered": NOT_COVERED,
    }))
}

/// Plain-language statement of what a subject search and export do not cover, returned with every result so absence is not read as proof of no data.
const NOT_COVERED: &str = "Names or e-mail addresses typed into free text (titles, comments, documents), the content of uploaded files, the snapshot JSON of audit rows, notification destinations (listed above under notification_sinks), request logs (which record the path of every call, this one included) and backups are not searched. Credential and digest fields are left out of the export.";

/// Query parameters for the erasure endpoint.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ApplyParams {
    /// `true` applies the change; absent or false only previews counts and changes nothing.
    #[serde(default)]
    apply: Option<bool>,
}

/// What a stored form of the reference becomes, keeping its shape: `person:<id>` becomes
/// `person:<pseudonym>`, a bare subject becomes the bare pseudonym.
fn replacement_for(form: &str, pseudo: Uuid) -> String {
    for prefix in ["person:", "worker:"] {
        if form.starts_with(prefix) {
            return format!("{prefix}{pseudo}");
        }
    }
    pseudo.to_string()
}

/// Count (preview) or change (apply) the rows of one column that hold any form of the subject.
async fn touch(
    tx: &mut crate::db::Tx,
    table: &str,
    column: &str,
    forms: &[String],
    pseudo: Uuid,
    apply: bool,
) -> Result<u64> {
    // Only the fixed names in `subjects::COLUMNS` reach the SQL; refuse anything else outright.
    if !rules::is_plain_identifier(table) || !rules::is_plain_identifier(column) {
        return Err(Error::string("not a known table or column"));
    }
    let mut n: u64 = 0;
    for form in forms {
        if apply {
            let mut update = sqlx::QueryBuilder::<sqlx::Postgres>::new(format!(
                "UPDATE \"{table}\" SET \"{column}\" = "
            ));
            update.push_bind(replacement_for(form, pseudo));
            update.push(format!(" WHERE \"{column}\" = "));
            update.push_bind(form.clone());
            n += update
                .build()
                .execute(tx.conn())
                .await
                .map_err(|error| {
                    // The database's own text stays in the log: it can name constraints and
                    // values (CQ-12).
                    tracing::error!(table, column, %error, "pseudonymisation update refused");
                    Error::CustomError(
                        StatusCode::CONFLICT,
                        ErrorDetail::new(
                            "conflict",
                            format!("{table}.{column} could not be changed; nothing was changed"),
                        ),
                    )
                })?
                .rows_affected();
        } else {
            let mut count = sqlx::QueryBuilder::<sqlx::Postgres>::new(format!(
                "SELECT COUNT(*) AS n FROM \"{table}\" WHERE \"{column}\" = "
            ));
            count.push_bind(form.clone());
            let found: i64 = count.build_query_scalar().fetch_one(tx.conn()).await?;
            n += u64::try_from(found).unwrap_or(0);
        }
    }
    Ok(n)
}

/// Count (preview) or delete (apply) the rows of one person-only column ([`rules::OWN_ROWS`]) that hold
/// any form of the subject.
async fn remove(
    tx: &mut crate::db::Tx,
    table: &str,
    column: &str,
    forms: &[String],
    apply: bool,
) -> Result<u64> {
    if !rules::is_plain_identifier(table) || !rules::is_plain_identifier(column) {
        return Err(Error::string("not a known table or column"));
    }
    if apply {
        let mut delete =
            sqlx::QueryBuilder::<sqlx::Postgres>::new(format!("DELETE FROM \"{table}\" WHERE "));
        push_in_list(&mut delete, column, forms);
        return Ok(delete
            .build()
            .execute(tx.conn())
            .await
            .map_err(|error| {
                tracing::error!(table, column, %error, "erasure delete refused");
                Error::CustomError(
                    StatusCode::CONFLICT,
                    ErrorDetail::new(
                        "conflict",
                        format!("{table} rows could not be deleted; nothing was changed"),
                    ),
                )
            })?
            .rows_affected());
    }
    let mut count = sqlx::QueryBuilder::<sqlx::Postgres>::new(format!(
        "SELECT COUNT(*) AS n FROM \"{table}\" WHERE "
    ));
    push_in_list(&mut count, column, forms);
    let found: i64 = count.build_query_scalar().fetch_one(tx.conn()).await?;
    Ok(u64::try_from(found).unwrap_or(0))
}

/// Erase one subject under `policy`: a preview (counts only, rolled back) unless `apply`.
async fn erase_under(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    subject: &str,
    apply: bool,
    policy: rules::ErasurePolicy,
) -> Result<Response> {
    if !allowed(caller) {
        return Err(forbidden());
    }
    let forms = rules::forms(subject);
    if forms.is_empty() {
        return Err(unprocessable("the subject reference is empty"));
    }
    if rules::is_pseudonym(subject) {
        return Err(unprocessable("that reference is already a pseudonym"));
    }
    let deleting = policy == rules::ErasurePolicy::Delete;
    let replacement = if deleting {
        rules::erasure_marker()
    } else {
        let salt = std::env::var(SALT_ENV).unwrap_or_default();
        rules::pseudonym(subject, &salt)
    };
    let mut tx = crate::db::begin(ctx).await?;
    sqlx::query("SELECT set_config('ppm.audit_pseudonymise', 'on', true)")
        .execute(tx.conn())
        .await?;
    let mut changed = Vec::new();
    let mut deleted = Vec::new();
    let mut total: u64 = 0;
    for (table, column) in COLUMNS {
        if deleting && rules::OWN_ROWS.contains(&(*table, *column)) {
            let n = remove(&mut tx, table, column, &forms, apply).await?;
            if n > 0 {
                total += n;
                deleted.push(serde_json::json!({ "table": table, "column": column, "rows": n }));
            }
            continue;
        }
        let n = touch(&mut tx, table, column, &forms, replacement, apply).await?;
        if n > 0 {
            total += n;
            changed.push(serde_json::json!({ "table": table, "column": column, "rows": n }));
        }
    }
    if apply {
        // Under delete the random reference is written nowhere outside the records it replaced, not even here.
        let detail = if deleting {
            serde_json::json!({ "policy": policy.name(), "tables": changed.len() + deleted.len(), "rows": total })
        } else {
            serde_json::json!({ "pseudonym": replacement, "tables": changed.len(), "rows": total })
        };
        AuditModel::record_on(
            tx.conn(),
            Uuid::nil(),
            if deleting {
                "subject_erased"
            } else {
                "subject_pseudonymised"
            },
            caller.actor(),
            Some(detail),
        )
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }
    let mut out = serde_json::json!({
        "applied": apply,
        "policy": policy.name(),
        "total_rows": total,
        "changed": changed,
        "note": if !apply {
            "Preview only: nothing was changed. Repeat with ?apply=true to apply."
        } else if deleting {
            "Done. The person's own rows are deleted and every other reference is replaced by a random one nothing can recompute. Free text, uploaded files, audit snapshots and backups are not covered."
        } else {
            "Done. The reference is replaced everywhere listed. Free text, uploaded files, audit snapshots, notification destinations and backups are not covered."
        },
    });
    if deleting {
        out["deleted"] = serde_json::Value::from(deleted);
    } else {
        out["pseudonym"] = serde_json::json!(replacement);
    }
    format::json(out)
}

/// `POST /api/admin/subjects/{subject}/pseudonymise` — replace the reference with a stable
/// pseudonym everywhere it is held. A preview (counts only) unless `?apply=true`.
#[debug_handler]
async fn pseudonymise(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(subject): Path<String>,
    Query(q): Query<ApplyParams>,
) -> Result<Response> {
    erase_under(
        &ctx,
        &caller,
        &subject,
        q.apply == Some(true),
        rules::ErasurePolicy::Pseudonymise,
    )
    .await
}

/// `POST /api/admin/subjects/{subject}/erase` — erase the person under the deployment's erasure policy
/// (`PROJECT_PORTFOLIO_MANAGEMENT_ERASURE_POLICY`: `pseudonymise`, the default, or `delete`; WW-1.8). A preview
/// (counts only) unless `?apply=true`.
#[debug_handler]
async fn erase(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(subject): Path<String>,
    Query(q): Query<ApplyParams>,
) -> Result<Response> {
    if !allowed(&caller) {
        return Err(forbidden());
    }
    let policy = rules::deployment_policy().map_err(|m| {
        Error::CustomError(
            StatusCode::SERVICE_UNAVAILABLE,
            ErrorDetail::new("misconfigured", m),
        )
    })?;
    erase_under(&ctx, &caller, &subject, q.apply == Some(true), policy).await
}

/// The data-subject routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/subjects")
        .add("/{subject}", get(export))
        .add("/{subject}/pseudonymise", post(pseudonymise))
        .add("/{subject}/erase", post(erase))
}
