//! `SeaORM` Entity — `meeting_items`. One agenda item or outcome of a meeting (NC-14).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "meeting_items")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub meeting_pid: Uuid,
    pub part: String,
    pub outcome_kind: Option<String>,
    pub text: String,
    pub link_kind: Option<String>,
    pub link_pid: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
