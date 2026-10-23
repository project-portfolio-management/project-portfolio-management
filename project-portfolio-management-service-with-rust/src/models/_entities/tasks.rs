//! `SeaORM` Entity — `tasks`. The per-plan operational task
//! sub-resource (spec §13): Kanban status + honest flow stamps
//! (`status_changed_at`, first `done_at`).

// SeaORM-generated entity shape: documented by the migration.
#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub sprint_pid: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assignee_ref: Option<String>,
    pub points: Option<i32>,
    pub status_changed_at: DateTimeWithTimeZone,
    pub done_at: Option<DateTimeWithTimeZone>,
    /// Due **calendar date** — zone-free by the time-zone doctrine (`crate::tz`).
    pub due_date: Option<Date>,
    /// Planned start (PJ-1); a lower bound for the critical-path engine.
    pub start_date: Option<Date>,
    /// Business-day duration (PJ-1); `0` is a milestone.
    pub duration_days: Option<i32>,
    /// Optimistic business-day duration (SC-8); the likely one is `duration_days`.
    pub duration_optimistic_days: Option<i32>,
    /// Pessimistic business-day duration (SC-8).
    pub duration_pessimistic_days: Option<i32>,
    /// Work, in minutes (PJ-6): with `units_percent` and the duration, one of three related quantities.
    pub work_minutes: Option<i64>,
    /// How much of the assignee's day the task takes, percent (PJ-6).
    pub units_percent: Option<i32>,
    /// Which quantity stays fixed when another is edited: `fixed_units`, `fixed_duration`, `fixed_work`.
    pub scheduling_rule: Option<String>,
    /// How the work is spread over the business days (PJ-5): `flat`, `front_loaded`, `back_loaded`, `bell`, `turtle`, `custom`.
    pub contour: String,
    /// The weights of a `custom` contour (percent, summing to 100).
    pub contour_custom: Option<Json>,
    /// A date constraint kind (SC-2): `snet`, `snlt`, `fnet`, `fnlt`, `mso`, `mfo`.
    pub constraint_kind: Option<String>,
    /// The constraint's date; set exactly when the kind is.
    pub constraint_date: Option<Date>,
    /// A soft deadline: bounds late dates only (negative float), never early ones.
    pub deadline: Option<Date>,
    /// Where the task came from (`mspdi:<UID>`); unique per plan (PJ-10).
    pub external_ref: Option<String>,
    /// A per-task earned-value method override (SC-4): `zero_hundred`, `fifty_fifty` or `manual`.
    pub percent_method: Option<String>,
    /// The person-entered percent (0-100), set exactly when the method is `manual`.
    pub percent_manual: Option<i32>,
    /// Why a manual percent is what it is (mandatory with one).
    pub percent_note: Option<String>,
    /// The component (grouping) of this plan the task belongs to, if any.
    pub component_pid: Option<Uuid>,
    /// The parent task (a subtask's parent), if any. Roll-ups derive on read.
    pub parent_task_pid: Option<Uuid>,
    /// Position among the tasks with the same parent (`PUT …/task-order`); `None` sorts after the positioned ones.
    pub sibling_position: Option<i32>,
    /// The declared Flow Framework work-item type (`feature` /
    /// `defect` / `risk` / `debt`). `None` means **nobody declared
    /// one**, reported as `unclassified` and counted separately — never
    /// folded into `feature`.
    pub flow_type: Option<String>,
    pub is_hammock: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
