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
//! financials, capacity, risk, compliance, engineering, scenarios, prioritization,
//! rollups, the audit and event feeds, and the rest — loads its rows through one
//! shared [`HiddenScope`], so a private plan contributes to **no** total, list,
//! ranking or count for someone outside it. A private-plan test compares every
//! aggregate route before and after a private plan appears, and another enumerates
//! the mounted routes so a new one cannot skip the decision. What stays full by
//! design: the admin-only bulk reads (board pack, board investments, board trends,
//! auditor trail, evidence pack), which answer 403 to everyone else.

use super::{db_err, unprocessable};
use std::collections::HashSet;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::plan_access_settings as plan_access;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_access as rules;

fn members_of(row: &plan_access::Model) -> Vec<String> {
    serde_json::from_value(row.members.clone()).unwrap_or_default()
}

/// Record that a visibility lookup failed and is being answered by hiding (CQ-6): an error log naming
/// the lookup, and `visibility_lookup_failed_total`. Failing closed stays the rule; failing silently
/// does not, because a database fault would otherwise look like an empty portfolio.
fn fail_closed(lookup: &'static str, error: &sea_orm::DbErr) {
    crate::metrics::Metrics::global()
        .visibility_lookup_failed_total
        .inc();
    let _ = LOOKUP_FAILED.try_with(|f| f.store(true, std::sync::atomic::Ordering::Relaxed));
    tracing::error!(lookup, %error, "visibility lookup failed; hiding what could not be checked");
}

tokio::task_local! {
    /// Set by [`fail_closed`] during a request scoped by [`lookup_fault_mw`].
    static LOOKUP_FAILED: std::sync::atomic::AtomicBool;
}

/// A read whose visibility lookup failed answers `503`, not a `200` with less in it (CQ-6).
///
/// `hidden_plans` and `readable_plans` return sets, so a fault shows up as an empty one; this
/// records the fault for the request and replaces a successful read with `503`. Writes are left
/// alone: the write has already happened and a `503` would invite a retry of it.
pub async fn lookup_fault_mw(req: Request, next: Next) -> AxumResponse {
    let is_read = matches!(
        *req.method(),
        axum::http::Method::GET | axum::http::Method::HEAD
    );
    LOOKUP_FAILED
        .scope(std::sync::atomic::AtomicBool::new(false), async move {
            let response = next.run(req).await;
            let failed = LOOKUP_FAILED.with(|f| f.load(std::sync::atomic::Ordering::Relaxed));
            if is_read && failed && response.status().is_success() {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    axum::Json(serde_json::json!({
                        "error": "unavailable",
                        "description": "could not check who may see this; try again",
                    })),
                )
                    .into_response();
            }
            response
        })
        .await
}

/// Whether a set of claims holds `access=admin`.
pub(crate) fn is_admin(claims: Option<&authentication_verifier::Claims>) -> bool {
    claims.is_some_and(|c| crate::auth::has_attr(c, "access", "admin"))
}

/// The caller's `clearance` attribute (RG-7): the first value, if any.
pub(crate) fn clearance_of(claims: Option<&authentication_verifier::Claims>) -> Option<String> {
    claims
        .and_then(|c| c.attrs.get("clearance"))
        .and_then(|v| v.first())
        .cloned()
}

/// The stored handling marks of some plans (`None` for all). Empty with no scheme configured.
async fn stored_marks(
    ctx: &AppContext,
    pids: Option<&[Uuid]>,
) -> std::result::Result<Vec<(Uuid, String)>, sea_orm::DbErr> {
    use crate::models::_entities::plan_handling_marks as marks;
    if !crate::handling_marks::scheme().is_on() {
        return Ok(Vec::new());
    }
    let mut find = marks::Entity::find();
    if let Some(p) = pids {
        find = find.filter(marks::Column::PlanPid.is_in(p.to_vec()));
    }
    Ok(find
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|m| (m.plan_pid, m.mark))
        .collect())
}

/// The plans among `pids` (all marked plans for `None`) hidden from `caller` by their handling mark
/// (RG-7). A lookup failure hides every candidate (fail closed).
pub(crate) async fn mark_hidden(
    ctx: &AppContext,
    pids: Option<&[Uuid]>,
    caller: &MaybeAuthUser,
) -> std::result::Result<HashSet<Uuid>, sea_orm::DbErr> {
    let scheme = crate::handling_marks::scheme();
    let clearance = clearance_of(caller.claims());
    Ok(stored_marks(ctx, pids)
        .await?
        .into_iter()
        .filter(|(_, m)| !scheme.can_see(Some(m), clearance.as_deref()))
        .map(|(p, _)| p)
        .collect())
}

