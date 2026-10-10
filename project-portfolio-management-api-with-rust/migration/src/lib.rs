//! `sea-orm-migration` schema migrations for `project-portfolio-management-service`.
//!
//! The [`Migrator`] runs the ordered list below at boot (or via
//! `cargo loco db migrate`): the `plans` table (the registry across
//! the four collections), then the `audit_logs` and `merge_records` side
//! tables. The audit trail these create is the who/what/when record over
//! every change (lead / assignee / member references are personal data).

// SEC-I3: migrators run pure SQL orchestration; forbid unsafe.
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(elided_lifetimes_in_paths)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_plans;
mod m20220101_000002_audit_logs;
mod m20220101_000003_merge_records;
mod m20220101_000004_event_outbox;
mod m20220101_000005_governance;
mod m20220101_000006_visibility;
mod m20220101_000007_strategy;
mod m20260719_000002_insight_columns;
mod m20260719_000003_insight_snapshots;
mod m20260720_000001_engineering;
mod m20260720_000002_engineering_moderate;
mod m20260722_000001_capabilities;
mod m20260728_000001_integrity_digests;
mod m20260823_000001_time_based_analysis;
mod m20260825_000001_total_project_control;
mod m20260825_000002_controls;
mod m20260825_000003_phase_transitions;
mod m20260825_000004_flow_type;
mod m20260825_000005_workflows;
mod m20260825_000006_key_results;
mod m20260826_000001_effort;
mod m20260826_000002_ceremonies;
mod m20260826_000003_value;
mod m20260902_000001_automation_multi_action;
mod m20260902_000002_automation_milestone_fires;
mod m20260908_000001_webhook_deliveries;
mod m20260918_000001_budget_baselines;
mod m20260918_000002_allocation_skills;
mod m20260918_000003_saved_views;
mod m20260920_000001_report_group_by;
mod m20261002_000001_canvases;
mod m20261002_000002_comments_reactions;
mod m20261002_000003_task_due_date;
mod m20261002_000004_notification_prefs;
mod m20261003_000001_plan_modules;
mod m20261003_000002_components;
mod m20261003_000003_custom_field_defs;
mod m20261003_000004_custom_field_values;
mod m20261003_000005_workflow_transition_roles;
mod m20261003_000006_watchers;
mod m20261003_000007_approval_requests;
mod m20261003_000008_task_relations;
mod m20261003_000009_task_parent;
mod m20261003_000010_feed_tokens;
mod m20261003_000011_plan_access_settings;
mod m20261003_000012_timesheet_weeks;
mod m20261003_000013_plan_templates;
mod m20261003_000014_planned_slots;
mod m20261003_000015_artifacts;
mod m20261003_000016_attachments;
mod m20261003_000017_bulk_jobs;
mod m20261003_000018_review_queue_items;
mod m20261003_000019_guest_grants;
mod m20261003_000020_plan_links;
mod m20261003_000021_notification_sinks;
mod m20261003_000022_sink_deliveries;
mod m20261003_000023_attachment_texts;
mod m20261003_000024_report_schedules;
mod m20261003_000025_schedule_fields;
mod m20261003_000026_work_calendars;
mod m20261003_000027_task_external_ref;
mod m20261003_000028_person_availabilities;
mod m20261003_000029_allocation_booking;
mod m20261003_000030_schedule_baselines;
mod m20261003_000031_schedule_baseline_tasks;
mod m20261003_000032_raci_assignments;
mod m20261003_000033_earned_value;
mod m20261003_000034_earned_value_snapshots;
mod m20261004_000035_resource_roles;
mod m20261004_000036_schedule_risk_inputs;
mod m20261005_000037_kpis;
mod m20261005_000038_milestone_links;
mod m20261005_000039_cost_risk_inputs;
mod m20261006_000040_assignment_fields;
mod m20261006_000041_cost_rates;
mod m20261006_000042_announcements;
mod m20261006_000043_onboarding;
mod m20261006_000044_hammocks;
mod m20261006_000045_job_progress_cancel;
mod m20261006_000046_processes;
mod m20261006_000047_controlled_documents;
mod m20261006_000048_onboarding_one_open;
mod m20261009_000049_oxford_spelling;

