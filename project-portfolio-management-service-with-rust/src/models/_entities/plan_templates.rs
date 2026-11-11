//! Row — `plan_templates`. A reusable plan tree (LT-2). The snapshot is self-contained JSON: once saved, a template
//! does not depend on the plan it was captured from. It carries **no person reference and no identity** (leads,
//! identifiers, codes), and every date is an **offset** from a base date.

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
    pub description: Option<String>,
    pub snapshot: Json,
    /// Bumped each time the template is revised (NC-12).
    pub version: i32,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    /// The plan this was saved from, so it is hidden with that plan. `None` for a starter and for a template saved before OC-20.
    pub source_plan_pid: Option<Uuid>,
}
