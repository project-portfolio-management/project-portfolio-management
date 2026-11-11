//! The migration runner on real databases: the bridge from the old migrator's history to sqlx's, and rollback.
//!
//! Each test makes its own throwaway database on the server `DB_URL` names and drops it after. A database "the
//! old migrator built" is made the way that migrator left one: the frozen files applied (they are what its
//! migrations ran; SQ-1 (b) compared the two schemas and data with `pg_dump`), no `_sqlx_migrations`, and
//! `seaql_migrations` as that migrator created and filled it.
//!
//! `#[ignore]`d: needs PostgreSQL via `DB_URL`, and the right to create a database. Run with
//! `cargo test --test migrations -- --ignored --test-threads=1`.

use project_portfolio_management_migration::{self as migrations, Bridged, Error, MIGRATOR};
use sqlx::{AssertSqlSafe, PgPool};

/// The server's own database, from which the throwaway ones are made.
fn server_url() -> String {
    std::env::var("DB_URL").unwrap_or_else(|_| {
        "postgres://loco:loco@localhost:5432/project_portfolio_management_test".to_string()
    })
}

/// `url` with its database replaced by `name`.
fn with_database(url: &str, name: &str) -> String {
    let (base, query) = url
        .split_once('?')
        .map_or((url, None), |(b, q)| (b, Some(q)));
    let base = &base[..=base.rfind('/').expect("a database URL has a path")];
    match query {
        Some(query) => format!("{base}{name}?{query}"),
        None => format!("{base}{name}"),
    }
}

/// A throwaway database, dropped by [`Scratch::drop_it`].
struct Scratch {
    name: String,
    pool: PgPool,
}

impl Scratch {
    async fn new() -> Self {
        let name = format!("ppm_migrations_{}", uuid::Uuid::new_v4().simple());
        let admin = PgPool::connect(&server_url())
            .await
            .expect("connect to DB_URL");
        sqlx::query(AssertSqlSafe(format!("CREATE DATABASE {name}")))
            .execute(&admin)
            .await
            .expect("create a throwaway database");
        admin.close().await;
        let pool = PgPool::connect(&with_database(&server_url(), &name))
            .await
            .expect("connect to the throwaway database");
        Self { name, pool }
    }

    async fn drop_it(self) {
        self.pool.close().await;
        let admin = PgPool::connect(&server_url())
            .await
            .expect("connect to DB_URL");
        sqlx::query(AssertSqlSafe(format!(
            "DROP DATABASE IF EXISTS {} WITH (FORCE)",
            self.name
        )))
        .execute(&admin)
        .await
        .expect("drop the throwaway database");
    }
}

/// The version of the `n`-th migration (1-based).
fn nth(n: usize) -> i64 {
    MIGRATOR
        .iter()
        .filter(|m| m.migration_type.is_up_migration())
        .nth(n - 1)
        .expect("that many migrations")
        .version
}

/// How many migrations there are.
fn count() -> usize {
    MIGRATOR
        .iter()
        .filter(|m| m.migration_type.is_up_migration())
        .count()
}

