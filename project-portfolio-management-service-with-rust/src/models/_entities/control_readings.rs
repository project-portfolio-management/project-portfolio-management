//! Row — `control_readings`. **Append-only**: correcting a
//! reading means recording another one, because a control history that
//! can be rewritten measures whatever the editor wanted. See entity
//! spec §5.9.8 / FR-38.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub control_pid: Uuid,
    pub observed_at: DateTimeWithTimeZone,
    /// `None` is `unmeasured` — a third verdict, never a pass.
    pub value: Option<i64>,
    pub verdict: String,
    pub gap: Option<i64>,
    pub method: String,
    /// An explicit acceptance of a failing reading. A failure with
    /// neither this nor an action is reported as **unanswered**.
    pub accepted_at: Option<DateTimeWithTimeZone>,
    pub accepted_reason: Option<String>,
}
