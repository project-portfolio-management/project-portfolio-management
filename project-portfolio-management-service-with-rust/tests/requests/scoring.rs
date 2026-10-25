//! WSJF and RICE (NC-6): the inputs are set per plan, validated and audited; each model scores from its inputs
//! alone; a plan missing an input is unscored with the missing input named, never scored as zero; and the
//! ranking by each model through `GET /api/prioritization?model=`.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn plans_are_ranked_by_wsjf_and_rice_from_their_estimated_inputs() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let make = |name: &'static str| {
            let request = &request;
            async move {
                let p: Value = request
                    .post("/api/plans")
                    .json(&json!({ "kind": "Project", "name": name }))
                    .await
                    .json();
                p["pid"].as_str().unwrap().to_string()
            }
        };
        let (big, quick, blank) = (make("Big bet").await, make("Quick win").await, make("Unestimated").await);
        let put = |pid: &str, body: Value| {
            let (request, url) = (&request, format!("/api/plans/{pid}/scoring-inputs"));
            async move { request.put(&url).json(&body).await }
        };

        // Nothing is set yet: both models say so and name what they wait for.
        let none: Value = request.get(&format!("/api/plans/{blank}/scoring-inputs")).await.json();
        assert_eq!(none["set"], false);
        let s: Value = request.get(&format!("/api/plans/{blank}/score?model=wsjf")).await.json();
        assert_eq!(s["score"]["score"], Value::Null);
        let missing: Vec<&str> = s["score"]["missing"].as_array().unwrap().iter().map(|m| m["input"].as_str().unwrap()).collect();
        assert_eq!(missing, ["business_value", "time_criticality", "risk_reduction", "job_size"]);
        assert!(s["score"]["missing"][0]["reason"].as_str().unwrap().len() > 5);

        // Bad values are refused, every one named; nothing is stored.
        let r = put(&big, json!({ "business_value": 0, "impact": 1.5, "effort": 0 })).await;
        assert_eq!(r.status_code(), 422, "{}", r.text());
        assert!(r.text().contains("business_value") && r.text().contains("impact") && r.text().contains("effort"), "{}", r.text());
        assert_eq!(request.get(&format!("/api/plans/{big}/scoring-inputs")).await.json::<Value>()["set"], false);

        // Two estimated plans.
        put(&big, json!({ "business_value": 13, "time_criticality": 8, "risk_reduction": 5, "job_size": 13,
                          "reach": 10000, "impact": 3, "confidence_percent": 50, "effort": 20 })).await.assert_status_ok();
        put(&quick, json!({ "business_value": 5, "time_criticality": 8, "risk_reduction": 3, "job_size": 2,
                            "reach": 500, "impact": 1, "confidence_percent": 100, "effort": 1 })).await.assert_status_ok();

        // WSJF: (13+8+5)/13 = 2 for the big bet, (5+8+3)/2 = 8 for the quick win.
        let w: Value = request.get(&format!("/api/plans/{big}/score?model=wsjf")).await.json();
        assert_eq!(w["score"]["score"], 2.0);
        assert!(w["score"]["formula"].as_str().is_some_and(|f| f.contains("job size")));
        let q: Value = request.get(&format!("/api/plans/{quick}/score?model=wsjf")).await.json();
        assert_eq!(q["score"]["score"], 8.0);
        // RICE: 10000 x 3 x 0.5 / 20 = 750 for the big bet; 500 x 1 x 1 / 1 = 500 for the quick win.
        let r1: Value = request.get(&format!("/api/plans/{big}/score?model=rice")).await.json();
        assert_eq!(r1["score"]["score"], 750.0);
        let r2: Value = request.get(&format!("/api/plans/{quick}/score?model=rice")).await.json();
        assert_eq!(r2["score"]["score"], 500.0);

        // The two models rank them in opposite orders; the unestimated plan is last in both, unscored.
        for (model, first, second) in [("wsjf", &quick, &big), ("rice", &big, &quick)] {
            let rank: Value = request.get(&format!("/api/prioritization?model={model}")).await.json();
            assert_eq!(rank["model"], model);
            let rows = rank["plans"].as_array().unwrap();
            let pos = |pid: &String| rows.iter().position(|r| r["pid"] == pid.as_str()).unwrap();
            assert!(pos(first) < pos(second), "{model}: {rank}");
            assert!(pos(second) < pos(&blank), "{model}: the estimated plans come before the unestimated");
            let row = &rows[pos(&blank)];
            assert_eq!(row["score"], Value::Null, "{model}: unscored, not zero");
            assert!(!row["missing_inputs"].as_array().unwrap().is_empty());
        }

        // Replacing the inputs with fewer makes the score null again: absent is not zero.
        put(&quick, json!({ "business_value": 5, "time_criticality": 8, "risk_reduction": 3 })).await.assert_status_ok();
        let q2: Value = request.get(&format!("/api/plans/{quick}/score?model=wsjf")).await.json();
        assert_eq!(q2["score"]["score"], Value::Null);
        assert_eq!(q2["score"]["missing"][0]["input"], "job_size");

        // An unknown model is refused; so is a band for a model that has none.
        assert_eq!(request.get(&format!("/api/plans/{big}/score?model=moscow")).await.status_code(), 422);
        assert_eq!(request.get("/api/prioritization?model=moscow").await.status_code(), 422);
        assert_eq!(request.get("/api/prioritization?model=rice&band=high").await.status_code(), 422);
        // The default is still the Smart Score.
        assert!(request.get("/api/prioritization").await.json::<Value>().get("model").is_none());

        // Setting inputs is audited with the before and after.
        let row = ctx.db.query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT snapshot::text AS s FROM audit_logs WHERE action = 'scoring_inputs_set' ORDER BY id DESC LIMIT 1",
        )).await.unwrap().expect("an audit row");
        let snap: Value = serde_json::from_str(&row.try_get::<String>("", "s").unwrap()).unwrap();
        assert_eq!(snap["before"]["job_size"], 2);
        assert_eq!(snap["after"]["job_size"], Value::Null);
    })
    .await;
}