/// The crate's migrator: drives the ordered migration set for the loco
/// CLI / boot-time migration.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// The ordered migration set. Order matters: `plans` is created
    /// first because the side tables reference its `pid`, then
    /// `audit_logs` and `merge_records`.
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_plans::Migration),
            Box::new(m20220101_000002_audit_logs::Migration),
            Box::new(m20220101_000003_merge_records::Migration),
            Box::new(m20220101_000004_event_outbox::Migration),
            Box::new(m20220101_000005_governance::Migration),
            Box::new(m20220101_000006_visibility::Migration),
            Box::new(m20220101_000007_strategy::Migration),
            Box::new(m20260719_000002_insight_columns::Migration),
            Box::new(m20260719_000003_insight_snapshots::Migration),
            Box::new(m20260720_000001_engineering::Migration),
            Box::new(m20260720_000002_engineering_moderate::Migration),
            Box::new(m20260722_000001_capabilities::Migration),
            Box::new(m20260728_000001_integrity_digests::Migration),
            Box::new(m20260823_000001_time_based_analysis::Migration),
            Box::new(m20260825_000001_total_project_control::Migration),
            Box::new(m20260825_000002_controls::Migration),
            Box::new(m20260825_000003_phase_transitions::Migration),
            Box::new(m20260825_000004_flow_type::Migration),
            Box::new(m20260825_000005_workflows::Migration),
            Box::new(m20260825_000006_key_results::Migration),
            Box::new(m20260826_000001_effort::Migration),
            Box::new(m20260826_000002_ceremonies::Migration),
            Box::new(m20260826_000003_value::Migration),
            Box::new(m20260902_000001_automation_multi_action::Migration),
            Box::new(m20260902_000002_automation_milestone_fires::Migration),
            Box::new(m20260908_000001_webhook_deliveries::Migration),
            Box::new(m20260918_000001_budget_baselines::Migration),
            Box::new(m20260918_000002_allocation_skills::Migration),
            Box::new(m20260918_000003_saved_views::Migration),
            Box::new(m20260920_000001_report_group_by::Migration),
            Box::new(m20261002_000001_canvases::Migration),
            Box::new(m20261002_000002_comments_reactions::Migration),
            Box::new(m20261002_000003_task_due_date::Migration),
            Box::new(m20261002_000004_notification_prefs::Migration),
            Box::new(m20261003_000001_plan_modules::Migration),
            Box::new(m20261003_000002_components::Migration),
            Box::new(m20261003_000003_custom_field_defs::Migration),
            Box::new(m20261003_000004_custom_field_values::Migration),
            Box::new(m20261003_000005_workflow_transition_roles::Migration),
            Box::new(m20261003_000006_watchers::Migration),
            Box::new(m20261003_000007_approval_requests::Migration),
            Box::new(m20261003_000008_task_relations::Migration),
            Box::new(m20261003_000009_task_parent::Migration),
            Box::new(m20261003_000010_feed_tokens::Migration),
            Box::new(m20261003_000011_plan_access_settings::Migration),
            Box::new(m20261003_000012_timesheet_weeks::Migration),
            Box::new(m20261003_000013_plan_templates::Migration),
            Box::new(m20261003_000014_planned_slots::Migration),
            Box::new(m20261003_000015_artifacts::Migration),
            Box::new(m20261003_000016_attachments::Migration),
            Box::new(m20261003_000017_bulk_jobs::Migration),
            Box::new(m20261003_000018_review_queue_items::Migration),
            Box::new(m20261003_000019_guest_grants::Migration),
            Box::new(m20261003_000020_plan_links::Migration),
            Box::new(m20261003_000021_notification_sinks::Migration),
            Box::new(m20261003_000022_sink_deliveries::Migration),
            Box::new(m20261003_000023_attachment_texts::Migration),
            Box::new(m20261003_000024_report_schedules::Migration),
            Box::new(m20261003_000025_schedule_fields::Migration),
            Box::new(m20261003_000026_work_calendars::Migration),
            Box::new(m20261003_000027_task_external_ref::Migration),
            Box::new(m20261003_000028_person_availabilities::Migration),
            Box::new(m20261003_000029_allocation_booking::Migration),
            Box::new(m20261003_000030_schedule_baselines::Migration),
            Box::new(m20261003_000031_schedule_baseline_tasks::Migration),
            Box::new(m20261003_000032_raci_assignments::Migration),
            Box::new(m20261003_000033_earned_value::Migration),
            Box::new(m20261003_000034_earned_value_snapshots::Migration),
            Box::new(m20261004_000035_resource_roles::Migration),
            Box::new(m20261004_000036_schedule_risk_inputs::Migration),
            Box::new(m20261005_000037_kpis::Migration),
            Box::new(m20261005_000038_milestone_links::Migration),
            Box::new(m20261005_000039_cost_risk_inputs::Migration),
            Box::new(m20261006_000040_assignment_fields::Migration),
            Box::new(m20261006_000041_cost_rates::Migration),
            Box::new(m20261006_000042_announcements::Migration),
            Box::new(m20261006_000043_onboarding::Migration),
            Box::new(m20261006_000044_hammocks::Migration),
            Box::new(m20261006_000045_job_progress_cancel::Migration),
            Box::new(m20261006_000046_processes::Migration),
            Box::new(m20261006_000047_controlled_documents::Migration),
            Box::new(m20261006_000048_onboarding_one_open::Migration),
            Box::new(m20261009_000049_oxford_spelling::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
