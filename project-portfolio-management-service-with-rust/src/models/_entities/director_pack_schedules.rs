//! `SeaORM` Entity — `director_pack_schedules`. A monthly schedule that drafts (never issues) a pack.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "director_pack_schedules")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub audience: String,
    pub day_of_month: i32,
    pub next_run_date: Date,
    pub enabled: bool,
    pub last_run_on: Option<Date>,
    pub last_result: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
