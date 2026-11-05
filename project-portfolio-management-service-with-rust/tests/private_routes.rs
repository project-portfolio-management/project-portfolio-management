//! Routes outside the plan guard, with real identities (`privacy_mw` guards only `/api/plans/{uuid}...`): whoever cannot read
//! a private plan gets nothing from a route that reaches that plan's records by their own pid or by a field in the body, and
//! a person's inbox belongs to that person.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! `cargo test --features paseto --test private_routes -- --ignored --test-threads=1`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use serial_test::serial;
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [83; 32];
const ISSUER: &str = "authentication-service";
const AUDIENCE: &str = "main-x-service";
const MEMBER: &str = "3f2c6b7e-0000-4000-8000-0000000002a1";
const OUTSIDER: &str = "3f2c6b7e-0000-4000-8000-0000000002a2";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000002ad";
const EXPERT: &str = "3f2c6b7e-0000-4000-8000-0000000002a3";

type Header = (axum::http::HeaderName, axum::http::HeaderValue);

fn keys_and_kid() -> (Value, String) {
    let public = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
    let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public));
    let keys = json!({
        "keys": [{ "kty": "OKP", "crv": "Ed25519", "use": "sig",
                   "kid": kid, "x": URL_SAFE_NO_PAD.encode(public) }]
    });
    (keys, kid)
}

fn bearer(kid: &str, sub: &str, attrs: &[(&str, &str)]) -> Header {
    let iat: i64 = 1_700_000_000;
    let attrs = attrs.iter().fold(serde_json::Map::new(), |mut m, (k, v)| {
        m.insert((*k).to_string(), json!([*v]));
        m
    });
    let payload = json!({
        "sub": sub, "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE, "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": attrs,
    })
    .to_string();
    let key = Key::<64>::from(SigningKey::from_bytes(&SEED).to_keypair_bytes());
    let private = PasetoAsymmetricPrivateKey::<V4, Public>::from(&key);
    let footer = format!(r#"{{"kid":"{kid}"}}"#);
    let mut builder = Paseto::<V4, Public>::builder();
    builder.set_payload(Payload::from(payload.as_str()));
    builder.set_footer(Footer::from(footer.as_str()));
    let token = builder.try_sign(&private).expect("sign");
    (
        axum::http::header::AUTHORIZATION,
        format!("Bearer {token}").parse().unwrap(),
    )
}

/// Turn on real identities. The tests run serially, so nothing races these settings.
fn set_up(keys: &Value) {
    // SAFETY: the tests here are `#[serial]`, and each writes the same values.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"],
                                "when": { "access": ["write", "admin"] } }] })
            .to_string(),
        );
    }
}

/// The three callers every test here compares.
struct Callers {
    member: Header,
    outsider: Header,
    admin: Header,
}

fn callers(kid: &str) -> Callers {
    Callers {
        member: bearer(kid, MEMBER, &[("access", "write")]),
        outsider: bearer(kid, OUTSIDER, &[("access", "write")]),
        admin: bearer(kid, ADMIN, &[("access", "admin")]),
    }
}

macro_rules! with {
    ($req:expr, $who:expr) => {
        $req.add_header($who.0.clone(), $who.1.clone())
    };
}

/// A plan of `name` that only `MEMBER` (and an administrator) can read.
async fn private_plan(request: &axum_test::TestServer, who: &Callers, name: &str) -> String {
    let plan: Value = with!(request.post("/api/plans"), who.member)
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    let plan = plan["pid"].as_str().expect("plan").to_string();
    with!(
        request.put(&format!("/api/plans/{plan}/access")),
        who.member
    )
    .json(&json!({ "private": true, "members": [MEMBER] }))
    .await
    .assert_status_ok();
    plan
}

/// Security audit: `recipient` was a free query parameter, so anyone signed in read anyone's inbox and marked it read.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn an_inbox_belongs_to_its_owner() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = with!(request.post("/api/plans"), who.member)
            .json(&json!({ "kind": "Project", "name": "Inbox" }))
            .await
            .json();
        let plan = plan["pid"].as_str().expect("plan");
        let task: Value = with!(
            request.post(&format!("/api/plans/{plan}/tasks")),
            who.member
        )
        .json(&json!({ "title": "Ask" }))
        .await
        .json();
        let task = task["pid"].as_str().expect("task");
        let mine = format!("person:{MEMBER}");
        with!(
            request.post(&format!("/api/plans/{plan}/tasks/{task}/assign")),
            who.member
        )
        .json(&json!({ "assignee_ref": mine }))
        .await
        .assert_status_ok();

        let url = format!("/api/notifications?recipient={mine}");
        let own = with!(request.get(&url), who.member).await;
        own.assert_status_ok();
        let own = own.json::<Value>();
        let note = own[0]["pid"].as_str().expect("the owner's notification");

        for other in [&who.outsider, &who.admin] {
            assert_eq!(with!(request.get(&url), other).await.status_code(), 403);
            let read = with!(
                request.post(&format!("/api/notifications/{note}/read")),
                other
            )
            .json(&json!({}))
            .await;
            assert_eq!(read.status_code(), 404, "as if it did not exist");
        }
        let still: Value = with!(request.get(&url), who.member).await.json();
        assert!(still[0]["read_at"].is_null(), "nobody else marked it read");

        // Upper case names the same person.
        let shouted = format!("person:{}", MEMBER.to_uppercase());
        with!(
            request.get(&format!("/api/notifications?recipient={shouted}")),
            who.member
        )
        .await
        .assert_status_ok();

        with!(
            request.post(&format!("/api/notifications/{note}/read")),
            who.member
        )
        .json(&json!({}))
        .await
        .assert_status_ok();
    })
    .await;
}

