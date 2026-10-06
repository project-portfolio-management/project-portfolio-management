//! RM-12b: `include` on a single task read. (Paging of every list is checked, from the
//! OpenAPI document, by `tests/list_paging.rs`.)

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn post(request: &axum_test::TestServer, path: &str, body: Value) -> Value {
    let r = request.post(path).json(&body).await;
    serde_json::from_str(&r.text()).unwrap_or(Value::Null)
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_task_can_be_read_alone_with_its_sub_resources_embedded() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        use axum::http::StatusCode as S;
        let who = "person:00000000-0000-4000-8000-0000000000c1";
        let p = post(&request, "/api/plans", json!({ "kind": "Project", "name": "Alone" })).await["pid"].as_str().unwrap().to_string();
        let other = post(&request, "/api/plans", json!({ "kind": "Project", "name": "Other" })).await["pid"].as_str().unwrap().to_string();
        let parent = post(&request, &format!("/api/plans/{p}/tasks"), json!({ "title": "Parent", "assignee_ref": who })).await["pid"].as_str().unwrap().to_string();
        let child = post(&request, &format!("/api/plans/{p}/tasks"), json!({ "title": "Child", "parent_task_pid": parent })).await["pid"].as_str().unwrap().to_string();
        let sibling = post(&request, &format!("/api/plans/{p}/tasks"), json!({ "title": "Sibling" })).await["pid"].as_str().unwrap().to_string();
        post(&request, &format!("/api/plans/{p}/time-entries"), json!({ "task_pid": parent, "actor_ref": who, "spent_on": "2026-10-06", "minutes": 60, "category": "work" })).await;
        request.put(&format!("/api/plans/{p}/tasks/{parent}/raci")).json(&json!({ "assignments": [{ "person_ref": who, "roles": "R" }] })).await.assert_status_ok();
        request.patch(&format!("/api/plans/{p}/tasks/{parent}")).json(&json!({ "status": "in_progress" })).await.assert_status_ok();
        let rel = request.post(&format!("/api/plans/{p}/tasks/{parent}/relations")).json(&json!({ "to_task_pid": sibling, "kind": "relates" })).await;
        let url = format!("/api/plans/{p}/tasks/{parent}");

        // Bare: exactly the task.
        let bare: Value = request.get(&url).await.json();
        assert_eq!((bare["pid"].as_str(), bare["title"].as_str()), (Some(parent.as_str()), Some("Parent")));
        assert!(bare.get("included").is_none());

        // Embedded: the same rows each sub-route would show.
        let full: Value = request.get(&format!("{url}?include=children,relations,transitions,raci,time_entries")).await.json();
        assert_eq!(full["task"]["pid"], parent.as_str());
        assert_eq!(full["included"]["children"].as_array().unwrap().len(), 1);
        assert_eq!(full["included"]["children"][0]["pid"], child.as_str());
        assert_eq!(full["included"]["time_entries"].as_array().unwrap().len(), 1);
        assert_eq!(full["included"]["raci"][0]["role"].as_str().map(str::len), Some(1));
        assert!(full["included"]["transitions"].as_array().unwrap().len() >= 2, "created, then moved");
        assert!(full["omitted"].as_array().unwrap().is_empty());
        if rel.status_code() == 200 {
            assert_eq!(full["included"]["relations"].as_array().unwrap().len(), 1);
        }
        // Naming one twice, or in another order, is the same answer.
        let twice: Value = request.get(&format!("{url}?include=raci,raci")).await.json();
        assert_eq!(twice["included"].as_object().unwrap().len(), 1);

        // An unknown name is a 422 that lists what is allowed.
        let bad = request.get(&format!("{url}?include=children,comments,bogus")).await;
        bad.assert_status(S::UNPROCESSABLE_ENTITY);
        let text = bad.text();
        assert!(text.contains("comments") && text.contains("bogus") && text.contains("time_entries"), "{text}");
        // An empty include is the bare task; another plan's task is not found.
        assert!(request.get(&format!("{url}?include=")).await.json::<Value>().get("task").is_none());
        request.get(&format!("/api/plans/{other}/tasks/{parent}")).await.assert_status(S::NOT_FOUND);
        request.get(&format!("/api/plans/{p}/tasks/not-a-uuid")).await.assert_status(S::NOT_FOUND);
        let deleted = request.delete(&format!("/api/plans/{p}/tasks/{child}")).await;
        deleted.assert_status_ok();
        request.get(&format!("/api/plans/{p}/tasks/{child}")).await.assert_status(S::NOT_FOUND);
        let after: Value = request.get(&format!("{url}?include=children")).await.json();
        assert!(after["included"]["children"].as_array().unwrap().is_empty(), "a deleted child is not embedded");
    })
    .await;
}
