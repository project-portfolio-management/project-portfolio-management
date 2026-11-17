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

/// How `GET /api/public-roadmaps/stale` lists the roadmap of `product`, if at all.
async fn stale_state(request: &axum_test::TestServer, product: &str) -> Option<Value> {
    let s: Value = request.get("/api/public-roadmaps/stale").await.json();
    s["roadmaps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["plan_pid"] == product)
        .map(|r| r["state"].clone())
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_roadmap_is_reviewed_only_on_purpose_and_stale_after_the_period() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let product = plan(&request, "Product", &format!("Results {tag}")).await;
        let url = format!("/api/plans/{product}/roadmap");
        let r: Value = request
            .put(&url)
            .json(&json!({ "title": format!("Test results online {tag}") }))
            .await
            .json();
        assert_eq!(r["freshness"]["state"], "unknown", "never reviewed");
        assert_eq!(r["freshness"]["stale_after_days"], 42);
        let today = project_portfolio_management::datetime::Date::parse_from_str(
            r["freshness"]["today"].as_str().unwrap(),
            "%Y-%m-%d",
        )
        .unwrap();
        let ago = |n: i64| {
            (today - project_portfolio_management::datetime::Duration::days(n)).to_string()
        };
        assert_eq!(
            stale_state(&request, &product).await,
            Some(json!("unknown"))
        );

        let reviewed = format!("{url}/reviewed");
        let r: Value = request
            .put(&reviewed)
            .json(&json!({ "reviewed_on": ago(42) }))
            .await
            .json();
        assert_eq!(
            r["freshness"]["state"], "fresh",
            "calendar day 42 is not stale"
        );
        assert_eq!(stale_state(&request, &product).await, None);
        let r: Value = request
            .put(&reviewed)
            .json(&json!({ "reviewed_on": ago(43) }))
            .await
            .json();
        assert_eq!(r["freshness"]["state"], "stale", "calendar day 43 is");
        assert_eq!(stale_state(&request, &product).await, Some(json!("stale")));

        // Publishing, editing and adding an item never touch the review date.
        request
            .put(&url)
            .json(&json!({ "published": true, "vision": "Results the same day" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("{url}/items"))
            .json(&json!({ "title": "Fix a typo", "placement": "now" }))
            .await
            .assert_status_ok();
        let r: Value = request.get(&url).await.json();
        assert_eq!(r["reviewed_on"], ago(43));
        assert_eq!(r["freshness"]["state"], "stale");

        let r: Value = request
            .put(&reviewed)
            .json(&json!({ "today": true }))
            .await
            .json();
        assert_eq!(r["reviewed_on"], today.to_string());
        assert_eq!(r["freshness"]["state"], "fresh");
        let future = request
            .put(&reviewed)
            .json(&json!({ "reviewed_on": ago(-1) }))
            .await;
        assert_eq!(future.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
        let r: Value = request
            .put(&reviewed)
            .json(&json!({ "reviewed_on": null }))
            .await
            .json();
        assert_eq!(r["freshness"]["state"], "unknown");
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_change_set_publishes_all_or_nothing_and_names_each_conflict() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let product = plan(&request, "Product", &format!("Referrals {tag}")).await;
        let url = format!("/api/plans/{product}/roadmap");
        request.put(&url).json(&json!({ "title": format!("Referrals {tag}") })).await.assert_status_ok();
        let item: Value = request
            .post(&format!("{url}/items"))
            .json(&json!({ "title": "Refer online", "placement": "next", "description": "For GPs" }))
            .await
            .json();
        let ipid = item["pid"].as_str().unwrap().to_string();
        let publish = format!("{url}/publish");

        // Two editors change different fields of one item: both land.
        let a = json!({ "items": [{ "pid": ipid, "fields": { "title": { "base": "Refer online", "value": "Refer a patient online" } } }] });
        let b = json!({ "items": [{ "pid": ipid, "fields": { "description": { "base": "For GPs", "value": "For GPs and dentists" } } }] });
        request.post(&publish).json(&a).await.assert_status_ok();
        let r: Value = request.post(&publish).json(&b).await.json();
        let it = &r["roadmap"]["items"][0];
        assert_eq!(it["title"], "Refer a patient online");
        assert_eq!(it["description"], "For GPs and dentists");

        // The same field, changed to something else meanwhile: 409 with both values, and nothing written at all.
        let clash = json!({
            "roadmap": { "vision": { "base": null, "value": "Every referral online" } },
            "items": [{ "pid": ipid, "fields": {
                "title": { "base": "Refer online", "value": "Refer by phone" },
                "placement": { "base": "next", "value": "now" } } }],
            "new": [{ "title": "Should not be added", "placement": "later" }],
        });
        let refused = request.post(&publish).json(&clash).await;
        assert_eq!(refused.status_code(), StatusCode::CONFLICT);
        let body: Value = refused.json();
        assert_eq!(body["conflicts"].as_array().unwrap().len(), 1);
        assert_eq!(body["conflicts"][0]["field"], "title");
        assert_eq!(body["conflicts"][0]["base"], "Refer online");
        assert_eq!(body["conflicts"][0]["value"], "Refer by phone");
        assert_eq!(body["conflicts"][0]["current"], "Refer a patient online");
        let r: Value = request.get(&url).await.json();
        assert_eq!(r["vision"], Value::Null, "nothing written");
        assert_eq!(r["items"].as_array().unwrap().len(), 1, "no item added");
        assert_eq!(r["items"][0]["placement"], "next");

        // A field already changed to the same value is not a conflict.
        let same = json!({ "items": [{ "pid": ipid, "fields": { "title": { "base": "Refer online", "value": "Refer a patient online" } } }] });
        let r: Value = request.post(&publish).json(&same).await.json();
        assert_eq!(r["applied"], json!([]));

        // The preview equals what publishing applies; a retry with the same key writes once.
        let set = json!({
            "roadmap": { "vision": { "base": null, "value": "  Every referral online  " }, "published": { "base": false, "value": true } },
            "items": [{ "pid": ipid, "fields": { "placement": { "base": "next", "value": "now" },
                                                 "tags": { "base": [], "value": ["GPs"] } } }],
            "new": [{ "title": "Track a referral", "placement": "later" }],
        });
        let preview: Value = request.post(&format!("{publish}/preview")).json(&set).await.json();
        assert_eq!(preview["would_publish"], true);
        assert_eq!(preview["changes"].as_array().unwrap().len(), 5);
        let r: Value = request.get(&url).await.json();
        assert_eq!(r["published"], false, "the preview wrote nothing");
        let key = format!("publish-{tag}");
        let first: Value = request
            .post(&publish)
            .add_header(axum::http::HeaderName::from_static("idempotency-key"), key.parse::<axum::http::HeaderValue>().unwrap())
            .json(&set)
            .await
            .json();
        assert_eq!(first["applied"], preview["changes"]);
        let again = request
            .post(&publish)
            .add_header(axum::http::HeaderName::from_static("idempotency-key"), key.parse::<axum::http::HeaderValue>().unwrap())
            .json(&set)
            .await;
        again.assert_status_ok();
        assert_eq!(again.header("idempotent-replayed"), "true");
        let r: Value = request.get(&url).await.json();
        assert_eq!(r["vision"], "Every referral online");
        assert_eq!(r["published"], true);
        let titles: Vec<&str> = r["items"].as_array().unwrap().iter().map(|i| i["title"].as_str().unwrap()).collect();
        assert_eq!(titles, ["Refer a patient online", "Track a referral"], "added once; now before later");
        assert_eq!(r["items"][0]["tags"], json!(["GPs"]));

        // A bad value or an unknown item refuses the set.
        let bad = request
            .post(&publish)
            .json(&json!({ "items": [{ "pid": ipid, "fields": { "placement": { "base": "now", "value": "soon" } } }] }))
            .await;
        assert_eq!(bad.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
        let gone = request
            .post(&publish)
            .json(&json!({ "items": [{ "pid": uuid::Uuid::new_v4(), "fields": { "title": { "base": "x", "value": "y" } } }] }))
            .await;
        assert_eq!(gone.status_code(), StatusCode::CONFLICT, "an item that is gone is a conflict");
    })
    .await;
}

/// Every roadmap `GET /api/public-roadmaps` lists, following each page's `Link: rel="next"`.
async fn every_listed(request: &axum_test::TestServer) -> Vec<Value> {
    let mut url = "/api/public-roadmaps?limit=500".to_string();
    let mut all = Vec::new();
    loop {
        let r = request.get(&url).await;
        r.assert_status_ok();
        let next = r
            .maybe_header("link")
            .map(|l| l.to_str().unwrap().to_string());
        all.extend(r.json::<Vec<Value>>());
        match next {
            Some(l) => {
                url = l
                    .trim_start_matches('<')
                    .split_once('>')
                    .unwrap()
                    .0
                    .to_string();
            }
            None => return all,
        }
    }
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn editors_list_every_roadmap_they_may_read_with_its_plan_id() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let first = plan(&request, "Product", &format!("Prescriptions {tag}")).await;
        let second = plan(&request, "Product", &format!("Appointments {tag}")).await;
        let none = plan(&request, "Product", &format!("No roadmap {tag}")).await;
        request
            .put(&format!("/api/plans/{first}/roadmap"))
            .json(&json!({ "title": format!("b repeat prescriptions {tag}"), "published": true }))
            .await
            .assert_status_ok();
        request
            .put(&format!("/api/plans/{second}/roadmap"))
            .json(&json!({ "title": format!("A appointments {tag}") }))
            .await
            .assert_status_ok();
        for (title, placement) in [
            ("Now", json!("now")),
            ("Later", json!("later")),
            ("Idea", Value::Null),
        ] {
            request
                .post(&format!("/api/plans/{first}/roadmap/items"))
                .json(&json!({ "title": title, "placement": placement }))
                .await
                .assert_status_ok();
        }
        let gone: Value = request
            .post(&format!("/api/plans/{first}/roadmap/items"))
            .json(&json!({ "title": "Gone", "placement": "next" }))
            .await
            .json();
        request
            .post(&format!(
                "/api/plans/{first}/roadmap/items/{}/delete",
                gone["pid"].as_str().unwrap()
            ))
            .await
            .assert_status_ok();

        let all = every_listed(&request).await;
        let mine: Vec<&Value> = all
            .iter()
            .filter(|r| r["title"].as_str().unwrap().ends_with(&tag))
            .collect();
        assert_eq!(mine.len(), 2, "a plan with no roadmap is not listed");
        assert!(all.iter().all(|r| r["plan_pid"] != none));
        // By title ignoring case: "A appointments" before "b repeat prescriptions".
        assert_eq!(mine[0]["plan_pid"], second);
        assert_eq!(mine[0]["published"], false, "unpublished is listed");
        assert_eq!(mine[0]["freshness"], "unknown");
        assert_eq!(mine[0]["items"], json!({ "placed": 0, "unplaced": 0 }));
        assert_eq!(mine[1]["plan_pid"], first);
        assert_eq!(mine[1]["published"], true);
        assert_eq!(mine[1]["slug"], format!("b-repeat-prescriptions-{tag}"));
        assert_eq!(
            mine[1]["items"],
            json!({ "placed": 2, "unplaced": 1 }),
            "a removed item is not counted"
        );

        // A page of one names the next.
        let r = request.get("/api/public-roadmaps?limit=1").await;
        r.assert_status_ok();
        assert_eq!(r.json::<Vec<Value>>().len(), 1);
        assert!(
            r.maybe_header("link").is_some(),
            "more than one roadmap exists"
        );
        request
            .get("/api/public-roadmaps?limit=0")
            .await
            .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_familys_editors_choose_its_colour() {
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let family = plan(&request, "Portfolio", &format!("Single Record {tag}")).await;
        let product: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Product", "name": format!("Records {tag}"), "parent_ref": family }))
            .await
            .json();
        let product = product["pid"].as_str().unwrap().to_string();
        let url = format!("/api/plans/{family}/roadmap-family");

        let f: Value = request.get(&url).await.json();
        assert_eq!(f["colour"], Value::Null, "no colour until one is chosen");
        assert_eq!(f["products"], 1);
        assert_eq!(f["name"], format!("Single Record {tag}"));

        let f: Value = request
            .put(&url)
            .json(&json!({ "colour": " #32508A ", "translations": { "cy": "Cofnod Sengl" } }))
            .await
            .json();
        assert_eq!(f["colour"], "#32508a", "any colour, kept in lower case");
        assert_eq!(f["contrast"]["white_text_meets_aa"], true);
        assert_eq!(f["contrast"]["better_text"], "white");
        assert_eq!(f["translations"]["cy"], "Cofnod Sengl");
        // A light colour is kept too; the contrast says white text would not read on it.
        let f: Value = request.put(&url).json(&json!({ "colour": "#ffeb3b" })).await.json();
        assert_eq!(f["colour"], "#ffeb3b");
        assert_eq!(f["contrast"]["white_text_meets_aa"], false);
        assert_eq!(f["contrast"]["better_text"], "black");
        assert_eq!(f["translations"]["cy"], "Cofnod Sengl", "absent translations are kept");
        for bad in [json!({ "colour": "#fff" }), json!({ "colour": "yellow" }),
                    json!({ "colour": "#ffeb3b", "translations": { "Welsh": "x" } }),
                    json!({ "colour": null, "translations": { "cy": "x" } })] {
            request.put(&url).json(&bad).await.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // The family list and the editors' roadmap list both carry it.
        let listed = |all: Vec<Value>| all.into_iter().find(|f| f["plan_pid"] == family);
        let all: Vec<Value> = request.get("/api/roadmap-families?limit=500").await.json();
        let mut found = listed(all);
        let mut next = None;
        while found.is_none() {
            // Walk the pages when other runs left many families.
            let r = request
                .get(next.as_deref().unwrap_or("/api/roadmap-families?limit=500"))
                .await;
            let link = r.maybe_header("link").map(|l| {
                l.to_str().unwrap().trim_start_matches('<').split_once('>').unwrap().0.to_string()
            });
            found = listed(r.json());
            assert!(found.is_some() || link.is_some(), "the family is listed");
            next = link;
        }
        let found = found.unwrap();
        assert_eq!(found["colour"], "#ffeb3b");
        assert_eq!(found["products"], 1);
        request
            .put(&format!("/api/plans/{product}/roadmap"))
            .json(&json!({ "title": format!("Records roadmap {tag}") }))
            .await
            .assert_status_ok();
        let mine = every_listed(&request)
            .await
            .into_iter()
            .find(|r| r["plan_pid"] == product)
            .unwrap();
        assert_eq!(mine["family"]["plan_pid"], family);
        assert_eq!(mine["family"]["colour"], "#ffeb3b");

        // null removes it; the change is audited.
        let f: Value = request.put(&url).json(&json!({ "colour": null })).await.json();
        assert_eq!(f["colour"], Value::Null);
        assert_eq!(f["translations"], json!({}));
        let audit: Value = request.get(&format!("/api/plans/{family}/audit")).await.json();
        assert!(audit.as_array().unwrap().iter().any(|e| e["action"] == "roadmap_family_changed"));
        request
            .get("/api/plans/00000000-0000-4000-8000-000000000000/roadmap-family")
            .await
            .assert_status_not_found();
    })
    .await;
}
