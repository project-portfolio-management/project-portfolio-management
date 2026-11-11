//! `DevOps` event ingestion and the delivery metrics derived from it.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::super::insights::item_ref;
use super::{conditional, refuse};
use crate::models::_entities::{devops_events, plans};
use crate::models::audit_logs::Model as AuditModel;

// ─── DevOps events ──────────────────────────────────────────────────────────

/// `POST /api/devops/events` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DevopsEventPayload {
    /// Plan the event belongs to; must be a live plan, else 404.
    plan_pid: Uuid,
    /// Event kind, one of `DEVOPS_EVENT_KINDS` (deploy, incident or recovery).
    kind: String,
    /// Target environment name; required (non-empty) for a deploy and optional otherwise.
    #[serde(default)]
    environment: Option<String>,
    /// Free-text version or release label that was deployed; absent when not supplied.
    #[serde(default)]
    version: Option<String>,
    /// Free-text external reference (build, ticket or change id); stored as given.
    #[serde(default)]
    reference: Option<String>,
    /// Incident event this recovery resolves; required on a recovery and must name an ingested incident.
    #[serde(default)]
    incident_pid: Option<Uuid>,
    /// Deploy event declared as the cause of this incident; only valid on incidents. Absent means causation is undeclared, never guessed.
    #[serde(default)]
    caused_by_deploy_pid: Option<Uuid>,
    /// When the event happened (UTC); absent means now, at ingestion.
    #[serde(default)]
    occurred_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// The devops event with this pid, if any.
async fn find_event(ctx: &AppContext, pid: Uuid) -> Result<Option<devops_events::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM devops_events WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Whether `event` is one of `kind`, on a plan the caller can read. A hidden one gets the answer of one that does not exist.
async fn is_readable_event(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    event: Option<devops_events::Model>,
    kind: &str,
) -> bool {
    match event {
        Some(e) if e.kind == kind => {
            crate::controllers::plan_access::require_readable(ctx, caller, e.plan_pid)
                .await
                .is_ok()
        }
        _ => false,
    }
}

