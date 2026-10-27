//! The ETag-conditional at-a-glance dashboard.

use axum::http::HeaderMap;
use uuid::Uuid;

use super::{is_finished, schedule_item};
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{budget_lines, plans, risks};
use crate::models::visibility as vis_models;
use crate::visibility as rules;
use loco_rs::prelude::*;

/// `GET /api/at-a-glance` — the portfolio dashboard (PPM-7): per
/// collection, counts by RAG / stage / status plus risk exposure and
/// per-currency budget variance; site tiles headline the whole
/// estate. ETag-conditional (the tag excludes `as_of`).
#[debug_handler]
#[expect(clippy::too_many_lines)] // one pass over the whole estate
pub(super) async fn at_a_glance(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let today = crate::tz::deployment_today();
    let items = scope.keep(
        plans::Entity::find()
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let risk_rows = scope.keep(
        risks::Entity::find()
            .filter(risks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let budget_rows = scope.keep(
        budget_lines::Entity::find()
            .filter(budget_lines::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let proposal_rows = scope.keep(
        crate::models::_entities::proposals::Entity::find()
            .filter(crate::models::_entities::proposals::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let dependency_rows = scope.keep_edges(vis_models::all_dependencies(&ctx.db).await?, |e| {
        (e.predecessor_pid, e.successor_pid)
    });
    let allocation_rows = scope.keep(vis_models::all_allocations(&ctx.db).await?);

    // Global schedule violations (per successor item).
    let facts: Vec<rules::ScheduleItem> = items.iter().map(schedule_item).collect();
    let edges: Vec<rules::ScheduleEdge> = dependency_rows
        .iter()
        .map(|e| rules::ScheduleEdge {
            pid: e.pid,
            predecessor: e.predecessor_pid,
            successor: e.successor_pid,
            lag_days: e.lag_days,
            kind: crate::cpm::Kind::parse(&e.kind).unwrap_or(crate::cpm::Kind::Fs),
        })
        .collect();
    let all_violations = rules::violations(&facts, &edges);
    let violated: std::collections::HashSet<Uuid> =
        all_violations.iter().map(|v| v.successor).collect();

    // Per-item RAG, grouped by the optional `kind` label (one recursive
    // collection now; a missing label groups under "unspecified").
    let mut per_collection: std::collections::BTreeMap<String, Vec<&plans::Model>> =
        std::collections::BTreeMap::new();
    for item in &items {
        per_collection
            .entry(
                item.kind
                    .clone()
                    .unwrap_or_else(|| "unspecified".to_string()),
            )
            .or_default()
            .push(item);
    }
    let mut collections = Vec::new();
    for (kind, members) in &per_collection {
        let mut rag_counts =
            std::collections::BTreeMap::from([("red", 0), ("amber", 0), ("green", 0)]);
        let mut stages: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        for member in members {
            let member_risks: Vec<_> = risk_rows
                .iter()
                .filter(|r| r.plan_pid == member.pid)
                .collect();
            let materialized = member_risks
                .iter()
                .filter(|r| r.status == "materialized")
                .count();
            let max_exposure = member_risks
                .iter()
                .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
                .map(|r| r.probability * r.impact)
                .max()
                .unwrap_or(0);
            let member_budgets: Vec<_> = budget_rows
                .iter()
                .filter(|b| b.plan_pid == member.pid)
                .collect();
            let overrun = {
                let mut currencies: Vec<&str> =
                    member_budgets.iter().map(|b| b.currency.as_str()).collect();
                currencies.sort_unstable();
                currencies.dedup();
                currencies.into_iter().any(|c| {
                    let planned: i64 = member_budgets
                        .iter()
                        .filter(|b| b.currency == c)
                        .map(|b| b.planned_minor)
                        .sum();
                    let actual: i64 = member_budgets
                        .iter()
                        .filter(|b| b.currency == c)
                        .map(|b| b.actual_minor)
                        .sum();
                    actual > planned
                })
            };
            let target = member
                .data
                .get("target_date")
                .and_then(|v| v.as_str())
                .and_then(|s| rules::parse_flex_date(s, true));
            let colour = rules::rag(
                is_finished(member),
                target,
                today,
                materialized,
                max_exposure,
                overrun,
                violated.contains(&member.pid),
            );
            *rag_counts.get_mut(colour).expect("fixed keys") += 1;
            *stages
                .entry(
                    member
                        .stage
                        .clone()
                        .unwrap_or_else(|| "pre_gate".to_string()),
                )
                .or_default() += 1;
        }
        collections.push(serde_json::json!({
            "collection": kind,
            "total": members.len(),
            "rag": rag_counts,
            "stages": stages,
        }));
    }

    // Site tiles.
    let open_risk_exposure: i32 = risk_rows
        .iter()
        .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
        .map(|r| r.probability * r.impact)
        .sum();
    let proposals_open = proposal_rows
        .iter()
        .filter(|p| matches!(p.status.as_str(), "draft" | "submitted" | "in_review"))
        .count();
    let over_allocated = {
        let mut people: Vec<&str> = allocation_rows
            .iter()
            .map(|a| a.person_ref.as_str())
            .collect();
        people.sort_unstable();
        people.dedup();
        people
            .into_iter()
            .filter(|person| {
                let theirs: Vec<_> = allocation_rows
                    .iter()
                    .filter(|a| a.person_ref == *person)
                    .map(|a| (a.percent, a.start_date, a.end_date))
                    .collect();
                rules::summed_percent(&theirs, today, today + chrono::Days::new(30)) > 100
            })
            .count()
    };
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "collections": collections,
        "site_tiles": {
            "plans": items.len(),
            "proposals_open": proposals_open,
            "materialized_risks": risk_rows.iter().filter(|r| r.status == "materialized").count(),
            "open_risk_exposure": open_risk_exposure,
            "schedule_violations": all_violations.len(),
            "over_allocated_people": over_allocated,
        },
    });
    let mut fingerprint = body.clone();
    if let Some(map) = fingerprint.as_object_mut() {
        map.remove("as_of");
    }
    super::super::conditional_json(&headers, &super::super::etag_of(&fingerprint), &body)
}
