//! `SeaORM` Entity — `time_entry_costs`. The cost of a recorded time entry as worked out when it was recorded: the rate used is stored with it, so a later rate change never rewrites an actual.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "time_entry_costs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub time_entry_pid: Uuid,
    pub plan_pid: Uuid,
    pub person_ref: String,
    pub table_pid: Uuid,
    pub spent_on: Date,
    pub minutes: i64,
    pub per_hour_minor: i64,
    pub currency: String,
    pub cost_minor: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
