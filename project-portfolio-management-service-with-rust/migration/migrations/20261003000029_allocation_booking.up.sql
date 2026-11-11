-- m20261003_000029_allocation_booking: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **allocation booking** (PJ-4). An allocation is `committed`
-- (firm: the default, so every existing row means what it meant) or `proposed`
-- (a hoped-for booking that does not count toward committed utilization), and
-- may be flagged `requested` (asked of the resource manager).

ALTER TABLE allocations ADD COLUMN booking text NOT NULL DEFAULT 'committed';

ALTER TABLE allocations ADD COLUMN requested boolean NOT NULL DEFAULT false;

ALTER TABLE allocations ADD CONSTRAINT allocations_booking CHECK (booking IN ('proposed','committed'));
