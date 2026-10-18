//! Cross-plan task links (PJ-13) and what they must never reveal, end to end with
//! real PASETO identities: a private plan, and a guest whose grant covers one plan.
//! A dependency the caller cannot read **both** ends of shows as an opaque
//! "external dependency (restricted)" — no name, no pid, no date of the hidden
//! side — in the critical-path view, the dependency list, a refused plan delete,
//! and it cannot be created or removed by someone who cannot read it.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test cross_plan -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [51; 32];
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
fn mint(kid: &str, sub: &str, access: &[&str], roles: &[&str]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": { "access": access, "role": roles },
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

fn hdr(
    kid: &str,
    sub: &str,
    access: &[&str],
    roles: &[&str],
) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    auth_header(&mint(kid, sub, access, roles))
}

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";
const GUEST: &str = "3f2c6b7e-0000-4000-8000-0000000000e1";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test cross_plan -- --ignored`"]
async fn a_link_never_reveals_a_plan_its_reader_may_not_see() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        // Writers may delete too, so the handler's own readability rule is what is tested.
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let alice = hdr(&kid, ALICE, &["write"], &[]);
    let bob = hdr(&kid, BOB, &["write"], &[]);
    let guest = hdr(&kid, GUEST, &[], &["guest"]);
    let admin = hdr(&kid, ADMIN, &["admin"], &[]);
    request::<App, _, _>(|request, _ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let mk_plan = |who: &(axum::http::HeaderName, axum::http::HeaderValue), name: &'static str| {
            let (request, who) = (&request, who.clone());
            async move {
                let p: Value = request.post("/api/plans").add_header(who.0, who.1).json(&json!({ "kind": "Project", "name": name })).await.json();
                p["pid"].as_str().unwrap().to_string()
            }
        };
        let mk_task = |who: &(axum::http::HeaderName, axum::http::HeaderValue), plan: &str, title: &'static str, days: i64| {
            let (request, who, plan) = (&request, who.clone(), plan.to_string());
            async move {
                let t: Value = request.post(&format!("/api/plans/{plan}/tasks")).add_header(who.0, who.1)
                    .json(&json!({ "title": title, "start_date": "2026-10-05", "duration_days": days })).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let pa = mk_plan(&bob, "Public plan").await;
        let a1 = mk_task(&bob, &pa, "Public task", 2).await;
        let pc = mk_plan(&alice, "Secret plan").await;
        let c1 = mk_task(&alice, &pc, "Secret task", 4).await;
        // Alice's plan becomes private; Bob is not a member.
        with!(request.put(&format!("/api/plans/{pc}/access")), alice).json(&json!({ "private": true, "members": [] })).await.assert_status_ok();
        // Alice links her secret task to Bob's task: a legitimate link, readable by her.
        let link: Value = with!(request.post("/api/dependencies"), alice).json(&json!({ "predecessor_pid": c1, "successor_pid": a1, "lag_days": 0 })).await.json();
        let link = link["pid"].as_str().unwrap().to_string();

        let secrets = [pc.as_str(), c1.as_str(), "Secret plan", "Secret task"];
        let clean = |body: &str, what: &str| for s in secrets { assert!(!body.contains(s), "{what} leaks {s}: {body}"); };
        let node = |c: &Value, pid: &str| c["nodes"].as_array().unwrap().iter().find(|n| n["pid"] == pid).cloned();

        // ── a private plan ──
        let bob_view = with!(request.get(&format!("/api/plans/{pa}/cpm")), bob).await;
        let bob_text = bob_view.text();
        clean(&bob_text, "the critical-path view");
        let bob_cpm: Value = serde_json::from_str(&bob_text).unwrap();
        assert_eq!(bob_cpm["restricted_edges"].as_array().unwrap().len(), 1);
        assert_eq!(bob_cpm["restricted_edges"][0]["label"], "external dependency (restricted)");
        assert_eq!(node(&bob_cpm, &a1).unwrap()["early_start"], "2026-10-05", "the hidden side does not move his task");
        let alice_cpm: Value = with!(request.get(&format!("/api/plans/{pa}/cpm")), alice).await.json();
        assert_eq!(node(&alice_cpm, &c1).unwrap()["plan_name"], "Secret plan", "a member sees the other side");
        assert_eq!(node(&alice_cpm, &a1).unwrap()["early_start"], "2026-10-09");
        assert_eq!(alice_cpm["restricted_edges"].as_array().unwrap().len(), 0);
        let admin_cpm: Value = with!(request.get(&format!("/api/plans/{pa}/cpm")), admin).await.json();
        assert!(node(&admin_cpm, &c1).is_some(), "an admin reads every plan");

        // The dependency list, and creating and removing links.
        let deps = with!(request.get(&format!("/api/dependencies?item={a1}")), bob).await;
        let deps_text = deps.text();
        clean(&deps_text, "the dependency list");
        let deps: Value = serde_json::from_str(&deps_text).unwrap();
        assert_eq!(deps[0]["restricted"], true);
        assert!(deps[0].get("predecessor_pid").is_none() && deps[0].get("successor_pid").is_none());
        assert_eq!(with!(request.delete(&format!("/api/dependencies/{link}")), bob).await.status_code(), 404, "he cannot remove what he cannot see");
        assert_eq!(with!(request.post("/api/dependencies"), bob).json(&json!({ "predecessor_pid": c1, "successor_pid": a1, "lag_days": 1 })).await.status_code(), 404, "nor probe for it");
        let alice_deps: Value = with!(request.get(&format!("/api/dependencies?item={a1}")), alice).await.json();
        assert_eq!(alice_deps[0]["predecessor_pid"], c1.as_str());

        // Deleting the public plan: refused, and the refusal names nobody it may not.
        let del = with!(request.delete(&format!("/api/plans/{pa}")), bob).await;
        assert_eq!(del.status_code(), 409);
        let del_text = del.text();
        clean(&del_text, "the refused delete");
        assert!(del_text.contains("you cannot see"), "{del_text}");

        // ── a guest whose grant covers only the public plan ──
        let pb = mk_plan(&bob, "Ungranted plan").await;
        let b1 = mk_task(&bob, &pb, "Ungranted task", 3).await;
        with!(request.post("/api/dependencies"), bob).json(&json!({ "predecessor_pid": b1, "successor_pid": a1, "lag_days": 0 })).await.assert_status_ok();
        with!(request.post(&format!("/api/plans/{pa}/guests")), bob).json(&json!({ "guest_sub": GUEST, "access": "read" })).await.assert_status_ok();
        // A guest reaches only the plan record itself: the critical-path view and the
        // dependency list are closed to them, and the record names no linked plan.
        for path in [format!("/api/plans/{pa}/cpm"), format!("/api/dependencies?item={a1}"), format!("/api/plans/{pa}/schedule")] {
            assert_eq!(with!(request.get(&path), guest).await.status_code(), 404, "{path}");
        }
        let rec = with!(request.get(&format!("/api/plans/{pa}")), guest).await;
        assert_eq!(rec.status_code(), 200);
        let rec_text = rec.text();
        for s in [pb.as_str(), b1.as_str(), "Ungranted plan", "Ungranted task", pc.as_str(), "Secret plan"] {
            assert!(!rec_text.contains(s), "the guest's plan record leaks {s}: {rec_text}");
        }
        let gd = with!(request.delete(&format!("/api/dependencies/{link}")), guest).await.status_code().as_u16();
        assert!(gd == 403 || gd == 404, "a guest cannot remove a link: {gd}");
        // A member who can read the other plan sees it through the link.
        let bob_cpm: Value = with!(request.get(&format!("/api/plans/{pa}/cpm")), bob).await.json();
        assert!(node(&bob_cpm, &b1).is_some_and(|n| n["plan_name"] == "Ungranted plan"));
        assert_eq!(bob_cpm["restricted_edges"].as_array().unwrap().len(), 1, "and still not the private one");
    })
    .await;
}
