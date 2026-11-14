//! The service's schema migrations: frozen SQL files in `migrations/`, run by sqlx, with a one-time bridge from the
//! history the old migrator kept in `seaql_migrations` (the service's migrator until SQ-1 (b), `sea-orm-migration`;
//! the one deliberate mention of it in this crate's code, as the table's name and the frozen files' headers say whose
//! it was).
//!
//! Each file is exactly the statements its old migration ran on PostgreSQL 18, in order, captured
//! from the server's statement log on 11 November 2026 (task SQ-1 (b)). A migration is named by its old name
//! turned into a sqlx version: `m20261109_000109_axa_outbox_record` is `20261109000109_axa_outbox_record`, so the
//! versions sort in the old order. A migration that can be rolled back has an `.up.sql` and a `.down.sql`; one that
//! is refused today is a plain `.sql`, and [`down`] refuses it with the reason it gave before.
//!
//! [`up`] first runs [`bridge`]: on a database the old migrator built, it copies every migration recorded in
//! `seaql_migrations` into sqlx's `_sqlx_migrations`, with the checksum of its frozen file, so nothing runs twice.
//! `seaql_migrations` is left as it is. Then sqlx applies what is pending, each migration in its own transaction, as
//! the old migrator did.

// SEC-I3: the runner runs SQL files; forbid unsafe.
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::LazyLock;

use sqlx::migrate::{Migrate, MigrateError, Migration, MigrationType, Migrator};
use sqlx::{AssertSqlSafe, Connection, PgConnection, PgPool, SqlSafeStr};

/// Every file in `migrations/`, name and text, embedded at build time by `build.rs`.
const FILES: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/migrations.rs"));

/// Every migration, read from [`FILES`] as sqlx's own resolver reads a directory (a unit test holds the two equal):
/// `<version>_<description>.sql`, `.up.sql` or `.down.sql`, the description with spaces for underscores, the
/// checksum the SHA-384 of the text.
pub static MIGRATOR: LazyLock<Migrator> = LazyLock::new(|| {
    Migrator::with_migrations(
        FILES
            .iter()
            .map(|(name, sql)| {
                let (version, rest) = name
                    .split_once('_')
                    .expect("a migration file is <version>_<description>.sql");
                let version: i64 = version.parse().expect("a migration version is a number");
                let migration_type = MigrationType::from_filename(rest);
                let description = rest
                    .trim_end_matches(migration_type.suffix())
                    .replace('_', " ");
                Migration::new(
                    version,
                    Cow::Owned(description),
                    migration_type,
                    sql.into_sql_str(),
                    sql.starts_with("-- no-transaction"),
                )
            })
            .collect(),
    )
});

/// How many migrations the old migrator knew: the frozen files, in its order. Every later migration is sqlx's own,
/// has no name in `seaql_migrations`, and is never bridged.
pub const FROZEN: usize = 143;

/// sqlx's history table.
pub const HISTORY: &str = "_sqlx_migrations";

/// The old migrator's history table, which the bridge reads and never changes.
pub const OLD_HISTORY: &str = "seaql_migrations";

/// The migrations that refuse to roll back, with the reason each gave under the old migrator. A migration with no
/// `.down.sql` that is missing here is refused with a general reason; a unit test keeps this list and the files in
/// step.
pub const REFUSED: &[(i64, &str)] = &[(
    20_261_109_000_109,
    "the event record cannot be rolled back: its tables refuse every change. axa_outbox is unchanged",
)];

/// What the runner can fail with.
#[derive(Debug)]
pub enum Error {
    /// A statement failed.
    Sqlx(sqlx::Error),
    /// sqlx's migrator failed: a migration failed, or the history does not match the files.
    Migrate(MigrateError),
    /// `seaql_migrations` or `_sqlx_migrations` names migrations this binary does not know: the database is newer
    /// than the binary.
    Unknown(Vec<String>),
    /// The migration refuses to roll back.
    Refused {
        /// Its old name.
        name: String,
        /// Why.
        reason: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlx(error) => write!(f, "{error}"),
            Self::Migrate(error) => write!(f, "{error}"),
            Self::Unknown(names) => write!(
                f,
                "the database records migrations this binary does not know ({}): it was migrated by a newer \
                 version. Run that version, or restore the backup taken before it",
                names.join(", ")
            ),
            Self::Refused { name, reason } => write!(f, "Migration Error: {reason} ({name})"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sqlx(error) => Some(error),
            Self::Migrate(error) => Some(error),
            Self::Unknown(_) | Self::Refused { .. } => None,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Sqlx(error)
    }
}

impl From<MigrateError> for Error {
    fn from(error: MigrateError) -> Self {
        Self::Migrate(error)
    }
}

/// The name the old migrator gave a migration: `m` + the date + `_` + the sequence + `_` + the description.
#[must_use]
pub fn old_name(migration: &Migration) -> String {
    let version = migration.version.to_string();
    let (date, sequence) = version.split_at(version.len().min(8));
    format!(
        "m{date}_{sequence}_{}",
        migration.description.replace(' ', "_")
    )
}

/// The migrations that change the schema (not the down files), oldest first.
fn ups() -> impl Iterator<Item = &'static Migration> {
    MIGRATOR
        .iter()
        .filter(|migration| migration.migration_type.is_up_migration())
}

