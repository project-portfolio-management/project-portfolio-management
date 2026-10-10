//! `SeaORM` Entity — `notification_sinks`. A person's own delivery destination for their notifications (LT-6, RM-11): a signed webhook, a chat incoming-webhook (Slack, Mattermost, Discord, Telegram) or an email address. The destination is a secret of its owner and is never returned in full by any read.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "notification_sinks")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub sub: String,
    pub kind: String,
    pub destination: String,
    pub settings: Json,
    pub enabled: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
