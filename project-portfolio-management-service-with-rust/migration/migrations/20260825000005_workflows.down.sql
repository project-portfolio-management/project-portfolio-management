-- m20260825_000005_workflows: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS workflow_transitions;
                 DROP TABLE IF EXISTS workflow_states;
                 DROP TABLE IF EXISTS workflows;
