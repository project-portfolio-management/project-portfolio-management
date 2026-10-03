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
