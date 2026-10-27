//! Resource allocations, the skill gap and the capacity rollup.

use super::super::unprocessable;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::allocations;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::validation::{MAX_ARRAY_LEN, MAX_ITEM_LEN};
use crate::visibility as rules;
use crate::workers_client;
use loco_rs::prelude::*;

/// `POST /api/plans/{pid}/allocations` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AllocationPayload {
    /// `person:` / `worker:` `EntityRef` URN. Omit it with `placeholder: true`.
    #[serde(default)]
    person_ref: String,
    /// Role-only demand with no person yet (SC-7): needs a `role`, takes no
    /// `person_ref`, and is resolved to a named person later.
    #[serde(default)]
    placeholder: bool,
    #[serde(default)]
    role: Option<String>,
    /// Percent of the person's capacity (1–100).
    percent: i32,
    #[serde(default)]
    start_date: Option<chrono::NaiveDate>,
    #[serde(default)]
    end_date: Option<chrono::NaiveDate>,
    /// Short skill tags this allocation requires (T-28c). Only the
    /// requirement tags land here — never any resolved skill data.
    #[serde(default)]
    skills_required: Vec<String>,
    /// `committed` (default, firm) or `proposed` (hoped for; PJ-4). A proposal
    /// never counts toward committed utilization or over-allocation.
    #[serde(default)]
    booking: Option<String>,
    /// Asked of the resource manager (PJ-4).
    #[serde(default)]
    requested: bool,
}

