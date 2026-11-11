-- m20261020_000081_link_systems: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **external systems on plan links**. A plan link may name the work-tracking system it points into
-- (`system`: Jira, Trello, Asana, Linear, monday.com and others) and that system's own identifier for the thing
-- (`external_id`, for example a Jira key or a board id). A new link kind, `tracker`, is a project or board in such
-- a system; a single issue there is a `ticket`. The service still never fetches the link.

ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS system VARCHAR NULL;
                 ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS external_id VARCHAR NULL;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_kind;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_kind
                     CHECK (kind IN ('docs','repo','ticket','design','chat','tracker','other'));
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_system;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_system
                     CHECK (system IS NULL OR system IN
                         ('jira','trello','asana','linear','monday','clickup','azure_devops','github','other'));
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_tracker_has_system;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_tracker_has_system
                     CHECK (kind <> 'tracker' OR system IS NOT NULL);
