//! `SeaORM` Entity — `timesheet_weeks`. The approval state of one person's week of time entries (LT-11). A row exists only once a week has been submitted; no row means the week is open. Approved weeks are locked against new entries. `week_start` is the ISO Monday, a zone-free calendar date.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "timesheet_weeks")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub person_ref: String,
    pub week_start: Date,
    pub status: String,
    pub submitted_at: Option<DateTimeWithTimeZone>,
    pub decided_by: Option<String>,
    pub decided_at: Option<DateTimeWithTimeZone>,
    pub decision_comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
