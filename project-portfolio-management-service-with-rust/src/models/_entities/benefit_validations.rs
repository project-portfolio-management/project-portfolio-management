//! `SeaORM` Entity — `benefit_validations`. Append-only: an independent person's statement of the
//! amount of a benefit validated to date, with the evidence.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "benefit_validations")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub benefit_pid: Uuid,
    pub plan_pid: Uuid,
    pub amount_minor: i64,
    pub currency: String,
    pub validator: Option<String>,
    pub evidence_attachment_pid: Option<Uuid>,
    pub validated_on: Date,
    pub note: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
