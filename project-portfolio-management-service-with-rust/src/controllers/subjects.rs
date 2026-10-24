//! Data-subject requests (RG-6): export everything held about one person reference, and replace
//! the reference with a pseudonym. Rules and the policy statement are in [`crate::subjects`].
//!
//! **Administrators only** (`access=admin`; the `subjects` area is closed to the policy file, like
//! `finance`). Every call is audited without writing the reference itself. Pseudonymising is a
//! **preview unless `?apply=true`**, runs in one transaction and is all or nothing.

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement, TransactionTrait, Value};
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::audit_logs::Model as AuditModel;
use crate::subjects::{self as rules, COLUMNS};

/// Most rows returned per table in an export.
const EXPORT_ROWS_PER_TABLE: i64 = 1_000;

/// The environment variable holding the secret salt for pseudonyms.
pub const SALT_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_PSEUDONYM_SALT";

fn forbidden() -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden", "data-subject requests need access=admin"),
    )
}

fn allowed(caller: &MaybeAuthUser) -> bool {
    crate::auth::area_allows(caller, "subjects", authentication_verifier::Action::Write)
}

/// `$1, $2, …` placeholders and the matching values for the forms of a subject.
fn in_list(forms: &[String]) -> (String, Vec<Value>) {
    let marks: Vec<String> = (1..=forms.len()).map(|i| format!("${i}")).collect();
    (
        marks.join(", "),
        forms.iter().map(|f| Value::from(f.clone())).collect(),
    )
}

/// Whether a JSON key looks like a credential or a digest, which an export leaves out.
fn withheld(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.contains("hash") || k.contains("secret") || k.contains("password") || k.contains("token")
}

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
    let (marks, values) = in_list(&forms);
    let mut tables = Vec::new();
    let mut total: i64 = 0;
    for (table, column) in COLUMNS {
        let sql = format!(
            "SELECT row_to_json(t)::text AS j FROM \"{table}\" t WHERE \"{column}\" IN ({marks}) LIMIT {}",
            EXPORT_ROWS_PER_TABLE + 1
        );
        let found = ctx
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                sql,
                values.clone(),
            ))
            .await
            .map_err(db_err)?;
        if found.is_empty() {
            continue;
        }
        let truncated = i64::try_from(found.len()).unwrap_or(0) > EXPORT_ROWS_PER_TABLE;
        let rows: Vec<serde_json::Value> = found
            .iter()
            .take(usize::try_from(EXPORT_ROWS_PER_TABLE).unwrap_or(1000))
            .filter_map(|r| r.try_get::<String>("", "j").ok())
            .filter_map(|j| serde_json::from_str(&j).ok())
            .map(clean)
            .collect();
        total += i64::try_from(rows.len()).unwrap_or(0);
        tables.push(serde_json::json!({
            "table": table, "column": column, "rows": rows, "truncated": truncated,
        }));
    }
    AuditModel::record(
        &ctx.db,
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

const NOT_COVERED: &str = "Names or e-mail addresses typed into free text (titles, comments, documents), the content of uploaded files, the snapshot JSON of audit rows, notification destinations (listed above under notification_sinks), request logs (which record the path of every call, this one included) and backups are not searched. Credential and digest fields are left out of the export.";

#[derive(Debug, Deserialize)]
struct ApplyParams {
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
    tx: &sea_orm::DatabaseTransaction,
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
            let update =
                format!("UPDATE \"{table}\" SET \"{column}\" = $2 WHERE \"{column}\" = $1");
            n += tx
                .execute_raw(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    update,
                    vec![
                        Value::from(form.clone()),
                        Value::from(replacement_for(form, pseudo)),
                    ],
                ))
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
            let count = format!("SELECT COUNT(*) AS n FROM \"{table}\" WHERE \"{column}\" = $1");
            let row = tx
                .query_one_raw(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    count,
                    vec![Value::from(form.clone())],
                ))
                .await
                .map_err(db_err)?;
            n += u64::try_from(
                row.and_then(|r| r.try_get::<i64>("", "n").ok())
                    .unwrap_or(0),
            )
            .unwrap_or(0);
        }
    }
    Ok(n)
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
    if !allowed(&caller) {
        return Err(forbidden());
    }
    let forms = rules::forms(&subject);
    if forms.is_empty() {
        return Err(unprocessable("the subject reference is empty"));
    }
    if rules::is_pseudonym(&subject) {
        return Err(unprocessable("that reference is already a pseudonym"));
    }
    let salt = std::env::var(SALT_ENV).unwrap_or_default();
    let pseudo = rules::pseudonym(&subject, &salt);
    let apply = q.apply == Some(true);
    let tx = ctx.db.begin().await.map_err(db_err)?;
    tx.execute_unprepared("SELECT set_config('ppm.audit_pseudonymise', 'on', true)")
        .await
        .map_err(db_err)?;
    let mut changed = Vec::new();
    let mut total: u64 = 0;
    for (table, column) in COLUMNS {
        let n = touch(&tx, table, column, &forms, pseudo, apply).await?;
        if n > 0 {
            total += n;
            changed.push(serde_json::json!({ "table": table, "column": column, "rows": n }));
        }
    }
    if apply {
        AuditModel::record(
            &tx,
            Uuid::nil(),
            "subject_pseudonymised",
            caller.actor(),
            Some(
                serde_json::json!({ "pseudonym": pseudo, "tables": changed.len(), "rows": total }),
            ),
        )
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
        tx.commit().await.map_err(db_err)?;
    } else {
        tx.rollback().await.map_err(db_err)?;
    }
    format::json(serde_json::json!({
        "applied": apply,
        "pseudonym": pseudo,
        "total_rows": total,
        "changed": changed,
        "note": if apply {
            "Done. The reference is replaced everywhere listed. Free text, uploaded files, audit snapshots, notification destinations and backups are not covered."
        } else {
            "Preview only: nothing was changed. Repeat with ?apply=true to apply."
        },
    }))
}

/// The data-subject routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/subjects")
        .add("/{subject}", get(export))
        .add("/{subject}/pseudonymise", post(pseudonymise))
}
