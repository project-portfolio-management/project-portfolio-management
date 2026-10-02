//! `SeaORM` Entity — `attachments`. A file attached to a plan, task, risk, proposal, idea, canvas item, budget line or scenario (LT-5). The bytes are an artifact; this row is the link, so removing an attachment (soft-delete) never loses the audit trail of what was attached.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attachments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub artifact_pid: Uuid,
    pub uploaded_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
