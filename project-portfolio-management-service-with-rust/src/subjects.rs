//! Data-subject requests (RG-6): everything held about one person reference, and its replacement by
//! a pseudonym. The pure parts, unit-tested; the SQL runner is in [`crate::controllers::subjects`].
//!
//! **Policy: pseudonymise, do not delete.** The audit trail exists to show who did what; deleting a
//! person's rows would destroy that evidence. Replacing the reference with a stable pseudonym keeps
//! the evidence and removes the link to the person. (Plan §62 question 2 recommended this; the owner
//! has not ruled otherwise.)
//!
//! **What this does not find:** a name or an e-mail address typed into free text (a title, a comment,
//! a document), the content of uploaded files, the `snapshot` JSON of an audit row, and backups.
//! Those need a human search and the retention schedule.

use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Every `(table, column)` that holds a **person reference** (a token subject, or a
/// `person:`/`worker:` reference). A test keeps this in step with
/// `governance/personal-data-inventory.csv`.
pub const COLUMNS: &[(&str, &str)] = &[
    ("access_records", "person_ref"),
    ("access_records", "recorded_by"),
    ("allocations", "person_ref"),
    ("announcements", "author_ref"),
    ("approval_requests", "approver_ref"),
    ("approval_requests", "decided_by"),
    ("approval_requests", "requested_by"),
    ("artifacts", "created_by"),
    ("attachments", "uploaded_by"),
    ("audit_logs", "actor"),
    ("bulk_jobs", "cancelled_by"),
    ("bulk_jobs", "requested_by"),
    ("business_case_targets", "approved_by_ref"),
    ("canvas_items", "owner_ref"),
    ("ceremonies", "facilitator_ref"),
    ("comment_edits", "edited_by"),
    ("comments", "author_sub"),
    ("components", "lead_ref"),
    ("control_actions", "owner_ref"),
    ("controlled_documents", "created_by"),
    ("controls", "owner_ref"),
    ("cost_rate_tables", "person_ref"),
    ("cost_rates", "created_by"),
    ("document_versions", "created_by"),
    ("event_outbox", "actor"),
    ("feed_tokens", "sub"),
    ("gate_reviews", "approver_ref"),
    ("guest_grants", "granted_by"),
    ("guest_grants", "guest_sub"),
    ("key_result_check_ins", "actor"),
    ("key_results", "owner_ref"),
    ("kpi_readings", "actor"),
    ("kpis", "owner_ref"),
    ("merge_records", "actor"),
    ("milestone_slips", "actor"),
    ("non_working_periods", "person_ref"),
    ("notification_prefs", "sub"),
    ("notification_sinks", "sub"),
    ("notifications", "recipient_ref"),
    ("onboarding_cases", "created_by"),
    ("onboarding_cases", "subject_ref"),
    ("onboarding_items", "owner_ref"),
    ("onboarding_items", "resolved_by"),
    ("person_availabilities", "person_ref"),
    ("phase_transitions", "actor"),
    ("plan_links", "created_by"),
    ("plan_templates", "created_by"),
    ("planned_slots", "sub"),
    ("process_execution_logs", "actor"),
    ("process_executions", "started_by"),
    ("process_versions", "created_by"),
    ("processes", "created_by"),
    ("proposals", "sponsor_ref"),
    ("raci_assignments", "person_ref"),
    ("reactions", "sub"),
    ("report_schedules", "owner"),
    ("resource_roles", "person_ref"),
    ("review_queue_items", "resolved_by"),
    ("reviews", "invited_by"),
    ("reviews", "reviewer_ref"),
    ("risks", "owner_ref"),
    ("saved_views", "sub"),
    ("schedule_baselines", "created_by"),
    ("scheduled_actions", "created_by"),
    ("task_transitions", "actor_ref"),
    ("task_transitions", "assignee_ref"),
    ("tasks", "assignee_ref"),
    ("time_entries", "actor_ref"),
    ("time_entry_costs", "person_ref"),
    ("timesheet_weeks", "decided_by"),
    ("timesheet_weeks", "person_ref"),
    ("value_points", "actor"),
    ("watchers", "sub"),
    ("issues", "owner_ref"),
    ("issues", "raised_by"),
];

/// Identifier columns in the inventory that are **not** person references, with the reason.
pub const EXCLUDED: &[(&str, &str, &str)] = &[
    ("adoption_snapshots", "active_users", "a count"),
    ("adoption_snapshots", "target_users", "a count"),
    ("business_case_targets", "promised_by", "a date"),
    (
        "document_versions",
        "superseded_by",
        "a version id, not a person",
    ),
    ("hammock_links", "member_pid", "a plan id, not a person"),
    (
        "report_definitions",
        "group_by",
        "a grouping name, not a person",
    ),
    ("reviews", "reviewer_scope", "a scope label, not a person"),
    (
        "notification_sinks",
        "destination",
        "a contact address; listed in the export, removed by the person in their preferences",
    ),
];

/// The values a stored reference to this person can take: the bare identifier (a token subject, as
/// the audit trail and many `*_by` columns store it), and `person:<id>` / `worker:<id>`. Input may be
/// any of those forms. Empty for a blank input.
#[must_use]
pub fn forms(subject: &str) -> Vec<String> {
    let s = subject.trim();
    let id = s
        .strip_prefix("person:")
        .or_else(|| s.strip_prefix("worker:"))
        .unwrap_or(s)
        .trim();
    if id.is_empty() {
        return Vec::new();
    }
    vec![
        id.to_string(),
        format!("person:{id}"),
        format!("worker:{id}"),
    ]
}

/// The identifier inside any form of a reference.
#[must_use]
pub fn bare(subject: &str) -> String {
    forms(subject).into_iter().next().unwrap_or_default()
}

/// A stable pseudonym for a person: a UUID derived from SHA-256 of the salt and the bare identifier.
/// The same person always gets the same pseudonym, so records stay linked to each other. With an empty
/// salt, anyone who already knows the identifier can recompute it; set a secret salt to prevent that.
#[must_use]
pub fn pseudonym(subject: &str, salt: &str) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(b"ppm-subject-pseudonym-v1\0");
    hasher.update(salt.as_bytes());
    hasher.update(b"\0");
    hasher.update(bare(subject).as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    // Mark it as a version-8 (custom) UUID so it is recognisable and valid.
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

/// Whether a value is already a pseudonym this module produced (version 8, variant 1) in any form.
#[must_use]
pub fn is_pseudonym(value: &str) -> bool {
    Uuid::parse_str(&bare(value)).is_ok_and(|u| u.get_version_num() == 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_form_of_a_reference_names_the_same_person() {
        let id = "3f2c6b7e-0000-4000-8000-00000000000a";
        let expected = vec![
            id.to_string(),
            format!("person:{id}"),
            format!("worker:{id}"),
        ];
        assert_eq!(forms(id), expected);
        assert_eq!(forms(&format!("person:{id}")), expected);
        assert_eq!(forms(&format!(" worker:{id} ")), expected);
        assert!(forms("  ").is_empty());
        assert!(forms("person:").is_empty());
    }

    #[test]
    fn the_pseudonym_is_stable_distinct_and_salted() {
        let a = pseudonym("person:ada", "");
        assert_eq!(a, pseudonym("ada", ""));
        assert_eq!(a, pseudonym("worker:ada", ""));
        assert_ne!(a, pseudonym("bob", ""));
        assert_ne!(a, pseudonym("ada", "secret"));
        assert_eq!(a.get_version_num(), 8);
        assert!(is_pseudonym(&a.to_string()));
        assert!(is_pseudonym(&format!("person:{a}")));
        assert!(!is_pseudonym("3f2c6b7e-0000-4000-8000-00000000000a"));
    }

    #[test]
    fn the_column_list_has_no_duplicates_and_no_overlap_with_the_exclusions() {
        for (i, a) in COLUMNS.iter().enumerate() {
            assert!(!COLUMNS[i + 1..].contains(a), "{a:?} twice");
            assert!(
                !EXCLUDED.iter().any(|(t, c, _)| (*t, *c) == *a),
                "{a:?} is both included and excluded"
            );
        }
    }
}
