//! The drum and the pipeline (CC-5), end to end with real PASETO identities: the drum is named or the most
//! overloaded pool, two projects needing it at once are staggered with a stated reason and a capacity
//! buffer, nothing is moved, and a private plan is neither shown nor allowed to hold up another.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test drum_pipeline -- --ignored`.

#![expect(
    clippy::items_after_statements,
    clippy::many_single_char_names,
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
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a9";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test drum_pipeline -- --ignored`"]
async fn two_projects_needing_the_drum_are_staggered_and_nothing_is_moved() {
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
    let outsider = bearer(&kid, OUTSIDER, &[("access", "write")]);
    let first = {
        use chrono::Datelike;
        chrono::Utc::now().date_naive().with_day(1).unwrap()
    };
    let month = first.format("%Y-%m").to_string();
    let last = (first + chrono::Months::new(1)).pred_opt().unwrap();
    let month_days = (last - first).num_days() + 1;
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let get = |path: String, who: (axum::http::HeaderName, axum::http::HeaderValue)| {
            let request = &request;
            async move {
                let r = request.get(&path).add_header(who.0, who.1).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let mk = |name: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone())
                    .json(&json!({ "kind": "Project", "name": name })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let q = format!("?from={month}&months=2");

        // No pools: no drum, and the reason.
        let (_, v) = get(format!("/api/portfolio-pipeline{q}"), writer.clone()).await;
        assert_eq!((v["state"].as_str(), v["reason"].as_str()), (Some("no_drum"), Some("no_skill_pools")), "{v}");

        let (apollo, gemini, secret) = (mk("Apollo").await, mk("Gemini").await, mk("Secret").await);
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();
        let eng: Value = with!(request.post("/api/skill-pools"), writer).json(&json!({ "name": "Engineering", "roles": ["Developer"] })).await.json();
        let design: Value = with!(request.post("/api/skill-pools"), writer).json(&json!({ "name": "Design", "roles": ["Designer"] })).await.json();
        let (eng, design) = (eng["pid"].as_str().unwrap().to_string(), design["pid"].as_str().unwrap().to_string());
        ctx.db.execute_unprepared("INSERT INTO resource_roles (pid, person_ref, role, created_at, updated_at) VALUES (gen_random_uuid(), 'person:d1', 'Developer', now(), now())").await.unwrap();
        let alloc = |plan: &str, who: &str, percent: i32| format!("INSERT INTO allocations (pid, plan_pid, person_ref, role, percent, start_date, end_date, skills_required, created_at, updated_at) VALUES (gen_random_uuid(), '{plan}', '{who}', 'Developer', {percent}, '{first}', '{last}', '[]', now(), now())");
        for sql in [alloc(&apollo, "person:d1", 100), alloc(&gemini, "person:d2", 100), alloc(&secret, "person:s1", 100)] {
            ctx.db.execute_unprepared(&sql).await.unwrap();
        }

        // No drum named: the most overloaded pool (Engineering: 2.0 asked of 1.0).
        let (_, d) = get(format!("/api/portfolio-drum{q}"), writer.clone()).await;
        assert_eq!((d["drum"]["state"].as_str(), d["drum"]["name"].as_str(), d["drum"]["source"].as_str()), (Some("drum"), Some("Engineering"), Some("most_overloaded")), "{d}");

        let (code, v) = get(format!("/api/portfolio-pipeline{q}"), writer.clone()).await;
        assert_eq!(code, 200);
        assert_eq!(v["state"], "pipeline", "{v}");
        assert_eq!(v["advisory"], true);
        assert_eq!(v["capacity_fte"], 1.0);
        let projects = v["projects"].as_array().unwrap();
        let by = |name: &str| projects.iter().find(|p| p["name"] == name).cloned();
        assert!(by("Secret").is_none(), "a plan the caller may not read is nowhere: {v}");
        let (a, g) = (by("Apollo").unwrap(), by("Gemini").unwrap());
        assert_eq!(a["outcome"], "as_planned");
        assert_eq!(g["outcome"], "staggered", "{g}");
        assert_eq!(g["delay_days"], month_days + 7, "after Apollo's last use plus the 7-day buffer: {g}");
        let why = g["explanation"].as_str().unwrap();
        assert!(why.contains("Apollo") && why.contains("capacity buffer") && why.contains("no date has been changed"), "{why}");
        assert_eq!(v["staggered"], 1);
        let text = v.to_string();
        assert!(!text.contains("person:"), "no person appears: {text}");

        // The service proposes and never moves: the allocations are as they were.
        let n: i64 = {
            use sea_orm::{ConnectionTrait, Statement, DatabaseBackend};
            let r = ctx.db.query_one_raw(Statement::from_string(DatabaseBackend::Postgres, format!("SELECT count(*) AS n FROM allocations WHERE start_date = '{first}'"))).await.unwrap().unwrap();
            r.try_get("", "n").unwrap()
        };
        assert_eq!(n, 3);

        // Someone else sees the same, because the private plan is nowhere for them too.
        let (_, o) = get(format!("/api/portfolio-pipeline{q}"), outsider.clone()).await;
        assert_eq!(o["projects"], v["projects"]);

        // Naming a drum: Design has no supply recorded, so there is nothing to stagger against.
        assert_eq!(with!(request.put("/api/portfolio-drum"), writer).json(&json!({ "skill_pool_pid": uuid::Uuid::new_v4() })).await.status_code(), 422);
        with!(request.put("/api/portfolio-drum"), writer).json(&json!({ "skill_pool_pid": design })).await.assert_status_ok();
        let (_, v) = get(format!("/api/portfolio-pipeline{q}"), writer.clone()).await;
        assert_eq!((v["state"].as_str(), v["drum"]["source"].as_str(), v["drum"]["name"].as_str()), (Some("no_capacity"), Some("named"), Some("Design")), "{v}");
        // Clearing the naming returns to the most overloaded pool.
        with!(request.put("/api/portfolio-drum"), writer).json(&json!({ "skill_pool_pid": null })).await.assert_status_ok();
        let (_, d) = get(format!("/api/portfolio-drum{q}"), writer.clone()).await;
        assert_eq!(d["drum"]["name"], "Engineering");
        assert_eq!(d["history"].as_array().unwrap().len(), 2);
        let _ = eng;

        assert_eq!(get("/api/portfolio-pipeline?months=0".into(), writer.clone()).await.0, 422);
        assert_eq!(get("/api/portfolio-pipeline?from=nope".into(), writer.clone()).await.0, 422);
        use sea_orm::ConnectionTrait;
        assert!(ctx.db.execute_unprepared("DELETE FROM portfolio_drums").await.is_err(), "the history is append-only");
    })
    .await;
}
