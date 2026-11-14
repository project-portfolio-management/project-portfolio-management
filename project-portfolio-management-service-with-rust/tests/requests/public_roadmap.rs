//! A product plan's public product roadmap (RE-1) and its translations (RE-6).
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use axum::http::StatusCode;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, kind: &str, name: &str) -> String {
    let p: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": kind, "name": name }))
        .await
        .json();
    p["pid"].as_str().expect("pid").to_string()
}

fn titles(roadmap: &Value) -> Vec<(String, Value)> {
    roadmap["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            (
                i["title"].as_str().unwrap().to_string(),
                i["placement"].clone(),
            )
        })
        .collect()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_product_keeps_a_public_roadmap_of_items_placed_now_next_and_later() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let project = plan(&request, "Project", &format!("Clinic move {tag}")).await;
        let refused = request
            .put(&format!("/api/plans/{project}/roadmap"))
            .json(&json!({ "title": "Clinic move" }))
            .await;
        assert_eq!(refused.status_code(), StatusCode::UNPROCESSABLE_ENTITY, "only a Product");

        let product = plan(&request, "Product", &format!("Video appointments {tag}")).await;
        let url = format!("/api/plans/{product}/roadmap");
        request.get(&url).await.assert_status_not_found();
        let made: Value = request
            .put(&url)
            .json(&json!({ "title": format!("  Video appointments {tag}  "),
                           "vision": "Every follow-up can be by video" }))
            .await
            .json();
        assert_eq!(made["title"], format!("Video appointments {tag}"));
        assert_eq!(made["slug"], format!("video-appointments-{tag}"), "made from the title");
        assert_eq!(made["published"], false);
        assert_eq!(made["reviewed_on"], Value::Null);
        let clash = request
            .put(&format!("/api/plans/{}/roadmap", plan(&request, "Product", &format!("Other {tag}")).await))
            .json(&json!({ "title": "Other", "slug": format!("video-appointments-{tag}") }))
            .await;
        assert_eq!(clash.status_code(), StatusCode::CONFLICT);

        let items = format!("{url}/items");
        let add = |body: Value| {
            let request = &request;
            let items = items.clone();
            async move { request.post(&items).json(&body).await }
        };
        let a: Value = add(json!({ "title": "Dermatology by video", "placement": "now",
                                    "expected_outcome": "Patients travel less",
                                    "tags": ["Patients", "Video appointments"],
                                    "details": [{ "text": "Follow-up", "level": 0 },
                                                { "text": "Adults first", "level": 1 }] }))
            .await
            .json();
        let b: Value = add(json!({ "title": "Cardiology by video", "placement": "now" })).await.json();
        let c: Value = add(json!({ "title": "Group sessions", "placement": "later" })).await.json();
        let u: Value = add(json!({ "title": "An idea not yet placed" })).await.json();
        for i in [&a, &b, &c, &u] {
            let id = i["public_id"].as_str().unwrap();
            assert!(id.len() == 8 && id.starts_with(|ch: char| ch.is_ascii_lowercase()), "{id}");
        }
        assert_eq!(u["placement"], Value::Null);

        let r: Value = request.get(&url).await.json();
        assert_eq!(
            titles(&r).iter().map(|(t, _)| t.as_str()).collect::<Vec<_>>(),
            ["Dermatology by video", "Cardiology by video", "Group sessions", "An idea not yet placed"],
            "page order, unplaced last"
        );

        // Move between placements, then reorder within one.
        let mv = |pid: &str, body: Value| {
            let request = &request;
            let at = format!("{items}/{pid}/move");
            async move { request.post(&at).json(&body).await }
        };
        mv(c["pid"].as_str().unwrap(), json!({ "placement": "next" })).await.assert_status_ok();
        mv(b["pid"].as_str().unwrap(), json!({ "placement": "now", "index": 0 })).await.assert_status_ok();
        let bad = mv(a["pid"].as_str().unwrap(), json!({ "placement": "soon" })).await;
        assert_eq!(bad.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
        let r: Value = request.get(&url).await.json();
        assert_eq!(
            titles(&r),
            [
                ("Cardiology by video".to_string(), json!("now")),
                ("Dermatology by video".to_string(), json!("now")),
                ("Group sessions".to_string(), json!("next")),
                ("An idea not yet placed".to_string(), Value::Null),
            ]
        );

        // Rules for the fields.
        for body in [
            json!({ "title": "Skips", "details": [{ "text": "Top", "level": 0 }, { "text": "Deep", "level": 2 }] }),
            json!({ "title": "Starts deep", "details": [{ "text": "Deep", "level": 1 }] }),
            json!({ "title": " " }),
            json!({ "title": "Tags", "tags": ["a, b"] }),
            json!({ "title": "Tags", "tags": ["Patients", "patients"] }),
            json!({ "title": "Org", "organization_pids": [uuid::Uuid::new_v4()] }),
            json!({ "title": "Where", "placement": "someday" }),
        ] {
            let r = add(body.clone()).await;
            assert_eq!(r.status_code(), StatusCode::UNPROCESSABLE_ENTITY, "{body}");
        }
        let changed = request
            .put(&format!("{items}/{}", a["pid"].as_str().unwrap()))
            .json(&json!({ "placement": "later" }))
            .await;
        assert_eq!(changed.status_code(), StatusCode::UNPROCESSABLE_ENTITY, "moving is …/move");

        // An item for an organization, and a category that must be active.
        let org: Value = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Health board {tag}"), "kind": "partner" }))
            .await
            .json();
        let with_org: Value = request
            .put(&format!("{items}/{}", a["pid"].as_str().unwrap()))
            .json(&json!({ "organization_pids": [org["pid"], org["pid"]], "description": "  " }))
            .await
            .json();
        assert_eq!(with_org["organization_pids"], json!([org["pid"]]), "no repeats");
        assert_eq!(with_org["description"], Value::Null, "an empty string clears it");
        assert_eq!(with_org["expected_outcome"], "Patients travel less", "left as it was");
        let cat: Value = request
            .post("/api/roadmap-categories")
            .json(&json!({ "name": format!("Outpatients {tag}") }))
            .await
            .json();
        request
            .put(&format!("/api/roadmap-categories/{}", cat["pid"].as_str().unwrap()))
            .json(&json!({ "active": false }))
            .await
            .assert_status_ok();
        let retired = request
            .put(&url)
            .json(&json!({ "category_pid": cat["pid"] }))
            .await;
        assert_eq!(retired.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

        // Removing an item: gone from the list, and the rest keep their order.
        request
            .post(&format!("{items}/{}/delete", u["pid"].as_str().unwrap()))
            .await
            .assert_status_ok();
        request
            .post(&format!("{items}/{}/delete", u["pid"].as_str().unwrap()))
            .await
            .assert_status_not_found();
        let r: Value = request.get(&url).await.json();
        assert_eq!(r["items"].as_array().unwrap().len(), 3);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_translation_is_kept_and_marked_for_review_when_its_source_changes() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let product = plan(&request, "Product", &format!("Booking {tag}")).await;
        let url = format!("/api/plans/{product}/roadmap");
        request
            .put(&url)
            .json(&json!({ "title": format!("Online booking {tag}") }))
            .await
            .assert_status_ok();
        let item: Value = request
            .post(&format!("{url}/items"))
            .json(&json!({ "title": "Book a blood test online", "placement": "next",
                           "tags": ["Patients", "Carers"] }))
            .await
            .json();
        let at = format!("{url}/items/{}", item["pid"].as_str().unwrap());

        let welsh: Value = request
            .put(&format!("{at}/translations/cy"))
            .json(&json!({ "title": " Trefnu prawf gwaed ar-lein ", "tags": ["Cleifion", "Gofalwyr"] }))
            .await
            .json();
        assert_eq!(welsh["translations"]["cy"]["title"]["value"], "Trefnu prawf gwaed ar-lein");
        assert_eq!(welsh["translations"]["cy"]["title"]["needs_review"], false);
        assert_eq!(welsh["translations"]["cy"]["tags"]["needs_review"], false);

        // The English changes: the Welsh is kept, and marked.
        let changed: Value = request
            .put(&at)
            .json(&json!({ "title": "Book any blood test online" }))
            .await
            .json();
        assert_eq!(changed["translations"]["cy"]["title"]["value"], "Trefnu prawf gwaed ar-lein");
        assert_eq!(changed["translations"]["cy"]["title"]["needs_review"], true);
        assert_eq!(changed["translations"]["cy"]["tags"]["needs_review"], false, "tags did not change");

        // Saving it again clears the mark.
        let again: Value = request
            .put(&format!("{at}/translations/cy"))
            .json(&json!({ "title": "Trefnu unrhyw brawf gwaed ar-lein" }))
            .await
            .json();
        assert_eq!(again["translations"]["cy"]["title"]["needs_review"], false);

        for (lang, body) in [
            ("Welsh", json!({ "title": "Teitl" })),
            ("cy", json!({ "slug": "trefnu" })),
            ("cy", json!({ "tags": ["Cleifion"] })),
            ("cy", json!({ "phase": "Cam 1" })),
        ] {
            let r = request.put(&format!("{at}/translations/{lang}")).json(&body).await;
            assert_eq!(r.status_code(), StatusCode::UNPROCESSABLE_ENTITY, "{lang} {body}");
        }

        // An empty value removes a translation; the roadmap's own fields translate the same way.
        let removed: Value = request
            .put(&format!("{at}/translations/cy"))
            .json(&json!({ "tags": "" }))
            .await
            .json();
        assert!(removed["translations"]["cy"].get("tags").is_none());
        let r: Value = request
            .put(&format!("{url}/translations/cy"))
            .json(&json!({ "title": "Trefnu ar-lein" }))
            .await
            .json();
        assert_eq!(r["translations"]["cy"]["title"]["needs_review"], false);
        let r: Value = request
            .put(&url)
            .json(&json!({ "title": format!("Online booking for everyone {tag}") }))
            .await
            .json();
        assert_eq!(r["translations"]["cy"]["title"]["needs_review"], true);
        assert_eq!(r["slug"], format!("online-booking-{tag}"), "a rename keeps the slug");
    })
    .await;
}
