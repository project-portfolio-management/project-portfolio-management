//! Delivery confidence, exceptions and the portfolio map (DP-2, DP-3, DP-4), end to end with real PASETO
//! identities: who may record an independent assessment, an append-only history, an exception that opens,
//! keeps its clock, is not closed by improving and is closed by a decision, a breach, strategic aims and
//! the map, a private programme that is nowhere, and the pack carrying all of it with a trend.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test delivery_confidence -- --ignored`.

#![expect(
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
const INDEP: &str = "3f2c6b7e-0000-4000-8000-0000000000a2";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000000a9";
const OWNER: &str = "3f2c6b7e-0000-4000-8000-0000000000b7";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test delivery_confidence -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn confidence_exceptions_and_the_map_keep_their_promises() {
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
    let indep = bearer(
        &kid,
        INDEP,
        &[("access", "write"), ("confidence", "independent")],
    );
    let issuer = bearer(
        &kid,
        ISSUER_SUB,
        &[("access", "write"), ("directorpack", "issuer")],
    );
    let outsider = bearer(&kid, OUTSIDER, &[("access", "write")]);
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let mk = |name: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone())
                    .json(&json!({ "kind": "Program", "name": name })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let alpha = mk("Alpha").await;
        let beta = mk("Beta").await;
        let gamma = mk("Gamma").await;
        let secret = mk("Secret").await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();
        let conf = |p: &str| format!("/api/plans/{p}/delivery-confidence");

        // ── DP-2: nothing assessed is not green ──
        let none: Value = with!(request.get(&conf(&alpha)), writer).await.json();
        assert_eq!(none["shown"]["value"], "not_assessed");
        // Refusals name every fault; independent needs the attribute.
        let bad = with!(request.put(&conf(&alpha)), writer).json(&json!({ "value": "purple", "kind": "boss", "rationale": "x" })).await;
        assert_eq!(bad.status_code(), 422);
        for w in ["value", "kind", "rationale"] { assert!(bad.text().contains(w), "{w}: {}", bad.text()); }
        let want_ind = json!({ "value": "green", "kind": "independent", "rationale": "Reviewed by the assurance team" });
        assert_eq!(with!(request.put(&conf(&alpha)), writer).json(&want_ind).await.status_code(), 403);
        assert_eq!(with!(request.put(&conf(&alpha)), indep).json(&json!({ "value": "green", "kind": "independent", "rationale": "Reviewed by the assurance team", "assessed_on": "2999-01-01" })).await.status_code(), 422, "not in the future");
        // A self assessment of amber, then an independent green: the independent one is shown, marked as such.
        with!(request.put(&conf(&alpha)), writer).json(&json!({ "value": "amber", "kind": "self", "rationale": "Two suppliers are running late" })).await.assert_status_ok();
        let s: Value = with!(request.get(&conf(&alpha)), writer).await.json();
        assert_eq!((s["shown"]["value"].as_str(), s["shown"]["kind"].as_str(), s["shown"]["marked_self"].as_bool()), (Some("amber"), Some("self"), Some(true)));
        with!(request.put(&conf(&alpha)), indep).json(&want_ind).await.assert_status_ok();
        let s: Value = with!(request.get(&conf(&alpha)), writer).await.json();
        assert_eq!((s["shown"]["value"].as_str(), s["shown"]["kind"].as_str()), (Some("green"), Some("independent")));
        assert_eq!(s["history"].as_array().unwrap().len(), 2, "history keeps both");
        // The database keeps the history append-only.
        for (what, sql) in [
            ("edit", "UPDATE delivery_confidence_assessments SET value = 'red'".to_string()),
            ("delete", "DELETE FROM delivery_confidence_assessments".to_string()),
            ("a thin rationale", format!("INSERT INTO delivery_confidence_assessments (pid, plan_pid, value, kind, rationale, assessed_on, created_at, updated_at) VALUES (gen_random_uuid(), '{alpha}', 'red', 'self', 'short', current_date, now(), now())")),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }

        // ── DP-3: an exception opens at amber-red, keeps its clock, is not closed by improving ──
        let r = with!(request.put(&conf(&beta)), writer).json(&json!({ "value": "amber_red", "kind": "self", "rationale": "The supplier has gone into administration" })).await;
        r.assert_status_ok();
        let ex = r.json::<Value>()["exception"].clone();
        let xp = ex["pid"].as_str().unwrap().to_string();
        assert_eq!((ex["status"].as_str(), ex["complete"].as_bool(), ex["breached"].as_bool()), (Some("open"), Some(false), Some(false)));
        assert_eq!(ex["missing"], json!(["cause", "decision_needed", "owner", "decide_by"]));
        let opened_at = ex["opened_at"].clone();
        let r: Value = with!(request.put(&conf(&beta)), writer).json(&json!({ "value": "red", "kind": "self", "rationale": "Now it is a total loss of supply" })).await.json();
        assert_eq!((r["exception"]["pid"].as_str(), r["exception"]["worst_value"].as_str(), r["exception"]["worsened_count"].as_i64()), (Some(xp.as_str()), Some("red"), Some(1)));
        assert_eq!(r["exception"]["opened_at"], opened_at, "a worsening does not restart the clock");
        let r: Value = with!(request.put(&conf(&beta)), writer).json(&json!({ "value": "amber", "kind": "self", "rationale": "A replacement supplier is found" })).await.json();
        assert!(r["exception"].is_null(), "improving to amber opens nothing");
        let open: Value = with!(request.get("/api/exceptions?status=open"), writer).await.json();
        assert_eq!(open["exceptions"].as_array().unwrap().len(), 1, "and does not close it");
        // Complete it. Owner by role is what a pack shows.
        let done: Value = with!(request.put(&format!("/api/exceptions/{xp}")), writer).json(&json!({
            "cause": "Supplier administration", "decision_needed": "Approve the replacement supplier",
            "owner_ref": format!("person:{OWNER}"), "owner_role": "Programme director", "decide_by": "2026-11-15" })).await.json();
        assert_eq!((done["complete"].as_bool(), done["missing"].as_array().map(Vec::len)), (Some(true), Some(0)));
        assert!(!done["completed_at"].is_null());
        // A thin decision is refused; a real one closes it; closing twice is 409.
        assert_eq!(with!(request.post(&format!("/api/exceptions/{xp}/decision")), writer).json(&json!({ "decision": "ok" })).await.status_code(), 422);
        let closed: Value = with!(request.post(&format!("/api/exceptions/{xp}/decision")), writer).json(&json!({ "decision": "Approved: switch to the replacement supplier" })).await.json();
        assert_eq!(closed["status"], "closed");
        assert_eq!(with!(request.post(&format!("/api/exceptions/{xp}/decision")), writer).json(&json!({ "decision": "Approved: switch again to be sure" })).await.status_code(), 409);
        assert_eq!(with!(request.put(&format!("/api/exceptions/{xp}")), writer).json(&json!({ "cause": "changed my mind" })).await.status_code(), 409, "a closed one is not edited");
        // A breach: an exception left incomplete for 49 hours is breached, listed, and notified once.
        let g = with!(request.put(&conf(&gamma)), writer).json(&json!({ "value": "red", "kind": "self", "rationale": "Everything is on fire at once" })).await.json::<Value>();
        let gx = g["exception"]["pid"].as_str().unwrap().to_string();
        ctx.db.execute_unprepared(&format!("UPDATE delivery_exceptions SET opened_at = now() - interval '49 hours', opened_by = '{WRITER}' WHERE pid = '{gx}'")).await.unwrap();
        let b: Value = with!(request.get("/api/exceptions?breached=true"), writer).await.json();
        assert_eq!(b["exceptions"].as_array().unwrap().len(), 1);
        assert_eq!(b["exceptions"][0]["breached"], true);
        let told = project_portfolio_management::tasks::exception_breaches::run_once(&ctx, chrono::Utc::now()).await.unwrap();
        assert_eq!(told, 1);
        assert_eq!(project_portfolio_management::tasks::exception_breaches::run_once(&ctx, chrono::Utc::now()).await.unwrap(), 0, "notified once");
        // A private programme's exception is nowhere for an outsider.
        let sx: Value = with!(request.put(&conf(&secret)), writer).json(&json!({ "value": "red", "kind": "self", "rationale": "Secret programme is failing" })).await.json();
        let sxp = sx["exception"]["pid"].as_str().map(str::to_string);
        let all: Value = with!(request.get("/api/exceptions"), outsider).await.json();
        assert!(!all.to_string().contains("Secret"), "{all}");
        if let Some(sxp) = sxp {
            assert_eq!(with!(request.put(&format!("/api/exceptions/{sxp}")), outsider).json(&json!({ "cause": "I should not see this" })).await.status_code(), 404);
            assert_eq!(with!(request.post(&format!("/api/exceptions/{sxp}/decision")), outsider).json(&json!({ "decision": "I should not see this one" })).await.status_code(), 404);
        }
        assert_eq!(with!(request.get(&conf(&secret)), outsider).await.status_code(), 404);

        // ── DP-4: strategic aims and the map ──
        let obj = |t: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/objectives").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "title": t })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let (growth, safety, plain) = (obj("Grow the business").await, obj("Keep people safe").await, obj("Just an objective").await);
        for o in [&growth, &safety] {
            with!(request.put(&format!("/api/objectives/{o}/strategic-aim")), writer).json(&json!({ "strategic": true })).await.assert_status_ok();
        }
        assert_eq!(with!(request.put(&format!("/api/plans/{alpha}/primary-aim")), writer).json(&json!({ "objective_pid": plain })).await.status_code(), 422, "not a strategic aim");
        // Alpha serves both aims, primary Growth; Beta only Safety (primary); Gamma none.
        with!(request.post(&format!("/api/plans/{alpha}/objectives")), writer).json(&json!({ "objective_pid": safety, "weight": 2 })).await.assert_status_ok();
        with!(request.put(&format!("/api/plans/{alpha}/primary-aim")), writer).json(&json!({ "objective_pid": growth })).await.assert_status_ok();
        with!(request.put(&format!("/api/plans/{beta}/primary-aim")), writer).json(&json!({ "objective_pid": safety })).await.assert_status_ok();
        // A second primary replaces the first; the database allows only one.
        with!(request.put(&format!("/api/plans/{alpha}/primary-aim")), writer).json(&json!({ "objective_pid": safety })).await.assert_status_ok();
        with!(request.put(&format!("/api/plans/{alpha}/primary-aim")), writer).json(&json!({ "objective_pid": growth })).await.assert_status_ok();
        assert!(ctx.db.execute_unprepared(&format!("UPDATE objective_links SET is_primary = true WHERE plan_pid = '{alpha}'")).await.is_err(), "two primaries");
        let map: Value = with!(request.get("/api/portfolio-map"), writer).await.json();
        let groups = map["groups"].as_array().unwrap();
        let find = |aim: &str| groups.iter().find(|g| g["aim"] == aim).unwrap()["programmes"].as_array().unwrap().iter().map(|p| p["name"].as_str().unwrap().to_string()).collect::<Vec<_>>();
        assert_eq!(find("Grow the business"), ["Alpha"]);
        assert_eq!(find("Keep people safe"), ["Beta"]);
        assert_eq!(find("no strategic aim"), ["Gamma"], "a programme with no aim is listed, not dropped");
        assert_eq!(map["programmes_total"], 3, "no double counting, and the private programme is not here");
        assert!(!map.to_string().contains("Secret"));
        let alpha_row = groups[0]["programmes"].as_array().unwrap().iter().chain(groups[1]["programmes"].as_array().unwrap()).find(|p| p["name"] == "Alpha").unwrap().clone();
        assert_eq!(alpha_row["also_serves"], json!(["Keep people safe"]));
        assert_eq!(alpha_row["confidence"]["kind"], "independent");
        assert_eq!(alpha_row["trend"], "no_earlier_pack");
        // Unmarking an aim clears who pointed at it.
        with!(request.put(&format!("/api/objectives/{safety}/strategic-aim")), writer).json(&json!({ "strategic": false })).await.assert_status_ok();
        let map2: Value = with!(request.get("/api/portfolio-map"), writer).await.json();
        assert!(map2["groups"].as_array().unwrap().iter().any(|g| g["aim"] == "no strategic aim" && g["programmes"].as_array().unwrap().iter().any(|p| p["name"] == "Beta")));
        with!(request.put(&format!("/api/objectives/{safety}/strategic-aim")), writer).json(&json!({ "strategic": true })).await.assert_status_ok();
        with!(request.put(&format!("/api/plans/{beta}/primary-aim")), writer).json(&json!({ "objective_pid": safety })).await.assert_status_ok();

        // ── the pack carries it, with a trend against the previous issued pack ──
        let draft = |period: &'static str| {
            let (request, issuer) = (&request, issuer.clone());
            async move {
                let d: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
                    .json(&json!({ "period": period, "audience": "board" })).await.json();
                let pid = d["pid"].as_str().unwrap().to_string();
                request.post(&format!("/api/director-packs/{pid}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
                pid
            }
        };
        let p1 = draft("2026-09").await;
        let first: Value = with!(request.get(&format!("/api/director-packs/{p1}")), issuer).await.json();
        assert_eq!(first["body"]["portfolio_map"]["programmes_total"], 3);
        // Alpha gets worse; the October pack says so, and an old assessment is stale.
        with!(request.put(&conf(&alpha)), indep).json(&json!({ "value": "red", "kind": "independent", "rationale": "Reviewed again after the audit" })).await.assert_status_ok();
        let delta = mk("Delta").await;
        let _epsilon = mk("Epsilon").await;
        with!(request.put(&conf(&delta)), writer).json(&json!({ "value": "amber_green", "kind": "self", "rationale": "Going well, one risk" })).await.assert_status_ok();
        let p2 = draft("2026-10").await;
        let second: Value = with!(request.get(&format!("/api/director-packs/{p2}")), issuer).await.json();
        let text = second.to_string();
        let row = |pack: &Value, name: &str| pack["body"]["portfolio_map"]["groups"].as_array().unwrap().iter()
            .flat_map(|g| g["programmes"].as_array().unwrap().clone()).find(|p| p["name"] == name).unwrap();
        assert_eq!(row(&second, "Alpha")["trend"], "down");
        assert_eq!(row(&second, "Alpha")["confidence"]["value"], "red");
        assert_eq!(row(&second, "Gamma")["trend"], "same", "red then and red now");
        assert_eq!(row(&second, "Delta")["trend"], "new", "assessed for the first time since the last pack");
        assert_eq!(row(&second, "Epsilon")["trend"], "not_assessed", "never assessed is not a direction");
        assert_eq!(row(&second, "Epsilon")["confidence"]["value"], "not_assessed");
        assert_eq!(row(&second, "Alpha")["disagreement"]["independent_vs_self"], true, "independent red against self amber");
        let ex = &second["body"]["exceptions"];
        assert!(ex["open"].as_array().unwrap().iter().any(|e| e["plan"] == "Gamma" && e["breached"] == true));
        assert!(!ex["breached"].as_array().unwrap().is_empty());
        assert!(!text.contains("person:") && !text.contains(OWNER) && !text.contains("Secret"), "no person ref, no private programme");
        assert!(text.contains("Programme director") || ex["closed_in_period"].as_array().unwrap().iter().any(|e| e["owner_role"] == "Programme director"));
        // The issued pack is frozen: a later change does not move it.
        let before = with!(request.get(&format!("/api/director-packs/{p2}")), issuer).await.text();
        with!(request.put(&conf(&alpha)), indep).json(&json!({ "value": "green", "kind": "independent", "rationale": "Recovered completely, verified" })).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/director-packs/{p2}")), issuer).await.text(), before);

        // The audit trail names who did what.
        let rows = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT action, actor FROM audit_logs WHERE action LIKE 'delivery_%' OR action LIKE 'strategic_%' OR action = 'primary_aim_set' ORDER BY id".to_string())).await.unwrap();
        let trail: Vec<String> = rows.iter().map(|r| r.try_get::<String>("", "action").unwrap()).collect();
        for a in ["delivery_confidence_set", "delivery_exception_opened", "delivery_exception_worsened", "delivery_exception_updated", "delivery_exception_closed", "strategic_aim_marked", "strategic_aim_unmarked", "primary_aim_set"] {
            assert!(trail.iter().any(|t| t == a), "{a} not audited: {trail:?}");
        }
    })
    .await;
}