/// Security audit: a proposal's duplicate check named private plans, an objective's alignment listed them, and a proposal
/// that became a private plan was readable by pid and in the promoted list.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn proposals_and_objectives_keep_a_private_plan_out() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, ctx| async move {
        let name = "Quarterly secret replatform of the payments estate";
        let plan = private_plan(&request, &who, name).await;

        // A title is not a way of finding a plan.
        let proposal: Value = with!(request.post("/api/proposals"), who.outsider)
            .json(&json!({ "title": name, "summary": "Same", "kind_target": "projects", "sponsor_ref": format!("organization:{}", uuid::Uuid::new_v4()), "requested_minor": 1000, "currency": "GBP" }))
            .await
            .json();
        let proposal = proposal["pid"].as_str().expect("proposal");
        let url = format!("/api/proposals/{proposal}/duplicates");
        let hits = |v: Value| {
            v.as_array()
                .expect("hits")
                .iter()
                .filter(|h| h["source"] == "plan")
                .count()
        };
        assert_eq!(hits(with!(request.get(&url), who.outsider).await.json()), 0);
        assert_eq!(hits(with!(request.get(&url), who.member).await.json()), 1);
        assert_eq!(hits(with!(request.get(&url), who.admin).await.json()), 1);

        // Alignment: left out of the items and of every total.
        let objective: Value = with!(request.post("/api/objectives"), who.member)
            .json(&json!({ "title": "Aligned", "period": "2026-H2" }))
            .await
            .json();
        let objective = objective["pid"].as_str().expect("objective");
        with!(request.post(&format!("/api/plans/{plan}/objectives")), who.member)
            .json(&json!({ "objective_pid": objective, "weight": 3 }))
            .await
            .assert_status_ok();
        let url = format!("/api/objectives/{objective}/alignment");
        let seen: Value = with!(request.get(&url), who.outsider).await.json();
        assert_eq!(seen["items"], json!([]));
        assert_eq!(seen["total_weight"], 0);
        assert_eq!(seen["weight_by_collection"], json!({}));
        let seen: Value = with!(request.get(&url), who.member).await.json();
        assert_eq!(seen["items"][0]["name"], name);
        assert_eq!(seen["total_weight"], 3);

        // A proposal that became the private plan is the plan's.
        ctx.db
            .execute_unprepared(&format!(
                "UPDATE proposals SET status = 'promoted', promoted_plan_pid = '{plan}'
                 WHERE pid = '{proposal}'"
            ))
            .await
            .expect("promote");
        let url = format!("/api/proposals/{proposal}");
        assert_eq!(with!(request.get(&url), who.outsider).await.status_code(), 404);
        with!(request.get(&url), who.member).await.assert_status_ok();
        let url = format!("/api/proposals/{proposal}/duplicates");
        assert_eq!(with!(request.get(&url), who.outsider).await.status_code(), 404);
        let listed = |v: Value| {
            v.as_array()
                .expect("rows")
                .iter()
                .any(|r| r["pid"] == proposal)
        };
        let url = "/api/proposals?status=promoted";
        assert!(!listed(with!(request.get(url), who.outsider).await.json()));
        assert!(listed(with!(request.get(url), who.member).await.json()));
    })
    .await;
}

