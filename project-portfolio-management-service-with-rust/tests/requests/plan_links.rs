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
