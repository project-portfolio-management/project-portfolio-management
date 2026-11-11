//! Row — `organizations`: the delivery organization, a partner, a sponsor or a supplier (EX-3).

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub name: String,
    /// `delivery`, `partner`, `sponsor` or `supplier`.
    pub kind: String,
    pub active: bool,
}
