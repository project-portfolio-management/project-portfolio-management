//! Controlled documents (PL-6), end to end with real PASETO identities: a version moves
//! `draft → in_review → approved → effective → superseded`; **a version once effective is
//! immutable** (an edit is the next draft), **exactly one version is effective**, **a
//! version with no approved approval cannot become effective**, superseding **records the
//! successor**, and every transition is audited. The database itself refuses what the
//! rules refuse, so the guarantees do not depend on this code being the only writer.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test controlled_documents -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [91; 32];
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

use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};

type Who = (axum::http::HeaderName, axum::http::HeaderValue);

const OWNER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";
const APPROVER: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";
const STRANGER: &str = "3f2c6b7e-0000-4000-8000-0000000000d4";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

fn hdr(kid: &str, sub: &str, access: &[&str]) -> Who {
    auth_header(&mint(kid, sub, access, &[]))
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test controlled_documents -- --ignored`"]
async fn a_controlled_document_is_immutable_once_effective_and_never_without_an_approval() {
    use project_portfolio_management_service::models::_entities::audit_logs;
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let (owner, approver, stranger, admin) = (
        hdr(&kid, OWNER, &["write"]),
        hdr(&kid, APPROVER, &["write"]),
        hdr(&kid, STRANGER, &["write"]),
        hdr(&kid, ADMIN, &["admin"]),
    );
    request::<App, _, _>(|request, ctx| async move {
        use axum::http::StatusCode as S;
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let audits = |action: &'static str| {
            let db = ctx.db.clone();
            async move { audit_logs::Entity::find().filter(audit_logs::Column::Action.eq(action)).count(&db).await.unwrap() }
        };
        let plan: Value = with!(request.post("/api/plans"), owner).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();

        // ── a document and its first draft ──
        with!(request.post("/api/documents"), owner).json(&json!({ "title": " " })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let doc: Value = with!(request.post("/api/documents"), owner).json(&json!({ "title": "Safety policy", "plan_pid": plan, "body": "Wear boots." })).await.json();
        let d = doc["pid"].as_str().unwrap().to_string();
        assert_eq!((doc["versions"][0]["status"].as_str(), doc["versions"][0]["version"].as_i64()), (Some("draft"), Some(1)));
        assert!(doc["in_force_today"].is_null(), "nothing is in force yet");
        let v = |n: i32| format!("/api/documents/{d}/versions/{n}");
        let edit = |n: i32, body: &str| {
            let (request, url, who) = (&request, v(n), owner.clone());
            let body = json!({ "body": body });
            async move { request.put(&url).add_header(who.0, who.1).json(&body).await }
        };
        edit(1, "Wear boots and a helmet.").await.assert_status_ok();
        let submit = |n: i32, who: &Who| {
            let (request, url, who) = (&request, format!("{}/submit", v(n)), who.clone());
            let body = json!({ "approver_ref": format!("person:{APPROVER}") });
            async move { request.post(&url).add_header(who.0, who.1).json(&body).await }
        };
        let make_effective = |n: i32, who: &Who| {
            let (request, url, who) = (&request, format!("{}/make-effective", v(n)), who.clone());
            async move { request.post(&url).add_header(who.0, who.1).await }
        };
        assert_eq!(submit(1, &stranger).await.status_code(), 403, "not the owner");
        assert_eq!(make_effective(1, &owner).await.status_code(), 409, "a draft is not effective");
        submit(1, &owner).await.assert_status_ok();
        assert_eq!(edit(1, "sneaky").await.status_code(), 409, "in review, the text is final");
        assert_eq!(submit(1, &owner).await.status_code(), 409, "already submitted");
        assert_eq!(make_effective(1, &owner).await.status_code(), 409, "in review is not approved");

        // A status flag is not an approval: forced to `approved` in the database with the approval still pending.
        ctx.db.execute_unprepared(&format!("UPDATE document_versions SET status='approved' WHERE document_pid='{d}' AND version=1")).await.unwrap();
        let r = make_effective(1, &owner).await;
        r.assert_status(S::UNPROCESSABLE_ENTITY);
        assert!(r.text().contains("no approval"), "{}", r.text());
        ctx.db.execute_unprepared(&format!("UPDATE document_versions SET status='in_review' WHERE document_pid='{d}' AND version=1")).await.unwrap();
        // And the database refuses an approved version with no approval at all.
        assert!(ctx.db.execute_unprepared(&format!("UPDATE document_versions SET status='approved', approval_pid=NULL WHERE document_pid='{d}' AND version=1")).await.is_err());

        // ── the approver decides; the version follows ──
        let ap = with!(request.get("/api/approvals?mine=approver&status=pending"), approver).await.json::<Value>();
        let ap = ap.as_array().unwrap().iter().find(|a| a["subject_kind"] == "document_version").unwrap()["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.post(&format!("/api/approvals/{ap}/decide")), stranger).json(&json!({ "decision": "approved" })).await.status_code(), 403);
        with!(request.post(&format!("/api/approvals/{ap}/decide")), approver).json(&json!({ "decision": "approved", "comment": "ok" })).await.assert_status_ok();
        let got: Value = with!(request.get(&format!("/api/documents/{d}")), owner).await.json();
        assert_eq!(got["versions"][0]["status"], "approved");
        assert_eq!(make_effective(1, &stranger).await.status_code(), 403, "only the owner or an admin");
        let eff: Value = make_effective(1, &owner).await.json();
        assert_eq!(eff["status"], "effective");
        let today = eff["effective_from"].as_str().unwrap().to_string();
        let got: Value = with!(request.get(&format!("/api/documents/{d}")), owner).await.json();
        assert_eq!(got["in_force_today"], 1);

        // ── an effective version is immutable: in the API and in the database ──
        assert_eq!(edit(1, "rewritten history").await.status_code(), 409);
        assert!(ctx.db.execute_unprepared(&format!("UPDATE document_versions SET body='rewritten' WHERE document_pid='{d}' AND version=1")).await.is_err());
        assert!(ctx.db.execute_unprepared(&format!("DELETE FROM document_versions WHERE document_pid='{d}' AND version=1")).await.is_err());
        let one: Value = with!(request.get(&v(1)), stranger).await.json();
        assert_eq!(one["body"], "Wear boots and a helmet.", "the text is unchanged");

        // ── the next draft ──
        let next: Value = with!(request.post(&format!("/api/documents/{d}/versions")), owner).json(&json!({ "change_summary": "add gloves" })).await.json();
        assert_eq!((next["version"].as_i64(), next["status"].as_str(), next["body"].as_str()), (Some(2), Some("draft"), Some("Wear boots and a helmet.")), "a copy of the text to edit");
        assert_eq!(with!(request.post(&format!("/api/documents/{d}/versions")), owner).json(&json!({})).await.status_code(), 409, "one in flight at a time");
        edit(2, "Wear boots, a helmet and gloves.").await.assert_status_ok();
        // Rejected: back to draft, with the approval let go; edit and submit again.
        submit(2, &owner).await.assert_status_ok();
        let ap = with!(request.get("/api/approvals?mine=approver&status=pending"), approver).await.json::<Value>();
        let ap = ap.as_array().unwrap().iter().find(|a| a["subject_kind"] == "document_version").unwrap()["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.post(&format!("/api/approvals/{ap}/decide")), approver).json(&json!({ "decision": "rejected" })).await.status_code(), 422, "says why");
        with!(request.post(&format!("/api/approvals/{ap}/decide")), approver).json(&json!({ "decision": "rejected", "comment": "mention eye protection" })).await.assert_status_ok();
        let got: Value = with!(request.get(&format!("/api/documents/{d}")), owner).await.json();
        assert_eq!(got["versions"][1]["status"], "draft");
        assert!(got["versions"][1]["approval_pid"].is_null());
        edit(2, "Wear boots, a helmet, gloves and eye protection.").await.assert_status_ok();
        // Withdrawing the request also returns it to draft.
        submit(2, &owner).await.assert_status_ok();
        let mine = with!(request.get("/api/approvals?mine=requester&status=pending"), owner).await.json::<Value>();
        let ap_w = mine.as_array().unwrap().iter().find(|a| a["subject_kind"] == "document_version").unwrap()["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/approvals/{ap_w}/withdraw")), owner).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/documents/{d}")), owner).await.json::<Value>()["versions"][1]["status"], "draft");
        submit(2, &owner).await.assert_status_ok();
        let ap = with!(request.get("/api/approvals?mine=approver&status=pending"), approver).await.json::<Value>();
        let ap = ap.as_array().unwrap().iter().find(|a| a["subject_kind"] == "document_version").unwrap()["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/approvals/{ap}/decide")), approver).json(&json!({ "decision": "approved" })).await.assert_status_ok();

        // Exactly one effective: the database refuses a second even if forced.
        assert!(ctx.db.execute_unprepared(&format!("UPDATE document_versions SET status='effective', effective_from=CURRENT_DATE WHERE document_pid='{d}' AND version=2")).await.is_err(), "two effective versions");

        // ── the successor takes over; the predecessor records it ──
        let eff2: Value = make_effective(2, &admin).await.json();
        assert_eq!(eff2["status"], "effective");
        let got: Value = with!(request.get(&format!("/api/documents/{d}")), owner).await.json();
        assert_eq!(got["versions"][0]["status"], "superseded");
        assert_eq!(got["versions"][0]["superseded_by"], got["versions"][1]["pid"], "superseding records the successor");
        assert_eq!(got["versions"][0]["superseded_on"].as_str(), Some(today.as_str()));
        assert_eq!(got["in_force_today"], 2);
        let on = |date: &str| {
            let (request, url, who) = (&request, format!("/api/documents/{d}/in-force?on={date}"), stranger.clone());
            async move { request.get(&url).add_header(who.0, who.1).await.json::<Value>() }
        };
        assert_eq!(on("2099-01-01").await["in_force"]["version"], 2);
        assert!(on("2000-01-01").await["in_force"].is_null(), "nothing was in force before it began");
        assert_eq!(on("2099-01-01").await["in_force"]["body"], "Wear boots, a helmet, gloves and eye protection.");
        assert!(ctx.db.execute_unprepared(&format!("UPDATE document_versions SET status='effective', superseded_by=NULL, superseded_on=NULL WHERE document_pid='{d}' AND version=1")).await.is_err(), "a superseded version does not come back");

        // ── every transition is audited ──
        for (action, n) in [
            ("document_created", 1), ("document_version_submitted", 4), ("document_version_approved", 2),
            ("document_version_returned_to_draft", 2), ("document_version_made_effective", 2), ("document_version_superseded", 1),
        ] {
            assert_eq!(audits(action).await, n, "{action}");
        }
        assert!(audits("document_version_edited").await >= 3);

        // A checklist case of the plan, opened while it was still public.
        let case: Value = with!(request.post(&format!("/api/plans/{plan}/onboarding/cases")), owner)
            .json(&json!({ "kind": "plan_intake", "effective_date": "2026-10-06" }))
            .await
            .json();
        let case = case["pid"].as_str().unwrap().to_string();
        // ── a document of a private plan is not there for someone outside it ──
        with!(request.put(&format!("/api/plans/{plan}/access")), owner).json(&json!({ "private": true, "members": [] })).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/documents/{d}")), stranger).await.status_code(), 404);
        assert_eq!(with!(request.get(&format!("/api/documents/{d}/in-force")), stranger).await.status_code(), 404);
        assert_eq!(with!(request.get(&v(2)), stranger).await.status_code(), 404);
        let listed = with!(request.get("/api/documents"), stranger).await.text();
        assert!(!listed.contains(&d) && !listed.contains("Safety policy"), "{listed}");
        assert!(with!(request.get("/api/documents"), owner).await.text().contains(&d));
        // The guard reads the path the way the handlers do: a braced or percent-encoded
        // spelling of the plan id is no way round it.
        for path in [format!("/api/plans/{plan}/cpm"), format!("/api/plans/%7B{plan}%7D/cpm"), format!("/api/plans/%7b{plan}%7d")] {
            assert_eq!(with!(request.get(&path), stranger).await.status_code(), 404, "{path}");
        }
        assert_eq!(with!(request.get(&format!("/api/plans/%7B{plan}%7D")), owner).await.status_code(), 200, "members still get in");
        // A checklist of a private plan is not there either, nor can it be changed.
        assert_eq!(with!(request.get(&format!("/api/onboarding/cases/{case}")), stranger).await.status_code(), 404);
        assert_eq!(with!(request.post(&format!("/api/onboarding/cases/{case}/cancel")), stranger).json(&json!({ "reason": "x" })).await.status_code(), 404);
        assert_eq!(with!(request.get(&format!("/api/onboarding/cases/{case}")), owner).await.status_code(), 200);
        // Someone who cannot read the plan cannot create a document in it, nor ask for its approval.
        assert_eq!(with!(request.post("/api/documents"), stranger).json(&json!({ "title": "x", "plan_pid": plan })).await.status_code(), 404);
    })
    .await;
}
