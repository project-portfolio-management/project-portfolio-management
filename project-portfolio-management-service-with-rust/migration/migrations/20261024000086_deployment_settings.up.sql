-- m20261024_000086_deployment_settings: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: `deployment_settings`, a small key/value table for settings an administrator changes at
-- run time. The first key is `today_time_zone` (the IANA zone that defines "today" for portfolio-wide
-- views). Absent means the default (UTC).

CREATE TABLE IF NOT EXISTS deployment_settings (
                     key        text PRIMARY KEY,
                     value      text NOT NULL,
                     updated_by text,
                     updated_at timestamptz NOT NULL DEFAULT now()
                 );
