//! Private plans (RM-8): mark a plan private and name who may see it. Pure
//! rules in [`crate::plan_access`].
//!
//! **What is enforced.** Any request addressed to a plan —
//! `/api/plans/{pid}` and everything beneath it — passes a guard layer that
//! answers `404` to a non-member. Plan **lists, search and duplicate checks**
//! omit private plans for non-members. Discussion, watchers, approvals,
//! custom fields and "My work" resolve their subjects through the same
//! check. A member (or an `access=admin` caller) sees the plan everywhere.
//!
//! **Aggregates (RM-8b).** Every cross-plan view — at-a-glance, executive, board,
//! financials, capacity, risk, compliance, engineering, scenarios, prioritisation,
//! rollups, the audit and event feeds, and the rest — loads its rows through one
//! shared [`HiddenScope`], so a private plan contributes to **no** total, list,
//! ranking or count for someone outside it. A private-plan test compares every
//! aggregate route before and after a private plan appears, and another enumerates
//! the mounted routes so a new one cannot skip the decision. What stays full by
//! design: the admin-only bulk reads (board pack, board investments, board trends,
//! auditor trail, evidence pack), which answer 403 to everyone else.

use std::collections::HashSet;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::plan_access_settings as plan_access;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_access as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn members_of(row: &plan_access::Model) -> Vec<String> {
    serde_json::from_value(row.members.clone()).unwrap_or_default()
}

/// Whether a set of claims holds `access=admin`.
fn is_admin(claims: Option<&authentication_verifier::Claims>) -> bool {
    claims.is_some_and(|c| {
        c.attrs
            .get("access")
            .is_some_and(|v| v.iter().any(|a| a == "admin"))
    })
}

/// The plan pids among `pids` that are private **and** hidden from `caller`.
pub(crate) async fn hidden_plans(
    ctx: &AppContext,
    pids: &[Uuid],
    caller: &MaybeAuthUser,
) -> HashSet<Uuid> {
    if pids.is_empty() {
        return HashSet::new();
    }
    let Ok(rows) = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.is_in(pids.to_vec()))
        .filter(plan_access::Column::Private.eq(true))
        .all(&ctx.db)
        .await
    else {
        // Fail closed on a lookup error: hide what we could not check.
        return pids.iter().copied().collect();
    };
    let admin = is_admin(caller.claims());
    rows.into_iter()
        .filter(|r| !rules::can_see(true, &members_of(r), caller.actor(), admin))
        .map(|r| r.plan_pid)
        .collect()
}

/// The subset of `pids` whose plan is live and **readable** by `caller`: not
/// private-and-hidden, and not denied by the record-level ABAC policy.
/// (A plan that does not exist is simply not in the result.)
pub(crate) async fn readable_plans(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pids: &[Uuid],
) -> HashSet<Uuid> {
    use crate::models::_entities::plans;
    if pids.is_empty() {
        return HashSet::new();
    }
    let Ok(rows) = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(pids.to_vec()))
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
    else {
        return HashSet::new();
    };
    let hidden = hidden_plans(ctx, pids, caller).await;
    rows.into_iter()
        .filter(|p| !hidden.contains(&p.pid))
        .filter(|p| {
            crate::auth::authorize_record(
                caller,
                authentication_verifier::Action::Read,
                &crate::auth::plan_resource_attrs(p.stage.as_deref()),
            )
            .is_ok()
        })
        .map(|p| p.pid)
        .collect()
}

/// **The one shared "what may this caller see" query** (RM-8b): the private plans
/// hidden from them, and the tasks inside those plans. Every cross-plan aggregate
/// loads its rows and passes them through [`HiddenScope::keep`] (or asks
/// [`HiddenScope::plan`] / [`HiddenScope::node`]), so a hidden plan contributes to
/// no total, list, ranking or count. Empty (and free) when nothing is private.
#[derive(Debug, Default, Clone)]
pub(crate) struct HiddenScope {
    plans: HashSet<Uuid>,
    tasks: HashSet<Uuid>,
}

