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

const MSPDI: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Project xmlns="http://schemas.microsoft.com/project">
  <Name>Apollo</Name><MinutesPerDay>480</MinutesPerDay><CalendarUID>1</CalendarUID>
  <Calendars><Calendar><UID>1</UID><IsBaseCalendar>1</IsBaseCalendar><WeekDays>
    <WeekDay><DayType>1</DayType><DayWorking>0</DayWorking></WeekDay>
    <WeekDay><DayType>2</DayType><DayWorking>1</DayWorking></WeekDay>
    <WeekDay><DayType>3</DayType><DayWorking>1</DayWorking></WeekDay>
    <WeekDay><DayType>4</DayType><DayWorking>1</DayWorking></WeekDay>
    <WeekDay><DayType>5</DayType><DayWorking>1</DayWorking></WeekDay>
    <WeekDay><DayType>6</DayType><DayWorking>1</DayWorking></WeekDay>
    <WeekDay><DayType>7</DayType><DayWorking>0</DayWorking></WeekDay></WeekDays></Calendar></Calendars>
  <Tasks>
    <Task><UID>0</UID><Name>Apollo</Name><Summary>1</Summary></Task>
    <Task><UID>1</UID><Name>Design</Name><OutlineLevel>1</OutlineLevel><Summary>1</Summary><Start>2026-10-05T08:00:00</Start><Duration>PT64H0M0S</Duration></Task>
    <Task><UID>2</UID><Name>Draft</Name><OutlineLevel>2</OutlineLevel><Start>2026-10-05T08:00:00</Start><Duration>PT24H0M0S</Duration><PercentComplete>100</PercentComplete></Task>
    <Task><UID>3</UID><Name>Review</Name><OutlineLevel>2</OutlineLevel><Duration>PT32H0M0S</Duration><PercentComplete>40</PercentComplete>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>1</Type><LinkLag>0</LinkLag></PredecessorLink></Task>
    <Task><UID>4</UID><Name>Sign-off</Name><OutlineLevel>1</OutlineLevel><Milestone>1</Milestone><Duration>PT0H0M0S</Duration>
      <ConstraintType>2</ConstraintType><ConstraintDate>2026-11-02T08:00:00</ConstraintDate>
      <PredecessorLink><PredecessorUID>3</PredecessorUID><Type>3</Type><LinkLag>4800</LinkLag></PredecessorLink>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>0</Type></PredecessorLink></Task>
    <Task><UID>5</UID><Name>Cycle</Name><OutlineLevel>1</OutlineLevel><Duration>PT8H0M0S</Duration>
      <PredecessorLink><PredecessorUID>4</PredecessorUID><Type>1</Type></PredecessorLink></Task>
    <Task><UID>6</UID><Name>Back</Name><OutlineLevel>1</OutlineLevel><Duration>PT8H0M0S</Duration>
      <ConstraintType>1</ConstraintType>
      <PredecessorLink><PredecessorUID>4</PredecessorUID><Type>1</Type></PredecessorLink></Task>
  </Tasks>
  <Resources><Resource><UID>1</UID><Name>Alex</Name></Resource></Resources>
  <Assignments><Assignment><TaskUID>2</TaskUID><ResourceUID>1</ResourceUID></Assignment></Assignments>
