-- m20260902_000001_automation_multi_action: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE automations
                     ADD COLUMN action_kind VARCHAR NOT NULL DEFAULT '',
                     ADD COLUMN action_value JSONB NOT NULL DEFAULT '{}'::jsonb;
                 UPDATE automations
                     SET action_kind = COALESCE(actions -> 0 ->> 'kind', ''),
                         action_value = COALESCE(actions -> 0 -> 'value', '{}'::jsonb);
                 ALTER TABLE automations
                     DROP CONSTRAINT IF EXISTS automations_actions_not_empty,
                     DROP COLUMN actions;

                 DROP INDEX IF EXISTS automation_runs_automation_action;
                 ALTER TABLE automation_runs DROP COLUMN action_index;
