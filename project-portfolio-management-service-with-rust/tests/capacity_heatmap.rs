//! The capacity heatmap by skill pool and month (DP-10), end to end with real PASETO identities:
//! demand, supply and the gap per pool and month, each with a word and a shape as well as a colour, a pool
//! with no supplier as "no supply recorded" and not zero, partner supply apart from the organization's
//! own and then combined, partner names hidden unless shown, a role in at most one pool, no person in the
//! output, a private programme that is nowhere, and the pack carrying the section.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test capacity_heatmap -- --ignored`.
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
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test capacity_heatmap -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn the_heatmap_keeps_its_promises() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier or these settings.
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
    let first = {
        use chrono::Datelike;
        chrono::Utc::now().date_naive().with_day(1).unwrap()
    };
    let month = first.format("%Y-%m").to_string();
    let last = (first + chrono::Months::new(1)).pred_opt().unwrap();
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
        let prog = mk("Care").await;
        let secret = mk("Secret").await;
        ctx.db.execute_unprepared(&format!("INSERT INTO plan_access_settings (pid, plan_pid, private, members, created_at, updated_at) VALUES (gen_random_uuid(), '{secret}', true, '[]', now(), now())")).await.unwrap();

        // ── pools: a role is in at most one ──
        let pool = |name: &'static str, roles: Vec<&'static str>| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r = request.post("/api/skill-pools").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "name": name, "roles": roles })).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let (c, eng) = pool("Engineering", vec!["Developer"]).await;
        assert_eq!(c, 200);
        let eng = eng["pid"].as_str().unwrap().to_string();
        let design = pool("Design", vec!["Designer"]).await.1["pid"].as_str().unwrap().to_string();
        let _data = pool("Data", vec!["Analyst"]).await;
        assert_eq!(pool("Dupe", vec!["developer"]).await.0, 409, "a role is in one pool only, ignoring case");
        assert_eq!(pool("engineering", vec![]).await.0, 409, "names are unique ignoring case");
        assert_eq!(pool("x", vec![]).await.0, 422);

        // ── people and demand ──
        for (who, role) in [("person:d1", "Developer"), ("person:g1", "Designer")] {
            ctx.db.execute_unprepared(&format!("INSERT INTO resource_roles (pid, person_ref, role, created_at, updated_at) VALUES (gen_random_uuid(), '{who}', '{role}', now(), now())")).await.unwrap();
        }
        let alloc = |plan: &str, who: &str, role: &str, percent: i32| format!("INSERT INTO allocations (pid, plan_pid, person_ref, role, percent, start_date, end_date, skills_required, created_at, updated_at) VALUES (gen_random_uuid(), '{plan}', '{who}', '{role}', {percent}, '{first}', '{last}', '[]', now(), now())");
        for sql in [
            alloc(&prog, "person:d1", "Developer", 100), alloc(&prog, "person:d2", "Developer", 100), // 2.0 asked, 1.0 own supply
            alloc(&prog, "person:g1", "Designer", 100),                                                 // 1.0 asked, 1.0 supply
            alloc(&prog, "person:a1", "Analyst", 100),                                                  // 1.0 asked, nobody supplies
            alloc(&prog, "person:t1", "Tester", 100),                                                   // in no pool
            alloc(&secret, "person:s1", "Developer", 500),                                              // private: nowhere
        ] {
            ctx.db.execute_unprepared(&sql).await.unwrap();
        }
        let get = |q: String| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r = request.get(&format!("/api/capacity-heatmap{q}")).add_header(writer.0.clone(), writer.1.clone()).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let cell = |v: &Value, pool: &str, m: &str| v["pools"].as_array().unwrap().iter().find(|p| p["name"] == pool).unwrap()["cells"].as_array().unwrap().iter().find(|c| c["month"] == m).cloned().unwrap();
        let (code, v) = get(format!("?from={month}&months=2")).await;
        assert_eq!(code, 200);
        let e = cell(&v, "Engineering", &month);
        assert_eq!((e["demand_fte"].as_f64(), e["supply_fte"].as_f64(), e["gap_fte"].as_f64()), (Some(2.0), Some(1.0), Some(-1.0)), "{e}");
        assert_eq!((e["word"].as_str(), e["shape"].as_str(), e["colour"].as_str()), (Some("shortfall"), Some("triangle"), Some("red")));
        let d = cell(&v, "Design", &month);
        assert_eq!((d["word"].as_str(), d["shape"].as_str(), d["gap_fte"].as_f64()), (Some("tight"), Some("diamond"), Some(0.0)));
        let a = cell(&v, "Data", &month);
        assert_eq!((a["word"].as_str(), a["shape"].as_str()), (Some("no supply recorded"), Some("dash")));
        assert!(a["supply_fte"].is_null() && a["gap_fte"].is_null(), "no zero capacity is invented: {a}");
        assert_eq!(a["demand_fte"], 1.0);
        let nopool = cell(&v, "no pool assigned", &month);
        assert_eq!(nopool["demand_fte"], 1.0, "a role in no pool is shown, not dropped");
        // The next month has no allocations at all.
        assert_eq!(cell(&v, "Engineering", &(first + chrono::Months::new(1)).format("%Y-%m").to_string())["word"], "no demand", "supply with nothing asked");
        // Every cell has a word, a shape and a colour, and a table twin carries them all.
        for p in v["pools"].as_array().unwrap() { for c in p["cells"].as_array().unwrap() { assert!(c["word"].is_string() && c["shape"].is_string() && c["colour"].is_string()); } }
        assert_eq!(v["table"].as_array().unwrap().len(), v["pools"].as_array().unwrap().len() * 2);
        let text = v.to_string();
        assert!(!text.contains("person:") && !text.contains("Secret") && e["demand_fte"] != 7.0, "no person, and the private programme's 5.0 FTE is nowhere: {text}");

        // ── partner commitments ──
        let commit = |body: Value| {
            let (request, writer) = (&request, writer.clone());
            async move { let r = request.put("/api/capacity-commitments").add_header(writer.0.clone(), writer.1.clone()).json(&body).await; (r.status_code(), r.json::<Value>()) }
        };
        for bad in [
            json!({ "pool_pid": eng, "plan_pid": prog, "kind": "loan", "month": month, "committed_fte": 1 }),
            json!({ "pool_pid": eng, "plan_pid": prog, "kind": "supply", "month": "soon", "committed_fte": 1 }),
            json!({ "pool_pid": eng, "plan_pid": prog, "kind": "supply", "month": month, "committed_fte": -1 }),
            json!({ "pool_pid": uuid::Uuid::new_v4(), "plan_pid": prog, "kind": "supply", "month": month, "committed_fte": 1 }),
        ] { assert_eq!(commit(bad.clone()).await.0, 422, "{bad}"); }
        assert_eq!(commit(json!({ "pool_pid": eng, "plan_pid": secret, "kind": "supply", "month": month, "committed_fte": 1 })).await.0, 404, "a private programme");
        commit(json!({ "pool_pid": eng, "plan_pid": prog, "organization": "Acme Ltd", "kind": "supply", "month": month, "committed_fte": 1.5, "delivered_fte": 1.0 })).await;
        commit(json!({ "pool_pid": eng, "plan_pid": prog, "organization": "acme ltd", "kind": "supply", "month": month, "committed_fte": 1.5, "delivered_fte": 1.0 })).await;
        commit(json!({ "pool_pid": design, "plan_pid": prog, "organization": "Partner B", "kind": "demand", "month": month, "committed_fte": 0.5 })).await;
        let listed: Value = with!(request.get("/api/capacity-commitments"), writer).await.json();
        assert_eq!(listed["commitments"].as_array().unwrap().len(), 2, "setting it again replaced it, ignoring case");
        let (_, v2) = get(format!("?from={month}&months=1")).await;
        let e2 = cell(&v2, "Engineering", &month);
        assert_eq!((e2["own_supply_fte"].as_f64(), e2["partner_supply_committed_fte"].as_f64(), e2["supply_fte"].as_f64()), (Some(1.0), Some(1.5), Some(2.5)), "own and partner apart, then combined: {e2}");
        assert_eq!((e2["word"].as_str(), e2["partner_delivery_short"].as_bool(), e2["partners"].as_i64()), (Some("enough"), Some(true), Some(1)), "{e2}");
        assert!(e2["by_organization"].is_null() && !v2.to_string().contains("Acme"), "partner names are hidden by default");
        let d2 = cell(&v2, "Design", &month);
        assert_eq!((d2["partner_demand_fte"].as_f64(), d2["demand_fte"].as_f64(), d2["word"].as_str()), (Some(0.5), Some(1.5), Some("shortfall")));
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS", "1") };
        let (_, v3) = get(format!("?from={month}&months=1")).await;
        assert_eq!(cell(&v3, "Engineering", &month)["by_organization"][0]["organization"], "Acme Ltd");
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_HEATMAP_SHOW_PARTNERS") };
        assert_eq!(get("?months=0".to_string()).await.0, 422);
        assert_eq!(get("?from=nope".to_string()).await.0, 422);

        // ── a private programme is nowhere ──
        let out: Value = with!(request.get(&format!("/api/capacity-heatmap?from={month}&months=1")), outsider).await.json();
        assert_eq!(cell(&out, "Engineering", &month)["demand_fte"], 2.0, "the private programme's 5.0 FTE is not there for anyone");

        // ── the pack ──
        let dr: Value = request.post("/api/director-packs").add_header(issuer.0.clone(), issuer.1.clone())
            .json(&json!({ "period": month, "audience": "board" })).await.json();
        let pk = dr["pid"].as_str().unwrap().to_string();
        request.post(&format!("/api/director-packs/{pk}/issue")).add_header(issuer.0.clone(), issuer.1.clone()).await.assert_status_ok();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pk}")), issuer).await.json();
        let h = &pack["body"]["capacity_heatmap"];
        assert_eq!(h["months"].as_array().unwrap().len(), 6);
        assert_eq!(cell(h, "Engineering", &month)["word"], "enough");
        assert!(!pack.to_string().contains("person:") && !pack.to_string().contains("Acme"));

        // ── the PDF shows exactly the figures the JSON does ──
        let pdf_url = format!("/api/director-packs/{pk}/pdf");
        let r1 = with!(request.get(&pdf_url), issuer).await;
        assert_eq!(r1.status_code(), 200);
        assert_eq!(r1.header("content-type"), "application/pdf");
        let bytes = r1.as_bytes().to_vec();
        assert!(bytes.starts_with(b"%PDF"));
        assert_eq!(with!(request.get(&pdf_url), issuer).await.as_bytes().to_vec(), bytes, "an issued pack gives the same bytes every time");
        let text = project_portfolio_management::pdf::text_of(&bytes).replace("\\(", "(").replace("\\)", ")");
        let body = &pack["body"];
        let has = |needle: String| assert!(text.contains(&needle), "the PDF lacks {needle:?}");
        has(format!("Director pack {month} for board"));
        has(format!("Plans included: {}.", body["data_quality"]["plans_included"]));
        has(format!("left out (private or marked): {}", body["data_quality"]["plans_left_out_private_or_marked"]));
        for p in body["capacity_heatmap"]["pools"].as_array().unwrap() {
            has(format!("Pool {}", p["name"].as_str().unwrap()));
            for c in p["cells"].as_array().unwrap() {
                let f = |k: &str| c[k].as_f64().map_or("-".to_string(), |x| format!("{x:.3}"));
                has(format!("{} | {} ({}) | demand {} | supply {} | gap {}", c["month"].as_str().unwrap(), c["word"].as_str().unwrap(), c["shape"].as_str().unwrap(), f("demand_fte"), f("supply_fte"), f("gap_fte")));
            }
        }
        for g in body["portfolio_map"]["groups"].as_array().unwrap() {
            for prog in g["programmes"].as_array().unwrap() { has(format!("{} | status {}", prog["name"].as_str().unwrap(), prog["programme_status"].as_str().unwrap())); }
        }
        assert!(!text.contains("Secret") && !text.contains("person:") && !text.contains("Acme"), "the PDF holds nothing the JSON does not");
        assert_eq!(with!(request.get(&pdf_url), outsider).await.status_code(), 404, "the same audiences as the JSON");
        assert!(pack["body"]["data_quality"]["sections_included"].as_array().unwrap().iter().any(|s| s == "capacity_heatmap"));
        assert_eq!(pack["body"]["data_quality"]["sections_not_yet_built"].as_array().unwrap().len(), 0, "every section of the director pack now exists");
    })
    .await;
}
