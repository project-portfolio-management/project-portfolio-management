//! Migration: **`meetings`** and **`meeting_items`** (NC-14). A meeting has an agenda before and minutes and outcomes
//! after; its items may point at a record of the same plan.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS meetings (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     title VARCHAR NOT NULL,
                     scheduled_on DATE NOT NULL,
                     status VARCHAR NOT NULL DEFAULT 'planned',
                     minutes TEXT NULL,
                     held_on DATE NULL,
                     created_by VARCHAR NULL,
                     held_by VARCHAR NULL,
                     deleted_at TIMESTAMPTZ NULL,
                     CONSTRAINT meetings_status CHECK (status IN ('planned', 'held', 'cancelled'))
                 );
                 CREATE INDEX IF NOT EXISTS meetings_plan_idx ON meetings (plan_pid, scheduled_on DESC) WHERE deleted_at IS NULL;
                 CREATE TABLE IF NOT EXISTS meeting_items (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     meeting_pid UUID NOT NULL,
                     part VARCHAR NOT NULL,
                     outcome_kind VARCHAR NULL,
                     text VARCHAR NOT NULL,
                     link_kind VARCHAR NULL,
                     link_pid UUID NULL,
                     CONSTRAINT meeting_items_part CHECK (part IN ('agenda', 'outcome')),
                     CONSTRAINT meeting_items_outcome CHECK (
                         (part = 'agenda' AND outcome_kind IS NULL)
                         OR (part = 'outcome' AND outcome_kind IN ('decision', 'action', 'note'))
                     ),
                     CONSTRAINT meeting_items_link CHECK ((link_kind IS NULL) = (link_pid IS NULL))
                 );
                 CREATE INDEX IF NOT EXISTS meeting_items_meeting_idx ON meeting_items (meeting_pid, id);",
            )
            .await?;
        Ok(())
    }

    /// Drop the tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS meeting_items; DROP TABLE IF EXISTS meetings")
            .await?;
        Ok(())
    }
}
