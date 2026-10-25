//! Many MS Project (MSPDI XML) files in one job (NC-13): `POST /api/import/mspdi-batch?dry_run=true` (the default).
//! Rules and tests in [`crate::mspdi_batch`]; each file is imported by the single-file importer
//! ([`super::mspdi_import`]), so everything it says it leaves out is still said.
//!
//! Each project goes into an existing plan (`plan_pid`) or a **new** plan (`name`, else the file's project name,
//! under `parent_pid` when given). The job reads and checks every file before writing anything, lists the resources of
//! all the files once, and stores the project's custom-field values on the matching tasks according to `field_map`.
//!
//! **Not one transaction.** Each file's import is many writes and a new plan is created on its own, as for the
//! single-file importer. The job stops at the first project that fails and says which were done and which were not
//! attempted; asking again converges for plans that exist (re-import is idempotent) but a plan created by a failed run
//! is kept, not undone.

use std::collections::BTreeMap;

use axum::extract::DefaultBodyLimit;
use loco_rs::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::{db_err, unprocessable_entity as unprocessable};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::tasks;
use crate::models::audit_logs::Model as AuditModel;
use crate::mspdi::{self, Parsed};
use crate::mspdi_batch::{self as rules, Mapped};

/// `?dry_run=` (default `true`).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BatchParams {
    /// `true` (default) reports without writing.
    #[serde(default)]
    dry_run: Option<bool>,
}

/// One file of the job.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ProjectIn {
    /// Import into this existing plan; omit to create a new one.
    #[serde(default)]
    plan_pid: Option<Uuid>,
    /// The new plan's name (default: the project's name in the file).
    #[serde(default)]
    name: Option<String>,
    /// The MSPDI XML, as UTF-8 text.
    xml: String,
}

/// `POST /api/import/mspdi-batch` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BatchBody {
    /// The files, at most ten.
    projects: Vec<ProjectIn>,
    /// Put every new plan under this plan.
    #[serde(default)]
    parent_pid: Option<Uuid>,
    /// A project field (its alias, name or id) to the key of an existing task custom field.
    #[serde(default)]
    field_map: BTreeMap<String, String>,
}

/// What a refusal says: a custom error's description, else the error's own text (a custom error prints as nothing).
fn message_of(e: &Error) -> String {
    match e {
        Error::CustomError(_, d) => d
            .description
            .clone()
            .or_else(|| d.error.clone())
            .unwrap_or_default(),
        other => other.to_string(),
    }
}

/// The custom-field values of one project that the service accepts, and a note for each it does not.
async fn resolve_values(
    ctx: &AppContext,
    mapped: &[Mapped],
) -> (Vec<(i64, String, Value)>, Vec<Value>) {
    let (mut accepted, mut rejected) = (Vec::new(), Vec::new());
    for m in mapped {
        let mut last = String::new();
        let mut ok = None;
        for candidate in rules::candidates(&m.raw) {
            let values: BTreeMap<String, Value> = [(m.key.clone(), candidate)].into();
            match super::custom_fields::prepare(ctx, "task", None, &values, false).await {
                Ok(p) => {
                    ok = p.into_iter().next().and_then(|(k, v)| v.map(|v| (k, v)));
                    break;
                }
                Err(e) => last = message_of(&e),
            }
        }
        match ok {
            Some((key, value)) => accepted.push((m.uid, key, value)),
            None => rejected.push(json!({ "uid": m.uid, "field": m.key, "reason": last })),
        }
    }
    (accepted, rejected)
}

/// Store accepted values on the tasks the import created or updated (found by their `mspdi:<UID>` reference).
async fn store_values(
    ctx: &AppContext,
    plan_pid: Uuid,
    accepted: &[(i64, String, Value)],
) -> Result<usize> {
    let refs: Vec<String> = accepted
        .iter()
        .map(|(u, _, _)| format!("mspdi:{u}"))
        .collect();
    let rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::ExternalRef.is_in(refs))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut stored = 0;
    for (uid, key, value) in accepted {
        let ext = format!("mspdi:{uid}");
        if let Some(t) = rows
            .iter()
            .find(|t| t.external_ref.as_deref() == Some(ext.as_str()))
        {
            super::custom_fields::store(
                &ctx.db,
                "task",
                t.pid,
                &[(key.clone(), Some(value.clone()))],
            )
            .await?;
            stored += 1;
        }
    }
    Ok(stored)
}

fn label_of(i: usize, p: &ProjectIn, parsed: &Parsed) -> String {
    p.name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_string)
        .or_else(|| parsed.name.clone())
        .unwrap_or_else(|| format!("Project {}", i + 1))
}

/// What a dry run says about a file whose plan does not exist yet.
fn summary_of(parsed: &Parsed) -> Value {
    json!({
        "project_name": parsed.name,
        "tasks_in_file": parsed.tasks.len(),
        "links_in_file": parsed.links.len(),
        "unmapped": parsed.unmapped.iter().map(|u| json!({ "what": u.what, "uid": u.uid, "reason": u.reason })).collect::<Vec<_>>(),
        "note": "The plan does not exist yet, so this is what the file holds; the full report (workflow states, calendar, constraints) needs the plan and is given when the job runs.",
    })
}

async fn create_plan(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    name: &str,
    parent: Option<Uuid>,
) -> Result<Uuid> {
    let plan = project_portfolio_management_matcher::Plan {
        kind: super::plans::parse_kind_label("project"),
        name: name.to_string(),
        parent_ref: parent.map(|p| p.to_string()),
        ..Default::default()
    };
    super::plans::validate(&plan)?;
    let model = crate::streaming::create_and_emit(&ctx.db, &plan, caller.actor()).await?;
    Ok(model.pid)
}

