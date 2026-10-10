//! `SeaORM` Entity — `milestone_slips`. One reschedule of a milestone: the old and new due dates, why, and who. Append-only: the history is how many times and how far a milestone has moved.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "milestone_slips")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub milestone_pid: Uuid,
    pub from_due: Date,
    pub to_due: Date,
    pub reason: Option<String>,
    pub actor: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
