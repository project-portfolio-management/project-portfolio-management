//! Migration: **controlled documents** (PL-6). A `controlled_documents` row is a document; each of its `document_versions` moves `draft → in_review → approved → effective → superseded`. The database itself enforces what the rules promise: a version that is no longer a draft cannot have its text changed or be deleted; at most one version of a document is `effective`, and at most one is in flight; a version cannot be approved, effective or superseded without the approval request that approved it; and `superseded` names its successor.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The controlled-documents migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, constraints and triggers.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "controlled_documents",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("title", ColType::String),
                ("plan_pid", ColType::UuidNull),
                ("created_by", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "document_versions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("document_pid", ColType::Uuid),
                ("version", ColType::Integer),
                ("body", ColType::Text),
                ("change_summary", ColType::StringNull),
                ("status", ColType::String),
                ("approval_pid", ColType::UuidNull),
                ("effective_from", ColType::DateNull),
                ("superseded_on", ColType::DateNull),
                ("superseded_by", ColType::UuidNull),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX document_versions_number ON document_versions (document_pid, version)",
            "ALTER TABLE document_versions ADD CONSTRAINT document_versions_status CHECK (status IN ('draft','in_review','approved','effective','superseded'))",
            // At most one effective version of a document, and at most one still in flight.
            "CREATE UNIQUE INDEX document_versions_one_effective ON document_versions (document_pid) WHERE status = 'effective'",
            "CREATE UNIQUE INDEX document_versions_one_in_flight ON document_versions (document_pid) WHERE status IN ('draft','in_review','approved')",
            // No approved, effective or superseded version without its approval.
            "ALTER TABLE document_versions ADD CONSTRAINT document_versions_needs_approval CHECK (status IN ('draft','in_review') OR approval_pid IS NOT NULL)",
            "ALTER TABLE document_versions ADD CONSTRAINT document_versions_effective_date CHECK (status NOT IN ('effective','superseded') OR effective_from IS NOT NULL)",
            "ALTER TABLE document_versions ADD CONSTRAINT document_versions_successor CHECK ((status = 'superseded') = (superseded_by IS NOT NULL AND superseded_on IS NOT NULL))",
            // Once it has left draft, the text is the text; and only a draft is deleted.
            "CREATE OR REPLACE FUNCTION document_versions_guard() RETURNS trigger AS $$ BEGIN IF TG_OP = 'DELETE' THEN IF OLD.status <> 'draft' THEN RAISE EXCEPTION 'a version past draft is never deleted'; END IF; RETURN OLD; END IF; IF OLD.status <> 'draft' AND (NEW.body <> OLD.body OR NEW.change_summary IS DISTINCT FROM OLD.change_summary OR NEW.version <> OLD.version OR NEW.document_pid <> OLD.document_pid) THEN RAISE EXCEPTION 'a version past draft is immutable: make the next draft'; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql",
            "DROP TRIGGER IF EXISTS document_versions_guard ON document_versions",
            "CREATE TRIGGER document_versions_guard BEFORE UPDATE OR DELETE ON document_versions FOR EACH ROW EXECUTE FUNCTION document_versions_guard()",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop it all again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "document_versions").await?;
        drop_table(m, "controlled_documents").await?;
        m.get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS document_versions_guard()")
            .await?;
        Ok(())
    }
}