</Project>"#;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn an_ms_project_file_imports_once_idempotently_and_reports_what_it_dropped() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let post = |q: &str, body: &str| {
            let (request, url, body) = (
                &request,
                format!("/api/plans/{p}/import/mspdi{q}"),
                body.to_string(),
            );
            async move {
                request
                    .post(&url)
                    .content_type("application/xml")
                    .text(body)
                    .await
            }
        };

        // Refusals: not MSPDI, a DTD, and not XML.
        assert_eq!(post("", "<Other/>").await.status_code(), 422);
        assert_eq!(post("", "not xml").await.status_code(), 422);
        assert_eq!(
            post(
                "",
                r#"<?xml version="1.0"?><!DOCTYPE p [<!ENTITY a "x">]><Project/>"#
            )
            .await
            .status_code(),
            422,
            "a DTD"
        );

        // A dry run (the default) reports and writes nothing.
        let dry: Value = post("", MSPDI).await.json();
        assert_eq!(
            (
                dry["dry_run"].as_bool(),
                dry["tasks_created"].as_i64(),
                dry["tasks_in_file"].as_i64()
            ),
            (Some(true), Some(6), Some(6)),
            "{dry}"
        );
        let none: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        assert!(
            none["tasks"].as_array().unwrap().is_empty(),
            "a dry run writes nothing"
        );

        // The real import. With auth off, the hard constraint on Sign-off is allowed.
        let done: Value = post("?dry_run=false", MSPDI).await.json();
        assert_eq!(
            (
                done["tasks_created"].as_i64(),
                done["tasks_updated"].as_i64()
            ),
            (Some(6), Some(0)),
            "{done}"
        );
        assert_eq!(done["calendar_imported"], true);
        let list: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        let by = |title: &str| {
            list["tasks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|t| t["title"] == title)
                .cloned()
                .unwrap()
        };
        assert_eq!(by("Draft")["duration_days"], 3);
        assert_eq!(by("Draft")["start_date"], "2026-10-05");
        assert_eq!(by("Draft")["status"], "done", "100% complete");
        assert_eq!(by("Review")["status"], "in_progress", "40% complete");
        assert_eq!(by("Design")["parent_task_pid"], Value::Null);
        assert_eq!(
            by("Draft")["parent_task_pid"],
            by("Design")["pid"],
            "outline level 2 under Design"
        );
        assert!(
            by("Design")["duration_days"].is_null(),
            "a summary row carries no schedule fields"
        );
        assert_eq!(by("Sign-off")["duration_days"], 0, "a milestone");
        assert!(
            by("Draft")["description"].is_null()
                || by("Draft")["description"]
                    .as_str()
                    .unwrap()
                    .contains("Alex")
        );

        // Dependencies keep their type and lag; the calendar came across; the network is analysable.
        let deps: Value = request.get("/api/dependencies").await.json();
        let kinds: Vec<(String, i64)> = deps
            .as_array()
            .unwrap()
            .iter()
            .map(|d| {
                (
                    d["kind"].as_str().unwrap().to_string(),
                    d["lag_days"].as_i64().unwrap(),
                )
            })
            .collect();
        assert!(kinds.contains(&("fs".to_string(), 0)), "{kinds:?}");
        assert!(
            kinds.contains(&("ss".to_string(), 1)),
            "Review→Sign-off SS with 1 day lag: {kinds:?}"
        );
        assert!(
            kinds.contains(&("ff".to_string(), 0)),
            "type 0 is FF, never FS: {kinds:?}"
        );
        let cal: Value = request
            .get(&format!("/api/plans/{p}/calendar"))
            .await
            .json();
        assert_eq!(
            (cal["source"].as_str(), cal["weekdays"].as_str()),
            (Some("plan"), Some("1111100"))
        );
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        assert_eq!(cpm["calendar"], "plan");
        assert!(!cpm["nodes"].as_array().unwrap().is_empty());

        // The report lists what did not come across, with reasons.
        let reasons: Vec<String> = done["unmapped"]
            .as_array()
            .unwrap()
            .iter()
            .map(|u| format!("{}:{}", u["what"], u["reason"]))
            .collect();
        let has = |needle: &str| reasons.iter().any(|r| r.contains(needle));
        assert!(has("as-late-as-possible"), "{reasons:?}");
        assert!(has("not matched to a person"), "{reasons:?}");
        assert!(has("cost rates"), "{reasons:?}");

        // Idempotent: importing again updates in place, creates nothing, moves no status.
        request
            .put(&format!(
                "/api/plans/{p}/tasks/{}",
                by("Review")["pid"].as_str().unwrap()
            ))
            .json(&json!({ "title": "Review", "assignee_ref": null }))
            .await
            .assert_status_ok();
        let again: Value = post("?dry_run=false", MSPDI).await.json();
        assert_eq!(
            (
                again["tasks_created"].as_i64(),
                again["tasks_updated"].as_i64()
            ),
            (Some(0), Some(6)),
            "{again}"
        );
        assert_eq!(again["links_created"], 0, "no duplicate dependencies");
        let list2: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        assert_eq!(list2["tasks"].as_array().unwrap().len(), 6, "no duplicates");
        let deps2: Value = request.get("/api/dependencies").await.json();
        assert_eq!(
            deps2.as_array().unwrap().len(),
            deps.as_array().unwrap().len()
        );
        let audit_note = again["note"].as_str().unwrap();
        assert!(audit_note.contains("never changes status"));
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn capacity_is_read_per_day_from_availability_ranges() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let who = "person:3f2c6b7e-0000-4000-8000-0000000000bb";
        request
            .post(&format!("/api/plans/{p}/allocations"))
            .json(&json!({ "person_ref": who, "percent": 80, "start_date": "2026-10-05", "end_date": "2026-10-18" }))
            .await
            .assert_status_ok();
        let cap = |from: &str, to: &str| {
            let (request, url) = (&request, format!("/api/capacity?from={from}&to={to}"));
            async move {
                let v: Value = request.get(&url).await.json();
                v["people"].as_array().unwrap().iter().find(|x| x["person_ref"] == who).cloned().unwrap()
            }
        };

        // No ranges: exactly as before (80% of 100 is not over).
        let before = cap("2026-10-05", "2026-10-18").await;
        assert_eq!((before["over_allocated"].as_bool(), before["capacity"]["source"].as_str()), (Some(false), Some("default")));
        assert_eq!(before["allocated_percent"], 80);

        // Validation and the range itself.
        let post = |body: Value| {
            let request = &request;
            async move { request.post("/api/availability").json(&body).await }
        };
        assert_eq!(post(json!({ "person_ref": "alex", "from": "2026-10-12", "to": "2026-10-18", "max_percent": 40 })).await.status_code(), 422);
        assert_eq!(post(json!({ "person_ref": who, "from": "2026-10-18", "to": "2026-10-12", "max_percent": 40 })).await.status_code(), 422);
        assert_eq!(post(json!({ "person_ref": who, "from": "2026-10-12", "to": "2026-10-18", "max_percent": 140 })).await.status_code(), 422);
        let part_time: Value = post(json!({ "person_ref": who, "from": "2026-10-12", "to": "2026-10-18", "max_percent": 40, "note": "part-time" })).await.json();
        assert_eq!(post(json!({ "person_ref": who, "from": "2026-10-18", "to": "2026-10-20", "max_percent": 50 })).await.status_code(), 409, "touching on the 18th overlaps");
        post(json!({ "person_ref": who, "from": "2026-10-19", "to": "2026-10-20", "max_percent": 0, "note": "leave" })).await.assert_status_ok();

        // Capacity is per day: the 40% week is over, the full-capacity week is not, and the sum is unchanged.
        let c = cap("2026-10-05", "2026-10-18").await;
        assert_eq!(c["over_allocated"], true, "{c}");
        assert_eq!(c["allocated_percent"], 80, "the old figure is unchanged");
        assert_eq!(c["capacity"], json!({ "source": "availability", "min_percent": 40 }));
        assert_eq!(c["over_capacity"], json!([{ "from": "2026-10-12", "to": "2026-10-18", "demand": 80, "capacity": 40 }]));
        let early = cap("2026-10-05", "2026-10-11").await;
        assert_eq!((early["over_allocated"].as_bool(), early["capacity"]["source"].as_str()), (Some(false), Some("default")), "a window before the range is untouched");

        // The list, and removal through the POST twin restores the old answer.
        let list: Value = request.get(&format!("/api/availability?person_ref={who}")).await.json();
        assert_eq!(list.as_array().unwrap().len(), 2);
        assert_eq!(list[0]["note"], "part-time");
        request.post(&format!("/api/availability/{}/delete", part_time["pid"].as_str().unwrap())).await.assert_status_ok();
        let after = cap("2026-10-05", "2026-10-18").await;
        assert_eq!(after["over_allocated"], false);
        // A leave day with demand is over capacity (0%).
        let leave = cap("2026-10-19", "2026-10-20").await;
        assert_eq!(leave["capacity"]["min_percent"], 0);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn proposed_bookings_never_count_until_committed() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let (a, b) = ("person:3f2c6b7e-0000-4000-8000-0000000000c1", "person:3f2c6b7e-0000-4000-8000-0000000000c2");
        let alloc = |who: &str, pct: i64, booking: Option<&str>| {
            let mut body = json!({ "person_ref": who, "percent": pct, "start_date": "2026-10-05", "end_date": "2026-10-18" });
            if let Some(k) = booking {
                body["booking"] = json!(k);
                body["requested"] = json!(true);
            }
            let (request, url) = (&request, format!("/api/plans/{p}/allocations"));
            async move { request.post(&url).json(&body).await }
        };
        assert_eq!(alloc(a, 30, Some("tentative")).await.status_code(), 422, "unknown booking");
        alloc(a, 70, None).await.assert_status_ok();
        let proposal: Value = alloc(a, 50, Some("proposed")).await.json();
        let only_proposed: Value = alloc(b, 20, Some("proposed")).await.json();
        let proposal = proposal["pid"].as_str().unwrap().to_string();
        let only_proposed = only_proposed["pid"].as_str().unwrap().to_string();

        let cap = |who: &str| {
            let (request, who) = (&request, who.to_string());
            async move {
                let v: Value = request.get("/api/capacity?from=2026-10-05&to=2026-10-18").await.json();
                v["people"].as_array().unwrap().iter().find(|x| x["person_ref"] == who.as_str()).cloned().unwrap()
            }
        };
        // The firm figures ignore the proposal; the proposed series is separate; a person with only proposals still shows.
        let c = cap(a).await;
        assert_eq!((c["allocated_percent"].as_i64(), c["over_allocated"].as_bool()), (Some(70), Some(false)), "{c}");
        assert_eq!(c["proposed"], json!({ "percent": 50, "allocations": 1, "would_exceed_capacity": true }));
        let cb = cap(b).await;
        assert_eq!((cb["allocated_percent"].as_i64(), cb["proposed"]["percent"].as_i64(), cb["proposed"]["would_exceed_capacity"].as_bool()), (Some(0), Some(20), Some(false)));

        // Committing the one that fits needs no confirmation; the one that overloads is a 409 with the numbers.
        request.post(&format!("/api/plans/{p}/allocations/{only_proposed}/commit")).await.assert_status_ok();
        let refused = request.post(&format!("/api/plans/{p}/allocations/{proposal}/commit")).await;
        assert_eq!(refused.status_code(), 409);
        assert!(refused.text().contains("over capacity") && refused.text().contains("120"), "{}", refused.text());
        assert_eq!(cap(a).await["proposed"]["allocations"], 1, "still a proposal");
        let done: Value = request.post(&format!("/api/plans/{p}/allocations/{proposal}/commit?confirm=true")).await.json();
        assert_eq!((done["booking"].as_str(), done["over_capacity"].as_bool()), (Some("committed"), Some(true)));
        let c = cap(a).await;
        assert_eq!((c["allocated_percent"].as_i64(), c["over_allocated"].as_bool(), c["proposed"]["allocations"].as_i64()), (Some(120), Some(true), Some(0)));
        assert_eq!(request.post(&format!("/api/plans/{p}/allocations/{proposal}/commit")).await.status_code(), 409, "already committed");

        // The plan's own list shows the booking and the request flag.
        let list: Value = request.get(&format!("/api/plans/{p}/allocations")).await.json();
        assert!(list.as_array().unwrap().iter().all(|x| x["booking"] == "committed"));
        assert!(list.as_array().unwrap().iter().any(|x| x["requested"] == true));
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn baselines_are_append_only_and_variance_separates_slip_added_and_removed() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let post = |reason: Value| {
            let (request, url) = (&request, format!("/api/plans/{p}/schedule-baselines"));
            async move { request.post(&url).json(&json!({ "reason": reason })).await }
        };

        // No baseline: variance is unknown, not zero; and nothing scheduled yet cannot be baselined.
        let none: Value = request
            .get(&format!("/api/plans/{p}/schedule-variance"))
            .await
            .json();
        assert_eq!(
            (none["reason"].as_str(), none["variance"].is_null()),
            (Some("no_baseline"), true)
        );
        assert_eq!(
            post(json!("kickoff")).await.status_code(),
            422,
            "nothing is scheduled yet"
        );

        let a = task(&request, &p, "A", Some("2026-10-05"), Some(3)).await; // Mon-Wed
        let b = task(&request, &p, "B", None, Some(4)).await;
        let gone = task(&request, &p, "Gone", Some("2026-10-05"), Some(10)).await;
        request
            .post("/api/dependencies")
            .json(&json!({ "predecessor_pid": a, "successor_pid": b }))
            .await
            .assert_status_ok();

        // A reason is required; then version 1.
        assert_eq!(post(json!("  ")).await.status_code(), 422);
        assert_eq!(post(json!("x".repeat(501))).await.status_code(), 422);
        let v1: Value = post(json!("Approved plan")).await.json();
        assert_eq!(
            (v1["version"].as_i64(), v1["task_count"].as_i64()),
            (Some(1), Some(3))
        );

        // The plan changes: A grows, a task is added, one is removed.
        request
            .put(&format!("/api/plans/{p}/tasks/{a}/schedule"))
            .json(&json!({ "start_date": "2026-10-05", "duration_days": 5 }))
            .await
            .assert_status_ok();
        let added = task(&request, &p, "Added", Some("2026-10-05"), Some(2)).await;
        request
            .delete(&format!("/api/plans/{p}/tasks/{gone}"))
            .await
            .assert_status_ok();

        let var: Value = request
            .get(&format!("/api/plans/{p}/schedule-variance"))
            .await
            .json();
        assert_eq!(var["baseline"]["version"], 1);
        let rows = var["variance"]["rows"].as_array().unwrap();
        let row = |pid: &str| rows.iter().find(|r| r["task_pid"] == pid).cloned().unwrap();
        assert_eq!(row(&a)["standing"], "slipped");
        assert_eq!(row(&a)["finish_variance_days"], 2, "3 days became 5");
        assert_eq!(row(&a)["duration_variance_days"], 2);
        assert_eq!(row(&b)["standing"], "slipped", "B follows A");
        assert_eq!(row(&added)["standing"], "added");
        assert!(
            row(&added)["finish_variance_days"].is_null(),
            "an added task is not slip"
        );
        assert_eq!(row(&gone)["standing"], "removed");
        assert_eq!(
            row(&gone)["baseline_finish"],
            "2026-10-14",
            "a removed task keeps its baselined dates"
        );
        let s = &var["variance"]["summary"];
        assert_eq!(
            (s["baseline_tasks"].as_i64(), s["current_tasks"].as_i64()),
            (Some(3), Some(3))
        );
        assert_eq!(
            (
                s["matched"].as_i64(),
                s["added"].as_i64(),
                s["removed"].as_i64(),
                s["slipped"].as_i64()
            ),
            (Some(2), Some(1), Some(1), Some(2))
        );
        // The longest baselined task was deleted, so the plan's own finish moved EARLIER by a
        // day: the report says so, and the removed row above shows why that is no improvement.
        assert_eq!(s["plan_finish_slip_days"], -1);

        // A re-baseline is version 2 with its reason; version 1 is untouched and still comparable.
        let v2: Value = post(json!("Re-baselined after the change")).await.json();
        assert_eq!(v2["version"], 2);
        let list: Value = request
            .get(&format!("/api/plans/{p}/schedule-baselines"))
            .await
            .json();
        assert_eq!(
            list.as_array()
                .unwrap()
                .iter()
                .map(|b| b["version"].as_i64().unwrap())
                .collect::<Vec<_>>(),
            vec![2, 1],
            "newest first, both kept"
        );
        assert_eq!(list[1]["reason"], "Approved plan");
        let latest: Value = request
            .get(&format!("/api/plans/{p}/schedule-variance"))
            .await
            .json();
        assert_eq!(latest["baseline"]["version"], 2);
        assert_eq!(
            latest["variance"]["summary"]["slipped"], 0,
            "against the new baseline nothing has slipped"
        );
        let old: Value = request
            .get(&format!("/api/plans/{p}/schedule-variance?baseline=1"))
            .await
            .json();
        assert_eq!(
            old["variance"]["summary"]["slipped"], 2,
            "version 1 still says what it said"
        );
        assert_eq!(
            request
                .get(&format!("/api/plans/{p}/schedule-variance?baseline=9"))
                .await
                .status_code(),
            404
        );

        // There is no way to change or remove a baseline.
        for method in ["put", "delete"] {
            let url = format!(
                "/api/plans/{p}/schedule-baselines/{}",
                v1["pid"].as_str().unwrap()
            );
            let status = if method == "put" {
                request.put(&url).json(&json!({})).await.status_code()
            } else {
                request.delete(&url).await.status_code()
            };
            assert!(status == 404 || status == 405, "{method} {status}");
        }
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_baseline_limit_refuses_rather_than_overwrites() {
    super::isolate_search_index();
    // SAFETY: `#[serial]` — nothing else runs while this is set.
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES", "2") };
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Limit" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        task(&request, &p, "A", Some("2026-10-05"), Some(3)).await;
        for _ in 0..2 {
            request
                .post(&format!("/api/plans/{p}/schedule-baselines"))
                .json(&json!({ "reason": "r" }))
                .await
                .assert_status_ok();
        }
        let third = request
            .post(&format!("/api/plans/{p}/schedule-baselines"))
            .json(&json!({ "reason": "r" }))
            .await;
        assert_eq!(third.status_code(), 409);
        assert!(
            third.text().contains("never overwritten"),
            "{}",
            third.text()
        );
        let list: Value = request
            .get(&format!("/api/plans/{p}/schedule-baselines"))
            .await
            .json();
        assert_eq!(list.as_array().unwrap().len(), 2, "both survive");
    })
    .await;
    unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES") };
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_raci_matrix_refuses_two_accountable_and_reports_gaps() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let (ann, bob, cy) = (
            "person:3f2c6b7e-0000-4000-8000-0000000000a1",
            "person:3f2c6b7e-0000-4000-8000-0000000000a2",
            "person:3f2c6b7e-0000-4000-8000-0000000000a3",
        );
        let design = task(&request, &p, "Design, v2", None, None).await;
        let build = task(&request, &p, "Build", None, None).await;
        let untouched = task(&request, &p, "Untouched", None, None).await;
        let put = |t: &str, body: Value| {
            let (request, url) = (&request, format!("/api/plans/{p}/tasks/{t}/raci"));
            async move { request.put(&url).json(&body).await }
        };

        // Nothing assigned: every task is a row with no cells and both gaps.
        let m0: Value = request.get(&format!("/api/plans/{p}/raci")).await.json();
        assert_eq!(m0["rows"].as_array().unwrap().len(), 3);
        assert_eq!(m0["summary"]["missing_accountable"], 3);
        assert!(m0["people"].as_array().unwrap().is_empty());

        // Refusals: two Accountable, a bad letter, the same person twice, no letters, not a URN, an unknown task.
        let two = json!({ "assignments": [{ "person_ref": ann, "roles": "A" }, { "person_ref": bob, "roles": "AR" }] });
        let r = put(&design, two).await;
        assert_eq!(r.status_code(), 422);
        assert!(r.text().contains("one Accountable person, not 2"), "{}", r.text());
        assert_eq!(put(&design, json!({ "assignments": [{ "person_ref": ann, "roles": "AX" }] })).await.status_code(), 422);
        assert_eq!(put(&design, json!({ "assignments": [{ "person_ref": ann, "roles": "A" }, { "person_ref": ann, "roles": "R" }] })).await.status_code(), 422);
        assert_eq!(put(&design, json!({ "assignments": [{ "person_ref": ann, "roles": "" }] })).await.status_code(), 422);
        assert_eq!(put(&design, json!({ "assignments": [{ "person_ref": "ann", "roles": "A" }] })).await.status_code(), 422);
        assert_eq!(put("3f2c6b7e-0000-4000-8000-000000000bad", json!({ "assignments": [] })).await.status_code(), 404);
        let still: Value = request.get(&format!("/api/plans/{p}/raci")).await.json();
        assert!(still["people"].as_array().unwrap().is_empty(), "a refused row wrote nothing");

        // A good row: A/R on one person, a consulted, an informed.
        let ok: Value = put(&design, json!({ "assignments": [{ "person_ref": ann, "roles": "ar" }, { "person_ref": bob, "roles": "C" }, { "person_ref": cy, "roles": "I" }] })).await.json();
        assert_eq!(ok["cells"][ann], "RA", "normalised to R-A-C-I order");
        assert_eq!(ok["findings"], json!([]));
        // An incomplete row is allowed, and says what is missing.
        let partial: Value = put(&build, json!({ "assignments": [{ "person_ref": bob, "roles": "R" }] })).await.json();
        assert_eq!(partial["findings"], json!(["no_accountable"]));

        let m: Value = request.get(&format!("/api/plans/{p}/raci")).await.json();
        assert_eq!(m["people"], json!([ann, bob, cy]));
        assert_eq!(m["summary"], json!({ "tasks": 3, "complete": 1, "missing_accountable": 2, "multiple_accountable": 0, "missing_responsible": 1 }));
        let rows = m["rows"].as_array().unwrap();
        let row = |t: &str| rows.iter().find(|r| r["task_pid"] == t).unwrap().clone();
        assert_eq!(row(&design)["cells"][bob], "C");
        assert!(row(&untouched)["cells"].as_object().unwrap().is_empty());
        let w = |who: &str| m["workload"].as_array().unwrap().iter().find(|x| x["person_ref"] == who).unwrap().clone();
        assert_eq!((w(ann)["accountable"].as_i64(), w(ann)["responsible"].as_i64()), (Some(1), Some(1)));
        assert_eq!((w(bob)["responsible"].as_i64(), w(bob)["consulted"].as_i64()), (Some(1), Some(1)));

        // The CSV: people as columns, the title quoted, the gaps in words.
        let csv = request.get(&format!("/api/plans/{p}/raci?format=csv")).await;
        assert_eq!(csv.status_code(), 200);
        let text = csv.text();
        assert!(text.starts_with(&format!("task,{ann},{bob},{cy},gaps\n")), "{text}");
        assert!(text.contains("\"Design, v2\",RA,C,I,"), "{text}");
        assert!(text.contains("no accountable; no responsible"), "{text}");

        // Replacing the row replaces it entirely: Ann is out, Bob now owns and does it.
        put(&design, json!({ "assignments": [{ "person_ref": bob, "roles": "AR" }] })).await.assert_status_ok();
        let m2: Value = request.get(&format!("/api/plans/{p}/raci")).await.json();
        let design_row = m2["rows"].as_array().unwrap().iter().find(|r| r["task_pid"] == design.as_str()).unwrap().clone();
        assert_eq!(design_row["cells"].as_object().unwrap().len(), 1);
        assert_eq!(design_row["cells"][bob], "RA");
        assert!(m2["workload"].as_array().unwrap().iter().all(|x| x["person_ref"] != ann), "Ann holds nothing now, so she is not a column");

        // Clearing a row is allowed and leaves it as a gap.
        put(&build, json!({ "assignments": [] })).await.assert_status_ok();
        let m3: Value = request.get(&format!("/api/plans/{p}/raci")).await.json();
        assert_eq!(m3["summary"]["missing_responsible"], 2);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn earned_value_is_computed_per_currency_and_says_when_it_cannot() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let ev = || {
            let (request, url) = (&request, format!("/api/plans/{p}/earned-value"));
            async move { request.get(&url).await.json::<Value>() }
        };

        // No baseline, then a baseline but no task: each says why, and neither is zero or 1.0.
        let none = ev().await;
        assert_eq!((none["absent"].as_str(), none["blocks"].is_null()), (Some("no_baseline"), true));
        let today = chrono::Utc::now().date_naive();
        let day = |n: i64| (today + chrono::Duration::days(n)).to_string();
        // GBP: 10 days ending today at 1 000, then the next 10 days at 3 000. BAC 4 000, PV today 1 000.
        request
            .post(&format!("/api/plans/{p}/budget-baselines"))
            .json(&json!({ "currency": "GBP", "periods": [
                { "period_start": day(-9), "period_end": day(0), "planned_minor": 1000 },
                { "period_start": day(1), "period_end": day(10), "planned_minor": 3000 } ] }))
            .await
            .assert_status_ok();
        assert_eq!(ev().await["absent"], "no_weighted_tasks");

        // Four tasks, two finished: 50% of a 4 000 budget is 2 000 earned against 1 000 planned.
        let mut ids = Vec::new();
        for (i, name) in ["A", "B", "C", "D"].iter().enumerate() {
            let t = task(&request, &p, name, None, None).await;
            if i < 2 {
                request.patch(&format!("/api/plans/{p}/tasks/{t}")).json(&json!({ "status": "done" })).await.assert_status_ok();
            }
            ids.push(t);
        }
        let line: Value = request
            .post(&format!("/api/plans/{p}/budget-lines"))
            .json(&json!({ "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": 4000 }))
            .await
            .json();
        let line = line["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/plans/{p}/budget-lines/{line}/actual")).json(&json!({ "amount_minor": 2500 })).await.assert_status_ok();
        // A line in a currency with no baseline is listed, never added in.
        request
            .post(&format!("/api/plans/{p}/budget-lines"))
            .json(&json!({ "category": "capex", "description": "dollars", "currency": "USD", "planned_minor": 100 }))
            .await
            .assert_status_ok();

        let v = ev().await;
        assert_eq!(v["percent_complete"]["percent_bp"], 5000, "{v}");
        assert_eq!(v["percent_complete"]["weighting"], "equal");
        assert_eq!(v["percent_complete"]["manual"], false);
        let b = &v["blocks"][0];
        assert_eq!((b["currency"].as_str(), b["baseline_version"].as_i64()), (Some("GBP"), Some(1)));
        assert_eq!((b["bac_minor"].as_i64(), b["pv_minor"].as_i64(), b["ev_minor"].as_i64(), b["ac_minor"].as_i64()), (Some(4000), Some(1000), Some(2000), Some(2500)));
        assert_eq!((b["sv_minor"].as_i64(), b["cv_minor"].as_i64()), (Some(1000), Some(-500)));
        assert_eq!(b["spi"]["bp"], 20000, "ahead of the plan: 2000 / 1000");
        assert_eq!(b["cpi"]["bp"], 8000, "over budget: 2000 / 2500");
        assert_eq!(v["blocks"].as_array().unwrap().len(), 1, "one block per baseline currency");
        assert_eq!(v["unbaselined_currencies"], json!(["USD"]));
        // Earned schedule: the plan first reaches 2 000 on day 14 (1 000 + 300 a day: 1 900 after day 13,
        // 2 200 after day 14), and today is day 10.
        assert_eq!(b["earned_schedule_days"], 14);
        assert_eq!(b["actual_days"], 10);
        assert_eq!(b["spi_t"]["bp"], 14000);

        // Completing a task moves earned value by exactly its weight (one of four tasks: 1 000).
        request.patch(&format!("/api/plans/{p}/tasks/{}", ids[2])).json(&json!({ "status": "done" })).await.assert_status_ok();
        assert_eq!(ev().await["blocks"][0]["ev_minor"], 3000);

        // A manual percent needs a number and a note, and is flagged.
        let manual = |body: Value| {
            let (request, url) = (&request, format!("/api/plans/{p}/tasks/{}/percent-complete", ids[3]));
            async move { request.put(&url).json(&body).await }
        };
        assert_eq!(manual(json!({ "method": "manual", "percent": 60 })).await.status_code(), 422, "no note");
        assert_eq!(manual(json!({ "method": "manual", "note": "why" })).await.status_code(), 422, "no percent");
        assert_eq!(manual(json!({ "method": "manual", "percent": 140, "note": "why" })).await.status_code(), 422);
        assert_eq!(manual(json!({ "method": "fifty_fifty", "percent": 10 })).await.status_code(), 422, "percent only with manual");
        assert_eq!(manual(json!({ "method": "vibes" })).await.status_code(), 422);
        manual(json!({ "method": "manual", "percent": 60, "note": "tests half written" })).await.assert_status_ok();
        let m = ev().await;
        assert_eq!(m["percent_complete"]["manual"], true);
        assert_eq!(m["percent_complete"]["manual_tasks"], json!([ids[3]]));
        assert_eq!(m["percent_complete"]["percent_bp"], 9000, "(100+100+100+60)/4");
        assert_eq!(m["blocks"][0]["ev_minor"], 3600);
        manual(json!({})).await.assert_status_ok(); // clears the override

        // The plan default: fifty_fifty gives started work half credit; points weights by points.
        assert_eq!(request.put(&format!("/api/plans/{p}/earned-value-settings")).json(&json!({ "default_method": "vibes" })).await.status_code(), 422);
        request.put(&format!("/api/plans/{p}/earned-value-settings")).json(&json!({ "default_method": "points" })).await.assert_status_ok();
        assert_eq!(ev().await["absent"], "no_weighted_tasks", "counting by points with no points is no percent, not zero");
        request.put(&format!("/api/plans/{p}/earned-value-settings")).json(&json!({ "default_method": "zero_hundred" })).await.assert_status_ok();

        // /performance now reports the real ratios, in basis points.
        let perf: Value = request.get(&format!("/api/plans/{p}/performance")).await.json();
        assert_eq!(perf["schedule"]["spi"], 30000, "3000 / 1000");
        assert_eq!(perf["schedule"]["unit"], "basis_points");
        assert!(perf["schedule"]["absent"].is_null());
    })
    .await;
}

