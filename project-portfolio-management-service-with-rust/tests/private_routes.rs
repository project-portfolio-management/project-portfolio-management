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

/// Security audit: a body `plan_pid` was only checked for existence, the lists and switches looked at nothing, and a rule
/// with no plan runs on private plans its author cannot see.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn a_private_plans_automations_are_private() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Automated").await;
        let rule = |plan: Option<&str>| {
            json!({ "plan_pid": plan, "name": "Rule", "trigger_kind": "task_moved",
                    "actions": [{ "kind": "add_label", "value": { "label": "seen" } }] })
        };
        let refused = with!(request.post("/api/automations"), who.outsider)
            .json(&rule(Some(&plan)))
            .await;
        assert_eq!(refused.status_code(), 404, "no rule on a hidden plan");
        let everywhere = rule(None);
        let refused = with!(request.post("/api/automations"), who.member)
            .json(&everywhere)
            .await;
        assert_eq!(
            refused.status_code(),
            403,
            "every plan includes private ones"
        );
        let wide: Value = with!(request.post("/api/automations"), who.admin)
            .json(&everywhere)
            .await
            .json();
        let wide = wide["pid"].as_str().expect("rule on every plan");
        let own: Value = with!(request.post("/api/automations"), who.member)
            .json(&rule(Some(&plan)))
            .await
            .json();
        let own = own["pid"].as_str().expect("rule");

        let listed =
            |v: Value, pid: &str| v.as_array().expect("rows").iter().any(|r| r["pid"] == pid);
        let seen: Value = with!(request.get("/api/automations"), who.outsider)
            .await
            .json();
        assert!(!listed(seen.clone(), own));
        assert!(
            listed(seen, wide),
            "a rule on every plan is no plan's secret"
        );
        let seen: Value = with!(request.get("/api/automations"), who.member)
            .await
            .json();
        assert!(listed(seen, own));

        for action in ["enable", "disable"] {
            let url = format!("/api/automations/{own}/{action}");
            let refused = with!(request.post(&url), who.outsider)
                .json(&json!({}))
                .await;
            assert_eq!(refused.status_code(), 404);
            let url = format!("/api/automations/{wide}/{action}");
            let refused = with!(request.post(&url), who.member).json(&json!({})).await;
            assert_eq!(refused.status_code(), 403);
        }
        assert_eq!(
            with!(
                request.delete(&format!("/api/automations/{own}")),
                who.outsider
            )
            .await
            .status_code(),
            404
        );

        // Runs follow the rule, and the subject of a rule on every plan.
        let task: Value = with!(
            request.post(&format!("/api/plans/{plan}/tasks")),
            who.member
        )
        .json(&json!({ "title": "Moved" }))
        .await
        .json();
        let task = task["pid"].as_str().expect("task");
        with!(
            request.patch(&format!("/api/plans/{plan}/tasks/{task}")),
            who.member
        )
        .json(&json!({ "status": "in_progress" }))
        .await
        .assert_status_ok();
        let member: Value = with!(request.get("/api/automations/runs"), who.member)
            .await
            .json();
        assert!(
            member.as_array().is_some_and(|r| !r.is_empty()),
            "the rules ran"
        );
        let seen = with!(request.get("/api/automations/runs"), who.outsider).await;
        assert_eq!(seen.header("x-total-count"), "0");

        // Scheduled actions.
        let schedule = json!({ "subject_kind": "plan", "subject_pid": plan,
                               "action_kind": "expire_review", "in_days": 5 });
        let refused = with!(request.post("/api/scheduled-actions"), who.outsider)
            .json(&schedule)
            .await;
        assert_eq!(refused.status_code(), 404);
        let action: Value = with!(request.post("/api/scheduled-actions"), who.member)
            .json(&schedule)
            .await
            .json();
        let action = action["pid"].as_str().expect("action");
        let seen: Value = with!(request.get("/api/scheduled-actions"), who.outsider)
            .await
            .json();
        assert!(!listed(seen, action));
        let seen: Value = with!(request.get("/api/scheduled-actions"), who.member)
            .await
            .json();
        assert!(listed(seen, action));
        assert_eq!(
            with!(
                request.delete(&format!("/api/scheduled-actions/{action}")),
                who.outsider
            )
            .await
            .status_code(),
            404
        );
    })
    .await;
}

