//! `SeaORM` Entity — `change_requests`. A controlled proposal to change an approved baseline.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "change_requests")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub reason: String,
    /// `scope`, `schedule`, `cost`, `quality` or `other`.
    pub kind: String,
    /// Business days; negative pulls the finish in.
    pub impact_schedule_days: Option<i32>,
    pub impact_cost_minor: Option<i64>,
    pub cost_currency: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub impact_summary: Option<String>,
    /// What-if edits (the sandbox's edit list) whose forecast is shown as the impact.
    pub edits: Option<Json>,
    pub status: String,
    pub requested_by: String,
    pub requested_on: Date,
    pub approver_ref: Option<String>,
    pub decided_by: Option<String>,
    pub decided_on: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub decision_reason: Option<String>,
    pub implemented_on: Option<Date>,
    pub baseline_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
