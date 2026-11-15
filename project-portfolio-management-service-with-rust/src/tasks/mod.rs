//! CLI tasks (extension point).

/// `dhcw_roadmaps_apply` — write the DHCW roadmap spreadsheet onto existing Product plans, unpublished (RE-5).
pub mod dhcw_roadmaps_apply;

/// `dhcw_roadmaps_dry_run` — what the DHCW roadmap spreadsheet would become, writing nothing (RE-5).
pub mod dhcw_roadmaps_dry_run;

/// `director_pack_schedule` — draft the monthly director packs that are due.
pub mod director_pack_schedule;

/// `exceptions_digest` — send the weekly portfolio-exceptions digest.
pub mod exceptions_digest;

/// `exception_breaches` — notify breached delivery exceptions.
pub mod exception_breaches;

/// `outbox_retry_failed` — send the event outbox's failed rows again.
pub mod outbox_retry_failed;

/// `retention_purge` — delete records past their retention period.
pub mod retention_purge;

/// `search_reindex` — rebuild the full-text index from the database.
pub mod search;

/// `tolerance_escalations` — escalate red tolerances nobody has escalated.
pub mod tolerance_escalations;
