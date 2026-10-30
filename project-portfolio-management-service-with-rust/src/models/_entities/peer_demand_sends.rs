//! `SeaORM` Entity — `peer_demand_sends`. The latest outcome of sending a programme's demand on a linked pool for a
//! month to the workforce service (FB-3). No token and no person is stored.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "peer_demand_sends")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
