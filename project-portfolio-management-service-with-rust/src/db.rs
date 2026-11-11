//! The way to the database. `AppContext::db` is `sqlx`'s pool (axa's `sqlx` feature), and every statement of the
//! service's own goes through [`pool`] or a [`begin`] transaction.
//!
//! Every statement goes through [`Counted`], so [`crate::query_count`] sees each one. A guard over that count can then
//! not pass by seeing nothing. Nothing else in the service, its tests or its benches takes the raw pool: a census test
//! below holds that.

use axa::AppContext;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use sqlx::postgres::{PgQueryResult, PgRow, PgStatement, PgTypeInfo};
use sqlx::{Either, Error, Execute, Executor, PgConnection, PgPool, Postgres, SqlStr};

/// An executor that counts each statement it runs.
#[derive(Clone, Copy, Debug)]
pub struct Counted<X>(X);

impl<'c, X: Executor<'c, Database = Postgres>> Executor<'c> for Counted<X> {
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<Either<PgQueryResult, PgRow>, Error>>
    where
        'c: 'e,
        E: 'q + Execute<'q, Postgres>,
    {
        crate::query_count::add_one();
        self.0.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E>(self, query: E) -> BoxFuture<'e, Result<Option<PgRow>, Error>>
    where
        'c: 'e,
        E: 'q + Execute<'q, Postgres>,
    {
        crate::query_count::add_one();
        self.0.fetch_optional(query)
    }

    fn prepare_with<'e>(
        self,
        sql: SqlStr,
        parameters: &'e [PgTypeInfo],
    ) -> BoxFuture<'e, Result<PgStatement, Error>>
    where
        'c: 'e,
    {
        self.0.prepare_with(sql, parameters)
    }
}

/// The shared pool, counted.
#[must_use]
pub fn pool(ctx: &AppContext) -> Counted<&PgPool> {
    Counted(&ctx.db)
}

/// The shared pool, **uncounted**, for an `axa` crate that runs statements of its own and takes a [`PgPool`]
/// (`axa-idempotency`). Those statements are not seen by the query-count guards; nothing in this crate may use
/// this to run its own.
#[must_use]
pub fn for_axa_crate(ctx: &AppContext) -> PgPool {
    ctx.db.clone()
}

/// The shared pool, **uncounted**, for the migration runner (`project_portfolio_management_migration`): sqlx's
/// migrator takes a [`PgPool`] and holds its lock on one connection. Its statements are the schema's, not a route's,
/// so no query-count guard is meant to see them; nothing else may use this.
#[must_use]
pub fn for_migrations(ctx: &AppContext) -> &PgPool {
    &ctx.db
}

/// An open transaction. Dropped without [`Tx::commit`], it rolls back.
#[derive(Debug)]
pub struct Tx(sqlx::Transaction<'static, Postgres>);

impl Tx {
    /// The transaction's connection, counted.
    pub fn conn(&mut self) -> Counted<&mut PgConnection> {
        Counted(&mut self.0)
    }

    /// Writes `event` to `axa-outbox` on this transaction, so it commits or rolls back with the change it describes.
    /// `axa-outbox` runs the statement itself, on the transaction; it is counted here as one.
    ///
    /// # Errors
    ///
    /// As [`axa::outbox::write`]: a database error (a repeated `event_id` among them), or a payload refused.
    pub async fn write_outbox(&mut self, event: &axa::outbox::Event) -> Result<(), Error> {
        crate::query_count::add_one();
        axa::outbox::write(&mut self.0, event).await
    }

    /// Puts the `failed` event `event_id` back to be sent (`axa::outbox::release`), on this transaction, naming who
    /// and why in the outbox's record. `axa-outbox` runs the statement itself, on the transaction; it is counted here
    /// as one. Whether it was released (`false` when the event was not `failed`).
    ///
    /// # Errors
    ///
    /// As [`axa::outbox::release`]: a database error, or `by` or `reason` refused.
    pub async fn release_outbox(
        &mut self,
        event_id: uuid::Uuid,
        by: &str,
        reason: &str,
    ) -> Result<bool, Error> {
        crate::query_count::add_one();
        axa::outbox::release(&mut self.0, event_id, by, reason).await
    }

    /// Commits.
    ///
    /// # Errors
    ///
    /// When the commit fails.
    pub async fn commit(self) -> Result<(), Error> {
        self.0.commit().await
    }

    /// Rolls back now. Dropping the transaction rolls it back too, but without saying whether that worked; a
    /// caller that answers an error when the rollback fails (the subjects preview) uses this.
    ///
    /// # Errors
    ///
    /// When the rollback fails.
    pub async fn rollback(self) -> Result<(), Error> {
        self.0.rollback().await
    }
}

/// Somewhere several statements can run in turn: a transaction ([`Tx`]), or the pool ([`pool`]), where each statement
/// commits on its own. A helper that runs more than one statement takes `&mut impl Session`, so it serves both and
/// never decides for its caller whether the writes share a transaction.
pub trait Session: Send {
    /// The executor for the next statement, counted.
    fn ex(&mut self) -> impl Executor<'_, Database = Postgres>;
}

impl Session for Tx {
    fn ex(&mut self) -> impl Executor<'_, Database = Postgres> {
        self.conn()
    }
}

