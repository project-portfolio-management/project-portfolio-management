//! HTTP controllers for the portfolio service.

pub mod access_review;
pub mod announcements;
pub mod assignments;
pub mod automation;
/// Validated benefits (DP-9).
pub mod benefit_validation;
pub mod break_glass;
/// Per-user saved views: route-scoped filter/sort/column presets,
/// keyed by the token `sub` (T-28l).
pub mod canvas;
/// Skill pools, capacity commitments and the capacity heatmap (DP-10).
pub mod capacity_heatmap;
/// Sprint ceremonies and the commitment snapshot.
pub mod ceremony;
pub mod change_requests;
pub mod collaboration;
/// Integrity-verification endpoints.
pub mod compliance;
pub mod confidence;
/// The Controlling-process register: standards, readings, actions, and
/// the coverage report of what is **not** being controlled.
pub mod controls;
pub mod director_packs;
pub mod director_summary;
pub mod discussion;
/// Flow Distribution: the feature / defect / risk / debt mix.
pub mod distribution;
pub mod docs;
/// Recorded effort and utilization, including per person.
pub mod effort;
pub mod engineering;
pub mod exceptions_digest;
/// The buffer fever chart (DP-7).
pub mod fever;
/// The phased budget baseline and the EAC/ETC forecast (T-28b).
pub mod financials;
/// Funding sources and financial-year money (DP-6).
pub mod funding;
pub mod governance;
pub mod handling_marks;
pub mod insights;
pub mod issue_configuration;
pub mod issues;
pub mod lessons;
pub mod metrics;
pub mod modules;
pub mod my_work;
pub mod notification_prefs;
/// The OKR engine: key results, check-ins, and the derived objective
/// and alignment-weighted plan scores.
pub mod okr;
pub mod onboarding;
pub mod oversight;
/// The monthly rhythm of the director pack: schedule, review, issue, notify (DP-12).
pub mod pack_rhythm;
/// The sequential project phase: one-step advancement, explicitly
/// reasoned regression, and per-phase durations.
pub mod phase;
pub mod plans;
pub mod prioritization;
/// Planned gate dates, gate decisions in a period and programme status (DP-5).
pub mod programme_gates;
pub mod sandbox;
pub mod saved_views;
pub mod scoring;
pub mod settings;
pub mod status_updates;
pub mod strategy;
/// Time-based analysis: the read surface over the task transition log —
/// per-task and plan flow, constraints, aging WIP, and Little's Law.
pub mod tba;
/// Total Project Control: DIPP, expected monetary value, cost estimate
/// to complete, and portfolio triage by DIPP.
pub mod tpc;
/// Realized gains and strategic performance.
pub mod value;
pub mod visibility;
/// Custom workflows: the configurable task and issue state
/// vocabularies, and the resolution of which one is in force.
pub mod workflow;

/// Map a model-layer error to its HTTP shape: a missing record is
/// `404 Not Found`; anything else stays a model error (500-class).
/// axa does not map `ModelError::EntityNotFound` itself (its
/// `IntoResponse` catch-all turns it into a 500), so every controller
/// lookup routes through this instead of a bare `?`.
#[must_use]
pub fn model_not_found(err: axa::model::ModelError) -> axa::Error {
    match err {
        axa::model::ModelError::EntityNotFound => axa::Error::NotFound,
        other => axa::Error::Model(other),
    }
}

/// A database error as the service's error type (`500`). One definition for every controller.
#[must_use]
pub(crate) fn db_err(e: sea_orm::DbErr) -> axa::Error {
    axa::Error::Model(axa::model::ModelError::from(e))
}

/// A write's audit row could not be written, so the write is refused (NFR-6): the audit row is written in
/// the same transaction as the change, and the transaction is rolled back when this is returned. Logged at
/// error level and counted in `audit_write_failed_total`, which an operator should alert on.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "passed to `map_err`, which hands it over"
)]
pub(crate) fn audit_failed(e: axa::model::ModelError) -> axa::Error {
    crate::metrics::Metrics::global()
        .audit_write_failed_total
        .inc();
    tracing::error!(error = %e, "audit row not written; the write is refused");
    axa::Error::CustomError(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        axa::error::ErrorDetail::new(
            "audit_unavailable",
            "the change was not made because it could not be recorded; try again",
        ),
    )
}

