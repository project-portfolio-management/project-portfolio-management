//! "My work" (LT-3), end to end over the real router with real PASETO
//! identities: bucketing by due date in the caller's own zone.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test my_work -- --ignored`.

#![expect(
    clippy::similar_names,
    clippy::too_many_lines,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [31; 32];
const ISSUER: &str = "authentication-service";
const AUDIENCE: &str = "main-x-service";

/// The published Ed25519 key set plus its `kid`, derived the way the
/// verifier derives it (SHA-256 of the public key, base64url).
fn keys_and_kid() -> (Value, String) {
    let public = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
    let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public));
    let keys = json!({
        "keys": [{ "kty": "OKP", "crv": "Ed25519", "use": "sig",
                   "kid": kid, "x": URL_SAFE_NO_PAD.encode(public) }]
    });
    (keys, kid)
}

/// Mint a `v4.public` token for `sub`.
fn mint(kid: &str, sub: &str) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": {},
    })
    .to_string();
    let keypair = SigningKey::from_bytes(&SEED).to_keypair_bytes();
    let key = Key::<64>::from(keypair);
    let private = PasetoAsymmetricPrivateKey::<V4, Public>::from(&key);
    let footer = format!(r#"{{"kid":"{kid}"}}"#);
    let mut builder = Paseto::<V4, Public>::builder();
    builder.set_payload(Payload::from(payload.as_str()));
    builder.set_footer(Footer::from(footer.as_str()));
    builder.try_sign(&private).expect("sign")
}

fn auth_header(token: &str) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    (
        axum::http::header::AUTHORIZATION,
        format!("Bearer {token}").parse().unwrap(),
    )
}

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";
const NOBODY: &str = "3f2c6b7e-0000-4000-8000-00000000000c";
const AS_OF: &str = "2026-10-02T23:30:00Z";

