//! `SeaORM` Entity — `programme_status_decisions`. Append-only: a programme is active until a
//! recorded decision pauses or stops it.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "programme_status_decisions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// `active`, `paused` or `stopped`.
    pub status: String,
    pub reason: String,
    pub decided_by: Option<String>,
    pub decided_on: Date,
    /// The gate review this decision follows from, if any.
    pub gate_review_pid: Option<Uuid>,
    /// The delivery exception this decision follows from, if any.
    pub exception_pid: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
