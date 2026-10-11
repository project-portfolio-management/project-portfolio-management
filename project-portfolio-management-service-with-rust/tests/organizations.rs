//! Organizations (EX-3), end to end with real PASETO identities: an organization is a record; a commitment
//! names one; a caller who belongs to an organization sees only its commitments; and a director pack can be
//! addressed to an organization, which alone (besides the pack's editors) can read it.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test organizations -- --ignored`.
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
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const ACME_USER: &str = "3f2c6b7e-0000-4000-8000-0000000000b1";
const MANAGER: &str = "3f2c6b7e-0000-4000-8000-0000000000c1";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";
const OTHER_USER: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";

async fn sweep_call(req: &axum_test::TestServer, who: &(axum::http::HeaderName, axum::http::HeaderValue)) -> Value {
    req.post("/api/threshold-sweep").add_header(who.0.clone(), who.1.clone()).await.json::<Value>()
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test organizations -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn organizations_are_records_and_a_partner_sees_its_own() {
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
    let issuer = bearer(&kid, ISSUER_SUB, &[("access", "write"), ("directorpack", "issuer")]);
    let first = {
        use chrono::Datelike;
        chrono::Utc::now().date_naive().with_day(1).unwrap()
    };
    let month = first.format("%Y-%m").to_string();
    request::<App, _, _>(|request, _ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }

        // ── the records ──
        let mk = |name: &'static str, kind: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r = request.post("/api/organizations").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "name": name, "kind": kind })).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let (c, acme) = mk("Acme Council", "partner").await;
        assert_eq!(c, 200);
        let acme_pid = acme["pid"].as_str().unwrap().to_string();
        assert_eq!(acme["audience"], format!("org:{acme_pid}"));
        let (c, other) = mk("Other Trust", "partner").await;
        assert_eq!(c, 200);
        let other_pid = other["pid"].as_str().unwrap().to_string();
        assert_eq!(mk("acme council", "supplier").await.0, 409, "names are unique ignoring case");
        assert_eq!(mk("Nobody", "friend").await.0, 422);
        assert_eq!(mk("  ", "partner").await.0, 422);
        let renamed: Value = with!(request.put(&format!("/api/organizations/{other_pid}")), writer).json(&json!({ "name": "Other Health Trust", "kind": "sponsor" })).await.json();
        assert_eq!((renamed["name"].as_str(), renamed["kind"].as_str()), (Some("Other Health Trust"), Some("sponsor")));
        let listed: Value = with!(request.get("/api/organizations"), writer).await.json();
        assert_eq!(listed["organizations"].as_array().unwrap().len(), 2);

        // ── commitments name an organization ──
        let prog: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Program", "name": "Care" })).await.json();
        let prog = prog["pid"].as_str().unwrap().to_string();
        let pool: Value = with!(request.post("/api/skill-pools"), writer).json(&json!({ "name": "Engineering", "roles": ["Developer"] })).await.json();
        let pool = pool["pid"].as_str().unwrap().to_string();
        let commit = |body: Value| {
            let (request, writer) = (&request, writer.clone());
            async move { let r = request.put("/api/capacity-commitments").add_header(writer.0.clone(), writer.1.clone()).json(&body).await; (r.status_code(), r.json::<Value>()) }
        };
        let (c, a) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": acme_pid, "kind": "supply", "month": month, "committed_fte": 2 })).await;
        assert_eq!(c, 200);
        assert_eq!((a["organization"].as_str(), a["organization_pid"].as_str()), (Some("Acme Council"), Some(acme_pid.as_str())), "the label is the name");
        let (_, b) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization": "OTHER HEALTH TRUST", "kind": "supply", "month": month, "committed_fte": 1 })).await;
        assert_eq!(b["organization_pid"].as_str(), Some(other_pid.as_str()), "a label that matches a record is linked to it");
        let (_, own) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert!(own["organization_pid"].is_null());
        let (c, _) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": uuid::Uuid::new_v4(), "kind": "supply", "month": month, "committed_fte": 1 })).await;
        assert_eq!(c, 422, "an unknown organization");

        // ── a partner sees its own commitments, by the organization attribute ──
        let acme_user = bearer(&kid, ACME_USER, &[("access", "write"), ("organization", &acme_pid)]);
        let other_user = bearer(&kid, OTHER_USER, &[("access", "write"), ("organization", &other_pid)]);
        let n = |v: &Value| v["commitments"].as_array().unwrap().len();
        let all: Value = with!(request.get("/api/capacity-commitments"), writer).await.json();
        assert_eq!(n(&all), 3, "a caller of no organization sees all");
        let mine: Value = with!(request.get("/api/capacity-commitments"), acme_user).await.json();
        assert_eq!(n(&mine), 1);
        assert_eq!(mine["commitments"][0]["organization"], "Acme Council");
        let theirs: Value = with!(request.get("/api/capacity-commitments"), other_user).await.json();
        assert_eq!(n(&theirs), 1);
        assert_eq!(theirs["commitments"][0]["organization"], "Other Health Trust");

        // ── an organization is a pack audience ──
        let aud = format!("org:{acme_pid}");
        let bad = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": format!("org:{}", uuid::Uuid::new_v4()) })).await;
        assert_eq!(bad.status_code(), 422, "an organization that does not exist");
        let draft: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": aud })).await.json();
        let pk = draft["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/director-packs/{pk}/issue")), issuer).await.assert_status_ok();
        let seen: Value = with!(request.get("/api/director-packs"), acme_user).await.json();
        assert_eq!(seen["packs"].as_array().unwrap().len(), 1, "the organization's own pack");
        let not: Value = with!(request.get("/api/director-packs"), other_user).await.json();
        assert!(not["packs"].as_array().unwrap().is_empty(), "another organization sees none");
        with!(request.get(&format!("/api/director-packs/{pk}")), acme_user).await.assert_status_ok();
        assert_ne!(with!(request.get(&format!("/api/director-packs/{pk}")), other_user).await.status_code(), 200);

        // ── EX-4: a milestone is met only with its partners' sign-off ──
        let proj: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Project", "name": "Go-live" })).await.json();
        let proj = proj["pid"].as_str().unwrap().to_string();
        let ms: Value = with!(request.post(&format!("/api/plans/{proj}/milestones")), writer).json(&json!({ "name": "Cutover", "due": "2026-12-01" })).await.json();
        let mp = ms["pid"].as_str().unwrap().to_string();
        let base = format!("/api/plans/{proj}/milestones/{mp}");
        with!(request.put(&format!("{base}/signoff-required")), writer).json(&json!({ "organization_pids": [acme_pid, other_pid] })).await.assert_status_ok();
        assert_eq!(with!(request.put(&format!("{base}/signoff-required")), writer).json(&json!({ "organization_pids": [uuid::Uuid::new_v4()] })).await.status_code(), 422);
        let state = |v: &Value| v["milestones"].as_array().unwrap().iter().find(|m| m["name"] == "Cutover").cloned().unwrap();
        let sum: Value = with!(request.get("/api/milestone-signoffs"), writer).await.json();
        assert_eq!(state(&sum)["signoff"]["state"], "not_delivered");

        with!(request.post(&format!("{base}/complete")), writer).json(&json!({})).await.assert_status_success();
        let sum: Value = with!(request.get("/api/milestone-signoffs"), writer).await.json();
        let m = state(&sum);
        assert_eq!(m["signoff"]["state"], "delivered_not_met", "{sum}");
        assert!(m["signoff"]["met_on"].is_null() && !m["signoff"]["delivered_on"].is_null());
        assert_eq!(m["signoff"]["outstanding"].as_array().unwrap().len(), 2);
        assert_eq!((sum["totals"]["delivered"].as_u64(), sum["totals"]["delivered_not_met"].as_u64(), sum["totals"]["met"].as_u64()), (Some(1), Some(1), Some(0)));

        let sign = |who: (axum::http::HeaderName, axum::http::HeaderValue), org: &str, action: &str, reason: Option<&str>| {
            let (request, base) = (&request, base.clone());
            let body = json!({ "organization_pid": org, "action": action, "reason": reason });
            async move { request.post(&format!("{base}/signoffs")).add_header(who.0, who.1).json(&body).await.status_code() }
        };
        assert_eq!(sign(writer.clone(), &acme_pid, "sign", None).await, 403, "a person of no organization cannot sign for one");
        assert_eq!(sign(other_user.clone(), &acme_pid, "sign", None).await, 403, "nor for another's");
        assert_eq!(sign(acme_user.clone(), &acme_pid, "sign", None).await, 200);
        let sum: Value = with!(request.get("/api/milestone-signoffs"), writer).await.json();
        assert_eq!(state(&sum)["signoff"]["outstanding"], json!(["Other Health Trust"]));
        assert_eq!(sign(other_user.clone(), &other_pid, "sign", None).await, 200);
        let sum: Value = with!(request.get("/api/milestone-signoffs"), writer).await.json();
        assert_eq!(state(&sum)["signoff"]["state"], "met");
        assert!(!state(&sum)["signoff"]["met_on"].is_null());
        assert_eq!(sum["totals"]["met_share_of_delivered_bp"], 10000);
        let one: Value = with!(request.get(&format!("/api/milestones/{mp}")), writer).await.json();
        assert_eq!(one["signoff"]["state"], "met", "the milestone itself carries it");

        assert_eq!(sign(acme_user.clone(), &acme_pid, "withdraw", None).await, 422, "a withdrawal needs a reason");
        assert_eq!(sign(acme_user.clone(), &acme_pid, "withdraw", Some("Evidence was incomplete")).await, 200);
        let sum: Value = with!(request.get("/api/milestone-signoffs"), writer).await.json();
        assert_eq!(state(&sum)["signoff"]["state"], "delivered_not_met", "withdrawn, so outstanding again");
        assert_eq!(state(&sum)["signoff"]["outstanding"], json!(["Acme Council"]));
        {
            use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
            use project_portfolio_management_service::models::_entities::audit_logs;
            let n = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq("milestone_signoff_withdrawn")).count(&_ctx.db).await.unwrap();
            assert_eq!(n, 1, "the withdrawal is audited");
        }
        {
            use sea_orm::ConnectionTrait;
            assert!(_ctx.db.execute_unprepared("DELETE FROM milestone_signoffs").await.is_err(), "append-only");
            assert!(_ctx.db.execute_unprepared("UPDATE milestone_signoffs SET reason = 'x'").await.is_err());
        }
        // The pack reports the met share and the gap.
        let d2: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": "board" })).await.json();
        let pk2 = d2["pid"].as_str().unwrap().to_string();
        let pack: Value = with!(request.get(&format!("/api/director-packs/{pk2}")), issuer).await.json();
        assert_eq!(pack["body"]["milestone_signoffs"]["totals"]["delivered_not_met"], 1, "{pack}");

        // ── EX-2: the delivery model and its owner, shown in the portfolio map ──
        let dm = format!("/api/plans/{prog}/delivery-model");
        let none: Value = with!(request.get(&dm), writer).await.json();
        assert!(none["delivery_model"].is_null() && none["delivered_by_organization"].is_null() && none["reporting_method"] == "evm");
        assert_eq!(with!(request.put(&dm), writer).json(&json!({ "delivery_model": "whatever" })).await.status_code(), 422);
        assert_eq!(with!(request.put(&dm), writer).json(&json!({ "delivery_model": "central_product", "delivery_owner_organization_pid": uuid::Uuid::new_v4() })).await.status_code(), 422);
        assert_eq!(with!(request.put(&format!("/api/plans/{proj}/delivery-model")), writer).json(&json!({ "delivery_model": "central_product" })).await.status_code(), 422, "a project is not a programme");
        let set: Value = with!(request.put(&dm), writer).json(&json!({ "delivery_model": "local_programme_central_standards", "delivery_owner_organization_pid": other_pid })).await.json();
        assert_eq!((set["delivery_model"].as_str(), set["delivered_by_organization"].as_bool()), (Some("local_programme_central_standards"), Some(false)));
        assert_eq!(set["delivery_owner"]["name"], "Other Health Trust");
        let map: Value = with!(request.get("/api/portfolio-map"), writer).await.json();
        let care = map["groups"].as_array().unwrap().iter().flat_map(|g| g["programmes"].as_array().unwrap().clone()).find(|p| p["name"] == "Care").unwrap();
        assert_eq!(care["delivery"]["delivery_model"], "local_programme_central_standards", "{care}");
        assert_eq!(care["delivery"]["delivered_by_organization"], false);

        // ── EX-14: a product programme reports burn-up and flow, never a blank ──
        let rm = format!("/api/plans/{prog}/reporting-method");
        assert_eq!(with!(request.put(&rm), writer).json(&json!({ "method": "agile" })).await.status_code(), 422);
        assert_eq!(with!(request.put(&format!("/api/plans/{proj}/reporting-method")), writer).json(&json!({ "method": "product" })).await.status_code(), 422);
        let care_of = |v: &Value| v["programmes"].as_array().unwrap().iter().find(|p| p["name"] == "Care").cloned().unwrap();
        let evm: Value = with!(request.get("/api/portfolio-earned-value"), writer).await.json();
        assert_eq!(care_of(&evm)["state"], "no_baseline");
        assert_eq!(care_of(&evm)["method"], "evm");
        with!(request.put(&rm), writer).json(&json!({ "method": "product" })).await.assert_status_ok();
        let evm: Value = with!(request.get("/api/portfolio-earned-value"), writer).await.json();
        assert_eq!(care_of(&evm)["state"], "no_flow_data", "no tasks yet: said, not blank: {evm}");
        assert!(care_of(&evm)["reason"].is_string());
        for title in ["One", "Two"] {
            with!(request.post(&format!("/api/plans/{prog}/tasks")), writer).json(&json!({ "title": title })).await.assert_status_success();
        }
        let evm: Value = with!(request.get("/api/portfolio-earned-value"), writer).await.json();
        let flow = &care_of(&evm)["flow"];
        assert_eq!((care_of(&evm)["state"].as_str(), flow["scope"].as_u64(), flow["open"].as_u64()), (Some("product"), Some(2), Some(2)), "{evm}");
        assert_eq!(flow["burn_up"].as_array().unwrap().len(), 8);
        {
            use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
            use project_portfolio_management_service::models::_entities::audit_logs;
            with!(request.put(&rm), writer).json(&json!({ "method": "product" })).await.assert_status_ok();
            let n = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq("reporting_method_changed")).count(&_ctx.db).await.unwrap();
            assert_eq!(n, 1, "the change is audited once, and a repeat is not a change");
        }

        // ── EX-13: an external ask needs a named trade-off before approval ──
        let ask: Value = with!(request.post("/api/proposals"), writer).json(&json!({ "title": "Sponsor wants a new portal", "kind_target": "", "external_ask": true })).await.json();
        let ap = ask["pid"].as_str().unwrap().to_string();
        let plain: Value = with!(request.post("/api/proposals"), writer).json(&json!({ "title": "Ordinary idea", "kind_target": "" })).await.json();
        let pp = plain["pid"].as_str().unwrap().to_string();
        for p in [&ap, &pp] {
            with!(request.post(&format!("/api/proposals/{p}/submit")), writer).await.assert_status_ok();
            with!(request.post(&format!("/api/proposals/{p}/review")), writer).await.assert_status_ok();
        }
        assert_eq!(with!(request.post(&format!("/api/proposals/{ap}/approve")), writer).await.status_code(), 422, "an ask with no trade-off");
        assert_eq!(with!(request.put(&format!("/api/proposals/{pp}/tradeoff")), writer).json(&json!({ "tradeoff": "Nothing at all really" })).await.status_code(), 422, "not an external ask");
        assert_eq!(with!(request.put(&format!("/api/proposals/{ap}/tradeoff")), writer).json(&json!({ "tradeoff": "short" })).await.status_code(), 422);
        with!(request.put(&format!("/api/proposals/{ap}/tradeoff")), writer).json(&json!({ "tradeoff": "Displaces the records migration by one quarter" })).await.assert_status_ok();
        with!(request.post(&format!("/api/proposals/{ap}/approve")), writer).await.assert_status_ok();
        with!(request.post(&format!("/api/proposals/{pp}/approve")), writer).await.assert_status_ok();
        assert_eq!(with!(request.put(&format!("/api/proposals/{ap}/tradeoff")), writer).json(&json!({ "tradeoff": "Changed my mind about it" })).await.status_code(), 422, "decided");
        let d3: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": "executive" })).await.json();
        let pack3: Value = with!(request.get(&format!("/api/director-packs/{}", d3["pid"].as_str().unwrap())), issuer).await.json();
        let asks = &pack3["body"]["external_asks"];
        assert_eq!(asks["asks"].as_array().unwrap().len(), 1, "{asks}");
        assert_eq!(asks["asks"][0]["tradeoff"], "Displaces the records migration by one quarter");
        assert_eq!(asks["without_named_tradeoff"], 0);

        // ── EX-1: register completeness, and "no register, no money" ──
        let reg: Value = with!(request.get("/api/register-completeness"), writer).await.json();
        let care = reg["programmes"].as_array().unwrap().iter().find(|p| p["name"] == "Care").cloned().unwrap();
        assert_eq!(care["complete"], false);
        let missing: Vec<&str> = care["missing"].as_array().unwrap().iter().map(|m| m.as_str().unwrap()).collect();
        for want in ["sro", "stage", "baseline", "target_date", "budget", "confidence", "top_risks", "dependencies"] {
            assert!(missing.contains(&want), "{want} should be named missing: {care}");
        }
        assert!(!missing.contains(&"partners"), "it has partner commitments on record (and a delivery model with partners)");
        assert!(care["not_applicable"].as_array().unwrap().iter().any(|f| f == "funding_source"), "no budget line, so no funding source is asked");
        assert_eq!(reg["enforced"], false);
        let alloc = |booking: &str| {
            let (request, writer, prog) = (&request, writer.clone(), prog.clone());
            let body = json!({ "person_ref": format!("person:{}", uuid::Uuid::new_v4()), "percent": 50, "booking": booking });
            async move { request.post(&format!("/api/plans/{prog}/allocations")).add_header(writer.0, writer.1).json(&body).await }
        };
        assert_eq!(alloc("committed").await.status_code(), 200, "switch off: the action goes ahead");
        {
            use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
            use project_portfolio_management_service::models::_entities::audit_logs;
            let n = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq("register_incomplete_warning")).count(&_ctx.db).await.unwrap();
            assert_eq!(n, 1, "and the warning is audited");
        }
        // SAFETY: this binary is single-test, so nothing else reads the setting concurrently.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED", "on") };
        let refused = alloc("committed").await;
        assert_eq!(refused.status_code(), 422, "switch on: refused");
        let text = refused.text();
        assert!(text.contains("register_incomplete") && text.contains("sro") && text.contains("baseline"), "naming the fields: {text}");
        assert_eq!(alloc("proposed").await.status_code(), 200, "a proposed booking is not money");
        let reg: Value = with!(request.get("/api/register-completeness"), writer).await.json();
        assert_eq!(reg["enforced"], true);
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED") };
        let d4: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": "escalation_board" })).await.json();
        let pack4: Value = with!(request.get(&format!("/api/director-packs/{}", d4["pid"].as_str().unwrap())), issuer).await.json();
        let rc = &pack4["body"]["data_quality"]["register_completeness"];
        assert!(rc["incomplete"].as_array().unwrap().iter().any(|p| p["name"] == "Care"), "the data-quality panel lists it: {rc}");

        // ── EX-9: watermelon measures, from stored packs only ──
        let conf = |v: &str| {
            let (request, writer, prog) = (&request, writer.clone(), prog.clone());
            let body = json!({ "value": v, "kind": "self", "rationale": "As the team sees it this week" });
            async move { request.put(&format!("/api/plans/{prog}/delivery-confidence")).add_header(writer.0, writer.1).json(&body).await.assert_status_ok(); }
        };
        let earlier = (first - chrono::Months::new(1)).format("%Y-%m").to_string();
        conf("green").await;
        let e: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": earlier, "audience": "partner" })).await.json();
        let ep = e["pid"].as_str().unwrap().to_string();
        let first_pack: Value = with!(request.get(&format!("/api/director-packs/{ep}")), issuer).await.json();
        assert!(first_pack["body"]["watermelon"]["late_discovery"].is_null(), "no earlier issued pack: null, not zero");
        assert!(first_pack["body"]["watermelon"]["reasons_null"]["late_discovery"].is_string());
        with!(request.post(&format!("/api/director-packs/{ep}/issue")), issuer).await.assert_status_ok();
        conf("red").await;
        let n: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": "partner" })).await.json();
        let np = n["pid"].as_str().unwrap().to_string();
        let now_pack: Value = with!(request.get(&format!("/api/director-packs/{np}")), issuer).await.json();
        let w = &now_pack["body"]["watermelon"];
        assert_eq!(w["late_discovery"]["programmes"], json!(["Care"]), "green last pack, red now: {w}");
        assert_eq!(w["red_without_decision"]["programmes"], json!(["Care"]), "red, and no decision recorded");
        assert!(w["self_vs_independent"].is_null(), "no independent assessment anywhere");

        // ── EX-11: the key-person register is for resource managers; the pack holds counts ──
        let manager = bearer(&kid, MANAGER, &[("access", "write"), ("role", "resource_manager")]);
        assert_eq!(with!(request.get("/api/key-persons"), writer).await.status_code(), 404, "a non-manager sees no register");
        assert_eq!(with!(request.put("/api/key-persons"), writer).json(&json!({ "subject_kind": "role", "subject": "Release manager", "readiness": "none" })).await.status_code(), 404);
        assert_eq!(with!(request.put("/api/key-persons"), manager).json(&json!({ "subject_kind": "role", "subject": "x", "readiness": "maybe" })).await.status_code(), 422);
        assert_eq!(with!(request.put("/api/key-persons"), manager).json(&json!({ "subject_kind": "person", "subject": "Ada", "readiness": "none" })).await.status_code(), 422, "a person is a reference, not a name");
        let (ada, bob) = ("person:3f2c6b7e-0000-4000-8000-00000000f0f1", "person:3f2c6b7e-0000-4000-8000-00000000f0f2");
        with!(request.put("/api/key-persons"), manager).json(&json!({ "subject_kind": "person", "subject": ada, "readiness": "none", "skill_pool_pid": pool, "note": "Only one who knows the interface" })).await.assert_status_ok();
        with!(request.put("/api/key-persons"), manager).json(&json!({ "subject_kind": "role", "subject": "Release manager", "readiness": "ready", "successor_ref": bob })).await.assert_status_ok();
        with!(request.put("/api/key-persons"), manager).json(&json!({ "subject_kind": "role", "subject": "release MANAGER", "readiness": "partly", "successor_ref": bob })).await.assert_status_ok();
        let reg: Value = with!(request.get("/api/key-persons"), manager).await.json();
        assert_eq!(reg["entries"].as_array().unwrap().len(), 2, "one entry per subject, ignoring case: {reg}");
        for who in [ada, bob] {
            _ctx.db.execute_unprepared(&format!("INSERT INTO resource_roles (pid, person_ref, role, created_at, updated_at) VALUES (gen_random_uuid(), '{who}', 'Developer', now(), now())")).await.unwrap();
        }
        with!(request.put("/api/key-persons/contingent"), manager).json(&json!({ "person_ref": bob, "contingent": true })).await.assert_status_ok();
        let d6: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": earlier, "audience": "board" })).await.json();
        let pk6 = d6["pid"].as_str().unwrap().to_string();
        let pack6: Value = with!(request.get(&format!("/api/director-packs/{pk6}")), issuer).await.json();
        let kp = &pack6["body"]["key_persons"];
        assert_eq!((kp["single_points"].as_u64(), kp["people"].as_u64(), kp["roles"].as_u64(), kp["without_successor"].as_u64()), (Some(2), Some(1), Some(1), Some(1)), "{kp}");
        assert_eq!(kp["readiness"]["partly"], 1);
        let eng = kp["contingent_share_by_pool"].as_array().unwrap().iter().find(|p| p["pool"] == "Engineering").unwrap();
        assert_eq!((eng["headcount"].as_u64(), eng["contingent"].as_u64(), eng["contingent_share_bp"].as_u64()), (Some(2), Some(1), Some(5000)));
        let text = pack6.to_string();
        assert!(!text.contains(ada) && !text.contains(bob) && !text.contains("Only one who knows") && !text.contains("Release manager"), "the pack names no one and no role");
        with!(request.delete(&format!("/api/key-persons/{}", reg["entries"][0]["pid"].as_str().unwrap())), manager).await.assert_status_ok();

        // ── EX-7: triage, thresholds, and decisions that nothing makes for you ──
        let tri = format!("/api/plans/{prog}/triage");
        assert_eq!(with!(request.put(&tri), writer).json(&json!({ "outcome": "maybe", "reason": "Because it is late" })).await.status_code(), 422);
        assert_eq!(with!(request.put(&tri), writer).json(&json!({ "outcome": "fix", "reason": "short" })).await.status_code(), 422);
        assert_eq!(with!(request.put(&format!("/api/plans/{proj}/triage")), writer).json(&json!({ "outcome": "fix", "reason": "A project is not a programme" })).await.status_code(), 422);
        with!(request.put(&tri), writer).json(&json!({ "outcome": "fix", "reason": "Supplier delays can be recovered" })).await.assert_status_ok();
        with!(request.put(&tri), writer).json(&json!({ "outcome": "pause", "reason": "Safety case not yet ready" })).await.assert_status_ok();
        let t: Value = with!(request.get(&tri), writer).await.json();
        assert_eq!((t["triage"]["outcome"].as_str(), t["history"].as_array().unwrap().len()), (Some("pause"), 2));
        assert!(_ctx.db.execute_unprepared("UPDATE programme_triages SET outcome = 'finish'").await.is_err(), "append-only");

        with!(request.post(&format!("/api/director-packs/{np}/issue")), issuer).await.assert_status_ok();
        let th = format!("/api/plans/{prog}/thresholds");
        for bad in [json!({ "kind": "confidence_red_packs", "count": 0 }), json!({ "kind": "forecast_past" }), json!({ "kind": "p80_cap", "cap_minor": 5 }), json!({ "kind": "mood" })] {
            assert_eq!(with!(request.put(&th), writer).json(&bad).await.status_code(), 422, "{bad}");
        }
        with!(request.post(&format!("/api/plans/{prog}/budget-lines")), writer).json(&json!({ "category": "capex", "description": "Build", "currency": "GBP", "planned_minor": 100_000 })).await.assert_status_success();
        with!(request.post(&format!("/api/plans/{prog}/tasks")), writer).json(&json!({ "title": "Dated", "start_date": "2026-10-05", "duration_days": 3 })).await.assert_status_success();
        // At the line: two packs in a row is not met by one red pack and one green; a forecast on the date is not past it.
        with!(request.put(&th), writer).json(&json!({ "kind": "confidence_red_packs", "count": 2 })).await.assert_status_ok();
        with!(request.put(&th), writer).json(&json!({ "kind": "forecast_past", "date": "2999-01-01" })).await.assert_status_ok();
        let r1 = sweep_call(&request, &writer).await;
        assert!(r1["results"].as_array().unwrap().iter().all(|r| r["crossed"] == false), "nothing is crossed yet: {r1}");
        with!(request.put(&th), writer).json(&json!({ "kind": "confidence_red_packs", "count": 1 })).await.assert_status_ok();
        with!(request.put(&th), writer).json(&json!({ "kind": "p80_cap", "cap_minor": 1, "currency": "GBP" })).await.assert_status_ok();
        let r2 = sweep_call(&request, &writer).await;
        let by_kind = |r: &Value, k: &str| r["results"].as_array().unwrap().iter().find(|x| x["kind"] == k).cloned().unwrap();
        assert_eq!(by_kind(&r2, "confidence_red_packs")["outcome"], "attached_to_open_exception", "Care already has an open confidence exception (red): the crossing joins it: {r2}");
        assert_eq!(by_kind(&r2, "p80_cap")["outcome"], "already_open", "one open exception per programme");
        let ex_pid = by_kind(&r2, "confidence_red_packs")["exception_pid"].as_str().unwrap().to_string();
        // Nothing was changed by crossing: the triage and the programme are as they were.
        assert_eq!(with!(request.post(&format!("/api/exceptions/{ex_pid}/decision")), writer).json(&json!({ "decision": "Carry on regardless of it" })).await.status_code(), 422, "the ordinary decision refuses a threshold exception");
        for bad in [json!({ "action": "override", "reason": "The sponsor accepts the risk" }), json!({ "action": "override", "reason": "The sponsor accepts the risk", "expiry": "2020-01-01" }), json!({ "action": "stop", "reason": "short" }), json!({ "action": "pause", "reason": "Pausing for the safety case", "expiry": "2999-01-01" })] {
            assert_eq!(with!(request.post(&format!("/api/exceptions/{ex_pid}/threshold-decision")), writer).json(&bad).await.status_code(), 422, "{bad}");
        }
        let soon = (chrono::Utc::now().date_naive() + chrono::Duration::days(30)).to_string();
        with!(request.post(&format!("/api/exceptions/{ex_pid}/threshold-decision")), writer).json(&json!({ "action": "override", "reason": "The sponsor accepts the risk", "expiry": soon })).await.assert_status_ok();
        assert_eq!(with!(request.post(&format!("/api/exceptions/{ex_pid}/threshold-decision")), writer).json(&json!({ "action": "stop", "reason": "Decided twice by mistake" })).await.status_code(), 409);
        let r3 = sweep_call(&request, &writer).await;
        assert_eq!(by_kind(&r3, "confidence_red_packs")["outcome"], "suppressed_by_override", "an override holds until it expires");
        assert_eq!(by_kind(&r3, "p80_cap")["outcome"], "opened", "the other threshold now has room to open");
        let ex2 = by_kind(&r3, "p80_cap")["exception_pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/exceptions/{ex2}/threshold-decision")), writer).json(&json!({ "action": "reset", "reason": "Re-plan the build with a lower ceiling" })).await.assert_status_ok();
        let d7: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": earlier, "audience": "executive" })).await.json();
        let pack7: Value = with!(request.get(&format!("/api/director-packs/{}", d7["pid"].as_str().unwrap())), issuer).await.json();
        let tt = &pack7["body"]["triage_and_thresholds"];
        // Decisions are counted in the month they were made, so the earlier month's pack counts none of today's.
        assert_eq!((tt["triage_counts"]["pause"].as_u64(), tt["decisions_in_period"]["override"].as_u64(), tt["decisions_in_period"]["reset"].as_u64()), (Some(1), Some(0), Some(0)), "{tt}");
        {
            use sea_orm::{EntityTrait, PaginatorTrait};
            use project_portfolio_management_service::models::_entities::threshold_decisions;
            assert_eq!(threshold_decisions::Entity::find().count(&_ctx.db).await.unwrap(), 2, "an override and a reset were recorded");
        }
        assert_eq!(tt["overrides_in_force"][0]["programme"], "Care");
        assert!(!tt.to_string().contains("person:"), "no one is named: {tt}");
        assert!(_ctx.db.execute_unprepared("DELETE FROM threshold_decisions").await.is_err(), "append-only");

        // ── EX-8: start control: a limit on programmes in active delivery, a queue, an audited override ──
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let mkprog = |name: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0, writer.1).json(&json!({ "kind": "Program", "name": name })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let phase = |pid: &str, to: &str, extra: Value| {
            let (request, writer) = (&request, writer.clone());
            let mut body = json!({ "phase": to });
            for (k, v) in extra.as_object().cloned().unwrap_or_default() { body[k] = v; }
            let url = format!("/api/plans/{pid}/phase");
            async move { let r = request.put(&url).add_header(writer.0, writer.1).json(&body).await; (r.status_code().as_u16(), r.text()) }
        };
        let (alpha, beta, gamma, delta) = (mkprog("Alpha").await, mkprog("Beta").await, mkprog("Gamma").await, mkprog("Delta").await);
        for p in [&alpha, &beta, &gamma, &delta] {
            assert_eq!(phase(p, "initiating", json!({})).await.0, 200);
            assert_eq!(phase(p, "planning", json!({})).await.0, 200);
        }
        // No limit set: anyone may start.
        // SAFETY: this binary is single-test, so nothing else reads the setting concurrently.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT", "1") };
        assert_eq!(phase(&alpha, "executing", json!({})).await.0, 200, "room for the first");
        let (code, text) = phase(&beta, "executing", json!({})).await;
        assert_eq!(code, 422, "{text}");
        assert!(text.contains("start_refused") && text.contains("1 of 1 places") && text.contains("number 1 in the queue"), "{text}");
        assert_eq!(phase(&delta, "executing", json!({})).await.0, 422);
        let q: Value = with!(request.get("/api/start-queue"), writer).await.json();
        let names: Vec<&str> = q["queue"].as_array().unwrap().iter().filter(|e| e["name"] == "Beta" || e["name"] == "Delta").map(|e| e["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["Beta", "Delta"], "in the order they asked: {q}");
        let chk: Value = with!(request.get(&format!("/api/plans/{beta}/start-check")), writer).await.json();
        assert_eq!((chk["would_be_refused"].as_bool(), chk["wip"]["limit"].as_u64(), chk["wip"]["in_delivery"].as_u64()), (Some(true), Some(1), Some(1)), "{chk}");
        // Asking again does not change the place; leaving does.
        assert_eq!(phase(&beta, "executing", json!({})).await.0, 422);
        let q2: Value = with!(request.get("/api/start-queue"), writer).await.json();
        assert_eq!(q2["queue"].as_array().unwrap().iter().filter(|e| e["name"] == "Beta").count(), 1, "one place each");
        with!(request.post(&format!("/api/plans/{delta}/start-queue/withdraw")), writer).await.assert_status_ok();
        // A refusal can be overridden, with a reason, and the override is recorded.
        assert_eq!(phase(&beta, "executing", json!({ "start_override_reason": "short" })).await.0, 422, "a thin reason is no override");
        assert_eq!(phase(&beta, "executing", json!({ "start_override_reason": "The sponsor needs it live before the audit" })).await.0, 200);
        {
            use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
            use project_portfolio_management_service::models::_entities::audit_logs;
            let n = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq("start_override")).count(&_ctx.db).await.unwrap();
            assert_eq!(n, 1, "the override is audited");
        }
        let q3: Value = with!(request.get("/api/start-queue"), writer).await.json();
        assert!(q3["queue"].as_array().unwrap().iter().all(|e| e["name"] != "Beta" && e["name"] != "Delta"), "started and withdrawn are out of the queue: {q3}");
        assert!(_ctx.db.execute_unprepared("DELETE FROM start_overrides").await.is_err(), "overrides are append-only");
        // Paused programmes use no place: pause both, and a third starts without an override.
        for p in [&alpha, &beta] {
            with!(request.post(&format!("/api/plans/{p}/programme-status")), admin).json(&json!({ "status": "paused", "reason": "Paused while the audit runs" })).await.assert_status_ok();
        }
        assert_eq!(phase(&gamma, "executing", json!({})).await.0, 200, "paused programmes do not count");
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT") };

        // ── EX-5: a deployment's go-live date is under change control ──
        let platform: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Program", "name": "Platform" })).await.json();
        let platform = platform["pid"].as_str().unwrap().to_string();
        let mkdep = |name: &'static str| {
            let (request, writer, platform) = (&request, writer.clone(), platform.clone());
            async move {
                let v: Value = request.post("/api/plans").add_header(writer.0, writer.1).json(&json!({ "kind": "Project", "name": name, "parent_ref": platform })).await.json();
                v["pid"].as_str().unwrap().to_string()
            }
        };
        let (d1, d2) = (mkdep("Deployment one").await, mkdep("Deployment two").await);
        let mktask = |plan: &str, title: &'static str, start: Option<&'static str>, days: i64| {
            let (request, writer, plan) = (&request, writer.clone(), plan.to_string());
            async move {
                let mut body = json!({ "title": title, "duration_days": days });
                if let Some(s) = start { body["start_date"] = json!(s); }
                let t: Value = request.post(&format!("/api/plans/{plan}/tasks")).add_header(writer.0, writer.1).json(&body).await.json();
                t["pid"].as_str().unwrap().to_string()
            }
        };
        let g1 = mktask(&d1, "Go-live one", Some("2026-10-05"), 3).await;
        let h1 = mktask(&d2, "Go-live two", None, 4).await;
        let other = mktask(&d2, "Unrelated", Some("2026-10-05"), 2).await;
        with!(request.post("/api/dependencies"), writer).json(&json!({ "predecessor_pid": g1, "successor_pid": h1 })).await.assert_status_ok();
        let starts = |plan: String| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let v: Value = request.get(&format!("/api/plans/{plan}/cpm")).add_header(writer.0, writer.1).await.json();
                v["nodes"].as_array().unwrap().iter().map(|n| (n["pid"].as_str().unwrap().to_string(), n["early_start"].as_str().unwrap_or("").to_string())).collect::<std::collections::BTreeMap<_, _>>()
            }
        };
        let before = starts(d2.clone()).await;
        let url = format!("/api/plans/{d1}/deployment-moves");
        let acme_ref = format!("person:{ACME_USER}");
        let ask = json!({ "task_pid": g1, "new_start": "2026-10-12", "reason": "Local supplier needs another week", "approver_ref": acme_ref });
        assert_eq!(with!(request.post(&url), writer).json(&ask).await.status_code(), 422, "not yet a controlled programme");
        with!(request.put(&format!("/api/plans/{platform}/delivery-model")), writer).json(&json!({ "delivery_model": "central_platform_local_deployment" })).await.assert_status_ok();
        let own = format!("person:{WRITER}");
        assert_eq!(with!(request.post(&url), writer).json(&json!({ "task_pid": g1, "new_start": "2026-10-12", "reason": "Local supplier needs another week", "approver_ref": own })).await.status_code(), 422, "the approver cannot be the requester");
        assert_eq!(with!(request.post(&url), writer).json(&json!({ "task_pid": g1, "new_start": "2026-10-12", "reason": "Local supplier needs another week" })).await.status_code(), 422, "an approver must be named");
        let req: Value = with!(request.post(&url), writer).json(&ask).await.json();
        let mv = req["pid"].as_str().unwrap().to_string();
        assert_eq!(req["status"], "requested", "{req}");
        let ko = req["knock_on"].as_array().unwrap();
        let knocked: std::collections::BTreeSet<String> = ko.iter().map(|k| k["task_pid"].as_str().unwrap().to_string()).collect();
        assert!(knocked.contains(&h1) && !knocked.contains(&other), "the dependent go-live moves and the unrelated task does not: {req}");
        assert_eq!(ko.iter().find(|k| k["task_pid"] == h1.as_str()).unwrap()["other_deployment"], true);
        // An unapproved move changes nothing.
        assert_eq!(starts(d2.clone()).await, before, "the schedule is as it was");
        let t1: Value = with!(request.get(&format!("/api/plans/{d1}/tasks/{g1}")), writer).await.json();
        assert_eq!(t1["start_date"], "2026-10-05");
        assert_eq!(with!(request.post(&url), writer).json(&ask).await.status_code(), 409, "one open move per go-live");
        // Only the named approver decides, and not the requester.
        let decide = format!("{url}/{mv}/decision");
        let body = json!({ "decision": "approve", "reason": "Agreed with the supplier and the board" });
        for (who, name) in [(&writer, "the requester"), (&other_user, "another writer"), (&admin, "an administrator")] {
            assert_eq!(with!(request.post(&decide), who).json(&body).await.status_code(), 403, "{name}");
        }
        assert_eq!(with!(request.post(&decide), acme_user).json(&json!({ "decision": "approve", "reason": "ok" })).await.status_code(), 422, "a thin reason");
        let done: Value = with!(request.post(&decide), acme_user).json(&body).await.json();
        assert_eq!(done["status"], "approved");
        // The knock-on list equals what the network now shows.
        let after = starts(d2.clone()).await;
        let changed: std::collections::BTreeSet<String> = after.iter().filter(|(k, v)| **k != g1 && before.get(*k) != Some(*v)).map(|(k, _)| k.clone()).collect();
        let listed: std::collections::BTreeSet<String> = knocked.iter().filter(|k| after.contains_key(*k)).cloned().collect();
        assert_eq!(changed, listed, "the listed knock-on is exactly what moved in the other deployment");
        let t1: Value = with!(request.get(&format!("/api/plans/{d1}/tasks/{g1}")), writer).await.json();
        assert_eq!(t1["start_date"], "2026-10-12", "approved, so the go-live moved");
        assert_eq!(with!(request.post(&decide), acme_user).json(&body).await.status_code(), 409, "decided once");
        // A refusal changes nothing and is audited.
        let ask2 = json!({ "task_pid": g1, "new_start": "2026-10-26", "reason": "Another delay was proposed", "approver_ref": "role:escalation_board" });
        let req2: Value = with!(request.post(&url), writer).json(&ask2).await.json();
        let board = bearer(&kid, "3f2c6b7e-0000-4000-8000-0000000000b9", &[("access", "write"), ("role", "escalation_board")]);
        let refused: Value = with!(request.post(&format!("{url}/{}/decision", req2["pid"].as_str().unwrap())), board).json(&json!({ "decision": "refuse", "reason": "The date would break the audit window" })).await.json();
        assert_eq!(refused["status"], "refused");
        let t1: Value = with!(request.get(&format!("/api/plans/{d1}/tasks/{g1}")), writer).await.json();
        assert_eq!(t1["start_date"], "2026-10-12", "refused: unchanged");
        {
            use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
            use project_portfolio_management_service::models::_entities::audit_logs;
            for (action, want) in [("deployment_move_requested", 2), ("deployment_move_approved", 1), ("deployment_move_refused", 1)] {
                let n = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq(action)).count(&_ctx.db).await.unwrap();
                assert_eq!(n, want, "{action}");
            }
        }
        let l: Value = with!(request.get(&url), writer).await.json();
        assert_eq!(l["moves"].as_array().unwrap().len(), 2);
        assert_eq!(l["cost_by_requester"][0]["requests"], 2, "recorded against the requester");

        // ── EX-6: a cutover go/no-go with published criteria and co-signatures ──
        let lead = bearer(&kid, "3f2c6b7e-0000-4000-8000-0000000000c2", &[("access", "write"), ("cutover_role", "accountable_professional_lead")]);
        let delivery = bearer(&kid, "3f2c6b7e-0000-4000-8000-0000000000c3", &[("access", "write"), ("cutover_role", "delivery_organization")]);
        let cut = format!("/api/plans/{proj}/cutover-decisions");
        assert_eq!(with!(request.post(&cut), writer).json(&json!({ "title": "Go-live cutover", "criteria": [] })).await.status_code(), 422);
        let c1: Value = with!(request.post(&cut), writer).json(&json!({ "title": "Go-live cutover", "criteria": ["Rollback rehearsed end to end", "Clinical safety case signed"] })).await.json();
        let c1p = c1["pid"].as_str().unwrap().to_string();
        let one = format!("{cut}/{c1p}");
        assert_eq!(c1["status"], "draft");
        with!(request.put(&format!("{one}/criteria")), writer).json(&json!({ "criteria": ["Rollback rehearsed end to end", "Clinical safety case signed", "Help desk staffed for the first week"] })).await.assert_status_ok();
        assert_eq!(with!(request.put(&format!("{one}/criteria/1/mark")), writer).json(&json!({ "met": true, "evidence": "Rehearsal record" })).await.status_code(), 409, "not open yet");
        with!(request.post(&format!("{one}/publish")), writer).await.assert_status_ok();
        // Frozen: neither the service nor the database lets the criteria change now.
        assert_eq!(with!(request.put(&format!("{one}/criteria")), writer).json(&json!({ "criteria": ["Something easier to meet"] })).await.status_code(), 409);
        for (what, sql) in [
            ("editing a criterion", "UPDATE cutover_criteria SET criterion = 'Something easier to meet' WHERE decision_pid IN (SELECT pid FROM cutover_decisions WHERE status <> 'draft')"),
            ("removing one", "DELETE FROM cutover_criteria WHERE decision_pid IN (SELECT pid FROM cutover_decisions WHERE status <> 'draft')"),
        ] {
            assert!(_ctx.db.execute_unprepared(sql).await.is_err(), "{what}");
        }
        assert!(_ctx.db.execute_unprepared(&format!("INSERT INTO cutover_criteria (pid, decision_pid, position, criterion, created_at, updated_at) VALUES (gen_random_uuid(), '{c1p}', 9, 'A late extra criterion', now(), now())")).await.is_err(), "adding one");
        with!(request.post(&format!("{one}/open")), writer).await.assert_status_ok();
        // Go is refused until every criterion is met and every role has signed.
        let go = json!({ "outcome": "go", "reason": "All criteria met and signed" });
        let refused = with!(request.post(&format!("{one}/decide")), writer).json(&go).await;
        assert_eq!(refused.status_code(), 422);
        let t = refused.text();
        assert!(t.contains("criteria not yet marked: 1, 2, 3") && t.contains("co-signatures missing"), "{t}");
        for n in 1..=3 {
            with!(request.put(&format!("{one}/criteria/{n}/mark")), writer).json(&json!({ "met": true, "evidence": "Evidence on file" })).await.assert_status_ok();
        }
        assert_eq!(with!(request.post(&format!("{one}/cosign")), writer).json(&json!({ "role": "accountable_professional_lead" })).await.status_code(), 403, "without the role");
        assert_eq!(with!(request.post(&format!("{one}/cosign")), lead).json(&json!({ "role": "supplier" })).await.status_code(), 422, "not a required role");
        with!(request.post(&format!("{one}/cosign")), lead).json(&json!({ "role": "accountable_professional_lead" })).await.assert_status_ok();
        assert_eq!(with!(request.post(&format!("{one}/cosign")), lead).json(&json!({ "role": "accountable_professional_lead" })).await.status_code(), 409, "once");
        let t = with!(request.post(&format!("{one}/decide")), writer).json(&go).await.text();
        assert!(t.contains("co-signatures missing: delivery_organization"), "{t}");
        with!(request.post(&format!("{one}/cosign")), delivery).json(&json!({ "role": "delivery_organization" })).await.assert_status_ok();
        let decided: Value = with!(request.post(&format!("{one}/decide")), writer).json(&go).await.json();
        assert_eq!((decided["status"].as_str(), decided["outcome"].as_str()), (Some("decided"), Some("go")));
        assert!(_ctx.db.execute_unprepared("UPDATE cutover_decisions SET outcome = 'no_go' WHERE status = 'decided'").await.is_err(), "a decided decision is final");
        assert_eq!(with!(request.post(&format!("{one}/new-version")), writer).await.status_code(), 409, "a go is final");

        // A delay for safety needs no met criteria, is a decision, and a new version can follow it.
        let c2: Value = with!(request.post(&cut), writer).json(&json!({ "title": "Second site cutover", "criteria": ["Clinical safety case signed"] })).await.json();
        let two = format!("{cut}/{}", c2["pid"].as_str().unwrap());
        with!(request.post(&format!("{two}/publish")), writer).await.assert_status_ok();
        with!(request.post(&format!("{two}/open")), writer).await.assert_status_ok();
        with!(request.put(&format!("{two}/criteria/1/mark")), writer).json(&json!({ "met": false, "evidence": "Safety case not yet signed" })).await.assert_status_ok();
        let delayed: Value = with!(request.post(&format!("{two}/decide")), writer).json(&json!({ "outcome": "delay_for_safety", "reason": "Safety case unsigned; hold the cutover" })).await.json();
        assert_eq!(delayed["outcome"], "delay_for_safety");
        let v2: Value = with!(request.post(&format!("{two}/new-version")), writer).await.json();
        assert_eq!((v2["version"].as_i64(), v2["status"].as_str(), v2["supersedes_pid"].as_str()), (Some(2), Some("draft"), c2["pid"].as_str()));
        let d8: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": format!("org:{other_pid}") })).await.json();
        let pack8: Value = with!(request.get(&format!("/api/director-packs/{}", d8["pid"].as_str().unwrap())), issuer).await.json();
        let cd = &pack8["body"]["cutover_decisions"];
        assert_eq!((cd["counts"]["go"].as_u64(), cd["counts"]["delay_for_safety"].as_u64(), cd["counts"]["no_go"].as_u64()), (Some(1), Some(1), Some(0)), "{cd}");
        assert!(cd["note"].as_str().unwrap().contains("not counted as slippage"));

        // ── EX-10: the outcome scorecard and the de-escalation tracker ──
        let today = chrono::Utc::now().date_naive();
        let lf = project_portfolio_management_service::deescalation::last_finished(today);
        let lf1 = lf.previous();
        let day = |q: project_portfolio_management_service::deescalation::Quarter| (q.start() + chrono::Duration::days(20)).to_string();
        let tpl: Value = with!(request.get("/api/scorecard-template"), writer).await.json();
        assert!(tpl["measures"].as_array().unwrap().iter().all(|m| m.get("target_value").is_none()), "the template carries no targets");
        let sc: Value = with!(request.post("/api/scorecards"), writer).json(&json!({ "name": "Recovery outcomes", "for_deescalation": true })).await.json();
        let scp = sc["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.post("/api/scorecards"), writer).json(&json!({ "name": "Another", "for_deescalation": true })).await.status_code(), 409, "only one is the tracker's");
        assert_eq!(with!(request.post(&format!("/api/scorecards/{scp}/from-template")), writer).json(&json!({ "measures": [{ "key": "milestones_met_on_time", "target_value": 50, "red_value": 80 }] })).await.status_code(), 422, "red must be on the worse side");
        assert_eq!(with!(request.post(&format!("/api/scorecards/{scp}/from-template")), writer).json(&json!({ "measures": [{ "key": "mood", "target_value": 1, "red_value": 0 }] })).await.status_code(), 422);
        let made: Value = with!(request.post(&format!("/api/scorecards/{scp}/from-template")), writer).json(&json!({ "measures": [
            { "key": "milestones_met_on_time", "target_value": 80, "red_value": 50 },
            { "key": "open_critical_issues", "target_value": 2, "red_value": 10 } ] })).await.json();
        let kp: Vec<String> = made["kpi_pids"].as_array().unwrap().iter().map(|k| k.as_str().unwrap().to_string()).collect();
        let reading = |kpi: &str, q, v: i64| {
            let (request, writer, kpi, d) = (&request, writer.clone(), kpi.to_string(), day(q));
            async move { request.post(&format!("/api/kpis/{kpi}/readings")).add_header(writer.0, writer.1).json(&json!({ "observed_on": d, "value": v })).await.assert_status_success(); }
        };
        let crit: Value = with!(request.post("/api/deescalation/criteria"), writer).json(&json!({ "criterion": "Independent assurance reports no new red programmes" })).await.json();
        let cp = crit["pid"].as_str().unwrap().to_string();
        let evid = |q: &str, met: bool| {
            let (request, writer, cp, q) = (&request, writer.clone(), cp.clone(), q.to_string());
            async move { request.put(&format!("/api/deescalation/criteria/{cp}/evidence")).add_header(writer.0, writer.1).json(&json!({ "quarter": q, "met": met, "link": "https://example.org/assurance/q", "note": "Assurance report on file" })).await.status_code() }
        };
        // Last quarter fully recorded and met; the one before has nothing: the run is one, and says why.
        reading(&kp[0], lf, 90).await;
        reading(&kp[1], lf, 1).await;
        assert_eq!(evid(&lf.label(), true).await, 200);
        assert_eq!(evid(&{ let n = project_portfolio_management_service::deescalation::Quarter::of(today); n.label() }, true).await, 422, "an unfinished quarter");
        let t: Value = with!(request.get("/api/deescalation"), writer).await.json();
        assert_eq!(t["consecutive_quarters_met"], 1, "{t}");
        assert_eq!(t["rule_satisfied"], false);
        assert_eq!(t["required_quarters"], 2);
        let prior = t["by_quarter"].as_array().unwrap().iter().find(|q| q["quarter"] == lf1.label().as_str()).unwrap();
        assert_eq!(prior["state"]["state"], "missing");
        assert!(prior["state"]["because"].to_string().contains("nothing recorded"), "a missing quarter says why: {prior}");
        // Fill in the quarter before: two in a row.
        reading(&kp[0], lf1, 85).await;
        reading(&kp[1], lf1, 2).await;
        assert_eq!(evid(&lf1.label(), true).await, 200);
        let t: Value = with!(request.get("/api/deescalation"), writer).await.json();
        assert_eq!((t["consecutive_quarters_met"].as_u64(), t["rule_satisfied"].as_bool()), (Some(2), Some(true)), "{t}");
        // A reading that misses the target breaks the run.
        reading(&kp[0], lf, 40).await;
        let t: Value = with!(request.get("/api/deescalation"), writer).await.json();
        assert_eq!(t["consecutive_quarters_met"], 0, "the latest reading in the quarter is below target: {t}");
        // The level is the sponsor's to set, and nothing computes it.
        assert_eq!(with!(request.put("/api/deescalation/level"), other_user).json(&json!({ "level": "Level 2" })).await.status_code(), 403);
        let sponsor = bearer(&kid, "3f2c6b7e-0000-4000-8000-0000000000d5", &[("access", "write"), ("role", "sponsor")]);
        with!(request.put("/api/deescalation/level"), sponsor).json(&json!({ "level": "Level 2", "note": "Set at the board" })).await.assert_status_ok();
        let t: Value = with!(request.get("/api/deescalation"), writer).await.json();
        assert_eq!(t["level"]["level"], "Level 2", "the level stays what the sponsor set, whatever the run: {t}");
        assert!(_ctx.db.execute_unprepared("UPDATE escalation_levels SET level = 'Level 0'").await.is_err(), "append-only");
        let one: Value = with!(request.get(&format!("/api/scorecards/{scp}?quarters=2")), writer).await.json();
        assert_eq!(one["kpis"].as_array().unwrap().len(), 2);
        let d10: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": (first - chrono::Months::new(3)).format("%Y-%m").to_string(), "audience": "executive" })).await.json();
        let pack10: Value = with!(request.get(&format!("/api/director-packs/{}", d10["pid"].as_str().unwrap())), issuer).await.json();
        assert_eq!(pack10["body"]["outcomes"]["level"]["level"], "Level 2");
        assert!(pack10["body"]["outcomes"]["criteria"][0]["quarters"][0]["link"].is_null(), "the pack carries no evidence link");

        // ── retired: it can no longer be named or addressed ──
        with!(request.put(&format!("/api/organizations/{acme_pid}")), writer).json(&json!({ "active": false })).await.assert_status_ok();
        let (c, _) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": acme_pid, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert_eq!(c, 422);
    })
    .await;
}