/// `422 Unprocessable Entity` with the error code `unprocessable` and `message`.
///
/// Controllers that answer with the code `unprocessable_entity` keep their own copy: the code string is part of
/// their responses and changing it would change the API.
#[must_use]
pub(crate) fn unprocessable(message: &str) -> axa::Error {
    axa::Error::CustomError(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        axa::error::ErrorDetail::new("unprocessable", message),
    )
}

/// `422 Unprocessable Entity` with the error code `unprocessable_entity` and `message`: the code most
/// controllers answer with (CQ-7). Which of the two codes survives is plan §70 question 3; until then
/// both exist, defined once each.
#[must_use]
pub(crate) fn unprocessable_entity(message: &str) -> axa::Error {
    axa::Error::CustomError(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        axa::error::ErrorDetail::new("unprocessable_entity", message),
    )
}

/// A pid from the path, or `404` when it is not a UUID (a malformed id names nothing).
///
/// # Errors
///
/// `404` when `raw` is not a UUID.
pub(crate) fn parse_pid(raw: &str) -> axa::Result<uuid::Uuid> {
    uuid::Uuid::parse_str(raw).map_err(|_| axa::Error::NotFound)
}

/// A live (not deleted) plan by its path pid, or `404` (CQ-7: one definition for every controller).
///
/// # Errors
///
/// `404` when the pid is malformed or names no live plan; `500` on a database error.
pub(crate) async fn find_live_plan(
    ctx: &axa::AppContext,
    raw: &str,
) -> axa::Result<crate::models::_entities::plans::Model> {
    use crate::models::_entities::plans;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(parse_pid(raw)?))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(axa::Error::NotFound)
}

/// A live task of `plan` by its path pid, or `404`; a task of another plan is `404` too.
///
/// # Errors
///
/// `404` when the pid is malformed or names no live task of `plan`; `500` on a database error.
pub(crate) async fn find_live_task(
    ctx: &axa::AppContext,
    plan: uuid::Uuid,
    raw: &str,
) -> axa::Result<crate::models::_entities::tasks::Model> {
    use crate::models::_entities::tasks;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(parse_pid(raw)?))
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(axa::Error::NotFound)
}

/// Take the transaction-scoped advisory lock for `scope` and `key` (CQ-12): the key is
/// `hashtext('<scope>:<key>')`, bound as a parameter rather than formatted into the SQL. Two
/// places that must exclude each other (a what-if commit and a plain schedule edit share
/// `sandbox`) use the same scope. `key` is usually a pid; availability locks on a person ref.
///
/// # Errors
///
/// `500` when the lock statement fails.
pub(crate) async fn advisory_lock<C: sea_orm::ConnectionTrait>(
    txn: &C,
    scope: &'static str,
    key: impl std::fmt::Display,
) -> axa::Result<()> {
    txn.execute_raw(sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT pg_advisory_xact_lock(hashtext($1))",
        [sea_orm::Value::from(format!("{scope}:{key}"))],
    ))
    .await
    .map_err(db_err)?;
    Ok(())
}

/// [`advisory_lock`] on a `sqlx` transaction ([`crate::db::begin`]): the same key, so the two sides exclude each other.
///
/// # Errors
///
/// `500` when the lock statement fails.
pub(crate) async fn advisory_lock_on(
    tx: &mut crate::db::Tx,
    scope: &'static str,
    key: impl std::fmt::Display,
) -> axa::Result<()> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1))")
        .bind(format!("{scope}:{key}"))
        .execute(tx.conn())
        .await?;
    Ok(())
}

/// `409 Conflict` with the error code `conflict` and `message`.
#[must_use]
pub(crate) fn conflict(message: &str) -> axa::Error {
    axa::Error::CustomError(
        axum::http::StatusCode::CONFLICT,
        axa::error::ErrorDetail::new("conflict", message),
    )
}

/// `403 Forbidden` with the error code `forbidden` and `message`.
#[must_use]
pub(crate) fn forbidden(message: &str) -> axa::Error {
    axa::Error::CustomError(
        axum::http::StatusCode::FORBIDDEN,
        axa::error::ErrorDetail::new("forbidden", message),
    )
}

/// An error with any status and code.
#[must_use]
pub(crate) fn refuse(status: axum::http::StatusCode, code: &str, message: &str) -> axa::Error {
    axa::Error::CustomError(status, axa::error::ErrorDetail::new(code, message))
}

