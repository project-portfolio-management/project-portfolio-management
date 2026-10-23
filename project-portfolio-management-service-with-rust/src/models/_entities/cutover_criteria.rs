//! `SeaORM` Entity — `cutover_criteria`. A stop criterion of a cutover decision, published before the decision (EX-6).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cutover_criteria")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub decision_pid: Uuid,
    pub position: i32,
    pub criterion: String,
    pub met: Option<bool>,
    pub evidence: Option<String>,
    pub marked_by: Option<String>,
    pub marked_on: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
