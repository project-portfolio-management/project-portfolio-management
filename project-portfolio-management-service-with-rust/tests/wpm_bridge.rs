//! The bridge to the workforce service (FB-1 to FB-3), end to end with real PASETO identities and a stand-in
//! workforce service on a local port: the pool link (set, refused, unique, cleared, audited with no figure), the
//! heatmap taking a linked pool's supply from WPM with its source and time and never adding what was entered here, the
//! last figure with its age when WPM is down or slow (only to the caller whose token fetched it), a hidden month as unknown, demand sent under the caller's own
//! token with the exact body WPM takes (with `sent_at`), a zero sent as the withdrawal (FB-5), every figure withdrawn
//! when a pool is unlinked, an unlinked pool exactly as before, and (FB-4) a plan's
//! people impact passed through from WPM with the plan's own end sent, `unknown` when WPM refuses, is down or slow.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`, and this sets the worker-service URL).
//! `#[ignore]`d — needs PostgreSQL. Run with `cargo test --test wpm_bridge -- --ignored`.
//!
//! The stand-in answers with the shapes read from WPM's `src/controllers/capacity/peer.rs` and `people_impact.rs`; **no running WPM is
//! called**.

#![expect(
    clippy::many_single_char_names,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]
#![cfg(feature = "paseto")]

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::audit_logs;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::ConnectionTrait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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

const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000b1";

/// One request the stand-in saw: method, query, authorization header, body.
type Seen = (String, String, Option<String>, Value);

/// What the stand-in WPM saw, and how it behaves: 0 answers, 1 is down (503), 2 is slow (10 s), 3 refuses (403).
#[derive(Default)]
struct Stub {
    mode: AtomicU8,
    seen: Mutex<Vec<Seen>>,
    supply: Mutex<Value>,
    impact: Mutex<Value>,
}