/// An RFC 9457 problem with this service's machine-readable `code`, as every error the service answers is (spec §8.3a).
/// For a refusal that needs a header or an extension member of its own; anything else returns an `axa::Error`.
#[must_use]
pub fn problem(
    status: axum::http::StatusCode,
    code: &str,
    detail: impl Into<String>,
) -> axa::problem::Problem {
    axa::problem::Problem::new(status)
        .with_detail(detail)
        .with_extension("code", serde_json::Value::from(code))
}

/// Map a record-level authorization rejection (`(status, reason)`) to
/// an axa error: `403` = policy denied; `401` = fail-safe when claims
/// are missing behind the guard.
#[must_use]
pub fn record_rejection((status, reason): (axum::http::StatusCode, String)) -> axa::Error {
    let code = if status == axum::http::StatusCode::FORBIDDEN {
        "forbidden"
    } else {
        "unauthorized"
    };
    axa::Error::CustomError(status, axa::error::ErrorDetail::new(code, reason))
}

/// Weak `ETag` over a serializable view (everything except `as_of`).
#[must_use]
pub fn etag_of<T: serde::Serialize>(value: &T) -> String {
    use std::hash::{Hash, Hasher};
    let json = serde_json::to_string(value).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    json.hash(&mut hasher);
    format!("W/\"{:016x}\"", hasher.finish())
}

/// Serve `body` as JSON with `etag`, honouring `If-None-Match` → `304`
/// (the patient-flow conditional-read pattern).
///
/// # Errors
///
/// When the body fails to serialize.
pub fn conditional_json<T: serde::Serialize>(
    headers: &axum::http::HeaderMap,
    etag: &str,
    body: &T,
) -> axa::Result<axum::response::Response> {
    use axum::http::{StatusCode, header};
    use axum::response::IntoResponse;
    let matched = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.trim() == etag);
    let etag_value = header::HeaderValue::from_str(etag)
        .unwrap_or_else(|_| header::HeaderValue::from_static("W/\"0\""));
    if matched {
        return Ok((StatusCode::NOT_MODIFIED, [(header::ETAG, etag_value)]).into_response());
    }
    Ok((
        StatusCode::OK,
        [(header::ETAG, etag_value)],
        axum::Json(serde_json::to_value(body).map_err(|e| axa::Error::Any(e.into()))?),
    )
        .into_response())
}

/// Family-wide collection-read paging (FR-46, `agents/restful.md`): `?limit=` and an opaque, signed `?cursor=` from
/// `axa-cursor-pagination`, a bare JSON array, and `Link: <…>; rel="next"` while more rows follow. There is no total,
/// because a total would count rows the caller's record policy hides, and no `?offset=`, because the position is the
/// service's to give, not the caller's.
///
/// Every collection-read controller and the paging layer share this one implementation.
pub mod pagination {
    use std::sync::Arc;

    use axa::cursor_pagination::{CursorKey, CursorLimits, CursorPage};
    use axum::extract::{FromRequestParts, OriginalUri};
    use axum::http::{HeaderValue, Uri, header::LINK, request::Parts};
    use axum::response::Response;

    use crate::cursor::Cursors;
    use crate::prelude::*;

    /// Largest page any collection read serves, and the page when none is asked for; a bigger `limit` is a `422`.
    pub const MAX_LIMIT: u64 = 500;

    /// Furthest position a cursor may reach; past it a request is a `400` (SEC-G7) rather than a query that makes the
    /// database materialize arbitrarily many rows just to discard them. A list that long needs a narrower query.
    pub const MAX_OFFSET: u64 = 10_000;

    /// On a response its handler paged, so the paging layer leaves it alone.
    #[derive(Debug, Clone, Copy)]
    pub struct Paged;

    /// What a handler needs to page: the key cursors are signed with, and the request's whole URI.
    #[derive(Debug, Clone)]
    pub struct Cursor {
        key: Arc<Cursors>,
        uri: Uri,
    }

    impl FromRequestParts<AppContext> for Cursor {
        type Rejection = std::convert::Infallible;