/// Security audit: nothing under `/api/canvases` looked at the plan, and an item's discussion passed as belonging to no plan.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn a_private_plans_canvas_is_private() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Canvassed").await;
        let body = json!({ "template": "swot", "title": "Hidden canvas", "plan_pid": plan });
        assert_eq!(
            with!(request.post("/api/canvases"), who.outsider)
                .json(&body)
                .await
                .status_code(),
            404,
            "a body field is no way onto a hidden plan"
        );
        let canvas: Value = with!(request.post("/api/canvases"), who.member)
            .json(&body)
            .await
            .json();
        let canvas = canvas["pid"].as_str().expect("canvas");
        let item_url = format!("/api/canvases/{canvas}/items");
        let item_body = json!({ "box": "strengths", "body": "A hidden strength" });
        let item: Value = with!(request.post(&item_url), who.member)
            .json(&item_body)
            .await
            .json();
        let item = item["pid"].as_str().expect("item");

        let out = &who.outsider;
        let root = format!("/api/canvases/{canvas}");
        let one = format!("{item_url}/{item}");
        let promote = format!("{one}/promote");
        assert_eq!(with!(request.get(&root), out).await.status_code(), 404);
        assert_eq!(
            with!(request.delete(&root), who.outsider)
                .await
                .status_code(),
            404
        );
        for (req, body) in [
            (request.post(&item_url), item_body.clone()),
            (request.put(&one), json!({ "body": "edited" })),
            (
                request.post(&promote),
                json!({ "target": "risk", "probability": 2, "impact": 2 }),
            ),
        ] {
            assert_eq!(with!(req, out).json(&body).await.status_code(), 404);
        }
        assert_eq!(
            with!(request.delete(&one), who.outsider)
                .await
                .status_code(),
            404
        );

        // Its discussion goes with it.
        let comment = json!({ "subject_kind": "canvas_item", "subject_pid": item, "body": "psst" });
        assert_eq!(
            with!(request.post("/api/comments"), who.outsider)
                .json(&comment)
                .await
                .status_code(),
            404
        );
        with!(request.post("/api/comments"), who.member)
            .json(&comment)
            .await
            .assert_status_ok();
        let thread = format!("/api/comments?subject_kind=canvas_item&subject_pid={item}");
        assert_eq!(
            with!(request.get(&thread), who.outsider)
                .await
                .status_code(),
            404
        );

        let listed = |v: Value| {
            v.as_array()
                .expect("rows")
                .iter()
                .any(|c| c["pid"] == canvas)
        };
        for url in [
            "/api/canvases".to_string(),
            format!("/api/canvases?plan={plan}"),
        ] {
            assert!(!listed(with!(request.get(&url), who.outsider).await.json()));
            assert!(listed(with!(request.get(&url), who.member).await.json()));
            assert!(listed(with!(request.get(&url), who.admin).await.json()));
        }

        // Still whole for a member.
        let whole: Value = with!(request.get(&root), who.member).await.json();
        assert_eq!(whole["items"][0]["body"], "A hidden strength");
        with!(request.post(&promote), who.member)
            .json(&json!({ "target": "risk", "probability": 2, "impact": 2 }))
            .await
            .assert_status_ok();
    })
    .await;
}

/// Security audit: nothing under `/api/reviews` looked at the plan. The invited expert keeps their way in: being asked is the
/// disclosure decision.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn a_private_plans_reviews_are_private() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    let expert = bearer(&kid, EXPERT, &[("access", "write")]);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Reviewed").await;
        let invite = json!({ "subject_kind": "plan", "subject_pid": plan,
                             "reviewer_ref": format!("person:{EXPERT}"), "reviewer_scope": "external" });
        let refused = with!(request.post("/api/reviews"), who.outsider)
            .json(&invite)
            .await;
        assert_eq!(refused.status_code(), 422, "as for a plan that does not exist");
        let review: Value = with!(request.post("/api/reviews"), who.member)
            .json(&invite)
            .await
            .json();
        let review = review["pid"].as_str().expect("review");

        let listed = |v: Value| v.as_array().expect("rows").iter().any(|r| r["pid"] == review);
        let list = format!("/api/reviews?subject_pid={plan}");
        let page = with!(request.get("/api/reviews"), who.outsider).await;
        assert_eq!(page.header("x-total-count"), "0", "the total does not count it");
        assert!(!listed(page.json()));
        assert!(!listed(with!(request.get(&list), who.outsider).await.json()));
        assert!(listed(with!(request.get(&list), who.member).await.json()));
        assert!(listed(with!(request.get(&list), expert).await.json()));

        let consensus = format!("/api/reviews/consensus?subject_kind=plan&subject_pid={plan}");
        assert_eq!(with!(request.get(&consensus), who.outsider).await.status_code(), 404);
        with!(request.get(&consensus), who.member).await.assert_status_ok();

        let respond = format!("/api/reviews/{review}/respond");
        let submit = format!("/api/reviews/{review}/submit");
        let verdict = json!({ "recommendation": "advance", "score": 80 });
        let accept = json!({ "response": "accept" });
        let refused = with!(request.post(&respond), who.outsider).json(&accept).await;
        assert_eq!(refused.status_code(), 404);
        let refused = with!(request.post(&submit), who.outsider).json(&verdict).await;
        assert_eq!(refused.status_code(), 404);
        assert_eq!(
            with!(request.delete(&format!("/api/reviews/{review}")), who.outsider)
                .await
                .status_code(),
            404
        );

        // The expert is not on the access list, and answers all the same.
        with!(request.post(&respond), expert).json(&accept).await.assert_status_ok();
        with!(request.post(&submit), expert).json(&verdict).await.assert_status_ok();
    })
    .await;
}
