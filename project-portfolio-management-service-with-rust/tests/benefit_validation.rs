//! Validated benefits, per year and non-financial (DP-9), end to end with real PASETO identities:
//! realized but unvalidated shown as claimed, a validator who is a member of the programme refused,
//! one-off and recurring never summed, a non-financial benefit with no KPI "not measured", an
//! append-only validation history, a private programme that is nowhere, and the pack carrying the section.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test benefit_validation -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
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
const MEMBER: &str = "3f2c6b7e-0000-4000-8000-0000000000b1";
const ASSIGNEE: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";
const AUDITOR: &str = "3f2c6b7e-0000-4000-8000-0000000000c1";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a9";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test benefit_validation -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn benefits_are_claimed_until_someone_outside_validates_them() {
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
    let member = bearer(&kid, MEMBER, &[("access", "write")]);
    let assignee = bearer(&kid, ASSIGNEE, &[("access", "write")]);
    let auditor = bearer(&kid, AUDITOR, &[("access", "write")]);
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
        let prog = mk("Program", "Care", None).await;
        let project = mk("Project", "Records", Some(prog.clone())).await;
        let secret = mk("Program", "Secret", None).await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();
        // Two people are on the programme: one by an allocation, one as a task assignee.
        ctx.db.execute_unprepared(&format!("INSERT INTO allocations (pid, plan_pid, person_ref, percent, skills_required, created_at, updated_at) VALUES (gen_random_uuid(), '{prog}', 'person:{MEMBER}', 50, '[]', now(), now())")).await.unwrap();
        with!(request.post(&format!("/api/plans/{project}/tasks")), writer).json(&json!({ "title": "Do it", "assignee_ref": format!("worker:{ASSIGNEE}") })).await.assert_status_ok();

        let benefit = |plan: String, title: &'static str, cur: Option<&'static str>, target: Option<i64>| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let mut b = json!({ "title": title, "category": "cost_saving" });
                if let Some(c) = cur { b["currency"] = json!(c); }
                if let Some(t) = target { b["target_minor"] = json!(t); } else { b["target_note"] = json!("fewer complaints"); }
                let v: Value = request.post(&format!("/api/plans/{plan}/benefits")).add_header(writer.0.clone(), writer.1.clone()).json(&b).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let classify = |plan: String, b: String, body: Value| {
            let (request, writer) = (&request, writer.clone());
            async move { request.put(&format!("/api/plans/{plan}/benefits/{b}/classification")).add_header(writer.0.clone(), writer.1.clone()).json(&body).await }
        };
        let saving = benefit(project.clone(), "Fewer agency staff", Some("GBP"), Some(100_000)).await; // recurring
        let one_off = benefit(project.clone(), "Sale of old building", Some("GBP"), Some(500_000)).await; // one-off
        let soft = benefit(project.clone(), "Happier patients", None, None).await; // non-financial
        let soft2 = benefit(project.clone(), "Shorter waits", None, None).await;
        let hidden = benefit(project.clone(), "Unclassified", Some("EUR"), Some(7_000)).await;
        // Refusals: a yearly amount belongs to a recurring benefit; a bad word; an unknown KPI.
        assert_eq!(classify(project.clone(), saving.clone(), json!({ "annualized_minor": 100 })).await.status_code(), 422);
        assert_eq!(classify(project.clone(), saving.clone(), json!({ "recurrence": "sometimes" })).await.status_code(), 422);
        assert_eq!(classify(project.clone(), soft.clone(), json!({ "kpi_pid": uuid::Uuid::new_v4() })).await.status_code(), 422);
        classify(project.clone(), saving.clone(), json!({ "recurrence": "recurring", "annualized_minor": 100_000, "classification": "cash_releasing" })).await.assert_status_ok();
        classify(project.clone(), one_off.clone(), json!({ "recurrence": "one_off", "classification": "cash_releasing" })).await.assert_status_ok();
        // A KPI for one of the non-financial benefits; the other has none.
        let k: Value = with!(request.post("/api/kpis"), writer).json(&json!({ "name": "Patient score", "direction": "higher_better", "target_value": 80, "red_value": 50 })).await.json();
        let kpi = k["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/kpis/{kpi}/readings")), writer).json(&json!({ "value": 85, "observed_on": chrono::Utc::now().date_naive().to_string() })).await.assert_status_ok();
        classify(project.clone(), soft.clone(), json!({ "kpi_pid": kpi, "classification": "societal" })).await.assert_status_ok();

        // ── realized is a claim ──
        for (b, amount) in [(&saving, 60_000), (&one_off, 500_000)] {
            with!(request.post(&format!("/api/plans/{project}/benefits/{b}/realize")), writer).json(&json!({ "amount_minor": amount })).await.assert_status_ok();
        }
        let hist = |b: &str| {
            let (request, writer, project, b) = (&request, writer.clone(), project.clone(), b.to_string());
            async move { request.get(&format!("/api/plans/{project}/benefits/{b}/validations")).add_header(writer.0.clone(), writer.1.clone()).await.json::<Value>() }
        };
        let h = hist(&saving).await;
        assert_eq!((h["standing"].as_str(), h["claimed_minor"].as_i64(), h["validated_minor"].is_null()), (Some("claimed"), Some(60_000), true), "{h}");
        let live: Value = with!(request.get("/api/portfolio-benefits"), writer).await.json();
        let care = live["programmes"].as_array().unwrap().iter().find(|p| p["name"] == "Care").unwrap().clone();
        let row = |rec: &str| care["financial"].as_array().unwrap().iter().find(|r| r["recurrence"] == rec && r["currency"] == "GBP").cloned().unwrap();
        assert_eq!((row("recurring")["planned_minor"].as_i64(), row("recurring")["claimed_minor"].as_i64(), row("recurring")["validated_minor"].as_i64(), row("recurring")["claimed_only"].as_i64()), (Some(100_000), Some(60_000), Some(0), Some(1)));
        assert_eq!((row("one_off")["planned_minor"].as_i64(), row("one_off")["unvalidated_claim_minor"].as_i64()), (Some(500_000), Some(500_000)));
        assert_eq!(care["financial"].as_array().unwrap().iter().filter(|r| r["currency"] == "GBP").count(), 2, "recurring and one-off are never one per-year figure");
        assert!(care["financial"].as_array().unwrap().iter().any(|r| r["currency"] == "EUR" && r["recurrence"] == "not_stated"), "an unclassified benefit is said to be unstated, not dropped: {care}");
        // Non-financial: a KPI gives a status; none is not measured.
        let nf = care["non_financial"].as_array().unwrap();
        assert_eq!(nf.iter().find(|b| b["title"] == "Happier patients").unwrap()["kpi"]["state"], "green");
        assert_eq!(nf.iter().find(|b| b["title"] == "Shorter waits").unwrap()["kpi"]["state"], "not measured");
        let _ = (soft2, hidden);

        // ── a member of the programme cannot validate ──
        let url = |b: &str| format!("/api/plans/{project}/benefits/{b}/validations");
        for (who, name) in [(&member, "an allocated person"), (&assignee, "a task assignee")] {
            assert_eq!(with!(request.post(&url(&saving)), who).json(&json!({ "amount_minor": 60_000 })).await.status_code(), 403, "{name}");
        }
        assert_eq!(hist(&saving).await["history"].as_array().unwrap().len(), 0, "nothing was recorded by the refused");
        // Bad requests from someone who may validate.
        assert_eq!(with!(request.post(&url(&saving)), auditor).json(&json!({ "amount_minor": -1 })).await.status_code(), 422);
        assert_eq!(with!(request.post(&url(&soft)), auditor).json(&json!({ "amount_minor": 10 })).await.status_code(), 422, "a non-financial benefit has a KPI, not an amount");
        assert_eq!(with!(request.post(&url(&saving)), auditor).json(&json!({ "amount_minor": 10, "evidence_attachment_pid": uuid::Uuid::new_v4() })).await.status_code(), 422, "evidence must exist");
        // Real evidence: an attachment of the plan.
        let art = uuid::Uuid::new_v4();
        let att = uuid::Uuid::new_v4();
        ctx.db.execute_unprepared(&format!("INSERT INTO attachments (pid, subject_kind, subject_pid, artifact_pid, created_at, updated_at) VALUES ('{att}', 'plan', '{project}', '{art}', now(), now())")).await.unwrap();
        // An outsider validates part of it; then more. The latest counts, the earlier stays.
        with!(request.post(&url(&saving)), auditor).json(&json!({ "amount_minor": 20_000, "evidence_attachment_pid": att, "note": "Checked the payroll for March" })).await.assert_status_ok();
        let partly = hist(&saving).await;
        assert_eq!((partly["standing"].as_str(), partly["validated_minor"].as_i64()), (Some("partly_validated"), Some(20_000)));
        with!(request.post(&url(&saving)), auditor).json(&json!({ "amount_minor": 60_000 })).await.assert_status_ok();
        let full = hist(&saving).await;
        assert_eq!((full["standing"].as_str(), full["history"].as_array().unwrap().len(), full["history"][0]["validator"].as_str()), (Some("validated"), 2, Some(AUDITOR)));
        let live2: Value = with!(request.get("/api/portfolio-benefits"), writer).await.json();
        let care2 = live2["programmes"].as_array().unwrap().iter().find(|p| p["name"] == "Care").unwrap().clone();
        let rec2 = care2["financial"].as_array().unwrap().iter().find(|r| r["recurrence"] == "recurring").cloned().unwrap();
        assert_eq!((rec2["validated_minor"].as_i64(), rec2["unvalidated_claim_minor"].as_i64(), rec2["claimed_only"].as_i64()), (Some(60_000), Some(0), Some(0)));
        // The database keeps it append-only.
        for (what, sql) in [
            ("an edit", "UPDATE benefit_validations SET amount_minor = 1".to_string()),
            ("a delete", "DELETE FROM benefit_validations".to_string()),
            ("no validator", format!("INSERT INTO benefit_validations (pid, benefit_pid, plan_pid, amount_minor, currency, validated_on, created_at, updated_at) VALUES (gen_random_uuid(), '{saving}', '{project}', 1, 'GBP', current_date, now(), now())")),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }

        // ── a private programme is nowhere ──
        assert_eq!(with!(request.get(&format!("/api/plans/{secret}/benefits/{saving}/validations")), outsider).await.status_code(), 404);
        let out: Value = with!(request.get("/api/portfolio-benefits"), outsider).await.json();
        assert!(!out.to_string().contains("Secret"));

        // ── the pack ──
        let d: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
            .json(&json!({ "period": chrono::Utc::now().format("%Y-%m").to_string(), "audience": "board" })).await.json();
        let pk = d["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/director-packs/{pk}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pk}")), issuer).await.json();
        let b = &pack["body"]["benefits"];
        assert!(b["programmes"].as_array().unwrap().iter().any(|p| p["name"] == "Care"));
        assert!(b["totals"].as_array().unwrap().iter().any(|r| r["recurrence"] == "recurring" && r["validated_minor"] == 60_000));
        assert!(b["totals"].as_array().unwrap().iter().all(|r| r["recurrence"] != "all"), "no combined per-year row");
        assert!(!pack.to_string().contains(AUDITOR) && !pack.to_string().contains("person:"), "no person in the pack");
        assert!(pack["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "benefits"));
        assert!(!pack["body"]["data_quality"]["sections_not_yet_built"].to_string().contains("validated benefits"));
    })
    .await;
}
