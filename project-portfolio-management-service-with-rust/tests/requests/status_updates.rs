//! Status updates on a cadence (NC-4): posting, history, due and overdue, the once-per-due-date reminder, and the
//! director pack's view as at its cut-off.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use std::collections::HashSet;

use axum::http::StatusCode;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::controllers::status_updates::{pack_section, remind_due};
use project_portfolio_management::models::_entities::{notifications, plan_status_cadences};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn updates_are_due_then_overdue_then_reminded_once_and_the_pack_reads_them_as_at_its_cut_off()
{
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Cadenced" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let plan_uuid = uuid::Uuid::parse_str(&p).unwrap();

        // No cadence: no due date, and the cadence itself is a 404.
        let none: Value = request.get(&format!("/api/plans/{p}/status")).await.json();
        assert!(
            none["cadence_days"].is_null()
                && none["standing"].is_null()
                && none["latest"].is_null()
        );
        request
            .get(&format!("/api/plans/{p}/status-cadence"))
            .await
            .assert_status_not_found();
        for bad in [0, 93] {
            request
                .put(&format!("/api/plans/{p}/status-cadence"))
                .json(&json!({ "cadence_days": bad }))
                .await
                .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // A weekly cadence just set is not due.
        let set: Value = request
            .put(&format!("/api/plans/{p}/status-cadence"))
            .json(&json!({ "cadence_days": 7 }))
            .await
            .json();
        assert_eq!(
            (
                set["standing"]["due"].as_bool(),
                set["standing"]["overdue"].as_bool()
            ),
            (Some(false), Some(false))
        );

        // Ten days on it is three days overdue: the cadence was set ten days ago, and nobody has posted. The reminder
        // goes to whoever set it (here a person set directly, since this test is unauthenticated).
        let who = uuid::Uuid::new_v4().to_string();
        let row = plan_status_cadences::Entity::find()
            .filter(plan_status_cadences::Column::PlanPid.eq(plan_uuid))
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        let mut a: plan_status_cadences::ActiveModel = row.into();
        a.set_on = ActiveValue::set(
            project_portfolio_management::tz::deployment_today() - chrono::Duration::days(10),
        );
        a.set_by = ActiveValue::set(Some(who.clone()));
        a.update(&ctx.db).await.unwrap();
        let status: Value = request.get(&format!("/api/plans/{p}/status")).await.json();
        assert_eq!(status["standing"]["overdue"], true, "{status}");
        assert_eq!(status["standing"]["days_overdue"], 3);
        let due: Value = request.get("/api/status-updates/due").await.json();
        assert!(
            due.as_array()
                .unwrap()
                .iter()
                .any(|d| d["plan_pid"] == p.as_str() && d["standing"]["days_overdue"] == 3)
        );

        // The reminder goes out once for this due date, not again.
        let first = remind_due(&ctx).await.unwrap();
        assert!(first.sent >= 1, "{first:?}");
        let again = remind_due(&ctx).await.unwrap();
        assert_eq!((again.sent, again.unaddressed), (0, 0));
        let told = notifications::Entity::find()
            .filter(notifications::Column::RecipientRef.eq(format!("person:{who}")))
            .filter(notifications::Column::Kind.eq("status_update_due"))
            .all(&ctx.db)
            .await
            .unwrap();
        assert_eq!(told.len(), 1);
        assert!(told[0].message.contains("3 day(s) overdue"));

        // Posting restarts the clock and is refused when it says nothing.
        for bad in [
            json!({ "status": "green", "narrative": "x" }),
            json!({ "status": "at_risk", "narrative": " " }),
        ] {
            request
                .post(&format!("/api/plans/{p}/status-updates"))
                .json(&bad)
                .await
                .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        }
        // An hour either side: an update's time is the database's clock, the cut-off's is the caller's, and on a
        // developer machine with a containerised database the two can differ by more than a test should assume.
        let before_post = chrono::Utc::now() - chrono::Duration::hours(1);
        let posted: Value = request
            .post(&format!("/api/plans/{p}/status-updates"))
            .json(&json!({ "status": "at_risk", "narrative": "Supplier slipped a fortnight" }))
            .await
            .json();
        assert_eq!(posted["status"], "at_risk");
        let status: Value = request.get(&format!("/api/plans/{p}/status")).await.json();
        assert_eq!(
            status["latest"]["narrative"],
            "Supplier slipped a fortnight"
        );
        assert_eq!(
            (
                status["standing"]["due"].as_bool(),
                status["standing"]["overdue"].as_bool()
            ),
            (Some(false), Some(false))
        );
        request
            .post(&format!("/api/plans/{p}/status-updates"))
            .json(&json!({ "status": "on_track", "narrative": "Recovered" }))
            .await
            .assert_status_ok();
        let history: Value = request
            .get(&format!("/api/plans/{p}/status-updates"))
            .await
            .json();
        let history = history.as_array().unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0]["status"], "on_track", "newest first");

        // The pack reads the latest update posted on or before its cut-off.
        let include: HashSet<uuid::Uuid> = [plan_uuid].into_iter().collect();
        let early = pack_section(&ctx, &include, before_post).await.unwrap();
        assert!(
            early["plans"][0]["latest"].is_null(),
            "nothing had been posted at that cut-off: {early}"
        );
        // `created_at` is the database's clock, which can run a fraction of a second ahead of this process's: a cut-off
        // of exactly now can fall before a row posted a moment ago.
        let late = pack_section(
            &ctx,
            &include,
            chrono::Utc::now() + chrono::Duration::minutes(1),
        )
        .await
        .unwrap();
        assert_eq!(late["plans"][0]["latest"]["status"], "on_track");
        assert_eq!(late["without_cadence"], 0);
    })
    .await;
}