/// A row that belongs to a plan, so it can be kept out of a view that may not see it.
pub(crate) trait PlanScoped {
    /// The plan this row belongs to (a plan is its own).
    fn scope_plan(&self) -> Option<Uuid>;
}

macro_rules! scoped_by_plan_pid {
    ($($m:ident),* $(,)?) => {$(
        impl PlanScoped for crate::models::_entities::$m::Model {
            fn scope_plan(&self) -> Option<Uuid> { Some(self.plan_pid) }
        }
    )*};
}
scoped_by_plan_pid!(
    risks,
    budget_lines,
    milestones,
    tasks,
    allocations,
    time_entries,
    objective_links,
    benefits,
    gate_reviews,
    devops_events,
    controls,
    sprints,
    components,
);
/// Rows that belong to no single plan (a scenario's members are filtered by its own handlers).
macro_rules! scoped_by_nothing {
    ($($m:ident),* $(,)?) => {$(
        impl PlanScoped for crate::models::_entities::$m::Model {
            fn scope_plan(&self) -> Option<Uuid> { None }
        }
    )*};
}
scoped_by_nothing!(scenarios, ideas);
impl PlanScoped for crate::models::_entities::controlled_documents::Model {
    /// A document that belongs to a plan is hidden with it; a free-standing one is public.
    fn scope_plan(&self) -> Option<Uuid> {
        self.plan_pid
    }
}
impl PlanScoped for crate::models::_entities::reviews::Model {
    /// A review of a plan is hidden with the plan.
    fn scope_plan(&self) -> Option<Uuid> {
        (self.subject_kind == "plan").then_some(self.subject_pid)
    }
}
impl PlanScoped for crate::models::_entities::proposals::Model {
    /// A proposal that became a plan is that plan's: hidden with it.
    fn scope_plan(&self) -> Option<Uuid> {
        self.promoted_plan_pid
    }
}
impl PlanScoped for crate::models::_entities::plans::Model {
    fn scope_plan(&self) -> Option<Uuid> {
        Some(self.pid)
    }
}

