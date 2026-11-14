//! Lists defined once (RE-7): organization codes and the national scope, and roadmap categories with translations.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use axum::http::StatusCode;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn an_organization_code_is_unique_and_one_organization_at_most_is_national() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_uppercase();
        let code = format!("HB{tag}");
        let board: Value = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Health board {tag}"), "kind": "partner", "code": code }))
            .await
            .json();
        assert_eq!(board["code"], code);
        assert_eq!(board["national"], false);

        // The same code in another case is the same code.
        let clash = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Another board {tag}"), "kind": "partner",
                           "code": code.to_lowercase() }))
            .await;
        assert_eq!(clash.status_code(), StatusCode::CONFLICT);

        let bad = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Bad code {tag}"), "kind": "partner", "code": "-NO" }))
            .await;
        assert_eq!(bad.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

        // One national organization at most, wherever it was set.
        let existing: Value = request.get("/api/organizations").await.json();
        let already = existing["organizations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["national"] == true);
        if !already {
            let all_wales: Value = request
                .post("/api/organizations")
                .json(&json!({ "name": format!("All Wales {tag}"), "kind": "sponsor",
                               "code": format!("NAT{tag}"), "national": true }))
                .await
                .json();
            assert_eq!(all_wales["national"], true);
        }
        let second = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Second national {tag}"), "kind": "sponsor", "national": true }))
            .await;
        assert_eq!(second.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
        let promoted = request
            .put(&format!("/api/organizations/{}", board["pid"].as_str().unwrap()))
            .json(&json!({ "national": true }))
            .await;
        assert_eq!(promoted.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

        // The national organization is listed first.
        let list: Value = request.get("/api/organizations").await.json();
        assert_eq!(list["organizations"][0]["national"], true);

        // An empty code clears it; then the code is free again.
        let cleared: Value = request
            .put(&format!("/api/organizations/{}", board["pid"].as_str().unwrap()))
            .json(&json!({ "code": "" }))
            .await
            .json();
        assert_eq!(cleared["code"], Value::Null);
        request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Another board {tag}"), "kind": "partner", "code": code }))
            .await
            .assert_status_ok();
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_roadmap_category_is_defined_once_with_its_translations() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let name = format!("Patients and public {tag}");
        let made: Value = request
            .post("/api/roadmap-categories")
            .json(&json!({ "name": format!("  {name}  "),
                           "translations": { "cy": format!("Cleifion a'r cyhoedd {tag}") } }))
            .await
            .json();
        assert_eq!(made["name"], name, "trimmed");
        assert_eq!(
            made["translations"]["cy"],
            format!("Cleifion a'r cyhoedd {tag}")
        );
        let pid = made["pid"].as_str().unwrap().to_string();

        let twice = request
            .post("/api/roadmap-categories")
            .json(&json!({ "name": name.to_uppercase() }))
            .await;
        assert_eq!(twice.status_code(), StatusCode::CONFLICT);
        for bad in [
            json!({ "name": " " }),
            json!({ "name": "Ok", "translations": { "Welsh": "Iawn" } }),
            json!({ "name": "Ok", "translations": { "cy": " " } }),
        ] {
            let r = request.post("/api/roadmap-categories").json(&bad).await;
            assert_eq!(r.status_code(), StatusCode::UNPROCESSABLE_ENTITY, "{bad}");
        }

        // A rename and a new set of translations replace the old ones; retiring keeps the record.
        let renamed: Value = request
            .put(&format!("/api/roadmap-categories/{pid}"))
            .json(
                &json!({ "name": format!("Patients {tag}"), "translations": { "fr": "Patients" },
                           "active": false }),
            )
            .await
            .json();
        assert_eq!(renamed["name"], format!("Patients {tag}"));
        assert_eq!(renamed["translations"], json!({ "fr": "Patients" }));
        assert_eq!(renamed["active"], false);
        let list: Value = request.get("/api/roadmap-categories").await.json();
        let found = list["categories"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["pid"] == pid)
            .expect("still listed");
        assert_eq!(found["active"], false);

        let missing = request
            .put(&format!("/api/roadmap-categories/{}", uuid::Uuid::new_v4()))
            .json(&json!({ "active": true }))
            .await;
        assert_eq!(missing.status_code(), StatusCode::NOT_FOUND);
    })
    .await;
}
