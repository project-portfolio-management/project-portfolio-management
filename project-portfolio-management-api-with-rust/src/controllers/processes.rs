//! Multi-step approval processes (PL-2). Rules in [`crate::processes`]; this is the wiring.
//!
//! * A **process** is a named list of steps (`approve`, `todo`, `notify`). Each edit
//!   adds a new immutable version; the one a process is on is `current_version`.
//! * Starting one makes an **execution** pinned to that version, so editing the
//!   process mid-run changes nothing for a run already under way.
//! * An execution keeps **all its state in the database** (which step it is on, an
//!   append-only log of what happened), so it survives a restart; `resume` reconciles
//!   one with the approval it is waiting on.
//! * A step starts only after the one before it completes. A rejected approval ends
//!   the execution `rejected` and **skips, and logs, every step after it**.

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{ConnectionTrait, QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use super::discussion::resolve_subject;
use crate::approvals::is_approver;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::{
    approval_requests, process_execution_logs, process_executions, process_versions, processes,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;
use crate::processes::{self as rules, Kind, Status, Step};

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn conflict(message: &str) -> Error {
    Error::CustomError(StatusCode::CONFLICT, ErrorDetail::new("conflict", message))
}

fn forbidden(message: &str) -> Error {
    Error::CustomError(
        StatusCode::FORBIDDEN,
        ErrorDetail::new("forbidden", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn is_admin(claims: &authentication_verifier::Claims) -> bool {
    claims
        .attrs
        .get("access")
        .is_some_and(|v| v.iter().any(|a| a == "admin"))
}

// ───────────────────────── definitions ─────────────────────────

#[derive(Debug, Deserialize)]
struct DefinitionBody {
    #[serde(default)]
    name: Option<String>,
    steps: Vec<Step>,
}

fn check_definition(name: Option<&str>, steps: &[Step]) -> Result<()> {
    let mut problems = rules::problems(steps);
    if name.is_some_and(|n| n.trim().is_empty() || n.chars().count() > 120) {
        problems.push("name is 1 to 120 characters".to_string());
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&problems.join("; ")))
    }
}

async fn find_process(ctx: &AppContext, pid: &str) -> Result<processes::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    processes::Entity::find()
        .filter(processes::Column::Pid.eq(pid))
        .filter(processes::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn version_steps<C: ConnectionTrait>(
    conn: &C,
    process: Uuid,
    version: i32,
) -> Result<Vec<Step>> {
    let row = process_versions::Entity::find()
        .filter(process_versions::Column::ProcessPid.eq(process))
        .filter(process_versions::Column::Version.eq(version))
        .one(conn)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    serde_json::from_value(row.steps).map_err(|e| Error::Any(e.into()))
}

/// `POST /api/processes` — define a process (version 1).
#[debug_handler]
async fn create_process(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Json(b): Json<DefinitionBody>,
) -> Result<Response> {
    let name = b.name.clone().unwrap_or_default();
    if name.trim().is_empty() {
        return Err(unprocessable("name is required"));
    }
    check_definition(Some(&name), &b.steps)?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let p = processes::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        name: ActiveValue::set(name.trim().to_string()),
        current_version: ActiveValue::set(1),
        created_by: ActiveValue::set(Some(claims.sub.clone())),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    add_version(&txn, p.pid, 1, &b.steps, &claims.sub).await?;
    AuditModel::record(
        &txn,
        p.pid,
        "process_defined",
        Some(&claims.sub),
        Some(serde_json::json!({ "name": p.name, "steps": b.steps.len() })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    process_view(&ctx, &p).await
}

async fn add_version<C: ConnectionTrait>(
    conn: &C,
    process: Uuid,
    version: i32,
    steps: &[Step],
    by: &str,
) -> Result<()> {
    process_versions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        process_pid: ActiveValue::set(process),
        version: ActiveValue::set(version),
        steps: ActiveValue::set(serde_json::to_value(steps).unwrap_or_default()),
        created_by: ActiveValue::set(Some(by.to_string())),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map_err(db_err)?;
    Ok(())
}

async fn process_view(ctx: &AppContext, p: &processes::Model) -> Result<Response> {
    let versions = process_versions::Entity::find()
        .filter(process_versions::Column::ProcessPid.eq(p.pid))
        .order_by_asc(process_versions::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(serde_json::json!({
        "pid": p.pid, "name": p.name, "current_version": p.current_version, "created_by": p.created_by,
        "steps": versions.iter().find(|v| v.version == p.current_version).map(|v| v.steps.clone()),
        "versions": versions.iter().map(|v| serde_json::json!({ "version": v.version, "steps": v.steps, "created_by": v.created_by, "created_at": v.created_at })).collect::<Vec<_>>(),
    }))
}

/// `GET /api/processes` — the live definitions.
#[debug_handler]
async fn list_processes(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = processes::Entity::find()
        .filter(processes::Column::DeletedAt.is_null())
        .order_by_asc(processes::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(|p| serde_json::json!({ "pid": p.pid, "name": p.name, "current_version": p.current_version })).collect::<Vec<_>>())
}

/// `GET /api/processes/{pid}` — a definition with every version.
#[debug_handler]
async fn get_process(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let p = find_process(&ctx, &pid).await?;
    process_view(&ctx, &p).await
}

/// `PUT /api/processes/{pid}` — edit: adds a **new version**; running executions keep theirs.
/// Only the definer or an admin.
#[debug_handler]
async fn update_process(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<DefinitionBody>,
) -> Result<Response> {
    let p = find_process(&ctx, &pid).await?;
    if p.created_by.as_deref() != Some(claims.sub.as_str()) && !is_admin(&claims) {
        return Err(forbidden(
            "only the person who defined this process, or an admin, may change it",
        ));
    }
    check_definition(b.name.as_deref(), &b.steps)?;
    let next = p.current_version + 1;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    add_version(&txn, p.pid, next, &b.steps, &claims.sub).await?;
    let mut a: processes::ActiveModel = p.into();
    a.current_version = ActiveValue::set(next);
    if let Some(n) = &b.name {
        a.name = ActiveValue::set(n.trim().to_string());
    }
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "process_versioned",
        Some(&claims.sub),
        Some(serde_json::json!({ "version": next, "steps": b.steps.len() })),
    )
    .await
    .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    process_view(&ctx, &row).await
}

/// `DELETE /api/processes/{pid}` — retire a definition (running executions continue).
#[debug_handler]
async fn delete_process(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let p = find_process(&ctx, &pid).await?;
    if p.created_by.as_deref() != Some(claims.sub.as_str()) && !is_admin(&claims) {
        return Err(forbidden(
            "only the person who defined this process, or an admin, may retire it",
        ));
    }
    let pid = p.pid;
    let mut a: processes::ActiveModel = p.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, pid, "process_retired", Some(&claims.sub), None)
        .await
        .ok();
    format::empty_json()
}

// ───────────────────────── the runner ─────────────────────────

async fn log<C: ConnectionTrait>(
    conn: &C,
    execution: Uuid,
    step: Option<usize>,
    event: &str,
    actor: Option<&str>,
    note: Option<&str>,
) -> Result<()> {
    process_execution_logs::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        execution_pid: ActiveValue::set(execution),
        step_index: ActiveValue::set(step.and_then(|s| i32::try_from(s).ok())),
        event: ActiveValue::set(event.to_string()),
        actor: ActiveValue::set(actor.map(str::to_string)),
        note: ActiveValue::set(note.map(str::to_string)),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Lock an execution row for the rest of the transaction (so two completions cannot race).
async fn lock<C: ConnectionTrait>(conn: &C, pid: Uuid) -> Result<process_executions::Model> {
    use sea_orm::QuerySelect;
    process_executions::Entity::find()
        .filter(process_executions::Column::Pid.eq(pid))
        .lock_exclusive()
        .one(conn)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

async fn finish<C: ConnectionTrait>(
    conn: &C,
    ex: process_executions::Model,
    status: Status,
    note: Option<&str>,
) -> Result<process_executions::Model> {
    let mut a: process_executions::ActiveModel = ex.into();
    a.status = ActiveValue::set(status.token().to_string());
    a.finished_at = ActiveValue::set(Some(Utc::now().into()));
    a.note = ActiveValue::set(note.map(str::to_string));
    a.update(conn).await.map_err(db_err)
}

/// End an execution early: every step after `from` (and `from` itself when
/// `include_from`) is skipped, **and logged**, then the final event.
struct Ending<'a> {
    /// The step it ended at.
    from: usize,
    /// Whether that step is itself skipped (it never completed).
    include_from: bool,
    status: Status,
    event: &'a str,
    actor: Option<&'a str>,
    note: Option<&'a str>,
}

async fn end_early<C: ConnectionTrait>(
    conn: &C,
    ex: process_executions::Model,
    steps: &[Step],
    e: Ending<'_>,
) -> Result<process_executions::Model> {
    let Ending {
        from,
        include_from,
        status,
        event,
        actor,
        note,
    } = e;
    let start = if include_from { from } else { from + 1 };
    for i in start..steps.len() {
        log(
            conn,
            ex.pid,
            Some(i),
            "step_skipped",
            actor,
            Some(&format!("the process {}", status.token())),
        )
        .await?;
    }
    log(conn, ex.pid, Some(from), event, actor, note).await?;
    AuditModel::record(
        conn,
        ex.pid,
        &format!("process_{}", status.token()),
        actor,
        note.map(|n| serde_json::json!({ "note": n })),
    )
    .await
    .map_err(Error::Model)?;
    finish(conn, ex, status, note).await
}

/// Run from step `from`: notices go out at once, the first step that needs a person
/// starts and the execution waits on it, and with none left it completes.
async fn run_from<C: ConnectionTrait>(
    conn: &C,
    ex: process_executions::Model,
    steps: &[Step],
    from: usize,
    actor: Option<&str>,
) -> Result<process_executions::Model> {
    let adv = rules::advance(steps, from);
    for &i in &adv.completed {
        log(
            conn,
            ex.pid,
            Some(i),
            "step_started",
            actor,
            Some(&steps[i].title),
        )
        .await?;
        cap_models::notify(
            conn,
            &steps[i].person_ref,
            &ex.subject_kind,
            ex.subject_pid,
            "process_notice",
            &steps[i].title,
        )
        .await?;
        log(conn, ex.pid, Some(i), "step_completed", actor, None).await?;
    }
    if let Some((i, kind)) = adv.waiting {
        log(
            conn,
            ex.pid,
            Some(i),
            "step_started",
            actor,
            Some(&steps[i].title),
        )
        .await?;
        match kind {
            Kind::Approve => {
                approval_requests::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    subject_kind: ActiveValue::set(ex.subject_kind.clone()),
                    subject_pid: ActiveValue::set(ex.subject_pid),
                    requested_by: ActiveValue::set(ex.started_by.clone()),
                    approver_ref: ActiveValue::set(steps[i].person_ref.clone()),
                    message: ActiveValue::set(Some(steps[i].title.clone())),
                    status: ActiveValue::set("pending".to_string()),
                    decision_comment: ActiveValue::set(None),
                    decided_by: ActiveValue::set(None),
                    decided_at: ActiveValue::set(None),
                    process_execution_pid: ActiveValue::set(Some(ex.pid)),
                    process_step: ActiveValue::set(i32::try_from(i).ok()),
                    ..Default::default()
                }
                .insert(conn)
                .await
                .map_err(db_err)?;
                cap_models::notify(
                    conn,
                    &steps[i].person_ref,
                    &ex.subject_kind,
                    ex.subject_pid,
                    "approval_requested",
                    "Your approval has been requested",
                )
                .await?;
            }
            _ => {
                cap_models::notify(
                    conn,
                    &steps[i].person_ref,
                    &ex.subject_kind,
                    ex.subject_pid,
                    "process_todo",
                    &steps[i].title,
                )
                .await?;
            }
        }
        let mut a: process_executions::ActiveModel = ex.into();
        a.current_step = ActiveValue::set(i32::try_from(i).unwrap_or(i32::MAX));
        return a.update(conn).await.map_err(db_err);
    }
    log(conn, ex.pid, None, "execution_completed", actor, None).await?;
    AuditModel::record(conn, ex.pid, "process_completed", actor, None)
        .await
        .map_err(Error::Model)?;
    let mut a: process_executions::ActiveModel = ex.into();
    a.current_step = ActiveValue::set(i32::try_from(steps.len()).unwrap_or(i32::MAX));
    let ex = a.update(conn).await.map_err(db_err)?;
    finish(conn, ex, Status::Completed, None).await
}

// ───────────────────────── executions ─────────────────────────

#[derive(Debug, Deserialize)]
struct StartBody {
    subject_kind: String,
    subject_pid: String,
}

/// `POST /api/processes/{pid}/start` — run a process on a subject, pinned to its
/// current version. The caller must be able to read the subject.
#[debug_handler]
async fn start(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<StartBody>,
) -> Result<Response> {
    let process = find_process(&ctx, &pid).await?;
    let caller = MaybeAuthUser(Some(claims.clone()));
    if !crate::discussion::is_token(crate::discussion::SUBJECT_KINDS, &b.subject_kind) {
        return Err(unprocessable(&format!(
            "unknown subject_kind (allowed: {:?})",
            crate::discussion::SUBJECT_KINDS
        )));
    }
    let subject = resolve_subject(&ctx, &caller, &b.subject_kind, &b.subject_pid).await?;
    let steps = version_steps(&ctx.db, process.pid, process.current_version).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let ex = process_executions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        process_pid: ActiveValue::set(process.pid),
        version: ActiveValue::set(process.current_version),
        subject_kind: ActiveValue::set(b.subject_kind.clone()),
        subject_pid: ActiveValue::set(subject.pid),
        status: ActiveValue::set("running".to_string()),
        current_step: ActiveValue::set(0),
        started_by: ActiveValue::set(claims.sub.clone()),
        finished_at: ActiveValue::set(None),
        note: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    log(
        &txn,
        ex.pid,
        None,
        "execution_started",
        Some(&claims.sub),
        Some(&format!("{} v{}", process.name, process.current_version)),
    )
    .await?;
    AuditModel::record(&txn, ex.pid, "process_started", Some(&claims.sub), Some(serde_json::json!({ "process_pid": process.pid, "version": process.current_version, "subject_kind": b.subject_kind, "subject_pid": subject.pid })))
        .await
        .map_err(Error::Model)?;
    txn.commit().await.map_err(db_err)?;
    // The steps run in a second transaction: if one cannot start, the execution exists
    // and says it failed, rather than vanishing.
    let ex_pid = ex.pid;
    advance_or_fail(&ctx, ex_pid, 0, &steps, Some(&claims.sub)).await?;
    execution_response(&ctx, ex_pid).await
}

/// Run from `from` in one transaction; if it fails, record the failure on the execution.
async fn advance_or_fail(
    ctx: &AppContext,
    ex_pid: Uuid,
    from: usize,
    steps: &[Step],
    actor: Option<&str>,
) -> Result<()> {
    let attempt = async {
        let txn = ctx.db.begin().await.map_err(db_err)?;
        let ex = lock(&txn, ex_pid).await?;
        if Status::is_final(&ex.status) {
            return Ok(());
        }
        run_from(&txn, ex, steps, from, actor).await?;
        txn.commit().await.map_err(db_err)?;
        Ok::<(), Error>(())
    };
    if let Err(e) = attempt.await {
        let txn = ctx.db.begin().await.map_err(db_err)?;
        let ex = lock(&txn, ex_pid).await?;
        if !Status::is_final(&ex.status) {
            let at = usize::try_from(ex.current_step).unwrap_or(0).max(from);
            let why = format!("{e}");
            log(&txn, ex.pid, Some(at), "step_failed", actor, Some(&why)).await?;
            end_early(
                &txn,
                ex,
                steps,
                Ending {
                    from: at,
                    include_from: false,
                    status: Status::Failed,
                    event: "execution_failed",
                    actor,
                    note: Some(&why),
                },
            )
            .await?;
        }
        txn.commit().await.map_err(db_err)?;
    }
    Ok(())
}

async fn execution_json(
    ctx: &AppContext,
    ex: &process_executions::Model,
) -> Result<serde_json::Value> {
    let steps = version_steps(&ctx.db, ex.process_pid, ex.version).await?;
    let logs = process_execution_logs::Entity::find()
        .filter(process_execution_logs::Column::ExecutionPid.eq(ex.pid))
        .order_by_asc(process_execution_logs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let state_of = |i: usize| -> &'static str {
        let latest = logs
            .iter()
            .rev()
            .find(|l| l.step_index == i32::try_from(i).ok() && l.event.starts_with("step_"))
            .map(|l| l.event.as_str());
        match latest {
            Some("step_completed") => "completed",
            Some("step_skipped") => "skipped",
            Some("step_rejected") => "rejected",
            Some("step_failed") => "failed",
            Some("step_started") => "waiting",
            _ => "pending",
        }
    };
    let name = processes::Entity::find()
        .filter(processes::Column::Pid.eq(ex.process_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .map(|p| p.name);
    Ok(serde_json::json!({
        "pid": ex.pid, "process_pid": ex.process_pid, "process_name": name, "version": ex.version,
        "subject_kind": ex.subject_kind, "subject_pid": ex.subject_pid,
        "status": ex.status, "current_step": ex.current_step, "started_by": ex.started_by,
        "finished_at": ex.finished_at, "note": ex.note,
        "steps": steps.iter().enumerate().map(|(i, s)| serde_json::json!({
            "index": i, "kind": s.kind, "person_ref": s.person_ref, "title": s.title, "state": state_of(i) })).collect::<Vec<_>>(),
        "log": logs.iter().map(|l| serde_json::json!({
            "step": l.step_index, "event": l.event, "actor": l.actor, "note": l.note, "at": l.created_at })).collect::<Vec<_>>(),
    }))
}

async fn execution_response(ctx: &AppContext, pid: Uuid) -> Result<Response> {
    let ex = process_executions::Entity::find()
        .filter(process_executions::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    format::json(execution_json(ctx, &ex).await?)
}

async fn find_execution(ctx: &AppContext, pid: &str) -> Result<process_executions::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    process_executions::Entity::find()
        .filter(process_executions::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// Whether the caller may read an execution: who started it, an admin, anyone named in
/// one of its steps, or anyone who can read its subject.
async fn may_read(
    ctx: &AppContext,
    claims: &authentication_verifier::Claims,
    ex: &process_executions::Model,
) -> bool {
    if ex.started_by == claims.sub || is_admin(claims) {
        return true;
    }
    if let Ok(steps) = version_steps(&ctx.db, ex.process_pid, ex.version).await
        && steps
            .iter()
            .any(|s| is_approver(&s.person_ref, Some(&claims.sub)))
    {
        return true;
    }
    resolve_subject(
        ctx,
        &MaybeAuthUser(Some(claims.clone())),
        &ex.subject_kind,
        &ex.subject_pid.to_string(),
    )
    .await
    .is_ok()
}

/// `GET /api/process-executions/{pid}` — an execution, its steps and its log.
#[debug_handler]
async fn get_execution(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let ex = find_execution(&ctx, &pid).await?;
    if !may_read(&ctx, &claims, &ex).await {
        return Err(Error::NotFound);
    }
    format::json(execution_json(&ctx, &ex).await?)
}

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    subject_kind: Option<String>,
    #[serde(default)]
    subject_pid: Option<String>,
}

/// `GET /api/process-executions` — the caller's own runs, or (with `subject_kind` and
/// `subject_pid`) every run on one subject the caller can read.
#[debug_handler]
async fn list_executions(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let mut query =
        process_executions::Entity::find().order_by_desc(process_executions::Column::Id);
    if let Some(s) = &q.status {
        query = query.filter(process_executions::Column::Status.eq(s.as_str()));
    }
    match (&q.subject_kind, &q.subject_pid) {
        (Some(k), Some(p)) => {
            let subject = resolve_subject(&ctx, &MaybeAuthUser(Some(claims.clone())), k, p).await?;
            query = query
                .filter(process_executions::Column::SubjectKind.eq(k.as_str()))
                .filter(process_executions::Column::SubjectPid.eq(subject.pid));
        }
        (None, None) => {
            if !is_admin(&claims) {
                query = query.filter(process_executions::Column::StartedBy.eq(claims.sub.as_str()));
            }
        }
        _ => return Err(unprocessable("give subject_kind and subject_pid together")),
    }
    let rows = query.all(&ctx.db).await.map_err(db_err)?;
    format::json(
        rows.iter()
            .map(|e| serde_json::json!({ "pid": e.pid, "process_pid": e.process_pid, "version": e.version,
                "subject_kind": e.subject_kind, "subject_pid": e.subject_pid, "status": e.status,
                "current_step": e.current_step, "started_by": e.started_by, "finished_at": e.finished_at }))
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug, Deserialize)]
struct NoteBody {
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/process-executions/{pid}/steps/{index}/complete` — the named person says
/// their `todo` step is done; the next step starts.
#[debug_handler]
async fn complete_step(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path((pid, index)): Path<(String, usize)>,
    Json(b): Json<NoteBody>,
) -> Result<Response> {
    let ex0 = find_execution(&ctx, &pid).await?;
    let steps = version_steps(&ctx.db, ex0.process_pid, ex0.version).await?;
    let step = steps.get(index).ok_or(Error::NotFound)?;
    if !is_approver(&step.person_ref, Some(&claims.sub)) {
        return Err(forbidden(
            "only the person this step is for may complete it",
        ));
    }
    if b.note.as_deref().is_some_and(|n| n.chars().count() > 500) {
        return Err(unprocessable("note is at most 500 characters"));
    }
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let ex = lock(&txn, ex0.pid).await?;
    if Status::is_final(&ex.status) {
        return Err(conflict("this execution is over"));
    }
    if usize::try_from(ex.current_step).ok() != Some(index) {
        return Err(conflict(
            "this is not the step the execution is on: a step runs only after the one before it completes",
        ));
    }
    if step.kind != Kind::Todo {
        return Err(unprocessable(
            "only a todo step is completed by hand: an approval is decided, a notice completes itself",
        ));
    }
    log(
        &txn,
        ex.pid,
        Some(index),
        "step_completed",
        Some(&claims.sub),
        b.note.as_deref(),
    )
    .await?;
    let ex_pid = ex.pid;
    let result = run_from(&txn, ex, &steps, index + 1, Some(&claims.sub)).await;
    if result.is_ok() {
        txn.commit().await.map_err(db_err)?;
    } else {
        {
            drop(txn);
            // The completion is real even if the next step could not start: record it, then fail.
            let t2 = ctx.db.begin().await.map_err(db_err)?;
            let ex = lock(&t2, ex_pid).await?;
            log(
                &t2,
                ex.pid,
                Some(index),
                "step_completed",
                Some(&claims.sub),
                b.note.as_deref(),
            )
            .await?;
            t2.commit().await.map_err(db_err)?;
            advance_or_fail(&ctx, ex_pid, index + 1, &steps, Some(&claims.sub)).await?;
        }
    }
    execution_response(&ctx, ex_pid).await
}

/// `POST /api/process-executions/{pid}/cancel` — stop a running execution (who started
/// it, or an admin): a pending approval is withdrawn, every step not yet done is
/// skipped and logged.
#[debug_handler]
async fn cancel(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(b): Json<NoteBody>,
) -> Result<Response> {
    let ex0 = find_execution(&ctx, &pid).await?;
    if ex0.started_by != claims.sub && !is_admin(&claims) {
        return Err(forbidden(
            "only the person who started this execution, or an admin, may cancel it",
        ));
    }
    let steps = version_steps(&ctx.db, ex0.process_pid, ex0.version).await?;
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let ex = lock(&txn, ex0.pid).await?;
    if Status::is_final(&ex.status) {
        return Err(conflict("this execution is already over"));
    }
    let at = usize::try_from(ex.current_step).unwrap_or(0);
    // The step it is waiting on is skipped too (it did not complete), and its approval withdrawn.
    approval_requests::Entity::update_many()
        .col_expr(
            approval_requests::Column::Status,
            sea_orm::sea_query::Expr::value("withdrawn"),
        )
        .filter(approval_requests::Column::ProcessExecutionPid.eq(ex.pid))
        .filter(approval_requests::Column::Status.eq("pending"))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    let ex_pid = ex.pid;
    end_early(
        &txn,
        ex,
        &steps,
        Ending {
            from: at,
            include_from: true,
            status: Status::Cancelled,
            event: "execution_cancelled",
            actor: Some(&claims.sub),
            note: b.note.as_deref(),
        },
    )
    .await?;
    txn.commit().await.map_err(db_err)?;
    execution_response(&ctx, ex_pid).await
}

/// What an approval's outcome does to the execution waiting on it. Called after a
/// decision (or withdrawal); safe to call again — it only acts when the execution is
/// still waiting on exactly that step.
///
/// # Errors
///
/// A database failure; the approval's decision stands, and `resume` can apply it.
pub(crate) async fn on_approval(
    ctx: &AppContext,
    approval: &approval_requests::Model,
) -> Result<()> {
    let (Some(ex_pid), Some(step)) = (approval.process_execution_pid, approval.process_step) else {
        return Ok(());
    };
    let Ok(index) = usize::try_from(step) else {
        return Ok(());
    };
    let ex0 = process_executions::Entity::find()
        .filter(process_executions::Column::Pid.eq(ex_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let Some(ex0) = ex0 else { return Ok(()) };
    let steps = version_steps(&ctx.db, ex0.process_pid, ex0.version).await?;
    let actor = approval.decided_by.clone();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let ex = lock(&txn, ex_pid).await?;
    if Status::is_final(&ex.status) || usize::try_from(ex.current_step).ok() != Some(index) {
        return Ok(()); // already applied, or not the step it is waiting on
    }
    match approval.status.as_str() {
        "approved" => {
            log(
                &txn,
                ex.pid,
                Some(index),
                "step_completed",
                actor.as_deref(),
                approval.decision_comment.as_deref(),
            )
            .await?;
            run_from(&txn, ex, &steps, index + 1, actor.as_deref()).await?;
            txn.commit().await.map_err(db_err)?;
        }
        "rejected" => {
            log(
                &txn,
                ex.pid,
                Some(index),
                "step_rejected",
                actor.as_deref(),
                approval.decision_comment.as_deref(),
            )
            .await?;
            end_early(
                &txn,
                ex,
                &steps,
                Ending {
                    from: index,
                    include_from: false,
                    status: Status::Rejected,
                    event: "execution_rejected",
                    actor: actor.as_deref(),
                    note: approval.decision_comment.as_deref(),
                },
            )
            .await?;
            txn.commit().await.map_err(db_err)?;
        }
        "withdrawn" => {
            end_early(
                &txn,
                ex,
                &steps,
                Ending {
                    from: index,
                    include_from: true,
                    status: Status::Cancelled,
                    event: "execution_cancelled",
                    actor: Some(&approval.requested_by),
                    note: Some("the approval request was withdrawn"),
                },
            )
            .await?;
            txn.commit().await.map_err(db_err)?;
        }
        _ => {}
    }
    Ok(())
}

/// `POST /api/process-executions/{pid}/resume` — reconcile a running execution with the
/// approval it is waiting on (after a crash between a decision and the step that
/// follows it). Idempotent: nothing happens when it is already in step.
#[debug_handler]
async fn resume(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let ex = find_execution(&ctx, &pid).await?;
    if ex.started_by != claims.sub && !is_admin(&claims) {
        return Err(forbidden(
            "only the person who started this execution, or an admin, may resume it",
        ));
    }
    if !Status::is_final(&ex.status) {
        let waiting = approval_requests::Entity::find()
            .filter(approval_requests::Column::ProcessExecutionPid.eq(ex.pid))
            .filter(approval_requests::Column::ProcessStep.eq(ex.current_step))
            .one(&ctx.db)
            .await
            .map_err(db_err)?;
        if let Some(a) = waiting {
            on_approval(&ctx, &a).await?;
        }
    }
    execution_response(&ctx, ex.pid).await
}

/// The process routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/processes", post(create_process))
        .add("/processes", get(list_processes))
        .add("/processes/{pid}", get(get_process))
        .add("/processes/{pid}", put(update_process))
        .add("/processes/{pid}", delete(delete_process))
        .add("/processes/{pid}/start", post(start))
        .add("/process-executions", get(list_executions))
        .add("/process-executions/{pid}", get(get_execution))
        .add(
            "/process-executions/{pid}/steps/{index}/complete",
            post(complete_step),
        )
        .add("/process-executions/{pid}/cancel", post(cancel))
        .add("/process-executions/{pid}/resume", post(resume))
}
