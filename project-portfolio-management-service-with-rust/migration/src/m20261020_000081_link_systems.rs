//! Migration: **external systems on plan links**. A plan link may name the work-tracking system it points into
//! (`system`: Jira, Trello, Asana, Linear, monday.com and others) and that system's own identifier for the thing
//! (`external_id`, for example a Jira key or a board id). A new link kind, `tracker`, is a project or board in such
//! a system; a single issue there is a `ticket`. The service still never fetches the link.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns and widen the checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS system VARCHAR NULL;
                 ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS external_id VARCHAR NULL;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_kind;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_kind
                     CHECK (kind IN ('docs','repo','ticket','design','chat','tracker','other'));
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_system;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_system
                     CHECK (system IS NULL OR system IN
                         ('jira','trello','asana','linear','monday','clickup','azure_devops','github','other'));
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_tracker_has_system;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_tracker_has_system
                     CHECK (kind <> 'tracker' OR system IS NOT NULL);",
            )
            .await?;
        Ok(())
    }

    /// Drop the columns and restore the narrower kind check (a `tracker` link cannot be kept).
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DELETE FROM plan_links WHERE kind = 'tracker';
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_tracker_has_system;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_system;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_kind;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_kind
                     CHECK (kind IN ('docs','repo','ticket','design','chat','other'));
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS external_id;
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS system;",
            )
            .await?;
        Ok(())
    }
}
