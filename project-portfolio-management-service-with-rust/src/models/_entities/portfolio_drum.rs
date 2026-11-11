//! Row — `portfolio_drum`. Append-only: the skill pool named as the portfolio's drum (CC-5).

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
    /// `None` when the naming was cleared.
    pub skill_pool_pid: Option<Uuid>,
    pub declared_on: Date,
}
