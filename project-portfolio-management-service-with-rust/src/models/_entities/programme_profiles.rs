//! Row — `programme_profiles`: a programme's delivery model and delivery owner (EX-2), and its
//! reporting method (EX-14).

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
    pub plan_pid: Uuid,
    /// `central_product`, `central_platform_local_deployment` or `local_programme_central_standards`.
    pub delivery_model: Option<String>,
    /// The organization that owns the delivery (EX-3).
    pub delivery_organization_pid: Option<Uuid>,
    /// `evm` (the default) or `product` (EX-14).
    pub method: String,
}
