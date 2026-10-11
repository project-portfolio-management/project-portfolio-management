//! `SeaORM` Entity — `programme_profiles`: a programme's delivery model and delivery owner (EX-2), and its
//! reporting method (EX-14).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "programme_profiles")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    #[sea_orm(unique)]
    pub plan_pid: Uuid,
    /// `central_product`, `central_platform_local_deployment` or `local_programme_central_standards`.
    pub delivery_model: Option<String>,
    /// The organization that owns the delivery (EX-3).
    pub delivery_organization_pid: Option<Uuid>,
    /// `evm` (the default) or `product` (EX-14).
    pub method: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