        fn from_request_parts(
            parts: &mut Parts,
            ctx: &AppContext,
        ) -> impl Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
            // The original URI, with the `/api` prefix, which is the one the `Link` must name.
            let uri = parts
                .extensions
                .get::<OriginalUri>()
                .map_or_else(|| parts.uri.clone(), |original| original.0.clone());
            std::future::ready(Ok(Self {
                key: Cursors::of(ctx),
                uri,
            }))
        }
    }

    impl Cursor {
        /// A cursor reader for `uri`, signed with `key`.
        #[must_use]
        pub const fn new(key: Arc<Cursors>, uri: Uri) -> Self {
            Self { key, uri }
        }

        /// The page asked for: `default_limit` rows when no `limit` is given, at most [`MAX_LIMIT`].
        ///
        /// # Errors
        ///
        /// As [`page_with`].
        pub fn page(&self, default_limit: u64) -> axa::Result<Page> {
            page_with(&self.key.0, &self.uri, default_limit, MAX_LIMIT)
        }

        /// As [`Cursor::page`], with a ceiling of the endpoint's own.
        ///
        /// # Errors
        ///
        /// As [`page_with`].
        pub fn page_up_to(&self, default_limit: u64, max_limit: u64) -> axa::Result<Page> {
            page_with(&self.key.0, &self.uri, default_limit, max_limit)
        }
    }

    /// The page `uri` asks for.
    ///
    /// # Errors
    ///
    /// `422` when the limit is zero, not a number, given twice or above `max_limit`, or the cursor was not signed with
    /// `key` for this path and these filters; `400` `offset_too_large` when the cursor is past [`MAX_OFFSET`].
    pub fn page_with(
        key: &CursorKey,
        uri: &Uri,
        default_limit: u64,
        max_limit: u64,
    ) -> axa::Result<Page> {
        let page = CursorPage::new(
            key,
            CursorLimits {
                default_limit,
                max_limit,
            },
            uri,
        )?;
        if page.offset() > MAX_OFFSET {
            return Err(axa::Error::CustomError(
                axum::http::StatusCode::BAD_REQUEST,
                axa::error::ErrorDetail::new(
                    "offset_too_large",
                    format!(
                        "a list is read at most {MAX_OFFSET} rows deep; narrow the query instead"
                    ),
                ),
            ));
        }
        Ok(Page(page))
    }

    /// One checked page of a list.
    #[derive(Debug, Clone)]
    pub struct Page(CursorPage);

    impl Page {
        /// Rows in the page.
        #[must_use]
        pub const fn limit(&self) -> u64 {
            self.0.limit()
        }

        /// Rows before the page.
        #[must_use]
        pub const fn offset(&self) -> u64 {
            self.0.offset()
        }

        /// The `LIMIT` to fetch: one more row than the page, so [`Page::respond`] can tell whether more follow.
        #[must_use]
        pub const fn fetch(&self) -> u64 {
            self.0.limit().saturating_add(1)
        }

        /// The page out of rows already in memory: [`Page::fetch`] of them, from [`Page::offset`].
        #[must_use]
        pub fn slice<T>(&self, rows: impl IntoIterator<Item = T>) -> Vec<T> {
            rows.into_iter()
                .skip(usize::try_from(self.offset()).unwrap_or(usize::MAX))
                .take(usize::try_from(self.fetch()).unwrap_or(usize::MAX))
                .collect()
        }

        /// Drop the extra row [`Page::fetch`] asked for; the flag says whether more rows follow.
        #[must_use]
        pub fn split<T>(&self, rows: Vec<T>) -> (Vec<T>, bool) {
            self.0.split(rows)
        }

        /// `rows`, fetched with [`Page::fetch`], as a bare JSON array, with `Link: rel="next"` while more follow.
        #[must_use]
        pub fn respond<T: serde::Serialize>(&self, rows: Vec<T>) -> Response {
            let mut response = self.0.respond(rows);
            response.extensions_mut().insert(Paged);
            response
        }

        /// Mark a response this page's handler built itself: `Link: rel="next"` when `more`.
        pub fn stamp(&self, response: &mut Response, more: bool) {
            if more && let Some(link) = self.next_link() {
                response.headers_mut().insert(LINK, link);
            }
            response.extensions_mut().insert(Paged);
        }

        /// The `Link` header for the next page.
        #[must_use]
        pub fn next_link(&self) -> Option<HeaderValue> {
            self.0.next_link()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn key() -> CursorKey {
            CursorKey::new(&[9; 32]).unwrap()
        }

        fn page(uri: &str) -> axa::Result<Page> {
            page_with(&key(), &uri.parse().unwrap(), 100, MAX_LIMIT)
        }

        fn next(page: &Page) -> String {
            let link = page.next_link().unwrap();
            let link = link.to_str().unwrap();
            link.trim_start_matches('<')
                .split_once('>')
                .unwrap()
                .0
                .to_string()
        }

        #[test]
        fn the_first_page_is_the_default_limit_from_the_start() {
            let p = page("/api/plans").unwrap();
            assert_eq!((p.limit(), p.offset(), p.fetch()), (100, 0, 101));
        }

        #[test]
        fn a_cursor_carries_on_where_the_last_page_stopped() {
            let first = page("/api/plans?parent=x&limit=10").unwrap();
            let second = page(&next(&first)).unwrap();
            assert_eq!((second.limit(), second.offset()), (10, 10));
            assert!(next(&second).starts_with("/api/plans?parent=x&limit=10&cursor="));
        }

        #[test]
        fn a_cursor_moved_to_another_filter_or_list_is_refused() {
            let first = page("/api/plans?parent=x&limit=10").unwrap();
            let link = next(&first);
            let cursor = link.split_once("cursor=").unwrap().1;
            for uri in [
                format!("/api/plans?parent=y&cursor={cursor}"),
                format!("/api/reviews?parent=x&cursor={cursor}"),
                "/api/plans?parent=x&cursor=made-up".to_string(),
            ] {
                let error = page(&uri).unwrap_err();
                assert_eq!(
                    axum::response::IntoResponse::into_response(error).status(),
                    422,
                    "{uri}"
                );
            }
        }

        #[test]
        fn a_limit_out_of_range_is_refused_not_clamped() {
            for uri in [
                "/api/plans?limit=0",
                "/api/plans?limit=501",
                "/api/plans?limit=ten",
            ] {
                assert!(page(uri).is_err(), "{uri}");
            }
            assert_eq!(page("/api/plans?limit=500").unwrap().limit(), 500);
        }

        #[test]
        fn a_cursor_past_the_ceiling_is_refused() {
            let mut p = page("/api/plans?limit=500").unwrap();
            while p.offset() < MAX_OFFSET {
                p = page(&next(&p)).unwrap();
            }
            assert_eq!(p.offset(), MAX_OFFSET);
            let error = page(&next(&p)).unwrap_err();
            assert_eq!(
                axum::response::IntoResponse::into_response(error).status(),
                400
            );
        }

        #[test]
        fn a_slice_takes_one_more_row_and_respond_drops_it_with_a_link() {
            let p = page("/api/plans?limit=2").unwrap();
            assert_eq!(p.slice(0..10), vec![0, 1, 2]);
            let response = p.respond(p.slice(0..10));
            assert!(response.headers().contains_key(LINK));
            assert!(response.extensions().get::<Paged>().is_some());
            let last = p.respond(p.slice(0..2));
            assert!(
                !last.headers().contains_key(LINK),
                "the last page has no next"
            );
        }
    }
}
pub mod approvals;
pub mod attachments;
pub mod availability;
pub mod blueprint_sync;
pub mod buffer_management;
pub mod buffers;
pub mod bulk;
pub mod calendars;
pub mod ccpm_estimates;
pub mod components;
pub mod controlled_documents;
pub mod cost_rates;
pub mod cost_risk;
pub mod cpm;
pub mod critical_chain;
pub mod custom_fields;
pub mod cutover;
pub mod deescalation;
pub mod deployment_moves;
pub mod drum;
pub mod earned_value;
pub mod email_deliveries;
pub mod escalations;
pub mod feeds;
pub mod frameworks;
pub mod goals;
pub mod guests;
pub mod hammock;
pub mod handover;
pub mod includes;
pub mod key_persons;
pub mod kpis;
pub mod ledger;
pub mod leveling;
pub mod live;
pub mod meeting_actions;
pub mod meetings;
pub mod milestone_signoffs;
pub mod milestones;
pub mod mspdi_batch;
pub mod mspdi_export;
pub mod mspdi_import;
pub mod organizations;
pub mod paging;
pub mod pdf;
pub mod people_impact;
pub mod plan_access;
pub mod plan_links;
pub mod plan_roles;
pub mod plan_templates;
pub mod planner;
pub mod portfolio_resources;
pub mod processes;
pub mod programme_profile;
pub mod raci;
pub mod register;
pub mod report_schedules;
pub mod role_tasks;
pub mod schedule_baselines;
pub mod schedule_health;
pub mod schedule_risk;
pub mod sinks;
pub mod start_control;
pub mod subjects;
pub mod task_bulk;
pub mod task_get;
pub mod task_relations;
pub mod task_tree;
pub mod thresholds;
pub mod timesheets;
pub mod tolerances;
pub mod tracker_import;
pub mod watchers;
pub mod webhook_deliveries;

