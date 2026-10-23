//! `SeaORM` Entity — `plan_links`. Typed external links on a plan (LT-8): where the plan's real documents live. The service stores a link, never the document, and never fetches it. Only `http` and `https` URLs are accepted, so a link can never be a script or file reference.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_links")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub kind: String,
    pub url: String,
    pub label: String,
    /// The work-tracking system the link points into (`jira`, `trello`, ...), when it points into one.
    pub system: Option<String>,
    /// That system's own identifier for the thing (a Jira key, a board id).
    pub external_id: Option<String>,
    /// The task of the plan this link belongs to, when it is a task's link rather than the plan's.
    pub task_pid: Option<Uuid>,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
