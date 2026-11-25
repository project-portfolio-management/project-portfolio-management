-- 20261125000117_cutover_evidence_binding: undo the up file beside it.

DROP INDEX IF EXISTS ccs_once;

CREATE UNIQUE INDEX ccs_once ON cutover_cosignatures (decision_pid, role);

ALTER TABLE cutover_cosignatures DROP COLUMN IF EXISTS evidence_digest;

ALTER TABLE cutover_decisions DROP COLUMN IF EXISTS decided_evidence_digest;

ALTER TABLE cutover_decisions DROP COLUMN IF EXISTS required_roles;
