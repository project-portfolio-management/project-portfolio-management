-- m20261010_000051_audit_logs_protected: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **protect the audit trail in the database** (RG-3). A trigger refuses an `UPDATE` of an `audit_logs` row unless the transaction has set `ppm.audit_pseudonymise = 'on'` (the data-subject tool), and refuses a `DELETE` unless it has set `ppm.retention_purge = 'on'` (the retention task). Both are transaction-local settings (`set_config(..., true)`), so a forgotten flag cannot leak to another request.
--
-- The limit, stated plainly: a database superuser or the table owner can drop the trigger. This stops an application bug or a casual `DELETE`, and makes a deliberate change a visible act; it is not protection against an administrator. A `TRUNCATE` is not blocked (the test harness uses it).

CREATE OR REPLACE FUNCTION audit_logs_guard() RETURNS trigger AS $$
BEGIN
  IF TG_OP = 'UPDATE' THEN
    IF coalesce(current_setting('ppm.audit_pseudonymise', true), '') <> 'on' THEN
      RAISE EXCEPTION 'audit_logs rows cannot be changed' USING ERRCODE = 'integrity_constraint_violation';
    END IF;
    RETURN NEW;
  END IF;
  IF coalesce(current_setting('ppm.retention_purge', true), '') <> 'on' THEN
    RAISE EXCEPTION 'audit_logs rows cannot be deleted outside the retention task' USING ERRCODE = 'integrity_constraint_violation';
  END IF;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_logs_guard BEFORE UPDATE OR DELETE ON audit_logs FOR EACH ROW EXECUTE FUNCTION audit_logs_guard();