/// Whether a table of this name is on the search path.
async fn exists(conn: &mut PgConnection, table: &str) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT to_regclass($1) IS NOT NULL")
        .bind(table)
        .fetch_one(conn)
        .await
}

/// The names `seaql_migrations` records that no migration here has.
fn unknown<'a>(recorded: impl IntoIterator<Item = &'a String>) -> Vec<String> {
    let known: HashSet<String> = ups().map(old_name).collect();
    recorded
        .into_iter()
        .filter(|name| !known.contains(*name))
        .cloned()
        .collect()
}

/// How many migrations the bridge copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bridged(pub usize);

/// The one-time step from the old migrator's history to sqlx's, in one transaction.
///
/// When `seaql_migrations` exists, every name in it must be a migration here, or this refuses ([`Error::Unknown`]) and
/// changes nothing. When `_sqlx_migrations` does not exist yet, it is made and every recorded migration is copied into
/// it, marked applied with its frozen file's checksum and the old migrator's time; after that, sqlx's table is the
/// history and this copies nothing again, so a migration rolled back by [`down`] is not marked applied again.
/// `seaql_migrations` is left as it is.
///
/// # Errors
///
/// A statement fails, or the database names a migration this binary does not know.
pub async fn bridge(conn: &mut PgConnection) -> Result<Bridged, Error> {
    let mut tx = conn.begin().await?;
    if !exists(&mut tx, OLD_HISTORY).await? {
        tx.commit().await?;
        return Ok(Bridged(0));
    }
    let recorded: Vec<(String, i64)> =
        sqlx::query_as("SELECT version, applied_at FROM seaql_migrations ORDER BY version")
            .fetch_all(&mut *tx)
            .await?;
    let strangers = unknown(recorded.iter().map(|(name, _)| name));
    if !strangers.is_empty() {
        return Err(Error::Unknown(strangers));
    }
    if exists(&mut tx, HISTORY).await? {
        tx.commit().await?;
        return Ok(Bridged(0));
    }
    tx.ensure_migrations_table(HISTORY).await?;
    let by_name: HashMap<String, &Migration> = ups().map(|m| (old_name(m), m)).collect();
    for (name, applied_at) in &recorded {
        let migration = by_name[name];
        sqlx::query(
            "INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time)
             VALUES ($1, $2, to_timestamp($3::bigint), TRUE, $4, 0)",
        )
        .bind(migration.version)
        .bind(&*migration.description)
        .bind(*applied_at)
        .bind(&*migration.checksum)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Bridged(recorded.len()))
}

/// Run `work` on one connection that holds sqlx's migration lock, so two processes migrating at once take turns. On
/// a failure the connection is closed rather than given back, so no lock is left held on a pooled connection.
async fn locked<T>(
    pool: &PgPool,
    work: impl AsyncFnOnce(&mut PgConnection) -> Result<T, Error>,
) -> Result<T, Error> {
    let mut conn = pool.acquire().await?;
    conn.lock().await?;
    match work(&mut conn).await {
        Ok(value) => {
            conn.unlock().await?;
            Ok(value)
        }
        Err(error) => {
            let _ = conn.detach().close().await;
            Err(error)
        }
    }
}

/// Apply every pending migration (`db migrate`, and the boot outside production): [`bridge`], then sqlx's runner,
/// which also refuses a history that names a migration this binary does not have, or whose file has changed.
///
/// # Errors
///
/// The bridge refuses, or a migration fails (it is rolled back, and those before it stay applied).
pub async fn up(pool: &PgPool) -> Result<(), Error> {
    locked(pool, async |conn| {
        bridge(conn).await?;
        MIGRATOR.run(&mut *conn).await?;
        Ok(())
    })
    .await
}

