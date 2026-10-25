//! `ActiveModelBehavior` impls for the tables added after the first
//! capability waves (one line each; no custom hooks).

// One line per table; a glob keeps the generator's output trivially mergeable. An `allow`, not an
// `expect`: clippy only reports the glob in the library build, not in the test build.
#![allow(clippy::wildcard_imports)]

use loco_rs::prelude::*;

use super::_entities::*;

impl ActiveModelBehavior for components::ActiveModel {}
impl ActiveModelBehavior for custom_field_defs::ActiveModel {}
impl ActiveModelBehavior for custom_field_values::ActiveModel {}
impl ActiveModelBehavior for watchers::ActiveModel {}
impl ActiveModelBehavior for approval_requests::ActiveModel {}
impl ActiveModelBehavior for task_relations::ActiveModel {}
impl ActiveModelBehavior for feed_tokens::ActiveModel {}
impl ActiveModelBehavior for plan_access_settings::ActiveModel {}
impl ActiveModelBehavior for timesheet_weeks::ActiveModel {}
impl ActiveModelBehavior for plan_templates::ActiveModel {}
impl ActiveModelBehavior for planned_slots::ActiveModel {}
impl ActiveModelBehavior for artifacts::ActiveModel {}
impl ActiveModelBehavior for attachments::ActiveModel {}
impl ActiveModelBehavior for bulk_jobs::ActiveModel {}
impl ActiveModelBehavior for review_queue_items::ActiveModel {}
impl ActiveModelBehavior for guest_grants::ActiveModel {}
impl ActiveModelBehavior for plan_links::ActiveModel {}
impl ActiveModelBehavior for frameworks::ActiveModel {}
impl ActiveModelBehavior for framework_versions::ActiveModel {}
impl ActiveModelBehavior for plan_frameworks::ActiveModel {}
impl ActiveModelBehavior for plan_tolerances::ActiveModel {}
impl ActiveModelBehavior for scoring_inputs::ActiveModel {}
impl ActiveModelBehavior for lessons::ActiveModel {}
impl ActiveModelBehavior for scheduled_job_runs::ActiveModel {}
impl ActiveModelBehavior for plan_status_cadences::ActiveModel {}
impl ActiveModelBehavior for status_updates::ActiveModel {}
impl ActiveModelBehavior for escalations::ActiveModel {}
impl ActiveModelBehavior for escalation_events::ActiveModel {}
impl ActiveModelBehavior for notification_sinks::ActiveModel {}
impl ActiveModelBehavior for sink_deliveries::ActiveModel {}
impl ActiveModelBehavior for attachment_texts::ActiveModel {}
impl ActiveModelBehavior for report_schedules::ActiveModel {}
impl ActiveModelBehavior for work_calendars::ActiveModel {}
impl ActiveModelBehavior for person_availabilities::ActiveModel {}
impl ActiveModelBehavior for schedule_baselines::ActiveModel {}
impl ActiveModelBehavior for schedule_baseline_tasks::ActiveModel {}
impl ActiveModelBehavior for raci_assignments::ActiveModel {}
impl ActiveModelBehavior for earned_value_settings::ActiveModel {}
impl ActiveModelBehavior for earned_value_snapshots::ActiveModel {}
impl ActiveModelBehavior for resource_roles::ActiveModel {}
impl ActiveModelBehavior for kpis::ActiveModel {}
impl ActiveModelBehavior for controlled_documents::ActiveModel {}
impl ActiveModelBehavior for document_versions::ActiveModel {}
impl ActiveModelBehavior for processes::ActiveModel {}
impl ActiveModelBehavior for process_versions::ActiveModel {}
impl ActiveModelBehavior for process_executions::ActiveModel {}
impl ActiveModelBehavior for process_execution_logs::ActiveModel {}
impl ActiveModelBehavior for hammock_links::ActiveModel {}
impl ActiveModelBehavior for access_records::ActiveModel {}
impl ActiveModelBehavior for onboarding_cases::ActiveModel {}
impl ActiveModelBehavior for onboarding_items::ActiveModel {}
impl ActiveModelBehavior for onboarding_templates::ActiveModel {}
impl ActiveModelBehavior for announcements::ActiveModel {}
impl ActiveModelBehavior for issues::ActiveModel {}
impl ActiveModelBehavior for change_requests::ActiveModel {}
impl ActiveModelBehavior for change_request_risks::ActiveModel {}
impl ActiveModelBehavior for plan_handling_marks::ActiveModel {}
impl ActiveModelBehavior for break_glass_grants::ActiveModel {}
impl ActiveModelBehavior for director_packs::ActiveModel {}
impl ActiveModelBehavior for delivery_confidence_assessments::ActiveModel {}
impl ActiveModelBehavior for delivery_exceptions::ActiveModel {}
impl ActiveModelBehavior for strategic_aims::ActiveModel {}
impl ActiveModelBehavior for planned_gates::ActiveModel {}
impl ActiveModelBehavior for funding_sources::ActiveModel {}
impl ActiveModelBehavior for programme_buffers::ActiveModel {}
impl ActiveModelBehavior for funding_allocations::ActiveModel {}
impl ActiveModelBehavior for cost_code_maps::ActiveModel {}
impl ActiveModelBehavior for scorecards::ActiveModel {}
impl ActiveModelBehavior for scorecard_kpis::ActiveModel {}
impl ActiveModelBehavior for escalation_levels::ActiveModel {}
impl ActiveModelBehavior for deescalation_criteria::ActiveModel {}
impl ActiveModelBehavior for criterion_evidence::ActiveModel {}
impl ActiveModelBehavior for cutover_decisions::ActiveModel {}
impl ActiveModelBehavior for cutover_criteria::ActiveModel {}
impl ActiveModelBehavior for cutover_cosignatures::ActiveModel {}
impl ActiveModelBehavior for deployment_moves::ActiveModel {}
impl ActiveModelBehavior for start_queue_entries::ActiveModel {}
impl ActiveModelBehavior for start_overrides::ActiveModel {}
impl ActiveModelBehavior for email_deliveries::ActiveModel {}
impl ActiveModelBehavior for programme_triages::ActiveModel {}
impl ActiveModelBehavior for programme_thresholds::ActiveModel {}
impl ActiveModelBehavior for threshold_decisions::ActiveModel {}
impl ActiveModelBehavior for key_person_entries::ActiveModel {}
impl ActiveModelBehavior for contingent_workers::ActiveModel {}
impl ActiveModelBehavior for programme_profiles::ActiveModel {}
impl ActiveModelBehavior for milestone_signoff_requirements::ActiveModel {}
impl ActiveModelBehavior for milestone_signoffs::ActiveModel {}
impl ActiveModelBehavior for organizations::ActiveModel {}
impl ActiveModelBehavior for portfolio_drum::ActiveModel {}
impl ActiveModelBehavior for buffer_plans::ActiveModel {}
impl ActiveModelBehavior for buffer_snapshots::ActiveModel {}
impl ActiveModelBehavior for buffer_recovery_actions::ActiveModel {}
impl ActiveModelBehavior for task_ccpm_estimates::ActiveModel {}
impl ActiveModelBehavior for benefit_validations::ActiveModel {}
impl ActiveModelBehavior for skill_pools::ActiveModel {}
impl ActiveModelBehavior for director_pack_schedules::ActiveModel {}
impl ActiveModelBehavior for director_pack_subscriptions::ActiveModel {}
impl ActiveModelBehavior for skill_pool_members::ActiveModel {}
impl ActiveModelBehavior for capacity_commitments::ActiveModel {}
impl ActiveModelBehavior for programme_status_decisions::ActiveModel {}
impl ActiveModelBehavior for cost_rate_tables::ActiveModel {}
impl ActiveModelBehavior for cost_rates::ActiveModel {}
impl ActiveModelBehavior for task_costs::ActiveModel {}
impl ActiveModelBehavior for time_entry_costs::ActiveModel {}
impl ActiveModelBehavior for milestone_tasks::ActiveModel {}
impl ActiveModelBehavior for milestone_slips::ActiveModel {}
impl ActiveModelBehavior for kpi_readings::ActiveModel {}
