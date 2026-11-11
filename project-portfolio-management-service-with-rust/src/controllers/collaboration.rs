//! Collaborative review, the notification inbox, and assignee
//! management.
//!
//! - **Collaborative review** — delegate one idea / proposal / plan to
//!   the right internal or external expert, track whether they accepted,
//!   collect their verdict, and read the consensus
//!   ([`crate::collaboration`] holds the state machine and the
//!   aggregation).
//! - **Assignees** — assign or unassign a task, and read the open
//!   workload per assignee (including the unassigned pile).
//! - **Notifications** — the in-app inbox written by automations and by
//!   the scheduled-action sweep. In-app only: this service sends no
//!   email and no push.
//!
//! Reviewers are named by `EntityRef` URN (`person:` / `worker:` /
//! `organization:`), never by raw email — an outside expert is still a
//! record in the person registry, so no contact detail is duplicated
//! here. `reviewer_scope` records the internal/external distinction
//! explicitly, because sharing something with an outsider is a
//! disclosure decision and must never be inferred.
//!
//! Every mutation writes an `audit_logs` row.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::governance::valid_ref;
use crate::auth::MaybeAuthUser;
use crate::collaboration as rules;
use crate::models::_entities::{notifications, reviews, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;
use crate::validation::MAX_TEXT_LEN;

/// Default page size for every list endpoint here — the cap each one
/// applied before paging (agents/restful.md) landed; a `limit` above
/// [`super::pagination::MAX_LIMIT`] is a `422`.
pub const LIST_CAP: u64 = 200;

/// A reviewer / recipient reference: an `EntityRef` over the family's
/// person-bearing services.
fn valid_actor_ref(value: &str) -> bool {
    valid_ref(value, &["person", "worker", "organization"])
}

/// Confirm the review subject actually exists and is live, so an
/// invitation can never point at nothing.
async fn subject_exists(ctx: &AppContext, subject_kind: &str, subject_pid: Uuid) -> Result<bool> {
    let sql = match subject_kind {
        "idea" => "SELECT 1 FROM ideas WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        "proposal" => "SELECT 1 FROM proposals WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        "plan" => "SELECT 1 FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        _ => return Ok(false),
    };
    Ok(sqlx::query_scalar::<_, i32>(sql)
        .bind(subject_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .is_some())
}

/// The plan a review subject belongs to: a plan itself, or the plan a proposal became. An idea, and a proposal still in the
/// pipeline, belong to none.
async fn subject_plan(ctx: &AppContext, kind: &str, pid: Uuid) -> Result<Option<Uuid>> {
    Ok(match kind {
        "plan" => Some(pid),
        "proposal" => sqlx::query_scalar::<_, Option<Uuid>>(
            "SELECT promoted_plan_pid FROM proposals WHERE pid = $1 LIMIT 1",
        )
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .flatten(),
        _ => None,
    })
}

/// `404` unless the caller can read this review's subject, or is the reviewer: being invited is the disclosure decision
/// (`reviewer_scope`), so an expert who is not on a private plan's access list can still answer.
async fn require_review_visible(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    review: &reviews::Model,
) -> Result<()> {
    if owns_inbox(caller, &review.reviewer_ref) {
        return Ok(());
    }
    let plan = subject_plan(ctx, &review.subject_kind, review.subject_pid).await?;
    super::plan_access::require_readable_if_any(ctx, caller, plan).await
}

/// `POST /api/reviews` body — delegate a subject to one expert.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct InvitePayload {
    /// Kind of thing to review (one of the allowed review subject kinds).
    subject_kind: String,
    /// Public id of the subject; it must name a live row of that kind.
    subject_pid: Uuid,
    /// Person, worker or organization reference of the invited expert (never a raw email).
    reviewer_ref: String,
    #[serde(default)]
    /// `internal` when absent, otherwise one of the allowed reviewer scopes.
    reviewer_scope: Option<String>,
    #[serde(default)]
    /// Optional free-text area of expertise sought; if present it must be non-blank.
    expertise: Option<String>,
    #[serde(default)]
    /// Optional date the verdict is due; absent means no deadline.
    due_on: Option<chrono::NaiveDate>,
}

/// Validate an invitation payload, reporting every problem at once.
fn invite_problems(payload: &InvitePayload, scope: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if !rules::is_token(rules::REVIEW_SUBJECT_KINDS, &payload.subject_kind) {
        problems.push(format!(
            "subject_kind must be one of {:?}",
            rules::REVIEW_SUBJECT_KINDS
        ));
    }
    if !valid_actor_ref(&payload.reviewer_ref) {
        problems.push(
            "reviewer_ref must be a person:/worker:/organization: URN (never a raw email)"
                .to_string(),
        );
    }
    if !rules::is_token(rules::REVIEWER_SCOPES, scope) {
        problems.push(format!(
            "reviewer_scope must be one of {:?}",
            rules::REVIEWER_SCOPES
        ));
    }
    if let Some(expertise) = payload.expertise.as_deref()
        && (expertise.trim().is_empty() || expertise.len() > MAX_TEXT_LEN)
    {
        problems.push(format!(
            "expertise, when set, must be non-blank and at most {MAX_TEXT_LEN} characters"
        ));
    }
    problems
}

/// `POST /api/reviews` — invite one expert to assess one subject.
///
/// A second live invitation for the same (subject, reviewer) is
/// refused rather than stacked: re-delegating the same item to the same
/// person is almost always a mistake, and the partial unique index
/// would reject it anyway. The invitation and its audit row commit
/// together; the reviewer notification is best effort afterwards.
#[debug_handler]
async fn invite_review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<InvitePayload>,
) -> Result<Response> {
    let scope = payload
        .reviewer_scope
        .clone()
        .unwrap_or_else(|| "internal".to_string());
    let problems = invite_problems(&payload, &scope);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let plan = subject_plan(&ctx, &payload.subject_kind, payload.subject_pid).await?;
    let readable = super::plan_access::require_readable_if_any(&ctx, &caller, plan).await;
    // The same answer for a plan that is hidden and one that does not exist.
    if readable.is_err()
        || !subject_exists(&ctx, &payload.subject_kind, payload.subject_pid).await?
    {
        return Err(unprocessable(&format!(
            "no live {} with pid {}",
            payload.subject_kind, payload.subject_pid
        )));
    }
    let live = cap_models::reviews_for_subject(&ctx, &payload.subject_kind, payload.subject_pid)
        .await?
        .into_iter()
        .any(|r| {
            r.reviewer_ref == payload.reviewer_ref
                && rules::LIVE_REVIEW_STATUSES.contains(&r.status.as_str())
        });
    if live {
        return Err(unprocessable(
            "this reviewer already has a live invitation for this subject",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: reviews::Model = sqlx::query_as(
        "INSERT INTO reviews
             (pid, subject_kind, subject_pid, reviewer_ref, reviewer_scope, expertise, status, due_on, score,
              recommendation, comment, invited_by, responded_at, submitted_at, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'invited', $7, NULL, NULL, NULL, $8, NULL, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(&payload.subject_kind)
    .bind(payload.subject_pid)
    .bind(&payload.reviewer_ref)
    .bind(&scope)
    .bind(payload.expertise.as_deref())
    .bind(payload.due_on)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "review_invited",
        caller.actor(),
        Some(serde_json::json!({
            "subject_kind": payload.subject_kind,
            "subject_pid": payload.subject_pid.to_string(),
            "reviewer_ref": payload.reviewer_ref,
            "reviewer_scope": scope,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    // The reviewer learns they have been asked.
    cap_models::notify_on(
        &mut crate::db::pool(&ctx),
        &payload.reviewer_ref,
        &payload.subject_kind,
        payload.subject_pid,
        "review_invited",
        &format!(
            "You have been asked to review a {}{}",
            payload.subject_kind,
            payload
                .due_on
                .map(|d| format!(" by {d}"))
                .unwrap_or_default()
        ),
    )
    .await
    .ok();
    format::json(row)
}

/// `GET /api/reviews` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReviewParams {
    #[serde(default)]
    /// Filter by subject kind; absent means all kinds.
    subject_kind: Option<String>,
    #[serde(default)]
    /// Filter by subject pid; absent means all subjects.
    subject_pid: Option<Uuid>,
    #[serde(default)]
    /// Filter by reviewer reference; absent means all reviewers.
    reviewer: Option<String>,
    #[serde(default)]
    /// Filter by review status; absent means all statuses.
    status: Option<String>,
}

/// `GET /api/reviews?subject_kind=&subject_pid=&reviewer=&status=&limit=&cursor=`
/// — the delegation list, newest first. Paged by cursor (agents/restful.md):
/// `Link: rel="next"` while more rows follow, and no total.
///
/// # Errors
///
/// `422` for a bad `limit` or `cursor`, and `400` for a cursor past [`super::pagination::MAX_OFFSET`]; a DB
/// error when the query fails.
#[debug_handler]
async fn list_reviews(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
    Query(params): Query<ReviewParams>,
) -> Result<Response> {
    let page = cursor.page(LIST_CAP)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let mut query =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM reviews WHERE deleted_at IS NULL");
    let hidden = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .plan_pids();
    if !hidden.is_empty() {
        // In the query, not after it, so the page and its total stay true.
        let proposals: Vec<Uuid> =
            sqlx::query_scalar("SELECT pid FROM proposals WHERE promoted_plan_pid = ANY($1)")
                .bind(&hidden)
                .fetch_all(crate::db::pool(&ctx))
                .await?;
        // The same people `owns_inbox` lets answer.
        let mut mine: Vec<String> = caller
            .actor()
            .map(str::to_lowercase)
            .map(|sub| vec![format!("person:{sub}"), format!("worker:{sub}")])
            .unwrap_or_default();
        mine.extend(
            super::organizations::caller_organizations(&caller)
                .iter()
                .map(|o| format!("organization:{o}")),
        );
        // Not a review of a hidden plan or of a proposal that became one, unless it is the caller's to answer.
        query
            .push(" AND (NOT ((subject_kind = 'plan' AND subject_pid = ANY(")
            .push_bind(hidden)
            .push(")) OR (subject_kind = 'proposal' AND subject_pid = ANY(")
            .push_bind(proposals)
            .push("))) OR reviewer_ref = ANY(")
            .push_bind(mine)
            .push("))");
    }
    if let Some(kind) = params.subject_kind {
        query.push(" AND subject_kind = ").push_bind(kind);
    }
    if let Some(pid) = params.subject_pid {
        query.push(" AND subject_pid = ").push_bind(pid);
    }
    if let Some(reviewer) = params.reviewer {
        query.push(" AND reviewer_ref = ").push_bind(reviewer);
    }
    if let Some(status) = params.status {
        query.push(" AND status = ").push_bind(status);
    }
    query
        .push(" ORDER BY id DESC LIMIT ")
        .push_bind(i64::try_from(limit).unwrap_or(i64::MAX))
        .push(" OFFSET ")
        .push_bind(i64::try_from(offset).unwrap_or(i64::MAX));
    let rows: Vec<reviews::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    Ok(page.respond(rows))
}

/// `POST /api/reviews/{pid}/respond` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RespondPayload {
    /// `accept` or `decline`.
    response: String,
}

/// `POST /api/reviews/{pid}/respond` — the reviewer accepts or
/// declines the invitation.
#[debug_handler]
async fn respond_review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<RespondPayload>,
) -> Result<Response> {
    let target = match payload.response.trim() {
        "accept" => "accepted",
        "decline" => "declined",
        other => {
            return Err(unprocessable(&format!(
                "response must be `accept` or `decline`, got `{other}`"
            )));
        }
    };
    let review = cap_models::find_review(&ctx, crate::models::governance::parse_pid(&pid)?).await?;
    require_review_visible(&ctx, &caller, &review).await?;
    rules::review_transition(&review.status, target).map_err(|e| unprocessable(&e))?;
    let review_pid = review.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    let row: reviews::Model = sqlx::query_as(
        "UPDATE reviews SET status = $1, responded_at = $2 WHERE id = $3 RETURNING *",
    )
    .bind(target)
    .bind(chrono::Utc::now())
    .bind(review.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        review_pid,
        &format!("review_{target}"),
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/reviews/{pid}/submit` body — the expert's verdict.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct VerdictPayload {
    #[serde(default)]
    /// Optional score from 0 to 100; null means the reviewer gave none, not zero.
    score: Option<i32>,
    /// One of the allowed recommendation tokens.
    recommendation: String,
    #[serde(default)]
    /// Optional free-text comment, length-capped.
    comment: Option<String>,
}

/// `POST /api/reviews/{pid}/submit` — record the verdict. Only a
/// reviewer who accepted may submit, so an unanswered invitation never
/// becomes evidence. Submitting fires any `review_submitted`
/// automations.
#[debug_handler]
async fn submit_review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<VerdictPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if !rules::is_token(rules::RECOMMENDATIONS, payload.recommendation.trim()) {
        problems.push(format!(
            "recommendation must be one of {:?}",
            rules::RECOMMENDATIONS
        ));
    }
    if let Some(score) = payload.score
        && !rules::valid_review_score(score)
    {
        problems.push("score, when given, must be between 0 and 100".to_string());
    }
    if let Some(comment) = payload.comment.as_deref()
        && comment.len() > MAX_TEXT_LEN
    {
        problems.push(format!("comment exceeds {MAX_TEXT_LEN} characters"));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let review = cap_models::find_review(&ctx, crate::models::governance::parse_pid(&pid)?).await?;
    require_review_visible(&ctx, &caller, &review).await?;
    rules::review_transition(&review.status, "submitted").map_err(|e| unprocessable(&e))?;
    let (review_pid, subject_kind, subject_pid, invited_by) = (
        review.pid,
        review.subject_kind.clone(),
        review.subject_pid,
        review.invited_by.clone(),
    );
    let mut tx = crate::db::begin(&ctx).await?;
    let row: reviews::Model = sqlx::query_as(
        "UPDATE reviews SET status = 'submitted', score = $1, recommendation = $2, comment = $3, submitted_at = $4
         WHERE id = $5
         RETURNING *",
    )
    .bind(payload.score)
    .bind(payload.recommendation.trim())
    .bind(payload.comment.as_deref())
    .bind(chrono::Utc::now())
    .bind(review.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        review_pid,
        "review_submitted",
        caller.actor(),
        Some(serde_json::json!({
            "recommendation": row.recommendation,
            "score": row.score,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    // Tell whoever delegated it that the verdict has landed.
    if let Some(invited_by) = invited_by.as_deref()
        && valid_actor_ref(invited_by)
    {
        cap_models::notify_on(
            &mut crate::db::pool(&ctx),
            invited_by,
            &subject_kind,
            subject_pid,
            "review_submitted",
            &format!(
                "A review you delegated came back: {}",
                row.recommendation.as_deref().unwrap_or("submitted")
            ),
        )
        .await
        .ok();
    }
    // A review verdict is a trigger like a board move is.
    if subject_kind == "plan" {
        super::automation::fire(
            &ctx,
            &crate::automation::TriggerFact {
                kind: "review_submitted".to_string(),
                plan_pid: subject_pid,
                from_status: None,
                to_status: None,
            },
            "review",
            review_pid,
            caller.actor(),
        )
        .await;
    }
    format::json(row)
}

/// `DELETE /api/reviews/{pid}` — withdraw an invitation (soft delete).
/// A submitted verdict cannot be withdrawn: the record of what an
/// expert said stands.
#[debug_handler]
async fn withdraw_review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let review = cap_models::find_review(&ctx, crate::models::governance::parse_pid(&pid)?).await?;
    require_review_visible(&ctx, &caller, &review).await?;
    rules::review_transition(&review.status, "withdrawn").map_err(|e| unprocessable(&e))?;
    let review_pid = review.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query_scalar::<_, i32>(
        "UPDATE reviews SET status = 'withdrawn', deleted_at = $1 WHERE id = $2 RETURNING id",
    )
    .bind(chrono::Utc::now())
    .bind(review.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        review_pid,
        "review_withdrawn",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `GET /api/reviews/consensus` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ConsensusParams {
    /// Kind of the subject whose verdicts are aggregated.
    subject_kind: String,
    /// Public id of the subject.
    subject_pid: Uuid,
}

/// `GET /api/reviews/consensus?subject_kind=&subject_pid=` — the
/// aggregate verdict on one subject. Outstanding invitations are
/// reported, and a tie is reported as a tie.
#[debug_handler]
async fn review_consensus(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<ConsensusParams>,
) -> Result<Response> {
    if !rules::is_token(rules::REVIEW_SUBJECT_KINDS, &params.subject_kind) {
        return Err(unprocessable(&format!(
            "subject_kind must be one of {:?}",
            rules::REVIEW_SUBJECT_KINDS
        )));
    }
    let plan = subject_plan(&ctx, &params.subject_kind, params.subject_pid).await?;
    super::plan_access::require_readable_if_any(&ctx, &caller, plan).await?;
    let rows =
        cap_models::reviews_for_subject(&ctx, &params.subject_kind, params.subject_pid).await?;
    format::json(consensus_of(&rows))
}

/// Aggregate stored review rows into the consensus view.
pub(crate) fn consensus_of(rows: &[reviews::Model]) -> rules::Consensus {
    let verdicts: Vec<rules::Verdict> = rows
        .iter()
        .filter(|r| r.status == "submitted")
        .map(|r| rules::Verdict {
            score: r.score,
            recommendation: r
                .recommendation
                .clone()
                .unwrap_or_else(|| "hold".to_string()),
            scope: r.reviewer_scope.clone(),
        })
        .collect();
    let outstanding = rows
        .iter()
        .filter(|r| rules::LIVE_REVIEW_STATUSES.contains(&r.status.as_str()))
        .count();
    let declined = rows.iter().filter(|r| r.status == "declined").count();
    rules::consensus(&verdicts, outstanding, declined)
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/assign` body. A `null` /
/// absent `assignee_ref` unassigns.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AssignPayload {
    #[serde(default)]
    /// Person or worker reference to assign; null or blank unassigns the task.
    assignee_ref: Option<String>,
}

/// `POST /api/plans/{pid}/tasks/{t_pid}/assign` — assign or unassign
/// one task.
#[debug_handler]
async fn assign_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(payload): Json<AssignPayload>,
) -> Result<Response> {
    let assignee = payload
        .assignee_ref
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let Some(assignee) = assignee
        && !valid_ref(assignee, &["person", "worker"])
    {
        return Err(unprocessable(
            "assignee_ref must be a person:/worker: URN, or null to unassign",
        ));
    }
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let task_pid = crate::models::governance::parse_pid(&t_pid)?;
    let task: tasks::Model = sqlx::query_as(
        "SELECT * FROM tasks WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(task_pid)
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let previous = task.assignee_ref.clone();
    let mut tx = crate::db::begin(&ctx).await?;
    let row: tasks::Model =
        sqlx::query_as("UPDATE tasks SET assignee_ref = $1 WHERE id = $2 RETURNING *")
            .bind(assignee)
            .bind(task.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        task_pid,
        "task_assigned",
        caller.actor(),
        Some(serde_json::json!({ "from": previous, "to": row.assignee_ref })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    if let Some(assignee) = row.assignee_ref.as_deref() {
        cap_models::notify_on(
            &mut crate::db::pool(&ctx),
            assignee,
            "task",
            task_pid,
            "task_assigned",
            &format!("You were assigned: {}", row.title),
        )
        .await
        .ok();
        // Being assigned follows the task (unless the assignee opted out).
        if let Some(sub) = crate::notification_prefs::recipient_sub(assignee) {
            super::watchers::ensure_watching(&ctx, "task", task_pid, &sub).await;
        }
    }
    format::json(row)
}

/// `GET /api/assignees/workload` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct WorkloadParams {
    /// Restrict to one plan's board; absent = every plan.
    #[serde(default)]
    plan: Option<Uuid>,
}

/// `GET /api/assignees/workload?plan=` — open work per assignee,
/// busiest first, including the `unassigned` pile. ETag-conditional.
#[debug_handler]
async fn assignee_workload(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
    Query(params): Query<WorkloadParams>,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut query =
        sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM tasks WHERE deleted_at IS NULL");
    if let Some(plan) = params.plan {
        query.push(" AND plan_pid = ").push_bind(plan);
    }
    let rows: Vec<tasks::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let rows = scope.keep(rows);
    let facts: Vec<rules::TaskLoadFact> = rows
        .iter()
        .map(|t| rules::TaskLoadFact {
            assignee_ref: t.assignee_ref.clone(),
            status: t.status.clone(),
            points: t.points,
        })
        .collect();
    let body = serde_json::json!({ "assignees": rules::workload(&facts) });
    let etag = super::etag_of(&body);
    super::conditional_json(
        &headers,
        &etag,
        &serde_json::json!({
            "assignees": body["assignees"],
            "as_of": chrono::Utc::now().to_rfc3339(),
        }),
    )
}

/// `GET /api/notifications` filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NotificationParams {
    /// Whose inbox to read: the caller's own, or that of an organization they belong to.
    recipient: String,
    /// `true` ⇒ only unread.
    #[serde(default)]
    unread: Option<bool>,
}

/// Whether an inbox belongs to the caller: their own `person:`/`worker:` reference, or an organization they belong to.
/// Not even an administrator reads another person's: a message can quote a plan the token's clearance does not reach, and
/// no screen needs it. With login off there is no caller to refuse.
fn owns_inbox(caller: &MaybeAuthUser, recipient: &str) -> bool {
    let organizations = super::organizations::caller_organizations(caller);
    caller.holds_or_open(|claims| {
        crate::notification_prefs::recipient_sub(recipient)
            .is_some_and(|id| id.eq_ignore_ascii_case(&claims.sub))
            || recipient
                .strip_prefix("organization:")
                .and_then(|id| Uuid::parse_str(id).ok())
                .is_some_and(|id| organizations.contains(&id))
    })
}

/// `GET /api/notifications?recipient=&unread=&limit=&cursor=` — the
/// caller's own inbox, newest first. Paged by cursor (agents/restful.md):
/// `Link: rel="next"` while more rows follow, and no total.
///
/// # Errors
///
/// `403` when the inbox is not the caller's; `422` when `recipient` is not
/// a valid actor ref; `422` for a bad `limit` or `cursor`, and `400` for a cursor past
///[`super::pagination::MAX_OFFSET`]; a DB error when the query fails.
#[debug_handler]
async fn list_notifications(
    State(ctx): State<AppContext>,
    cursor: super::pagination::Cursor,
    caller: MaybeAuthUser,
    Query(params): Query<NotificationParams>,
) -> Result<Response> {
    if !valid_actor_ref(&params.recipient) {
        return Err(unprocessable(
            "recipient must be a person:/worker:/organization: URN",
        ));
    }
    if !owns_inbox(&caller, &params.recipient) {
        return Err(super::forbidden("this inbox is not yours"));
    }
    let page = cursor.page(LIST_CAP)?;
    let (limit, offset) = (page.fetch(), page.offset());
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM notifications WHERE recipient_ref = ",
    );
    query.push_bind(params.recipient.as_str());
    query.push(" AND deleted_at IS NULL");
    scope.notes_visible(&ctx, &mut query).await?;
    if params.unread.unwrap_or(false) {
        query.push(" AND read_at IS NULL");
    }
    query
        .push(" ORDER BY id DESC LIMIT ")
        .push_bind(i64::try_from(limit).unwrap_or(i64::MAX))
        .push(" OFFSET ")
        .push_bind(i64::try_from(offset).unwrap_or(i64::MAX));
    let rows: Vec<notifications::Model> = query
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    Ok(page.respond(rows))
}

/// `POST /api/notifications/{pid}/read` — mark one of the caller's own
/// notifications read (idempotent: the first stamp stands). Someone
/// else's is `404`, the same as one that does not exist.
#[debug_handler]
async fn read_notification(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row =
        cap_models::find_notification(&ctx, crate::models::governance::parse_pid(&pid)?).await?;
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    if !owns_inbox(&caller, &row.recipient_ref) || scope.hides_note(&ctx, &row).await? {
        return Err(Error::NotFound);
    }
    if row.read_at.is_some() {
        return format::json(row);
    }
    let row: notifications::Model =
        sqlx::query_as("UPDATE notifications SET read_at = $1 WHERE id = $2 RETURNING *")
            .bind(chrono::Utc::now())
            .bind(row.id)
            .fetch_one(crate::db::pool(&ctx))
            .await?;
    format::json(row)
}

/// The collaboration routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/reviews", post(invite_review))
        .add("/reviews", get(list_reviews))
        .add("/reviews/consensus", get(review_consensus))
        .add("/reviews/{pid}/respond", post(respond_review))
        .add("/reviews/{pid}/submit", post(submit_review))
        .add("/reviews/{pid}", delete(withdraw_review))
        .add("/plans/{pid}/tasks/{t_pid}/assign", post(assign_task))
        .add("/assignees/workload", get(assignee_workload))
        .add("/notifications", get(list_notifications))
        .add("/notifications/{pid}/read", post(read_notification))
}
