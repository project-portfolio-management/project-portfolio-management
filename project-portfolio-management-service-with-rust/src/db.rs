//! The database pool for plain SQL. While the service moves from `SeaORM` to sqlx a controller at a time, both use the one
//! pool, so a query here sees what `SeaORM` wrote. Only a transaction is not shared: a handler stays on one side.

use axa::AppContext;
use sqlx::PgPool;

/// The pool of the service's connection.
#[must_use]
pub fn pool(ctx: &AppContext) -> &PgPool {
    ctx.db.get_postgres_connection_pool()
}
