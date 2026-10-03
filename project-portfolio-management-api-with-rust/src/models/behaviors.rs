//! `ActiveModelBehavior` impls for the tables added after the first
//! capability waves (one line each; no custom hooks).

// One line per table; a glob keeps the generator's output trivially mergeable.
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
impl ActiveModelBehavior for notification_sinks::ActiveModel {}
impl ActiveModelBehavior for sink_deliveries::ActiveModel {}
impl ActiveModelBehavior for attachment_texts::ActiveModel {}
impl ActiveModelBehavior for report_schedules::ActiveModel {}
impl ActiveModelBehavior for work_calendars::ActiveModel {}
impl ActiveModelBehavior for person_availabilities::ActiveModel {}
impl ActiveModelBehavior for schedule_baselines::ActiveModel {}
impl ActiveModelBehavior for schedule_baseline_tasks::ActiveModel {}