#[debug_handler]
pub(super) async fn create_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<AllocationPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.placeholder {
        if !payload.person_ref.is_empty() {
            problems.push("a placeholder takes no person_ref".to_string());
        }
        if payload.role.as_deref().is_none_or(|r| r.trim().is_empty()) {
            problems.push("a placeholder needs a role".to_string());
        }
    } else if !super::super::governance::valid_ref(&payload.person_ref, &["person", "worker"]) {
        problems.push("person_ref must be a person:/worker: URN".to_string());
    }
    if !(1..=100).contains(&payload.percent) {
        problems.push(format!("percent must be 1–100, got {}", payload.percent));
    }
    if let (Some(start), Some(end)) = (payload.start_date, payload.end_date)
        && end < start
    {
        problems.push("end_date is before start_date".to_string());
    }
    if payload
        .booking
        .as_deref()
        .is_some_and(|b| !["proposed", "committed"].contains(&b))
    {
        problems.push("booking must be proposed or committed".to_string());
    }
    if payload.skills_required.len() > MAX_ARRAY_LEN {
        problems.push(format!("skills_required: exceeds {MAX_ARRAY_LEN} entries"));
    }
    for (i, tag) in payload.skills_required.iter().enumerate() {
        if tag.trim().is_empty() {
            problems.push(format!("skills_required[{i}]: must not be blank"));
        } else if tag.chars().count() > MAX_ITEM_LEN {
            problems.push(format!(
                "skills_required[{i}]: exceeds {MAX_ITEM_LEN} characters"
            ));
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    if payload.booking.as_deref() != Some("proposed") {
        super::super::register::guard(&ctx, &caller, item.pid, "booking a firm allocation").await?;
    }
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = allocations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(item.pid),
        person_ref: ActiveValue::set(if payload.placeholder {
            format!(
                "{}{}",
                crate::portfolio_resources::PLACEHOLDER_PREFIX,
                Uuid::new_v4()
            )
        } else {
            payload.person_ref.clone()
        }),
        role: ActiveValue::set(payload.role.clone()),
        percent: ActiveValue::set(payload.percent),
        booking: ActiveValue::set(
            payload
                .booking
                .clone()
                .unwrap_or_else(|| "committed".to_string()),
        ),
        requested: ActiveValue::set(payload.requested),
        start_date: ActiveValue::set(payload.start_date),
        end_date: ActiveValue::set(payload.end_date),
        deleted_at: ActiveValue::set(None),
        skills_required: ActiveValue::set(serde_json::json!(payload.skills_required)),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "allocation_added", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(serde_json::json!({ "pid": row.pid.to_string() }))
}

/// `GET /api/plans/{pid}/skill-gap` — for every allocation on this
/// plan carrying `skills_required` tags, resolve the assigned
/// person's held skills live against the worker service (by
/// `EntityRef`, TTL-cached, T-28c) and report each tag's status.
#[debug_handler]
pub(super) async fn skill_gap(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let rows = vis_models::allocations_for(&ctx.db, item.pid).await?;
    let base_url = std::env::var(workers_client::BASE_URL_ENV).ok();
    let mut findings = Vec::new();
    for row in &rows {
        let tags: Vec<String> =
            serde_json::from_value(row.skills_required.clone()).unwrap_or_default();
        // A placeholder has no person whose skills could be held yet.
        if tags.is_empty() || crate::portfolio_resources::is_placeholder(&row.person_ref) {
            continue;
        }
        let resolved = workers_client::resolve_skills(base_url.as_deref(), &row.person_ref).await;
        let (held, reason) = match &resolved {
            Ok(held) => (Some(held.as_slice()), None),
            Err(reason) => (None, Some(reason.as_str())),
        };
        findings.extend(rules::skill_gap(
            row.pid,
            &row.person_ref,
            &tags,
            held,
            reason,
        ));
    }
    format::json(serde_json::json!({ "plan_pid": item.pid.to_string(), "findings": findings }))
}

/// `GET /api/plans/{pid}/allocations`.
#[debug_handler]
pub(super) async fn list_allocations(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    format::json(vis_models::allocations_for(&ctx.db, item.pid).await?)
}

/// `POST …/allocations/{a_pid}/commit` query.
#[derive(Debug, Deserialize, Default, schemars::JsonSchema)]
pub(crate) struct CommitBody {
    /// Commit even though it would push the person over their capacity.
    #[serde(default)]
    confirm: bool,
}

/// `POST /api/plans/{pid}/allocations/{a_pid}/commit` — turn a proposed booking
/// into a committed one (PJ-4). Allowed to the named person, a resource manager
/// or an admin. If, with this booking firm, the person would be **over their
/// capacity on any day** (PJ-3 availability), the answer is `409` with the
/// numbers, and only `?confirm=true` commits it anyway (audited as an
/// over-commitment).
#[debug_handler]
pub(super) async fn commit_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, a_pid)): Path<(String, String)>,
    Query(body): Query<CommitBody>,
) -> Result<Response> {
    let confirm = body.confirm;
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let a_pid = super::super::parse_pid(&a_pid)?;
    let allocation = vis_models::find_allocation(&ctx.db, a_pid).await?;
    if allocation.plan_pid != item.pid
        || !super::super::availability::may_manage_person(&caller, &allocation.person_ref)
    {
        return Err(Error::NotFound);
    }
    if allocation.booking == "committed" {
        return Err(Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            loco_rs::controller::ErrorDetail::new("conflict", "this booking is already committed"),
        ));
    }
    // The person's committed load with this booking made firm, over its own span.
    let today = crate::tz::deployment_today();
    let from = allocation.start_date.unwrap_or(today);
    let to = allocation.end_date.unwrap_or(from + chrono::Days::new(365));
    let mut allocs: Vec<crate::availability::Alloc> = vis_models::all_allocations(&ctx.db)
        .await?
        .iter()
        .filter(|a| a.person_ref == allocation.person_ref)
        .map(|a| crate::availability::Alloc {
            percent: a.percent,
            start: a.start_date,
            end: a.end_date,
        })
        .collect();
    allocs.push(crate::availability::Alloc {
        percent: allocation.percent,
        start: allocation.start_date,
        end: allocation.end_date,
    });
    let ranges = super::super::availability::ranges_for(&ctx, &allocation.person_ref).await?;
    let load = crate::availability::load(from, to, &allocs, &ranges);
    let over = load.days_over > 0;
    if over && !confirm {
        return Err(Error::CustomError(
            axum::http::StatusCode::CONFLICT,
            loco_rs::controller::ErrorDetail::new(
                "over_capacity",
                format!(
                    "committing this would put the person over capacity on {} calendar day(s) (peak {}% against a capacity of {}%); repeat the request with ?confirm=true to commit anyway. Over: {}",
                    load.days_over,
                    load.peak_demand,
                    load.min_capacity,
                    serde_json::to_string(&load.over).unwrap_or_default()
                ),
            ),
        ));
    }
    let row_pid = allocation.pid;
    let mut active: allocations::ActiveModel = allocation.into();
    active.booking = ActiveValue::set("committed".to_string());
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(
        &txn,
        row_pid,
        "allocation_committed",
        caller.actor(),
        Some(
            serde_json::json!({ "over_capacity": over, "days_over": load.days_over,
                                  "peak_percent": load.peak_demand }),
        ),
    )
    .await
    .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::json(
        serde_json::json!({ "pid": row_pid, "booking": "committed", "over_capacity": over }),
    )
}

