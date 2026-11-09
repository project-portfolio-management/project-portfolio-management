//! Migration: the outbox becomes **a record** (`axa-outbox` 0.2). An event and everything that happens to it are only
//! ever inserted, into `axa_outbox_events`, `axa_outbox_payloads` and `axa_outbox_attempts`, whose triggers refuse
//! `UPDATE`, `DELETE` and `TRUNCATE`. [`SETUP_STATEMENTS`](axa::outbox::SETUP_STATEMENTS) make them, and
//! [`IMPORT_STATEMENTS`](axa::outbox::IMPORT_STATEMENTS) copy every `axa_outbox` row across in its order, with one
//! attempt for its state: `delivered` for `sent`, `failed` for `failed`, and `imported` for one still owed, which the
//! relay then sends. Both are safe to repeat.
//!
//! `axa-outbox` 0.1 named an index `axa_outbox_owed`, the name 0.2 gives a table. `CREATE TABLE IF NOT EXISTS` then
//! sees the name taken, makes nothing, and the next statement fails on a column the index does not have. So
//! [`FREE_THE_NAME`] first renames that index, if it is one, to `axa_outbox_v01_owed`; it still serves `axa_outbox`.
//!
//! Stop every relay before this runs. `axa_outbox` is left as it was, and is dropped in a later release, once no
//! binary that writes it is running.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

/// Rename 0.1's index `axa_outbox_owed`, which takes the name of 0.2's table, when it is an index.
const FREE_THE_NAME: &str = "DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'axa_outbox_owed' AND relkind = 'i') THEN
        ALTER INDEX axa_outbox_owed RENAME TO axa_outbox_v01_owed;
    END IF;
END $$";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Make the record's tables and copy `axa_outbox` into them.
    ///
    /// # Errors
    ///
    /// Propagates any SQL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(FREE_THE_NAME).await?;
        for statement in axa::outbox::SETUP_STATEMENTS
            .iter()
            .chain(axa::outbox::IMPORT_STATEMENTS.iter())
        {
            db.execute_unprepared(statement).await?;
        }
        Ok(())
    }

    /// Refused: the record's triggers refuse every change, by design, and `axa_outbox` still holds what was copied.
    /// Roll the binary back to one on `axa` 0.7 and it reads `axa_outbox` again; anything written since is in the
    /// record only.
    ///
    /// # Errors
    ///
    /// Always.
    async fn down(&self, _m: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "the event record cannot be rolled back: its tables refuse every change. axa_outbox is unchanged".into(),
        ))
    }
}
