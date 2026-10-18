//! Multi-step approval processes (PL-2), end to end with real PASETO identities: a step
//! runs only after the one before it completes; a rejection ends the execution and
//! **logs each step it skips**; state lives in the database (a stalled execution is
//! resumable, idempotently); the log is append-only; and an edit mid-run does not
//! change a run already under way (it is pinned by version).
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test processes -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [81; 32];
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

use sea_orm::ConnectionTrait;

type Who = (axum::http::HeaderName, axum::http::HeaderValue);

const STARTER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";
const BOB: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";
const CAROL: &str = "3f2c6b7e-0000-4000-8000-0000000000c3";
const DAVE: &str = "3f2c6b7e-0000-4000-8000-0000000000d4";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

fn hdr(kid: &str, sub: &str, access: &[&str]) -> Who {
    auth_header(&mint(kid, sub, access, &[]))
}

fn events(ex: &Value) -> Vec<String> {
    ex["log"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| {
            format!(
                "{}:{}",
                l["step"].as_i64().map_or("-".into(), |s| s.to_string()),
                l["event"].as_str().unwrap()
            )
        })
        .collect()
}

fn states(ex: &Value) -> Vec<String> {
    ex["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["state"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test processes -- --ignored`"]
async fn a_process_runs_one_step_at_a_time_and_ends_honestly() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let (starter, bob, carol, dave, admin) = (
        hdr(&kid, STARTER, &["write"]),
        hdr(&kid, BOB, &["write"]),
        hdr(&kid, CAROL, &["write"]),
        hdr(&kid, DAVE, &["write"]),
        hdr(&kid, ADMIN, &["admin"]),
    );
    request::<App, _, _>(|request, ctx| async move {
        use axum::http::StatusCode as S;
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let person = |sub: &str| format!("person:{sub}");
        let steps = |extra: Value| {
            let mut v = json!([
                { "kind": "notify", "person_ref": person(DAVE), "title": "Heads up" },
                { "kind": "approve", "person_ref": person(BOB), "title": "Approve the plan" },
                { "kind": "todo", "person_ref": person(CAROL), "title": "File the paperwork" },
                { "kind": "notify", "person_ref": person(DAVE), "title": "All done" },
            ]);
            if !extra.is_null() { v = extra; }
            v
        };
        let plan: Value = with!(request.post("/api/plans"), starter).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();

        // ── definitions ──
        with!(request.post("/api/processes"), starter).json(&json!({ "name": "Sign-off", "steps": [] })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        with!(request.post("/api/processes"), starter).json(&json!({ "name": "Sign-off", "steps": [{ "kind": "approve", "person_ref": "bob", "title": "x" }] })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        with!(request.post("/api/processes"), starter).json(&json!({ "steps": steps(Value::Null) })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let def: Value = with!(request.post("/api/processes"), starter).json(&json!({ "name": "Sign-off", "steps": steps(Value::Null) })).await.json();
        let proc = def["pid"].as_str().unwrap().to_string();
        assert_eq!(def["current_version"], 1);
        let start = |who: &Who| {
            let (request, url, who) = (&request, format!("/api/processes/{proc}/start"), who.clone());
            let body = json!({ "subject_kind": "plan", "subject_pid": plan });
            async move { request.post(&url).add_header(who.0, who.1).json(&body).await }
        };
        let get_ex = |id: &str, who: &Who| {
            let (request, url, who) = (&request, format!("/api/process-executions/{id}"), who.clone());
            async move { request.get(&url).add_header(who.0, who.1).await.json::<Value>() }
        };

        // ── a run: the notice goes at once, then it waits on the approval ──
        let ex: Value = start(&starter).await.json();
        let e1 = ex["pid"].as_str().unwrap().to_string();
        assert_eq!((ex["status"].as_str(), ex["current_step"].as_i64(), ex["version"].as_i64()), (Some("running"), Some(1), Some(1)));
        assert_eq!(states(&ex), vec!["completed", "waiting", "pending", "pending"]);
        let approvals: Value = with!(request.get("/api/approvals?mine=approver"), bob).await.json();
        let ap = approvals.as_array().unwrap().iter().find(|a| a["process_execution_pid"] == e1.as_str()).cloned().expect("an approval request for Bob");
        let ap_pid = ap["pid"].as_str().unwrap().to_string();

        // A step runs only after the one before it: Carol cannot do her step yet, and not someone else's.
        let complete = |id: &str, i: usize, who: &Who| {
            let (request, url, who) = (&request, format!("/api/process-executions/{id}/steps/{i}/complete"), who.clone());
            async move { request.post(&url).add_header(who.0, who.1).json(&json!({})).await }
        };
        assert_eq!(complete(&e1, 2, &carol).await.status_code(), 409, "step 3 before step 2 completes");
        assert_eq!(complete(&e1, 1, &carol).await.status_code(), 403, "not hers");
        assert_eq!(complete(&e1, 1, &bob).await.status_code(), 422, "an approval is decided, not completed by hand");
        assert_eq!(complete(&e1, 9, &bob).await.status_code(), 404);

        // The process is edited while this run is under way: the run is pinned to version 1.
        assert_eq!(with!(request.put(&format!("/api/processes/{proc}")), dave).json(&json!({ "steps": steps(Value::Null) })).await.status_code(), 403, "only the definer or an admin");
        let v2: Value = with!(request.put(&format!("/api/processes/{proc}")), admin).json(&json!({ "steps": [{ "kind": "notify", "person_ref": person(DAVE), "title": "Only a notice now" }] })).await.json();
        assert_eq!(v2["current_version"], 2);
        let pinned = get_ex(&e1, &starter).await;
        assert_eq!(pinned["version"], 1);
        assert_eq!(pinned["steps"].as_array().unwrap().len(), 4, "the running execution keeps its four steps");
        let fresh: Value = start(&starter).await.json();
        assert_eq!((fresh["version"].as_i64(), fresh["status"].as_str()), (Some(2), Some("completed")), "a new run uses version 2: one notice, done");

        // ── the approval is decided: the next step starts ──
        assert_eq!(with!(request.post(&format!("/api/approvals/{ap_pid}/decide")), dave).json(&json!({ "decision": "approved" })).await.status_code(), 403);
        with!(request.post(&format!("/api/approvals/{ap_pid}/decide")), bob).json(&json!({ "decision": "approved", "comment": "fine" })).await.assert_status_ok();
        let mid = get_ex(&e1, &starter).await;
        assert_eq!((mid["status"].as_str(), mid["current_step"].as_i64()), (Some("running"), Some(2)));
        assert_eq!(states(&mid), vec!["completed", "completed", "waiting", "pending"]);
        assert_eq!(complete(&e1, 2, &dave).await.status_code(), 403, "not Dave's step");
        assert_eq!(complete(&e1, 3, &dave).await.status_code(), 409, "the notice after hers has not started");
        let done: Value = complete(&e1, 2, &carol).await.json();
        assert_eq!(done["status"], "completed");
        assert_eq!(states(&done), vec!["completed"; 4]);
        assert_eq!(events(&done), vec![
            "-:execution_started", "0:step_started", "0:step_completed", "1:step_started", "1:step_completed",
            "2:step_started", "2:step_completed", "3:step_started", "3:step_completed", "-:execution_completed",
        ]);
        assert_eq!(complete(&e1, 2, &carol).await.status_code(), 409, "a finished execution is over");

        // ── a rejection ends it, and every step it skips is logged ──
        with!(request.put(&format!("/api/processes/{proc}")), admin).json(&json!({ "steps": steps(Value::Null) })).await.assert_status_ok();
        let ex: Value = start(&starter).await.json();
        let e2 = ex["pid"].as_str().unwrap().to_string();
        let ap2 = with!(request.get("/api/approvals?mine=approver&status=pending"), bob).await.json::<Value>();
        let ap2 = ap2.as_array().unwrap().iter().find(|a| a["process_execution_pid"] == e2.as_str()).unwrap()["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.post(&format!("/api/approvals/{ap2}/decide")), bob).json(&json!({ "decision": "rejected" })).await.status_code(), 422, "a rejection says why");
        with!(request.post(&format!("/api/approvals/{ap2}/decide")), bob).json(&json!({ "decision": "rejected", "comment": "not yet" })).await.assert_status_ok();
        let rej = get_ex(&e2, &starter).await;
        assert_eq!(rej["status"], "rejected");
        assert_eq!(states(&rej), vec!["completed", "rejected", "skipped", "skipped"]);
        assert_eq!(events(&rej), vec![
            "-:execution_started", "0:step_started", "0:step_completed", "1:step_started", "1:step_rejected",
            "2:step_skipped", "3:step_skipped", "1:execution_rejected",
        ]);
        assert_eq!(complete(&e2, 2, &carol).await.status_code(), 409, "nothing runs after a rejection");

        // ── cancelling ──
        let ex: Value = start(&starter).await.json();
        let e3 = ex["pid"].as_str().unwrap().to_string();
        let cancel = |id: &str, who: &Who| {
            let (request, url, who) = (&request, format!("/api/process-executions/{id}/cancel"), who.clone());
            async move { request.post(&url).add_header(who.0, who.1).json(&json!({ "note": "changed our minds" })).await }
        };
        assert_eq!(cancel(&e3, &dave).await.status_code(), 403);
        let c = cancel(&e3, &starter).await;
        c.assert_status_ok();
        let c: Value = c.json();
        assert_eq!(c["status"], "cancelled");
        assert_eq!(states(&c), vec!["completed", "skipped", "skipped", "skipped"], "the step it was on did not complete");
        assert_eq!(cancel(&e3, &starter).await.status_code(), 409);
        let pending: Value = with!(request.get("/api/approvals?mine=approver&status=pending"), bob).await.json();
        assert!(pending.as_array().unwrap().iter().all(|a| a["process_execution_pid"] != e3.as_str()), "its approval was withdrawn");

        // ── withdrawing the approval cancels the run ──
        let ex: Value = start(&starter).await.json();
        let e4 = ex["pid"].as_str().unwrap().to_string();
        let mine: Value = with!(request.get("/api/approvals?mine=requester&status=pending"), starter).await.json();
        let ap4 = mine.as_array().unwrap().iter().find(|a| a["process_execution_pid"] == e4.as_str()).unwrap()["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/approvals/{ap4}/withdraw")), starter).await.assert_status_ok();
        assert_eq!(get_ex(&e4, &starter).await["status"], "cancelled");

        // ── resumable: state is in the database, and resuming is idempotent ──
        let ex: Value = start(&starter).await.json();
        let e5 = ex["pid"].as_str().unwrap().to_string();
        // A crash between the decision and the step after it: the approval is decided, nothing moved.
        ctx.db.execute_unprepared(&format!("UPDATE approval_requests SET status='approved', decided_by='{BOB}', decided_at=now() WHERE process_execution_pid='{e5}'")).await.unwrap();
        assert_eq!(get_ex(&e5, &starter).await["current_step"], 1, "still waiting");
        assert_eq!(with!(request.post(&format!("/api/process-executions/{e5}/resume")), dave).await.status_code(), 403);
        let resumed: Value = with!(request.post(&format!("/api/process-executions/{e5}/resume")), starter).await.json();
        assert_eq!(resumed["current_step"], 2);
        let log_len = resumed["log"].as_array().unwrap().len();
        let again: Value = with!(request.post(&format!("/api/process-executions/{e5}/resume")), starter).await.json();
        assert_eq!(again["log"].as_array().unwrap().len(), log_len, "resuming twice changes nothing");

        // ── the log and the versions are append-only, in the database itself ──
        for q in [
            "UPDATE process_execution_logs SET note = 'rewritten'",
            "DELETE FROM process_execution_logs",
            "UPDATE process_versions SET steps = '[]'::jsonb",
            "DELETE FROM process_versions",
        ] {
            assert!(ctx.db.execute_unprepared(q).await.is_err(), "{q} must be refused");
        }

        // ── who may start, and who may read ──
        let ghost = with!(request.post(&format!("/api/processes/{proc}/start")), starter).json(&json!({ "subject_kind": "plan", "subject_pid": "00000000-0000-4000-8000-00000000ffff" })).await;
        ghost.assert_status(S::NOT_FOUND);
        with!(request.post(&format!("/api/processes/{proc}/start")), starter).json(&json!({ "subject_kind": "nonsense", "subject_pid": plan })).await.assert_status(S::UNPROCESSABLE_ENTITY);
        let mine: Value = with!(request.get("/api/process-executions"), starter).await.json();
        assert!(mine.as_array().unwrap().len() >= 5);
        let strangers: Value = with!(request.get("/api/process-executions"), dave).await.json();
        assert!(strangers.as_array().unwrap().is_empty(), "someone else's runs are not in my list");
        let on_plan: Value = with!(request.get(&format!("/api/process-executions?subject_kind=plan&subject_pid={plan}")), dave).await.json();
        assert!(on_plan.as_array().unwrap().len() >= 5, "a reader of the subject sees the runs on it");
        // Retiring a process leaves its runs alone.
        with!(request.delete(&format!("/api/processes/{proc}")), dave).await.assert_status(S::FORBIDDEN);
        with!(request.delete(&format!("/api/processes/{proc}")), admin).await.assert_status_ok();
        assert_eq!(get_ex(&e5, &starter).await["status"], "running");
        with!(request.post(&format!("/api/processes/{proc}/start")), starter).json(&json!({ "subject_kind": "plan", "subject_pid": plan })).await.assert_status(S::NOT_FOUND);
    })
    .await;
}
