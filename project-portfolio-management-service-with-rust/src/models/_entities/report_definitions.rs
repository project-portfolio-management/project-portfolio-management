//! Row — `report_definitions`. Saved reports (PPM-9):
//! a named filter + field projection over one collection.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub name: String,
    pub collection: String,
    pub filters: Json,
    pub fields: Json,
    pub group_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
