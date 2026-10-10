//! `SeaORM` Entity — `delivery_exceptions`. Opened when delivery confidence reaches amber-red or red.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "delivery_exceptions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// `open` or `closed`.
    pub status: String,
    pub opened_at: DateTimeWithTimeZone,
    pub opened_by: Option<String>,
    pub opened_value: String,
    pub worst_value: String,
    pub worsened_count: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub cause: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub decision_needed: Option<String>,
    /// A reference to the named owner (used to notify; never shown in a pack).
    pub owner_ref: Option<String>,
    /// The owner's role, as shown in a pack.
    pub owner_role: Option<String>,
    pub decide_by: Option<Date>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub breach_notified_at: Option<DateTimeWithTimeZone>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub decision: Option<String>,
    pub decided_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
