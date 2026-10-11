//! `SeaORM` Entity — `capacity_commitments`. Capacity promised to (supply) or work owed on (demand) a
//! programme, for a pool and a month, by a partner or by the organization itself.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "capacity_commitments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub pool_pid: Uuid,
    pub plan_pid: Uuid,
    /// The partner's name as a label (until partner organizations exist); none is the organization's own.
    pub organization: Option<String>,
    /// The organization record it names (EX-3); the label above is then its name.
    pub organization_pid: Option<Uuid>,
    /// `supply` or `demand`.
    pub kind: String,
    /// The first day of the month.
    pub month: Date,
    pub committed_milli_fte: i32,
    pub delivered_milli_fte: Option<i32>,
    pub note: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