/// `POST /api/import/mspdi-batch`: read every file first, list the resources of all of them once, then import each
/// into its plan (existing, or created) and store the mapped custom-field values. The default is a dry run.
#[expect(clippy::too_many_lines)] // one linear job: check, report, then write file by file
#[debug_handler]
async fn batch(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<BatchParams>,
    Json(b): Json<BatchBody>,
) -> Result<Response> {
    let dry_run = params.dry_run.unwrap_or(true);
    if b.projects.is_empty() || b.projects.len() > rules::MAX_PROJECTS {
        return Err(unprocessable(&format!(
            "send between 1 and {} projects",
            rules::MAX_PROJECTS
        )));
    }
    if let Some(parent) = b.parent_pid {
        resolve_subject(&ctx, &caller, "plan", &parent.to_string())
            .await
            .map_err(|_| unprocessable("parent_pid names no plan this caller can see"))?;
    }
    // Everything is read before anything is written.
    let mut parsed: Vec<Parsed> = Vec::new();
    for (i, p) in b.projects.iter().enumerate() {
        parsed.push(
            mspdi::parse(&p.xml).map_err(|e| unprocessable(&format!("project {}: {e}", i + 1)))?,
        );
    }
    let labels: Vec<String> = b
        .projects
        .iter()
        .enumerate()
        .map(|(i, p)| label_of(i, p, &parsed[i]))
        .collect();
    let named: Vec<(String, &Parsed)> = labels.iter().cloned().zip(parsed.iter()).collect();
    let pool = rules::merge_resources(&named);

    let mut results: Vec<Value> = Vec::new();
    let mut failed: Option<usize> = None;
    let mut created_plans = 0usize;
    let mut first_plan: Option<Uuid> = None;
    for (i, p) in b.projects.iter().enumerate() {
        let label = &labels[i];
        if failed.is_some() {
            results.push(json!({ "project": label, "outcome": "not_attempted" }));
            continue;
        }
        let (mapped, left_out) = rules::map_fields(&parsed[i], &b.field_map);
        let (accepted, rejected) = resolve_values(&ctx, &mapped).await;
        let fields_left_out: Vec<Value> = left_out
            .iter()
            .map(|l| json!({ "field": l.field, "values": l.values, "reason": l.reason }))
            .collect();
        let step: Result<Value> = async {
            let mut plan_pid = p.plan_pid;
            let mut created = false;
            if let Some(pid) = plan_pid {
                super::governance::find_item(&ctx, &pid.to_string()).await?;
            } else if !dry_run {
                plan_pid = Some(create_plan(&ctx, &caller, label, b.parent_pid).await?);
                created = true;
            }
            let report = match plan_pid {
                Some(pid) => {
                    mspdi_import_run(&ctx, &caller, pid, dry_run, p.xml.as_bytes()).await?
                }
                None => summary_of(&parsed[i]),
            };
            let stored = match (dry_run, plan_pid) {
                (false, Some(pid)) => store_values(&ctx, pid, &accepted).await?,
                _ => 0,
            };
            Ok(json!({
                "project": label,
                "outcome": if dry_run { "would_import" } else { "imported" },
                "plan_pid": plan_pid,
                "plan_created": created,
                "plan_would_be_created": plan_pid.is_none(),
                "report": report,
                "custom_fields": {
                    "values_to_store": accepted.len(),
                    "values_stored": stored,
                    "values_rejected": rejected,
                    "fields_left_out": fields_left_out,
                },
            }))
        }
        .await;
        match step {
            Ok(v) => {
                if v["plan_created"] == true {
                    created_plans += 1;
                }
                if first_plan.is_none() {
                    first_plan = v["plan_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok());
                }
                results.push(v);
            }
            Err(e) => {
                failed = Some(i);
                results.push(
                    json!({ "project": label, "outcome": "failed", "error": message_of(&e) }),
                );
            }
        }
    }
    if !dry_run && let Some(pid) = first_plan {
        AuditModel::record(
            &ctx.db,
            pid,
            "mspdi_batch_import",
            caller.actor(),
            Some(json!({
                "projects": b.projects.len(), "plans_created": created_plans,
                "failed_at": failed, "resources_in_pool": pool.len(),
            })),
        )
        .await
        .map_err(super::audit_failed)?;
    }
    format::json(json!({
        "dry_run": dry_run,
        "projects": results,
        "failed_at": failed,
        "resource_pool": pool.iter().map(|r| json!({
            "name": r.name, "projects": r.projects, "assignments": r.assignments,
            "note": "not matched to a person (names are not identities)",
        })).collect::<Vec<_>>(),
        "left_out": [
            "cross-project links (a task waiting on another project's task) are not read: each file's own links come across, links between files do not",
            "enterprise resource data (rates, calendars, skills) and resource costs: only the names, listed once above",
            "custom fields with no entry in field_map, and values the service's field type refuses (listed per project)",
        ],
    }))
}

/// Alias so the call reads as the single-file importer's.
async fn mspdi_import_run(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan_pid: Uuid,
    dry_run: bool,
    xml: &[u8],
) -> Result<Value> {
    super::mspdi_import::run(ctx, caller, &plan_pid.to_string(), dry_run, xml).await
}

/// The batch import route (body limit raised to ten files).
pub fn routes() -> Routes {
    let ceiling = mspdi::MAX_BYTES * rules::MAX_PROJECTS + 1024 * 1024;
    Routes::new().prefix("/api").add(
        "/import/mspdi-batch",
        post(batch).layer(DefaultBodyLimit::max(ceiling)),
    )
}
