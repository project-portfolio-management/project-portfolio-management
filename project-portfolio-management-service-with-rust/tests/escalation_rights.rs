//! The right to acknowledge and the right to resolve an escalation (CF-4), end to end with real PASETO
//! identities: each is held by an administrator or by its own attribute (`escalation=acknowledge`,
//! `escalation=resolve`) and a plain writer holds neither (`403`, on the `POST` and on the `PUT` alike);
//! the acknowledgement and the resolution are resources with `GET` and `PUT`; repeating a `PUT` records
//! nothing new; a resolution is not overwritten; taking an escalation up a level clears the
//! acknowledgement; and each act leaves a history event and an audit entry.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test escalation_rights -- --ignored`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [53; 32];
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

const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";
const ACKER: &str = "3f2c6b7e-0000-4000-8000-0000000000a2";
const RESOLVER: &str = "3f2c6b7e-0000-4000-8000-0000000000a3";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test escalation_rights -- --ignored`"]
async fn acknowledging_and_resolving_are_separate_audited_rights() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier or these settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let writer = bearer(&kid, WRITER, &[("access", "write")]);
    let acker = bearer(
        &kid,
        ACKER,
        &[("access", "write"), ("escalation", "acknowledge")],
    );
    let resolver = bearer(
        &kid,
        RESOLVER,
        &[("access", "write"), ("escalation", "resolve")],
    );
    let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
    request::<App, _, _>(|request, ctx| async move {
        let as_ = |who: &(axum::http::HeaderName, axum::http::HeaderValue)| who.clone();
        let plan: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "kind": "Project", "name": "Escalation rights" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let raise = |title: &str| {
            let (request, url, who, title) = (&request, format!("/api/plans/{p}/escalations"), as_(&writer), title.to_string());
            async move {
                let r = request.post(&url).add_header(who.0, who.1).json(&json!({ "title": title })).await;
                assert_eq!(r.status_code(), 200, "{}", r.text());
                r.json::<Value>()["pid"].as_str().unwrap().to_string()
            }
        };
        let e = raise("Slip on the supplier").await;
        let ack = format!("/api/plans/{p}/escalations/{e}/acknowledgement");
        let res = format!("/api/plans/{p}/escalations/{e}/resolution");

        // Neither resource exists yet.
        assert_eq!(request.get(&ack).add_header(writer.0.clone(), writer.1.clone()).await.status_code(), 404);
        assert_eq!(request.get(&res).add_header(writer.0.clone(), writer.1.clone()).await.status_code(), 404);

        // A plain writer holds neither right, by the resource and by the older POST.
        let r = request.put(&ack).add_header(writer.0.clone(), writer.1.clone()).json(&json!({})).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.post(&format!("/api/plans/{p}/escalations/{e}/acknowledge")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({})).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let body = json!({ "outcome": "decided", "response": "Approved by the board." });
        let r = request.put(&res).add_header(writer.0.clone(), writer.1.clone()).json(&body).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        // The right to acknowledge is not the right to resolve, nor the reverse.
        let r = request.put(&res).add_header(acker.0.clone(), acker.1.clone()).json(&body).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.put(&ack).add_header(resolver.0.clone(), resolver.1.clone()).json(&json!({})).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        assert_eq!(request.get(&format!("/api/plans/{p}/escalations/{e}")).add_header(writer.0.clone(), writer.1.clone()).await.json::<Value>()["status"], "open", "nothing changed");

        // The holder acknowledges: who, when, what they said.
        let r = request.put(&ack).add_header(acker.0.clone(), acker.1.clone()).json(&json!({ "note": "Seen, looking into it" })).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        let first: Value = r.json();
        assert_eq!((first["acknowledged_by"].as_str(), first["note"].as_str(), first["level"].as_i64()), (Some(ACKER), Some("Seen, looking into it"), Some(1)));
        // Anyone who can read it can read it back, the same.
        let got: Value = request.get(&ack).add_header(writer.0.clone(), writer.1.clone()).await.json();
        assert_eq!(got, first);
        // Repeating the PUT records nothing new.
        let again: Value = request.put(&ack).add_header(admin.0.clone(), admin.1.clone()).json(&json!({ "note": "me too" })).await.json();
        assert_eq!(again, first, "the acknowledgement already there is answered");
        let one: Value = request.get(&format!("/api/plans/{p}/escalations/{e}")).add_header(writer.0.clone(), writer.1.clone()).await.json();
        let events: Vec<&str> = one["events"].as_array().unwrap().iter().map(|x| x["event"].as_str().unwrap()).collect();
        assert_eq!(events, ["raised", "acknowledged"]);

        // The resolver resolves; the resolution is readable, and not overwritten.
        let r = request.put(&res).add_header(resolver.0.clone(), resolver.1.clone()).json(&json!({ "outcome": "decided", "response": "short" })).await;
        assert_eq!(r.status_code(), 422, "{}", r.text());
        let r = request.put(&res).add_header(resolver.0.clone(), resolver.1.clone()).json(&body).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        let resolution: Value = r.json();
        assert_eq!((resolution["outcome"].as_str(), resolution["resolved_by"].as_str(), resolution["response"].as_str()), (Some("decided"), Some(RESOLVER), Some("Approved by the board.")));
        assert_eq!(request.get(&res).add_header(writer.0.clone(), writer.1.clone()).await.json::<Value>(), resolution);
        let same: Value = request.put(&res).add_header(admin.0.clone(), admin.1.clone()).json(&body).await.json();
        assert_eq!(same, resolution, "the same resolution again is the one already there");
        let r = request.put(&res).add_header(admin.0.clone(), admin.1.clone()).json(&json!({ "outcome": "rejected", "response": "Changed my mind entirely." })).await;
        assert_eq!(r.status_code(), 409, "{}", r.text());
        assert_eq!(request.get(&res).add_header(writer.0.clone(), writer.1.clone()).await.json::<Value>(), resolution, "still the first");
        // A resolved escalation keeps the acknowledgement it had.
        assert_eq!(request.get(&ack).add_header(writer.0.clone(), writer.1.clone()).await.json::<Value>(), first);

        // Each act is in the history and in the audit trail, with what was said.
        let audit = |action: &'static str| {
            let db = ctx.db.clone();
            async move {
                let row = db.query_one_raw(sea_orm::Statement::from_string(
                    sea_orm::DatabaseBackend::Postgres,
                    format!("SELECT actor, snapshot::text AS s FROM audit_logs WHERE action = '{action}' ORDER BY id DESC LIMIT 1"),
                )).await.unwrap().unwrap_or_else(|| panic!("an audit row for {action}"));
                (row.try_get::<Option<String>>("", "actor").unwrap(), serde_json::from_str::<Value>(&row.try_get::<String>("", "s").unwrap()).unwrap())
            }
        };
        let (actor, snap) = audit("escalation_acknowledged").await;
        assert_eq!((actor.as_deref(), snap["level"].as_i64(), snap["note"].as_str()), (Some(ACKER), Some(1), Some("Seen, looking into it")));
        let (actor, snap) = audit("escalation_resolved").await;
        assert_eq!((actor.as_deref(), snap["outcome"].as_str()), (Some(RESOLVER), Some("decided")));

        // Taking an escalation up a level sends it back to open: the old acknowledgement is not the new level's.
        let e2 = raise("Second slip").await;
        let ack2 = format!("/api/plans/{p}/escalations/{e2}/acknowledgement");
        request.put(&ack2).add_header(acker.0.clone(), acker.1.clone()).json(&json!({})).await.assert_status_ok();
        let r = request.post(&format!("/api/plans/{p}/escalations/{e2}/raise-level")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "note": "Needs the next level up" })).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        assert_eq!(request.get(&ack2).add_header(writer.0.clone(), writer.1.clone()).await.status_code(), 404, "the new level has not acknowledged it");
        let renewed: Value = request.put(&ack2).add_header(acker.0.clone(), acker.1.clone()).json(&json!({})).await.json();
        assert_eq!(renewed["level"].as_i64(), Some(2));
    })
    .await;
}