/// `POST /api/devops/events` — ingest one deploy / incident /
/// recovery event. A `deploy` requires an `environment`; a `recovery`
/// must reference its `incident`; an `incident` may declare the
/// deploy that caused it. The DORA-style metrics derive **only** from
/// events ingested here.
#[debug_handler]
pub(super) async fn ingest_devops_event(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<DevopsEventPayload>,
) -> Result<Response> {
    if !rules::DEVOPS_EVENT_KINDS.contains(&payload.kind.as_str()) {
        return Err(refuse(&format!(
            "kind must be one of {:?}",
            rules::DEVOPS_EVENT_KINDS
        )));
    }
    let item: Uuid =
        sqlx::query_scalar("SELECT pid FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(payload.plan_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    crate::controllers::plan_access::require_readable(&ctx, &caller, item).await?;
    match payload.kind.as_str() {
        "deploy" if payload.environment.as_deref().is_none_or(str::is_empty) => {
            return Err(refuse("a deploy event requires an environment"));
        }
        "recovery" => {
            let Some(incident_pid) = payload.incident_pid else {
                return Err(refuse("a recovery event must reference its incident_pid"));
            };
            let incident = find_event(&ctx, incident_pid).await?;
            if !is_readable_event(&ctx, &caller, incident, "incident").await {
                return Err(refuse("incident_pid must name an ingested incident event"));
            }
        }
        _ => {}
    }
    if payload.caused_by_deploy_pid.is_some() && payload.kind != "incident" {
        return Err(refuse("caused_by_deploy_pid belongs on incident events"));
    }
    if let Some(deploy_pid) = payload.caused_by_deploy_pid {
        let deploy = find_event(&ctx, deploy_pid).await?;
        if !is_readable_event(&ctx, &caller, deploy, "deploy").await {
            return Err(refuse(
                "caused_by_deploy_pid must name an ingested deploy event",
            ));
        }
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: devops_events::Model = sqlx::query_as(
        "INSERT INTO devops_events
             (pid, plan_pid, kind, environment, version, reference, incident_pid, caused_by_deploy_pid,
              occurred_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(item)
    .bind(&payload.kind)
    .bind(payload.environment.as_deref())
    .bind(payload.version.as_deref())
    .bind(payload.reference.as_deref())
    .bind(payload.incident_pid)
    .bind(payload.caused_by_deploy_pid)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(
        payload.occurred_at.unwrap_or_else(chrono::Utc::now),
    ))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "devops_event_ingested",
        caller.actor(),
        Some(serde_json::json!({ "kind": row.kind })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// Query for the devops metrics window.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DevopsMetricsQuery {
    /// Trailing months (default 3, cap 24).
    months: Option<u32>,
}

/// `GET /api/devops/metrics?months=` — DORA-style metrics derived
/// **only** from ingested events: deploys per month (+ per
/// environment), incidents per month, MTTR over linked
/// incident→recovery pairs (unresolved incidents counted, never
/// timed), and change-failure over **declared-cause** incidents only
/// (undeclared causation is not guessed).
#[debug_handler]
pub(super) async fn devops_metrics(
    axa::extract::Query(query): axa::extract::Query<DevopsMetricsQuery>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let months = i64::from(query.months.unwrap_or(3).clamp(1, 24));
    let now = chrono::Utc::now();
    let window_start = now - chrono::Days::new(u64::try_from(months * 31).unwrap_or(93));
    let rows = scope.keep(
        sqlx::query_as::<_, devops_events::Model>("SELECT * FROM devops_events")
            .fetch_all(crate::db::pool(&ctx))
            .await?,
    );
    let in_window = |at: chrono::DateTime<chrono::Utc>| at >= window_start && at <= now;

    let mut deploys_by_month: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut deploys_by_environment: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut incidents_by_month: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut deploy_count = 0usize;
    let mut incident_count = 0usize;
    let mut declared_cause_incidents = 0usize;
    for row in &rows {
        let at = row.occurred_at.to_utc();
        if !in_window(at) {
            continue;
        }
        let month = at.format("%Y-%m").to_string();
        match row.kind.as_str() {
            "deploy" => {
                deploy_count += 1;
                *deploys_by_month.entry(month).or_default() += 1;
                if let Some(environment) = row.environment.as_deref() {
                    *deploys_by_environment
                        .entry(environment.to_string())
                        .or_default() += 1;
                }
            }
            "incident" => {
                incident_count += 1;
                *incidents_by_month.entry(month).or_default() += 1;
                if row.caused_by_deploy_pid.is_some() {
                    declared_cause_incidents += 1;
                }
            }
            _ => {}
        }
    }

    // MTTR: linked incident→recovery pairs whose incident is in-window.
    let recovery_of: std::collections::BTreeMap<Uuid, chrono::DateTime<chrono::Utc>> = rows
        .iter()
        .filter(|row| row.kind == "recovery")
        .filter_map(|row| row.incident_pid.map(|pid| (pid, row.occurred_at.to_utc())))
        .collect();
    let mut recovery_hours: Vec<i64> = Vec::new();
    let mut unresolved = 0usize;
    for row in rows.iter().filter(|row| row.kind == "incident") {
        let at = row.occurred_at.to_utc();
        if !in_window(at) {
            continue;
        }
        match recovery_of.get(&row.pid) {
            Some(recovered_at) => recovery_hours.push((*recovered_at - at).num_hours()),
            None => unresolved += 1,
        }
    }
    recovery_hours.sort_unstable();
    let median_recovery_hours = if recovery_hours.is_empty() {
        None
    } else {
        Some(recovery_hours[recovery_hours.len() / 2])
    };

    #[expect(clippy::cast_precision_loss)] // display ratio, not money math
    let change_failure_rate = if deploy_count == 0 {
        None
    } else {
        Some(declared_cause_incidents as f64 / deploy_count as f64)
    };
    let body = serde_json::json!({
        "as_of": now,
        "window_months": months,
        "derivation": "derived only from ingested devops events; MTTR = linked \
                       incident->recovery pairs (unresolved counted, never timed); \
                       change_failure_rate = declared-cause incidents / deploys \
                       (undeclared causation is not guessed)",
        "deploys": deploy_count,
        "deploys_by_month": deploys_by_month,
        "deploys_by_environment": deploys_by_environment,
        "incidents": incident_count,
        "incidents_by_month": incidents_by_month,
        "resolved_incidents": recovery_hours.len(),
        "unresolved_incidents": unresolved,
        "median_recovery_hours": median_recovery_hours,
        "declared_cause_incidents": declared_cause_incidents,
        "change_failure_rate": change_failure_rate,
    });
    conditional(&headers, &body)
}

/// `GET /api/devops/releases` — the release register: ingested deploy
/// events newest first (version / environment / item), cap 200.
#[debug_handler]
pub(super) async fn devops_releases(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let rows: Vec<devops_events::Model> = sqlx::query_as(
        "SELECT * FROM devops_events WHERE kind = 'deploy' ORDER BY occurred_at DESC LIMIT 200",
    )
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let rows = scope.keep(rows);
    let releases: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "pid": row.pid,
                "occurred_at": row.occurred_at.to_utc(),
                "environment": row.environment,
                "version": row.version,
                "reference": row.reference,
                "item": by_pid.get(&row.plan_pid).map(|i| item_ref(i)),
            })
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "releases": releases,
    });
    conditional(&headers, &body)
}
