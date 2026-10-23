//! `SeaORM` Entity — `review_queue_items`. A row held for a human decision (T-8): a keyless imported plan that the matcher scores as a probable duplicate of an existing one. Nothing is created or merged automatically (spec §5.1). `provenance` is `import` here.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review_queue_items")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub status: String,
    pub provenance: String,
    pub candidate: Json,
    pub matches: Json,
    pub job_pid: Option<Uuid>,
    pub source_line: i32,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub resolution_pid: Option<Uuid>,
    pub resolution_comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
