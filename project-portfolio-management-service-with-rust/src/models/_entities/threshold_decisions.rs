//! `SeaORM` Entity — `threshold_decisions`. Append-only: what was decided when a threshold was crossed (EX-7).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "threshold_decisions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub exception_pid: Uuid,
    pub plan_pid: Uuid,
    pub threshold_kind: String,
    /// `pause`, `reset`, `stop` or `override`.
    pub action: String,
    /// An override ends on this date.
    pub expiry: Option<Date>,
    pub reason: String,
    pub decided_by: Option<String>,
    pub decided_on: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
