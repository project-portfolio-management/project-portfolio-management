//! Row — `peer_demand_sends`. The latest outcome of sending a programme's demand on a linked pool for a
//! month to the workforce service (FB-3). No token and no person is stored.

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
    pub pool_pid: Uuid,
    pub plan_pid: Uuid,
    /// The first day of the month.
    pub month: Date,
    /// The WPM pool it was sent to.
    pub peer_pool_ref: String,
    /// Hundredths of an FTE sent; none when nothing was sent.
    pub fte_centi: Option<i64>,
    pub peer_event_id: String,
    /// The write that made this version.
    pub version_at: DateTimeWithTimeZone,
    pub outcome: String,
    pub detail: Option<String>,
}
