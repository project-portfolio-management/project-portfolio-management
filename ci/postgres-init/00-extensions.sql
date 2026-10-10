-- Runs once, on first initialization of a fresh Postgres data directory
-- (docker-entrypoint-initdb.d). Idempotent: safe to run again.
--
-- It enables, in `template1`, every extension the service needs, so each
-- database created later (the test harness creates its own) inherits them.
--
-- Today the migrations (project-portfolio-management-api-with-rust/migration)
-- and queries need NO extension: gen_random_uuid() is built in since
-- PostgreSQL 13, and full-text search uses the built-in to_tsvector('simple').
-- Verified by grepping the migrations and src for CREATE EXTENSION and
-- extension-provided functions. If one is ever added, add it here too, e.g.:
--
--   \connect template1
--   CREATE EXTENSION IF NOT EXISTS pg_trgm;

SELECT 'postgres-init: no extensions required' AS note;
