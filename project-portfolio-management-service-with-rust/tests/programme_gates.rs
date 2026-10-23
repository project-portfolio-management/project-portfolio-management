//! Gate decisions, planned gate dates and programme status (DP-5), end to end with real PASETO identities:
//! who may pause or stop a programme, a status that changes only by a recorded decision, stopped being
//! terminal in the service and in the database, an append-only history, a pause that flags allocations and
//! deletes none, a private programme that is nowhere, and the pack listing passed, failed, deferred,
//! paused and stopped.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test programme_gates -- --ignored`.
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
const DECIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a2";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a9";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test programme_gates -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn gates_and_programme_status_keep_their_promises() {
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
    let decider = bearer(
        &kid,
        DECIDER,
        &[("access", "write"), ("programme", "decider")],
    );
    let issuer = bearer(
        &kid,
        ISSUER_SUB,
        &[("access", "write"), ("directorpack", "issuer")],
    );
    let outsider = bearer(&kid, OUTSIDER, &[("access", "write")]);
    let period = chrono::Utc::now().format("%Y-%m").to_string();
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let mk = |kind: &'static str, name: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone())
                    .json(&json!({ "kind": kind, "name": name })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let review = |pid: String, gate: &'static str, decision: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r: Value = request.post(&format!("/api/plans/{pid}/gate-reviews")).add_header(writer.0.clone(), writer.1.clone())
                    .json(&json!({ "gate": gate, "decision": decision })).await.json();
                r
            }
        };
        let alpha = mk("Program", "Alpha").await;
        let beta = mk("Program", "Beta").await;
        let gamma = mk("Program", "Gamma").await;
        let delta = mk("Program", "Delta").await;
        let epsilon = mk("Program", "Epsilon").await;
        let secret = mk("Program", "Secret").await;
        let project = mk("Project", "Just a project").await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();

        // ── gate reviews: passed, failed, held ──
        review(alpha.clone(), "g0_concept", "approved_with_conditions").await;
        review(alpha.clone(), "g1_feasibility", "hold").await;
        review(beta.clone(), "g0_concept", "rejected").await;
        let eps_review: Value = review(epsilon.clone(), "g0_concept", "approved").await;
        let eps_review_pid = {
            let l: Value = with!(request.get(&format!("/api/plans/{epsilon}/gate-reviews")), writer).await.json();
            l["reviews"][0]["pid"].as_str().or(eps_review["pid"].as_str()).unwrap_or_default().to_string()
        };
        let alpha_review_pid = {
            let l: Value = with!(request.get(&format!("/api/plans/{alpha}/gate-reviews")), writer).await.json();
            l["reviews"][0]["pid"].as_str().unwrap_or_default().to_string()
        };

        assert!(!eps_review_pid.is_empty() && !alpha_review_pid.is_empty(), "gate review pids are readable");

        // ── planned gate dates ──
        let planned = |p: &str, g: &str| format!("/api/plans/{p}/planned-gates/{g}");
        assert_eq!(with!(request.put(&planned(&gamma, "g9_nope")), writer).json(&json!({ "planned_on": "2020-01-01" })).await.status_code(), 422);
        with!(request.put(&planned(&gamma, "g0_concept")), writer).json(&json!({ "planned_on": "2020-01-01" })).await.assert_status_ok();
        with!(request.put(&planned(&gamma, "g0_concept")), writer).json(&json!({ "planned_on": "2020-02-01" })).await.assert_status_ok();
        let listed: Value = with!(request.get(&format!("/api/plans/{gamma}/planned-gates")), writer).await.json();
        assert_eq!(listed["planned_gates"].as_array().unwrap().len(), 1, "one date per gate");
        assert_eq!(listed["planned_gates"][0]["planned_on"], "2020-02-01");
        // A date in the future is not late.
        with!(request.put(&planned(&beta, "g1_feasibility")), writer).json(&json!({ "planned_on": "2999-01-01" })).await.assert_status_ok();

        // ── programme status: only by a recorded decision, only by a decider ──
        let status = |p: &str| format!("/api/plans/{p}/programme-status");
        let pause = json!({ "status": "paused", "reason": "Funding is under review by the board" });
        assert_eq!(with!(request.post(&status(&delta)), writer).json(&pause).await.status_code(), 403, "a plain writer cannot pause");
        assert_eq!(with!(request.post(&status(&delta)), decider).json(&json!({ "status": "paused", "reason": "short" })).await.status_code(), 422);
        assert_eq!(with!(request.post(&status(&project)), decider).json(&pause).await.status_code(), 422, "only a programme has a status");
        assert_eq!(with!(request.post(&status(&delta)), decider).json(&json!({ "status": "frozen", "reason": "Funding is under review" })).await.status_code(), 409);
        let now0: Value = with!(request.get(&status(&delta)), writer).await.json();
        assert_eq!(now0["status"], "active");
        with!(request.post(&status(&delta)), decider).json(&pause).await.assert_status_ok();
        assert_eq!(with!(request.post(&status(&delta)), decider).json(&pause).await.status_code(), 409, "a repeat is refused");
        with!(request.post(&status(&delta)), decider).json(&json!({ "status": "active", "reason": "Funding confirmed again" })).await.assert_status_ok();
        with!(request.post(&status(&delta)), decider).json(&pause).await.assert_status_ok();
        // An allocation on the paused programme is flagged, never deleted.
        ctx.db.execute_unprepared(&format!("INSERT INTO allocations (pid, plan_pid, person_ref, percent, skills_required, created_at, updated_at) VALUES (gen_random_uuid(), '{delta}', 'person:x', 50, '[]', now(), now())")).await.unwrap();
        let paused: Value = with!(request.get(&status(&delta)), writer).await.json();
        assert_eq!(paused["status"], "paused");
        assert_eq!(paused["history"].as_array().unwrap().len(), 3);
        assert_eq!(paused["still_held"]["allocations_flagged"], 1);
        // The decision names who decided it.
        assert_eq!(paused["history"][2]["decided_by"], DECIDER);

        // Stop Epsilon: a link must be this programme's own; stopped is terminal and keeps its record.
        let foreign = json!({ "status": "stopped", "reason": "The business case no longer holds", "gate_review_pid": alpha_review_pid });
        assert_eq!(with!(request.post(&status(&epsilon)), decider).json(&foreign).await.status_code(), 422, "another programme's review");
        let stop = json!({ "status": "stopped", "reason": "The business case no longer holds", "gate_review_pid": eps_review_pid });
        with!(request.post(&status(&epsilon)), decider).json(&stop).await.assert_status_ok();
        for next in ["active", "paused", "stopped"] {
            assert_eq!(with!(request.post(&status(&epsilon)), decider).json(&json!({ "status": next, "reason": "Trying to bring it back" })).await.status_code(), 409, "{next} after stopped");
        }
        let stopped: Value = with!(request.get(&status(&epsilon)), writer).await.json();
        assert_eq!(stopped["status"], "stopped");
        // The database does not rely on the service being right.
        for (what, sql) in [
            ("after stopped", format!("INSERT INTO programme_status_decisions (pid, plan_pid, status, reason, decided_by, decided_on, created_at, updated_at) VALUES (gen_random_uuid(), '{epsilon}', 'active', 'Trying to bring it back', 'x', current_date, now(), now())")),
            ("no decider", format!("INSERT INTO programme_status_decisions (pid, plan_pid, status, reason, decided_on, created_at, updated_at) VALUES (gen_random_uuid(), '{gamma}', 'paused', 'A reason long enough', current_date, now(), now())")),
            ("a repeat", format!("INSERT INTO programme_status_decisions (pid, plan_pid, status, reason, decided_by, decided_on, created_at, updated_at) VALUES (gen_random_uuid(), '{gamma}', 'active', 'A reason long enough', 'x', current_date, now(), now())")),
            ("a thin reason", format!("INSERT INTO programme_status_decisions (pid, plan_pid, status, reason, decided_by, decided_on, created_at, updated_at) VALUES (gen_random_uuid(), '{gamma}', 'paused', 'short', 'x', current_date, now(), now())")),
            ("an edit", "UPDATE programme_status_decisions SET reason = 'Rewritten to look better'".to_string()),
            ("a delete", "DELETE FROM programme_status_decisions".to_string()),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }

        // ── a private programme is nowhere ──
        assert_eq!(with!(request.get(&status(&secret)), outsider).await.status_code(), 404);
        assert_eq!(with!(request.post(&status(&secret)), decider).json(&pause).await.status_code(), 404);
        assert_eq!(with!(request.put(&planned(&secret, "g0_concept")), outsider).json(&json!({ "planned_on": "2020-01-01" })).await.status_code(), 404);

        // ── the pack lists passed, failed, deferred, paused and stopped ──
        let d: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
            .json(&json!({ "period": period, "audience": "board" })).await.json();
        let pid = d["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/director-packs/{pid}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pid}")), issuer).await.json();
        let g = &pack["body"]["gate_decisions"];
        let names = |k: &str| g[k].as_array().unwrap().iter().map(|l| format!("{}:{}", l["plan"].as_str().unwrap(), l["gate"].as_str().unwrap_or(""))).collect::<Vec<_>>();
        assert!(names("passed").contains(&"Alpha:g0_concept".to_string()), "approved with conditions is passed: {g}");
        assert!(names("passed").contains(&"Epsilon:g0_concept".to_string()));
        assert_eq!(names("failed"), ["Beta:g0_concept"]);
        let deferred = names("deferred");
        assert!(deferred.contains(&"Alpha:g1_feasibility".to_string()), "a hold is deferred: {deferred:?}");
        assert!(deferred.contains(&"Gamma:g0_concept".to_string()), "a planned date that passed without a review is deferred");
        assert!(!deferred.iter().any(|d| d.starts_with("Beta:")), "a future planned date is not late");
        let gamma_line = g["deferred"].as_array().unwrap().iter().find(|l| l["plan"] == "Gamma").unwrap();
        assert_eq!(gamma_line["planned_on"], "2020-02-01");
        assert!(gamma_line["days_late"].as_i64().unwrap() > 2000);
        assert_eq!(g["paused"].as_array().unwrap().len(), 1);
        assert_eq!(g["paused"][0]["plan"], "Delta");
        assert_eq!(g["paused"][0]["still_held"]["allocations_flagged"], 1);
        assert_eq!(g["stopped"][0]["plan"], "Epsilon");
        assert!(g["status_changes_in_period"].as_array().unwrap().len() >= 4);
        let text = pack.to_string();
        assert!(!text.contains("Secret") && !text.contains("person:") && !text.contains(DECIDER), "no private programme, no person");
        assert!(pack["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "gate_decisions"));
        let not_built = pack["body"]["data_quality"]["sections_not_yet_built"].to_string();
        assert!(!not_built.contains("gate decisions"));
        // The map shows the status.
        let map: Value = with!(request.get("/api/portfolio-map"), writer).await.json();
        let status_of = |name: &str| map["groups"].as_array().unwrap().iter().flat_map(|g| g["programmes"].as_array().unwrap().clone())
            .find(|p| p["name"] == name).map(|p| p["programme_status"].clone());
        assert_eq!(status_of("Delta"), Some(json!("paused")));
        assert_eq!(status_of("Epsilon"), Some(json!("stopped")));
        assert_eq!(status_of("Alpha"), Some(json!("active")));
        // A stopped programme is still in the estate, still reportable, never dropped.
        assert!(status_of("Epsilon").is_some());

        // The audit trail names who did what.
        let rows = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT action, actor FROM audit_logs WHERE action = 'gate_planned' OR action LIKE 'programme_%' ORDER BY id".to_string())).await.unwrap();
        let trail: Vec<(String, String)> = rows.iter().map(|r| (r.try_get("", "action").unwrap(), r.try_get("", "actor").unwrap())).collect();
        for a in ["gate_planned", "programme_paused", "programme_resumed", "programme_stopped"] {
            assert!(trail.iter().any(|(t, _)| t == a), "{a} not audited: {trail:?}");
        }
        assert!(trail.iter().filter(|(t, _)| t.starts_with("programme_")).all(|(_, who)| who == DECIDER));
    })
    .await;
}
