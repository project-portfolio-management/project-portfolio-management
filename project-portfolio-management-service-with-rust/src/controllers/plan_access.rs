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
//! shared `HiddenScope`, so a private plan contributes to **no** total, list,
//! ranking or count for someone outside it. A private-plan test compares every
//! aggregate route before and after a private plan appears, and another enumerates
//! the mounted routes so a new one cannot skip the decision. What stays full by
//! design: the admin-only bulk reads (board pack, board investments, board trends,
//! auditor trail, evidence pack), which answer 403 to everyone else.

use super::unprocessable;
use std::collections::HashSet;

use crate::prelude::*;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response as AxumResponse};
use serde::Deserialize;
use uuid::Uuid;

use super::governance::find_item;
use crate::auth::{AuthUser, MaybeAuthUser};
use crate::models::_entities::plan_access_settings as plan_access;
use crate::models::audit_logs::Model as AuditModel;
use crate::plan_access as rules;

/// The member ids stored on an access row; unreadable JSON yields no members, which hides the plan from everyone but admins rather than exposing it.
fn members_of(row: &plan_access::Model) -> Vec<String> {
    serde_json::from_value(row.members.clone()).unwrap_or_default()
}

/// Record that a visibility lookup failed and is being answered by hiding (CQ-6): an error log naming
/// the lookup, and `visibility_lookup_failed_total`. Failing closed stays the rule; failing silently
/// does not, because a database fault would otherwise look like an empty portfolio.
fn fail_closed(lookup: &'static str, error: &sqlx::Error) {
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
                return super::problem(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "unavailable",
                    "could not check who may see this; try again",
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
) -> std::result::Result<Vec<(Uuid, String)>, sqlx::Error> {
    if !crate::handling_marks::scheme().is_on() {
        return Ok(Vec::new());
    }
    match pids {
        Some(p) => {
            sqlx::query_as(
                "SELECT plan_pid, mark FROM plan_handling_marks WHERE plan_pid = ANY($1)",
            )
            .bind(p)
            .fetch_all(crate::db::pool(ctx))
            .await
        }
        None => {
            sqlx::query_as("SELECT plan_pid, mark FROM plan_handling_marks")
                .fetch_all(crate::db::pool(ctx))
                .await
        }
    }
}

/// The plans among `pids` (all marked plans for `None`) hidden from `caller` by their handling mark
/// (RG-7). A lookup failure hides every candidate (fail closed).
pub(crate) async fn mark_hidden(
    ctx: &AppContext,
    pids: Option<&[Uuid]>,
    caller: &MaybeAuthUser,
) -> std::result::Result<HashSet<Uuid>, sqlx::Error> {
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
    let mut out: std::collections::HashMap<Uuid, Vec<String>> = std::collections::HashMap::new();
    if pids.is_empty() {
        return out;
    }
    let rows: Vec<(Uuid, String)> = match sqlx::query_as(
        "SELECT plan_pid, sub FROM break_glass_grants
         WHERE plan_pid = ANY($1) AND revoked_at IS NULL AND expires_at > $2",
    )
    .bind(pids)
    .bind(chrono::Utc::now())
    .fetch_all(crate::db::pool(ctx))
    .await
    {
        Ok(rows) => rows,
        Err(error) => {
            fail_closed("break_glass_grants", &error);
            return out;
        }
    };
    for (plan_pid, sub) in rows {
        out.entry(plan_pid).or_default().push(sub);
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
    let rows: Vec<plan_access::Model> = match sqlx::query_as(
        "SELECT * FROM plan_access_settings WHERE plan_pid = ANY($1) AND private = true",
    )
    .bind(pids)
    .fetch_all(crate::db::pool(ctx))
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
    if pids.is_empty() {
        return HashSet::new();
    }
    let rows: Vec<(Uuid, Option<String>)> = match sqlx::query_as(
        "SELECT pid, stage FROM plans WHERE pid = ANY($1) AND deleted_at IS NULL",
    )
    .bind(pids)
    .fetch_all(crate::db::pool(ctx))
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
        .filter(|(pid, _)| !hidden.contains(pid))
        .filter(|(_, stage)| {
            crate::auth::authorize_record(
                caller,
                authentication_verifier::Action::Read,
                &crate::auth::plan_resource_attrs(stage.as_deref()),
            )
            .is_ok()
        })
        .map(|(pid, _)| pid)
        .collect()
}

/// `404` unless `caller` can read `plan`, the same answer as a record that does not exist. For a route outside the plan guard
/// (`privacy_mw` covers only `/api/plans/{uuid}...`) that reaches a plan through a record's own pid or a field of the body.
pub(crate) async fn require_readable(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
) -> Result<()> {
    if readable_plans(ctx, caller, &[plan]).await.contains(&plan) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// [`require_readable`] for a record that may belong to no plan, or to one since deleted. The lists that show such a record
/// ask only whether the plan is hidden, so a deleted plan must not turn a listed record into a `404`.
pub(crate) async fn require_readable_if_any(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Option<Uuid>,
) -> Result<()> {
    let Some(plan) = plan else {
        return Ok(());
    };
    if hidden_plans(ctx, &[plan], caller).await.contains(&plan) {
        return Err(Error::NotFound);
    }
    let live = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(plan)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .is_some();
    if live {
        require_readable(ctx, caller, plan).await
    } else {
        Ok(())
    }
}

/// `column` is none of `pids`, as **one** bound value: pushes `{column} <> ALL($n::uuid[])` with the pids bound as one
/// array. [`HiddenScope::owned_pids`] can run past the 65,535 bind parameters Postgres allows, which one parameter per
/// pid (`NOT IN (…)`) would turn into a failed query. `column` is a literal at every call, never input.
pub(crate) fn push_none_of(
    q: &mut sqlx::QueryBuilder<sqlx::Postgres>,
    column: &'static str,
    pids: impl IntoIterator<Item = Uuid>,
) {
    q.push(column)
        .push(" <> ALL(")
        .push_bind(pids.into_iter().collect::<Vec<Uuid>>())
        .push("::uuid[])");
}

/// Notifications sent to someone by name because they must act or be told, whether or not they can read the plan: an
/// invitation to review, a request to approve, a process step, and the notice that a plan was opened by break-glass.
const ADDRESSED_KINDS: &[&str] = &[
    "approval_requested",
    "break_glass",
    "process_notice",
    "process_todo",
    "review_invited",
];

/// **The one shared "what may this caller see" query** (RM-8b): the private plans
/// hidden from them, and the tasks inside those plans. Every cross-plan aggregate
/// loads its rows and passes them through [`HiddenScope::keep`] (or asks
/// [`HiddenScope::plan`] / [`HiddenScope::node`]), so a hidden plan contributes to
/// no total, list, ranking or count. Empty (and free) when nothing is private.
#[derive(Debug, Default, Clone)]
pub(crate) struct HiddenScope {
    /// Private plans the caller is not a member of.
    plans: HashSet<Uuid>,
    /// Tasks inside those hidden plans.
    tasks: HashSet<Uuid>,
}

/// A row that belongs to a plan, so it can be kept out of a view that may not see it.
pub(crate) trait PlanScoped {
    /// The plan this row belongs to (a plan is its own).
    fn scope_plan(&self) -> Option<Uuid>;
}

/// Implement [`PlanScoped`] for entities whose rows carry a `plan_pid`, so each is hidden with its plan.
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
impl PlanScoped for crate::models::_entities::canvases::Model {
    /// A canvas on a plan is hidden with it; a free-standing one is public.
    fn scope_plan(&self) -> Option<Uuid> {
        self.plan_pid
    }
}
impl PlanScoped for crate::models::_entities::plan_templates::Model {
    /// A template is a snapshot of the plan it came from: hidden with it.
    fn scope_plan(&self) -> Option<Uuid> {
        self.source_plan_pid
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
        let mut private: Vec<Uuid> =
            sqlx::query_scalar("SELECT plan_pid FROM plan_access_settings WHERE private = true")
                .fetch_all(crate::db::pool(ctx))
                .await?;
        // Plans with a handling mark are candidates too (RG-7).
        private.extend(stored_marks(ctx, None).await?.into_iter().map(|(p, _)| p));
        private.sort();
        private.dedup();
        if private.is_empty() {
            return Ok(Self::default());
        }
        let plans = hidden_plans(ctx, &private, caller).await;
        if plans.is_empty() {
            return Ok(Self::default());
        }
        let tasks: Vec<Uuid> = sqlx::query_scalar("SELECT pid FROM tasks WHERE plan_pid = ANY($1)")
            .bind(plans.iter().copied().collect::<Vec<_>>())
            .fetch_all(crate::db::pool(ctx))
            .await?;
        let tasks = tasks.into_iter().collect();
        Ok(Self { plans, tasks })
    }

    /// Whether nothing is hidden from this caller.
    pub(crate) fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// The hidden plans, for a list that must filter in its query so that paging stays true.
    pub(crate) fn plan_pids(&self) -> Vec<Uuid> {
        self.plans.iter().copied().collect()
    }

    /// The notifications `caller`'s own inbox keeps: not those about something in a hidden plan, whose stored words can quote
    /// it, unless they were sent to the caller by name ([`ADDRESSED_KINDS`]). Pushes `AND (subject_pid <> ALL(…) OR kind =
    /// ANY(…))` onto a query over `notifications` that already has a `WHERE`, or nothing when nothing is hidden.
    ///
    /// # Errors
    ///
    /// When the owned pids cannot be read ([`Self::owned_pids`]).
    pub(crate) async fn notes_visible(
        &self,
        ctx: &AppContext,
        q: &mut sqlx::QueryBuilder<sqlx::Postgres>,
    ) -> Result<()> {
        if self.is_empty() {
            return Ok(());
        }
        let owned = self.owned_pids(ctx).await?;
        q.push(" AND (");
        push_none_of(q, "subject_pid", owned);
        q.push(" OR kind = ANY(")
            .push_bind(ADDRESSED_KINDS.to_vec())
            .push("))");
        Ok(())
    }

    /// Whether [`Self::notes_visible`] leaves this notification out.
    pub(crate) async fn hides_note(
        &self,
        ctx: &AppContext,
        note: &crate::models::_entities::notifications::Model,
    ) -> Result<bool> {
        Ok(!self.is_empty()
            && !ADDRESSED_KINDS.contains(&note.kind.as_str())
            && self.owned_pids(ctx).await?.contains(&note.subject_pid))
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
        let db = crate::db::pool(ctx);
        // The pids of one table's rows whose `$column` is any of `$among`; the table and column are literals.
        macro_rules! pids_of {
            ($table:expr, $column:literal, $among:expr) => {
                sqlx::query_scalar::<_, Uuid>(concat!(
                    "SELECT pid FROM ",
                    $table,
                    " WHERE ",
                    $column,
                    " = ANY($1)"
                ))
                .bind($among)
                .fetch_all(db)
                .await?
            };
        }
        macro_rules! owned {
            ($($m:ident),* $(,)?) => {$(
                out.extend(pids_of!(stringify!($m), "plan_pid", &hidden));
            )*};
        }
        owned!(
            risks,
            issues,
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
        // The items of a hidden plan's canvases.
        let canvases: Vec<Uuid> = pids_of!("canvases", "plan_pid", &hidden);
        out.extend(pids_of!("canvas_items", "canvas_pid", &canvases));
        // A review of a hidden plan, a proposal that became one, and any review of that proposal.
        let promoted: Vec<Uuid> = pids_of!("proposals", "promoted_plan_pid", &hidden);
        out.extend(promoted.iter().copied());
        let subjects: Vec<Uuid> = hidden.iter().chain(promoted.iter()).copied().collect();
        out.extend(pids_of!("reviews", "subject_pid", &subjects));
        // A KPI may belong to a plan (or to none).
        out.extend(pids_of!("kpis", "plan_pid", &hidden));
        // Documents of a hidden plan, and their versions.
        let doc_pids: Vec<Uuid> = pids_of!("controlled_documents", "plan_pid", &hidden);
        if !doc_pids.is_empty() {
            out.extend(doc_pids.iter().copied());
            out.extend(pids_of!("document_versions", "document_pid", &doc_pids));
        }
        // A proposal that became a hidden plan is hidden with it.
        out.extend(pids_of!("proposals", "promoted_plan_pid", &hidden));
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

    /// A new route outside the guard has to say how it treats a plan the caller cannot read, whatever its method: the audit
    /// behind OC-13 to OC-23 found routes that had simply never been asked, because only GETs were in a census.
    #[test]
    fn every_route_outside_the_guard_is_declared() {
        const CLASSES: &[&str] = &["checks", "filters", "own", "admin", "no-plan", "left-open"];
        let declared: std::collections::BTreeMap<&str, &str> =
            include_str!("../../plan-guard-census.txt")
                .lines()
                .filter(|l| !l.starts_with('#') && !l.is_empty())
                .map(|l| l.split_once('\t').expect("route<TAB>class"))
                .collect();
        for (route, class) in &declared {
            assert!(CLASSES.contains(class), "{route}: unknown class {class}");
        }
        let outside: std::collections::BTreeSet<&str> = include_str!("../../api-routes.txt")
            .lines()
            .filter(|l| l.contains(" /"))
            .filter(|l| {
                !l.split_once(' ')
                    .is_some_and(|(_, p)| p.starts_with("/api/plans/{pid}"))
            })
            .collect();
        let missing: Vec<_> = outside
            .iter()
            .filter(|r| !declared.contains_key(*r))
            .collect();
        assert!(
            missing.is_empty(),
            "add to plan-guard-census.txt: {missing:?}"
        );
        let stale: Vec<_> = declared.keys().filter(|r| !outside.contains(*r)).collect();
        assert!(stale.is_empty(), "no longer mounted: {stale:?}");
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
                return axa::Error::NotFound.into_response();
            }
            label = scheme.effective(mark.as_deref()).map(str::to_string);
        }
        let private: std::result::Result<Option<plan_access::Model>, sqlx::Error> = sqlx::query_as(
            "SELECT * FROM plan_access_settings WHERE plan_pid = $1 AND private = true LIMIT 1",
        )
        .bind(plan_pid)
        .fetch_optional(crate::db::pool(&ctx))
        .await;
        // Could not tell whether the plan is private: refuse rather than pass (CQ-6). This
        // used to fall through to the handler, which a non-member must not reach.
        if let Err(error) = &private {
            fail_closed("plan_access_settings", error);
            return super::problem(
                StatusCode::SERVICE_UNAVAILABLE,
                "unavailable",
                "could not check who may see this plan; try again",
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
                    return axa::Error::NotFound.into_response();
                }
                // Fail closed: a break-glass request that cannot be audited is not served, so the use
                // of the grant is never invisible.
                if let Err(error) = AuditModel::record_on(
                    crate::db::pool(&ctx),
                    plan_pid,
                    "break_glass_access",
                    sub,
                    Some(serde_json::json!({
                        "method": req.method().as_str(), "path": req.uri().path(),
                    })),
                )
                .await
                {
                    return super::audit_failed(error).into_response();
                }
            }
        }
    }
    let mut response = next.run(req).await;
    if let Some(l) = label
        && let Ok(v) = axum::http::HeaderValue::from_str(&crate::handling_marks::header_value(&l))
    {
        response.headers_mut().insert("x-handling-mark", v);
    }
    response
}

/// A plan's access row, if it has one.
async fn access_row(ctx: &AppContext, plan_pid: Uuid) -> Result<Option<plan_access::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plan_access_settings WHERE plan_pid = $1 LIMIT 1")
            .bind(plan_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// `GET /api/plans/{pid}/access`.
#[debug_handler]
async fn get_access(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = find_item(&ctx, &pid).await?;
    let row = access_row(&ctx, plan.pid).await?;
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "private": row.as_ref().is_some_and(|r| r.private),
        "members": row.as_ref().map(members_of).unwrap_or_default(),
    }))
}

/// `PUT /api/plans/{pid}/access` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AccessPayload {
    /// `true` hides the plan from everyone who is not a member or an admin.
    private: bool,
    /// User ids allowed to see a private plan; the caller is added automatically when making it private. Normalized by the rules module.
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
    let existing = access_row(&ctx, plan.pid).await?;
    let before = existing.as_ref().map(|r| (r.private, members_of(r)));
    let mut tx = crate::db::begin(&ctx).await?;
    match existing {
        Some(row) => {
            // `fetch_one`: an update that finds no row is an error, as before SQ-1.
            sqlx::query_scalar::<_, i64>(
                "UPDATE plan_access_settings SET private = $1, members = $2 WHERE id = $3 RETURNING id",
            )
            .bind(p.private)
            .bind(serde_json::json!(members))
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
        }
        None => {
            sqlx::query(
                "INSERT INTO plan_access_settings (pid, plan_pid, private, members) VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::new_v4())
            .bind(plan.pid)
            .bind(p.private)
            .bind(serde_json::json!(members))
            .execute(tx.conn())
            .await?;
        }
    }
    AuditModel::record_on(
        tx.conn(),
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
    tx.commit().await?;
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
