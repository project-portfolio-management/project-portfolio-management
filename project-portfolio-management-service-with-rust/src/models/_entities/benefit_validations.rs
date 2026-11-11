//! Row — `benefit_validations`. Append-only: an independent person's statement of the
//! amount of a benefit validated to date, with the evidence.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
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