/// Roll back the newest `steps` migrations, newest first, each with its `.down.sql` in its own transaction, as the old
/// migrator's `down` did. A migration with no down file is refused, and the ones rolled back before it stay rolled
/// back.
///
/// # Errors
///
/// The bridge refuses, a migration refuses or its down file fails, or the history names an unknown migration.
pub async fn down(pool: &PgPool, steps: u32) -> Result<(), Error> {
    locked(pool, async |conn| {
        bridge(conn).await?;
        conn.ensure_migrations_table(HISTORY).await?;
        if let Some(version) = conn.dirty_version(HISTORY).await? {
            return Err(MigrateError::Dirty(version).into());
        }
        let mut applied: Vec<i64> = conn
            .list_applied_migrations(HISTORY)
            .await?
            .into_iter()
            .map(|m| m.version)
            .collect();
        if let Some(version) = applied.iter().find(|v| !MIGRATOR.version_exists(**v)) {
            return Err(MigrateError::VersionMissing(*version).into());
        }
        applied.sort_unstable_by(|a, b| b.cmp(a));
        for version in applied
            .into_iter()
            .take(usize::try_from(steps).unwrap_or(usize::MAX)) {
            let reverse = MIGRATOR
                .iter()
                .find(|m| m.version == version && m.migration_type.is_down_migration());
            let Some(reverse) = reverse else {
                let up = ups().find(|m| m.version == version);
                let name = up.map_or_else(|| version.to_string(), old_name);
                let reason = REFUSED
                    .iter()
                    .find(|(v, _)| *v == version)
                    .map_or("this migration has no down file: it cannot be rolled back; restore the backup taken before it", |(_, reason)| reason);
                return Err(Error::Refused {
                    name,
                    reason: reason.to_string(),
                });
            };
            tracing::info!("Rolling back migration '{}'", old_name(reverse));
            conn.revert(HISTORY, reverse).await?;
        }
        Ok(())
    })
    .await
}

/// Quote a name for SQL, as the old migrator's statement builder did.
fn quoted(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Drop every table and type in the current schema, then apply every migration (`db reset`, and each test's boot). It
/// drops what the old migrator's `fresh` dropped, found the same way: the base tables in `current_schema()`, then the
/// types there that are not array types and belong to no extension. Functions are left, as before (the migrations make
/// theirs with `CREATE OR REPLACE`).
///
/// **This destroys all data.** `axa::cli::parse` refuses `db reset` outside development and test.
///
/// # Errors
///
/// A statement fails, or a migration does.
pub async fn fresh(pool: &PgPool) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT table_name::text FROM information_schema.tables
         WHERE table_schema = CURRENT_SCHEMA() AND table_type = 'BASE TABLE'",
    )
    .fetch_all(&mut *tx)
    .await?;
    for table in tables {
        let sql = format!("DROP TABLE IF EXISTS {} CASCADE", quoted(&table));
        sqlx::query(AssertSqlSafe(sql)).execute(&mut *tx).await?;
    }
    let types: Vec<String> = sqlx::query_scalar(
        "SELECT pg_type.typname::text FROM pg_type
         LEFT JOIN pg_namespace ON pg_namespace.oid = pg_type.typnamespace
         LEFT JOIN pg_depend ON pg_depend.objid = pg_type.oid
              AND pg_depend.refclassid = 'pg_extension'::regclass::oid AND pg_depend.deptype = 'e'
         WHERE CURRENT_SCHEMA() = pg_namespace.nspname AND pg_type.typelem = 0 AND pg_depend.objid IS NULL",
    )
    .fetch_all(&mut *tx)
    .await?;
    for name in types {
        let sql = format!("DROP TYPE IF EXISTS {} CASCADE", quoted(&name));
        sqlx::query(AssertSqlSafe(sql)).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    up(pool).await
}

/// One migration and whether it has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// Its old name, which `db status` has always shown.
    pub name: String,
    /// Its sqlx version.
    pub version: i64,
    /// Whether it has been applied.
    pub applied: bool,
}

