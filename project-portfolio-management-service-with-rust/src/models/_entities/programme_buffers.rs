//! `SeaORM` Entity — `programme_buffers`. Append-only: a programme's project buffer, in business days.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "programme_buffers")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub buffer_days: i32,
    /// `declared` (a person chose the number) or `rule` (a stated rule gave it).
    pub source: String,
    pub rule: Option<String>,
    pub note: Option<String>,
    pub declared_by: Option<String>,
    pub declared_on: Date,
    /// The schedule baseline version in force when it was set.
    pub baseline_version: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
