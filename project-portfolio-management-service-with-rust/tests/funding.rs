//! Funding sources and financial-year money (DP-6), end to end with real PASETO identities:
//! the financial-year boundary at 31 March and 1 April, a line with no funding source counted and listed,
//! capital and revenue apart unless asked, currencies never summed, a private plan that is nowhere, and
//! the pack carrying the section.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test funding -- --ignored`.
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
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test funding -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn money_by_funding_source_keeps_its_promises() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
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
        let programme = mk("Program", "Digital Care", None).await;
        let project = mk("Project", "Records", Some(programme.clone())).await;
        let secret_project = mk("Project", "Secret works", Some(programme.clone())).await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret_project}', true, '[]', now(), now())")).await.unwrap();
        // The parent link must have taken: otherwise the lines below are not under the programme.
        let kid_row: Value = with!(request.get(&format!("/api/plans/{project}")), writer).await.json();
        assert!(kid_row.to_string().contains(&programme), "the project sits under the programme: {kid_row}");

        // ── the funding-source list ──
        assert_eq!(with!(request.post("/api/funding-sources"), writer).json(&json!({ "name": "x" })).await.status_code(), 422);
        let grant: Value = with!(request.post("/api/funding-sources"), writer).json(&json!({ "name": "Departmental grant" })).await.json();
        let grant_pid = grant["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.post("/api/funding-sources"), writer).json(&json!({ "name": "departmental GRANT" })).await.status_code(), 409, "names are unique, ignoring case");
        let loan: Value = with!(request.post("/api/funding-sources"), writer).json(&json!({ "name": "Treasury loan" })).await.json();
        let loan_pid = loan["pid"].as_str().unwrap().to_string();
        let retired: Value = with!(request.post("/api/funding-sources"), writer).json(&json!({ "name": "Retired fund" })).await.json();
        let retired_pid = retired["pid"].as_str().unwrap().to_string();
        with!(request.put(&format!("/api/funding-sources/{retired_pid}")), writer).json(&json!({ "active": false })).await.assert_status_ok();

        // ── budget lines: both sides of the 31 March / 1 April boundary ──
        let line = |plan: String, category: &'static str, currency: &'static str, planned: i64, start: Option<&'static str>, source: Option<String>| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let mut b = json!({ "category": category, "description": "A line", "currency": currency, "planned_minor": planned });
                if let Some(s) = start { b["period_start"] = json!(s); }
                if let Some(s) = source { b["funding_source_pid"] = json!(s); }
                let r = request.post(&format!("/api/plans/{plan}/budget-lines")).add_header(writer.0.clone(), writer.1.clone()).json(&b).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        assert_eq!(line(project.clone(), "capex", "GBP", 1, Some("2026-05-01"), Some(retired_pid.clone())).await.0, 422, "a retired source cannot be chosen");
        assert_eq!(line(project.clone(), "capex", "GBP", 1, Some("2026-05-01"), Some(uuid::Uuid::new_v4().to_string())).await.0, 422, "an unknown source");
        let (_, a) = line(project.clone(), "capex", "GBP", 100_000, Some("2026-04-01"), Some(grant_pid.clone())).await;
        let (_, _b) = line(project.clone(), "opex", "GBP", 50_000, Some("2026-06-01"), Some(grant_pid.clone())).await;
        let (_, _c) = line(project.clone(), "capex", "EUR", 7_000, Some("2026-06-01"), Some(loan_pid.clone())).await;
        let (_, d) = line(project.clone(), "opex", "GBP", 9_000, Some("2026-03-31"), Some(grant_pid.clone())).await;
        let (_, e) = line(project.clone(), "opex", "GBP", 4_000, Some("2026-07-01"), None).await;
        let (_, _f) = line(project.clone(), "opex", "GBP", 3_000, None, Some(grant_pid.clone())).await;
        let (_, _g) = line(secret_project.clone(), "opex", "GBP", 777_000, Some("2026-07-01"), Some(grant_pid.clone())).await;
        // Spend against the 1 April line.
        let a_pid = a["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/plans/{project}/budget-lines/{a_pid}/actual")), writer).json(&json!({ "amount_minor": 25_000 })).await.assert_status_ok();

        let rows_of = |v: &Value, prog: &str| v["programmes"].as_array().unwrap().iter().find(|p| p["name"] == prog).unwrap()["by_funding_source"].as_array().unwrap().clone();
        let this: Value = with!(request.get("/api/portfolio-money?fy=2026"), writer).await.json();
        assert_eq!(this["financial_year"]["label"], "2026/27");
        assert_eq!(this["financial_year"]["start"], "2026-04-01");
        assert_eq!(this["financial_year"]["end"], "2027-03-31");
        let rows = rows_of(&this, "Digital Care");
        let find = |rows: &[Value], src: &str, class: &str, cur: &str| rows.iter().find(|r| r["funding_source"] == src && r["class"] == class && r["currency"] == cur).cloned();
        let capex = find(&rows, "Departmental grant", "capex", "GBP").expect("1 April is in the year");
        assert_eq!((capex["budget_minor"].as_i64(), capex["spend_minor"].as_i64(), capex["variance_minor"].as_i64()), (Some(100_000), Some(25_000), Some(75_000)));
        assert_eq!(find(&rows, "Departmental grant", "opex", "GBP").unwrap()["budget_minor"], 50_000, "31 March is not in this year");
        assert_eq!(find(&rows, "Treasury loan", "capex", "EUR").unwrap()["budget_minor"], 7_000, "euros stay apart from pounds");
        assert!(find(&rows, "no funding source", "opex", "GBP").is_some(), "a line with no source is counted");
        assert!(rows.iter().all(|r| r["class"] != "all"), "capital and revenue are not summed unless asked");
        // The previous year holds the 31 March line and nothing of this year.
        let last: Value = with!(request.get("/api/portfolio-money?fy=2025"), writer).await.json();
        assert_eq!(last["financial_year"]["label"], "2025/26");
        let last_rows = rows_of(&last, "Digital Care");
        assert_eq!(last_rows.len(), 1);
        assert_eq!(last_rows[0]["budget_minor"], 9_000, "31 March belongs to the earlier year");
        // The data-quality panel lists the unsourced line and counts the undated one.
        let dq = &this["data_quality"];
        assert_eq!(dq["lines_with_no_funding_source"], 1);
        assert_eq!(dq["lines_with_no_dates"], 1);
        assert_eq!(dq["lines_in_other_financial_years"], 1);
        assert!(dq["listed_lines_with_no_funding_source"].as_array().unwrap().iter().any(|l| l["line"] == e["pid"]));
        let _ = d;
        // Combining classes is asked for, and says so.
        let both: Value = with!(request.get("/api/portfolio-money?fy=2026&combine_classes=true"), writer).await.json();
        assert_eq!(both["classes_combined"], true);
        let comb = find(&rows_of(&both, "Digital Care"), "Departmental grant", "all", "GBP").unwrap();
        assert_eq!(comb["budget_minor"], 150_000, "capital and revenue summed on request, pounds only");
        assert_eq!(with!(request.get("/api/portfolio-money?fy=1999"), writer).await.status_code(), 422);

        // ── a private project's money is nowhere, for anyone ──
        assert!(!this.to_string().contains("777000") && !this.to_string().contains("Secret works"));
        assert_eq!(with!(request.put(&format!("/api/plans/{secret_project}/budget-lines/{a_pid}/funding-source")), outsider).json(&json!({ "funding_source_pid": null })).await.status_code(), 404);

        // ── set and clear a line's source ──
        with!(request.put(&format!("/api/plans/{project}/budget-lines/{}/funding-source", e["pid"].as_str().unwrap())), writer).json(&json!({ "funding_source_pid": loan_pid })).await.assert_status_ok();
        let after: Value = with!(request.get("/api/portfolio-money?fy=2026"), writer).await.json();
        assert_eq!(after["data_quality"]["lines_with_no_funding_source"], 0);

        // ── the pack carries it for the financial year of its cut-off ──
        let dr: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
            .json(&json!({ "period": chrono::Utc::now().format("%Y-%m").to_string(), "audience": "board" })).await.json();
        let pk = dr["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/director-packs/{pk}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pk}")), issuer).await.json();
        assert!(pack["body"]["funding"]["financial_year"]["label"].is_string());
        assert!(pack["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "funding"));
        assert!(!pack["body"]["data_quality"]["sections_not_yet_built"].to_string().contains("funding"));
        assert!(!pack.to_string().contains("777000"), "a private project's money is not in the pack");
        // The forecast sits beside it per currency, never merged.
        assert!(pack["body"]["funding"]["programmes"][0]["whole_life_forecast"]["rows"].is_array());
    })
    .await;
}
