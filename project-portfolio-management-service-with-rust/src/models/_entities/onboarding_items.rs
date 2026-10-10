//! `SeaORM` Entity — `onboarding_items`. A checklist item of a case.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "onboarding_items")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub case_pid: Uuid,
    pub position: i32,
    pub title: String,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    /// `done` or `skipped`; null while open.
    pub resolution: Option<String>,
    pub note: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub resolved_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
