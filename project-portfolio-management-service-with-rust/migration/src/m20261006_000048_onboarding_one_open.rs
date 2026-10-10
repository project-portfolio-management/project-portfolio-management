//! Migration: two guards the code alone could not hold. At most one **open** onboarding case per plan, kind and subject — what the
//! API checks with a read, the database now enforces, so two simultaneous starts cannot both win.

use sea_orm_migration::prelude::*;

/// The one-open-case migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the partial unique index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error (including existing duplicate open cases).
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("CREATE UNIQUE INDEX onboarding_cases_one_open ON onboarding_cases (plan_pid, kind, COALESCE(subject_ref, '')) WHERE status = 'open'").await?;
        // A hammock is a derived span with no dependencies of its own (PJ-7): the API checks
        // before enabling one, and this refuses a link that arrives in between.
        db.execute_unprepared("CREATE OR REPLACE FUNCTION plan_dependencies_not_hammock() RETURNS trigger AS $$ BEGIN IF EXISTS (SELECT 1 FROM tasks WHERE is_hammock AND pid IN (NEW.predecessor_pid, NEW.successor_pid)) THEN RAISE EXCEPTION 'a hammock has no dependencies of its own'; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql").await?;
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS plan_dependencies_not_hammock ON plan_dependencies",
        )
        .await?;
        db.execute_unprepared("CREATE TRIGGER plan_dependencies_not_hammock BEFORE INSERT ON plan_dependencies FOR EACH ROW EXECUTE FUNCTION plan_dependencies_not_hammock()").await?;
        Ok(())
    }

    /// Drop it.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS onboarding_cases_one_open")
            .await?;
        Ok(())
    }
}
