//! Tolerances: limits per dimension, inheritance down the plan tree, and the red/amber/green view.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

fn dim<'a>(picture: &'a Value, name: &str) -> &'a Value {
    picture["dimensions"]
        .as_array()
        .expect("dimensions")
        .iter()
        .find(|d| d["dimension"] == name)
        .expect("a dimension")
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_project_has_tolerances_it_sets_or_inherits_shown_as_red_amber_green() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let make_plan =
            |name: &'static str, parent: Option<String>, target: Option<&'static str>| {
                let request = &request;
                async move {
                    let mut body = json!({ "kind": "Project", "name": name });
                    if let Some(p) = parent {
                        body["parent_ref"] = json!(p);
                    }
                    if let Some(t) = target {
                        body["target_date"] = json!(t);
                    }
                    let plan: Value = request.post("/api/plans").json(&body).await.json();
                    plan["pid"].as_str().expect("pid").to_string()
                }
            };
        let programme = make_plan("Tol programme", None, None).await;
        let project = make_plan("Tol project", Some(programme.clone()), Some("2026-11-02")).await;
        let url = format!("/api/plans/{project}/tolerances");
        let picture = || {
            let (request, url) = (&request, url.clone());
            async move { request.get(&url).await.json::<Value>() }
        };
        let set = |plan: &str, d: &str, limit: i64| {
            let (request, path) = (&request, format!("/api/plans/{plan}/tolerances/{d}"));
            async move { request.put(&path).json(&json!({ "limit": limit })).await }
        };

        // Nothing set anywhere: not_set, and no measurement is attempted.
        let none = picture().await;
        assert_eq!(none["overall"], "not_set");
        assert_eq!(dim(&none, "time")["status"], "not_set");

        // The programme sets a time limit; the project inherits it. The project has tasks that finish late.
        set(&programme, "time", 30).await.assert_status_ok();
        let task: Value = request
            .post(&format!("/api/plans/{project}/tasks"))
            .json(&json!({ "title": "Build", "start_date": "2026-11-09", "duration_days": 10 }))
            .await
            .json();
        assert!(task["pid"].is_string());
        let inherited = picture().await;
        let time = dim(&inherited, "time");
        assert_eq!(time["source"], "inherited");
        assert_eq!(time["from_plan_pid"], programme.as_str());
        let slip = time["measured"].as_i64().expect("the slip is measured");
        assert!(
            slip >= 7,
            "the work starts a week after the target and takes longer: {slip}"
        );

        // The project's own limit wins, and moves the colour as the limit moves around the slip.
        for (limit, want) in [
            (slip * 2, "green"),
            (slip, "amber"),
            (slip - 1, "red"),
            (0, "red"),
        ] {
            set(&project, "time", limit).await.assert_status_ok();
            assert_eq!(
                dim(&picture().await, "time")["status"],
                want,
                "limit {limit}, slip {slip}"
            );
        }
        assert_eq!(dim(&picture().await, "time")["source"], "own");

        // Cost with no baseline cannot be measured: unknown, with the reason, and never green.
        set(&project, "cost", 500).await.assert_status_ok();
        let cost = picture().await;
        assert_eq!(dim(&cost, "cost")["status"], "unknown");
        assert!(
            dim(&cost, "cost")["reason"]
                .as_str()
                .expect("reason")
                .contains("baseline")
        );

        // Risk: nothing recorded is unknown; a score of 20 against 25 is amber, against 15 red.
        set(&project, "risk", 25).await.assert_status_ok();
        assert_eq!(dim(&picture().await, "risk")["status"], "unknown");
        request
            .post(&format!("/api/plans/{project}/risks"))
            .json(&json!({ "title": "Vendor slips", "probability": 4, "impact": 5 }))
            .await
            .assert_status_ok();
        let risk = picture().await;
        assert_eq!(dim(&risk, "risk")["measured"], 20);
        assert_eq!(dim(&risk, "risk")["status"], "amber");
        set(&project, "risk", 15).await.assert_status_ok();
        assert_eq!(dim(&picture().await, "risk")["status"], "red");

        // Resources: one person booked 80% here and 50% on another plan is 30 points over.
        let other = make_plan("Tol other", None, None).await;
        let person = format!("person:{}", uuid::Uuid::new_v4());
        for (plan, percent) in [(&project, 80), (&other, 50)] {
            request
                .post(&format!("/api/plans/{plan}/allocations"))
                .json(&json!({ "person_ref": person, "percent": percent }))
                .await
                .assert_status_ok();
        }
        set(&project, "resources", 40).await.assert_status_ok();
        let res = picture().await;
        assert_eq!(dim(&res, "resources")["measured"], 30);
        assert_eq!(
            dim(&res, "resources")["status"],
            "amber",
            "30 of 40 points is 75 percent"
        );
        set(&project, "resources", 20).await.assert_status_ok();
        assert_eq!(dim(&picture().await, "resources")["status"], "red");

        // The plan is as bad as its worst dimension; the board lists it, red first.
        assert_eq!(picture().await["overall"], "red");
        let board: Value = request.get("/api/tolerances").await.json();
        let first = &board["plans"][0];
        assert_eq!(first["overall"], "red");
        let listed: Vec<&str> = board["plans"]
            .as_array()
            .expect("plans")
            .iter()
            .filter_map(|p| p["plan_pid"].as_str())
            .collect();
        assert!(listed.contains(&project.as_str()));
        let only_red: Value = request
            .get("/api/tolerances?status=red&dimension=risk")
            .await
            .json();
        assert!(
            only_red["plans"]
                .as_array()
                .expect("plans")
                .iter()
                .all(|p| p["overall"] == "red")
        );
        assert_eq!(
            request
                .get("/api/tolerances?status=purple")
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .get("/api/tolerances?dimension=scope")
                .await
                .status_code(),
            422
        );

        // Refusals, and removing the project's own limit hands the dimension back to the programme.
        for (path, body, why) in [
            ("scope", json!({ "limit": 1 }), "an unknown dimension"),
            ("risk", json!({ "limit": 0 }), "a risk score starts at 1"),
            ("time", json!({ "limit": 3651 }), "too large"),
            ("cost", json!({ "limit": -1 }), "negative"),
            (
                "time",
                json!({ "limit": 5, "amber_from_percent": 100 }),
                "amber line out of range",
            ),
        ] {
            assert_eq!(
                request
                    .put(&format!("{url}/{path}"))
                    .json(&body)
                    .await
                    .status_code(),
                422,
                "{why}"
            );
        }
        request
            .post(&format!("{url}/time/delete"))
            .await
            .assert_status_ok();
        assert_eq!(dim(&picture().await, "time")["source"], "inherited");
        assert_eq!(
            request
                .post(&format!("{url}/time/delete"))
                .await
                .status_code(),
            404,
            "it set none of its own"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn schedule_slip_is_held_against_the_latest_schedule_baseline() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Baselined" })).await.json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let a: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({ "title": "A", "start_date": "2026-11-02", "duration_days": 5 })).await.json();
        let b: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({ "title": "B", "duration_days": 5 })).await.json();
        request
            .post("/api/dependencies")
            .json(&json!({ "predecessor_pid": a["pid"], "successor_pid": b["pid"] }))
            .await
            .assert_status_ok();
        let picture = || async { request.get(&format!("/api/plans/{p}/tolerances")).await.json::<Value>() };

        // A limit with no baseline is unknown, never green, and says why.
        request.put(&format!("/api/plans/{p}/tolerances/schedule")).json(&json!({ "limit": 5 })).await.assert_status_ok();
        let before = picture().await;
        let s = dim(&before, "schedule");
        assert_eq!(s["status"], "unknown");
        assert!(s["reason"].as_str().is_some_and(|r| r.contains("no schedule baseline")));

        // Baseline the plan, then make the work longer: the forecast finish moves past the baselined one.
        request.post(&format!("/api/plans/{p}/schedule-baselines")).json(&json!({ "reason": "kickoff" })).await.assert_status_ok();
        let on_time = picture().await;
        assert_eq!(dim(&on_time, "schedule")["measured"], 0);
        assert_eq!(dim(&on_time, "schedule")["status"], "green");

        request
            .put(&format!("/api/plans/{p}/tasks/{}/schedule", b["pid"].as_str().expect("pid")))
            .json(&json!({ "duration_days": 15 }))
            .await
            .assert_status_ok();
        let late = picture().await;
        let s = dim(&late, "schedule");
        assert!(s["measured"].as_i64().is_some_and(|d| d >= 10), "ten more business days: {s}");
        assert_eq!(s["status"], "red");
        assert_eq!(late["escalation_needed"], true);

        // The escalation can cite the dimension.
        request
            .post(&format!("/api/plans/{p}/escalations"))
            .json(&json!({ "title": "Slipped past the baseline", "reason_kind": "tolerance", "tolerance_dimension": "schedule" }))
            .await
            .assert_status_ok();
    })
    .await;
}
