//! The resource breakdown's drill-down: the open tasks of the people in a role.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_role_lists_what_its_people_are_working_on() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let ann = format!("person:{}", uuid::Uuid::new_v4());
        let bob = format!("person:{}", uuid::Uuid::new_v4());
        let cat = format!("person:{}", uuid::Uuid::new_v4());
        for (who, role) in [
            (&ann, "role-drill"),
            (&bob, "role-drill"),
            (&cat, "other-drill"),
        ] {
            request
                .put("/api/resource-roles")
                .json(&json!({ "person_ref": who, "role": role }))
                .await
                .assert_status_ok();
        }
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Drill-down" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let mut done_task = String::new();
        for (title, who) in [
            ("Ann one", &ann),
            ("Ann two", &ann),
            ("Cat one", &cat),
            ("Ann finished", &ann),
        ] {
            let task: Value = request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&json!({ "title": title, "assignee_ref": who }))
                .await
                .json();
            if title == "Ann finished" {
                done_task = task["pid"].as_str().expect("pid").to_string();
            }
        }
        for status in ["in_progress", "done"] {
            request
                .patch(&format!("/api/plans/{p}/tasks/{done_task}"))
                .json(&json!({ "status": status }))
                .await
                .assert_status_ok();
        }

        let answer: Value = request
            .get("/api/resource-plan/role-tasks?role=role-drill")
            .await
            .json();
        let people = answer["people"].as_array().expect("people");
        assert_eq!(
            people.len(),
            2,
            "both people in the role, even Bob with nothing"
        );
        let titles = |who: &str| -> Vec<String> {
            people
                .iter()
                .find(|x| x["person_ref"] == who)
                .and_then(|x| x["tasks"].as_array())
                .expect("person")
                .iter()
                .map(|t| t["title"].as_str().expect("title").to_string())
                .collect()
        };
        assert_eq!(
            titles(&ann),
            ["Ann one", "Ann two"],
            "open tasks only, not Cat's, not the finished one"
        );
        assert!(titles(&bob).is_empty());
        assert_eq!(answer["tasks"], 2);
        assert_eq!(answer["omitted_unreadable"], 0);
        let ann_entry = people
            .iter()
            .find(|x| x["person_ref"] == ann.as_str())
            .expect("Ann");
        assert_eq!(ann_entry["tasks"][0]["plan_name"], "Drill-down");

        let none: Value = request
            .get("/api/resource-plan/role-tasks?role=nobody-has-this-role")
            .await
            .json();
        assert_eq!(none["people"], json!([]));
        assert_eq!(
            request
                .get("/api/resource-plan/role-tasks?role=unclassified")
                .await
                .status_code(),
            422
        );
        assert_eq!(
            request
                .get("/api/resource-plan/role-tasks")
                .await
                .status_code(),
            400
        );
    })
    .await;
}