/// LT-3 acceptance: a task due on a given date is `today` in one zone and
/// `overdue` in another at the same instant; a person sees only their own
/// open tasks; every bucket is present; bad zones are refused; a `sub`
/// that is not a UUID gets empty buckets with a stated reason.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test my_work -- --ignored`"]
async fn my_work_buckets_by_due_date_in_the_callers_zone() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE));
    let (bk, bv) = auth_header(&mint(&kid, BOB));
    let (nk, nv) = auth_header(&mint(&kid, NOBODY));
    let (ck, cv) = auth_header(&mint(&kid, "carol"));

    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let mk = |title: &str, who: &str, due: Option<&str>, status: Option<&str>| {
            let mut body = json!({ "title": title, "assignee_ref": who });
            if let Some(d) = due {
                body["due_date"] = json!(d);
            }
            if let Some(s) = status {
                body["status"] = json!(s);
            }
            body
        };
        let alice_p = format!("person:{ALICE}");
        let alice_w = format!("worker:{ALICE}");
        let bob_p = format!("person:{BOB}");
        for body in [
            mk("due Oct 2", &alice_p, Some("2026-10-02"), None),
            mk("due Oct 5", &alice_p, Some("2026-10-05"), None),
            mk("due Oct 20", &alice_p, Some("2026-10-20"), None),
            mk("no date", &alice_p, None, None),
            mk("worker ref, overdue", &alice_w, Some("2026-10-01"), None),
            mk("already done", &alice_p, Some("2026-10-02"), Some("done")),
            mk("bob's", &bob_p, Some("2026-10-02"), None),
        ] {
            request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&body)
                .await
                .assert_status_ok();
        }
        // A malformed due date is refused, not stored.
        let bad = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "x", "due_date": "02/10/2026" }))
            .await;
        assert!(bad.status_code().is_client_error());

        let get = |tz: &'static str| {
            let (k, v) = (ak.clone(), av.clone());
            let request = &request;
            async move {
                let r = request
                    .get(&format!("/api/me/tasks?tz={tz}&as_of={AS_OF}"))
                    .add_header(k, v)
                    .await;
                r.assert_status_ok();
                r.json::<Value>()
            }
        };
        let titles = |v: &Value, bucket: &str| -> Vec<String> {
            v["buckets"][bucket]
                .as_array()
                .expect("bucket present")
                .iter()
                .map(|t| t["title"].as_str().expect("title").to_string())
                .collect()
        };

        // Los Angeles: it is still 2026-10-02 there.
        let la = get("America/Los_Angeles").await;
        assert_eq!(la["today"], "2026-10-02");
        assert_eq!(titles(&la, "today"), ["due Oct 2"]);
        assert_eq!(titles(&la, "overdue"), ["worker ref, overdue"]);
        assert_eq!(titles(&la, "this_week"), ["due Oct 5"]);
        assert_eq!(titles(&la, "later"), ["due Oct 20"]);
        assert_eq!(titles(&la, "undated"), ["no date"]);
        assert_eq!(la["total"], 5, "done and other people's tasks excluded");

        // Auckland: the same instant is already 2026-10-03.
        let nz = get("Pacific/Auckland").await;
        assert_eq!(nz["today"], "2026-10-03");
        assert_eq!(titles(&nz, "today"), Vec::<String>::new());
        let mut overdue = titles(&nz, "overdue");
        overdue.sort();
        assert_eq!(overdue, ["due Oct 2", "worker ref, overdue"]);

        // Zones: a fixed offset, an abbreviation and a missing zone are refused.
        for tz in ["%2B02:00", "PST"] {
            assert_eq!(
                request
                    .get(&format!("/api/me/tasks?tz={tz}"))
                    .add_header(ak.clone(), av.clone())
                    .await
                    .status_code(),
                422,
                "{tz}"
            );
        }
        assert!(
            request
                .get("/api/me/tasks")
                .add_header(ak.clone(), av.clone())
                .await
                .status_code()
                .is_client_error()
        );
        // No token at all.
        assert_eq!(request.get("/api/me/tasks?tz=UTC").await.status_code(), 401);

        // Bob sees only his own task.
        let bob: Value = request
            .get(&format!("/api/me/tasks?tz=UTC&as_of={AS_OF}"))
            .add_header(bk, bv)
            .await
            .json();
        assert_eq!(bob["total"], 1);
        assert_eq!(titles(&bob, "today"), ["bob's"]);

        // A person with nothing assigned: all five buckets, empty, no error.
        let none: Value = request
            .get("/api/me/tasks?tz=UTC")
            .add_header(nk, nv)
            .await
            .json();
        assert_eq!(none["total"], 0);
        for b in ["overdue", "today", "this_week", "later", "undated"] {
            assert!(none["buckets"][b].as_array().expect("present").is_empty());
        }
        assert!(none["reason"].is_null());

        // A non-UUID sub cannot be addressed as a person: say so.
        let carol: Value = request
            .get("/api/me/tasks?tz=UTC")
            .add_header(ck, cv)
            .await
            .json();
        assert_eq!(carol["total"], 0);
        assert_eq!(carol["reason"], "sub_not_addressable");

        // SC-10: float for the worker. A second plan with scheduled tasks.
        let plan2: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Gemini" }))
            .await
            .json();
        let p2 = plan2["pid"].as_str().expect("plan pid").to_string();
        let sched = |title: &str, who: &str, days: Option<i64>| {
            let mut body = json!({ "title": title, "assignee_ref": who });
            if let Some(d) = days {
                body["start_date"] = json!("2026-10-05");
                body["duration_days"] = json!(d);
            }
            body
        };
        let mut pids = std::collections::HashMap::new();
        for (title, who, days) in [
            ("long", &alice_p, Some(5)),
            ("short", &alice_p, Some(2)),
            ("late", &alice_p, Some(5)),
            ("bobs short", &bob_p, Some(2)),
            ("unplanned", &alice_p, None),
        ] {
            let t: Value = request
                .post(&format!("/api/plans/{p2}/tasks"))
                .json(&sched(title, who, days))
                .await
                .json();
            pids.insert(title, t["pid"].as_str().expect("task pid").to_string());
        }
        // A deadline two business days into a five-business-day task: negative float.
        request
            .put(&format!("/api/plans/{p2}/tasks/{}/schedule", pids["late"]))
            .json(&json!({ "start_date": "2026-10-05", "duration_days": 5, "deadline": "2026-10-07" }))
            .await
            .assert_status_ok();
        let mine: Value = request
            .get(&format!("/api/me/tasks?tz=UTC&as_of={AS_OF}"))
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        let find = |title: &str| -> Value {
            ["overdue", "today", "this_week", "later", "undated"]
                .iter()
                .flat_map(|b| mine["buckets"][b].as_array().cloned().unwrap_or_default())
                .find(|t| t["title"] == title)
                .unwrap_or_else(|| panic!("{title} listed"))
        };
        let long = find("long");
        assert_eq!(long["schedule"]["critical"], true, "it finishes with the project");
        assert_eq!(long["schedule"]["total_float"], 0);
        assert_eq!(long["schedule"]["late"], false);
        let short = find("short");
        assert!(short["schedule"]["total_float"].as_i64().unwrap() > 0);
        assert_eq!(short["schedule"]["late"], false);
        let late = find("late");
        assert!(late["schedule"]["total_float"].as_i64().unwrap() < 0, "{late}");
        assert_eq!(late["schedule"]["late"], true);
        assert_eq!(late["schedule"]["critical"], true);
        // No schedule data: no figure and a reason, never zero.
        let plain = find("due Oct 20");
        assert!(plain["schedule"].is_null());
        assert_eq!(plain["schedule_reason"], "no_duration");
        let unplanned = find("unplanned");
        assert!(unplanned["schedule"].is_null());
        assert!(unplanned["schedule_reason"].is_string());
        // Nothing about other people's tasks appears.
        assert!(!mine.to_string().contains("bobs short"));
    })
    .await;
}
