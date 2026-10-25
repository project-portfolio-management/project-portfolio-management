//! `SeaORM` Entity — `meetings`. An agenda, minutes and outcomes for one plan (NC-14).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "meetings")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub title: String,
    pub scheduled_on: Date,
    pub status: String,
    #[sea_orm(column_type = "Text")]
    pub minutes: Option<String>,
    pub held_on: Option<Date>,
    pub created_by: Option<String>,
    pub held_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
