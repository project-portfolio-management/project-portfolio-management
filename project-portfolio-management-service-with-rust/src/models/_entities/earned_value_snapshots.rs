//! `SeaORM` Entity — `earned_value_snapshots`. A day's earned-value figures for one plan in one currency (SC-5): what was planned, earned and spent as of that date. Written once a day by the scheduler (or on request) so the S-curve has a real history to draw: earned value and actual cost are not dated at source, so the past cannot be reconstructed, only recorded going forward. One row per plan, currency and day; a second snapshot the same day replaces the first.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "earned_value_snapshots")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub currency: String,
    pub as_of: Date,
    pub baseline_version: i32,
    pub bac_minor: i64,
    pub pv_minor: i64,
    pub ev_minor: i64,
    pub ac_minor: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