/// `DELETE /api/plans/{pid}/allocations/{a_pid}` — soft-delete.
#[debug_handler]
pub(super) async fn delete_allocation(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, a_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let a_pid = super::super::parse_pid(&a_pid)?;
    let allocation = vis_models::find_allocation(&ctx.db, a_pid).await?;
    if allocation.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let row_pid = allocation.pid;
    let mut active: allocations::ActiveModel = allocation.into();
    active.deleted_at = ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row_pid, "allocation_removed", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::empty_json()
}

/// `GET /api/capacity?from=&to=` — the per-person capacity rollup
/// over a window (default: today → today + 30 calendar days). Summed percent
/// over 100 ⇒ over-allocated (PPM-8).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CapacityParams {
    #[serde(default)]
    from: Option<chrono::NaiveDate>,
    #[serde(default)]
    to: Option<chrono::NaiveDate>,
}

#[debug_handler]
pub(super) async fn capacity(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<CapacityParams>,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let today = crate::tz::deployment_today();
    let from = params.from.unwrap_or(today);
    let to = params.to.unwrap_or(from + chrono::Days::new(30));
    if to < from {
        return Err(unprocessable("to is before from"));
    }
    if (to - from).num_days() > crate::availability::MAX_WINDOW_DAYS {
        return Err(unprocessable("the window may span at most ten years"));
    }
    let rows = scope.keep(vis_models::all_allocations(&ctx.db).await?);
    // PJ-4: proposed bookings are reported as their own series, never merged in.
    let proposed_rows: Vec<_> = scope
        .keep(vis_models::all_allocations_with_proposed(&ctx.db).await?)
        .into_iter()
        .filter(|a| a.booking == "proposed")
        .collect();
    let all_ranges = super::super::availability::all_ranges(&ctx).await?;
    let mut people: Vec<&str> = rows
        .iter()
        .chain(proposed_rows.iter())
        .map(|a| a.person_ref.as_str())
        .collect();
    people.sort_unstable();
    people.dedup();
    let mut out = Vec::with_capacity(people.len());
    for person in people {
        let theirs: Vec<(i32, Option<chrono::NaiveDate>, Option<chrono::NaiveDate>)> = rows
            .iter()
            .filter(|a| a.person_ref == person)
            .map(|a| (a.percent, a.start_date, a.end_date))
            .collect();
        let total = rules::summed_percent(&theirs, from, to);
        // PJ-3: capacity is read per day from the person's availability ranges.
        // With none in the window the answer is exactly what it was before
        // (summed demand over 100); with some, a day is over when its demand
        // exceeds the capacity in force that day.
        let ranges = all_ranges.get(person).map_or(&[][..], Vec::as_slice);
        let in_window: Vec<_> = ranges
            .iter()
            .copied()
            .filter(|r| r.from <= to && r.to >= from)
            .collect();
        let allocs: Vec<crate::availability::Alloc> = theirs
            .iter()
            .map(|(percent, start, end)| crate::availability::Alloc {
                percent: *percent,
                start: *start,
                end: *end,
            })
            .collect();
        let load = crate::availability::load(from, to, &allocs, &in_window);
        let proposed: Vec<&allocations::Model> = proposed_rows
            .iter()
            .filter(|a| {
                a.person_ref == person
                    && a.start_date.is_none_or(|s| s <= to)
                    && a.end_date.is_none_or(|e| e >= from)
            })
            .collect();
        let would_exceed = !proposed.is_empty() && {
            let mut both = allocs.clone();
            both.extend(proposed.iter().map(|a| crate::availability::Alloc {
                percent: a.percent,
                start: a.start_date,
                end: a.end_date,
            }));
            crate::availability::load(from, to, &both, &in_window).days_over > 0
        };
        let over_allocated = if in_window.is_empty() {
            total > 100
        } else {
            load.days_over > 0
        };
        out.push(serde_json::json!({
            "person_ref": person,
            "allocated_percent": total,
            "over_allocated": over_allocated,
            "allocations": theirs.len(),
            "capacity": {
                "source": if in_window.is_empty() { "default" } else { "availability" },
                "min_percent": load.min_capacity,
            },
            "peak_percent": load.peak_demand,
            "over_capacity": load.over,
            // The proposed series: what is hoped for, kept apart from the firm
            // figures above. `would_exceed_capacity` asks whether committing all
            // of it would push this person over their capacity on any day.
            "proposed": {
                "percent": proposed.iter().map(|a| i64::from(a.percent)).sum::<i64>(),
                "allocations": proposed.len(),
                "would_exceed_capacity": would_exceed,
            },
        }));
    }
    out.sort_by_key(|v| -v["allocated_percent"].as_i64().unwrap_or(0));
    format::json(serde_json::json!({ "from": from, "to": to, "people": out }))
}
