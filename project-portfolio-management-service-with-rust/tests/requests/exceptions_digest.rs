//! The weekly portfolio-exceptions digest (NC-17): one red tolerance, one open threshold exception and one overdue
//! escalation are each counted and listed, and the weekly send goes once per recipient per week.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::controllers::exceptions_digest::send_weekly;
use project_portfolio_management::models::_entities::{escalations, notifications};
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_digest_with_one_item_of_each_kind_is_built_and_sent_once_a_week() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let empty: Value = request.get("/api/exceptions/digest").await.json();
        assert_eq!(empty["digest"]["counts"]["tolerance"], 0);
        assert!(empty["message"].as_str().unwrap().ends_with("none."));

        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Digest subject" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        // A red tolerance (a 5 x 5 risk against a limit of 10).
        request
            .post(&format!("/api/plans/{p}/risks"))
            .json(&json!({ "title": "Vendor", "probability": 5, "impact": 5 }))
            .await
            .assert_status_ok();
        request
            .put(&format!("/api/plans/{p}/tolerances/risk"))
            .json(&json!({ "limit": 10 }))
            .await
            .assert_status_ok();
        // An overdue escalation.
        // (An answer date in the past is refused when raising, so the date is moved back afterwards.)
        let raised: Value = request
            .post(&format!("/api/plans/{p}/escalations"))
            .json(&json!({ "title": "Blocked on supplier", "respond_by": "2099-01-01" }))
            .await
            .json();
        let row = sqlx::query_as::<_, escalations::Model>("SELECT * FROM escalations WHERE pid = $1 LIMIT 1").bind(uuid::Uuid::parse_str(raised["pid"].as_str().unwrap()).unwrap()).fetch_optional(project_portfolio_management::db::pool(&ctx))
            .await
            .unwrap()
            .unwrap();
        sqlx::query("UPDATE escalations SET respond_by = $1 WHERE id = $2 RETURNING id")
            .bind(chrono::NaiveDate::from_ymd_opt(2020, 1, 1))
            .bind(row.id)
            .fetch_one(project_portfolio_management::db::pool(&ctx))
            .await
            .expect("move the date back");
        // An open exception opened by a threshold.
        let opened_at: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        sqlx::query(
            "INSERT INTO delivery_exceptions (pid, plan_pid, status, opened_at, opened_value, worst_value, worsened_count, threshold_kind) \
             VALUES ($1, $2, 'open', $3, 'threshold', 'threshold', 0, 'pause') RETURNING id",
        )
        .bind(uuid::Uuid::new_v4())
        .bind(uuid::Uuid::parse_str(&p).unwrap())
        .bind(opened_at)
        .fetch_one(project_portfolio_management::db::pool(&ctx))
        .await
        .expect("exception");

        let body: Value = request.get("/api/exceptions/digest").await.json();
        let counts = &body["digest"]["counts"];
        assert_eq!(
            (
                counts["tolerance"].as_u64(),
                counts["threshold"].as_u64(),
                counts["escalation"].as_u64()
            ),
            (Some(1), Some(1), Some(1)),
            "{body}"
        );
        let items = body["digest"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items.iter().all(|i| i["plan_name"] == "Digest subject"));
        assert!(items[0]["severity"].as_u64().unwrap() <= items[2]["severity"].as_u64().unwrap());
        assert!(
            body["message"]
                .as_str()
                .unwrap()
                .contains("1 tolerance, 1 threshold, 1 escalation")
        );

        // Sent once per recipient per week; a reference that is not a person is not sent to.
        let who = format!("person:{}", uuid::Uuid::new_v4());
        let recipients = vec![who.clone(), "organization:acme".to_string()];
        let first = send_weekly(&ctx, &recipients).await.expect("send");
        assert_eq!((first.sent, first.already, first.unaddressable), (1, 0, 1));
        let again = send_weekly(&ctx, &recipients).await.expect("send");
        assert_eq!((again.sent, again.already, again.unaddressable), (0, 1, 1));
        let stored = sqlx::query_as::<_, notifications::Model>("SELECT * FROM notifications WHERE recipient_ref = $1 AND kind = $2").bind(who.as_str()).bind("exceptions_digest").fetch_all(project_portfolio_management::db::pool(&ctx))
            .await
            .expect("notifications");
        assert_eq!(stored.len(), 1);
        assert!(stored[0].message.contains("Digest subject"));
    })
    .await;
}