impl Session for Counted<&PgPool> {
    fn ex(&mut self) -> impl Executor<'_, Database = Postgres> {
        *self
    }
}

/// Starts a transaction.
///
/// # Errors
///
/// When no connection can be had.
pub async fn begin(ctx: &AppContext) -> Result<Tx, Error> {
    ctx.db.begin().await.map(Tx)
}

#[cfg(test)]
mod tests {
    /// Whether a line of code reaches the context's pool itself: `<context>.db` as a field (not `.db_…` or a call), or
    /// `get_postgres_connection_pool`, the way to it the service used until SQ-1 (c), anywhere on the line, as the census
    /// always read it. A `.db` in a comment does not count.
    fn takes_the_raw_pool(line: &str) -> bool {
        if line.contains("get_postgres_connection_pool") {
            return true;
        }
        let code = line.split("//").next().unwrap_or("");
        code.match_indices(".db").any(|(at, _)| {
            let next = code[at + 3..].chars().next();
            !next.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '(')
        })
    }

    #[test]
    fn the_raw_pool_is_recognised() {
        for line in [
            "    .fetch_all(&ctx.db)",
            "let pool = ctx.db.clone();",
            "boot.app_context.db.begin().await",
            "sqlx::query(\"SELECT 1\").execute(&self.ctx.db).await",
            "x.db",
            "ctx.db.get_postgres_connection_pool()",
            "// get_postgres_connection_pool",
        ] {
            assert!(takes_the_raw_pool(line), "{line}");
        }
        for line in [
            "db::pool(&ctx)",
            "config.database.uri",
            "let x = self.db_name;",
            "crate::db::begin(&ctx)",
            "// ctx.db is the pool",
            "let db = axa::db::connect(&config.database).await?;",
        ] {
            assert!(!takes_the_raw_pool(line), "{line}");
        }
    }

    /// A statement on the raw pool would go uncounted, and the query-count guards would pass without seeing it. Every
    /// `.rs` file under `src/`, `tests/` and `benches/` but this one; the tests reach the database through
    /// [`super::pool`] and [`super::begin`] too (the migration tests open throwaway databases of their own). (Until
    /// SQ-1 (c) this read `src/` alone, for `get_postgres_connection_pool`.)
    #[test]
    fn nothing_outside_this_file_takes_the_raw_pool() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut stack = vec![
            manifest.join("src"),
            manifest.join("tests"),
            manifest.join("benches"),
        ];
        let (mut offenders, mut files) = (Vec::new(), 0_usize);
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read dir") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && !path.ends_with("src/db.rs")
                {
                    files += 1;
                    let text = std::fs::read_to_string(&path).expect("read file");
                    for (n, line) in text.lines().enumerate() {
                        if takes_the_raw_pool(line) {
                            offenders.push(format!(
                                "{}:{}: {}",
                                path.display(),
                                n + 1,
                                line.trim()
                            ));
                        }
                    }
                }
            }
        }
        assert!(files > 500, "read only {files} files");
        assert!(offenders.is_empty(), "uncounted statements: {offenders:#?}");
    }
}