async fn stand_in(stub: Arc<Stub>) -> String {
    use axum::extract::{Json, RawQuery, State};
    use axum::http::{HeaderMap, StatusCode};
    use axum::response::IntoResponse;
    fn auth(h: &HeaderMap) -> Option<String> {
        h.get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
    }
    async fn behave(s: &Stub) -> Option<StatusCode> {
        match s.mode.load(Ordering::SeqCst) {
            1 => Some(StatusCode::SERVICE_UNAVAILABLE),
            3 => Some(StatusCode::FORBIDDEN),
            2 => {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                None
            }
            _ => None,
        }
    }
    let app = axum::Router::new()
        .route(
            "/api/capacity/peer-supply",
            axum::routing::get(
                |State(s): State<Arc<Stub>>, h: HeaderMap, RawQuery(q): RawQuery| async move {
                    s.seen.lock().unwrap().push((
                        "GET".into(),
                        q.unwrap_or_default(),
                        auth(&h),
                        Value::Null,
                    ));
                    if let Some(code) = behave(&s).await {
                        return code.into_response();
                    }
                    Json(s.supply.lock().unwrap().clone()).into_response()
                },
            ),
        )
        .route(
            "/api/capacity/people-impact",
            axum::routing::get(
                |State(s): State<Arc<Stub>>, h: HeaderMap, RawQuery(q): RawQuery| async move {
                    s.seen.lock().unwrap().push((
                        "IMPACT".into(),
                        q.unwrap_or_default(),
                        auth(&h),
                        Value::Null,
                    ));
                    if let Some(code) = behave(&s).await {
                        return code.into_response();
                    }
                    Json(s.impact.lock().unwrap().clone()).into_response()
                },
            ),
        )
        .route(
            "/api/capacity/peer-demand",
            axum::routing::post(
                |State(s): State<Arc<Stub>>, h: HeaderMap, Json(b): Json<Value>| async move {
                    s.seen
                        .lock()
                        .unwrap()
                        .push(("POST".into(), String::new(), auth(&h), b));
                    if let Some(code) = behave(&s).await {
                        return code.into_response();
                    }
                    Json(json!({ "pid": uuid::Uuid::new_v4(), "duplicate": false })).into_response()
                },
            ),
        )
        .with_state(stub);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    format!("http://{addr}")
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test wpm_bridge -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn the_bridge_keeps_one_owner_per_fact() {
    let (keys, kid) = keys_and_kid();
    let stub = Arc::new(Stub::default());
    let base = stand_in(stub.clone()).await;
    // SAFETY: this binary is single-test, so nothing races these settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL", &base);
    }
    let token = mint(&kid, WRITER, &[("access", "write")]);
    let writer = auth_header(&token);
    let first = {
        use chrono::Datelike;
        chrono::Utc::now().date_naive().with_day(1).unwrap()
    };
    let next = first + chrono::Months::new(1);
    let (month, month2) = (
        first.format("%Y-%m").to_string(),
        next.format("%Y-%m").to_string(),
    );
    let last = next.pred_opt().unwrap();
    let tag = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();
    let (pool_name, role) = (format!("Bridge {tag}"), format!("BridgeDev{tag}"));
    let peer = uuid::Uuid::new_v4().to_string();
    *stub.supply.lock().unwrap() = json!({
        "pool_pid": peer, "peer_pool_ref": "ignored", "source": "wpm", "as_of": "2026-10-30T09:00:00Z", "floor": 5,
        "months": [
            { "month": format!("{first}"), "supply_centi": 300, "hidden": false },
            { "month": format!("{next}"), "supply_centi": null, "hidden": true },
        ],
    });
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr) => { $rb.add_header(writer.0.clone(), writer.1.clone()) }; }
        let prog: Value = with!(request.post("/api/plans")).json(&json!({ "kind": "Program", "name": format!("Care {tag}") })).await.json();
        let prog = prog["pid"].as_str().unwrap().to_string();
        let pool: Value = with!(request.post("/api/skill-pools")).json(&json!({ "name": pool_name, "roles": [role] })).await.json();
        let pool = pool["pid"].as_str().unwrap().to_string();
        assert_eq!(pool.len(), 36);
        let other: Value = with!(request.post("/api/skill-pools")).json(&json!({ "name": format!("Other {tag}") })).await.json();
        let other = other["pid"].as_str().unwrap().to_string();
        ctx.db.execute_unprepared(&format!("INSERT INTO resource_roles (pid, person_ref, role, created_at, updated_at) VALUES (gen_random_uuid(), 'person:b{tag}', '{role}', now(), now())")).await.unwrap();
        ctx.db.execute_unprepared(&format!("INSERT INTO allocations (pid, plan_pid, person_ref, role, percent, start_date, end_date, skills_required, created_at, updated_at) VALUES (gen_random_uuid(), '{prog}', 'person:b{tag}', '{role}', 100, '{first}', '{last}', '[]', now(), now())")).await.unwrap();
        let heat = || async { with!(request.get(&format!("/api/capacity-heatmap?from={month}&months=2"))).await };
        let cell = |v: &Value, m: &str| v["pools"].as_array().unwrap().iter().find(|p| p["name"] == pool_name.as_str()).unwrap()["cells"].as_array().unwrap().iter().find(|c| c["month"] == m).cloned().unwrap();
        let posts = || stub.seen.lock().unwrap().iter().filter(|s| s.0 == "POST").cloned().collect::<Vec<_>>();

        // ── unlinked: exactly as before, and nothing is called ──
        let v: Value = heat().await.json();
        let c = cell(&v, &month);
        assert_eq!((c["supply_source"].as_str(), c["supply_fte"].as_f64(), c["demand_fte"].as_f64()), (Some("here"), Some(1.0), Some(1.0)), "{c}");
        assert!(c["peer_supply"].is_null());
        assert!(stub.seen.lock().unwrap().is_empty(), "an unlinked pool calls nothing");

        // ── the link: shape checked, one PPM pool per WPM pool, audited with no figure ──
        let link = |pid: String, r: Value| { let request = &request; let w = writer.clone(); async move { request.put(&format!("/api/skill-pools/{pid}/peer-link")).add_header(w.0, w.1).json(&json!({ "peer_pool_ref": r })).await } };
        assert_eq!(link(pool.clone(), json!("Engineering")).await.status_code(), 422, "never a name");
        assert_eq!(link(pool.clone(), json!(peer.to_uppercase())).await.status_code(), 422);
        assert_eq!(link(uuid::Uuid::new_v4().to_string(), json!(peer)).await.status_code(), 404);
        let linked = link(pool.clone(), json!(peer)).await;
        assert_eq!(linked.status_code(), 200);
        assert_eq!(linked.json::<Value>()["peer_pool_ref"], json!(peer));
        assert_eq!(link(other.clone(), json!(peer)).await.status_code(), 409, "one PPM pool per WPM pool");
        let pool_uuid = uuid::Uuid::parse_str(&pool).unwrap();
        let rows = audit_logs::Entity::find().filter(audit_logs::Column::EntityPid.eq(pool_uuid)).filter(audit_logs::Column::Action.eq("skill_pool_peer_linked")).all(&ctx.db).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].snapshot.is_none(), "audited with no figure");
        let listed: Value = with!(request.get("/api/skill-pools")).await.json();
        assert!(listed["skill_pools"].as_array().unwrap().iter().any(|p| p["pid"] == pool.as_str() && p["peer_pool_ref"] == peer.as_str()), "the link is shown with the pool");

        // ── a linked pool's supply is WPM's, labelled, and what was entered here is never added ──
        with!(request.put("/api/capacity-commitments")).json(&json!({ "pool_pid": pool, "plan_pid": prog, "kind": "supply", "month": month, "committed_fte": 0.5 })).await.assert_status_ok();
        assert!(posts().is_empty(), "a supply commitment is not sent");
        let v: Value = heat().await.json();
        let c = cell(&v, &month);
        assert_eq!((c["supply_source"].as_str(), c["supply_fte"].as_f64(), c["entered_here_supply_fte"].as_f64()), (Some("wpm"), Some(3.0), Some(1.5)), "WPM's 3.0, not 4.5: {c}");
        assert_eq!((c["peer_supply"]["status"].as_str(), c["peer_supply"]["as_of"].as_str()), (Some("live"), Some("2026-10-30T09:00:00Z")));
        assert_eq!((c["word"].as_str(), c["gap_fte"].as_f64()), (Some("enough"), Some(2.0)));
        let h = cell(&v, &month2);
        assert_eq!((h["state"].as_str(), h["peer_supply"]["status"].as_str()), (Some("supply_unknown"), Some("hidden")), "a hidden month: {h}");
        assert!(h["supply_fte"].is_null() && h["gap_fte"].is_null(), "hidden is not zero");
        let got = stub.seen.lock().unwrap().iter().find(|s| s.0 == "GET").cloned().unwrap();
        assert_eq!(got.1, format!("pool={peer}&from={first}&months=2"));
        assert_eq!(got.2, Some(format!("Bearer {token}")), "the caller's own token, forwarded");
        assert_eq!(v["table"].as_array().unwrap().iter().find(|t| t["pool"] == pool_name.as_str() && t["month"] == month.as_str()).unwrap()["supply_source"], "wpm");

        // ── WPM down, then slow: the last figure with its age; the request never fails ──
        stub.mode.store(1, Ordering::SeqCst);
        let r = heat().await;
        assert_eq!(r.status_code(), 200);
        let c = cell(&r.json::<Value>(), &month);
        assert_eq!((c["peer_supply"]["status"].as_str(), c["supply_fte"].as_f64()), (Some("last_received"), Some(3.0)), "{c}");
        assert!(c["peer_supply"]["age_seconds"].as_i64().unwrap() >= 0);
        assert_eq!(c["peer_supply"]["reason"], "WPM answered 503");
        // Another caller, WPM still down: never the figure the first caller's token fetched (WPM-D135).
        let second = auth_header(&mint(&kid, "3f2c6b7e-0000-4000-8000-0000000000b3", &[("access", "write")]));
        let r = request.get(&format!("/api/capacity-heatmap?from={month}&months=2")).add_header(second.0.clone(), second.1.clone()).await;
        assert_eq!(r.status_code(), 200);
        let c = cell(&r.json::<Value>(), &month);
        assert_eq!((c["peer_supply"]["status"].as_str(), c["state"].as_str()), (Some("unknown"), Some("supply_unknown")), "another caller's figure is not shown: {c}");
        assert!(c["supply_fte"].is_null(), "{c}");
        stub.mode.store(2, Ordering::SeqCst);
        let started = std::time::Instant::now();
        let r = heat().await;
        assert!(started.elapsed() < std::time::Duration::from_secs(6), "a slow WPM is abandoned");
        assert_eq!(r.status_code(), 200);
        assert_eq!(cell(&r.json::<Value>(), &month)["peer_supply"]["status"], "last_received");
        stub.mode.store(0, Ordering::SeqCst);
        // The pack-style reads (the drum) ask nothing and show no kept figure.
        let before = stub.seen.lock().unwrap().len();
        with!(request.get(&format!("/api/portfolio-drum?from={month}&months=2"))).await.assert_status_ok();
        assert_eq!(stub.seen.lock().unwrap().len(), before, "the drum makes no call");

        // ── demand out: the programme's total for the pool and month, under the caller's token ──
        let put = |body: Value| { let request = &request; let w = writer.clone(); async move { let r = request.put("/api/capacity-commitments").add_header(w.0, w.1).json(&body).await; assert_eq!(r.status_code(), 200); r.json::<Value>() } };
        let r = put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 1.5 })).await;
        assert_eq!((r["peer_demand_send"]["outcome"].as_str(), r["peer_demand_send"]["fte_centi"].as_i64()), (Some("started"), Some(150)), "{r}");
        let wait_for = |n: usize| async move { for _ in 0..50 { if posts().len() >= n { return; } tokio::time::sleep(std::time::Duration::from_millis(100)).await; } panic!("no send reached the stand-in"); };
        wait_for(1).await;
        let (_, _, auth, body) = posts()[0].clone();
        assert_eq!(auth, Some(format!("Bearer {token}")), "the caller's own token, no service account");
        let event = r["peer_demand_send"]["peer_event_id"].as_str().unwrap().to_string();
        let sent_at = body["sent_at"].as_str().unwrap().to_string();
        assert_eq!(body, json!({ "pool_pid": peer, "programme_ref": format!("thing:{prog}"), "month": format!("{first}"), "fte_centi": 150, "peer_event_id": event, "sent_at": sent_at }));
        let at = chrono::DateTime::parse_from_rfc3339(&sent_at).expect("sent_at is RFC 3339");
        assert!(event.ends_with(&format!(":{}", at.timestamp_micros())), "sent_at is the time of the write the event id names: {event} {sent_at}");
        let mut outcome = Value::Null;
        for _ in 0..50 {
            let l: Value = with!(request.get("/api/capacity-commitments")).await.json();
            outcome = l["commitments"].as_array().unwrap().iter().find(|c| c["pool_pid"] == pool.as_str() && c["kind"] == "demand").unwrap()["peer_demand_send"]["outcome"].clone();
            if outcome == "sent" { break; }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        assert_eq!(outcome, "sent", "the outcome is recorded");
        // A partner's demand on the same pool and month: WPM keeps one figure per programme, so the total goes.
        put(json!({ "pool_pid": pool, "plan_pid": prog, "organization": "Acme", "kind": "demand", "month": month, "committed_fte": 0.5 })).await;
        wait_for(2).await;
        assert_eq!(posts()[1].3["fte_centi"], 200);
        assert_ne!(posts()[1].3["peer_event_id"], posts()[0].3["peer_event_id"], "a new version, a new id");
        // Demand stays PPM's on the heatmap: 1.0 allocated + 2.0 owed.
        assert_eq!(cell(&heat().await.json::<Value>(), &month)["demand_fte"], 3.0);
        // WPM refusing is recorded, and the write still succeeded.
        stub.mode.store(1, Ordering::SeqCst);
        put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 1.0 })).await;
        wait_for(3).await;
        stub.mode.store(0, Ordering::SeqCst);
        // A total of zero is sent as fte_centi 0: WPM withdraws its ppm claim (FB-5).
        put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 0 })).await;
        wait_for(4).await;
        assert_eq!(posts()[3].3["fte_centi"], 50, "the first zero still had Acme's 0.5");
        let z = put(json!({ "pool_pid": pool, "plan_pid": prog, "organization": "Acme", "kind": "demand", "month": month, "committed_fte": 0 })).await;
        assert_eq!((z["peer_demand_send"]["outcome"].as_str(), z["peer_demand_send"]["fte_centi"].as_i64(), z["peer_demand_send"]["detail"].as_str()), (Some("started"), Some(0), Some("withdrawing the figure from the workforce service")), "{z}");
        wait_for(5).await;
        let w = posts()[4].3.clone();
        assert_eq!(w, json!({ "pool_pid": peer, "programme_ref": format!("thing:{prog}"), "month": format!("{first}"), "fte_centi": 0, "peer_event_id": z["peer_demand_send"]["peer_event_id"], "sent_at": w["sent_at"] }), "the exact withdrawal");
        let t = |v: &Value| chrono::DateTime::parse_from_rfc3339(v["sent_at"].as_str().unwrap()).unwrap();
        assert!(t(&w) > t(&posts()[3].3), "a later write, a later sent_at");
        let send_of = |l: &Value, m: &str| l["commitments"].as_array().unwrap().iter().find(|c| c["pool_pid"] == pool.as_str() && c["kind"] == "demand" && c["month"] == m && c["organization"].is_null()).unwrap()["peer_demand_send"].clone();
        let mut s = Value::Null;
        for _ in 0..50 {
            s = send_of(&with!(request.get("/api/capacity-commitments")).await.json::<Value>(), &month);
            if s["outcome"] == "sent" { break; }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        assert_eq!((s["outcome"].as_str(), s["fte_centi"].as_i64(), s["detail"].as_str()), (Some("sent"), Some(0), Some("WPM took the withdrawal")), "{s}");
        // A figure in the next month, so the unlink below has one to withdraw.
        let r2 = put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month2, "committed_fte": 0.75 })).await;
        assert_eq!(r2["peer_demand_send"]["fte_centi"], 75);
        wait_for(6).await;
        // No token is not a send.
        let r = request.put("/api/capacity-commitments").json(&json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert_eq!(r.status_code(), 401, "this deployment requires a token anyway");

        // ── unlinked again: every figure sent there is withdrawn (best effort), then local figures at once ──
        let u = link(pool.clone(), Value::Null).await;
        assert_eq!(u.status_code(), 200);
        let u: Value = u.json();
        let ws = u["peer_demand_withdrawals"].as_array().unwrap().clone();
        assert_eq!(ws.len(), 1, "only the month whose latest send carried a figure; the first month is already withdrawn: {u}");
        assert_eq!((ws[0]["month"].as_str(), ws[0]["outcome"].as_str(), ws[0]["fte_centi"].as_i64()), (Some(month2.as_str()), Some("started"), Some(0)));
        wait_for(7).await;
        let w = posts()[6].clone();
        assert_eq!(w.2, Some(format!("Bearer {token}")), "under the unlinking caller's own token");
        assert_eq!(w.3, json!({ "pool_pid": peer, "programme_ref": format!("thing:{prog}"), "month": format!("{next}"), "fte_centi": 0, "peer_event_id": ws[0]["peer_event_id"], "sent_at": w.3["sent_at"] }), "the withdrawal goes to the pool it was linked to");
        let mut s = Value::Null;
        for _ in 0..50 {
            s = send_of(&with!(request.get("/api/capacity-commitments")).await.json::<Value>(), &month2);
            if s["outcome"] == "sent" { break; }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        assert_eq!((s["outcome"].as_str(), s["fte_centi"].as_i64()), (Some("sent"), Some(0)), "the withdrawal is recorded and shown after the unlink: {s}");
        let c = cell(&heat().await.json::<Value>(), &month);
        assert_eq!((c["supply_source"].as_str(), c["supply_fte"].as_f64()), (Some("here"), Some(1.5)), "{c}");
        let n = posts().len();
        put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 2 })).await;
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        assert_eq!(posts().len(), n, "an unlinked pool sends nothing");

        // ── people impact (FB-4): WPM's answer passed through, the plan's own end sent, the caller's token ──
        let impacts = || stub.seen.lock().unwrap().iter().filter(|s| s.0 == "IMPACT").cloned().collect::<Vec<_>>();
        with!(request.put(&format!("/api/plans/{prog}"))).json(&json!({ "kind": "Program", "name": format!("Care {tag}"), "target_date": "2027-06-30" })).await.assert_status_ok();
        *stub.impact.lock().unwrap() = json!({
            "programme_ref": format!("thing:{prog}"), "source": "wpm", "as_of": "2026-10-30",
            "programme_ends_on": "2027-06-30", "programme_ends_on_source": "caller", "months": [format!("{first}")], "floor": 10,
            "workers_committed": { "known": true, "reason": null,
                "pools": [{ "pool_pid": peer, "pool": "Nursing", "count": null, "hidden": true }], "in_no_pool": { "count": 0, "hidden": false } },
            "contingent_ending_before_end": { "known": true, "reason": null, "pools": [], "in_no_pool": { "count": 0, "hidden": false }, "end_not_recorded": { "count": null, "hidden": true } },
            "over_committed_pool_months": { "known": false, "reason": "no claim", "considered": null, "over": null, "unknown": null, "pools": null },
            "derivation": "…",
        });
        let impact = |q: &'static str| { let request = &request; let w = writer.clone(); let prog = prog.clone(); async move { request.get(&format!("/api/plans/{prog}/people-impact{q}")).add_header(w.0, w.1).await } };
        let r = impact("?months=3").await;
        assert_eq!(r.status_code(), 200);
        let v: Value = r.json();
        assert_eq!((v["source"].as_str(), v["status"].as_str(), v["as_of"].as_str()), (Some("wpm"), Some("live"), Some("2026-10-30")), "{v}");
        let (_, q, auth, _) = impacts()[0].clone();
        assert_eq!(q, format!("programme=thing:{prog}&ends_on=2027-06-30&months=3"), "the plan's own end, sent");
        assert_eq!(auth, Some(format!("Bearer {token}")), "the caller's own token");
        assert_eq!(v["workers_committed"], stub.impact.lock().unwrap()["workers_committed"], "passed through unchanged");
        assert!(v["workers_committed"]["pools"][0]["count"].is_null(), "hidden is not zero");
        assert_eq!(v["over_committed_pool_months"]["known"], false);
        assert_eq!(impact("?months=0").await.status_code(), 422);
        assert_eq!(impact("?months=37").await.status_code(), 422);
        assert_eq!(impacts().len(), 1, "a bad window asks WPM nothing");
        // A refusal is unknown with the reason: no figure, none kept from the call before.
        stub.mode.store(3, Ordering::SeqCst);
        let v: Value = impact("").await.json();
        assert_eq!((v["status"].as_str(), v["reason"].as_str()), (Some("unknown"), Some("WPM refused this caller (403)")), "{v}");
        assert!(v["workers_committed"]["pools"].is_null() && v["as_of"].is_null(), "no figure: {v}");
        // Down, then slow: unknown, and the read still succeeds.
        stub.mode.store(1, Ordering::SeqCst);
        assert_eq!(impact("").await.json::<Value>()["reason"], "WPM answered 503");
        stub.mode.store(2, Ordering::SeqCst);
        let started = std::time::Instant::now();
        let r = impact("").await;
        assert!(started.elapsed() < std::time::Duration::from_secs(6), "a slow WPM is abandoned");
        assert_eq!((r.status_code().as_u16(), r.json::<Value>()["reason"].as_str()), (200, Some("WPM did not answer in time")));
        stub.mode.store(0, Ordering::SeqCst);
        // A plan with no full end date sends none.
        with!(request.put(&format!("/api/plans/{prog}"))).json(&json!({ "kind": "Program", "name": format!("Care {tag}"), "target_date": "2027-06" })).await.assert_status_ok();
        let v: Value = impact("").await.json();
        assert_eq!(impacts().last().unwrap().1, format!("programme=thing:{prog}"));
        assert!(v["plan_ends_on"].is_null() && v["plan_ends_on_reason"].as_str().unwrap().contains("not a full date"), "{v}");
        // The private-plan guard: a non-member gets 404 and WPM is not asked.
        let secret: Value = with!(request.post("/api/plans")).json(&json!({ "kind": "Program", "name": format!("Secret {tag}") })).await.json();
        let secret = secret["pid"].as_str().unwrap().to_string();
        with!(request.put(&format!("/api/plans/{secret}/access"))).json(&json!({ "private": true, "members": [] })).await.assert_status_ok();
        let outsider = auth_header(&mint(&kid, "3f2c6b7e-0000-4000-8000-0000000000b2", &[("access", "write")]));
        let n = impacts().len();
        let r = request.get(&format!("/api/plans/{secret}/people-impact")).add_header(outsider.0, outsider.1).await;
        assert_eq!(r.status_code(), 404, "a private plan is not disclosed");
        assert_eq!(impacts().len(), n, "and WPM is not asked");
        assert_eq!(with!(request.get(&format!("/api/plans/{}/people-impact", uuid::Uuid::new_v4()))).await.status_code(), 404);

        // ── stub mode: no URL, no call, and the answer says so ──
        let relinked: Value = link(pool.clone(), json!(peer)).await.json();
        assert!(relinked["peer_demand_withdrawals"].is_null(), "linking withdraws nothing");
        let n = posts().len();
        put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month2, "committed_fte": 1 })).await;
        wait_for(n + 1).await;
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL") };
        let calls = stub.seen.lock().unwrap().len();
        let r = put(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 2.5 })).await;
        assert_eq!(r["peer_demand_send"]["outcome"], "not_configured");
        let c = cell(&heat().await.json::<Value>(), &month);
        assert_eq!(c["peer_supply"]["status"], "last_received", "the figure received earlier, with its age: {c}");
        let v: Value = impact("").await.json();
        assert_eq!((v["status"].as_str(), v["reason"].as_str()), (Some("unknown"), Some("the worker service URL is not set, so WPM was not asked")));
        assert_eq!(stub.seen.lock().unwrap().len(), calls, "nothing is called without a URL");
        let fresh = uuid::Uuid::new_v4().to_string();
        let changed: Value = link(pool.clone(), json!(fresh)).await.json();
        let ws = changed["peer_demand_withdrawals"].as_array().unwrap();
        assert_eq!(ws.len(), 1, "a changed link withdraws from the old pool; a month never sent is not withdrawn: {changed}");
        assert_eq!((ws[0]["month"].as_str(), ws[0]["outcome"].as_str()), (Some(month2.as_str()), Some("not_configured")), "recorded as not sent with no URL");
        assert_eq!(stub.seen.lock().unwrap().len(), calls, "and nothing is called");
        let c = cell(&heat().await.json::<Value>(), &month);
        assert_eq!((c["peer_supply"]["status"].as_str(), c["state"].as_str()), (Some("unknown"), Some("supply_unknown")), "nothing received is unknown, never zero: {c}");
        assert!(c["supply_fte"].is_null());
        let gone: Value = link(pool.clone(), Value::Null).await.json();
        assert_eq!(gone["peer_demand_withdrawals"], json!([]), "nothing was sent to the fresh pool");
    })
    .await;
}
