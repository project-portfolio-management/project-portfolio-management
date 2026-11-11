-- m20261006_000047_controlled_documents: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "document_versions";

DROP TABLE "controlled_documents";

DROP FUNCTION IF EXISTS document_versions_guard();
