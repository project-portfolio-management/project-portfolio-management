//! Row — `insight_snapshots`. One row per point-in-time
//! estate capture (kind + JSONB body) behind the board/CRO trends.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub taken_at: DateTimeWithTimeZone,
    pub kind: String,
    pub body: Json,
}
