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
const OTHER_USER: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";

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

        // ── retired: it can no longer be named or addressed ──
        with!(request.put(&format!("/api/organizations/{acme_pid}")), writer).json(&json!({ "active": false })).await.assert_status_ok();
        let (c, _) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": acme_pid, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert_eq!(c, 422);
    })
    .await;
}
