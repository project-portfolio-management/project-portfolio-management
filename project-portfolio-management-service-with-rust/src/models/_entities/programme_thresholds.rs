//! `SeaORM` Entity — `programme_thresholds`. A programme's pause, reset or stop threshold (EX-7).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "programme_thresholds")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// `confidence_red_packs`, `forecast_past` or `p80_cap`.
    pub kind: String,
    pub count_value: Option<i32>,
    pub date_value: Option<Date>,
    pub amount_minor: Option<i64>,
    pub currency: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
