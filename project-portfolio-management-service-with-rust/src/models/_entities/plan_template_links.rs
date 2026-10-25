//! `SeaORM` Entity — `plan_template_links`. Where a plan came from (NC-12): the template, the plan's node in the
//! template tree, the version it was last brought up to, the baseline tasks, and the task-to-task map.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_template_links")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub plan_pid: Uuid,
    pub template_pid: Uuid,
    pub node_key: i32,
    pub template_version: i32,
    pub baseline: Json,
    pub task_map: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
