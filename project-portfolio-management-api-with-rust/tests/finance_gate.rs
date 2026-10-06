//! The finance and announcement areas, end to end with real PASETO identities:
//! the policy (with the service's own rules in front) decides who may see rates
//! and labour cost and who may publish announcements.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test finance_gate -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [41; 32];
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
fn mint(kid: &str, sub: &str, attrs: &[(&str, &str)]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": attrs.iter().fold(serde_json::Map::new(), |mut m, (k, v)| { m.insert((*k).to_string(), json!([v])); m }),
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

fn bearer(
    kid: &str,
    sub: &str,
    attrs: &[(&str, &str)],
) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    auth_header(&mint(kid, sub, attrs))
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test finance_gate -- --ignored`"]
async fn rates_labour_cost_and_publishing_follow_the_policy() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        // A deployment policy that would, by itself, let every writer do everything:
        // the service's own rules must still hold.
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let writer = bearer(&kid, "writer", &[("access", "write")]);
    let admin = bearer(&kid, "ops", &[("access", "admin")]);
    let fin = bearer(
        &kid,
        "fin",
        &[("access", "write"), ("finance", "cost_rates")],
    );
    let publisher = bearer(&kid, "pub", &[("access", "write"), ("comms", "publish")]);
    request::<App, _, _>(|request, _ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }

        // Rates: invisible (404) to a writer, open to finance and to an admin.
        assert_eq!(with!(request.get("/api/cost-rate-tables"), writer).await.status_code(), 404);
        let body = json!({ "name": "standard", "role": "Engineer", "currency": "GBP" });
        assert_eq!(with!(request.post("/api/cost-rate-tables"), writer).json(&body).await.status_code(), 404);
        let t: Value = with!(request.post("/api/cost-rate-tables"), fin).json(&body).await.json();
        let table = t["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/cost-rate-tables/{table}/rates")), admin)
            .json(&json!({ "effective_from": "2026-01-01", "per_hour_minor": 5000 })).await.assert_status_ok();
        assert_eq!(with!(request.get("/api/cost-rate-tables"), fin).await.status_code(), 200);
        assert_eq!(with!(request.get("/api/cost-rates/lookup?person_ref=person:00000000-0000-4000-8000-0000000000c1&date=2026-10-05"), writer).await.status_code(), 404);

        // Announcements: everyone reads; only a publisher (or admin) writes.
        assert_eq!(with!(request.get("/api/announcements"), writer).await.status_code(), 200);
        let note = json!({ "title": "Hello", "body": "World" });
        assert_eq!(with!(request.post("/api/announcements"), writer).json(&note).await.status_code(), 403);
        assert_eq!(with!(request.get("/api/announcements?include=all"), writer).await.status_code(), 403);
        let a: Value = with!(request.post("/api/announcements"), publisher).json(&note).await.json();
        let pid = a["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.get("/api/announcements?include=all"), publisher).await.status_code(), 200);
        assert_eq!(with!(request.delete(&format!("/api/announcements/{pid}")), writer).await.status_code(), 403);
        with!(request.delete(&format!("/api/announcements/{pid}")), admin).await.assert_status_ok();

        // Earned value from labour cost is finance data; the default is not.
        let plan: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}/earned-value")), writer).await.status_code(), 200);
        for ac in ["labour", "both"] {
            assert_eq!(with!(request.get(&format!("/api/plans/{p}/earned-value?ac={ac}")), writer).await.status_code(), 403, "{ac}");
            assert_eq!(with!(request.get(&format!("/api/plans/{p}/earned-value?ac={ac}")), fin).await.status_code(), 200, "{ac}");
        }
        assert_eq!(with!(request.get(&format!("/api/plans/{p}/earned-value?ac=nonsense")), fin).await.status_code(), 422);

        // The what-if sandbox applies the same gates: no cost and no assignee for a plain
        // writer, a hard constraint only for who may set one, and a refusal writes nothing.
        let t: Value = with!(request.post(&format!("/api/plans/{p}/tasks")), writer)
            .json(&json!({ "title": "Ship", "start_date": "2026-10-05", "duration_days": 3,
                           "assignee_ref": "person:00000000-0000-4000-8000-0000000000f1" }))
            .await.json();
        let t = t["pid"].as_str().unwrap().to_string();
        let url = format!("/api/plans/{p}/sandbox/preview");
        let edit = json!({ "edits": [{ "op": "duration", "task_pid": t, "duration_days": 5 }] });
        let as_writer: Value = with!(request.post(&url), writer).json(&edit).await.json();
        assert_eq!(as_writer["masked"], true);
        assert_eq!(as_writer["cost_impact"]["reason"], "no_finance_access");
        assert!(as_writer["tasks"][0].get("assignee").is_none(), "who does the work stays masked: {as_writer}");
        let as_admin: Value = with!(request.post(&url), admin).json(&edit).await.json();
        assert_eq!(as_admin["masked"], false);
        assert!(as_admin["tasks"][0].get("assignee").is_some());
        assert_ne!(as_admin["cost_impact"]["reason"], "no_finance_access");
        let hard = json!({ "edits": [{ "op": "constraint", "task_pid": t, "constraint_kind": "mso",
            "constraint_date": "2026-10-07", "reason": "contractual" }] });
        assert_eq!(with!(request.post(&url), writer).json(&hard).await.status_code(), 403);
        with!(request.post(&url), admin).json(&hard).await.assert_status_ok();
        let commit = format!("/api/plans/{p}/sandbox/commit");
        let etag = with!(request.post(&url), writer).json(&edit).await.header("etag").to_str().unwrap().to_string();
        assert_eq!(with!(request.post(&commit), writer).json(&hard).add_header(axum::http::header::IF_MATCH, etag.parse::<axum::http::HeaderValue>().unwrap()).await.status_code(), 403, "a commit is gated like a preview");

        // Onboarding: a plan's own checklist is open to a writer; anything about a person
        // (joiners and leavers, access, handover) needs the permission to manage people.
        let person = "person:00000000-0000-4000-8000-0000000000f2";
        let cases = format!("/api/plans/{p}/onboarding/cases");
        let w_plan: Value = with!(request.post(&cases), writer).json(&json!({ "kind": "plan_closeout", "effective_date": "2026-10-12" })).await.json();
        assert_eq!(w_plan["masked"], true);
        let leave = json!({ "kind": "person_leave", "subject_ref": person, "effective_date": "2026-10-12" });
        assert_eq!(with!(request.post(&cases), writer).json(&leave).await.status_code(), 403);
        let made: Value = with!(request.post(&cases), admin).json(&leave).await.json();
        let case = made["pid"].as_str().unwrap().to_string();
        assert_eq!(made["subject_ref"], person);
        assert_eq!(with!(request.get(&format!("/api/onboarding/cases/{case}")), writer).await.status_code(), 404, "a person's case does not exist for a writer");
        assert_eq!(with!(request.get(&format!("/api/onboarding/cases/{case}")), admin).await.status_code(), 200);
        let listed: Value = with!(request.get(&cases), writer).await.json();
        assert_eq!(listed["hidden_people_cases"], 1);
        assert_eq!(listed["cases"].as_array().unwrap().len(), 1, "only the plan's own checklist");
        let access = format!("/api/plans/{p}/access-records");
        let grant = json!({ "person_ref": person, "access": "repo write", "action": "granted" });
        assert_eq!(with!(request.post(&access), writer).json(&grant).await.status_code(), 403);
        assert_eq!(with!(request.get(&access), writer).await.status_code(), 403);
        with!(request.post(&access), admin).json(&grant).await.assert_status_ok();
        let hand = json!({ "from_person": person, "to_person": null });
        assert_eq!(with!(request.post(&format!("/api/plans/{p}/handover/preview")), writer).json(&hand).await.status_code(), 403);
        with!(request.post(&format!("/api/plans/{p}/handover/preview")), admin).json(&hand).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}/closeout")), writer).await.status_code(), 200, "readiness names no one");

        // A task read with `include`: what names people is masked for a caller who may not manage them.
        let task_url = format!("/api/plans/{p}/tasks/{t}");
        let as_writer: Value = with!(request.get(&format!("{task_url}?include=children,raci,time_entries,transitions")), writer).await.json();
        assert!(as_writer["included"].get("children").is_some());
        let masked: Vec<&str> = as_writer["omitted"].as_array().unwrap().iter().map(|o| o["name"].as_str().unwrap()).collect();
        assert_eq!(masked, vec!["raci", "time_entries", "transitions"], "{as_writer}");
        let as_admin: Value = with!(request.get(&format!("{task_url}?include=raci,time_entries,transitions")), admin).await.json();
        assert!(as_admin["omitted"].as_array().unwrap().is_empty());

        // Only the requester or an admin may cancel a bulk job; anyone else gets the 404 an unknown id would.
        let imported = with!(request.post("/api/plans/import?format=jsonl&on_keyless=create"), writer)
            .content_type("text/plain")
            .text(r#"{"name":"Imported","kind":"Project","code":"IMP-1"}"#)
            .await;
        let job_id = imported.json::<Value>()["job_id"].as_str().unwrap().to_string();
        let cancel_url = format!("/api/plans/bulk-jobs/{job_id}/cancel");
        assert_eq!(with!(request.post(&cancel_url), fin).await.status_code(), 404, "not the requester, not an admin");
        assert_eq!(with!(request.post(&cancel_url), writer).await.status_code(), 200, "the requester");
        assert_eq!(with!(request.post(&cancel_url), admin).await.status_code(), 200, "an admin");
    })
    .await;
}
