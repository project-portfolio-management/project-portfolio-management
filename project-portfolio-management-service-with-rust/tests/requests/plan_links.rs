//! Plan-link request tests (LT-8): safe http(s) URLs only, one link per URL,
//! listing, and removal (via the POST twin).
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_plan_links_out_safely_and_never_to_a_script_or_file() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let add = |kind: &str, url: &str, label: &str| {
            let (request, url_path, body) = (
                &request,
                format!("/api/plans/{p}/links"),
                json!({ "kind": kind, "url": url, "label": label }),
            );
            async move { request.post(&url_path).json(&body).await }
        };

        let ok: Value = add("docs", "https://wiki.example.com/apollo", "Spec")
            .await
            .json();
        assert_eq!(ok["label"], "Spec");
        assert_eq!(
            add("docs", "https://wiki.example.com/apollo", "Again")
                .await
                .status_code(),
            409,
            "once per URL"
        );
        for bad in [
            "javascript:alert(1)",
            "data:text/html,<b>",
            "file:///etc/passwd",
            "https://user:pw@evil.com/",
            "//evil.com",
            "",
        ] {
            assert_eq!(add("docs", bad, "x").await.status_code(), 422, "{bad:?}");
        }
        assert_eq!(add("nope", "https://x.com", "x").await.status_code(), 422);
        assert_eq!(
            add("repo", "https://git.example.com/apollo", "  ")
                .await
                .status_code(),
            422
        );
        add("repo", "http://git.example.com/apollo", "Repo")
            .await
            .assert_status_ok();

        let list: Value = request.get(&format!("/api/plans/{p}/links")).await.json();
        assert_eq!(list.as_array().unwrap().len(), 2);
        let id = ok["pid"].as_str().unwrap();
        request
            .post(&format!("/api/plans/{p}/links/{id}/delete"))
            .await
            .assert_status_ok();
        let after: Value = request.get(&format!("/api/plans/{p}/links")).await.json();
        assert_eq!(after.as_array().unwrap().len(), 1);
        assert_eq!(
            request
                .post(&format!("/api/plans/{p}/links/{id}/delete"))
                .await
                .status_code(),
            404,
            "already gone"
        );
        // Once removed, the URL may be linked again.
        add("docs", "https://wiki.example.com/apollo", "Spec v2")
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_plan_links_into_jira_trello_asana_linear_and_monday() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Tracked" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let path = format!("/api/plans/{p}/links");
        let add = |body: Value| {
            let (request, path) = (&request, path.clone());
            async move { request.post(&path).json(&body).await }
        };

        for (system, kind, url, id) in [
            ("jira", "tracker", "https://acme.atlassian.net/jira/software/projects/PPM/boards/1", Some("PPM")),
            ("jira", "ticket", "https://acme.atlassian.net/browse/PPM-7", Some("PPM-7")),
            ("trello", "tracker", "https://trello.com/b/AbC123/launch", Some("AbC123")),
            ("asana", "tracker", "https://app.asana.com/0/1200000000/list", None),
            ("linear", "tracker", "https://linear.app/acme/project/launch-1a2b", None),
            ("monday", "tracker", "https://acme.monday.com/boards/5550001", Some("5550001")),
        ] {
            let mut body = json!({ "kind": kind, "url": url, "label": format!("{system} {kind}"), "system": system });
            if let Some(id) = id {
                body["external_id"] = json!(id);
            }
            let r = add(body).await;
            r.assert_status_ok();
            let link: Value = r.json();
            assert_eq!(link["system"], system);
            assert_eq!(link["external_id"], id.map_or(Value::Null, Value::from));
        }

        // Wrong in each way the rules name.
        for (body, why) in [
            (json!({ "kind": "tracker", "url": "https://trello.com/b/x", "label": "x" }), "a tracker without a system"),
            (json!({ "kind": "tracker", "url": "https://app.asana.com/0/1", "label": "x", "system": "trello" }), "a Trello board on an Asana address"),
            (json!({ "kind": "tracker", "url": "https://eviltrello.com/b/x", "label": "x", "system": "trello" }), "a look-alike domain"),
            (json!({ "kind": "docs", "url": "https://wiki.example.com/a", "label": "x", "system": "jira" }), "a system on a docs link"),
            (json!({ "kind": "tracker", "url": "https://linear.app/x", "label": "x", "system": "notion" }), "an unknown system"),
            (json!({ "kind": "docs", "url": "https://wiki.example.com/b", "label": "x", "external_id": "A-1" }), "an id with no system"),
            (json!({ "kind": "ticket", "url": "https://acme.atlassian.net/browse/X-1", "label": "x", "system": "jira", "external_id": "has space" }), "an id with a space"),
        ] {
            assert_eq!(add(body).await.status_code(), 422, "{why}");
        }
        // Self-hosted Jira is accepted on any host.
        add(json!({ "kind": "tracker", "url": "https://jira.internal.example.com/projects/PPM", "label": "Internal Jira", "system": "jira" }))
            .await
            .assert_status_ok();

        let all: Value = request.get(&path).await.json();
        assert_eq!(all.as_array().map(Vec::len), Some(7));
        let jira: Value = request.get(&format!("{path}?system=jira")).await.json();
        assert_eq!(jira.as_array().map(Vec::len), Some(3), "two Jira links and the self-hosted one");
        let trackers: Value = request.get(&format!("{path}?system=jira&kind=ticket")).await.json();
        assert_eq!(trackers.as_array().map(Vec::len), Some(1));
        assert_eq!(request.get(&format!("{path}?system=notion")).await.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_task_links_to_its_issue_without_touching_the_plans_own_links() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Task links" })).await.json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let other: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Elsewhere" })).await.json();
        let o = other["pid"].as_str().expect("pid").to_string();
        let task: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({ "title": "Fix login" })).await.json();
        let t = task["pid"].as_str().expect("task pid").to_string();
        let issue = json!({ "kind": "ticket", "url": "https://acme.atlassian.net/browse/PPM-7", "label": "PPM-7", "system": "jira", "external_id": "PPM-7" });

        let made: Value = request.post(&format!("/api/plans/{p}/tasks/{t}/links")).json(&issue).await.json();
        assert_eq!((made["system"].as_str(), made["external_id"].as_str(), made["task_pid"].as_str()), (Some("jira"), Some("PPM-7"), Some(t.as_str())));
        assert_eq!(request.post(&format!("/api/plans/{p}/tasks/{t}/links")).json(&issue).await.status_code(), 409, "once per task");

        let on_task: Value = request.get(&format!("/api/plans/{p}/tasks/{t}/links")).await.json();
        assert_eq!(on_task.as_array().map(Vec::len), Some(1));
        let on_plan: Value = request.get(&format!("/api/plans/{p}/links")).await.json();
        assert_eq!(on_plan.as_array().map(Vec::len), Some(0), "a task's link is not the plan's");
        request.post(&format!("/api/plans/{p}/links")).json(&issue).await.assert_status_ok();

        // The same task cannot be reached through another plan; the rules of a plan link apply.
        assert_eq!(request.get(&format!("/api/plans/{o}/tasks/{t}/links")).await.status_code(), 404);
        let bad = json!({ "kind": "tracker", "url": "https://eviltrello.com/b/x", "label": "x", "system": "trello" });
        assert_eq!(request.post(&format!("/api/plans/{p}/tasks/{t}/links")).json(&bad).await.status_code(), 422);

        // Removing it from the plan's list finds nothing; from the task's, removes it.
        let lpid = made["pid"].as_str().expect("link pid");
        assert_eq!(request.post(&format!("/api/plans/{p}/links/{lpid}/delete")).await.status_code(), 404);
        request.post(&format!("/api/plans/{p}/tasks/{t}/links/{lpid}/delete")).await.assert_status_ok();
        let after: Value = request.get(&format!("/api/plans/{p}/tasks/{t}/links")).await.json();
        assert_eq!(after.as_array().map(Vec::len), Some(0));
    })
    .await;
}
