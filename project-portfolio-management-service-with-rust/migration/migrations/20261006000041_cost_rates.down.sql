-- m20261006_000041_cost_rates: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "time_entry_costs";

DROP TABLE "task_costs";

DROP TABLE "cost_rates";

DROP TABLE "cost_rate_tables";
