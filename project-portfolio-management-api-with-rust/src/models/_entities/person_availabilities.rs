//! `SeaORM` Entity — `person_availabilities`. A person's availability over a date range (PJ-3): the most of their time (0-100 percent) they can commit between two dates, for part-time spells, leave blocks and ramp-up. Where no range applies capacity is 100 percent, so nothing stored before this existed changes. Ranges for one person may not overlap (checked by the service).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "person_availabilities")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub person_ref: String,
    pub from_date: Date,
    pub to_date: Date,
    pub max_percent: i32,
    pub note: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
