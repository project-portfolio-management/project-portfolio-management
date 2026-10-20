//! RM-8b: a private plan stays out of **every aggregate view**, end to end with real
//! PASETO identities. The method is differential: capture every aggregate route as a
//! non-member with only the public plan in the world; add a private plan carrying
//! data of every kind; capture again. For a non-member the two must be identical —
//! as if the private plan did not exist. A member, and an admin, must see the full
//! totals (and the same ones as each other).
//!
//! A test also enumerates the mounted GET routes, so a new aggregate cannot be added
//! without someone deciding whether it filters.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test private_aggregates -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [61; 32];
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
fn mint(kid: &str, sub: &str, access: &[&str], roles: &[&str]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": { "access": access, "role": roles },
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

type Who = (axum::http::HeaderName, axum::http::HeaderValue);

fn hdr(kid: &str, sub: &str, access: &[&str]) -> Who {
    auth_header(&mint(kid, sub, access, &[]))
}

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";
const PERSON: &str = "person:00000000-0000-4000-8000-0000000000c1";

/// Aggregate routes: a GET that counts, lists, totals or ranks across plans. Each is
/// captured as a non-member before and after a private plan appears, and must not change.
/// `{pub}` / `{secret}` are filled with the plans' pids.
const AGGREGATES: &[&str] = &[
    "/api/at-a-glance",
    "/api/capacity",
    "/api/capacity/utilization?by=plan",
    "/api/capacity/utilization?by=person",
    "/api/cost-risk",
    "/api/executive/alignment",
    "/api/executive/benefits",
    "/api/executive/decisions",
    "/api/executive/health",
    "/api/financials/exposure",
    "/api/financials/variance",
    "/api/issues",
    "/api/change-requests",
    "/api/director-pack/issues-and-changes",
    "/api/exceptions",
    "/api/portfolio-map",
    "/api/portfolio-money",
    "/api/buffer-fever",
    "/api/portfolio-earned-value",
    "/api/portfolio-benefits",
    "/api/capacity-heatmap",
    "/api/portfolio-drum",
    "/api/portfolio-pipeline",
    "/api/milestone-signoffs",
    "/api/start-queue",
    "/api/deescalation",
    "/api/funding-allocations",
    "/api/cost-codes",
    "/api/register-completeness",
    "/api/capacity-commitments",
    "/api/financials/forecast?plan={pub}",
    "/api/financials/forecast?plan={hub}",
    "/api/plans/{hub}/flow-distribution?rollup=true",
    "/api/plans/{hub}/schedule",
    "/api/plans?parent={hub}",
    "/api/kpis",
    "/api/lifecycle",
    "/api/milestones",
    "/api/okrs",
    "/api/objectives",
    "/api/prioritization",
    "/api/proposals",
    "/api/proposals/forecast",
    "/api/reports",
    "/api/resource-plan",
    "/api/resource-plan/hours",
    "/api/risk/heatmap",
    "/api/security/register",
    "/api/compliance/register",
    "/api/technology/debt",
    "/api/technology/flow",
    "/api/technology/radar",
    "/api/technology/dependency-risk",
    "/api/time-reports?group_by=plan",
    "/api/time-reports?group_by=person",
    "/api/time-reports?group_by=month",
    "/api/tpc",
    "/api/work-profile",
    "/api/compliance/findings",
    "/api/controls/coverage",
    "/api/auditor/findings",
    "/api/regulator/extract",
    "/api/engineering/blocked",
    "/api/engineering/delivery-links",
    "/api/engineering/milestone-calendar",
    "/api/engineering/moscow",
    "/api/devops/metrics",
    "/api/devops/releases",
    "/api/assignees/workload",
    "/api/plans",
    "/api/ways-of-working",
    "/api/governance-frameworks",
    "/api/plans/search?q=Secret",
    "/api/plans/search?q=Public",
    "/api/plans/audit/recent",
    "/api/plans/events/recent",
    "/api/plans/merges/recent",
    "/api/dependencies",
    "/api/ideas",
    "/api/reviews",
    "/api/approvals",
    "/api/plans/bulk-jobs",
    "/api/plans/review-queue",
    "/api/documents",
];

/// Reads that need an admin (or a machine peer) and are full by design: a non-admin, member or not,
/// gets 403, and an admin sees everything.
const PRIVILEGED: &[&str] = &[
    "/api/board/investments",
    "/api/board/pack",
    "/api/board/pack.pdf",
    "/api/board/trends",
    "/api/auditor/trail",
    "/api/auditor/evidence-pack",
    // The access review is administrators only (the `access_review` area), and still leaves out
    // plans the caller cannot read.
    "/api/access-review",
    // What named people work on: administrators and resource managers only. It filters out tasks in plans
    // the caller cannot read the same way `/api/me/tasks` does, but this census only checks the 403/200
    // split for it; the private-plan filter itself has no test of its own yet.
    "/api/resource-plan/role-tasks?role=Engineer",
];

/// Reads where a member and an admin legitimately differ because a person's detail is masked for a
/// caller who may not manage people (the plan filter is still checked in the leak comparison).
const MASKED_FOR_MEMBER: &[&str] = &[
    "/api/resource-plan",
    "/api/resource-plan/hours",
    "/api/capacity",
    "/api/assignees/workload",
];

/// GET routes without a path parameter that are **not** cross-plan aggregates, each with the reason.
/// Every mounted such route must be here or in one of the lists above, so a new aggregate cannot
/// be added without someone deciding whether it filters private plans.
const NOT_AGGREGATE: &[(&str, &str)] = &[
    (
        "/api/scorecards",
        "the scorecard names only; the KPIs are read, filtered by the plans the caller may read, from /api/scorecards/{pid} and /api/deescalation",
    ),
    (
        "/api/scorecard-template",
        "a fixed list of suggested measures built into the service; no record",
    ),
    (
        "/api/email-deliveries",
        "per-recipient email deliveries: administrators only (404 for anyone else), no plan, no amount",
    ),
    (
        "/api/key-persons",
        "the key-person register: resource managers only (404 for anyone else), holds no plan, and the pack carries counts only (tests/organizations.rs)",
    ),
    (
        "/api/organizations",
        "organization records (a name, a kind and an audience string); they hold no plan, person or amount",
    ),
    (
        "/api/director-pack-schedules",
        "the monthly schedules (an audience and a day); they hold no plan, person or amount",
    ),
    (
        "/api/director-pack-subscriptions",
        "your own subscriptions to audiences; they hold no plan or amount",
    ),
    (
        "/api/director-packs/{pid}/pdf",
        "one stored pack as a PDF, made from the stored JSON alone; computed once at drafting over plans that are neither private nor marked, and read by audience (tests/capacity_heatmap.rs)",
    ),
    (
        "/api/skill-pools",
        "the owner-supplied grouping of roles and skills into pools; it holds names, not plans, people or amounts",
    ),
    (
        "/api/funding-sources",
        "the owner-supplied reference list of where money comes from; it holds names, not plans or amounts",
    ),
    (
        "/api/director-packs",
        "the list of stored packs (metadata only); each pack is computed once, at drafting, over plans that are neither private nor marked, and read by audience (tests/director_packs.rs)",
    ),
    (
        "/api/handling-marks",
        "the deployment's configured scheme of marks (a list of words); no plan data",
    ),
    (
        "/api/break-glass",
        "the caller's own grants (all of them for an administrator): a plan id, a reason and times; no plan data",
    ),
    (
        "/api/processes",
        "process definitions: steps and people, no plan data",
    ),
    (
        "/api/process-executions",
        "the caller's own runs, or the runs on one subject they can read",
    ),
    ("/", "service banner"),
    ("/api/whoami", "the caller's own identity"),
    ("/api/me/inbox", "the caller's own inbox"),
    ("/api/me/plan", "the caller's own work"),
    (
        "/api/me/tasks",
        "the caller's own tasks (hidden plans filtered at source)",
    ),
    ("/api/me/watching", "the caller's own watches"),
    ("/api/me/calendar", "the caller's own availability"),
    ("/api/me/notification-prefs", "the caller's own preferences"),
    ("/api/me/notification-sinks", "the caller's own sinks"),
    ("/api/notifications", "the caller's own notifications"),
    ("/api/saved-views", "the caller's own views"),
    ("/api/availability", "a person's availability ranges"),
    ("/api/timesheets", "the caller's own week"),
    ("/api/workflows", "workflow definitions"),
    (
        "/api/plan-templates",
        "templates (private plans' templates filtered at source)",
    ),
    ("/api/canvas-templates", "templates"),
    ("/api/canvases", "canvases are per plan and read by plan"),
    ("/api/custom-fields", "field definitions"),
    (
        "/api/custom-fields/search",
        "searches within one kind, by plan guard",
    ),
    ("/api/custom-fields/values", "values of one subject"),
    ("/api/flow-classes", "vocabulary"),
    ("/api/onboarding/templates", "checklist templates"),
    ("/api/announcements", "messages to everyone"),
    ("/api/automations", "rule definitions"),
    ("/api/automations/runs", "runs of rules"),
    ("/api/scheduled-actions", "scheduled actions"),
    ("/api/report-schedules", "schedules of saved reports"),
    ("/api/resource-roles", "role vocabulary"),
    (
        "/api/cost-rate-tables",
        "finance rate tables, finance-gated",
    ),
    ("/api/cost-rates/lookup", "a rate lookup, finance-gated"),
    (
        "/api/feeds/calendar.ics",
        "token-authenticated personal feed",
    ),
    ("/api/feeds/me.atom", "token-authenticated personal feed"),
    ("/api/watchers", "watchers of one subject"),
    ("/api/reactions", "reactions on one subject"),
    ("/api/comments", "comments on one subject"),
    ("/api/attachments", "attachments of one subject"),
    (
        "/api/attachments/search",
        "searches attachments the caller can read",
    ),
    ("/api/reviews/consensus", "consensus on one subject"),
    (
        "/api/scenarios/compare",
        "two scenarios, evaluated for the caller",
    ),
    (
        "/api/approvals",
        "approvals, subjects resolved through the plan guard",
    ),
    ("/api/records/verify", "integrity check"),
    ("/api/audit/verify", "integrity check"),
    ("/api/metrics.prom", "process metrics"),
    ("/api/api-docs/openapi.json", "documentation"),
    ("/api/swagger-ui", "documentation"),
    ("/api/plans/whoami", "the caller's own identity"),
    ("/metrics.prom", "process metrics"),
];

/// Keys whose values legitimately differ between two captures (clocks), removed before comparing.
fn scrub(v: &mut Value) {
    const VOLATILE: &[&str] = &[
        "as_of",
        "generated_at",
        "computed_at",
        "now",
        "etag",
        "ts",
        "request_id",
        "created_at",
        "updated_at",
        "at",
        "occurred_at",
        "history_window",
        // Flow efficiency measures time since a task was created, so it moves with the clock.
        "denominator_ms",
        "numerator_ms",
    ];
    match v {
        Value::Object(m) => {
            for k in VOLATILE {
                m.remove(*k);
            }
            for x in m.values_mut() {
                scrub(x);
            }
        }
        Value::Array(a) => a.iter_mut().for_each(scrub),
        _ => {}
    }
}

async fn capture(
    request: &axum_test::TestServer,
    who: &Who,
    public: &str,
    hub: &str,
) -> std::collections::BTreeMap<String, (u16, Value)> {
    let mut out = std::collections::BTreeMap::new();
    for path in AGGREGATES.iter().chain(PRIVILEGED) {
        let url = path.replace("{pub}", public).replace("{hub}", hub);
        let r = request
            .get(&url)
            .add_header(who.0.clone(), who.1.clone())
            .await;
        let status = r.status_code().as_u16();
        let mut body: Value = serde_json::from_str(&r.text()).unwrap_or(Value::Null);
        scrub(&mut body);
        out.insert((*path).to_string(), (status, body));
    }
    out
}

struct Seeded {
    plan: String,
    task: String,
}

async fn seed(
    request: &axum_test::TestServer,
    who: &Who,
    name: &str,
    scale: i64,
    objective: &str,
    hub: &str,
) -> Seeded {
    let post = |path: String, body: Value| {
        let (request, who) = (request, who.clone());
        async move {
            request
                .post(&path)
                .add_header(who.0, who.1)
                .json(&body)
                .await
        }
    };
    let p: Value = post(
        "/api/plans".into(),
        json!({ "kind": "Project", "name": name, "code": format!("{name}-1"), "parent_ref": hub }),
    )
    .await
    .json();
    let p = p["pid"].as_str().unwrap().to_string();
    let line: Value = post(format!("/api/plans/{p}/budget-lines"), json!({ "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": 100_000 * scale })).await.json();
    post(
        format!(
            "/api/plans/{p}/budget-lines/{}/actual",
            line["pid"].as_str().unwrap()
        ),
        json!({ "amount_minor": 40_000 * scale }),
    )
    .await;
    post(format!("/api/plans/{p}/risks"), json!({ "title": format!("{name} risk"), "probability": 4, "impact": 5, "owner_ref": PERSON, "cost_impact_minor": 5000 * scale, "cost_currency": "GBP", "category": "security" })).await;
    post(format!("/api/plans/{p}/risks"), json!({ "title": format!("{name} compliance risk"), "probability": 3, "impact": 3, "category": "compliance" })).await;
    post(
        format!("/api/plans/{p}/milestones"),
        json!({ "name": format!("{name} milestone"), "due": "2026-10-20" }),
    )
    .await;
    let t: Value = post(format!("/api/plans/{p}/tasks"), json!({ "title": format!("{name} task"), "start_date": "2026-10-05", "duration_days": 4 * scale, "assignee_ref": PERSON, "points": 3 })).await.json();
    let t = t["pid"].as_str().unwrap().to_string();
    post(
        format!("/api/plans/{p}/tasks/{t}/assignment"),
        json!({ "units_percent": 100 }),
    )
    .await;
    let blocked: Value = post(
        format!("/api/plans/{p}/tasks"),
        json!({ "title": format!("{name} blocked") }),
    )
    .await
    .json();
    request
        .patch(&format!(
            "/api/plans/{p}/tasks/{}",
            blocked["pid"].as_str().unwrap()
        ))
        .add_header(who.0.clone(), who.1.clone())
        .json(&json!({ "status": "blocked" }))
        .await;
    post(
        format!("/api/plans/{p}/allocations"),
        json!({ "person_ref": PERSON, "percent": 50, "role": "Engineer" }),
    )
    .await;
    post(format!("/api/plans/{p}/time-entries"), json!({ "task_pid": t, "actor_ref": PERSON, "spent_on": "2026-10-06", "minutes": 120 * scale, "category": "work" })).await;
    post("/api/kpis".into(), json!({ "name": format!("{name} kpi"), "direction": "higher_better", "target_value": 90, "red_value": 50, "plan_pid": p })).await;
    post(format!("/api/plans/{p}/benefits"), json!({ "title": format!("{name} benefit"), "category": "financial", "currency": "GBP", "target_minor": 9000 * scale })).await;
    post(
        format!("/api/plans/{p}/gate-reviews"),
        json!({ "gate": "g0_concept", "decision": "approved" }),
    )
    .await;
    post(
        "/api/documents".into(),
        json!({ "title": format!("{name} policy"), "plan_pid": p, "body": format!("{name} text") }),
    )
    .await;
    post(format!("/api/plans/{p}/change-requests"), json!({ "title": format!("{name} change"), "description": "d", "reason": "r", "kind": "schedule", "impact_schedule_days": 3 * scale })).await;
    post(format!("/api/plans/{p}/issues"), json!({ "title": format!("{name} issue"), "kind": "blocker", "severity": "high", "owner_ref": PERSON, "raised_on": "2026-10-01" })).await;
    post(format!("/api/plans/{p}/access-records"), json!({ "person_ref": PERSON, "access": format!("{name} repo"), "action": "granted", "recorded_on": "2026-01-05" })).await;
    // A framework owned by the plan: a non-member must not see it in either list.
    post("/api/ways-of-working".into(), json!({ "name": format!("{name} way"), "methodology": "kanban", "plan_pid": p })).await;
    post("/api/governance-frameworks".into(), json!({ "name": format!("{name} board"), "decision_model": "sponsor", "plan_pid": p })).await;
    post(format!("/api/plans/{p}/sprints"), json!({ "name": format!("{name} sprint"), "starts_on": "2026-10-05", "ends_on": "2026-10-19" })).await;
    post("/api/devops/events".into(), json!({ "plan_pid": p, "kind": "deploy", "environment": "prod", "version": format!("{name}-1.0") })).await;
    post(
        format!("/api/plans/{p}/objectives"),
        json!({ "objective_pid": objective, "weight": 3 }),
    )
    .await;
    Seeded { plan: p, task: t }
}

/// A proposal that has become a plan, so the proposal itself must follow the plan's privacy.
async fn promoted_proposal(request: &axum_test::TestServer, who: &Who, title: &str) -> String {
    let post = |path: String, body: Value| {
        let (request, who) = (request, who.clone());
        async move {
            request
                .post(&path)
                .add_header(who.0, who.1)
                .json(&body)
                .await
        }
    };
    let pr: Value = post("/api/proposals".into(), json!({ "title": title, "kind_target": "projects", "requested_minor": 7000, "currency": "GBP" })).await.json();
    let pid = pr["pid"].as_str().unwrap().to_string();
    for step in ["submit", "review", "approve"] {
        post(format!("/api/proposals/{pid}/{step}"), json!({})).await;
    }
    let promoted: Value = post(format!("/api/proposals/{pid}/promote"), json!({}))
        .await
        .json();
    promoted["plan_pid"]
        .as_str()
        .map_or_else(|| promoted.to_string(), str::to_string)
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test private_aggregates -- --ignored`"]
async fn a_private_plan_is_in_no_aggregate_for_a_non_member() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let (alice, bob, admin) = (
        hdr(&kid, ALICE, &["write"]),
        hdr(&kid, BOB, &["write"]),
        hdr(&kid, ADMIN, &["admin"]),
    );
    request::<App, _, _>(|request, _ctx| async move {
        // ── the mounted routes: every parameterless GET is decided ──
        let docs: Value = request.get("/api-docs/openapi.json").add_header(bob.0.clone(), bob.1.clone()).await.json();
        let mut undecided = Vec::new();
        for (path, item) in docs["paths"].as_object().unwrap() {
            if path.contains('{') || item.get("get").is_none() { continue; }
            let known = AGGREGATES.iter().chain(PRIVILEGED).any(|a| a.split('?').next() == Some(path.as_str()))
                || NOT_AGGREGATE.iter().any(|(p, _)| p == path);
            if !known { undecided.push(path.clone()); }
        }
        assert!(undecided.is_empty(), "these GET routes are neither in AGGREGATES/PRIVILEGED (filtered and compared) nor in NOT_AGGREGATE (with a reason): {undecided:#?}");

        // ── the world as the non-member sees it with only the public plan in it ──
        let shared: Value = request.post("/api/objectives").add_header(bob.0.clone(), bob.1.clone()).json(&json!({ "title": "Shared objective" })).await.json();
        let shared = shared["pid"].as_str().unwrap().to_string();
        let hub: Value = request.post("/api/plans").add_header(bob.0.clone(), bob.1.clone()).json(&json!({ "kind": "Portfolio", "name": "Hub" })).await.json();
        let hub = hub["pid"].as_str().unwrap().to_string();
        let public = seed(&request, &bob, "Public", 1, &shared, &hub).await;
        let scen_a: Value = request.post("/api/scenarios").add_header(bob.0.clone(), bob.1.clone())
            .json(&json!({ "name": "Set", "plan_pids": [public.plan], "budget_cap_minor": 500_000, "currency": "GBP" })).await.json();
        let scen_a = scen_a["pid"].as_str().unwrap().to_string();
        let without = capture(&request, &bob, &public.plan, &hub).await;
        // The subtree rollup measures elapsed time, so it moves with the clock: compare what it
        // covers (which plans, how many), not the durations.
        let rollup_of = |who: Who| {
            let (request, url) = (&request, format!("/api/plans/{hub}/rollup"));
            async move { request.get(&url).add_header(who.0, who.1).await.json::<Value>() }
        };
        let rollup_without = rollup_of(bob.clone()).await;
        let eval_without: Value = request.get(&format!("/api/scenarios/{scen_a}/evaluate")).add_header(bob.0.clone(), bob.1.clone()).await.json();

        // ── add a private plan carrying data of every kind (twice the size, so any leak shows) ──
        let secret = seed(&request, &alice, "Secret", 2, &shared, &hub).await;
        let promoted = promoted_proposal(&request, &alice, "Secret proposal").await;
        for p in [&secret.plan, &promoted] {
            request.put(&format!("/api/plans/{p}/access")).add_header(alice.0.clone(), alice.1.clone()).json(&json!({ "private": true, "members": [] })).await.assert_status_ok();
        }

        let rollup_with = rollup_of(bob.clone()).await;
        assert_eq!(rollup_without["tree"], rollup_with["tree"], "the plans a non-member's rollup covers");
        assert_eq!(rollup_without["plan"], rollup_with["plan"]);
        assert!(!rollup_with.to_string().contains(&secret.plan), "{rollup_with}");
        assert!(rollup_of(alice.clone()).await["tree"]["plans"].as_i64() > rollup_with["tree"]["plans"].as_i64(), "a member's rollup covers the private plan too");
        let with_hidden = capture(&request, &bob, &public.plan, &hub).await;
        let member = capture(&request, &alice, &public.plan, &hub).await;
        let full = capture(&request, &admin, &public.plan, &hub).await;

        // A scenario naming the private plan (created after the captures: a scenario is a public record).
        let scen_b: Value = request.post("/api/scenarios").add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "name": "Set", "plan_pids": [public.plan, secret.plan], "must_include": [secret.plan], "budget_cap_minor": 500_000, "currency": "GBP" })).await.json();
        let scen_b = scen_b["pid"].as_str().unwrap().to_string();

        let mut leaks = Vec::new();
        let mut member_gaps = Vec::new();
        let mut not_ok = Vec::new();
        for path in AGGREGATES {
            let (s0, b0) = &without[*path];
            let (s1, b1) = &with_hidden[*path];
            if *s0 != 200 { not_ok.push(format!("{path} answered {s0}")); }
            if s0 != s1 || b0 != b1 {
                let (t0, t1) = (b0.to_string(), b1.to_string());
                let at = t0.bytes().zip(t1.bytes()).position(|(x, y)| x != y).unwrap_or(t0.len().min(t1.len()));
                let from = at.saturating_sub(60);
                let cut = |t: &str| t.get(from..t.len().min(at + 200)).unwrap_or("").to_string();
                leaks.push(format!("{path}: {} vs {}", cut(&t0), cut(&t1)));
            }
            if MASKED_FOR_MEMBER.contains(path) { continue; }
            let (sm, bm) = &member[*path];
            let (sf, bf) = &full[*path];
            if sm != sf || bm != bf { member_gaps.push(format!("{path} [member {sm}, admin {sf}]")); }
        }
        assert!(not_ok.is_empty(), "a route the test does not exercise (fix its parameters): {not_ok:?}");
        assert!(leaks.is_empty(), "for a non-member these change when a private plan appears (it leaks): {leaks:#?}");
        assert!(member_gaps.is_empty(), "a member must see the full totals an admin sees: {member_gaps:#?}");
        for path in PRIVILEGED {
            assert_eq!(with_hidden[*path].0, 403, "{path} for a non-admin");
            assert_eq!(member[*path].0, 403, "{path} for a member who is not an admin");
            assert_eq!(full[*path].0, 200, "{path} for an admin");
        }

        // The aggregates are not simply empty: the member and the admin do see the private plan's figures.
        let g: Value = request.get("/api/at-a-glance").add_header(admin.0.clone(), admin.1.clone()).await.json();
        let g_without = &without["/api/at-a-glance"].1;
        assert_ne!(&{ let mut x = g.clone(); scrub(&mut x); x }, g_without, "the private plan really does contribute to the full totals");

        // A route that names its plan in the query is not protected by the plan guard, so it checks itself.
        for url in [format!("/api/financials/forecast?plan={}", secret.plan), format!("/api/time-reports?group_by=plan&plan={}", secret.plan)] {
            let r = request.get(&url).add_header(bob.0.clone(), bob.1.clone()).await;
            let text = r.text();
            assert!(r.status_code() == 404 || !text.contains(&secret.plan), "{url}: {text}");
        }
        assert_eq!(request.get(&format!("/api/financials/forecast?plan={}", secret.plan)).add_header(bob.0.clone(), bob.1.clone()).await.status_code(), 404);
        assert_eq!(request.get(&format!("/api/financials/forecast?plan={}", secret.plan)).add_header(alice.0.clone(), alice.1.clone()).await.status_code(), 200);

        // A scenario that names the private plan evaluates, for a non-member, as if it did not.
        let eval_with: Value = request.get(&format!("/api/scenarios/{scen_b}/evaluate")).add_header(bob.0.clone(), bob.1.clone()).await.json();
        let strip = |mut v: Value| { for k in ["pid", "name", "as_of", "inputs_read"] { if let Some(m) = v.as_object_mut() { m.remove(k); } } v };
        let (a, b) = (strip(eval_without.clone()), strip(eval_with.clone()));
        assert_eq!(a, b, "the scenario for a non-member");
        assert!(!eval_with.to_string().contains(&secret.plan), "{eval_with}");
        let listed = request.get("/api/scenarios").add_header(bob.0.clone(), bob.1.clone()).await.text();
        assert!(!listed.contains(&secret.plan), "the scenario list names the private plan: {listed}");
        let alice_eval: Value = request.get(&format!("/api/scenarios/{scen_b}/evaluate")).add_header(alice.0.clone(), alice.1.clone()).await.json();
        assert_ne!(strip(alice_eval), b, "a member's evaluation does include it");
        // Nor can a non-member put it in a scenario of their own.
        let r = request.post("/api/scenarios").add_header(bob.0.clone(), bob.1.clone()).json(&json!({ "name": "Probe", "plan_pids": [secret.plan] })).await;
        assert_eq!(r.status_code(), 422);
        assert!(r.text().contains("unknown plan"));
        // The proposal that became the private plan is hidden with it.
        let proposals = request.get("/api/proposals").add_header(bob.0.clone(), bob.1.clone()).await.text();
        assert!(!proposals.contains("Secret proposal"), "{proposals}");
        let _ = (public.task, secret.task);
    })
    .await;
}
