-- m20260825_000006_key_results: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the **OKR engine** — `key_results` and
-- `key_result_check_ins` (entity spec §5.9.2 / FR-27).
--
-- ## Why these hang off `objectives`, not `goals`
--
-- The spec first anchored key results to a plan's `goals[]` via a
-- `goal_id`. `Goal` has **no identifier** — it is a bare struct in the
-- JSONB payload, addressable only by array position, so a key result
-- bound to one would be orphaned by any reordering. `objectives`
-- already carries a `pid`, a `period` (the OKR cycle) and weighted
-- plan alignment through `objective_links`; that is the O in OKR.
--
-- ## `start_value` has no update path
--
-- The baseline is captured once at creation. Progress measured from a
-- moving baseline is not progress — an objective can be made to look
-- complete by editing where it started. The column is therefore
-- written on insert and never touched again by this service, the same
-- posture as `business_case_targets.approved_at`.
--
-- ## Check-ins are append-only
--
-- A check-in records what was observed on a date. Correcting one means
-- recording another, as with every other log here.

CREATE TABLE IF NOT EXISTS key_results (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     objective_pid UUID NOT NULL,
                     title VARCHAR NOT NULL,
                     metric VARCHAR NOT NULL
                         CHECK (metric IN ('number', 'percent', 'currency', 'boolean')),
                     direction VARCHAR NOT NULL
                         CHECK (direction IN ('increase', 'decrease', 'maintain')),
                     -- The baseline. Never updated: see the module note.
                     start_value BIGINT NOT NULL,
                     target_value BIGINT NOT NULL,
                     current_value BIGINT NOT NULL,
                     -- Required by `maintain`: a band is what the
                     -- direction means, and without one the key result
                     -- is unmeasurable rather than merely imprecise.
                     tolerance BIGINT NULL CHECK (tolerance IS NULL OR tolerance >= 0),
                     unit VARCHAR NULL,
                     currency VARCHAR NULL,
                     owner_ref VARCHAR NULL,
                     due_date DATE NULL,
                     deleted_at TIMESTAMPTZ NULL,
                     CONSTRAINT key_results_maintain_needs_tolerance
                         CHECK (direction <> 'maintain' OR tolerance IS NOT NULL),
                     CONSTRAINT key_results_currency_needs_code
                         CHECK (metric <> 'currency' OR currency IS NOT NULL)
                 );
                 CREATE INDEX IF NOT EXISTS key_results_objective
                     ON key_results (objective_pid);

                 CREATE TABLE IF NOT EXISTS key_result_check_ins (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     key_result_pid UUID NOT NULL,
                     observed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     value BIGINT NOT NULL,
                     -- Recorded, and **never blended into the score**: a
                     -- self-report and a measurement are different kinds
                     -- of evidence, and averaging them would make the
                     -- measured half unfalsifiable.
                     confidence SMALLINT NULL
                         CHECK (confidence IS NULL OR (confidence BETWEEN 0 AND 100)),
                     note VARCHAR NULL,
                     actor VARCHAR NULL
                 );
                 CREATE INDEX IF NOT EXISTS key_result_check_ins_kr
                     ON key_result_check_ins (key_result_pid, observed_at DESC);
