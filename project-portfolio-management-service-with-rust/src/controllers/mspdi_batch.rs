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

use std::collections::{BTreeMap, HashMap};

use axum::extract::DefaultBodyLimit;
use loco_rs::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::{db_err, unprocessable_entity as unprocessable};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plan_dependencies, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::mspdi::{self, Parsed};
use crate::mspdi_batch::{self as rules, Mapped, Resolved, Terms};

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
    /// A project field (its alias, name or id) to the key of a task custom field.
    #[serde(default)]
    field_map: BTreeMap<String, String>,
    /// Declare the mapped custom fields that do not exist yet (type from the values: integer, date, yes/no or text).
    #[serde(default)]
    create_fields: bool,
    /// Who each pool resource is, by its name in the files. Without an entry a resource's rates and calendar are
    /// reported and not stored: a name is not an identity.
    #[serde(default)]
    resource_map: BTreeMap<String, ResourceIdentity>,
    /// The date the imported rates take effect (default: today in the deployment's time zone).
    #[serde(default)]
    rates_effective_from: Option<chrono::NaiveDate>,
}

/// Who a resource of the files is.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ResourceIdentity {
    /// A person (`person:` or `worker:` URN): their rates go in their own rate tables.
    #[serde(default)]
    person_ref: Option<String>,
    /// A role: the rates go in the role's tables, for any person without their own.
    #[serde(default)]
    role: Option<String>,
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
    will_create: &HashMap<String, &'static str>,
) -> (Vec<(i64, String, Value)>, Vec<Value>) {
    let (mut accepted, mut rejected) = (Vec::new(), Vec::new());
    for m in mapped {
        if let Some(ty) = will_create.get(&m.key) {
            accepted.push((m.uid, m.key.clone(), typed(&m.raw, ty)));
            continue;
        }
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

/// A raw value as the JSON of a field of type `ty` (the types [`rules::infer_type`] gives).
fn typed(raw: &str, ty: &str) -> Value {
    let t = raw.trim();
    match ty {
        "integer" => t
            .parse::<i64>()
            .map_or_else(|_| Value::from(t), Value::from),
        "bool" => Value::Bool(matches!(t.to_lowercase().as_str(), "yes" | "true")),
        "date" => Value::from(t.get(..10).unwrap_or(t)),
        _ => Value::from(t),
    }
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

/// Declare the mapped custom fields that do not exist yet, when the caller asked for it. Returns the report and
/// the keys that will exist (so a dry run can count their values as acceptable).
async fn declare_fields(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    b: &BatchBody,
    mapped: &[&Vec<Mapped>],
    parsed: &[Parsed],
    dry_run: bool,
) -> Result<(Vec<Value>, HashMap<String, &'static str>)> {
    let mut report = Vec::new();
    let mut will_create: HashMap<String, &'static str> = HashMap::new();
    let mut keys: Vec<&String> = mapped
        .iter()
        .flat_map(|m| m.iter().map(|x| &x.key))
        .collect();
    keys.sort();
    keys.dedup();
    for key in keys {
        if super::custom_fields::is_declared(ctx, "task", key).await? {
            continue;
        }
        if !b.create_fields {
            report.push(json!({ "key": key, "outcome": "missing",
                "reason": "no such task custom field; send create_fields: true to declare it" }));
            continue;
        }
        let raws: Vec<&str> = mapped
            .iter()
            .flat_map(|m| m.iter().filter(|x| &x.key == key).map(|x| x.raw.as_str()))
            .collect();
        let ty = rules::infer_type(raws);
        let label = b
            .field_map
            .iter()
            .find(|(_, k)| *k == key)
            .and_then(|(source, _)| {
                let s = source.trim().to_lowercase();
                parsed.iter().flat_map(|p| &p.fields).find_map(|f| {
                    (f.alias.as_deref().is_some_and(|a| a.to_lowercase() == s)
                        || f.name.to_lowercase() == s
                        || f.field_id == s)
                        .then(|| f.alias.clone().unwrap_or_else(|| f.name.clone()))
                })
            })
            .unwrap_or_else(|| key.clone());
        if dry_run {
            // The same checks the route makes, so a dry run does not promise what the job would refuse.
            let problems = crate::custom_fields::def_problems(key, &label, ty, "task", &[], &[]);
            if problems.is_empty() {
                will_create.insert(key.clone(), ty);
                report.push(
                    json!({ "key": key, "outcome": "would_declare", "type": ty, "label": label }),
                );
            } else {
                report.push(
                    json!({ "key": key, "outcome": "refused", "reason": problems.join("; ") }),
                );
            }
            continue;
        }
        match super::custom_fields::declare(
            ctx,
            caller.actor(),
            key,
            &label,
            ty,
            "task",
            &[],
            false,
            &[],
        )
        .await
        {
            Ok(_) => report
                .push(json!({ "key": key, "outcome": "declared", "type": ty, "label": label })),
            Err(e) => {
                report.push(json!({ "key": key, "outcome": "refused", "reason": message_of(&e) }));
            }
        }
    }
    Ok((report, will_create))
}

/// Create the links between files that the job could place (a dependency from a task of one plan to a task of
/// another), skipping a pair that is already linked and refusing one that would close a cycle.
async fn link_projects(
    ctx: &AppContext,
    plan_pids: &[Option<Uuid>],
    resolved: &[Resolved],
) -> Result<(usize, usize, Vec<Value>)> {
    let mut pids: HashMap<(usize, i64), Uuid> = HashMap::new();
    for (idx, plan) in plan_pids.iter().enumerate() {
        let Some(plan) = plan else { continue };
        for t in tasks::Entity::find()
            .filter(tasks::Column::PlanPid.eq(*plan))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
        {
            if let Some(uid) = t
                .external_ref
                .as_deref()
                .and_then(|e| e.strip_prefix("mspdi:"))
                .and_then(|u| u.parse().ok())
            {
                pids.insert((idx, uid), t.pid);
            }
        }
    }
    let mut edges: Vec<(Uuid, Uuid)> = plan_dependencies::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|d| (d.predecessor_pid, d.successor_pid))
        .collect();
    edges.extend(
        task_relations::Entity::find()
            .filter(task_relations::Column::DeletedAt.is_null())
            .filter(task_relations::Column::Kind.eq("precedes"))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| (r.from_task_pid, r.to_task_pid)),
    );
    let (mut created, mut present, mut skipped) = (0, 0, Vec::new());
    for l in resolved {
        let (Some(&pred), Some(&succ)) = (
            pids.get(&(l.predecessor_project, l.predecessor)),
            pids.get(&(l.successor_project, l.successor)),
        ) else {
            skipped.push(
                json!({ "successor": l.successor, "predecessor": l.predecessor,
                "reason": "a task of the link was not imported" }),
            );
            continue;
        };
        if edges.contains(&(pred, succ)) {
            present += 1;
            continue;
        }
        if crate::visibility::would_create_cycle(&edges, pred, succ) {
            skipped.push(
                json!({ "successor": l.successor, "predecessor": l.predecessor,
                "reason": "the link would close a cycle; skipped" }),
            );
            continue;
        }
        plan_dependencies::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            predecessor_pid: ActiveValue::set(pred),
            successor_pid: ActiveValue::set(succ),
            lag_days: ActiveValue::set(i32::try_from(l.lag_days).unwrap_or(0)),
            kind: ActiveValue::set(
                match l.kind {
                    crate::cpm::Kind::Fs => "fs",
                    crate::cpm::Kind::Ss => "ss",
                    crate::cpm::Kind::Ff => "ff",
                    crate::cpm::Kind::Sf => "sf",
                }
                .to_string(),
            ),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(db_err)?;
        edges.push((pred, succ));
        created += 1;
    }
    Ok((created, present, skipped))
}

/// What storing one part of a resource's terms came to.
fn part(what: &str, outcome: &str, reason: &str) -> Value {
    json!({ "what": what, "outcome": outcome, "reason": reason })
}

/// Store each resource's rates and calendar where the caller said who the resource is. Everything else is reported
/// with the reason it was not stored.
#[expect(clippy::too_many_lines)] // one linear pass per resource: decide, then write each part
async fn store_terms(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    b: &BatchBody,
    terms: &[Terms],
    dry_run: bool,
) -> Result<Vec<Value>> {
    let effective = b
        .rates_effective_from
        .unwrap_or_else(crate::tz::deployment_today);
    let finance = super::cost_rates::may_see_rates(caller);
    let identities: HashMap<String, &ResourceIdentity> = b
        .resource_map
        .iter()
        .map(|(k, v)| (rules::normalize(k), v))
        .collect();
    let mut out = Vec::new();
    for t in terms {
        let mut parts: Vec<Value> = Vec::new();
        if !t.conflicts.is_empty() {
            parts.push(part(
                "terms",
                "not_stored",
                &format!(
                    "the files disagree on: {}; settle it in the files or set it by hand",
                    t.conflicts.join(", ")
                ),
            ));
        }
        parts.extend(t.notes.iter().map(|n| part("rate", "not_converted", n)));
        if t.cost_per_use.is_some() {
            parts.push(part(
                "cost_per_use",
                "not_stored",
                "a per-use cost has no place in the rate tables",
            ));
        }
        let id = identities.get(&rules::normalize(&t.name));
        let owner = id.and_then(|i| match (&i.person_ref, &i.role) {
            (Some(p), None) if super::governance::valid_ref(p, &["person", "worker"]) => {
                Some((Some(p.as_str()), None))
            }
            (None, Some(r)) if !r.trim().is_empty() && r.chars().count() <= 60 => {
                Some((None, Some(r.trim())))
            }
            _ => None,
        });
        for (what, table, value) in [
            ("standard_rate", "standard", t.standard),
            ("overtime_rate", "overtime", t.overtime),
        ] {
            let Some(value) = value else { continue };
            let text = format!("{value} minor units per hour from {effective}");
            parts.push(match (owner, &t.currency, finance) {
                (None, ..) => part(what, "not_stored", "no resource_map entry says who this resource is (a name is not an identity), or it names both or neither of person_ref and role"),
                (_, None, _) => part(what, "not_stored", "the files give no currency, or disagree on it"),
                (_, _, false) => part(what, "not_stored", "storing rates needs finance access"),
                (Some(_), Some(_), true) if dry_run => part(what, "would_store", &text),
                (Some(owner), Some(currency), true) => {
                    match super::cost_rates::import_rate(ctx, caller.actor(), owner, table, currency, effective, value).await {
                        Ok(super::cost_rates::RateOutcome::Stored) => part(what, "stored", &text),
                        Ok(super::cost_rates::RateOutcome::AlreadyThere) => part(what, "already_there", "the table already holds this rate"),
                        Ok(super::cost_rates::RateOutcome::DiffersFromExisting) => part(what, "not_stored", "the table already has a different rate from that date; rates are append-only"),
                        Err(e) => part(what, "failed", &message_of(&e)),
                    }
                }
            });
        }
        if let Some(cal) = &t.calendar {
            let own = id
                .and_then(|i| i.person_ref.as_deref())
                .filter(|p| Some(*p) == caller.actor());
            let text = format!(
                "weekdays {}, {} holiday(s)",
                cal.weekdays,
                cal.holidays.len()
            );
            parts.push(match own {
                None => part(
                    "calendar",
                    "not_stored",
                    &format!("a person's calendar is theirs to set (PUT /api/me/calendar); {text}"),
                ),
                Some(_) if dry_run => part("calendar", "would_store", &text),
                Some(person)
                    if crate::calendars::problems(&cal.weekdays, &cal.holidays).is_empty() =>
                {
                    match super::calendars::set_person_calendar(
                        ctx,
                        person,
                        &cal.weekdays,
                        &cal.holidays,
                        caller.actor(),
                    )
                    .await
                    {
                        Ok(()) => part("calendar", "stored", &text),
                        Err(e) => part("calendar", "failed", &message_of(&e)),
                    }
                }
                Some(_) => part(
                    "calendar",
                    "not_stored",
                    "no business day, or dates out of range",
                ),
            });
        }
        out.push(json!({ "resource": t.name, "parts": parts }));
    }
    Ok(out)
}

/// Store each task's fixed cost from one imported file. Costs that follow from rates and work (a resource's or an
/// assignment's cost) are not carried: they are computed here from the rate tables.
async fn store_costs(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan_pid: Option<Uuid>,
    parsed: &Parsed,
    dry_run: bool,
) -> Result<Value> {
    if parsed.fixed_costs.is_empty() {
        return Ok(json!({ "fixed_costs_in_file": 0 }));
    }
    let Some(currency) = parsed.currency.as_deref().filter(|c| c.len() == 3) else {
        return Ok(
            json!({ "fixed_costs_in_file": parsed.fixed_costs.len(), "stored": 0,
            "reason": "the file gives no currency code, so a fixed cost has no currency" }),
        );
    };
    if !super::cost_rates::may_see_rates(caller) {
        return Ok(
            json!({ "fixed_costs_in_file": parsed.fixed_costs.len(), "stored": 0,
            "reason": "storing costs needs finance access" }),
        );
    }
    let mut stored = 0;
    if let (false, Some(plan)) = (dry_run, plan_pid) {
        for c in &parsed.fixed_costs {
            let Some(task) = tasks::Entity::find()
                .filter(tasks::Column::PlanPid.eq(plan))
                .filter(tasks::Column::ExternalRef.eq(format!("mspdi:{}", c.uid)))
                .filter(tasks::Column::DeletedAt.is_null())
                .one(&ctx.db)
                .await
                .map_err(db_err)?
            else {
                continue;
            };
            if super::cost_rates::import_fixed_cost(
                ctx,
                caller.actor(),
                task.pid,
                "Fixed cost (imported from MS Project)",
                c.amount_minor,
                &currency.to_ascii_uppercase(),
                c.accrual,
            )
            .await?
            {
                stored += 1;
            }
        }
    }
    Ok(json!({ "fixed_costs_in_file": parsed.fixed_costs.len(), "stored": stored }))
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
    let terms = rules::merge_terms(&named);
    let (resolved_links, unresolved_links) = rules::resolve_cross_links(&named);
    let field_results: Vec<(Vec<Mapped>, Vec<rules::LeftOut>)> = parsed
        .iter()
        .map(|p| rules::map_fields(p, &b.field_map))
        .collect();
    let mapped_all: Vec<&Vec<Mapped>> = field_results.iter().map(|(m, _)| m).collect();
    let (fields_report, will_create) =
        declare_fields(&ctx, &caller, &b, &mapped_all, &parsed, dry_run).await?;

    let mut results: Vec<Value> = Vec::new();
    let mut failed: Option<usize> = None;
    let mut created_plans = 0usize;
    let mut first_plan: Option<Uuid> = None;
    let mut plan_pids: Vec<Option<Uuid>> = vec![None; b.projects.len()];
    for (i, p) in b.projects.iter().enumerate() {
        let label = &labels[i];
        if failed.is_some() {
            results.push(json!({ "project": label, "outcome": "not_attempted" }));
            continue;
        }
        let (mapped, left_out) = &field_results[i];
        let (accepted, rejected) = resolve_values(&ctx, mapped, &will_create).await;
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
            let costs = store_costs(&ctx, &caller, plan_pid, &parsed[i], dry_run).await?;
            Ok(json!({
                "project": label,
                "outcome": if dry_run { "would_import" } else { "imported" },
                "plan_pid": plan_pid,
                "plan_created": created,
                "plan_would_be_created": plan_pid.is_none(),
                "report": report,
                "costs": costs,
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
                plan_pids[i] = v["plan_pid"].as_str().and_then(|s| Uuid::parse_str(s).ok());
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
    let links_report = if dry_run {
        json!({ "would_create": resolved_links.len(), "unresolved": unresolved_json(&unresolved_links) })
    } else if failed.is_some() {
        json!({ "created": 0, "reason": "a project failed, so links between the files were not made; ask again once it is fixed",
            "unresolved": unresolved_json(&unresolved_links) })
    } else {
        let (created, present, skipped) = link_projects(&ctx, &plan_pids, &resolved_links).await?;
        json!({ "created": created, "already_present": present, "skipped": skipped,
            "unresolved": unresolved_json(&unresolved_links) })
    };
    let terms_report = if failed.is_some() && !dry_run {
        json!([])
    } else {
        json!(store_terms(&ctx, &caller, &b, &terms, dry_run).await?)
    };
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
        "custom_field_definitions": fields_report,
        "cross_project_links": links_report,
        "resource_terms": terms_report,
        "left_out": [
            "a link to a file that is not in this job, or to a task that file does not have (listed under cross_project_links.unresolved)",
            "a resource's per-use cost, week/month/year rates, skills and any cost that follows from rates and work (computed here from the rate tables)",
            "custom fields with no entry in field_map, and values the service's field type refuses (listed per project)",
        ],
    }))
}

fn unresolved_json(items: &[rules::Unresolved]) -> Vec<Value> {
    items
        .iter()
        .map(|u| json!({ "project": u.project, "successor": u.successor, "reason": u.reason }))
        .collect()
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