impl HiddenScope {
    /// The scope for `caller`: every private plan they are not a member of.
    ///
    /// # Errors
    ///
    /// A lookup failure is an error, never an empty scope: not knowing what is
    /// private must not turn into showing it.
    pub(crate) async fn of(ctx: &AppContext, caller: &MaybeAuthUser) -> Result<Self> {
        use crate::models::_entities::tasks;
        let private: Vec<Uuid> = plan_access::Entity::find()
            .filter(plan_access::Column::Private.eq(true))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| r.plan_pid)
            .collect();
        if private.is_empty() {
            return Ok(Self::default());
        }
        let plans = hidden_plans(ctx, &private, caller).await;
        if plans.is_empty() {
            return Ok(Self::default());
        }
        let tasks = tasks::Entity::find()
            .filter(tasks::Column::PlanPid.is_in(plans.iter().copied().collect::<Vec<_>>()))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|t| t.pid)
            .collect();
        Ok(Self { plans, tasks })
    }

    /// Whether nothing is hidden from this caller.
    pub(crate) fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// Whether this plan is hidden.
    pub(crate) fn plan(&self, pid: &Uuid) -> bool {
        self.plans.contains(pid)
    }

    /// Whether a node (a plan or a task) is hidden.
    pub(crate) fn node(&self, pid: &Uuid) -> bool {
        self.plans.contains(pid) || self.tasks.contains(pid)
    }

    /// The pids of everything that belongs to a hidden plan (tasks, risks, budget lines,
    /// milestones, bookings, benefits, time entries, sprints, components, controls,
    /// gate reviews, deployments, alignments, RACI rows, checklists): what an audit or
    /// event row about "an entity" must not be shown for.
    #[allow(clippy::too_many_lines)]
    pub(crate) async fn owned_pids(&self, ctx: &AppContext) -> Result<HashSet<Uuid>> {
        let mut out: HashSet<Uuid> = self
            .plans
            .iter()
            .chain(self.tasks.iter())
            .copied()
            .collect();
        if self.plans.is_empty() {
            return Ok(out);
        }
        let hidden: Vec<Uuid> = self.plans.iter().copied().collect();
        macro_rules! owned {
            ($($m:ident),* $(,)?) => {$(
                out.extend(
                    crate::models::_entities::$m::Entity::find()
                        .filter(crate::models::_entities::$m::Column::PlanPid.is_in(hidden.clone()))
                        .all(&ctx.db)
                        .await
                        .map_err(db_err)?
                        .into_iter()
                        .map(|r| r.pid),
                );
            )*};
        }
        owned!(
            risks,
            budget_lines,
            milestones,
            allocations,
            benefits,
            time_entries,
            sprints,
            components,
            controls,
            gate_reviews,
            devops_events,
            objective_links,
            raci_assignments,
            onboarding_cases,
            access_records,
            automations,
            budget_baselines,
            canvases,
            business_case_targets,
            earned_value_snapshots,
            earned_value_settings,
            plan_links,
            phase_transitions,
            satisfaction_responses,
            schedule_baselines,
            task_transitions,
            value_points,
            time_entry_costs,
            adoption_snapshots,
            workflows,
        );
        // A KPI may belong to a plan (or to none).
        out.extend(
            crate::models::_entities::kpis::Entity::find()
                .filter(crate::models::_entities::kpis::Column::PlanPid.is_in(hidden.clone()))
                .all(&ctx.db)
                .await
                .map_err(db_err)?
                .into_iter()
                .map(|r| r.pid),
        );
        // Documents of a hidden plan, and their versions.
        let doc_pids: Vec<Uuid> = crate::models::_entities::controlled_documents::Entity::find()
            .filter(
                crate::models::_entities::controlled_documents::Column::PlanPid
                    .is_in(hidden.clone()),
            )
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|d| d.pid)
            .collect();
        if !doc_pids.is_empty() {
            out.extend(doc_pids.iter().copied());
            out.extend(
                crate::models::_entities::document_versions::Entity::find()
                    .filter(
                        crate::models::_entities::document_versions::Column::DocumentPid
                            .is_in(doc_pids),
                    )
                    .all(&ctx.db)
                    .await
                    .map_err(db_err)?
                    .into_iter()
                    .map(|v| v.pid),
            );
        }
        // A proposal that became a hidden plan is hidden with it.
        out.extend(
            crate::models::_entities::proposals::Entity::find()
                .filter(crate::models::_entities::proposals::Column::PromotedPlanPid.is_in(hidden))
                .all(&ctx.db)
                .await
                .map_err(db_err)?
                .into_iter()
                .map(|p| p.pid),
        );
        Ok(out)
    }

    /// Whether `text` (a serialised snapshot or event) names a hidden plan.
    pub(crate) fn mentions(&self, text: &str) -> bool {
        self.plans.iter().any(|p| text.contains(&p.to_string()))
    }

    /// The audit rows the caller may see: none about, or naming, anything in a hidden plan.
    pub(crate) async fn visible_audit(
        &self,
        ctx: &AppContext,
        rows: Vec<crate::models::_entities::audit_logs::Model>,
    ) -> Result<Vec<crate::models::_entities::audit_logs::Model>> {
        if self.is_empty() {
            return Ok(rows);
        }
        let owned = self.owned_pids(ctx).await?;
        Ok(rows
            .into_iter()
            .filter(|r| {
                !owned.contains(&r.entity_pid)
                    && !r
                        .snapshot
                        .as_ref()
                        .is_some_and(|s| self.mentions(&s.to_string()))
            })
            .collect())
    }

    /// The edges the caller may see: those with **neither** end hidden.
    pub(crate) fn keep_edges<T>(&self, rows: Vec<T>, ends: impl Fn(&T) -> (Uuid, Uuid)) -> Vec<T> {
        if self.is_empty() {
            return rows;
        }
        rows.into_iter()
            .filter(|r| {
                let (a, b) = ends(r);
                !self.node(&a) && !self.node(&b)
            })
            .collect()
    }

    /// The rows the caller may see: those not belonging to a hidden plan.
    pub(crate) fn keep<T: PlanScoped>(&self, rows: Vec<T>) -> Vec<T> {
        if self.is_empty() {
            return rows;
        }
        rows.into_iter()
            .filter(|r| r.scope_plan().is_none_or(|p| !self.plans.contains(&p)))
            .collect()
    }
}

