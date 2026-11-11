-- m20261003_000029_allocation_booking: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE allocations DROP CONSTRAINT allocations_booking;

ALTER TABLE allocations DROP COLUMN requested;

ALTER TABLE allocations DROP COLUMN booking;
