//! Engineering-team endpoints — the spec-§13 operational **tasks**
//! sub-resource (Kanban board: CRUD + a PATCH status move with honest
//! flow stamps), **sprints** + the honest **burndown** (real `done_at`
//! completions only), the per-item **standup digest** (audit-derived
//! last-24h changes), and the estate views: **blocked-work aging**,
//! the **`MoSCoW`** scope cut (`moscow:<band>` tags), the
//! **delivery-links** panel (external tracker identifiers), and the
//! **milestone calendar** (demo / release / checkpoint kinds).
//!
//! Tasks and sprints are operational data: deliberately **not** part
//! of the matcher payload (the spec's partition rule) — nothing here
//! feeds matching.

use crate::prelude::*;
use axum::http::HeaderMap;
use uuid::Uuid;

/// Load all live rows of one table (soft-deleted excluded), unordered, that the scope lets the caller see. The table
/// is the entity module's name, a literal in the code.
macro_rules! live {
    ($module:ident, $ctx:expr, $scope:expr) => {
        $scope.keep(
            sqlx::query_as::<_, $module::Model>(concat!(
                "SELECT * FROM ",
                stringify!($module),
                " WHERE deleted_at IS NULL"
            ))
            .fetch_all(crate::db::pool($ctx))
            .await?,
        )
    };
}

mod agile;
mod board;
pub(crate) use board::enforce_move_roles;
mod devops;
mod estate;
pub(crate) use agile::{NotePayload, SprintPayload};
use agile::{
    burndown, convert_note, create_note, create_sprint, list_notes, list_sprints, standup, velocity,
};
pub(crate) use board::{MovePayload, TaskPayload, task_view};
pub use board::{UNPAGED_TASK_CAP, task_window};
use board::{create_task, delete_task, list_tasks, move_task, update_task};
pub(crate) use devops::DevopsEventPayload;
use devops::{devops_metrics, devops_releases, ingest_devops_event};
use estate::{blocked, delivery_links, milestone_calendar, moscow};

/// `422` with a reason.
fn refuse(reason: &str) -> Error {
    Error::CustomError(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        axa::error::ErrorDetail::new("unprocessable_entity", reason),
    )
}

/// A `worker:`/`person:` URN check (mirrors governance's rule).
fn valid_person_ref(value: &str) -> bool {
    value.split_once(':').is_some_and(|(scheme, id)| {
        matches!(scheme, "worker" | "person") && Uuid::parse_str(id).is_ok()
    })
}

/// Find one live task of one plan, or 404 (the shared definition, CQ-7).
pub(crate) use super::find_live_task as find_task;

/// ETag-conditional JSON response; the tag excludes `as_of`.
fn conditional(headers: &HeaderMap, body: &serde_json::Value) -> Result<Response> {
    let mut fingerprint = body.clone();
    if let Some(map) = fingerprint.as_object_mut() {
        map.remove("as_of");
    }
    super::conditional_json(headers, &super::etag_of(&fingerprint), body)
}

/// The engineering routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks", post(create_task))
        .add("/plans/{pid}/tasks", get(list_tasks))
        .add("/plans/{pid}/tasks/{t_pid}", put(update_task))
        .add("/plans/{pid}/tasks/{t_pid}", patch(move_task))
        .add("/plans/{pid}/tasks/{t_pid}", delete(delete_task))
        .add("/plans/{pid}/sprints", post(create_sprint))
        .add("/plans/{pid}/sprints", get(list_sprints))
        .add("/plans/{pid}/burndown", get(burndown))
        .add("/plans/{pid}/velocity", get(velocity))
        .add("/plans/{pid}/sprints/{s_pid}/notes", post(create_note))
        .add("/plans/{pid}/sprints/{s_pid}/notes", get(list_notes))
        .add(
            "/plans/{pid}/sprints/{s_pid}/notes/{n_pid}/convert",
            post(convert_note),
        )
        .add("/plans/{pid}/standup", get(standup))
        .add("/devops/events", post(ingest_devops_event))
        .add("/devops/metrics", get(devops_metrics))
        .add("/devops/releases", get(devops_releases))
        .add("/engineering/blocked", get(blocked))
        .add("/engineering/moscow", get(moscow))
        .add("/engineering/delivery-links", get(delivery_links))
        .add("/engineering/milestone-calendar", get(milestone_calendar))
}

#[cfg(test)]
mod task_window_tests {
    use super::{UNPAGED_TASK_CAP, task_window};
    use crate::controllers::pagination::Cursor;
    use crate::cursor::Cursors;

    fn window(uri: &str) -> Result<(u64, u64), u16> {
        let cursor = Cursor::new(std::sync::Arc::new(Cursors::random()), uri.parse().unwrap());
        task_window(&cursor)
            .map(|page| (page.limit(), page.offset()))
            .map_err(|e| {
                axum::response::IntoResponse::into_response(e)
                    .status()
                    .as_u16()
            })
    }

    #[test]
    fn no_paging_parameters_means_the_first_capped_slice() {
        assert_eq!(window("/api/plans/p/tasks"), Ok((UNPAGED_TASK_CAP, 0)));
    }

    #[test]
    fn a_limit_asks_for_fewer_and_never_more() {
        assert_eq!(window("/api/plans/p/tasks?limit=10"), Ok((10, 0)));
        assert_eq!(window("/api/plans/p/tasks?limit=2000"), Ok((2000, 0)));
        assert_eq!(
            window("/api/plans/p/tasks?limit=2001"),
            Err(422),
            "refused, not clamped"
        );
        assert_eq!(window("/api/plans/p/tasks?limit=0"), Err(422));
    }

    #[test]
    fn an_offset_is_not_a_cursor() {
        assert_eq!(window("/api/plans/p/tasks?cursor=40"), Err(422));
    }
}