/// The plan a path addresses: `/api/plans/{uuid}` or anything beneath it.
fn plan_pid_of(path: &str) -> Option<Uuid> {
    let rest = path.strip_prefix("/api/plans/")?;
    Uuid::parse_str(rest.split('/').next()?).ok()
}

/// The guard layer: a request addressed to a private plan by a non-member is
/// `404` (existence is not disclosed). Every other request passes untouched,
/// and a lookup failure fails **closed** for a private plan only.
pub async fn privacy_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> AxumResponse {
    if let Some(plan_pid) = plan_pid_of(req.uri().path()) {
        let private = plan_access::Entity::find()
            .filter(plan_access::Column::PlanPid.eq(plan_pid))
            .filter(plan_access::Column::Private.eq(true))
            .one(&ctx.db)
            .await;
        if let Ok(Some(row)) = private {
            let claims = crate::auth::request_claims(req.headers()).await.ok();
            let sub = claims.as_ref().map(|c| c.sub.as_str());
            if !rules::can_see(true, &members_of(&row), sub, is_admin(claims.as_ref())) {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(serde_json::json!({ "error": "not_found" })),
                )
                    .into_response();
            }
        }
    }
    next.run(req).await
}

/// `GET /api/plans/{pid}/access`.
#[debug_handler]
async fn get_access(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "private": row.as_ref().is_some_and(|r| r.private),
        "members": row.as_ref().map(members_of).unwrap_or_default(),
    }))
}

/// `PUT /api/plans/{pid}/access` body.
#[derive(Debug, Deserialize)]
struct AccessPayload {
    private: bool,
    #[serde(default)]
    members: Vec<String>,
}

/// `PUT /api/plans/{pid}/access` — make a plan private (with its members) or
/// public again. The caller is always added as a member when making a plan
/// private, so a plan can never be locked away from the person who locked it.
/// Only someone who can already see the plan reaches this (the guard layer).
#[debug_handler]
async fn put_access(
    State(ctx): State<AppContext>,
    AuthUser(claims): AuthUser,
    Path(pid): Path<String>,
    Json(p): Json<AccessPayload>,
) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let mut members = rules::normalise_members(&p.members).map_err(|m| unprocessable(&m))?;
    if p.private {
        let me = Uuid::parse_str(&claims.sub)
            .map_err(|_| {
                unprocessable(
                    "sub_not_addressable: a private plan needs a member identity that is a user id",
                )
            })?
            .to_string();
        if !members.contains(&me) {
            members.push(me);
        }
    }
    let existing = plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.eq(plan.pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let before = existing.as_ref().map(|r| (r.private, members_of(r)));
    match existing {
        Some(row) => {
            let mut active: plan_access::ActiveModel = row.into();
            active.private = ActiveValue::set(p.private);
            active.members = ActiveValue::set(serde_json::json!(members));
            active.update(&ctx.db).await.map_err(db_err)?;
        }
        None => {
            plan_access::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                private: ActiveValue::set(p.private),
                members: ActiveValue::set(serde_json::json!(members)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
    }
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "plan_access_changed",
        Some(&claims.sub),
        Some(serde_json::json!({
            "before": before,
            "after": { "private": p.private, "members": members },
        })),
    )
    .await
    .ok();
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "private": p.private, "members": members }),
    )
}

/// The plan-access routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/access", get(get_access))
        .add("/plans/{pid}/access", put(put_access))
}
