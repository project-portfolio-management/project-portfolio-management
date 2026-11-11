//! A count of the statements the service has run (CQ-11), so a request test can assert that a list endpoint's query
//! count does not grow with the rows it returns. Every statement of the service's own goes through
//! [`crate::db::Counted`], which counts it here; a census test in `src/db.rs` refuses the raw pool, so none goes
//! uncounted. (Until SQ-1 (c) the ORM's statements reached it through a metric callback on its connection.)

use std::sync::atomic::{AtomicU64, Ordering};

/// Process-wide running total of statements counted. Relaxed ordering is enough: it is only a counter.
static STATEMENTS: AtomicU64 = AtomicU64::new(0);

/// One statement run through [`crate::db::Counted`].
pub fn add_one() {
    STATEMENTS.fetch_add(1, Ordering::Relaxed);
}

/// Statements counted since the process started. Take a difference around one request; background work counts too,
/// so use it in a serial test.
#[must_use]
pub fn total() -> u64 {
    STATEMENTS.load(Ordering::Relaxed)
}
