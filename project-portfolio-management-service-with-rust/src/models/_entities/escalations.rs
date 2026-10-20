//! `SeaORM` Entity — `escalations`. A project referring something upward at a level of its governance escalation
//! path. Its history is in `escalation_events`.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "escalations")]
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
    pub reason_kind: String,
    pub tolerance_dimension: Option<String>,
    pub level: i32,
    pub escalated_to: Option<String>,
    pub owner_ref: Option<String>,
    pub status: String,
    pub raised_by: Option<String>,
    pub raised_on: Date,
    pub respond_by: Option<Date>,
    pub outcome: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub response: Option<String>,
    pub resolved_on: Option<Date>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