/// Security audit: a field of the body naming a plan was checked for existence at most, so it reached a plan the guard on
/// `/api/plans/{pid}` would have refused.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn a_field_of_the_body_is_no_way_into_a_private_plan() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Fenced").await;
        let kpi = json!({ "name": "Uptime", "unit": "percent", "direction": "higher_better",
                          "target_value": 99, "amber_value": 95, "red_value": 90, "plan_pid": plan });
        let framework = json!({ "name": "Scrum", "methodology": "scrum", "cadence_days": 14,
                                "wip_limit": 5, "definition_of_done": "Done", "ceremonies": [],
                                "plan_pid": plan });
        let deploy = json!({ "plan_pid": plan, "kind": "deploy", "environment": "production" });
        let child = json!({ "kind": "Project", "name": "Child", "parent_ref": plan });
        for (url, body) in [
            ("/api/kpis", &kpi),
            ("/api/ways-of-working", &framework),
            ("/api/devops/events", &deploy),
            ("/api/plans", &child),
        ] {
            let refused = with!(request.post(url), who.outsider).json(body).await;
            assert_eq!(refused.status_code(), 404, "{url}");
        }
        for (url, body) in [("/api/devops/events", &deploy), ("/api/plans", &child)] {
            with!(request.post(url), who.member).json(body).await.assert_status_ok();
        }

        // A parent on an existing plan, by update.
        let mine: Value = with!(request.post("/api/plans"), who.outsider)
            .json(&json!({ "kind": "Project", "name": "Mine" }))
            .await
            .json();
        let mine = mine["pid"].as_str().expect("plan");
        let moved = with!(request.put(&format!("/api/plans/{mine}")), who.outsider)
            .json(&child)
            .await;
        assert_eq!(moved.status_code(), 404);

        // A batch import names the plan per project: refused as if it did not exist, and nothing written.
        let xml = "<Project xmlns=\"http://schemas.microsoft.com/project\"><Name>X</Name><Tasks>\
                   <Task><UID>1</UID><Name>Planted</Name><OutlineLevel>1</OutlineLevel></Task></Tasks></Project>";
        let batch = |pid: &str| json!({ "projects": [{ "xml": xml, "plan_pid": pid }] });
        let url = "/api/import/mspdi-batch?dry_run=false";
        let hidden: Value = with!(request.post(url), who.outsider).json(&batch(&plan)).await.json();
        let absent = uuid::Uuid::new_v4().to_string();
        let missing: Value = with!(request.post(url), who.outsider).json(&batch(&absent)).await.json();
        assert_eq!(hidden.to_string().replace(&plan, ""), missing.to_string().replace(&absent, ""));

        // A plan import: a row keyed to a hidden plan gets the error of a pid that names nothing.
        let row = json!({ "pid": plan, "kind": "Project", "name": "Overwritten" }).to_string();
        let job = with!(request.post("/api/plans/import?format=jsonl&on_keyless=create"), who.outsider)
            .content_type("text/plain")
            .bytes(row.into_bytes().into())
            .await;
        let job: Value = job.json();
        let job: Value = with!(
            request.get(&format!("/api/plans/import/{}", job["job_id"].as_str().expect("job"))),
            who.outsider
        )
        .await
        .json();
        assert_eq!(job["counts"]["failed"], 1, "{job}");

        // Merging into or out of one.
        let merge = json!({ "main_pid": mine, "duplicate_pid": plan });
        let refused = with!(request.post("/api/plans/merge"), who.outsider).json(&merge).await;
        assert_eq!(refused.status_code(), 404);

        let seen: Value = with!(request.get(&format!("/api/plans/{plan}")), who.member).await.json();
        assert_eq!(seen["name"], "Fenced", "still whole");
        let tasks: Value = with!(request.get(&format!("/api/plans/{plan}/tasks")), who.member).await.json();
        assert_eq!(tasks["tasks"], json!([]), "nothing planted");
    })
    .await;
}

