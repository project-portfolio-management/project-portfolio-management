//! Row — `plan_access_settings`. Who may see a private plan (RM-8). A row exists only where someone made a plan
//! private; no row means the plan is as visible as it always was. Members are token `sub`s (user pids). Its own table
//! so the plan row and its integrity hash stay untouched.

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
    pub plan_pid: Uuid,
    pub private: bool,
    pub members: Json,
}
