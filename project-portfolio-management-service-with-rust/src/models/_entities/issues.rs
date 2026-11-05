//! `SeaORM` Entity — `issues`. Something that has already happened and needs managing, under a plan.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "issues")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    /// A key in `issue_types`.
    pub kind: String,
    /// `low`, `med`, `high` or `critical`.
    pub severity: String,
    pub priority: Option<String>,
    /// A state of the issue workflow in force when it was set.
    pub status: String,
    /// What the status means: `todo`, `active`, `waiting` or `done`.
    pub status_category: String,
    pub owner_ref: Option<String>,
    pub raised_by: Option<String>,
    pub raised_on: Date,
    pub due_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub resolution: Option<String>,
    pub resolved_on: Option<Date>,
    /// When it was reported. None on an issue from before times were kept.
    pub reported_at: Option<DateTimeWithTimeZone>,
    /// The first move out of the `todo` category.
    pub acknowledged_at: Option<DateTimeWithTimeZone>,
    /// The move into the `done` category; cleared on reopening.
    pub resolved_at: Option<DateTimeWithTimeZone>,
    /// The move into a terminal state; cleared on reopening.
    pub closed_at: Option<DateTimeWithTimeZone>,
    /// When it really began, as a person states it, often only known later.
    pub occurred_at: Option<DateTimeWithTimeZone>,
    /// When it was noticed, as a person states it.
    pub detected_at: Option<DateTimeWithTimeZone>,
    pub source_risk_pid: Option<Uuid>,
    pub source_task_pid: Option<Uuid>,
    pub change_request_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
