//! `SeaORM` Entity — `work_calendars`. A working calendar (SC-2): the weekdays that work and a list of holidays, for one plan or one person. A task is scheduled on its plan's calendar; an assigned person's own calendar (plus their recorded absences) narrows it. Also adds the per-task date constraint and deadline columns the critical-path engine reads.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "work_calendars")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub scope: String,
    pub subject: String,
    pub weekdays: String,
    pub holidays: Json,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
