//! `SeaORM` Entity — `attachment_texts`. The text extracted from an attachment so it can be found by search (PL-8). Extraction is best-effort and never fails an upload: a file with no extractor is recorded `unindexed` with the reason, never silently skipped. The text inherits its attachment's access: a search result is shown only to someone who may read the attachment's subject.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attachment_texts")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub attachment_pid: Uuid,
    pub status: String,
    pub reason: Option<String>,
    pub body: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
