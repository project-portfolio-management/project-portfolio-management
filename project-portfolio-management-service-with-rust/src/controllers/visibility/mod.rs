//! PPM Phase-B visibility controllers (spec/15-roadmap PPM-6/7/8/9).
//!
//! - cross-item dependencies + the portfolio schedule view
//!   (violations + critical path)
//! - milestones, resource allocations + the capacity rollup
//! - saved reports, and the ETag-conditional at-a-glance dashboard
//!
//! Pure rules live in [`crate::visibility`]; nothing here feeds the
//! matcher (§8 partition rule).

use loco_rs::prelude::*;
use uuid::Uuid;

use crate::models::_entities::plans;
use crate::visibility as rules;

mod allocations;
mod dashboard;
mod dependencies;
mod milestones;
mod reports;
pub(crate) use allocations::AllocationPayload;
use allocations::{
    capacity, commit_allocation, create_allocation, delete_allocation, list_allocations, skill_gap,
};
use dashboard::at_a_glance;
pub(crate) use dependencies::DependencyPayload;
use dependencies::{create_dependency, delete_dependency, list_dependencies, portfolio_schedule};
pub(crate) use milestones::{MilestonePayload, MilestoneReschedulePayload};
use milestones::{complete_milestone, create_milestone, list_milestones, reschedule_milestone};
pub(crate) use reports::{ReportPayload, execute_report};
use reports::{create_report, delete_report, list_reports, run_report};

/// Find an active plan by pid across **any** collection (edges
/// may span kinds).
async fn find_any_item(ctx: &AppContext, pid: Uuid) -> Result<plans::Model> {
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .ok_or(Error::NotFound)
}

/// The schedule facts (parsed flexible dates) of one stored item.
pub(crate) fn schedule_item(model: &plans::Model) -> rules::ScheduleItem {
    let start = model.data.get("start_date").and_then(|v| v.as_str());
    let end = model.data.get("target_date").and_then(|v| v.as_str());
    rules::ScheduleItem {
        pid: model.pid,
        start: start.and_then(|s| rules::parse_flex_date(s, false)),
        end: end.and_then(|s| rules::parse_flex_date(s, true)),
    }
}

/// Whether the stored item is finished (Completed / Cancelled).
pub(crate) fn is_finished(model: &plans::Model) -> bool {
    matches!(
        model.data.get("status").and_then(|v| v.as_str()),
        Some("Completed" | "Cancelled")
    )
}

/// The visibility routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/dependencies", post(create_dependency))
        .add("/dependencies", get(list_dependencies))
        .add("/dependencies/{pid}", delete(delete_dependency))
        .add("/plans/{pid}/schedule", get(portfolio_schedule))
        .add("/plans/{pid}/milestones", post(create_milestone))
        .add("/plans/{pid}/milestones", get(list_milestones))
        .add("/plans/{pid}/milestones/{m_pid}", put(reschedule_milestone))
        .add(
            "/plans/{pid}/milestones/{m_pid}/complete",
            post(complete_milestone),
        )
        .add("/plans/{pid}/allocations", post(create_allocation))
        .add("/plans/{pid}/allocations", get(list_allocations))
        .add(
            "/plans/{pid}/allocations/{a_pid}/commit",
            post(commit_allocation),
        )
        .add(
            "/plans/{pid}/allocations/{a_pid}",
            delete(delete_allocation),
        )
        .add("/plans/{pid}/skill-gap", get(skill_gap))
        .add("/capacity", get(capacity))
        .add("/reports", post(create_report))
        .add("/reports", get(list_reports))
        .add("/reports/{pid}", delete(delete_report))
        .add("/reports/{pid}/run", get(run_report))
        .add("/at-a-glance", get(at_a_glance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use serde_json::json;

    fn model(data: serde_json::Value) -> plans::Model {
        let at = chrono::DateTime::parse_from_rfc3339("2026-10-01T09:30:00+00:00").unwrap();
        plans::Model {
            created_at: at,
            updated_at: at,
            id: 1,
            pid: Uuid::from_u128(1),
            kind: None,
            name: "Alpha".to_string(),
            data,
            parent_pid: None,
            active: true,
            stage: None,
            phase: None,
            deleted_at: None,
            content_hash: None,
            content_hash_sha3: None,
            content_mac: None,
        }
    }

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn schedule_item_reads_whole_dates() {
        let m = model(json!({"start_date": "2026-03-04", "target_date": "2026-05-06"}));
        let item = schedule_item(&m);
        assert_eq!(item.pid, m.pid);
        assert_eq!(item.start, Some(day(2026, 3, 4)));
        assert_eq!(item.end, Some(day(2026, 5, 6)));
    }

    #[test]
    fn schedule_item_widens_a_month_to_its_first_and_last_day() {
        let m = model(json!({"start_date": "2026-02", "target_date": "2028-02"}));
        let item = schedule_item(&m);
        assert_eq!(item.start, Some(day(2026, 2, 1)));
        assert_eq!(item.end, Some(day(2028, 2, 29)), "2028 is a leap year");
    }

    #[test]
    fn schedule_item_leaves_missing_or_unreadable_dates_unknown() {
        let m = model(json!({"start_date": "soon", "target_date": 5}));
        let item = schedule_item(&m);
        assert_eq!((item.start, item.end), (None, None));
        let none = schedule_item(&model(json!({})));
        assert_eq!((none.start, none.end), (None, None));
    }

    #[test]
    fn only_completed_and_cancelled_are_finished() {
        for (status, finished) in [
            ("Completed", true),
            ("Cancelled", true),
            ("Active", false),
            ("completed", false),
        ] {
            assert_eq!(
                is_finished(&model(json!({ "status": status }))),
                finished,
                "{status}"
            );
        }
        assert!(!is_finished(&model(json!({}))), "no status is not finished");
        assert!(!is_finished(&model(json!({"status": 3}))));
    }
}
