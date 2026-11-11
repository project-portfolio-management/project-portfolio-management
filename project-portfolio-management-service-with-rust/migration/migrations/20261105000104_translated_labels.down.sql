-- m20261105_000104_translated_labels: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE issue_types DROP COLUMN labels;
                 ALTER TABLE organization_roles DROP COLUMN labels;
                 ALTER TABLE workflow_states DROP COLUMN labels;