/// Leave the database as the old migrator would have after its first `n` migrations.
async fn as_the_old_migrator_left_it(pool: &PgPool, n: usize) {
    MIGRATOR
        .run_to(nth(n), pool)
        .await
        .expect("apply the frozen files");
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("DROP TABLE _sqlx_migrations")
        .execute(&mut *tx)
        .await
        .unwrap();
    // The statement the old migrator's `install` ran.
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS "seaql_migrations" ( "version" varchar NOT NULL PRIMARY KEY, "applied_at" bigint NOT NULL )"#,
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    for (i, migration) in MIGRATOR
        .iter()
        .filter(|m| m.migration_type.is_up_migration())
        .take(n)
        .enumerate()
    {
        sqlx::query(r#"INSERT INTO "seaql_migrations" ("version", "applied_at") VALUES ($1, $2)"#)
            .bind(migrations::old_name(migration))
            .bind(1_794_380_615_i64 + i64::try_from(i).unwrap())
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    tx.commit().await.unwrap();
}

/// The schema, as sorted lines: columns, constraints, indexes, triggers and functions, outside the history tables.
async fn schema(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT line FROM (
           SELECT 'col ' || table_name || '.' || column_name || ' ' || data_type || ' ' || is_nullable || ' '
                  || coalesce(column_default, '') AS line
           FROM information_schema.columns
           WHERE table_schema = current_schema() AND table_name NOT IN ('_sqlx_migrations', 'seaql_migrations')
         UNION ALL
           SELECT 'con ' || conrelid::regclass::text || ' ' || conname || ' ' || pg_get_constraintdef(oid)
           FROM pg_constraint
           WHERE connamespace = current_schema()::regnamespace
             AND conrelid::regclass::text NOT IN ('_sqlx_migrations', 'seaql_migrations')
         UNION ALL
           SELECT 'idx ' || indexdef FROM pg_indexes
           WHERE schemaname = current_schema() AND tablename NOT IN ('_sqlx_migrations', 'seaql_migrations')
         UNION ALL
           SELECT 'trg ' || pg_get_triggerdef(oid) FROM pg_trigger WHERE NOT tgisinternal
         UNION ALL
           SELECT 'fn ' || proname || ' ' || md5(prosrc) FROM pg_proc
           WHERE pronamespace = current_schema()::regnamespace
         ) lines ORDER BY line",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

/// `_sqlx_migrations`, every column.
async fn history(pool: &PgPool) -> Vec<(i64, String, String, bool, Vec<u8>, i64)> {
    sqlx::query_as(
        "SELECT version, description, installed_on::text, success, checksum, execution_time
         FROM _sqlx_migrations ORDER BY version",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

/// `seaql_migrations`, every column.
async fn old_history(pool: &PgPool) -> Vec<(String, i64)> {
    sqlx::query_as("SELECT version, applied_at FROM seaql_migrations ORDER BY version")
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn exists(pool: &PgPool, table: &str) -> bool {
    sqlx::query_scalar("SELECT to_regclass($1) IS NOT NULL")
        .bind(table)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// The schema every migration gives, applied by sqlx from nothing.
async fn the_whole_schema() -> Vec<String> {
    let reference = Scratch::new().await;
    migrations::up(&reference.pool).await.unwrap();
    let lines = schema(&reference.pool).await;
    reference.drop_it().await;
    lines
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn a_database_the_old_migrator_built_is_bridged_and_nothing_runs_again() {
    let whole = the_whole_schema().await;
    let db = Scratch::new().await;
    as_the_old_migrator_left_it(&db.pool, count()).await;
    let before = schema(&db.pool).await;
    let old_before = old_history(&db.pool).await;
    assert_eq!(before, whole);

    migrations::up(&db.pool).await.unwrap();

    assert_eq!(schema(&db.pool).await, before, "nothing ran");
    let rows = history(&db.pool).await;
    assert_eq!(rows.len(), count());
    for ((version, description, _, success, checksum, took), migration) in rows.iter().zip(
        MIGRATOR
            .iter()
            .filter(|m| m.migration_type.is_up_migration()),
    ) {
        assert_eq!(*version, migration.version);
        assert_eq!(description, &*migration.description);
        assert!(success);
        assert_eq!(checksum.as_slice(), &*migration.checksum, "{version}");
        assert_eq!(*took, 0, "{version} was copied, not run");
    }
    assert_eq!(
        old_history(&db.pool).await,
        old_before,
        "seaql_migrations is left as it was"
    );
    assert!(
        migrations::status(&db.pool)
            .await
            .unwrap()
            .iter()
            .all(|m| m.applied)
    );
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn a_database_the_old_migrator_built_part_way_gets_the_rest_from_sqlx() {
    let whole = the_whole_schema().await;
    let db = Scratch::new().await;
    // Up to the one before `oxford_spelling`, with a risk in the spelling it rewrites.
    let applied = 82;
    assert_eq!(
        migrations::old_name(
            MIGRATOR
                .iter()
                .find(|m| m.version == nth(applied + 1))
                .unwrap()
        ),
        "m20261009_000049_oxford_spelling"
    );
    as_the_old_migrator_left_it(&db.pool, applied).await;
    sqlx::query(
        "INSERT INTO risks (pid, plan_pid, title, probability, impact, status)
         VALUES (gen_random_uuid(), gen_random_uuid(), 'a risk', 3, 3, 'materialised')",
    )
    .execute(&db.pool)
    .await
    .unwrap();
    let status = migrations::status(&db.pool).await.unwrap();
    assert_eq!(
        status.iter().filter(|m| m.applied).count(),
        applied,
        "status reads the old history"
    );

    migrations::up(&db.pool).await.unwrap();

    assert_eq!(schema(&db.pool).await, whole);
    let rows = history(&db.pool).await;
    assert_eq!(rows.len(), count());
    assert!(rows[..applied].iter().all(|row| row.5 == 0), "copied");
    assert!(rows[applied..].iter().all(|row| row.5 > 0), "run by sqlx");
    let status: String = sqlx::query_scalar("SELECT status FROM risks")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(status, "materialized", "the data migration ran");
    assert_eq!(
        old_history(&db.pool).await.len(),
        applied,
        "seaql_migrations is left as it was"
    );
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn a_second_run_changes_nothing() {
    let db = Scratch::new().await;
    as_the_old_migrator_left_it(&db.pool, 120).await;
    migrations::up(&db.pool).await.unwrap();
    let (schema_after, rows, old) = (
        schema(&db.pool).await,
        history(&db.pool).await,
        old_history(&db.pool).await,
    );

    migrations::up(&db.pool).await.unwrap();
    let mut conn = db.pool.acquire().await.unwrap();
    assert_eq!(migrations::bridge(&mut conn).await.unwrap(), Bridged(0));
    drop(conn);

    assert_eq!(schema(&db.pool).await, schema_after);
    assert_eq!(history(&db.pool).await, rows);
    assert_eq!(old_history(&db.pool).await, old);
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn an_old_history_naming_a_migration_this_binary_does_not_know_is_refused() {
    let db = Scratch::new().await;
    as_the_old_migrator_left_it(&db.pool, count()).await;
    sqlx::query("INSERT INTO seaql_migrations (version, applied_at) VALUES ('m20991231_000999_from_a_newer_binary', 1)")
        .execute(&db.pool)
        .await
        .unwrap();
    let before = (schema(&db.pool).await, old_history(&db.pool).await);

    let refused = migrations::up(&db.pool).await.unwrap_err();
    assert!(
        matches!(&refused, Error::Unknown(names) if names == &["m20991231_000999_from_a_newer_binary"]),
        "{refused}"
    );
    assert!(
        refused.to_string().contains("migrated by a newer version"),
        "{refused}"
    );
    assert!(
        !exists(&db.pool, "_sqlx_migrations").await,
        "nothing was copied"
    );
    assert_eq!(
        (schema(&db.pool).await, old_history(&db.pool).await),
        before
    );
    assert!(matches!(
        migrations::status(&db.pool).await,
        Err(Error::Unknown(_))
    ));
    assert!(matches!(
        migrations::down(&db.pool, 1).await,
        Err(Error::Unknown(_))
    ));

    // Once bridged, too: the name is checked on every run.
    sqlx::query("DELETE FROM seaql_migrations WHERE version LIKE 'm2099%'")
        .execute(&db.pool)
        .await
        .unwrap();
    migrations::up(&db.pool).await.unwrap();
    sqlx::query("INSERT INTO seaql_migrations (version, applied_at) VALUES ('m20991231_000999_from_a_newer_binary', 1)")
        .execute(&db.pool)
        .await
        .unwrap();
    assert!(matches!(
        migrations::up(&db.pool).await,
        Err(Error::Unknown(_))
    ));
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn a_rollback_refused_before_is_refused_still() {
    let db = Scratch::new().await;
    migrations::up(&db.pool).await.unwrap();
    let (before, rows) = (schema(&db.pool).await, history(&db.pool).await);

    let refused = migrations::down(&db.pool, 1).await.unwrap_err();
    match &refused {
        Error::Refused { name, reason } => {
            assert_eq!(name, "m20261109_000109_axa_outbox_record");
            assert_eq!(
                reason,
                "the event record cannot be rolled back: its tables refuse every change. axa_outbox is unchanged"
            );
        }
        other => panic!("not refused: {other}"),
    }
    assert_eq!(schema(&db.pool).await, before);
    assert_eq!(history(&db.pool).await, rows);
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn a_rollback_that_worked_before_works_still_and_is_not_undone_by_the_bridge() {
    // On a database sqlx built: through `axa_outbox`, then back one.
    let db = Scratch::new().await;
    MIGRATOR.run_to(nth(141), &db.pool).await.unwrap();
    let at_141 = schema(&db.pool).await;
    MIGRATOR.run_to(nth(142), &db.pool).await.unwrap();
    assert!(exists(&db.pool, "axa_outbox").await);
    migrations::down(&db.pool, 1).await.unwrap();
    assert!(!exists(&db.pool, "axa_outbox").await, "the down file ran");
    assert_eq!(schema(&db.pool).await, at_141);
    assert_eq!(history(&db.pool).await.len(), 141);
    db.drop_it().await;

    // On one the old migrator built: the bridge copies 142, the rollback removes 108 from sqlx's history only, and the
    // next run applies 108 again rather than taking the old history's word for it.
    let whole = the_whole_schema().await;
    let db = Scratch::new().await;
    as_the_old_migrator_left_it(&db.pool, 142).await;
    migrations::down(&db.pool, 1).await.unwrap();
    assert!(!exists(&db.pool, "axa_outbox").await);
    assert_eq!(history(&db.pool).await.len(), 141);
    assert_eq!(
        old_history(&db.pool).await.len(),
        142,
        "seaql_migrations is left as it was"
    );
    migrations::up(&db.pool).await.unwrap();
    assert_eq!(schema(&db.pool).await, whole);
    let rows = history(&db.pool).await;
    assert!(rows[141..].iter().all(|row| row.5 > 0), "108 and 109 ran");
    db.drop_it().await;
}

#[tokio::test]
#[ignore = "needs PostgreSQL via DB_URL"]
async fn reset_drops_both_histories_and_builds_from_nothing() {
    let whole = the_whole_schema().await;
    let db = Scratch::new().await;
    as_the_old_migrator_left_it(&db.pool, 60).await;
    migrations::fresh(&db.pool).await.unwrap();
    assert!(!exists(&db.pool, "seaql_migrations").await);
    assert_eq!(schema(&db.pool).await, whole);
    assert!(history(&db.pool).await.iter().all(|row| row.5 > 0));
    db.drop_it().await;
}
