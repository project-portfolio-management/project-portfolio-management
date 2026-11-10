//! The way to the database for `sqlx`. `SeaORM` and `sqlx` share one pool while the service moves across, so a controller can
//! move on its own; a transaction is not shared, so one handler's writes stay on one side.
//!
//! Every statement goes through [`Counted`], so [`crate::query_count`] sees `sqlx` as it sees `SeaORM`. A guard over that
//! count can then not pass by seeing nothing. Nothing else in the service takes the raw pool: a census test below holds
//! that.

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
    Counted(ctx.db.get_postgres_connection_pool())
}

/// The shared pool, **uncounted**, for an `axa` crate that runs statements of its own and takes a [`PgPool`]
/// (`axa-idempotency`). Those statements are not seen by the query-count guards; nothing in this crate may use
/// this to run its own.
#[must_use]
pub fn for_axa_crate(db: &sea_orm::DatabaseConnection) -> PgPool {
    db.get_postgres_connection_pool().clone()
}

/// An open transaction. Dropped without [`Tx::commit`], it rolls back.
#[derive(Debug)]
pub struct Tx(sqlx::Transaction<'static, Postgres>);

impl Tx {
    /// The transaction's connection, counted.
    pub fn conn(&mut self) -> Counted<&mut PgConnection> {
        Counted(&mut self.0)
    }

    /// Commits.
    ///
    /// # Errors
    ///
    /// When the commit fails.
    pub async fn commit(self) -> Result<(), Error> {
        self.0.commit().await
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
    ctx.db.get_postgres_connection_pool().begin().await.map(Tx)
}

#[cfg(test)]
mod tests {
    /// A statement on the raw pool would go uncounted, and the query-count guards would pass without seeing it.
    #[test]
    fn nothing_outside_this_file_takes_the_raw_pool() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![root];
        let mut offenders = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && !path.ends_with("src/db.rs")
                    && std::fs::read_to_string(&path)
                        .expect("read file")
                        .contains("get_postgres_connection_pool")
                {
                    offenders.push(path.display().to_string());
                }
            }
        }
        assert!(offenders.is_empty(), "uncounted statements: {offenders:?}");
    }
}
