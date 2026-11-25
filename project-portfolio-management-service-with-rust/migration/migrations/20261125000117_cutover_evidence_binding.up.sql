-- 20261125000117_cutover_evidence_binding: a co-signature attests to the evidence it was given on (EX-6a).
-- Required roles are frozen when a decision opens; each signature carries a digest of the marked criteria.

ALTER TABLE cutover_decisions ADD COLUMN required_roles text NULL;

ALTER TABLE cutover_decisions ADD COLUMN decided_evidence_digest text NULL;

ALTER TABLE cutover_cosignatures ADD COLUMN evidence_digest text NULL;

DROP INDEX IF EXISTS ccs_once;

CREATE UNIQUE INDEX ccs_once ON cutover_cosignatures (decision_pid, role, coalesce(evidence_digest, ''));
