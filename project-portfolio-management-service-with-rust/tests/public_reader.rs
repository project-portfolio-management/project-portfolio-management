//! RE-4: the public reader of public product roadmaps, with no token, through the real login guard. It serves only
//! a published roadmap of a live plan that is neither private nor marked above the lowest level; every other case is
//! the same `404`; it carries an `ETag` and caching; an unpublish shows on the next read; and it is off unless turned
//! on.
//!
//! Its **own test binary** (the verifier and the mark scheme are process-wide). `#[ignore]`d: needs PostgreSQL.
//! `cargo test --features paseto --test public_reader -- --ignored`.

#![cfg(feature = "paseto")]

use axa_test::prelude::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [89; 32];
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
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test --test public_reader -- --ignored`"]
#[expect(clippy::too_many_lines, reason = "one linear end-to-end story of the reader")]
async fn the_public_reader_serves_only_what_was_published_from_an_open_plan() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS",
            r#"["Public","Internal"]"#,
        );
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS", "on");
    }
    let lead = bearer(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000e1",
        &[("access", "write"), ("clearance", "Internal")],
    );
    request::<App, _, _>(|request, _ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let as_lead = |r: axum_test::TestRequest| r.add_header(lead.0.clone(), lead.1.clone());
        // A product with a published roadmap, an unplaced item and a removed one; and a Welsh translation.
        let product = |name: String| {
            let request = &request;
            let lead = lead.clone();
            async move {
                let p: Value = request.post("/api/plans").add_header(lead.0, lead.1)
                    .json(&json!({ "kind": "Product", "name": name })).await.json();
                p["pid"].as_str().unwrap().to_string()
            }
        };
        let open = product(format!("Open product {tag}")).await;
        let url = format!("/api/plans/{open}/roadmap");
        as_lead(request.put(&url)).json(&json!({ "title": format!("Open roadmap {tag}"), "vision": "Shown" }))
            .await.assert_status_ok();
        let org: Value = as_lead(request.post("/api/organizations"))
            .json(&json!({ "name": format!("Board {tag}"), "kind": "partner", "code": format!("B{tag}") })).await.json();
        let shown: Value = as_lead(request.post(&format!("{url}/items")))
            .json(&json!({ "title": "Shown item", "placement": "now", "organization_pids": [org["pid"]] })).await.json();
        as_lead(request.post(&format!("{url}/items"))).json(&json!({ "title": "Unplaced item" })).await.assert_status_ok();
        let gone: Value = as_lead(request.post(&format!("{url}/items")))
            .json(&json!({ "title": "Removed item", "placement": "next" })).await.json();
        as_lead(request.post(&format!("{url}/items/{}/delete", gone["pid"].as_str().unwrap()))).await.assert_status_ok();
        as_lead(request.put(&format!("{url}/items/{}/translations/cy", shown["pid"].as_str().unwrap())))
            .json(&json!({ "title": "Eitem a ddangosir" })).await.assert_status_ok();
        let slug = format!("open-roadmap-{tag}");
        let public = format!("/api/public/roadmaps/{slug}");

        // Unpublished: the same 404 as one that does not exist.
        let unpublished = request.get(&public).await;
        assert_eq!(unpublished.status_code(), 404);
        let missing = request.get(&format!("/api/public/roadmaps/no-such-{tag}")).await;
        assert_eq!(missing.status_code(), 404);
        assert_eq!(unpublished.text(), missing.text(), "the two misses look the same");

        as_lead(request.put(&url)).json(&json!({ "published": true })).await.assert_status_ok();
        // Published: readable with no token at all.
        let r = request.get(&public).await;
        r.assert_status_ok();
        assert_eq!(r.header("cache-control"), "public, max-age=60");
        let etag = r.header("etag").to_str().unwrap().to_string();
        let body: Value = r.json();
        assert_eq!(body["title"], format!("Open roadmap {tag}"));
        let titles: Vec<&str> = body["items"].as_array().unwrap().iter().map(|i| i["title"].as_str().unwrap()).collect();
        assert_eq!(titles, ["Shown item"], "no unplaced and no removed item");
        assert_eq!(body["items"][0]["organizations"], json!([{ "code": format!("B{tag}"), "name": format!("Board {tag}"), "national": false }]));
        assert_eq!(body["items"][0]["translations"]["cy"]["title"]["value"], "Eitem a ddangosir");
        let text = body.to_string();
        for internal in [open.as_str(), shown["pid"].as_str().unwrap(), "plan_pid", "\"pid\"", "created_at", "source", "published"] {
            assert!(!text.contains(internal), "the reader serves {internal}: {text}");
        }
        let same = request.get(&public).add_header(axum::http::header::IF_NONE_MATCH, etag.parse::<axum::http::HeaderValue>().unwrap()).await;
        assert_eq!(same.status_code(), 304);
        let listed: Value = request.get("/api/public/roadmaps").await.json();
        assert!(listed["roadmaps"].as_array().unwrap().iter().any(|r| r["slug"] == slug.as_str()));

        // A private plan's roadmap and a marked plan's roadmap are not served, though published.
        let private = product(format!("Private product {tag}")).await;
        as_lead(request.put(&format!("/api/plans/{private}/roadmap")))
            .json(&json!({ "title": format!("Private roadmap {tag}"), "published": true })).await.assert_status_ok();
        as_lead(request.put(&format!("/api/plans/{private}/access")))
            .json(&json!({ "private": true, "members": [] })).await.assert_status_ok();
        let marked = product(format!("Marked product {tag}")).await;
        as_lead(request.put(&format!("/api/plans/{marked}/roadmap")))
            .json(&json!({ "title": format!("Marked roadmap {tag}"), "published": true })).await.assert_status_ok();
        as_lead(request.put(&format!("/api/plans/{marked}/handling-mark")))
            .json(&json!({ "mark": "Internal" })).await.assert_status_ok();
        for s in [format!("private-roadmap-{tag}"), format!("marked-roadmap-{tag}")] {
            let r = request.get(&format!("/api/public/roadmaps/{s}")).await;
            assert_eq!(r.status_code(), 404, "{s}");
            assert_eq!(r.text(), missing.text(), "{s} looks like a miss");
        }
        let listed: Value = request.get("/api/public/roadmaps").await.json();
        let slugs: Vec<&str> = listed["roadmaps"].as_array().unwrap().iter().map(|r| r["slug"].as_str().unwrap()).collect();
        assert!(!slugs.iter().any(|s| s.contains(&format!("private-roadmap-{tag}")) || s.contains(&format!("marked-roadmap-{tag}"))));

        // RE-8: a new slug keeps the old one as a redirect, and no other roadmap may take the old one.
        as_lead(request.put(&url)).json(&json!({ "slug": format!("open-{tag}") })).await.assert_status_ok();
        let old = request.get(&public).await;
        assert_eq!(old.status_code(), 301);
        assert_eq!(old.header("location"), format!("/api/public/roadmaps/open-{tag}").as_str());
        request.get(&format!("/api/public/roadmaps/open-{tag}")).await.assert_status_ok();
        let other = product(format!("Another product {tag}")).await;
        let taken = as_lead(request.put(&format!("/api/plans/{other}/roadmap")))
            .json(&json!({ "title": "Another", "slug": slug })).await;
        assert_eq!(taken.status_code(), 409, "an old slug is never given to another roadmap");
        // The roadmap may take its own old slug back.
        as_lead(request.put(&url)).json(&json!({ "slug": slug })).await.assert_status_ok();
        request.get(&public).await.assert_status_ok();
        assert_eq!(request.get(&format!("/api/public/roadmaps/open-{tag}")).await.status_code(), 301);
        // A redirect never points at a roadmap that is not served.
        as_lead(request.put(&format!("/api/plans/{private}/roadmap"))).json(&json!({ "slug": format!("private-new-{tag}") })).await.assert_status_ok();
        assert_eq!(request.get(&format!("/api/public/roadmaps/private-roadmap-{tag}")).await.status_code(), 404);

        // Unpublished again: gone on the next read. And the reader can be turned off.
        as_lead(request.put(&url)).json(&json!({ "published": false })).await.assert_status_ok();
        assert_eq!(request.get(&public).await.status_code(), 404);
        as_lead(request.put(&url)).json(&json!({ "published": true })).await.assert_status_ok();
        // SAFETY: as above.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PUBLIC_ROADMAPS", "off") };
        assert_eq!(request.get(&public).await.status_code(), 404);
        assert_eq!(request.get("/api/public/roadmaps").await.status_code(), 404);
        // The editor's routes still need a token.
        assert_eq!(request.get(&url).await.status_code(), 401);
    })
    .await;
}
