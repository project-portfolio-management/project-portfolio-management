//! Critical-path request tests (SC-1, PJ-1, PJ-8): the textbook network end to
//! end, relationship kinds and leads, cycle refusal across dependencies and task
//! relations, unscheduled tasks listed with a reason, and "what moved".
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn task(
    request: &axum_test::TestServer,
    p: &str,
    title: &str,
    start: Option<&str>,
    days: Option<i64>,
) -> String {
    let mut body = json!({ "title": title });
    if let Some(s) = start {
        body["start_date"] = json!(s);
    }
    if let Some(d) = days {
        body["duration_days"] = json!(d);
    }
    let t: Value = request
        .post(&format!("/api/plans/{p}/tasks"))
        .json(&body)
        .await
        .json();
    t["pid"].as_str().expect("task pid").to_string()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn float_critical_chains_cycles_unscheduled_and_what_moved() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let a = task(&request, &p, "A", Some("2026-10-05"), Some(3)).await;
        let b = task(&request, &p, "B", None, Some(4)).await;
        let c = task(&request, &p, "C", None, Some(2)).await;
        let d = task(&request, &p, "D", None, Some(5)).await;
        let loose = task(&request, &p, "Loose", None, None).await;
        let dep = |from: &str, to: &str, extra: Value| {
            let mut body = json!({ "predecessor_pid": from, "successor_pid": to });
            for (k, v) in extra.as_object().cloned().unwrap_or_default() {
                body[k] = v;
            }
            let request = &request;
            async move { request.post("/api/dependencies").json(&body).await }
        };
        for (f, t) in [(&a, &b), (&a, &c), (&b, &d), (&c, &d)] {
            dep(f, t, json!({})).await.assert_status_ok();
        }

        // Refusals: unknown kind, absurd lag, unknown endpoint, a cycle.
        assert_eq!(dep(&d, &a, json!({})).await.status_code(), 422, "cycle");
        assert_eq!(
            dep(&a, &d, json!({ "kind": "xx" })).await.status_code(),
            422
        );
        assert_eq!(
            dep(&a, &d, json!({ "lag_days": 4000 })).await.status_code(),
            422
        );
        assert_eq!(
            dep("3f2c6b7e-0000-4000-8000-000000000bad", &d, json!({}))
                .await
                .status_code(),
            404
        );

        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let node = |pid: &str| {
            cpm["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|n| n["pid"] == pid)
                .cloned()
                .unwrap()
        };
        assert_eq!(
            (
                node(&a)["total_float"].as_i64(),
                node(&b)["total_float"].as_i64(),
                node(&c)["total_float"].as_i64(),
                node(&d)["total_float"].as_i64()
            ),
            (Some(0), Some(0), Some(2), Some(0)),
            "{cpm}"
        );
        assert_eq!(node(&c)["free_float"], 2);
        assert_eq!(node(&a)["early_start"], "2026-10-05");
        assert_eq!(
            cpm["critical_chains"],
            json!([[a.clone(), b.clone(), d.clone()]])
        );
        assert_eq!(
            cpm["project_finish"], "2026-10-16",
            "12 days from the 5th, seven-day calendar"
        );
        // A task with no duration is listed with why, not given a float of zero.
        assert_eq!(cpm["unscheduled"][0]["pid"], loose.as_str());
        assert_eq!(cpm["unscheduled"][0]["reason"], "no_duration");
        assert!(
            cpm["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .all(|n| n["pid"] != loose.as_str())
        );
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/cpm?near=-1"))
                .await
                .status_code(),
            422
        );

        // What moved: growing B by 2 moves B's finish and D entirely, not A or C.
        let r: Value = request
            .put(&format!("/api/plans/{p}/tasks/{b}/schedule"))
            .json(&json!({ "duration_days": 6 }))
            .await
            .json();
        let moved = r["moved"].as_array().unwrap();
        let m = |pid: &str| moved.iter().find(|x| x["pid"] == pid).cloned();
        assert_eq!(
            m(&b).map(|x| (x["moved"].clone(), x["finish_days"].clone())),
            Some((json!("finish"), json!(2)))
        );
        assert_eq!(m(&d).map(|x| x["moved"].clone()), Some(json!("both")));
        assert!(m(&a).is_none() && m(&c).is_none());
        let again: Value = request
            .put(&format!("/api/plans/{p}/tasks/{b}/schedule"))
            .json(&json!({ "duration_days": 6 }))
            .await
            .json();
        assert!(
            again["moved"].as_array().unwrap().is_empty(),
            "no change, nothing moved"
        );
        assert_eq!(
            request
                .put(&format!("/api/plans/{p}/tasks/{b}/schedule"))
                .json(&json!({ "duration_days": -1 }))
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .put(&format!(
                    "/api/plans/{p}/tasks/3f2c6b7e-0000-4000-8000-000000000bad/schedule"
                ))
                .json(&json!({ "duration_days": 1 }))
                .await
                .status_code(),
            404
        );

        // Relationship kinds and a lead. E starts 2 working days after A starts (SS lag 2),
        // F finishes when A finishes (FF), G leads D's finish by 1 (FS lag -1).
        let e = task(&request, &p, "E", None, Some(3)).await;
        let f = task(&request, &p, "F", None, Some(2)).await;
        let g = task(&request, &p, "G", None, Some(1)).await;
        dep(&a, &e, json!({ "kind": "ss", "lag_days": 2 }))
            .await
            .assert_status_ok();
        dep(&a, &f, json!({ "kind": "FF" }))
            .await
            .assert_status_ok();
        dep(&d, &g, json!({ "lag_days": -1 }))
            .await
            .assert_status_ok();
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let node = |pid: &str| {
            cpm["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|n| n["pid"] == pid)
                .cloned()
                .unwrap()
        };
        assert_eq!(
            node(&e)["early_start"],
            "2026-10-07",
            "A starts the 5th; SS lag 2"
        );
        assert_eq!(
            node(&f)["early_finish"],
            "2026-10-07",
            "F finishes when A (3 days) does"
        );
        assert_eq!(
            node(&g)["early_start"],
            node(&d)["early_finish"],
            "a one-day lead: G starts on D's last day"
        );

        // A cycle through a task relation is refused too.
        let (x, y) = (
            task(&request, &p, "X", None, Some(1)).await,
            task(&request, &p, "Y", None, Some(1)).await,
        );
        request
            .post(&format!("/api/plans/{p}/tasks/{x}/relations"))
            .json(&json!({ "kind": "precedes", "to_task_pid": y }))
            .await
            .assert_status_ok();
        assert_eq!(
            dep(&y, &x, json!({})).await.status_code(),
            422,
            "Y→X would close a cycle with X precedes Y"
        );

        // The plan-level schedule view carries a float-based block alongside the longest chain.
        let sched: Value = request
            .get(&format!("/api/plans/{p}/schedule"))
            .await
            .json();
        assert!(sched["cpm"]["calendar"].is_string(), "{sched}");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn calendars_constraints_deadlines_and_a_persons_absence() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Gemini" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();

        // The default calendar, then validation, then a Monday-to-Friday calendar with a holiday.
        let none: Value = request.get(&format!("/api/plans/{p}/calendar")).await.json();
        assert_eq!((none["source"].as_str(), none["weekdays"].as_str()), (Some("default"), Some("1111111")));
        for bad in [json!({ "weekdays": "0000000" }), json!({ "weekdays": "11111" }), json!({ "weekdays": "1111100", "holidays": ["2026-12-25", "2026-12-25"] })] {
            assert_eq!(request.put(&format!("/api/plans/{p}/calendar")).json(&bad).await.status_code(), 422, "{bad}");
        }
        request
            .put(&format!("/api/plans/{p}/calendar"))
            .json(&json!({ "weekdays": "1111100", "holidays": ["2026-10-12"] }))
            .await
            .assert_status_ok();

        // A Friday start of 2 working days ends the following Monday; across the holiday it ends Tuesday.
        let a = task(&request, &p, "A", Some("2026-10-02"), Some(2)).await;
        let c = task(&request, &p, "C", Some("2026-10-09"), Some(2)).await;
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let node = |pid: &str| cpm["nodes"].as_array().unwrap().iter().find(|n| n["pid"] == pid).cloned().unwrap();
        assert_eq!(cpm["calendar"], "plan");
        assert_eq!(node(&a)["early_finish"], "2026-10-05", "Fri + Mon");
        assert_eq!(node(&c)["early_finish"], "2026-10-13", "Fri, skip the weekend and the Monday holiday, Tue");

        // A person's recorded absence stretches the task assigned to them.
        let who = "person:3f2c6b7e-0000-4000-8000-0000000000aa";
        let t: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Mine", "assignee_ref": who, "start_date": "2026-10-05", "duration_days": 3 }))
            .await
            .json();
        let mine = t["pid"].as_str().unwrap().to_string();
        let before: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let f = |v: &Value| v["nodes"].as_array().unwrap().iter().find(|n| n["pid"] == mine.as_str()).cloned().unwrap();
        assert_eq!(f(&before)["early_finish"], "2026-10-07");
        assert_eq!(f(&before)["calendar"], "plan");
        request
            .post("/api/non-working")
            .json(&json!({ "person_ref": who, "starts_on": "2026-10-06", "ends_on": "2026-10-06", "kind": "leave" }))
            .await
            .assert_status_ok();
        let after: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        assert_eq!(f(&after)["early_finish"], "2026-10-08", "Mon, (leave), Wed, Thu");
        assert_eq!(f(&after)["calendar"], "plan+person");

        // Constraints: validation, a threatened FNLT gives negative float without moving the early date.
        let put = |task: &str, body: Value| {
            let (request, url) = (&request, format!("/api/plans/{p}/tasks/{task}/schedule"));
            async move { request.put(&url).json(&body).await }
        };
        assert_eq!(put(&a, json!({ "start_date": "2026-10-02", "duration_days": 2, "constraint_kind": "fnlt" })).await.status_code(), 422, "kind without date");
        assert_eq!(put(&a, json!({ "start_date": "2026-10-02", "duration_days": 2, "constraint_kind": "alap", "constraint_date": "2026-10-09" })).await.status_code(), 422, "alap unsupported");
        put(&a, json!({ "start_date": "2026-10-02", "duration_days": 2, "constraint_kind": "fnlt", "constraint_date": "2026-10-02" })).await.assert_status_ok();
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let n = |pid: &str| cpm["nodes"].as_array().unwrap().iter().find(|x| x["pid"] == pid).cloned().unwrap();
        assert_eq!(n(&a)["early_finish"], "2026-10-05", "FNLT does not move the early finish");
        assert!(n(&a)["total_float"].as_i64().unwrap() < 0, "negative float: {}", n(&a));
        assert_eq!(n(&a)["constraint"], "fnlt");

        // A soft deadline behaves the same way and never moves an early date.
        put(&c, json!({ "start_date": "2026-10-09", "duration_days": 2, "deadline": "2026-10-09" })).await.assert_status_ok();
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let c_node = cpm["nodes"].as_array().unwrap().iter().find(|x| x["pid"] == c.as_str()).cloned().unwrap();
        assert_eq!(c_node["early_finish"], "2026-10-13");
        assert!(c_node["total_float"].as_i64().unwrap() < 0);

        // A hard constraint needs a reason; with a reason (auth off) it pins the task and is flagged when logic disagrees.
        assert_eq!(put(&a, json!({ "start_date": "2026-10-02", "duration_days": 2, "constraint_kind": "mso", "constraint_date": "2026-10-06" })).await.status_code(), 422, "no reason");
        put(&a, json!({ "start_date": "2026-10-02", "duration_days": 2, "constraint_kind": "mso", "constraint_date": "2026-10-06", "reason": "customer fixed the start" }))
            .await
            .assert_status_ok();
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let a_node = cpm["nodes"].as_array().unwrap().iter().find(|x| x["pid"] == a.as_str()).cloned().unwrap();
        assert_eq!(a_node["early_start"], "2026-10-06", "the pin wins");
        assert_eq!(a_node["constraint"], "mso");
    })
    .await;
}