#[cfg(test)]
mod helper_census {
    /// CQ-7: the shared helpers are defined once, here. A controller that needs one imports it
    /// (`use super::find_live_plan as find_plan;`) instead of copying it, so a fix lands everywhere.
    #[test]
    fn controllers_do_not_copy_the_shared_helpers() {
        // A `db_err` that maps a named database constraint to a 409 before falling back is not a
        // copy; nor is governance's validation 422, which answers a different code.
        let allowed: &[(&str, &str)] = &[
            ("capacity_heatmap.rs", "fn db_err("),
            ("deescalation.rs", "fn db_err("),
            ("ledger.rs", "fn db_err("),
            ("organizations.rs", "fn db_err("),
            ("programme_gates.rs", "fn db_err("),
            ("governance.rs", "fn unprocessable("),
        ];
        let needles = [
            "fn db_err(",
            "fn unprocessable(",
            "fn find_plan(",
            "fn find_task(",
            "fn find_plan_pid(",
        ];
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controllers");
        let mut offenders = Vec::new();
        for entry in std::fs::read_dir(dir).expect("read controllers") {
            let path = entry.expect("entry").path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "mod.rs" || path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read file");
            for needle in needles {
                if text.contains(needle) && !allowed.contains(&(name, needle)) {
                    offenders.push(format!("{name}: {needle}"));
                }
            }
            // A path pid parsed by hand: `parse_pid` says what a malformed id means, once.
            if text
                .lines()
                .any(|l| l.contains("Uuid::parse_str(") && l.contains("Error::NotFound"))
            {
                offenders.push(format!("{name}: Uuid::parse_str(..) to Error::NotFound"));
            }
        }
        offenders.sort();
        assert!(offenders.is_empty(), "copied helpers: {offenders:?}");
    }

