-- m20261006_000046_processes: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE approval_requests DROP COLUMN IF EXISTS process_step;

ALTER TABLE approval_requests DROP COLUMN IF EXISTS process_execution_pid;

DROP TABLE "process_execution_logs";

DROP TABLE "process_executions";

DROP TABLE "process_versions";

DROP TABLE "processes";

DROP FUNCTION IF EXISTS processes_refuse_change();
