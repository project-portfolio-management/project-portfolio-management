//! The administrator's "today" time zone, end to end with real PASETO identities: anyone signed in can
//! read it, it starts as UTC, only an administrator can change it (`403` otherwise, `422` for a name that
//! is not an IANA zone), a change is audited with the before and after, and an instance that missed the
//! change converges on the stored value when it next refreshes.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test today_time_zone -- --ignored`.

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
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test today_time_zone -- --ignored`"]
async fn an_administrator_sets_the_zone_that_defines_today() {
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
    let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
    request::<App, _, _>(|request, ctx| async move {
        let url = "/api/settings/today-time-zone";
        // Start from nothing stored, whatever an earlier run left.
        ctx.db
            .execute_unprepared("DELETE FROM deployment_settings WHERE key = 'today_time_zone'")
            .await
            .unwrap();
        project_portfolio_management::controllers::settings::refresh(&ctx.db).await.unwrap();

        // Default: UTC, and it says so.
        let got = request.get(url).add_header(writer.0.clone(), writer.1.clone()).await;
        got.assert_status_ok();
        let v: Value = got.json();
        assert_eq!((v["time_zone"].as_str(), v["source"].as_str()), (Some("UTC"), Some("default")));
        assert_eq!(v["today"], chrono::Utc::now().date_naive().to_string());

        // A writer who is not an administrator cannot change it.
        let r = request.put(url).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "time_zone": "Pacific/Auckland" })).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.delete(url).add_header(writer.0.clone(), writer.1.clone()).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());

        // An administrator cannot set a fixed offset, an abbreviation or nonsense.
        for bad in ["+02:00", "PST", "Mars/Olympus", ""] {
            let r = request.put(url).add_header(admin.0.clone(), admin.1.clone()).json(&json!({ "time_zone": bad })).await;
            assert_eq!(r.status_code(), 422, "{bad:?}: {}", r.text());
        }
        assert_eq!(request.get(url).add_header(admin.0.clone(), admin.1.clone()).await.json::<Value>()["time_zone"], "UTC");

        // An administrator sets Honolulu (UTC-10, no daylight saving): "today" follows it.
        let r = request.put(url).add_header(admin.0.clone(), admin.1.clone()).json(&json!({ "time_zone": "Pacific/Honolulu" })).await;
        r.assert_status_ok();
        let v: Value = r.json();
        assert_eq!((v["time_zone"].as_str(), v["source"].as_str(), v["updated_by"].as_str()), (Some("Pacific/Honolulu"), Some("admin"), Some(ADMIN)));
        let honolulu: chrono_tz::Tz = "Pacific/Honolulu".parse().unwrap();
        assert_eq!(v["today"], chrono::Utc::now().with_timezone(&honolulu).date_naive().to_string());

        // The change is audited with the before and after.
        let row = ctx.db.query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT snapshot::text AS s FROM audit_logs WHERE action = 'today_time_zone_changed' ORDER BY id DESC LIMIT 1",
        )).await.unwrap().expect("an audit row");
        let snap: Value = serde_json::from_str(&row.try_get::<String>("", "s").unwrap()).unwrap();
        assert_eq!((snap["before"].as_str(), snap["after"].as_str()), (Some("UTC"), Some("Pacific/Honolulu")));

        // Another instance that missed the change picks it up from the table on its next refresh.
        project_portfolio_management::tz::set_today_zone(chrono_tz::UTC);
        project_portfolio_management::controllers::settings::refresh(&ctx.db).await.unwrap();
        assert_eq!(project_portfolio_management::tz::today_zone(), honolulu);

        // Back to the default.
        let r = request.delete(url).add_header(admin.0.clone(), admin.1.clone()).await;
        r.assert_status_ok();
        let v: Value = r.json();
        assert_eq!((v["time_zone"].as_str(), v["source"].as_str()), (Some("UTC"), Some("default")));
        project_portfolio_management::controllers::settings::refresh(&ctx.db).await.unwrap();
        assert_eq!(project_portfolio_management::tz::today_zone(), chrono_tz::UTC);
    })
    .await;
}
