//! `SeaORM` Entity — `approval_requests`. Ask a named person to approve something, and record their decision (PL-1). Generic over the same subject kinds as comments, plus budget lines and scenarios. A decision is final and carries a comment (required on reject).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "approval_requests")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub requested_by: String,
    pub approver_ref: String,
    pub message: Option<String>,
    pub status: String,
    pub decision_comment: Option<String>,
    pub decided_by: Option<String>,
    pub decided_at: Option<DateTimeWithTimeZone>,
    pub process_execution_pid: Option<Uuid>,
    pub process_step: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
