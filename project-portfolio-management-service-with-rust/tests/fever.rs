//! The buffer fever chart (DP-7), end to end with real PASETO identities:
//! a point built from baselined critical work done and the forecast slip against a declared buffer, the zone
//! at the lines, recovery and overrun drawn at the edge with the true number, no point (with the reason)
//! when the buffer, baseline or forecast is missing, an append-only buffer, a private programme that is
//! nowhere, and the pack carrying the chart with the trail from the last issued pack.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test fever -- --ignored`.
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
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test fever -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn the_fever_chart_keeps_its_promises() {
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
        let mk = |kind: &'static str, name: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone())
                    .json(&json!({ "kind": kind, "name": name })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let task = |plan: String, title: &'static str, start: Option<&'static str>, days: i64| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let mut b = json!({ "title": title, "duration_days": days });
                if let Some(s) = start { b["start_date"] = json!(s); }
                let t: Value = request.post(&format!("/api/plans/{plan}/tasks")).add_header(writer.0.clone(), writer.1.clone()).json(&b).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let prog = mk("Program", "Prog").await;
        let bare = mk("Program", "Bare").await;
        let project = mk("Project", "A project").await;
        let secret = mk("Program", "Secret").await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();
        let buf = |p: &str| format!("/api/plans/{p}/project-buffer");

        // A chain of 4 + 3 + 3 = 10 days, and a side task with float.
        let t1 = task(prog.clone(), "T1", Some("2026-10-05"), 4).await;
        let t2 = task(prog.clone(), "T2", None, 3).await;
        let t3 = task(prog.clone(), "T3", None, 3).await;
        let _side = task(prog.clone(), "Side", Some("2026-10-05"), 1).await;
        for (a, b) in [(&t1, &t2), (&t2, &t3)] {
            with!(request.post("/api/dependencies"), writer).json(&json!({ "predecessor_pid": a, "successor_pid": b })).await.assert_status_ok();
        }

        // No buffer: no point, and the reason is said.
        let none: Value = with!(request.get(&buf(&prog)), writer).await.json();
        assert!(none["now"]["point"].is_null());
        assert_eq!(none["now"]["no_fever_chart_point"]["code"], "no_buffer");
        // A buffer without a baseline: no point, a different reason.
        assert_eq!(with!(request.put(&buf(&bare)), writer).json(&json!({ "rule": "half_of_critical_duration" })).await.status_code(), 422, "the rule needs a baseline");
        with!(request.put(&buf(&bare)), writer).json(&json!({ "buffer_days": 5 })).await.assert_status_ok();
        let nb: Value = with!(request.get(&buf(&bare)), writer).await.json();
        assert_eq!(nb["now"]["no_fever_chart_point"]["code"], "no_baseline");
        // Refusals.
        assert_eq!(with!(request.put(&buf(&project)), writer).json(&json!({ "buffer_days": 5 })).await.status_code(), 422, "only a programme");
        for bad in [json!({}), json!({ "buffer_days": 0 }), json!({ "buffer_days": 2001 }), json!({ "buffer_days": 5, "rule": "half_of_critical_duration" }), json!({ "rule": "double" })] {
            assert_eq!(with!(request.put(&buf(&prog)), writer).json(&bad).await.status_code(), 422, "{bad}");
        }

        // Baseline, then the stated rule gives half of the 10 critical days.
        with!(request.post(&format!("/api/plans/{prog}/schedule-baselines")), writer).json(&json!({ "reason": "Approved plan" })).await.assert_status_ok();
        let ruled: Value = with!(request.put(&buf(&prog)), writer).json(&json!({ "rule": "half_of_critical_duration" })).await.json();
        assert_eq!((ruled["buffer_days"].as_i64(), ruled["source"].as_str()), (Some(5), Some("rule")), "{ruled}");
        // A person declares ten days; the latest wins and the earlier stays on record.
        with!(request.put(&buf(&prog)), writer).json(&json!({ "buffer_days": 10, "note": "Agreed with the sponsor" })).await.assert_status_ok();
        let now: Value = with!(request.get(&buf(&prog)), writer).await.json();
        assert_eq!(now["history"].as_array().unwrap().len(), 2);
        let p0 = &now["now"]["point"];
        assert_eq!((p0["chain_completed_pct"].as_f64(), p0["buffer_consumed_pct"].as_f64(), p0["zone"].as_str()), (Some(0.0), Some(0.0), Some("green")), "{now}");

        // T1 done: 4 of 10 critical days = 40 %.
        with!(request.patch(&format!("/api/plans/{prog}/tasks/{t1}")), writer).json(&json!({ "status": "done" })).await.assert_status_ok();
        let set_len = |days: i64| {
            let (request, writer, prog, t3) = (&request, writer.clone(), prog.clone(), t3.clone());
            async move {
                request.put(&format!("/api/plans/{prog}/tasks/{t3}/schedule")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "duration_days": days })).await.assert_status_ok();
                let v: Value = request.get(&format!("/api/plans/{prog}/project-buffer")).add_header(writer.0.clone(), writer.1.clone()).await.json();
                v["now"]["point"].clone()
            }
        };
        // +6 days against a ten-day buffer: 60 % at 40 % done. Lines there: 46 and 76 → amber.
        let amber = set_len(9).await;
        assert_eq!((amber["chain_completed_pct"].as_f64(), amber["buffer_consumed_pct"].as_f64(), amber["zone"].as_str(), amber["slip_days"].as_i64()), (Some(40.0), Some(60.0), Some("amber"), Some(6)), "{amber}");

        // The pack for September holds that point.
        let issue = |period: &'static str| {
            let (request, issuer) = (&request, issuer.clone());
            async move {
                let d: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone()).json(&json!({ "period": period, "audience": "board" })).await.json();
                let pid = d["pid"].as_str().unwrap().to_string();
                request.post(&format!("/api/director-packs/{pid}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
                let p: Value = request.get(&format!("/api/director-packs/{pid}")).add_header(issuer.0.clone(), issuer.1.clone()).await.json();
                p
            }
        };
        let first = issue("2026-09").await;
        let row = |pack: &Value, name: &str| pack["body"]["buffer_fever"]["programmes"].as_array().unwrap().iter().find(|p| p["plan"] == name).cloned().unwrap();
        assert_eq!(row(&first, "Prog")["point"]["zone"], "amber");
        assert!(row(&first, "Prog")["previous"].is_null(), "no earlier pack, no trail");

        // +8 days: 80 % > 76 % → red. Then the true overrun is drawn at the edge.
        let red = set_len(11).await;
        assert_eq!((red["buffer_consumed_pct"].as_f64(), red["zone"].as_str(), red["edge"].is_null()), (Some(80.0), Some("red"), true));
        let over = set_len(30).await;
        assert_eq!((over["buffer_consumed_pct"].as_f64(), over["plotted_y_pct"].as_f64(), over["edge"].as_str(), over["zone"].as_str()), (Some(270.0), Some(100.0), Some("above"), Some("red")), "{over}");
        // Shorter than planned: recovery below 0 %, drawn at 0 with the true number.
        let rec = set_len(1).await;
        assert_eq!((rec["buffer_consumed_pct"].as_f64(), rec["plotted_y_pct"].as_f64(), rec["edge"].as_str(), rec["zone"].as_str()), (Some(-20.0), Some(0.0), Some("below"), Some("green")), "{rec}");
        set_len(11).await;

        // October's pack: red now, amber before, and every point is in the table.
        let second = issue("2026-10").await;
        let r = row(&second, "Prog");
        assert_eq!((r["point"]["zone"].as_str(), r["previous"]["zone"].as_str()), (Some("red"), Some("amber")), "the trail runs from the last pack to this one");
        let table = second["body"]["buffer_fever"]["table"].as_array().unwrap();
        assert!(table.iter().any(|t| t["plan"] == "Prog" && t["previous_zone"] == "amber" && t["zone"] == "red"));
        assert_eq!(second["body"]["buffer_fever"]["lines"]["green_amber"]["at_start"], 30.0);
        // No buffer / no baseline programmes are listed with the reason, never as zero.
        assert_eq!(row(&second, "Bare")["no_fever_chart_point"]["code"], "no_baseline");
        assert!(row(&second, "Bare")["point"].is_null());
        let text = second.to_string();
        assert!(!text.contains("Secret"), "a private programme is not in the pack");
        assert!(second["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "buffer_fever"));
        assert!(!second["body"]["data_quality"]["sections_not_yet_built"].to_string().contains("fever"));
        // Frozen: later work does not move an issued pack.
        let frozen = second.to_string();
        set_len(3).await;
        let pid = second["body"]["cover"]["pack_pid"].as_str().unwrap();
        let again: Value = with!(request.get(&format!("/api/director-packs/{pid}")), issuer).await.json();
        assert_eq!(again.to_string(), frozen);

        // ── a private programme is nowhere ──
        ctx.db.execute_unprepared(&format!("INSERT INTO programme_buffers (pid, plan_pid, buffer_days, source, declared_on, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', 5, 'declared', current_date, now(), now())")).await.unwrap();
        assert_eq!(with!(request.put(&buf(&secret)), writer).json(&json!({ "buffer_days": 5 })).await.status_code(), 404, "not even its creator, when it is private to others");
        assert_eq!(with!(request.get(&buf(&secret)), outsider).await.status_code(), 404);
        assert_eq!(with!(request.put(&buf(&secret)), outsider).json(&json!({ "buffer_days": 5 })).await.status_code(), 404);
        let live: Value = with!(request.get("/api/buffer-fever"), outsider).await.json();
        assert!(!live.to_string().contains("Secret"));

        // ── the database does not rely on the service being right ──
        for (what, sql) in [
            ("an edit", "UPDATE programme_buffers SET buffer_days = 99".to_string()),
            ("a delete", "DELETE FROM programme_buffers".to_string()),
            ("zero days", format!("INSERT INTO programme_buffers (pid, plan_pid, buffer_days, source, declared_on, created_at, updated_at) VALUES (gen_random_uuid(), '{prog}', 0, 'declared', current_date, now(), now())")),
            ("a rule with no rule", format!("INSERT INTO programme_buffers (pid, plan_pid, buffer_days, source, declared_on, created_at, updated_at) VALUES (gen_random_uuid(), '{prog}', 3, 'rule', current_date, now(), now())")),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }
        let rows = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT actor FROM audit_logs WHERE action = 'project_buffer_set'".to_string())).await.unwrap();
        assert!(rows.len() >= 3 && rows.iter().all(|r| r.try_get::<String>("", "actor").unwrap() == WRITER));
    })
    .await;
}
