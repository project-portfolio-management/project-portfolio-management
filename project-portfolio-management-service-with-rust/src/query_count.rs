//! A count of the statements the app's own connection has run (CQ-11), so a request test can assert that a list
//! endpoint's query count does not grow with the rows it returns. A cloned connection does not see the server's
//! statements; the callback has to be set on the connection the app builds, in `after_context`.

use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicU64, Ordering};

static STATEMENTS: AtomicU64 = AtomicU64::new(0);

/// Count every statement run on `db` from now on.
pub fn install(db: &mut DatabaseConnection) {
    db.set_metric_callback(|_| {
        STATEMENTS.fetch_add(1, Ordering::Relaxed);
    });
}

/// Statements counted since the process started. Take a difference around one request; background work counts too,
/// so use it in a serial test.
#[must_use]
pub fn total() -> u64 {
    STATEMENTS.load(Ordering::Relaxed)
}
