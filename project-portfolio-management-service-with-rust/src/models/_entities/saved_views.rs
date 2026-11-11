//! Row — `saved_views`. A per-user saved filter/sort/
//! column preset for a front-end route (T-28l). Keyed by the token
//! `sub` and nothing else identity-shaped.

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
    pub sub: String,
    pub route: String,
    pub name: String,
    pub filter: Json,
    pub sort: Json,
    pub columns: Json,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
