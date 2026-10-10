//! CLI tasks (loco extension point).

/// `exception_breaches` — notify breached delivery exceptions.
pub mod exception_breaches;

/// `retention_purge` — delete records past their retention period.
pub mod retention_purge;

/// `search_reindex` — rebuild the full-text index from the database.
pub mod search;
