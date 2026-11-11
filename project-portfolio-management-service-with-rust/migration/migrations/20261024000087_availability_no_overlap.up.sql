-- m20261024_000087_availability_no_overlap: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: one person's availability ranges cannot overlap (CF-3). The service already checks this
-- under a per-person advisory lock; this is the database's own guarantee, so a second writer, a
-- script or a lost lock cannot store two ranges that cover the same day. Inclusive on both ends, as
-- the service reads them, and only for ranges not removed.
--
-- Needs the `btree_gist` extension (equality on text inside a `GiST` index). It ships with PostgreSQL's
-- contrib and is a *trusted* extension from version 13, so a role that may create objects in the
-- database can add it. If the table already holds overlapping ranges this fails, naming the
-- constraint; remove or change one of each pair and run it again.

CREATE EXTENSION IF NOT EXISTS btree_gist;

ALTER TABLE person_availabilities ADD CONSTRAINT person_availabilities_no_overlap
             EXCLUDE USING gist (person_ref WITH =, daterange(from_date, to_date, '[]') WITH &&)
             WHERE (deleted_at IS NULL);
