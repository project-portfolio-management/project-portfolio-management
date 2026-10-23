//! `SeaORM` Entity — `report_schedules`. A saved report run on a cadence (T-28e): each due run is stored as an expiring artifact and audited like an export, even at zero rows. Dates are UTC calendar dates (a portfolio view, not a personal one).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "report_schedules")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub report_pid: Uuid,
    pub owner: String,
    pub cadence: String,
    pub format: String,
    pub anchor_date: Date,
    pub next_run_date: Date,
    pub enabled: bool,
    pub last_run_at: Option<DateTimeWithTimeZone>,
    pub last_status: Option<String>,
    pub last_rows: Option<i32>,
    pub last_artifact_pid: Option<Uuid>,
    pub last_error: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