    /// Since axa 0.11 the stack binds every request body to the service-wide limit, so a route raised only with
    /// Axum's `DefaultBodyLimit` still refuses a body over 2 MiB with `413` (a 3 MiB attachment did, in
    /// `requests::attachments`). A route that takes a larger body says `axa::middleware::body_limit`, which raises both.
    #[test]
    fn a_larger_body_is_allowed_with_axas_body_limit_not_axums() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controllers");
        let mut offenders = Vec::new();
        for entry in std::fs::read_dir(dir).expect("read controllers") {
            let path = entry.expect("entry").path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "mod.rs" || path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read file");
            if text.contains("DefaultBodyLimit::") {
                offenders.push(name.to_string());
            }
        }
        offenders.sort();
        assert!(
            offenders.is_empty(),
            "use axa::middleware::body_limit(route, bytes): {offenders:?}"
        );
    }
}

#[cfg(test)]
mod database_unavailable {
    use axum::{http::StatusCode, response::IntoResponse};

    /// Since axa 0.11 a database that cannot serve for now is `503` `unavailable` with `Retry-After: 5`, not `500`:
    /// through [`super::db_err`] (`SeaORM`) and through `?` on a plain `sqlx` error alike.
    #[test]
    fn a_database_that_cannot_serve_answers_503_with_retry_after() {
        let seaorm = super::db_err(sea_orm::DbErr::ConnectionAcquire(
            sea_orm::error::ConnAcquireErr::Timeout,
        ));
        let sqlx: axa::Error = sqlx::Error::PoolTimedOut.into();
        for response in [seaorm.into_response(), sqlx.into_response()] {
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(response.headers()["retry-after"], "5");
        }
        // A database that refused this request is still the service's own `500`.
        let refused = super::db_err(sea_orm::DbErr::Custom("boom".into())).into_response();
        assert_eq!(refused.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
