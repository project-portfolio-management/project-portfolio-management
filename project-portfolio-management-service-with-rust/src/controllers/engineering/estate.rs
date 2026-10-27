//! Estate views: blocked-work aging, `MoSCoW`, delivery links and the milestone calendar.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use axum::http::HeaderMap;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::super::insights::item_ref;
use super::{conditional, refuse, task_view};
use crate::models::_entities::{milestones, plans, tasks};

// ─── Estate views ───────────────────────────────────────────────────────────

/// `GET /api/engineering/blocked` — every blocked task estate-wide,
/// oldest blockage first, with age in calendar days (from `status_changed_at`).
#[debug_handler]
pub(super) async fn blocked(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let mut rows: Vec<&tasks::Model> = Vec::new();
    let all_tasks = scope.keep(
        tasks::Entity::find()
            .filter(tasks::Column::Status.eq("blocked"))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    for task in &all_tasks {
        rows.push(task);
    }
    rows.sort_by_key(|task| task.status_changed_at);
    let blocked: Vec<serde_json::Value> = rows
        .iter()
        .map(|task| {
            let mut view = task_view(task);
            if let Some(map) = view.as_object_mut() {
                map.insert(
                    "item".to_string(),
                    by_pid
                        .get(&task.plan_pid)
                        .map_or(serde_json::Value::Null, |i| item_ref(i)),
                );
            }
            view
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "derivation": "age = calendar days since the task entered blocked (status_changed_at)",
        "blocked": blocked,
    });
    conditional(&headers, &body)
}

/// `GET /api/engineering/moscow` — the `MoSCoW` scope cut from
/// `moscow:<band>` tags (convention disclosed; untagged items listed
/// separately, never guessed into a band).
#[debug_handler]
pub(super) async fn moscow(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let mut bands: std::collections::BTreeMap<&str, Vec<serde_json::Value>> = rules::MOSCOW_BANDS
        .iter()
        .map(|band| (*band, Vec::new()))
        .collect();
    let mut untagged = 0usize;
    for item in &items {
        let tags = item
            .data
            .get("tags")
            .and_then(|v| v.as_array())
            .map_or(&[][..], Vec::as_slice);
        let band = tags
            .iter()
            .filter_map(|tag| tag.as_str())
            .find_map(rules::parse_moscow_tag);
        match band {
            Some(band) => bands
                .get_mut(band)
                .expect("known band")
                .push(item_ref(item)),
            None => untagged += 1,
        }
    }
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "convention": "tag `moscow:<must|should|could|wont>`; the first moscow tag wins; \
                       untagged items are counted, never guessed into a band",
        "bands": bands,
        "untagged": untagged,
    });
    conditional(&headers, &body)
}

/// `GET /api/engineering/delivery-links` — which items are tracked in
/// which external delivery tool (the `identifiers` schemes the matcher
/// already short-circuits on), plus the untracked list.
#[debug_handler]
pub(super) async fn delivery_links(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    const TRACKER_SCHEMES: [&str; 6] = [
        "JiraProjectKey",
        "AsanaGid",
        "TrelloBoardId",
        "MsProjectId",
        "GitHubProjectId",
        "LinearId",
    ];
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let mut tracked = Vec::new();
    let mut untracked = Vec::new();
    for item in &items {
        let identifiers = item
            .data
            .get("identifiers")
            .and_then(|v| v.as_array())
            .map_or(&[][..], Vec::as_slice);
        let links: Vec<serde_json::Value> = identifiers
            .iter()
            .filter_map(|identifier| {
                let scheme = identifier.get("scheme").and_then(|v| v.as_str())?;
                TRACKER_SCHEMES.contains(&scheme).then(|| {
                    serde_json::json!({
                        "scheme": scheme,
                        "value": identifier.get("value"),
                    })
                })
            })
            .collect();
        if links.is_empty() {
            untracked.push(item_ref(item));
        } else {
            tracked.push(serde_json::json!({ "item": item_ref(item), "links": links }));
        }
    }
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "schemes": TRACKER_SCHEMES,
        "tracked": tracked,
        "untracked": untracked,
    });
    conditional(&headers, &body)
}

/// Query for the milestone calendar: optional kind filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CalendarQuery {
    kind: Option<String>,
}

/// `GET /api/engineering/milestone-calendar?kind=` — estate milestones
/// with their kinds (absent reads `milestone`), due-date order, for
/// the demo / release calendar.
#[debug_handler]
pub(super) async fn milestone_calendar(
    axum::extract::Query(query): axum::extract::Query<CalendarQuery>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    if let Some(kind) = query.kind.as_deref()
        && !rules::MILESTONE_KINDS.contains(&kind)
    {
        return Err(refuse(&format!(
            "kind must be one of {:?}",
            rules::MILESTONE_KINDS
        )));
    }
    let items = live!(plans, &ctx.db, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let mut rows = live!(milestones, &ctx.db, &scope);
    rows.sort_by_key(|m| m.due);
    let entries: Vec<serde_json::Value> = rows
        .iter()
        .filter(|m| {
            query
                .kind
                .as_deref()
                .is_none_or(|kind| m.kind.as_deref().unwrap_or("milestone") == kind)
        })
        .map(|m| {
            serde_json::json!({
                "pid": m.pid,
                "name": m.name,
                "kind": m.kind.as_deref().unwrap_or("milestone"),
                "due": m.due,
                "done": m.done,
                "item": by_pid.get(&m.plan_pid).map(|i| item_ref(i)),
            })
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "kinds": rules::MILESTONE_KINDS,
        "milestones": entries,
    });
    conditional(&headers, &body)
}
