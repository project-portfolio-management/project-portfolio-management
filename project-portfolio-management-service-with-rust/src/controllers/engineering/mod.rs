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

use axum::http::HeaderMap;
use loco_rs::prelude::*;
use uuid::Uuid;

/// Load all live rows of one entity (soft-deleted excluded).
macro_rules! live {
    ($module:ident, $db:expr, $scope:expr) => {
        $scope.keep(
            $module::Entity::find()
                .filter($module::Column::DeletedAt.is_null())
                .all($db)
                .await
                .map_err(|e| Error::Model(ModelError::from(e)))?,
        )
    };
}

mod agile;
mod board;
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
        loco_rs::controller::ErrorDetail::new("unprocessable_entity", reason),
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

    #[test]
    fn no_paging_parameters_means_the_first_capped_slice() {
        assert_eq!(task_window(None, None), (UNPAGED_TASK_CAP, 0));
    }

    #[test]
    fn a_limit_or_an_offset_means_a_standard_page() {
        assert_eq!(task_window(Some(10), None), (10, 0));
        assert_eq!(
            task_window(None, Some(40)),
            (100, 40),
            "the default page is 100"
        );
        assert_eq!(
            task_window(Some(10_000), Some(5)),
            (500, 5),
            "a big limit is clamped to 500"
        );
        assert_eq!(
            task_window(Some(0), None),
            (100, 0),
            "zero falls back to the default"
        );
    }
}