/// For each of `pids`, the token subjects that hold a **live break-glass grant** on it (RG-10): not
/// revoked and not expired. A lookup failure gives an empty answer, which means no extra access (fail
/// closed).
pub(crate) async fn grant_subs(
    ctx: &AppContext,
    pids: &[Uuid],
) -> std::collections::HashMap<Uuid, Vec<String>> {
    use crate::models::_entities::break_glass_grants as grants;
    let mut out: std::collections::HashMap<Uuid, Vec<String>> = std::collections::HashMap::new();
    if pids.is_empty() {
        return out;
    }
    let rows = match grants::Entity::find()
        .filter(grants::Column::PlanPid.is_in(pids.to_vec()))
        .filter(grants::Column::RevokedAt.is_null())
        .filter(grants::Column::ExpiresAt.gt(chrono::Utc::now()))
        .all(&ctx.db)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            fail_closed("break_glass_grants", &error);
            return out;
        }
    };
    for g in rows {
        out.entry(g.plan_pid).or_default().push(g.sub);
    }
    out
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
    // Hidden by handling mark (RG-7): applies to everyone, administrators included.
    let by_mark = match mark_hidden(ctx, Some(pids), caller).await {
        Ok(hidden) => hidden,
        Err(error) => {
            fail_closed("handling_marks", &error);
            return pids.iter().copied().collect();
        }
    };
    let rows = match plan_access::Entity::find()
        .filter(plan_access::Column::PlanPid.is_in(pids.to_vec()))
        .filter(plan_access::Column::Private.eq(true))
        .all(&ctx.db)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            // Fail closed on a lookup error: hide what we could not check.
            fail_closed("plan_access_settings", &error);
            return pids.iter().copied().collect();
        }
    };
    let admin = is_admin(caller.claims());
    // A live break-glass grant makes the grantee a temporary member for visibility (RG-10).
    let granted = grant_subs(ctx, pids).await;
    let mut hidden: HashSet<Uuid> = rows
        .into_iter()
        .filter(|r| {
            let mut members = members_of(r);
            members.extend(granted.get(&r.plan_pid).cloned().unwrap_or_default());
            !rules::can_see(true, &members, caller.actor(), admin)
        })
        .map(|r| r.plan_pid)
        .collect();
    hidden.extend(by_mark);
    hidden
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
    let rows = match plans::Entity::find()
        .filter(plans::Column::Pid.is_in(pids.to_vec()))
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            fail_closed("readable_plans", &error);
            return HashSet::new();
        }
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
        let mut private: Vec<Uuid> = plan_access::Entity::find()
            .filter(plan_access::Column::Private.eq(true))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| r.plan_pid)
            .collect();
        // Plans with a handling mark are candidates too (RG-7).
        private.extend(
            stored_marks(ctx, None)
                .await
                .map_err(db_err)?
                .into_iter()
                .map(|(p, _)| p),
        );
        private.sort();
        private.dedup();
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
    #[expect(clippy::too_many_lines)]
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

    /// Whether `text` (a serialized snapshot or event) names a hidden plan.
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
///
/// The segment is percent-decoded **before** it is parsed, because the handlers see it
/// decoded: a guard that parsed the raw text would be skipped by `%7B<uuid>%7D` (a braced
/// form `Uuid::parse_str` accepts) while the handler served the plan.
fn plan_pid_of(path: &str) -> Option<Uuid> {
    let rest = path.strip_prefix("/api/plans/")?;
    let raw = rest.split('/').next()?;
    Uuid::parse_str(&percent_decode(raw)).ok()
}