#[allow(clippy::too_many_arguments)]
async fn ev_plan(
    request: &axum_test::TestServer,
    name: &str,
    parent: Option<&str>,
    currency: &str,
    planned: i64,
    done: usize,
    todo: usize,
    actual: i64,
) -> String {
    let mut body = json!({ "kind": "Project", "name": name });
    if let Some(p) = parent {
        body["parent_ref"] = json!(p);
    }
    let plan: Value = request.post("/api/plans").json(&body).await.json();
    let p = plan["pid"].as_str().unwrap().to_string();
    let today = chrono::Utc::now().date_naive();
    request
        .post(&format!("/api/plans/{p}/budget-baselines"))
        .json(&json!({ "currency": currency, "periods": [{
            "period_start": (today - chrono::Duration::days(9)).to_string(), "period_end": today.to_string(), "planned_minor": planned }] }))
        .await
        .assert_status_ok();
    for i in 0..done + todo {
        let t = task(request, &p, &format!("T{i}"), None, None).await;
        if i < done {
            request
                .patch(&format!("/api/plans/{p}/tasks/{t}"))
                .json(&json!({ "status": "done" }))
                .await
                .assert_status_ok();
        }
    }
    let line: Value = request
        .post(&format!("/api/plans/{p}/budget-lines"))
        .json(&json!({ "category": "capex", "description": "spend", "currency": currency, "planned_minor": planned }))
        .await
        .json();
    request
        .post(&format!(
            "/api/plans/{p}/budget-lines/{}/actual",
            line["pid"].as_str().unwrap()
        ))
        .json(&json!({ "amount_minor": actual }))
        .await
        .assert_status_ok();
    p
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn evm_forecasts_name_their_formula_and_the_series_and_rollup_stay_honest() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        // BAC 4 000 (10 days ending today at 1 000, 10 more at 3 000); 2 of 4 done: EV 2 000; AC 2 500;
        // PV today 1 000, so SPI 2.00 and CPI 0.80.
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let today = chrono::Utc::now().date_naive();
        let day = |n: i64| (today + chrono::Duration::days(n)).to_string();
        request
            .post(&format!("/api/plans/{p}/budget-baselines"))
            .json(&json!({ "currency": "GBP", "periods": [
                { "period_start": day(-9), "period_end": day(0), "planned_minor": 1000 },
                { "period_start": day(1), "period_end": day(10), "planned_minor": 3000 } ] }))
            .await
            .assert_status_ok();
        let none: Value = request.get(&format!("/api/plans/{p}/earned-value/forecast")).await.json();
        assert_eq!(none["absent"], "no_weighted_tasks");
        for i in 0..4 {
            let t = task(&request, &p, &format!("T{i}"), None, None).await;
            if i < 2 {
                request.patch(&format!("/api/plans/{p}/tasks/{t}")).json(&json!({ "status": "done" })).await.assert_status_ok();
            }
        }
        let line: Value = request.post(&format!("/api/plans/{p}/budget-lines")).json(&json!({ "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": 4000 })).await.json();
        request.post(&format!("/api/plans/{p}/budget-lines/{}/actual", line["pid"].as_str().unwrap())).json(&json!({ "amount_minor": 2500 })).await.assert_status_ok();

        // Every EAC variant is named by its formula, and each number is the hand-worked one.
        let f: Value = request.get(&format!("/api/plans/{p}/earned-value/forecast")).await.json();
        let fc = &f["forecasts"][0];
        let v = |name: &str| fc["variants"].as_array().unwrap().iter().find(|x| x["variant"] == name).cloned().unwrap();
        assert_eq!(v("budget_rate")["eac"]["minor"], 4500, "2 500 + (4 000 - 2 000)");
        assert_eq!(v("cpi")["eac"]["minor"], 5000, "2 500 + 2 000 / 0.80");
        assert_eq!(v("cpi_spi")["eac"]["minor"], 3750, "2 500 + 2 000 / (0.80 x 2.00)");
        assert_eq!(v("cpi")["vac"]["minor"], -1000);
        assert_eq!(v("cpi")["etc"]["minor"], 2500);
        assert!(v("cpi")["formula"].as_str().unwrap().contains("CPI"));
        assert_eq!(v("bottom_up")["eac"]["absent"], "no_bottom_up_estimate", "nobody re-estimated");
        assert_eq!((fc["recommended"]["variant"].as_str(), fc["recommended"]["eac_minor"].as_i64()), (Some("cpi"), Some(5000)), "schedule is fine, so cost efficiency continues");
        assert!(fc["recommended"]["why"].as_str().unwrap().len() > 10, "the rule is stated");
        assert_eq!(fc["tcpi_to_budget"]["tcpi"]["bp"], 13333, "(4 000 - 2 000) / (4 000 - 2 500)");
        assert_eq!(fc["tcpi_to_budget"]["tcpi"]["needs_improved_efficiency"], true);
        assert_eq!(fc["tcpi_to_budget"]["rag"], "red");
        assert_eq!(fc["tcpi_to_recommended_eac"]["tcpi"]["bp"], 8000);
        assert_eq!((fc["spi"]["rag"].as_str(), fc["cpi"]["rag"].as_str()), (Some("green"), Some("red")));
        assert_eq!(f["thresholds"]["source"], "default");

        // Thresholds are per deployment and the response says where they came from.
        // SAFETY: `#[serial]` — nothing else runs while this is set.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS", "spi_cpi_amber=8000,spi_cpi_red=7000") };
        let lenient: Value = request.get(&format!("/api/plans/{p}/earned-value/forecast")).await.json();
        assert_eq!(lenient["forecasts"][0]["cpi"]["rag"], "green", "0.80 clears an 0.80 line");
        assert_eq!(lenient["thresholds"]["source"], "PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS");
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS") };

        // The series: the planned curve is whole; earned value and cost only where a snapshot was recorded.
        let before: Value = request.get(&format!("/api/plans/{p}/earned-value/series")).await.json();
        let pts = before["curves"][0]["points"].as_array().unwrap();
        assert!(pts.iter().all(|x| x["ev_minor"].is_null() && x["ac_minor"].is_null()), "nothing recorded yet, nothing invented");
        assert_eq!(pts.last().unwrap()["pv_minor"], 4000);
        assert_eq!(pts[0]["pv_minor"], 0, "the day before the baseline starts");
        let snap: Value = request.post(&format!("/api/plans/{p}/earned-value/snapshot")).await.json();
        assert_eq!(snap["snapshotted"], 1);
        request.post(&format!("/api/plans/{p}/earned-value/snapshot")).await.assert_status_ok(); // idempotent per day
        let after: Value = request.get(&format!("/api/plans/{p}/earned-value/series")).await.json();
        assert_eq!(after["curves"][0]["snapshots"], 1, "a second snapshot the same day replaced the first");
        let today_pt = after["curves"][0]["points"].as_array().unwrap().iter().find(|x| x["date"] == today.to_string().as_str()).cloned().unwrap();
        assert_eq!((today_pt["pv_minor"].as_i64(), today_pt["ev_minor"].as_i64(), today_pt["ac_minor"].as_i64()), (Some(1000), Some(2000), Some(2500)));
        let csv = request.get(&format!("/api/plans/{p}/earned-value/series?format=csv")).await;
        assert!(csv.text().starts_with("currency,date,pv_minor,ev_minor,ac_minor\n"), "{}", csv.text());
        assert!(csv.text().contains(&format!("GBP,{today},1000,2000,2500\n")), "{}", csv.text());
        assert!(csv.text().contains(",4000,,\n"), "no EV or AC is invented on the baseline's last day: {}", csv.text());

        // Roll-up: a parent with no baseline of its own, a GBP child and a USD child. Currencies stay apart.
        let parent: Value = request.post("/api/plans").json(&json!({ "kind": "Portfolio", "name": "Programme" })).await.json();
        let parent = parent["pid"].as_str().unwrap().to_string();
        ev_plan(&request, "GBP child", Some(&parent), "GBP", 1000, 1, 1, 400).await; // EV 500, AC 400
        ev_plan(&request, "USD child", Some(&parent), "USD", 2000, 1, 0, 1000).await; // EV 2 000, AC 1 000
        let r: Value = request.get(&format!("/api/plans/{parent}/earned-value/rollup")).await.json();
        let cur = |c: &str| r["currencies"].as_array().unwrap().iter().find(|x| x["currency"] == c).cloned().unwrap();
        let (gbp, usd) = (cur("GBP"), cur("USD"));
        assert_eq!((gbp["totals"]["bac_minor"].as_i64(), gbp["totals"]["ev_minor"].as_i64(), gbp["totals"]["ac_minor"].as_i64()), (Some(1000), Some(500), Some(400)), "{r}");
        assert_eq!((gbp["spi"]["bp"].as_i64(), gbp["cpi"]["bp"].as_i64()), (Some(5000), Some(12500)));
        assert_eq!((usd["totals"]["bac_minor"].as_i64(), usd["totals"]["ev_minor"].as_i64()), (Some(2000), Some(2000)));
        assert_eq!(r["currencies"].as_array().unwrap().len(), 2, "two currencies, two totals, no merged figure");
        assert_eq!(r["plans_without_figures"], 1, "the parent itself has no baseline: counted, not guessed");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn leveling_proposes_within_float_and_applies_only_when_confirmed() {
    use project_portfolio_management_service::models::_entities::audit_logs;
    use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let who = format!("person:{}", uuid::Uuid::new_v4());
        let mk = |title: &'static str, days: i64, assignee: Option<String>| {
            let request = &request;
            let p = p.clone();
            async move {
                let mut body = json!({ "title": title, "start_date": "2026-10-05", "duration_days": days });
                if let Some(a) = assignee {
                    body["assignee_ref"] = json!(a);
                }
                let t: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&body).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let start_of = |c: &str| {
            let request = &request;
            let p = p.clone();
            let c = c.to_string();
            async move {
                let list: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
                let items = list["tasks"].as_array().cloned().unwrap();
                items.iter().find(|t| t["pid"] == c.as_str()).cloned().unwrap()["start_date"].clone()
            }
        };
        let _a = mk("Critical", 5, None).await;
        let _b = mk("B", 2, Some(who.clone())).await;
        let c = mk("C", 2, Some(who.clone())).await;

        // The proposal is advice: it names the over-allocation and one delay, and
        // changes nothing.
        let r: Value = request.get(&format!("/api/plans/{p}/leveling")).await.json();
        assert_eq!(r["advisory"], true);
        assert_eq!(r["over_allocated"].as_array().unwrap().len(), 1);
        let moves = r["moves"].as_array().unwrap();
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0]["pid"], c.as_str());
        assert_eq!(moves[0]["to_start"], "2026-10-07");
        assert_eq!(moves[0]["critical"], false);
        assert_eq!(r["finish_delay_days"], 0);
        assert_eq!(r["finish_before"], r["finish_after"]);
        assert_eq!(moves[0]["person_ref"], who.as_str(), "auth is off: not masked");
        let again: Value = request.get(&format!("/api/plans/{p}/leveling")).await.json();
        assert_eq!(r, again, "the same input gives the same proposal");
        assert_eq!(start_of(&c).await, "2026-10-05", "reading changed nothing");

        // Applying is a separate, confirmed call.
        let move_body = |confirm: bool, date: &str| json!({
            "moves": [{ "task_pid": c, "start_date": date }], "confirm": confirm, "reason": "level" });
        request
            .post(&format!("/api/plans/{p}/leveling/apply"))
            .json(&move_body(false, "2026-10-07"))
            .await
            .assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
        request
            .post(&format!("/api/plans/{p}/leveling/apply"))
            .json(&move_body(true, "2026-10-09"))
            .await
            .assert_status(axum::http::StatusCode::CONFLICT);
        let ok: Value = request
            .post(&format!("/api/plans/{p}/leveling/apply"))
            .json(&move_body(true, "2026-10-07"))
            .await
            .json();
        assert_eq!(ok["ok"], 1);
        assert_eq!(ok["refused"], 0);
        assert_eq!(start_of(&c).await, "2026-10-07");

        // One audit entry per moved task, and nothing is over-allocated now.
        let n = audit_logs::Entity::find()
            .filter(audit_logs::Column::EntityPid.eq(uuid::Uuid::parse_str(&c).unwrap()))
            .filter(audit_logs::Column::Action.eq("task_leveled"))
            .count(&ctx.db)
            .await
            .unwrap();
        assert_eq!(n, 1);
        let after: Value = request.get(&format!("/api/plans/{p}/leveling")).await.json();
        assert!(after["moves"].as_array().unwrap().is_empty());
        assert!(after["over_allocated"].as_array().unwrap().is_empty());
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn portfolio_resources_show_role_gaps_placeholders_and_honest_hours() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let mk_plan = |name: &'static str| {
            let request = &request;
            async move {
                let p: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": name })).await.json();
                p["pid"].as_str().unwrap().to_string()
            }
        };
        let (a, b) = (mk_plan("Apollo").await, mk_plan("Gemini").await);
        let who = |n: u8| format!("person:00000000-0000-4000-8000-0000000000{n:02}");
        let (alice, bob) = (who(1), who(2));
        let qs = "from=2026-10-05&to=2026-10-11&period=week";

        // Two developers (2.00 FTE); demand 100% + 50% named, plus a 100% placeholder.
        for p in [&alice, &bob] {
            request.put("/api/resource-roles").json(&json!({ "person_ref": p, "role": "dev" })).await.assert_status_ok();
        }
        let alloc = |plan: String, body: Value| {
            let request = &request;
            async move {
                let r: Value = request.post(&format!("/api/plans/{plan}/allocations")).json(&body).await.json();
                r["pid"].as_str().unwrap().to_string()
            }
        };
        let _ = alloc(a.clone(), json!({ "person_ref": alice, "role": "dev", "percent": 100, "start_date": "2026-10-05", "end_date": "2026-10-11" })).await;
        let _ = alloc(b.clone(), json!({ "person_ref": bob, "role": "dev", "percent": 50, "start_date": "2026-10-05", "end_date": "2026-10-11" })).await;
        let ph = alloc(b.clone(), json!({ "placeholder": true, "role": "dev", "percent": 100, "start_date": "2026-10-05", "end_date": "2026-10-11" })).await;
        let _ = alloc(a.clone(), json!({ "person_ref": alice, "role": "dev", "percent": 100, "booking": "proposed", "start_date": "2026-10-05", "end_date": "2026-10-11" })).await;
        // A placeholder takes a role and no person.
        request.post(&format!("/api/plans/{a}/allocations")).json(&json!({ "placeholder": true, "percent": 10 })).await.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);

        let r: Value = request.get(&format!("/api/resource-plan?{qs}")).await.json();
        let row = r["periods"][0]["rows"].as_array().unwrap().iter().find(|x| x["key"] == "dev").cloned().unwrap();
        assert_eq!(row["demand_fte"], 2.5);
        assert_eq!(row["placeholder_fte"], 1.0, "the placeholder is shown apart");
        assert_eq!(row["supply_fte"], 2.0);
        assert_eq!(row["gap_fte"], -0.5, "short by half an FTE");
        assert_eq!(row["headcount"], 2);
        assert_eq!(row["proposed_fte"], 1.0, "proposed stays a separate series");
        assert_eq!(r["placeholders"].as_array().unwrap().len(), 1);
        let by_plan: i64 = r["by_plan"].as_array().unwrap().iter().map(|s| s["demand_percent_days"][0].as_i64().unwrap()).sum();
        assert_eq!(by_plan, r["periods"][0]["total_demand_percent_days"].as_i64().unwrap(), "per-plan sums to the total");

        // A placeholder never counts as a person's load.
        let cap: Value = request.get("/api/capacity?from=2026-10-05&to=2026-10-11").await.json();
        assert!(cap["people"].as_array().unwrap().iter().all(|p| !p["person_ref"].as_str().unwrap().starts_with("placeholder:")));

        // Resolving names the person; the placeholder then stops being unassigned.
        request.post(&format!("/api/plans/{b}/allocations/{ph}/resolve")).json(&json!({ "person_ref": bob })).await.assert_status(axum::http::StatusCode::CONFLICT); // bob would be at 150%
        request.post(&format!("/api/plans/{b}/allocations/{ph}/resolve")).json(&json!({ "person_ref": bob, "confirm": true })).await.assert_status_ok();
        request.post(&format!("/api/plans/{b}/allocations/{ph}/resolve")).json(&json!({ "person_ref": alice })).await.assert_status(axum::http::StatusCode::CONFLICT);
        let r: Value = request.get(&format!("/api/resource-plan?{qs}")).await.json();
        assert!(r["placeholders"].as_array().unwrap().is_empty());
        assert_eq!(r["periods"][0]["rows"][0]["placeholder_fte"], 0.0);

        // Hours: nothing recorded is null, never zero; unplanned time is listed.
        let h: Value = request.get("/api/resource-plan/hours?from=2026-10-05&to=2026-10-11").await.json();
        assert!(h["total_actual_minutes"].is_null());
        assert_eq!(h["total_actual_reason"], "no_actuals");
        let plan_a = h["plans"].as_array().unwrap().iter().find(|x| x["plan_pid"] == a.as_str()).cloned().unwrap();
        assert_eq!(plan_a["planned_minutes"], 5 * 480);
        assert!(plan_a["actual_minutes"].is_null());
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn schedule_risk_is_seeded_honest_about_inputs_and_uses_register_events() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let a = task(&request, &p, "A", Some("2026-10-05"), Some(5)).await;
        let b = task(&request, &p, "B", Some("2026-10-05"), Some(4)).await;
        let c = task(&request, &p, "C", None, Some(3)).await;
        for (f, t) in [(&a, &c), (&b, &c)] {
            request
                .post("/api/dependencies")
                .json(&json!({ "predecessor_pid": f, "successor_pid": t }))
                .await
                .assert_status_ok();
        }
        let url = |q: &str| format!("/api/plans/{p}/schedule-risk?{q}");

        // No estimates: every task uses the stated fallback, and says so.
        let r: Value = request.get(&url("seed=7")).await.json();
        assert_eq!(r["assumptions"]["correlation"], "none");
        assert!(
            r["assumptions"]["independence"]
                .as_str()
                .unwrap()
                .contains("independent")
        );
        assert_eq!(
            r["assumptions"]["tasks_using_fallback"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
        assert!(
            r["tasks"]
                .as_array()
                .unwrap()
                .iter()
                .all(|t| t["source"] == "fallback")
        );

        // Same seed, same result; another seed, another.
        let again: Value = request.get(&url("seed=7")).await.json();
        assert_eq!(r, again);
        let other: Value = request.get(&url("seed=8")).await.json();
        assert_ne!(r["histogram"], other["histogram"]);

        // Percentiles are ordered.
        let ps: Vec<String> = r["percentiles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["finish"].as_str().unwrap().to_string())
            .collect();
        assert!(ps.windows(2).all(|w| w[0] <= w[1]));

        // Zero uncertainty: every percentile is the CPM finish exactly.
        let flat: Value = request.get(&url("fallback_band_percent=0")).await.json();
        assert!(
            flat["percentiles"]
                .as_array()
                .unwrap()
                .iter()
                .all(|x| x["finish"] == flat["cpm_finish"])
        );
        assert_eq!(flat["cpm_finish_confidence_bp"], 10_000);

        // An estimate must sit around the likely duration.
        request
            .put(&format!("/api/plans/{p}/tasks/{a}/estimate"))
            .json(&json!({ "optimistic_days": 6 }))
            .await
            .assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
        request
            .put(&format!("/api/plans/{p}/tasks/{a}/estimate"))
            .json(&json!({ "optimistic_days": 3, "pessimistic_days": 10 }))
            .await
            .assert_status_ok();
        let tasks: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        let ta_listed = tasks["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["pid"] == a.as_str())
            .cloned()
            .unwrap();
        assert_eq!(ta_listed["duration_optimistic_days"], 3);
        assert_eq!(ta_listed["duration_pessimistic_days"], 10);
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).await.json();
        let node = cpm["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| n["pid"] == a.as_str())
            .cloned()
            .unwrap();
        assert_eq!(node["duration_optimistic_days"], 3);
        assert_eq!(node["duration_pessimistic_days"], 10);
        let est: Value = request
            .get(&url("fallback_band_percent=0&seed=3"))
            .await
            .json();
        let ta = est["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["pid"] == a.as_str())
            .cloned()
            .unwrap();
        assert_eq!(ta["source"], "estimated");
        assert!(
            ta["sensitivity"].as_f64().unwrap() > 0.5,
            "A now drives the finish"
        );

        // Register risks: one complete event, one with no probability (named).
        let risk = |title: &'static str| {
            let request = &request;
            let p = p.clone();
            async move {
                let r: Value = request
                    .post(&format!("/api/plans/{p}/risks"))
                    .json(&json!({ "title": title, "probability": 3, "impact": 3 }))
                    .await
                    .json();
                r["pid"].as_str().unwrap().to_string()
            }
        };
        let (hit, nop, plain) = (
            risk("Vendor slips").await,
            risk("Unquantified").await,
            risk("Not a schedule risk").await,
        );
        let _ = plain;
        request
            .put(&format!("/api/plans/{p}/risks/{hit}/schedule-impact"))
            .json(&json!({ "task_pid": a, "impact_days": 10, "probability_percent": 100 }))
            .await
            .assert_status_ok();
        request
            .put(&format!("/api/plans/{p}/risks/{nop}/schedule-impact"))
            .json(&json!({ "task_pid": b, "impact_days": 5 }))
            .await
            .assert_status_ok();
        let ev: Value = request
            .get(&url("fallback_band_percent=0&seed=3"))
            .await
            .json();
        assert_eq!(ev["events"]["used"].as_array().unwrap().len(), 1);
        let skipped = ev["events"]["skipped"].as_array().unwrap();
        assert_eq!(skipped.len(), 1);
        assert_eq!(skipped[0]["risk_pid"], nop.as_str());
        assert_eq!(skipped[0]["reason"], "no_probability");
        assert_eq!(ev["events"]["risks_without_schedule_fields"], 1);
        assert_eq!(
            ev["cpm_finish_confidence_bp"], 0,
            "a certain 10-day slip never meets the CPM date"
        );

        // Caps are refused, not truncated.
        request
            .get(&url("iterations=0"))
            .await
            .assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
        request
            .get(&url("iterations=1000000"))
            .await
            .assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn schedule_health_runs_fourteen_checks_and_never_passes_what_it_cannot_compute() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let a = task(&request, &p, "A", Some("2026-10-05"), Some(5)).await;
        let b = task(&request, &p, "B", None, Some(5)).await;
        let c = task(&request, &p, "C", None, Some(50)).await;
        for (f, t) in [(&a, &b), (&b, &c)] {
            request
                .post("/api/dependencies")
                .json(&json!({ "predecessor_pid": f, "successor_pid": t }))
                .await
                .assert_status_ok();
        }
        let h: Value = request
            .get(&format!("/api/plans/{p}/schedule-health?as_of=2026-10-05"))
            .await
            .json();
        assert_eq!(h["preset"], "dcma-default");
        assert!(
            h["disclaimer"]
                .as_str()
                .unwrap()
                .contains("not a DCMA assessment")
        );
        assert_eq!(h["thresholds"]["source"], "default");
        let checks = h["checks"].as_array().unwrap();
        assert_eq!(checks.len(), 14);
        let by = |k: &str| checks.iter().find(|x| x["key"] == k).cloned().unwrap();
        // No baseline: BEI and missed tasks are null with a reason, not passes.
        for k in ["missed_tasks", "bei"] {
            assert!(by(k)["pass"].is_null(), "{k}");
            assert_eq!(by(k)["reason"], "no_baseline");
        }
        assert_eq!(h["all_pass"], false);
        // C is 50 working days: the one high-duration offender, named by pid.
        let hd = by("high_duration");
        assert_eq!(hd["pass"], false);
        assert_eq!(hd["offenders"][0]["pid"], c.as_str());
        // Nobody is assigned: every duration task is a resources offender.
        assert_eq!(by("resources")["count"], 3);
        assert_eq!(by("negative_float")["pass"], true);
        // A bad status date is refused.
        request
            .get(&format!("/api/plans/{p}/schedule-health?as_of=1900-01-01"))
            .await
            .assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);

        // With a baseline taken, the baseline checks compute.
        request
            .post(&format!("/api/plans/{p}/schedule-baselines"))
            .json(&json!({ "reason": "first" }))
            .await
            .assert_status_ok();
        let h: Value = request
            .get(&format!("/api/plans/{p}/schedule-health?as_of=2026-10-30"))
            .await
            .json();
        let bei = h["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["key"] == "bei")
            .cloned()
            .unwrap();
        assert!(
            bei["pass"].is_boolean(),
            "computed once a baseline exists: {bei}"
        );
        assert_eq!(
            bei["pass"], false,
            "nothing is done but A and B were baselined due"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn kpis_derive_status_from_readings_and_okrs_can_be_listed_edited_and_deleted() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let today = chrono::Utc::now().date_naive();
        let day = |n: i64| (today - chrono::Duration::days(n)).to_string();

        // ── KPIs ──
        // The red line must sit on the worse side of the target.
        request.post("/api/kpis").json(&json!({ "name": "Uptime", "direction": "higher_better", "target_value": 95, "red_value": 99 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.post("/api/kpis").json(&json!({ "name": "Defects", "direction": "lower_better", "target_value": 5, "red_value": 2 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.post("/api/kpis").json(&json!({ "name": "x", "direction": "sideways", "target_value": 1, "red_value": 0 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let k: Value = request.post("/api/kpis").json(&json!({
            "name": "Uptime", "unit": "percent", "direction": "higher_better",
            "target_value": 95, "red_value": 80, "cadence": "weekly" })).await.json();
        let kp = k["pid"].as_str().unwrap().to_string();
        let defects: Value = request.post("/api/kpis").json(&json!({
            "name": "Open defects", "direction": "lower_better", "target_value": 5, "red_value": 20 })).await.json();
        let dp = defects["pid"].as_str().unwrap().to_string();

        // No reading: no data — not red, not green.
        let one: Value = request.get(&format!("/api/kpis/{kp}")).await.json();
        assert_eq!(one["status"], "no_data");
        assert_eq!(one["stale"], true, "a cadence KPI never read is stale");
        assert!(one["latest"].is_null() && one["trend"].is_null());

        let read = |pid: String, value: i64, on: String| {
            let request = &request;
            async move {
                request.post(&format!("/api/kpis/{pid}/readings")).json(&json!({ "value": value, "observed_on": on })).await
            }
        };
        let r = read(kp.clone(), 90, day(8)).await;
        r.assert_status_ok();
        assert_eq!(r.json::<Value>()["kpi"]["status"], "amber");
        let r: Value = read(kp.clone(), 70, day(1)).await.json();
        assert_eq!(r["kpi"]["status"], "red");
        assert_eq!(r["kpi"]["trend"], "worsening");
        assert_eq!(r["kpi"]["stale"], false);
        // A real zero is a reading.
        read(dp.clone(), 0, day(0)).await.assert_status_ok();
        assert_eq!(request.get(&format!("/api/kpis/{dp}")).await.json::<Value>()["status"], "green");
        // A future date is refused.
        let future = (today + chrono::Duration::days(3)).to_string();
        read(kp.clone(), 99, future).await.assert_status(S::UNPROCESSABLE_ENTITY);

        // A correction is a newer reading for the same day; the status follows it.
        let r: Value = read(kp.clone(), 99, day(1)).await.json();
        assert_eq!(r["kpi"]["status"], "green");
        let rs: Value = request.get(&format!("/api/kpis/{kp}/readings")).await.json();
        let rs = rs["readings"].as_array().unwrap();
        assert_eq!(rs.len(), 3);
        assert_eq!(rs[0]["value"], 99);
        assert_eq!(rs[1]["superseded"], true, "the 70 is superseded by the 99");

        // The list: red first, summary over the whole set, filter by status.
        let k2: Value = request.post("/api/kpis").json(&json!({
            "name": "Cost per unit", "direction": "lower_better", "target_value": 10, "red_value": 30 })).await.json();
        let k2p = k2["pid"].as_str().unwrap().to_string();
        read(k2p.clone(), 40, day(0)).await.assert_status_ok();
        let list: Value = request.get("/api/kpis").await.json();
        assert_eq!(list["kpis"][0]["status"], "red");
        assert!(list["summary"]["green"].as_u64().unwrap() >= 2 && list["summary"]["red"].as_u64().unwrap() >= 1);
        let reds: Value = request.get("/api/kpis?status=red").await.json();
        assert!(reds["kpis"].as_array().unwrap().iter().all(|x| x["status"] == "red"));
        assert_eq!(reds["summary"], list["summary"], "the summary ignores the status filter");
        request.get("/api/kpis?status=purple").await.assert_status(S::UNPROCESSABLE_ENTITY);

        // Direction is not editable; thresholds are checked against it.
        request.put(&format!("/api/kpis/{kp}")).json(&json!({ "name": "Uptime", "target_value": 95, "red_value": 80, "direction": "lower_better" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.put(&format!("/api/kpis/{kp}")).json(&json!({ "name": "Uptime", "target_value": 95, "red_value": 97 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.put(&format!("/api/kpis/{kp}")).json(&json!({ "name": "Uptime (SLO)", "target_value": 99, "red_value": 90 })).await.assert_status_ok();
        let after: Value = request.get(&format!("/api/kpis/{kp}")).await.json();
        assert_eq!(after["status"], "green", "99 still meets the new 99 target");
        request.delete(&format!("/api/kpis/{k2p}")).await.assert_status_ok();
        request.get(&format!("/api/kpis/{k2p}")).await.assert_status(S::NOT_FOUND);

        // ── OKRs ──
        let obj = |title: &'static str| {
            let request = &request;
            async move {
                let o: Value = request.post("/api/objectives").json(&json!({ "title": title, "period": "2026-Q4" })).await.json();
                o["pid"].as_str().unwrap().to_string()
            }
        };
        let (measured, empty) = (obj("Grow").await, obj("Unmeasured one").await);
        let kr: Value = request.post(&format!("/api/objectives/{measured}/key-results")).json(&json!({
            "title": "Users", "metric": "number", "direction": "increase", "start_value": 0, "target_value": 100 })).await.json();
        let krp = kr["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/key-results/{krp}/check-ins")).json(&json!({ "value": 40, "confidence": 3 })).await.assert_status_ok();

        let okrs: Value = request.get("/api/okrs?period=2026-Q4").await.json();
        let objs = okrs["objectives"].as_array().unwrap();
        assert_eq!(objs.len(), 2);
        assert_eq!(objs[0]["objective"]["pid"], measured.as_str());
        assert_eq!(objs[0]["score_basis_points"], 4000);
        assert_eq!(objs[0]["key_results"][0]["latest_check_in"]["confidence"], 3);
        assert_eq!(objs[1]["objective"]["pid"], empty.as_str());
        assert_eq!(objs[1]["measured"], false, "unmeasured sorts last and is not a zero");
        assert!(objs[1]["score_basis_points"].is_null());
        let none: Value = request.get("/api/okrs?period=1999-Q1").await.json();
        assert!(none["objectives"].as_array().unwrap().is_empty());

        // The baseline and unit cannot be edited; the target can.
        request.put(&format!("/api/key-results/{krp}")).json(&json!({ "title": "Users", "target_value": 200, "start_value": 50 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.put(&format!("/api/key-results/{krp}")).json(&json!({ "title": "Users", "target_value": 0 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.put(&format!("/api/key-results/{krp}")).json(&json!({ "title": "Active users", "target_value": 200 })).await.assert_status_ok();
        let okrs: Value = request.get("/api/okrs?period=2026-Q4").await.json();
        assert_eq!(okrs["objectives"][0]["score_basis_points"], 2000, "40 of 200 from a baseline of 0");

        // Edit and delete an objective.
        request.put(&format!("/api/objectives/{measured}")).json(&json!({ "title": "Grow faster", "period": "2026-Q4" })).await.assert_status_ok();
        request.put(&format!("/api/objectives/{measured}")).json(&json!({ "title": "  " })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.delete(&format!("/api/key-results/{krp}")).await.assert_status_ok();
        let okrs: Value = request.get("/api/okrs?period=2026-Q4").await.json();
        assert!(okrs["objectives"].as_array().unwrap().iter().all(|o| o["measured"] == false), "with its key result gone it is unmeasured");
        request.delete(&format!("/api/objectives/{measured}")).await.assert_status_ok();
        request.delete(&format!("/api/objectives/{measured}")).await.assert_status(S::NOT_FOUND);
        let okrs: Value = request.get("/api/okrs?period=2026-Q4").await.json();
        assert_eq!(okrs["objectives"].as_array().unwrap().len(), 1);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn milestones_follow_their_tasks_forecast_slip_and_reopen_honestly() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let other: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Other" }))
            .await
            .json();
        let op = other["pid"].as_str().unwrap().to_string();
        let a = task(&request, &p, "A", Some("2026-10-05"), Some(5)).await;
        let b = task(&request, &p, "B", None, Some(3)).await;
        let stray = task(&request, &op, "Stray", Some("2026-10-05"), Some(1)).await;
        request
            .post("/api/dependencies")
            .json(&json!({ "predecessor_pid": a, "successor_pid": b }))
            .await
            .assert_status_ok();
        let ms = |name: &'static str, due: &'static str| {
            let request = &request;
            let p = p.clone();
            async move {
                let m: Value = request
                    .post(&format!("/api/plans/{p}/milestones"))
                    .json(&json!({ "name": name, "due": due }))
                    .await
                    .json();
                m["pid"].as_str().unwrap().to_string()
            }
        };
        let m1 = ms("Beta", "2026-10-20").await;
        let m2 = ms("Kickoff", "2026-10-06").await;
        let get = |q: &str| {
            let request = &request;
            let url = format!("/api/milestones?{q}");
            async move { request.get(&url).await.json::<Value>() }
        };
        let find = |v: &Value, pid: &str| {
            v["milestones"]
                .as_array()
                .unwrap()
                .iter()
                .find(|m| m["pid"] == pid)
                .cloned()
                .unwrap()
        };

        // Links: the tasks must be this plan's; a linked milestone needs tasks.
        let put_links = |m: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/milestones/{m}/tasks");
            async move { request.put(&url).json(&body).await }
        };
        put_links(&m1, json!({ "task_pids": [stray] }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_links(&m1, json!({ "task_pids": [], "completion_mode": "linked" }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_links(&m1, json!({ "task_pids": [a, a] }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_links(&m1, json!({ "task_pids": [a, b] }))
            .await
            .assert_status_ok();

        // Forecast on the default seven-day calendar: A runs 5-9 Oct, B 10-12 Oct;
        // the due date, 20 Oct, is 8 days later.
        let v = get("as_of=2026-10-05").await;
        let m = find(&v, &m1);
        assert_eq!(m["completion_mode"], "linked");
        assert_eq!(m["schedule"]["forecast"], "2026-10-12");
        assert_eq!(m["schedule"]["slack_days"], 8);
        assert_eq!(
            m["schedule"]["state"], "on_track",
            "more than 5 days of slack"
        );
        assert_eq!(m["open_tasks"], 2);
        let tight = get("as_of=2026-10-05&at_risk_days=8").await;
        assert_eq!(
            find(&tight, &m1)["schedule"]["state"],
            "at_risk",
            "8 days of slack is inside an 8-day window"
        );
        // Move the due date before the forecast: forecast late.
        request
            .put(&format!("/api/plans/{p}/milestones/{m1}"))
            .json(&json!({ "due": "2026-10-11", "reason": "customer asked" }))
            .await
            .assert_status_ok();
        let moved = find(&get("as_of=2026-10-05").await, &m1);
        assert_eq!(moved["schedule"]["state"], "forecast_late");
        assert_eq!(moved["schedule"]["slack_days"], -1);
        // A manual milestone with nothing to forecast from is never "on track".
        let k = find(&v, &m2);
        assert_eq!(k["schedule"]["state"], "no_forecast");
        assert_eq!(k["schedule"]["reason"], "no_linked_tasks");

        // Status against the calendar and the filter; the summary ignores it.
        let late = get("as_of=2026-10-10&status=late").await;
        assert_eq!(late["milestones"].as_array().unwrap().len(), 1);
        assert_eq!(late["milestones"][0]["pid"], m2.as_str());
        assert_eq!(late["summary"]["by_status"]["late"], 1);
        get("status=bogus").await;
        request
            .get("/api/milestones?status=bogus")
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);

        // Completing a linked milestone over open tasks needs a reason.
        let complete = |m: &str, body: Option<Value>| {
            let request = &request;
            let url = format!("/api/plans/{p}/milestones/{m}/complete");
            async move {
                match body {
                    Some(b) => request.post(&url).json(&b).await,
                    None => request.post(&url).await,
                }
            }
        };
        complete(&m1, None)
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        complete(&m1, Some(json!({ "reason": "  " })))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        complete(&m1, Some(json!({ "reason": "scope cut, A and B dropped" })))
            .await
            .assert_status_ok();
        let done = find(&get("as_of=2026-10-05").await, &m1);
        assert_eq!(done["achieved"], true);
        assert_eq!(done["achieved_by"], "manual");
        assert_eq!(done["done_reason"], "scope cut, A and B dropped");
        // The old no-body complete still works for a manual milestone.
        complete(&m2, None).await.assert_status_ok();

        // Reopen needs a reason and a completion to undo.
        let reopen = |m: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/milestones/{m}/reopen");
            async move { request.post(&url).json(&body).await }
        };
        reopen(&m1, json!({ "reason": "" }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        reopen(&m1, json!({ "reason": "scope is back" }))
            .await
            .assert_status_ok();
        reopen(&m1, json!({ "reason": "again" }))
            .await
            .assert_status(S::CONFLICT);
        assert_eq!(find(&get("as_of=2026-10-05").await, &m1)["achieved"], false);

        // Finish the tasks: the milestone is achieved by them, with their date.
        for t in [&a, &b] {
            request
                .patch(&format!("/api/plans/{p}/tasks/{t}"))
                .json(&json!({ "status": "done" }))
                .await
                .assert_status_ok();
        }
        let by_tasks = find(&get("as_of=2026-10-05").await, &m1);
        assert_eq!(by_tasks["achieved"], true);
        assert_eq!(by_tasks["achieved_by"], "tasks");
        assert!(by_tasks["achieved_on"].is_string());
        assert_eq!(by_tasks["schedule"]["reason"], "achieved");
        // It cannot be reopened here; reopen a task.
        reopen(&m1, json!({ "reason": "x" }))
            .await
            .assert_status(S::CONFLICT);

        // Slip history: only real moves are recorded, with their reasons.
        for (due, why) in [
            ("2026-10-09", Some("vendor")),
            ("2026-10-09", None),
            ("2026-10-16", None),
        ] {
            let mut body = json!({ "due": due });
            if let Some(w) = why {
                body["reason"] = json!(w);
            }
            request
                .put(&format!("/api/plans/{p}/milestones/{m2}"))
                .json(&body)
                .await
                .assert_status_ok();
        }
        let detail: Value = request.get(&format!("/api/milestones/{m2}")).await.json();
        let hist = detail["history"].as_array().unwrap();
        assert_eq!(hist.len(), 2, "re-sending the same date is not a move");
        assert_eq!(hist[0]["from"], "2026-10-06");
        assert_eq!(hist[0]["reason"], "vendor");
        assert!(hist[1]["reason"].is_null());
        assert_eq!(detail["slip"]["original_due"], "2026-10-06");
        assert_eq!(detail["slip"]["reschedules"], 2);
        assert_eq!(detail["slip"]["later_moves"], 2);
        assert_eq!(detail["slip"]["net_days"], 10);
        // A milestone that never moved reports no slip.
        assert_eq!(
            find(&get("as_of=2026-10-05").await, &m1)["slip"]["reschedules"],
            1
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn cost_and_joint_risk_are_seeded_per_currency_and_share_one_event() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let a = task(&request, &p, "A", Some("2026-10-05"), Some(5)).await;
        let b = task(&request, &p, "B", None, Some(3)).await;
        request.post("/api/dependencies").json(&json!({ "predecessor_pid": a, "successor_pid": b })).await.assert_status_ok();
        let line = |cat: &'static str, cur: &'static str, planned: i64| {
            let request = &request;
            let p = p.clone();
            async move {
                let l: Value = request.post(&format!("/api/plans/{p}/budget-lines")).json(&json!({
                    "category": cat, "description": cat, "currency": cur, "planned_minor": planned })).await.json();
                l["pid"].as_str().unwrap().to_string()
            }
        };
        // No lines yet: a clear refusal.
        request.get(&format!("/api/plans/{p}/cost-risk")).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let l1 = line("capex", "GBP", 10_000).await;
        let _l2 = line("opex", "GBP", 5_000).await;
        let _usd = line("capex", "USD", 700).await;

        // Estimates must sit around the planned amount.
        let put_est = |l: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/budget-lines/{l}/estimate");
            async move { request.put(&url).json(&body).await }
        };
        put_est(&l1, json!({ "low_minor": 12_000 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        put_est(&l1, json!({ "high_minor": 9_000 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        put_est(&l1, json!({ "low_minor": 8_000, "high_minor": 16_000 })).await.assert_status_ok();

        let url = |q: &str| format!("/api/plans/{p}/cost-risk?{q}");
        let r: Value = request.get(&url("seed=5")).await.json();
        assert_eq!(r["assumptions"]["correlation"], "none");
        assert!(r["assumptions"]["independence"].as_str().unwrap().contains("independent"));
        let cur = |v: &Value, c: &str| v["currencies"].as_array().unwrap().iter().find(|x| x["currency"] == c).cloned().unwrap();
        // Currencies are separate blocks, never added.
        assert_eq!(r["currencies"].as_array().unwrap().len(), 2);
        assert_eq!(cur(&r, "GBP")["budget_minor"], 15_000);
        assert_eq!(cur(&r, "USD")["budget_minor"], 700);
        // The un-estimated GBP line uses the marked fallback.
        let lines = cur(&r, "GBP")["lines"].as_array().unwrap().clone();
        assert_eq!(lines.iter().filter(|l| l["source"] == "fallback").count(), 1);
        // Seeded.
        assert_eq!(r, request.get(&url("seed=5")).await.json::<Value>());
        // No uncertainty: every percentile is the budget.
        put_est(&l1, json!({ "low_minor": 10_000, "high_minor": 10_000 })).await.assert_status_ok();
        let flat: Value = request.get(&url("fallback_band_percent=0")).await.json();
        let g = cur(&flat, "GBP");
        assert!(g["percentiles"].as_array().unwrap().iter().all(|x| x["minor"] == 15_000));
        assert_eq!(g["budget_confidence_bp"], 10_000);
        request.get(&url("iterations=0")).await.assert_status(S::UNPROCESSABLE_ENTITY);

        // A risk: probability is shared; cost needs a currency.
        let risk: Value = request.post(&format!("/api/plans/{p}/risks")).json(&json!({ "title": "Vendor", "probability": 3, "impact": 3 })).await.json();
        let rp = risk["pid"].as_str().unwrap().to_string();
        let noprob: Value = request.post(&format!("/api/plans/{p}/risks")).json(&json!({ "title": "Unquantified", "probability": 2, "impact": 2 })).await.json();
        let np = noprob["pid"].as_str().unwrap().to_string();
        let put_cost = |r: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/risks/{r}/cost-impact");
            async move { request.put(&url).json(&body).await }
        };
        put_cost(&rp, json!({ "cost_impact_minor": 4_000 })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        put_cost(&rp, json!({ "cost_impact_minor": 0, "cost_currency": "GBP" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        put_cost(&rp, json!({ "cost_impact_minor": 4_000, "cost_currency": "gbp", "probability_percent": 100 })).await.assert_status_ok();
        put_cost(&np, json!({ "cost_impact_minor": 1_000, "cost_currency": "GBP" })).await.assert_status_ok();
        // The inputs come back from the lists, so a form can show what is stored.
        let listed: Value = request.get(&format!("/api/plans/{p}/risks")).await.json();
        let mine = listed.as_array().unwrap().iter().find(|r| r["pid"] == rp.as_str()).cloned().unwrap();
        assert_eq!(mine["cost_impact_minor"], 4_000);
        assert_eq!(mine["cost_currency"], "GBP");
        assert_eq!(mine["probability_percent"], 100);
        assert!(mine["task_pid"].is_null() && mine["schedule_impact_days"].is_null());
        let lines: Value = request.get(&format!("/api/plans/{p}/budget-lines")).await.json();
        let first = lines["lines"].as_array().unwrap().iter().find(|l| l["pid"] == l1.as_str()).cloned().unwrap();
        assert_eq!(first["low_minor"], 10_000);
        assert_eq!(first["high_minor"], 10_000);
        let ev: Value = request.get(&url("fallback_band_percent=0&seed=2")).await.json();
        assert_eq!(ev["events"]["used"].as_array().unwrap().len(), 1);
        assert_eq!(ev["events"]["skipped"][0]["risk_pid"], np.as_str());
        assert_eq!(ev["events"]["skipped"][0]["reason"], "no_probability");
        let g = cur(&ev, "GBP");
        assert!(g["percentiles"].as_array().unwrap().iter().all(|x| x["minor"] == 19_000), "15 000 + a certain 4 000");
        assert_eq!(g["budget_confidence_bp"], 0);

        // Portfolio: the same plan appears, with its mean beside its budget.
        let all: Value = request.get("/api/cost-risk?fallback_band_percent=0").await.json();
        assert_eq!(all["scope"]["kind"], "portfolio");
        assert!(cur(&all, "GBP")["plans"].as_array().unwrap().iter().any(|x| x["plan"] == p.as_str() && x["name"] == "Apollo"));

        // Joint: several currencies need a choice; then the shared event drives both.
        request.get(&format!("/api/plans/{p}/joint-risk")).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.put(&format!("/api/plans/{p}/risks/{rp}/schedule-impact")).json(&json!({ "task_pid": a, "impact_days": 4, "probability_percent": 100 })).await.assert_status_ok();
        let jurl = |q: &str| format!("/api/plans/{p}/joint-risk?currency=GBP&{q}");
        let j: Value = request.get(&jurl("fallback_band_percent=0&delay_cost_per_day_minor=500&seed=3")).await.json();
        assert_eq!(j["currency"], "GBP");
        assert_eq!(j["other_currencies_not_simulated"][0], "USD");
        assert_eq!(j["assumptions"]["shared_events"].as_array().unwrap().len(), 1);
        // Certain: 4 days late at 500 a day, plus the 4 000 risk, on 15 000.
        assert!(j["cost"]["percentiles"].as_array().unwrap().iter().all(|x| x["minor"] == 21_000));
        assert_eq!(j["confidence"]["joint_bp"], 0);
        assert_eq!(j["cost"]["mean_delay_cost_minor"], 2_000);
        assert_eq!(j, request.get(&jurl("fallback_band_percent=0&delay_cost_per_day_minor=500&seed=3")).await.json::<Value>());
        let pts = j["points"].as_array().unwrap();
        assert!(!pts.is_empty() && pts.len() <= 300);
        request
            .get(&format!("/api/plans/{p}/joint-risk?currency=EUR"))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        request.get(&format!("/api/plans/{p}/joint-risk?currency=GBP&delay_cost_per_day_minor=-5")).await.assert_status(S::UNPROCESSABLE_ENTITY);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn assignments_follow_their_rule_and_work_is_spread_exactly_over_working_days() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let who = "person:00000000-0000-4000-8000-0000000000a1";
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        // A Monday-to-Friday calendar, so weekends are not working days.
        request
            .put(&format!("/api/plans/{p}/calendar"))
            .json(&json!({ "weekdays": "1111100" }))
            .await
            .assert_status_ok();
        let mk = |title: &'static str,
                  assignee: Option<&'static str>,
                  start: Option<&'static str>,
                  days: i64| {
            let request = &request;
            let p = p.clone();
            async move {
                let mut body = json!({ "title": title, "duration_days": days });
                if let Some(a) = assignee {
                    body["assignee_ref"] = json!(a);
                }
                if let Some(s) = start {
                    body["start_date"] = json!(s);
                }
                let t: Value = request
                    .post(&format!("/api/plans/{p}/tasks"))
                    .json(&body)
                    .await
                    .json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let put_asg = |t: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/tasks/{t}/assignment");
            async move { request.put(&url).json(&body).await }
        };
        let nobody = mk("Unassigned", None, Some("2026-10-05"), 5).await;
        put_asg(&nobody, json!({ "units_percent": 100 }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);

        // A first assignment: duration 5 stored, units 100 given, work derived.
        let a = mk("Build", Some(who), Some("2026-10-05"), 8).await;
        let r: Value = put_asg(&a, json!({ "units_percent": 100 })).await.json();
        assert_eq!(
            (
                r["work_minutes"].as_i64(),
                r["derived"].as_str(),
                r["exact"].as_bool()
            ),
            (Some(3840), Some("work"), Some(true)),
            "8 days at 100%"
        );
        put_asg(&a, json!({ "units_percent": 50, "work_minutes": 100 }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_asg(&a, json!({ "rule": "sideways" }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_asg(&a, json!({}))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);

        // fixed_units (the default): raising work moves duration, never units.
        let r: Value = put_asg(&a, json!({ "work_minutes": 4800 })).await.json();
        assert_eq!(
            (
                r["duration_days"].as_i64(),
                r["units_percent"].as_i64(),
                r["work_minutes"].as_i64(),
                r["derived"].as_str()
            ),
            (Some(10), Some(100), Some(4800), Some("duration"))
        );
        // fixed_duration: changing work moves UNITS and never duration.
        let r: Value = put_asg(
            &a,
            json!({ "rule": "fixed_duration", "work_minutes": 2400 }),
        )
        .await
        .json();
        assert_eq!(
            (
                r["duration_days"].as_i64(),
                r["units_percent"].as_i64(),
                r["derived"].as_str()
            ),
            (Some(10), Some(50), Some("units"))
        );
        // Units above what the assignee can give are flagged, not clamped.
        let r: Value = put_asg(&a, json!({ "units_percent": 300 })).await.json();
        assert_eq!(r["units_percent"], 300);
        assert_eq!(r["capacity"]["checked"], true);
        assert_eq!(
            r["capacity"]["over_capacity"], true,
            "300% against a default capacity of 100%"
        );
        assert!(r["capacity"]["days_over"].as_i64().unwrap() > 0);

        // The schedule editor applies the same rule when the duration changes (fixed_work: units move).
        put_asg(&a, json!({ "rule": "fixed_work", "units_percent": 100 }))
            .await
            .assert_status_ok();
        let work_now = put_asg(&a, json!({ "rule": "fixed_work" }))
            .await
            .json::<Value>()["work_minutes"]
            .as_i64()
            .unwrap();
        let edited: Value = request
            .put(&format!("/api/plans/{p}/tasks/{a}/schedule"))
            .json(&json!({ "start_date": "2026-10-05", "duration_days": 20 }))
            .await
            .json();
        assert_eq!(edited["assignment"]["derived"], "units");
        assert_eq!(
            edited["assignment"]["work_minutes"], work_now,
            "fixed work is kept"
        );
        assert_eq!(
            edited["assignment"]["units_percent"], 150,
            "14 400 minutes over 20 days is 150%"
        );
        // A task that carries work cannot become a milestone.
        request
            .put(&format!("/api/plans/{p}/tasks/{a}/schedule"))
            .json(&json!({ "start_date": "2026-10-05", "duration_days": 0 }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);

        // ── contours and the work profile ──
        let b = mk("Test", Some(who), Some("2026-10-05"), 8).await;
        put_asg(&b, json!({ "units_percent": 100 }))
            .await
            .assert_status_ok(); // 3 840 minutes over 8 working days
        let put_contour = |t: &str, body: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/tasks/{t}/contour");
            async move { request.put(&url).json(&body).await }
        };
        put_contour(&b, json!({ "contour": "wavy" }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_contour(&b, json!({ "contour": "custom", "custom": [40, 50] }))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
        put_contour(&b, json!({ "contour": "front_loaded" }))
            .await
            .assert_status_ok();

        let prof: Value = request
            .get(&format!(
                "/api/plans/{p}/work-profile?period=day&from=2026-10-05&to=2026-10-20"
            ))
            .await
            .json();
        let task_b = prof["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["pid"] == b.as_str())
            .cloned()
            .unwrap();
        let days = task_b["buckets"].as_array().unwrap();
        let sum: i64 = days.iter().map(|d| d["minutes"].as_i64().unwrap()).sum();
        assert_eq!(sum, 3840, "the buckets sum to the work exactly");
        // 5-9 and 12-14 Oct are the eight working days; the weekend between them carries nothing.
        let on = |date: &str| {
            days.iter()
                .find(|d| d["from"] == date)
                .map(|d| d["minutes"].as_i64().unwrap())
                .unwrap()
        };
        assert_eq!(
            (on("2026-10-10"), on("2026-10-11")),
            (0, 0),
            "no work on the weekend"
        );
        assert!(
            on("2026-10-05") > on("2026-10-14"),
            "front-loaded: more first than last"
        );
        // The plan's totals are the sum of its tasks'.
        let total: i64 = prof["totals"]["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["minutes"].as_i64().unwrap())
            .sum();
        let tasks_sum: i64 = prof["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|t| {
                t["buckets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|d| d["minutes"].as_i64().unwrap())
            })
            .sum();
        assert_eq!(total, tasks_sum);
        assert_eq!(prof["totals"]["unplaced_work_minutes"], 0);
        // Another contour keeps the total unchanged.
        put_contour(&b, json!({ "contour": "custom", "custom": [10, 20, 70] }))
            .await
            .assert_status_ok();
        let prof2: Value = request
            .get(&format!(
                "/api/plans/{p}/work-profile?period=day&from=2026-10-05&to=2026-10-20"
            ))
            .await
            .json();
        let b2 = prof2["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["pid"] == b.as_str())
            .cloned()
            .unwrap();
        assert_eq!(
            b2["buckets"]
                .as_array()
                .unwrap()
                .iter()
                .map(|d| d["minutes"].as_i64().unwrap())
                .sum::<i64>(),
            3840
        );
        assert_ne!(b2["buckets"], task_b["buckets"], "the shape changed");

        // Work that cannot be placed is listed and counted, never dropped.
        let loose = mk("Loose", Some(who), None, 3).await;
        put_asg(&loose, json!({ "units_percent": 100 }))
            .await
            .assert_status_ok();
        let prof3: Value = request
            .get(&format!("/api/plans/{p}/work-profile?period=week"))
            .await
            .json();
        let un = prof3["unplaced"].as_array().unwrap();
        assert_eq!(un.len(), 1);
        assert_eq!(un[0]["reason"], "unscheduled");
        assert_eq!(prof3["totals"]["unplaced_work_minutes"], 1440);

        // The portfolio is the sum of the plans; a plan with no assignments says so.
        let empty: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Empty" }))
            .await
            .json();
        let ep = empty["pid"].as_str().unwrap().to_string();
        let e: Value = request
            .get(&format!("/api/plans/{ep}/work-profile"))
            .await
            .json();
        assert_eq!(e["reason"], "no_assignments");
        assert!(e["totals"]["buckets"].is_null());
        let all: Value = request
            .get("/api/work-profile?period=week&from=2026-10-05&to=2026-12-28")
            .await
            .json();
        let ours = all["plans"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["plan_pid"] == p.as_str())
            .cloned()
            .unwrap();
        let ours_sum: i64 = ours["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["minutes"].as_i64().unwrap())
            .sum();
        let plan_total: i64 = request
            .get(&format!(
                "/api/plans/{p}/work-profile?period=week&from=2026-10-05&to=2026-12-28"
            ))
            .await
            .json::<Value>()["totals"]["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["minutes"].as_i64().unwrap())
            .sum();
        assert_eq!(
            ours_sum, plan_total,
            "the portfolio's figure for a plan is the plan's own"
        );
        let grand: i64 = all["totals"]["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["minutes"].as_i64().unwrap())
            .sum();
        let by_plan: i64 = all["plans"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|x| {
                x["buckets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|d| d["minutes"].as_i64().unwrap())
            })
            .sum();
        assert_eq!(
            grand, by_plan,
            "the portfolio total is the sum of its plans"
        );
        request
            .get(&format!("/api/plans/{p}/work-profile?period=year"))
            .await
            .assert_status(S::UNPROCESSABLE_ENTITY);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn cost_rates_price_work_by_date_and_never_rewrite_an_actual() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let who = "person:00000000-0000-4000-8000-0000000000b1";
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Costed" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        request.put(&format!("/api/plans/{p}/calendar")).json(&json!({ "weekdays": "1111100" })).await.assert_status_ok();

        // Tables and rates validate; a date carries one rate.
        request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "currency": "GBP" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "person_ref": who, "currency": "pounds" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let t: Value = request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "person_ref": who, "currency": "gbp" })).await.json();
        let table = t["pid"].as_str().unwrap().to_string();
        request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "person_ref": who, "currency": "GBP" })).await.assert_status(S::CONFLICT);
        let add = |from: &str, rate: i64| {
            let request = &request;
            let url = format!("/api/cost-rate-tables/{table}/rates");
            let body = json!({ "effective_from": from, "per_hour_minor": rate });
            async move { request.post(&url).json(&body).await }
        };
        add("2026-01-01", 5000).await.assert_status_ok();
        add("2026-10-07", 6000).await.assert_status_ok();
        add("2026-10-07", 7000).await.assert_status(S::CONFLICT);
        add("2026-10-08", -1).await.assert_status(S::UNPROCESSABLE_ENTITY);

        let look = |date: &str| {
            let request = &request;
            let url = format!("/api/cost-rates/lookup?person_ref={who}&date={date}");
            async move { request.get(&url).await.json::<Value>() }
        };
        assert_eq!(look("2026-10-06").await["rate"]["per_hour_minor"], 5000);
        assert_eq!(look("2026-10-07").await["rate"]["per_hour_minor"], 6000);
        let before = look("2025-12-31").await;
        assert!(before["rate"].is_null());
        assert_eq!(before["reason"], "no_rate");

        // Planned labour: 3 days at 100% from Mon 5 Oct; Wed 7 Oct is at the new rate.
        let task: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({
            "title": "Build", "assignee_ref": who, "start_date": "2026-10-05", "duration_days": 3 })).await.json();
        let tp = task["pid"].as_str().unwrap().to_string();
        request.put(&format!("/api/plans/{p}/tasks/{tp}/assignment")).json(&json!({ "units_percent": 100 })).await.assert_status_ok();
        request.post(&format!("/api/plans/{p}/tasks/{tp}/costs")).json(&json!({
            "kind": "material", "description": "Timber", "amount_minor": 100, "currency": "gbp", "accrual": "prorated" })).await.assert_status_ok();
        request.post(&format!("/api/plans/{p}/tasks/{tp}/costs")).json(&json!({
            "kind": "fixed", "description": "x", "amount_minor": 0, "currency": "GBP", "accrual": "start" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let plan_cost: Value = request.get(&format!("/api/plans/{p}/cost-plan?period=day")).await.json();
        let gbp = &plan_cost["currencies"][0];
        assert_eq!(gbp["currency"], "GBP");
        assert_eq!(gbp["labour"]["total_minor"], 2 * 40_000 + 48_000, "two days at 50.00, one at 60.00");
        assert_eq!(gbp["fixed"]["total_minor"], 100, "the prorated expense sums exactly");
        assert_eq!(gbp["total"]["total_minor"], 128_100);
        assert!(plan_cost["unrated"].as_array().unwrap().is_empty());

        // Time entries are priced once, with their rate stored; a later rate does not rewrite them.
        let entry: Value = request.post(&format!("/api/plans/{p}/time-entries")).json(&json!({
            "task_pid": tp, "actor_ref": who, "spent_on": "2026-10-06", "minutes": 120, "category": "work" })).await.json();
        let _ = entry;
        let rec: Value = request.post(&format!("/api/plans/{p}/labour-costs/record")).await.json();
        assert_eq!(rec["recorded"], 1);
        add("2026-10-06", 9000).await.assert_status_ok(); // a rate backdated onto the entry's day
        let again: Value = request.post(&format!("/api/plans/{p}/labour-costs/record")).await.json();
        assert_eq!((again["recorded"].as_i64(), again["already_recorded"].as_i64()), (Some(0), Some(1)));
        let actuals: Value = request.get(&format!("/api/plans/{p}/labour-costs")).await.json();
        assert_eq!(actuals["entries"][0]["per_hour_minor"], 5000, "the rate it was priced at");
        assert_eq!(actuals["currencies"][0]["cost_minor"], 10_000, "2 h at 50.00");
        assert_eq!(actuals["unpriced"]["entries"], 0);

        // Someone with no finance attribute does not see any of it exist.
        let nobody = "person:00000000-0000-4000-8000-0000000000b2";
        let r: Value = request.get(&format!("/api/cost-rates/lookup?person_ref={nobody}&date=2026-10-06")).await.json();
        assert!(r["rate"].is_null(), "an unrated person has no rate, not zero");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_announcement_feed_orders_hides_scheduled_and_expired_and_retracts() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let post = |body: Value| {
            let request = &request;
            async move { request.post("/api/announcements").json(&body).await }
        };
        post(json!({ "title": "", "body": "x" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        post(json!({ "title": "t", "body": "x", "severity": "loud" })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let info: Value = post(json!({ "title": "Lunch", "body": "Pizza at noon" })).await.json();
        let urgent: Value = post(json!({ "title": "Outage", "body": "Down", "severity": "urgent" })).await.json();
        let pinned: Value = post(json!({ "title": "Welcome", "body": "Hello", "pinned": true })).await.json();
        post(json!({ "title": "Later", "body": "x", "publish_at": "2099-01-01T00:00:00Z" })).await.assert_status_ok();
        post(json!({ "title": "Gone", "body": "x", "publish_at": "2020-01-01T00:00:00Z", "expires_at": "2020-01-02T00:00:00Z" })).await.assert_status_ok();
        post(json!({ "title": "Backwards", "body": "x", "publish_at": "2030-01-02T00:00:00Z", "expires_at": "2030-01-01T00:00:00Z" })).await.assert_status(S::UNPROCESSABLE_ENTITY);

        let feed: Value = request.get("/api/announcements").await.json();
        let titles: Vec<&str> = feed["announcements"].as_array().unwrap().iter().map(|a| a["title"].as_str().unwrap()).collect();
        assert_eq!(titles, vec!["Welcome", "Outage", "Lunch"], "pinned, then urgent, then info; scheduled and expired are hidden");
        let all: Value = request.get("/api/announcements?include=all").await.json();
        let states: Vec<&str> = all["announcements"].as_array().unwrap().iter().map(|a| a["state"].as_str().unwrap()).collect();
        assert!(states.contains(&"scheduled") && states.contains(&"expired"));
        let one: Value = request.get("/api/announcements?limit=1").await.json();
        assert_eq!((one["announcements"].as_array().unwrap().len(), one["truncated"].as_bool(), one["total"].as_i64()), (1, Some(true), Some(3)));

        let pid = urgent["pid"].as_str().unwrap();
        let edited: Value = request.put(&format!("/api/announcements/{pid}")).json(&json!({ "title": "Outage over", "body": "Fixed", "severity": "info" })).await.json();
        assert_eq!(edited["title"], "Outage over");
        request.delete(&format!("/api/announcements/{}", info["pid"].as_str().unwrap())).await.assert_status_ok();
        request.delete(&format!("/api/announcements/{}", info["pid"].as_str().unwrap())).await.assert_status(S::NOT_FOUND);
        let feed: Value = request.get("/api/announcements").await.json();
        let titles: Vec<&str> = feed["announcements"].as_array().unwrap().iter().map(|a| a["title"].as_str().unwrap()).collect();
        assert_eq!(titles, vec!["Welcome", "Outage over"]);
        let _ = pinned;
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn earned_value_can_take_its_actual_cost_from_recorded_labour() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let who = "person:00000000-0000-4000-8000-0000000000d1";
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let today = chrono::Utc::now().date_naive();
        let day = |n: i64| (today + chrono::Duration::days(n)).to_string();
        request.post(&format!("/api/plans/{p}/budget-baselines")).json(&json!({ "currency": "GBP", "periods": [
            { "period_start": day(-9), "period_end": day(0), "planned_minor": 1000 },
            { "period_start": day(1), "period_end": day(10), "planned_minor": 3000 } ] })).await.assert_status_ok();
        let a = task(&request, &p, "A", None, None).await;
        task(&request, &p, "B", None, None).await;
        request.patch(&format!("/api/plans/{p}/tasks/{a}")).json(&json!({ "status": "done" })).await.assert_status_ok();
        let line: Value = request.post(&format!("/api/plans/{p}/budget-lines")).json(&json!({
            "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": 4000 })).await.json();
        request.post(&format!("/api/plans/{p}/budget-lines/{}/actual", line["pid"].as_str().unwrap())).json(&json!({ "amount_minor": 2500 })).await.assert_status_ok();
        let ac = |q: &'static str| {
            let (request, url) = (&request, format!("/api/plans/{p}/earned-value{q}"));
            async move { request.get(&url).await.json::<Value>() }
        };

        // The default is unchanged and says where it came from.
        let d = ac("").await;
        assert_eq!((d["ac_source"].as_str(), d["blocks"][0]["ac_minor"].as_i64()), (Some("budget_line_actuals"), Some(2500)));
        // Asked for labour before any is recorded: no actual cost (null), not zero.
        let none = ac("?ac=labour").await;
        assert_eq!(none["ac_source"], "labour_costs");
        assert!(none["blocks"][0]["ac_minor"].is_null(), "{none}");
        assert!(none["blocks"][0]["cpi"].is_null() || none["blocks"][0]["cpi"]["bp"].is_null());

        // Two hours at 50.00 recorded: 10 000.
        let t: Value = request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "person_ref": who, "currency": "GBP" })).await.json();
        request.post(&format!("/api/cost-rate-tables/{}/rates", t["pid"].as_str().unwrap())).json(&json!({ "effective_from": "2026-01-01", "per_hour_minor": 5000 })).await.assert_status_ok();
        request.post(&format!("/api/plans/{p}/time-entries")).json(&json!({
            "task_pid": a, "actor_ref": who, "spent_on": today.to_string(), "minutes": 120, "category": "work" })).await.assert_status_ok();
        request.post(&format!("/api/plans/{p}/labour-costs/record")).await.assert_status_ok();
        assert_eq!(ac("?ac=labour").await["blocks"][0]["ac_minor"], 10_000);
        assert_eq!(ac("?ac=both").await["blocks"][0]["ac_minor"], 12_500, "budget lines 2 500 + labour 10 000");
        assert_eq!(ac("").await["blocks"][0]["ac_minor"], 2500, "the default still ignores labour");
        assert_eq!(ac("?ac=both").await["ac_source"], "budget_line_actuals_plus_labour_costs");
        assert_eq!(request.get(&format!("/api/plans/{p}/earned-value?ac=sideways")).await.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn the_what_if_sandbox_previews_without_writing_and_commits_exactly_what_it_showed() {
    use project_portfolio_management_service::models::_entities::{audit_logs, plan_dependencies};
    use sea_orm::{EntityTrait, PaginatorTrait};
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        use axum::http::StatusCode as S;
        let who = "person:00000000-0000-4000-8000-0000000000e1";
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Sandbox" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        request.put(&format!("/api/plans/{p}/calendar")).json(&json!({ "weekdays": "1111100" })).await.assert_status_ok();
        let a: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({
            "title": "A", "start_date": "2026-10-05", "duration_days": 3, "assignee_ref": who })).await.json();
        let a = a["pid"].as_str().unwrap().to_string();
        let b = task(&request, &p, "B", Some("2026-10-05"), Some(2)).await;
        let c = task(&request, &p, "C", Some("2026-10-05"), Some(1)).await;
        let link: Value = request.post("/api/dependencies").json(&json!({ "predecessor_pid": a, "successor_pid": b, "lag_days": 0 })).await.json();
        let link = link["pid"].as_str().unwrap().to_string();
        // A carries work at a rate, so a longer A costs more.
        request.put(&format!("/api/plans/{p}/tasks/{a}/assignment")).json(&json!({ "units_percent": 100 })).await.assert_status_ok();
        let t: Value = request.post("/api/cost-rate-tables").json(&json!({ "name": "standard", "person_ref": who, "currency": "GBP" })).await.json();
        request.post(&format!("/api/cost-rate-tables/{}/rates", t["pid"].as_str().unwrap())).json(&json!({ "effective_from": "2026-01-01", "per_hour_minor": 5000 })).await.assert_status_ok();

        let preview = |edits: Value| {
            let request = &request;
            let url = format!("/api/plans/{p}/sandbox/preview");
            async move { request.post(&url).json(&json!({ "edits": edits })).await }
        };
        let audit_count = || {
            let db = ctx.db.clone();
            async move { audit_logs::Entity::find().count(&db).await.unwrap() }
        };
        let cpm = || {
            let request = &request;
            let url = format!("/api/plans/{p}/cpm");
            async move { request.get(&url).await.json::<Value>() }
        };
        let start_of = |t: &Value, pid: &str| t["nodes"].as_array().unwrap().iter().find(|n| n["pid"] == pid).cloned().unwrap();

        // ── A preview writes nothing, and the same edits give the same answer ──
        let before_cpm = cpm().await;
        let audits = audit_count().await;
        let edits = json!([{ "op": "duration", "task_pid": a, "duration_days": 6 }]);
        let r1 = preview(edits.clone()).await;
        r1.assert_status_ok();
        let etag = r1.header("etag").to_str().unwrap().to_string();
        let r1: Value = r1.json();
        let r2: Value = preview(edits.clone()).await.json();
        assert_eq!(r1, r2, "the same input gives the same preview");
        assert_eq!(r1["plan_etag"], etag.as_str());
        assert_eq!(cpm().await, before_cpm, "the plan is untouched");
        assert_eq!(audit_count().await, audits, "a preview leaves no audit entry");
        let dep_rows = plan_dependencies::Entity::find().count(&ctx.db).await.unwrap();
        assert_eq!(dep_rows, 1);

        // What it shows: A is longer, B moves later, the finish moves, and A's work (so its cost) doubles.
        assert_eq!(r1["changes"].as_array().unwrap().len(), 1);
        let moved: Vec<&str> = r1["moved"].as_array().unwrap().iter().map(|m| m["pid"].as_str().unwrap()).collect();
        assert!(moved.contains(&b.as_str()) && !moved.contains(&c.as_str()));
        assert_ne!(r1["finish"]["before"], r1["finish"]["after"]);
        assert!(r1["critical_path"]["after"].as_array().unwrap().iter().any(|x| x == a.as_str()));
        assert_eq!(r1["cost_impact"]["available"], true, "{r1}");
        let gbp = &r1["cost_impact"]["labour"][0];
        assert_eq!((gbp["currency"].as_str(), gbp["before_minor"].as_i64(), gbp["after_minor"].as_i64()), (Some("GBP"), Some(120_000), Some(240_000)), "3 d then 6 d at 8 h x 50.00");
        assert_eq!(r1["assignments_recomputed"][0]["work_minutes"], 2880);

        // Edits are refused before anything is shown: bad input, unknown task, cycle, task relation.
        preview(json!([])).await.assert_status(S::UNPROCESSABLE_ENTITY);
        preview(json!([{ "op": "duration", "task_pid": a, "duration_days": -2 }])).await.assert_status(S::UNPROCESSABLE_ENTITY);
        preview(json!([{ "op": "move", "task_pid": "00000000-0000-4000-8000-00000000ffff", "start_date": "2026-10-07" }])).await.assert_status(S::UNPROCESSABLE_ENTITY);
        preview(json!([{ "op": "add_link", "predecessor_pid": b, "successor_pid": a }])).await.assert_status(S::UNPROCESSABLE_ENTITY);
        preview(json!([{ "op": "constraint", "task_pid": c, "constraint_kind": "mso", "constraint_date": "2026-10-07" }])).await.assert_status(S::UNPROCESSABLE_ENTITY);
        assert_eq!(audit_count().await, audits, "refusals write nothing either");

        // ── A commit needs the preview's etag ──
        let commit = |edits: Value, etag: Option<&str>| {
            let request = &request;
            let url = format!("/api/plans/{p}/sandbox/commit");
            let etag = etag.map(str::to_string);
            async move {
                let mut rb = request.post(&url).json(&json!({ "edits": edits, "reason": "what if" }));
                if let Some(e) = etag {
                    rb = rb.add_header(axum::http::header::IF_MATCH, e.parse::<axum::http::HeaderValue>().unwrap());
                }
                rb.await
            }
        };
        assert_eq!(commit(edits.clone(), None).await.status_code(), 428);
        assert_eq!(commit(edits.clone(), Some("W/\"0000000000000000\"")).await.status_code(), 409);
        assert_eq!(audit_count().await, audits);
        // A change made after the preview makes the commit stale: refused, not merged.
        request.put(&format!("/api/plans/{p}/tasks/{c}/schedule")).json(&json!({ "start_date": "2026-10-06", "duration_days": 1 })).await.assert_status_ok();
        assert_eq!(commit(edits.clone(), Some(&etag)).await.status_code(), 409, "the plan changed since the preview");
        assert_eq!(start_of(&cpm().await, &a)["early_finish"], start_of(&before_cpm, &a)["early_finish"], "A is unchanged");

        // ── Commit applies exactly the previewed result, one audit entry per changed item ──
        let fresh = preview(edits.clone()).await;
        let etag = fresh.header("etag").to_str().unwrap().to_string();
        let shown: Value = fresh.json();
        let audits = audit_count().await;
        let done = commit(edits.clone(), Some(&etag)).await;
        done.assert_status_ok();
        let done: Value = done.json();
        assert_eq!((done["committed"].as_bool(), done["audit_entries"].as_i64()), (Some(true), Some(1)));
        assert_eq!(audit_count().await, audits + 1);
        let after_cpm = cpm().await;
        for d in shown["tasks"].as_array().unwrap() {
            let node = start_of(&after_cpm, d["pid"].as_str().unwrap());
            assert_eq!(node["early_start"], d["after"]["early_start"], "{d}");
            assert_eq!(node["early_finish"], d["after"]["early_finish"], "{d}");
            assert_eq!(node["total_float"], d["after"]["total_float"], "{d}");
        }
        assert_eq!(after_cpm["project_finish"], shown["finish"]["after"]);
        assert_eq!(start_of(&after_cpm, &a)["work_minutes"], 2880, "the recomputed work was stored");
        // The same commit again is stale (the plan is now what the preview said it would become).
        assert_eq!(commit(edits.clone(), Some(&etag)).await.status_code(), 409);

        // ── Links: remove one, add one; one audit entry each; a discarded preview of them wrote nothing ──
        let link_edits = json!([
            { "op": "remove_link", "link_pid": link },
            { "op": "add_link", "predecessor_pid": c, "successor_pid": b, "kind": "ss", "lag_days": 1 },
        ]);
        let pv = preview(link_edits.clone()).await;
        let etag = pv.header("etag").to_str().unwrap().to_string();
        let pv: Value = pv.json();
        assert_eq!(pv["changes"].as_array().unwrap().len(), 2);
        assert_eq!(plan_dependencies::Entity::find().count(&ctx.db).await.unwrap(), 1, "still the original link");
        let audits = audit_count().await;
        commit(link_edits.clone(), Some(&etag)).await.assert_status_ok();
        assert_eq!(audit_count().await, audits + 2);
        let deps = plan_dependencies::Entity::find().all(&ctx.db).await.unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!((deps[0].kind.as_str(), deps[0].lag_days, deps[0].predecessor_pid.to_string()), ("ss", 1, c.clone()));
    })
    .await;
}
