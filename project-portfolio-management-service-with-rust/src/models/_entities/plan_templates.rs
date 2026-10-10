//! `SeaORM` Entity — `plan_templates`. A reusable plan tree (LT-2). The snapshot is self-contained JSON: once saved, a template does not depend on the plan it was captured from. It carries **no person reference and no identity** (leads, identifiers, codes), and every date is an **offset** from a base date.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_templates")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub snapshot: Json,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