/// Decode `%XX` escapes (invalid ones are left as they are).
fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            let pair = (
                char::from(b[i + 1]).to_digit(16),
                char::from(b[i + 2]).to_digit(16),
            );
            if let (Some(h), Some(l)) = pair {
                out.push(u8::try_from(h * 16 + l).unwrap_or(b'%'));
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod path_tests {
    use super::*;

    const P: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";

    #[test]
    fn every_spelling_the_handler_would_accept_is_guarded() {
        let want = Uuid::parse_str(P).ok();
        for path in [
            format!("/api/plans/{P}"),
            format!("/api/plans/{P}/cpm"),
            format!("/api/plans/%7B{P}%7D/cpm"),
            format!("/api/plans/%7b{P}%7d"),
            format!("/api/plans/{}", P.replace('-', "")),
            format!("/api/plans/urn:uuid:{P}/tasks"),
            "/api/plans/%33f2c6b7e-0000-4000-8000-0000000000a1/cpm".to_string(),
        ] {
            assert_eq!(plan_pid_of(&path), want, "{path}");
        }
        for path in [
            "/api/plans/search",
            "/api/plans",
            "/api/other/x",
            "/api/plans/%zz",
            "/api/plans/%",
        ] {
            assert_eq!(plan_pid_of(path), None, "{path}");
        }
    }

    /// CQ-25: every plan-link and task-link route sits under `/api/plans/{pid}/`, so the guard
    /// above decides privacy for all of them (the CHANGELOG once said they skip the check).
    /// Read from the controller's own route table, so a link route added elsewhere fails here.
    #[test]
    fn every_link_route_is_behind_the_privacy_guard() {
        let source = include_str!("plan_links.rs");
        assert!(source.contains(".prefix(\"/api\")"));
        let routes: Vec<&str> = source
            .lines()
            .filter_map(|l| l.trim().strip_prefix(".add(\""))
            .filter_map(|l| l.split('"').next())
            .collect();
        assert!(routes.len() >= 7, "{routes:?}");
        let want = Uuid::parse_str(P).ok();
        for route in routes {
            let path = format!("/api{route}")
                .replace("{pid}", P)
                .replace("{lpid}", P)
                .replace("{tpid}", P);
            assert_eq!(plan_pid_of(&path), want, "{route} is not under the guard");
        }
    }
}

/// The guard layer: a request addressed to a private plan by a non-member is
/// `404` (existence is not disclosed). Every other request passes untouched.
/// A failed privacy lookup is `503` for every plan, since whether the plan is
/// private is the thing that could not be checked (CQ-6).
pub async fn privacy_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> AxumResponse {
    let mut label: Option<String> = None;
    if let Some(plan_pid) = plan_pid_of(req.uri().path()) {
        // Handling mark (RG-7): too high for the caller is a 404, whoever the caller is; otherwise the
        // plan's mark goes out on the response.
        let scheme = crate::handling_marks::scheme();
        if scheme.is_on() {
            let claims = crate::auth::request_claims(req.headers()).await.ok();
            let clearance = clearance_of(claims.as_ref());
            let stored = stored_marks(&ctx, Some(&[plan_pid])).await;
            let mark = match &stored {
                Ok(v) => v.first().map(|(_, m)| m.clone()),
                // Could not check: say nothing is there.
                Err(error) => {
                    fail_closed("handling_marks", error);
                    Some(scheme.levels().last().cloned().unwrap_or_default())
                }
            };
            if stored.is_err() || !scheme.can_see(mark.as_deref(), clearance.as_deref()) {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(serde_json::json!({ "error": "not_found" })),
                )
                    .into_response();
            }
            label = scheme.effective(mark.as_deref()).map(str::to_string);
        }
        let private = plan_access::Entity::find()
            .filter(plan_access::Column::PlanPid.eq(plan_pid))
            .filter(plan_access::Column::Private.eq(true))
            .one(&ctx.db)
            .await;
        // Could not tell whether the plan is private: refuse rather than pass (CQ-6). This
        // used to fall through to the handler, which a non-member must not reach.
        if let Err(error) = &private {
            fail_closed("plan_access_settings", error);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(serde_json::json!({
                    "error": "unavailable",
                    "description": "could not check who may see this plan; try again",
                })),
            )
                .into_response();
        }
        if let Ok(Some(row)) = private {
            let claims = crate::auth::request_claims(req.headers()).await.ok();
            let sub = claims.as_ref().map(|c| c.sub.as_str());
            let members = members_of(&row);
            let admin = is_admin(claims.as_ref());
            if !rules::can_see(true, &members, sub, admin) {
                // Not a member. A live break-glass grant (RG-10) opens it, and every request it
                // opens is audited, so the use of the grant is as visible as its opening.
                let granted = grant_subs(&ctx, &[plan_pid]).await;
                let via_grant = sub.is_some_and(|s| {
                    granted
                        .get(&plan_pid)
                        .is_some_and(|v| v.iter().any(|g| g == s))
                });
                if !via_grant {
                    return (
                        StatusCode::NOT_FOUND,
                        axum::Json(serde_json::json!({ "error": "not_found" })),
                    )
                        .into_response();
                }
                AuditModel::record_or_log(
                    &ctx.db,
                    plan_pid,
                    "break_glass_access",
                    sub,
                    Some(serde_json::json!({
                        "method": req.method().as_str(), "path": req.uri().path(),
                    })),
                )
                .await;
            }
        }
    }
    let mut response = next.run(req).await;
    if let Some(l) = label
        && let Ok(v) = axum::http::HeaderValue::from_str(&l)
    {
        response.headers_mut().insert("x-handling-mark", v);
    }
    response
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
    let mut members = rules::normalize_members(&p.members).map_err(|m| unprocessable(&m))?;
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
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    match existing {
        Some(row) => {
            let mut active: plan_access::ActiveModel = row.into();
            active.private = ActiveValue::set(p.private);
            active.members = ActiveValue::set(serde_json::json!(members));
            active.update(&txn).await.map_err(db_err)?;
        }
        None => {
            plan_access::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                private: ActiveValue::set(p.private),
                members: ActiveValue::set(serde_json::json!(members)),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .map_err(db_err)?;
        }
    }
    AuditModel::record(
        &txn,
        plan.pid,
        "plan_access_changed",
        Some(&claims.sub),
        Some(serde_json::json!({
            "before": before,
            "after": { "private": p.private, "members": members },
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
