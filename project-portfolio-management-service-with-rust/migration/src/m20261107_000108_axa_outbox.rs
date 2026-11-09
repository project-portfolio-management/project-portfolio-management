//! Migration: the durable event bus's outbox moves to **`axa_outbox`**, the table `axa-outbox` 0.1 relayed from, made
//! by that release's own setup statements, kept here as [`V01_SETUP`] so that this migration makes the same table
//! whichever `axa` the service builds with (0.2 makes other tables; migration 109 moves the rows into them). Every `event_outbox` row moves across in its order:
//! an unpublished one as `pending`, so the new relay sends it, and a published one as `sent`, so a webhook redelivery
//! can still find it until the sweep. The old table is left empty, and is dropped in a later release, once no
//! binary that writes it is running.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

/// `axa-outbox` 0.1.0's `SETUP_STATEMENTS`, as that release had them.
const V01_SETUP: [&str; 5] = [
    "CREATE TABLE IF NOT EXISTS axa_outbox (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id UUID NOT NULL UNIQUE,
    topic TEXT NOT NULL,
    kind TEXT NOT NULL,
    key TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    error TEXT,
    run_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
)",
    "CREATE INDEX IF NOT EXISTS axa_outbox_ready ON axa_outbox (id) WHERE status = 'pending'",
    "CREATE INDEX IF NOT EXISTS axa_outbox_owed
    ON axa_outbox (topic, key, id) WHERE status <> 'sent'",
    "CREATE INDEX IF NOT EXISTS axa_outbox_claimed
    ON axa_outbox (updated_at) WHERE status = 'claimed'",
    "CREATE INDEX IF NOT EXISTS axa_outbox_sent ON axa_outbox (sent_at) WHERE status = 'sent'",
];

/// Move the rows across, then empty the old table.
const MOVE: &str = "INSERT INTO axa_outbox (event_id, topic, kind, key, payload, status, sent_at, created_at, updated_at)
     SELECT event_id, entity, kind, entity_pid::text, payload,
            CASE WHEN published_at IS NULL THEN 'pending' ELSE 'sent' END,
            published_at, occurred_at, updated_at
     FROM event_outbox
     ORDER BY id
     ON CONFLICT (event_id) DO NOTHING;
     DELETE FROM event_outbox;";

/// Put the rows back for the old relay. A row still `failed` or `claimed` goes back unpublished.
const MOVE_BACK: &str =
    "INSERT INTO event_outbox (event_id, entity, entity_pid, kind, occurred_at, actor,
            schema_version, payload, published_at, created_at, updated_at)
     SELECT event_id, topic, key::uuid, kind, created_at, payload->>'actor',
            COALESCE((payload->>'schema_version')::int, 1), payload,
            CASE WHEN status = 'sent' THEN sent_at END, created_at, updated_at
     FROM axa_outbox
     ORDER BY id
     ON CONFLICT (event_id) DO NOTHING;";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create `axa_outbox` and move every `event_outbox` row into it.
    ///
    /// # Errors
    ///
    /// Propagates any SQL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for statement in V01_SETUP {
            db.execute_unprepared(statement).await?;
        }
        db.execute_unprepared(MOVE).await?;
        Ok(())
    }

    /// Move the rows back and drop `axa_outbox`.
    ///
    /// # Errors
    ///
    /// Propagates any SQL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(MOVE_BACK).await?;
        db.execute_unprepared("DROP TABLE IF EXISTS axa_outbox")
            .await?;
        Ok(())
    }
}
