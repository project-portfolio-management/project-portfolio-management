//! Portfolio earned value over the threshold (DP-8), end to end with real PASETO identities:
//! figures equal to the roll-up endpoint for the same date, a programme under the budget threshold shown
//! as below it (and exactly at it as in), a programme with no baseline as no baseline, a bad threshold
//! setting named, the actual-cost source stated, a private programme that is nowhere, and the pack
//! carrying the section.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test portfolio_evm -- --ignored`.
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
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a9";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test portfolio_evm -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn earned_value_over_the_threshold_keeps_its_promises() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier or these settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS",
            r#"{"GBP":1000}"#,
        );
    }
    let writer = bearer(&kid, WRITER, &[("access", "write")]);
    let issuer = bearer(
        &kid,
        ISSUER_SUB,
        &[("access", "write"), ("directorpack", "issuer")],
    );
    let outsider = bearer(&kid, OUTSIDER, &[("access", "write")]);
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let mk = |kind: &'static str, name: &'static str, parent: Option<String>| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let mut b = json!({ "kind": kind, "name": name });
                if let Some(p) = parent { b["parent_ref"] = json!(p); }
                let v: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone()).json(&b).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        // A project with a baseline of `bac` (half before today, half after), four tasks of which two are
        // done, and `ac` spent: EV is half the BAC.
        let today = chrono::Utc::now().date_naive();
        let day = |n: i64| (today + chrono::Duration::days(n)).to_string();
        let costed = |parent: String, name: &'static str, bac: i64, ac: i64| {
            let (request, writer, day) = (&request, writer.clone(), day);
            let mk = &mk;
            async move {
                let p = mk("Project", name, Some(parent)).await;
                let w = |rb: axum_test::TestRequest| rb.add_header(writer.0.clone(), writer.1.clone());
                w(request.post(&format!("/api/plans/{p}/budget-baselines"))).json(&json!({ "currency": "GBP", "periods": [
                    { "period_start": day(-9), "period_end": day(0), "planned_minor": bac / 4 },
                    { "period_start": day(1), "period_end": day(10), "planned_minor": bac - bac / 4 } ] })).await.assert_status_ok();
                for i in 0..4 {
                    let t: Value = w(request.post(&format!("/api/plans/{p}/tasks"))).json(&json!({ "title": format!("T{i}") })).await.json();
                    if i < 2 {
                        w(request.patch(&format!("/api/plans/{p}/tasks/{}", t["pid"].as_str().unwrap()))).json(&json!({ "status": "done" })).await.assert_status_ok();
                    }
                }
                let line: Value = w(request.post(&format!("/api/plans/{p}/budget-lines"))).json(&json!({ "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": bac })).await.json();
                w(request.post(&format!("/api/plans/{p}/budget-lines/{}/actual", line["pid"].as_str().unwrap()))).json(&json!({ "amount_minor": ac })).await.assert_status_ok();
                p
            }
        };
        let big = mk("Program", "Big", None).await;
        let small = mk("Program", "Small", None).await;
        let bare = mk("Program", "Bare", None).await;
        let secret = mk("Program", "Secret", None).await;
        costed(big.clone(), "Big work", 4000, 2500).await;
        costed(small.clone(), "Small work", 500, 100).await;
        let hidden_work = costed(secret.clone(), "Secret work", 9_000_000, 1).await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();
        // Privacy is per plan and is not inherited by what sits beneath, so the work is made private too.
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{hidden_work}', true, '[]', now(), now())")).await.unwrap();

        let live = |q: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r = request.get(&format!("/api/portfolio-earned-value{q}")).add_header(writer.0.clone(), writer.1.clone()).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let programme = |v: &Value, name: &str| v["programmes"].as_array().unwrap().iter().find(|p| p["name"] == name).cloned().unwrap_or(Value::Null);
        let (_, v) = live("").await;
        let b = programme(&v, "Big");
        let gbp = &b["currencies"][0];
        assert_eq!(gbp["state"], "included");
        assert_eq!((gbp["totals"]["bac_minor"].as_i64(), gbp["totals"]["ev_minor"].as_i64(), gbp["totals"]["ac_minor"].as_i64()), (Some(4000), Some(2000), Some(2500)), "{b}");
        assert_eq!((gbp["spi"]["bp"].as_i64(), gbp["cpi"]["bp"].as_i64()), (Some(20_000), Some(8000)));
        assert_eq!(gbp["ac_source"], "budget_line_actuals");
        assert_eq!(gbp["eac"]["variants_used"], json!(["cpi"]), "the variant used is named");
        assert_eq!(gbp["eac"]["minor"], 5000);
        // The same figures as the roll-up endpoint for the same date.
        let roll: Value = with!(request.get(&format!("/api/plans/{big}/earned-value/rollup")), writer).await.json();
        let r0 = &roll["currencies"][0];
        for k in ["currency", "totals", "spi", "cpi", "eac", "vac_minor"] {
            assert_eq!(gbp[k], r0[k], "{k} equals the roll-up");
        }
        // A programme under the threshold is below it, with no ratios; a programme with no baseline says so.
        let s = programme(&v, "Small");
        assert_eq!(s["currencies"][0]["state"], "below EVM threshold");
        assert!(s["currencies"][0]["spi"].is_null() && s["currencies"][0]["cpi"].is_null());
        assert_eq!((s["currencies"][0]["bac_minor"].as_i64(), s["currencies"][0]["threshold_minor"].as_i64()), (Some(500), Some(1000)));
        let n = programme(&v, "Bare");
        assert_eq!(n["state"], "no_baseline");
        assert!(n["currencies"].is_null(), "no figures are invented");
        assert_eq!(v["bac_inclusion_threshold"]["minor_by_currency"]["GBP"], 1000);
        assert_eq!(v["bac_inclusion_threshold"]["configured"], true);

        // Exactly at the threshold is in; one over is out.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS", r#"{"GBP":4000}"#) };
        assert_eq!(programme(&live("").await.1, "Big")["currencies"][0]["state"], "included");
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS", r#"{"GBP":4001}"#) };
        assert_eq!(programme(&live("").await.1, "Big")["currencies"][0]["state"], "below EVM threshold");
        // A setting that cannot be used is named, and nothing is silently dropped.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS", "lots") };
        let bad = live("").await.1;
        assert_eq!(bad["bac_inclusion_threshold"]["setting_unusable_so_none_applied"], true);
        assert_eq!(programme(&bad, "Small")["currencies"][0]["state"], "included");
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS", r#"{"GBP":1000}"#) };

        // The actual-cost source is stated; labour with none recorded is no cost, not zero.
        let (code, lab) = live("?ac=labour").await;
        assert_eq!(code, 200);
        let lg = &programme(&lab, "Big")["currencies"][0];
        assert_eq!(lg["ac_source"], "labour_costs");
        assert!(lg["cpi"]["bp"].is_null(), "no labour recorded gives no CPI, not a flattering one: {lg}");
        assert_eq!(live("?ac=nonsense").await.0, 422);

        // ── a private programme is nowhere ──
        let out: Value = with!(request.get("/api/portfolio-earned-value"), outsider).await.json();
        assert!(programme(&out, "Secret").is_null() && !out.to_string().contains("9000000"));

        // ── the pack carries it ──
        let d: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
            .json(&json!({ "period": chrono::Utc::now().format("%Y-%m").to_string(), "audience": "board" })).await.json();
        let pk = d["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/director-packs/{pk}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pk}")), issuer).await.json();
        let ev = &pack["body"]["earned_value"];
        assert_eq!(programme(ev, "Big")["currencies"][0]["totals"]["ev_minor"], 2000);
        assert_eq!(programme(ev, "Small")["currencies"][0]["state"], "below EVM threshold");
        assert_eq!(programme(ev, "Bare")["state"], "no_baseline");
        assert_eq!(ev["bac_inclusion_threshold"]["minor_by_currency"]["GBP"], 1000, "the threshold is printed in the pack");
        assert!(ev["ac_source"].is_string());
        assert!(programme(ev, "Secret").is_null() && !pack.to_string().contains("9000000"));
        assert!(pack["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "earned_value"));
        assert!(!pack["body"]["data_quality"]["sections_not_yet_built"].to_string().contains("earned value"));
        let _ = bare;
    })
    .await;
}