/// Every migration, oldest first, with whether it has run (`db status`). It reads and never writes: on a database
/// not yet bridged it reads `seaql_migrations`, so it shows what [`up`] would take as applied.
///
/// # Errors
///
/// A statement fails, or the history names a migration this binary does not know.
pub async fn status(pool: &PgPool) -> Result<Vec<Status>, Error> {
    let mut conn = pool.acquire().await?;
    let applied: HashSet<i64> = if exists(&mut conn, HISTORY).await? {
        let versions: Vec<i64> = sqlx::query_scalar(
            "SELECT version FROM _sqlx_migrations WHERE success ORDER BY version",
        )
        .fetch_all(&mut *conn)
        .await?;
        let strangers: Vec<String> = versions
            .iter()
            .filter(|v| !MIGRATOR.version_exists(**v))
            .map(ToString::to_string)
            .collect();
        if !strangers.is_empty() {
            return Err(Error::Unknown(strangers));
        }
        versions.into_iter().collect()
    } else if exists(&mut conn, OLD_HISTORY).await? {
        let names: Vec<String> =
            sqlx::query_scalar("SELECT version FROM seaql_migrations ORDER BY version")
                .fetch_all(&mut *conn)
                .await?;
        let strangers = unknown(&names);
        if !strangers.is_empty() {
            return Err(Error::Unknown(strangers));
        }
        let names: HashSet<String> = names.into_iter().collect();
        ups()
            .filter(|m| names.contains(&old_name(m)))
            .map(|m| m.version)
            .collect()
    } else {
        HashSet::new()
    };
    Ok(ups()
        .map(|m| Status {
            name: old_name(m),
            version: m.version,
            applied: applied.contains(&m.version),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every name the old migrator wrote into `seaql_migrations`, in its order: the bridge must know each one.
    const OLD_NAMES: &str = include_str!("old_names.txt");

    #[test]
    fn every_old_migration_has_its_frozen_file_in_the_same_order() {
        let names: Vec<String> = ups().take(FROZEN).map(old_name).collect();
        let old: Vec<&str> = OLD_NAMES.lines().collect();
        assert_eq!(old.len(), FROZEN);
        assert_eq!(names, old);
    }

    #[test]
    fn versions_are_fourteen_digits_and_ascending() {
        let versions: Vec<i64> = ups().map(|m| m.version).collect();
        assert!(
            versions.iter().all(|v| v.to_string().len() == 14),
            "{versions:?}"
        );
        assert!(versions.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn exactly_the_refused_migrations_have_no_down_file() {
        let downs: HashSet<i64> = MIGRATOR
            .iter()
            .filter(|m| m.migration_type.is_down_migration())
            .map(|m| m.version)
            .collect();
        let without: Vec<i64> = ups()
            .map(|m| m.version)
            .filter(|v| !downs.contains(v))
            .collect();
        let refused: Vec<i64> = REFUSED.iter().map(|(v, _)| *v).collect();
        assert_eq!(without, refused);
        assert_eq!(downs.len(), 142);
        assert!(MIGRATOR
            .iter()
            .filter(|m| m.migration_type.is_down_migration())
            .all(|m| ups().any(|u| u.version == m.version && u.migration_type.is_reversible())));
    }

    #[test]
    fn the_embedded_migrations_are_what_sqlx_reads_from_the_directory() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
        let resolved = sqlx::migrate::resolve_blocking(&dir).expect("resolve migrations/");
        let theirs: Vec<_> = resolved
            .iter()
            .map(|(m, _)| {
                (
                    m.version,
                    m.description.to_string(),
                    m.migration_type,
                    m.checksum.to_vec(),
                    m.no_tx,
                )
            })
            .collect();
        let ours: Vec<_> = MIGRATOR
            .iter()
            .map(|m| {
                (
                    m.version,
                    m.description.to_string(),
                    m.migration_type,
                    m.checksum.to_vec(),
                    m.no_tx,
                )
            })
            .collect();
        assert_eq!(ours.len(), 285);
        assert_eq!(ours, theirs);
    }

    #[test]
    fn every_file_runs_in_a_transaction() {
        assert!(MIGRATOR.iter().all(|m| !m.no_tx));
    }

    #[test]
    fn a_name_the_binary_does_not_know_is_found() {
        let names = vec![
            "m20220101_000001_plans".to_string(),
            "m20991231_000999_from_a_newer_binary".to_string(),
        ];
        assert_eq!(
            unknown(&names),
            vec!["m20991231_000999_from_a_newer_binary"]
        );
    }

    #[test]
    fn names_are_quoted_as_the_old_migrator_quoted_them() {
        assert_eq!(quoted("plans"), "\"plans\"");
        assert_eq!(quoted("a\"b"), "\"a\"\"b\"");
    }
}
