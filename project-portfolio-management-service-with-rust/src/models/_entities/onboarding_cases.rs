//! `SeaORM` Entity — `onboarding_cases`. One onboarding or offboarding, of a person on a plan or of a plan itself.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "onboarding_cases")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub kind: String,
    pub plan_pid: Uuid,
    pub subject_ref: Option<String>,
    pub title: String,
    pub effective_date: Date,
    /// `open`, `completed` or `cancelled`.
    pub status: String,
    pub created_by: Option<String>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub closed_reason: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
