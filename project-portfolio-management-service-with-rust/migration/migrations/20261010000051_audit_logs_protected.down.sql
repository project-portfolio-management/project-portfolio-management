-- m20261010_000051_audit_logs_protected: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS audit_logs_guard ON audit_logs;

DROP FUNCTION IF EXISTS audit_logs_guard();