/// Security audit: sprint ceremonies and commitments sit outside the guard and looked at nothing, and a calendar feed kept
/// listing a private plan after its owner lost access to it.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn sprints_and_calendars_keep_a_private_plan_out() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Sprinted").await;
        let sprint: Value = with!(
            request.post(&format!("/api/plans/{plan}/sprints")),
            who.member
        )
        .json(&json!({ "name": "One", "starts_on": "2026-11-02", "ends_on": "2026-11-13" }))
        .await
        .json();
        let sprint = sprint["pid"].as_str().expect("sprint");
        let ceremony = json!({ "kind": "planning" });
        let ceremonies = format!("/api/sprints/{sprint}/ceremonies");
        let commitment = format!("/api/sprints/{sprint}/commitment");
        let commit = format!("/api/sprints/{sprint}/commit");
        for url in [&ceremonies, &commitment] {
            assert_eq!(
                with!(request.get(url), who.outsider).await.status_code(),
                404
            );
        }
        for (url, body) in [(&ceremonies, &ceremony), (&commit, &json!({}))] {
            let refused = with!(request.post(url), who.outsider).json(body).await;
            assert_eq!(refused.status_code(), 404);
        }
        with!(request.post(&ceremonies), who.member)
            .json(&ceremony)
            .await
            .assert_status_ok();
        with!(request.post(&commit), who.member)
            .json(&json!({}))
            .await
            .assert_status_ok();
        with!(request.get(&ceremonies), who.member)
            .await
            .assert_status_ok();

        // The outsider was assigned a task, was taken off the list, and still holds a feed URL.
        let task = json!({ "title": "Due soon", "due_date": "2026-12-01",
                           "assignee_ref": format!("person:{OUTSIDER}") });
        with!(
            request.post(&format!("/api/plans/{plan}/tasks")),
            who.member
        )
        .json(&task)
        .await
        .assert_status_ok();
        let feed: Value = with!(request.post("/api/me/feed-token"), who.outsider)
            .json(&json!({}))
            .await
            .json();
        let url = feed["calendar_url"].as_str().expect("feed url");
        let text = request.get(url).await.text();
        assert!(
            !text.contains("Sprinted") && !text.contains("Due soon"),
            "{text}"
        );

        // A member's own feed still has it.
        let feed: Value = with!(request.post("/api/me/feed-token"), who.member)
            .json(&json!({}))
            .await
            .json();
        let task = json!({ "title": "Mine too", "due_date": "2026-12-02",
                           "assignee_ref": format!("person:{MEMBER}") });
        with!(
            request.post(&format!("/api/plans/{plan}/tasks")),
            who.member
        )
        .json(&task)
        .await
        .assert_status_ok();
        let text = request
            .get(feed["calendar_url"].as_str().expect("feed url"))
            .await
            .text();
        assert!(text.contains("Mine too"), "{text}");
    })
    .await;
}

/// Security audit: a snapshot of a private plan, tasks and all, was readable and copyable by everyone.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_routes -- --ignored`"]
async fn a_template_is_hidden_with_its_plan() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    let who = callers(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let plan = private_plan(&request, &who, "Templated").await;
        with!(
            request.post(&format!("/api/plans/{plan}/tasks")),
            who.member
        )
        .json(&json!({ "title": "A hidden step" }))
        .await
        .assert_status_ok();
        let saved: Value = with!(
            request.post(&format!("/api/plans/{plan}/save-as-template")),
            who.member
        )
        .json(&json!({ "name": "Hidden template" }))
        .await
        .json();
        let template = saved["pid"].as_str().expect("template");

        let listed = |v: Value| {
            v.as_array()
                .expect("rows")
                .iter()
                .any(|t| t["pid"] == template)
        };
        let list = "/api/plan-templates";
        assert!(!listed(with!(request.get(list), who.outsider).await.json()));
        assert!(listed(with!(request.get(list), who.member).await.json()));

        let one = format!("/api/plan-templates/{template}");
        assert_eq!(
            with!(request.get(&one), who.outsider).await.status_code(),
            404
        );
        with!(request.get(&one), who.member)
            .await
            .assert_status_ok();
        let copy = with!(
            request.post(&format!("/api/plans/from-template/{template}")),
            who.outsider
        )
        .json(&json!({ "name": "Copy" }))
        .await;
        assert_eq!(copy.status_code(), 404);
        let delete = with!(
            request.post(&format!("/api/plan-templates/{template}/delete")),
            who.outsider
        )
        .json(&json!({}))
        .await;
        assert_eq!(delete.status_code(), 404);
        for url in [format!("{one}/revise"), format!("{one}/plans")] {
            let response = if url.ends_with("plans") {
                with!(request.get(&url), who.outsider).await
            } else {
                with!(request.post(&url), who.outsider)
                    .json(&json!({ "from_plan": plan }))
                    .await
            };
            assert_eq!(response.status_code(), 404, "{url}");
        }
    })
    .await;
}
