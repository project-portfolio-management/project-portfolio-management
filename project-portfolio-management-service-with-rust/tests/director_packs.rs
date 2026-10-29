//! The monthly director pack (DP-1), end to end with real PASETO identities: who may draft and issue, who
//! may read which audience's pack, a confined partner reader who sees nothing live, a body that is
//! byte-identical forever, reissue, and a database that refuses to change an issued pack.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test director_packs -- --ignored`.

#![expect(
    clippy::similar_names,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::{ConnectionTrait, TransactionTrait};
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

const EDITOR: &str = "3f2c6b7e-0000-4000-8000-0000000000e1";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const PARTNER: &str = "3f2c6b7e-0000-4000-8000-0000000000f1";
const BOARD: &str = "3f2c6b7e-0000-4000-8000-0000000000f2";
const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test director_packs -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn a_pack_is_drafted_by_some_issued_by_fewer_read_by_audience_and_never_changes() {
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
    let editor = bearer(
        &kid,
        EDITOR,
        &[("access", "write"), ("directorpack", "editor")],
    );
    let issuer = bearer(
        &kid,
        ISSUER_SUB,
        &[("access", "write"), ("directorpack", "issuer")],
    );
    // A partner reader: confined to the packs, and holding the partner audience only.
    let partner = bearer(
        &kid,
        PARTNER,
        &[("role", "pack_reader"), ("pack_audience", "partner")],
    );
    // A board reader who is not confined (an ordinary account that also reads the board pack).
    let board = bearer(
        &kid,
        BOARD,
        &[("access", "read"), ("pack_audience", "board")],
    );
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }

        // Data for the pack: a public plan with an issue and a change request, and a private plan with
        // a critical issue that must not appear.
        let pubp: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Project", "name": "Public" })).await.json();
        let pubp = pubp["pid"].as_str().unwrap().to_string();
        let privp: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Project", "name": "Secret" })).await.json();
        let privp = privp["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/plans/{pubp}/issues")), writer)
            .json(&json!({ "title": "Build down", "severity": "high", "status": "open" })).await.assert_status_ok();
        with!(request.post(&format!("/api/plans/{privp}/issues")), writer)
            .json(&json!({ "title": "SECRET-ISSUE", "severity": "critical", "status": "open" })).await.assert_status_ok();
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{privp}', true, '[]', now(), now())")).await.unwrap();

        // Who may draft: an ordinary writer cannot, whatever the policy says.
        let want = json!({ "period": "2026-10", "audience": "partner" });
        assert_eq!(with!(request.post("/api/director-packs"), writer).json(&want).await.status_code(), 403);
        let confined = with!(request.post("/api/director-packs"), partner).json(&want).await.status_code();
        assert!(confined == 403 || confined == 404, "a confined reader cannot draft: {confined}");
        // Refusals that name the fault.
        for bad in [json!({ "period": "2026-13", "audience": "partner" }), json!({ "period": "2026-10", "audience": "everyone" }),
                    json!({ "period": "2026-10", "audience": "partner", "cut_off": "2999-01-01T00:00:00Z" })] {
            assert_eq!(with!(request.post("/api/director-packs"), editor).json(&bad).await.status_code(), 422, "{bad}");
        }
        let d: Value = with!(request.post("/api/director-packs"), editor).json(&want).await.json();
        let pid = d["pid"].as_str().unwrap().to_string();
        assert_eq!((d["status"].as_str(), d["version"].as_i64()), (Some("draft"), Some(1)));
        assert_eq!(with!(request.post("/api/director-packs"), editor).json(&want).await.status_code(), 409, "one pack per period and audience");

        // A draft is invisible to its audience; an editor cannot issue; only an issuer can.
        assert_eq!(with!(request.get(&format!("/api/director-packs/{pid}")), partner).await.status_code(), 404);
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), editor).await.status_code(), 403);
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), writer).await.status_code(), 403);
        let listed: Value = with!(request.get("/api/director-packs"), partner).await.json();
        assert_eq!(listed["packs"].as_array().unwrap().len(), 0, "the partner is shown no draft");
        let issued: Value = with!(request.post(&format!("/api/director-packs/{pid}/issue")), issuer).await.json();
        assert_eq!((issued["status"].as_str(), issued["issued_by"].as_str()), (Some("issued"), Some(ISSUER_SUB)));
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), issuer).await.status_code(), 409, "issuing twice");

        // The partner reads the issued pack and nothing live.
        let first = with!(request.get(&format!("/api/director-packs/{pid}")), partner).await;
        assert_eq!(first.status_code(), 200);
        let first_text = first.text();
        for live in ["/api/plans", "/api/issues", "/api/change-requests", "/api/milestones", "/api/director-pack/issues-and-changes", "/api/search?q=x"] {
            assert_eq!(with!(request.get(live), partner).await.status_code(), 404, "{live}");
        }
        assert_eq!(with!(request.get(&format!("/api/plans/{pubp}")), partner).await.status_code(), 404);
        let create = with!(request.post("/api/plans"), partner).json(&json!({ "kind": "Project", "name": "x" })).await.status_code();
        assert!(create == 403 || create == 404, "a confined reader cannot write: {create}");
        // Another audience's pack is not theirs.
        let b: Value = with!(request.post("/api/director-packs"), editor).json(&json!({ "period": "2026-10", "audience": "board" })).await.json();
        let bpid = b["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/director-packs/{bpid}/issue")), issuer).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/director-packs/{bpid}")), partner).await.status_code(), 404);
        assert_eq!(with!(request.get(&format!("/api/director-packs/{bpid}")), board).await.status_code(), 200);
        assert_eq!(with!(request.get(&format!("/api/director-packs/{pid}")), board).await.status_code(), 404);
        let listed: Value = with!(request.get("/api/director-packs"), partner).await.json();
        assert_eq!(listed["packs"].as_array().unwrap().len(), 1);

        // The body: only counted what is public, no person ref, no cost rate, sections honestly absent.
        let pack: Value = serde_json::from_str(&first_text).unwrap();
        assert_eq!(pack["body"]["issues"]["open"], 1, "the private plan's issue is not counted");
        assert_eq!(pack["body"]["data_quality"]["plans_left_out_private_or_marked"], 1);
        assert!(!first_text.contains("SECRET-ISSUE"));
        assert!(!first_text.contains("person:") && !first_text.to_lowercase().contains("cost_rate") && !first_text.contains(WRITER));
        assert!(first_text.contains("no earlier pack to compare with"));
        assert_eq!(pack["body"]["what_changed_items"][0], json!({ "key": "open_issues", "now": 1, "before": null }));

        // Byte-identical after the data changes, and on every read.
        with!(request.post(&format!("/api/plans/{pubp}/issues")), writer)
            .json(&json!({ "title": "Another", "severity": "critical", "status": "open" })).await.assert_status_ok();
        for who in [&partner, &issuer, &editor] {
            assert_eq!(with!(request.get(&format!("/api/director-packs/{pid}")), who).await.text(), first_text);
        }

        // A reissue: needs a real note, keeps v1 readable and links them, and sees the new data.
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/reissue")), editor).json(&json!({ "change_note": "fix" })).await.status_code(), 422);
        let r: Value = with!(request.post(&format!("/api/director-packs/{pid}/reissue")), editor).json(&json!({ "change_note": "Added the issue raised after drafting" })).await.json();
        let rpid = r["pid"].as_str().unwrap().to_string();
        assert_eq!((r["version"].as_i64(), r["supersedes_pid"].as_str()), (Some(2), Some(pid.as_str())));
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/reissue")), editor).json(&json!({ "change_note": "Second try at it" })).await.status_code(), 409, "one open draft per period and audience");
        with!(request.post(&format!("/api/director-packs/{rpid}/issue")), issuer).await.assert_status_ok();
        let v2: Value = with!(request.get(&format!("/api/director-packs/{rpid}")), partner).await.json();
        assert_eq!(v2["body"]["issues"]["open"], 2);
        assert_eq!(v2["body"]["cover"]["change_note"], "Added the issue raised after drafting");
        assert_eq!(with!(request.get(&format!("/api/director-packs/{pid}")), partner).await.text(), first_text, "v1 is untouched");
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/reissue")), editor).json(&json!({ "change_note": "From the old version" })).await.status_code(), 409, "only the latest is reissued");

        // The audit trail.
        let rows = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT action, actor FROM audit_logs WHERE action LIKE 'director_pack_%' ORDER BY id".to_string())).await.unwrap();
        let trail: Vec<(String, String)> = rows.iter().map(|r| (r.try_get("", "action").unwrap(), r.try_get("", "actor").unwrap())).collect();
        assert_eq!(trail.first(), Some(&("director_pack_drafted".to_string(), EDITOR.to_string())));
        assert!(trail.contains(&("director_pack_issued".to_string(), ISSUER_SUB.to_string())));
        assert!(trail.contains(&("director_pack_reissue_drafted".to_string(), EDITOR.to_string())));

        // The database does not rely on the service: an issued pack cannot be changed or deleted.
        for (what, sql) in [
            ("edit the body", format!("UPDATE director_packs SET body = '{{}}' WHERE pid = '{pid}'")),
            ("edit the note", format!("UPDATE director_packs SET change_note = 'a long enough note' WHERE pid = '{pid}'")),
            ("un-issue", format!("UPDATE director_packs SET status = 'draft', issued_at = NULL, issued_by = NULL WHERE pid = '{pid}'")),
            ("delete", format!("DELETE FROM director_packs WHERE pid = '{pid}'")),
            ("a bad audience", "INSERT INTO director_packs (pid, period, audience, version, status, cut_off, body, created_at, updated_at) VALUES (gen_random_uuid(), '2026-11', 'public', 1, 'draft', now(), '{}', now(), now())".to_string()),
            ("a bad period", "INSERT INTO director_packs (pid, period, audience, version, status, cut_off, body, created_at, updated_at) VALUES (gen_random_uuid(), '2026-1', 'board', 1, 'draft', now(), '{}', now(), now())".to_string()),
            ("issued with no issuer", "INSERT INTO director_packs (pid, period, audience, version, status, cut_off, body, created_at, updated_at) VALUES (gen_random_uuid(), '2026-11', 'board', 1, 'issued', now(), '{}', now(), now())".to_string()),
            ("version 2 without a note", "INSERT INTO director_packs (pid, period, audience, version, status, cut_off, body, supersedes_pid, created_at, updated_at) VALUES (gen_random_uuid(), '2026-11', 'board', 2, 'draft', now(), '{}', gen_random_uuid(), now(), now())".to_string()),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }
        assert_eq!(with!(request.get(&format!("/api/director-packs/{pid}")), partner).await.text(), first_text, "and still the same");

        // The one allowed change: a data-subject pseudonymisation may blank who drafted or issued it,
        // and nothing else in the row moves, the body included.
        let tx = ctx.db.begin().await.unwrap();
        tx.execute_unprepared("SELECT set_config('ppm.audit_pseudonymise', 'on', true)").await.unwrap();
        tx.execute_unprepared(&format!("UPDATE director_packs SET drafted_by = 'pseudonym-1', issued_by = 'pseudonym-1' WHERE pid = '{pid}'")).await.unwrap();
        assert!(tx.execute_unprepared(&format!("UPDATE director_packs SET body = '{{}}' WHERE pid = '{pid}'")).await.is_err(), "not even then the body");
        tx.rollback().await.unwrap();
        let tx = ctx.db.begin().await.unwrap();
        tx.execute_unprepared("SELECT set_config('ppm.audit_pseudonymise', 'on', true)").await.unwrap();
        tx.execute_unprepared(&format!("UPDATE director_packs SET drafted_by = 'pseudonym-1', issued_by = 'pseudonym-1' WHERE pid = '{pid}'")).await.unwrap();
        tx.commit().await.unwrap();
        let after = with!(request.get(&format!("/api/director-packs/{pid}")), partner).await.text();
        assert!(after.contains("pseudonym-1") && after.ends_with(first_text.split("\"body\":").nth(1).unwrap()), "the body bytes are the same");
    })
    .await;
}
