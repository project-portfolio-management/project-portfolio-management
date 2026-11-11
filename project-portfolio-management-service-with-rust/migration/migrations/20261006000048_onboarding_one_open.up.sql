-- m20261006_000048_onboarding_one_open: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: two guards the code alone could not hold. At most one **open** onboarding case per plan, kind and subject — what the
-- API checks with a read, the database now enforces, so two simultaneous starts cannot both win.

CREATE UNIQUE INDEX onboarding_cases_one_open ON onboarding_cases (plan_pid, kind, COALESCE(subject_ref, '')) WHERE status = 'open';

CREATE OR REPLACE FUNCTION plan_dependencies_not_hammock() RETURNS trigger AS $$ BEGIN IF EXISTS (SELECT 1 FROM tasks WHERE is_hammock AND pid IN (NEW.predecessor_pid, NEW.successor_pid)) THEN RAISE EXCEPTION 'a hammock has no dependencies of its own'; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS plan_dependencies_not_hammock ON plan_dependencies;

CREATE TRIGGER plan_dependencies_not_hammock BEFORE INSERT ON plan_dependencies FOR EACH ROW EXECUTE FUNCTION plan_dependencies_not_hammock();
