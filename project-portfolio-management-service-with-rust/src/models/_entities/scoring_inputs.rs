//! `SeaORM` Entity — `scoring_inputs`. The estimated inputs of the WSJF and RICE models for one plan (NC-6).
//! Every input is optional; an input nobody has estimated is absent, not zero.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scoring_inputs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub plan_pid: Uuid,
    pub business_value: Option<i16>,
    pub time_criticality: Option<i16>,
    pub risk_reduction: Option<i16>,
    pub job_size: Option<i16>,
    pub reach: Option<i32>,
    pub impact: Option<f64>,
    pub confidence_percent: Option<i16>,
    pub effort: Option<f64>,
    pub set_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
