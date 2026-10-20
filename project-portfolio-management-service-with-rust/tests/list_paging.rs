//! RM-12b: every list pages the same way, **driven by the OpenAPI document**. The test
//! signs in as an admin, seeds one of each kind of record, then calls every documented
//! `GET` it can reach. Every one that answers `200` with a bare JSON array is a list and
//! must carry `X-Total-Count` / `X-Limit` / `X-Offset`, honour `limit` and `offset`,
//! refuse an absurd offset, and document `limit` and `offset` itself. A documented `GET`
//! it cannot reach must be named, with a reason, in `UNREACHED`, so a new list cannot
//! slip past by needing a parameter nobody supplied.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test list_paging -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [71; 32];
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

use std::collections::BTreeMap;

fn admin(kid: &str) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    auth_header(&mint(
        kid,
        "3f2c6b7e-0000-4000-8000-0000000000ad",
        &["admin"],
        &[],
    ))
}

/// Documented GETs the census cannot call, each with the reason. Anything not here that
/// is unreachable fails the test.
const UNREACHED: &[(&str, &str)] = &[
    (
        "/api/plans/{pid}/escalations/{epid}",
        "one escalation with its history (an object, not a list); needs an escalation this census does not create",
    ),
    (
        "/api/ways-of-working/{pid}",
        "one record (an object, not a list); needs a record this census does not create",
    ),
    (
        "/api/governance-frameworks/{pid}",
        "one record (an object, not a list); needs a record this census does not create",
    ),
    (
        "/api/resource-plan/role-tasks",
        "one role's people with their open tasks (an object, not a list); needs `?role=`, which names a role this census does not create",
    ),
    (
        "/api/funding-allocations/{pid}/trace",
        "one allocation's trace (an object with lists inside, bounded by the lines drawing on it); needs an allocation this census does not create",
    ),
    (
        "/api/scorecards/{pid}",
        "one scorecard's KPIs by quarter (an object, not a list); needs a scorecard this census does not create",
    ),
    (
        "/api/plans/{pid}/thresholds",
        "one programme's few thresholds (an object with its list inside, at most three); needs a programme this census does not create",
    ),
    (
        "/api/plans/{pid}/triage",
        "one programme's triage history (an object with the latest and its history); needs a programme this census does not create",
    ),
    (
        "/api/director-packs/{pid}/pdf",
        "one stored pack as a PDF document, not a list; needs a pack this census does not create",
    ),
    (
        "/api/plans/{pid}/benefits/{benefit}/validations",
        "one benefit's validation history (an object with its history inside, a few rows per benefit); needs a benefit this census does not create",
    ),
    (
        "/api/director-packs/{pid}",
        "one stored pack, an object and not a list; needs a pack this census does not create",
    ),
    (
        "/api/plans/{pid}/change-requests/{cr}",
        "one change request, an object and not a list; needs a request this census does not create",
    ),
    (
        "/api/plans/{pid}/change-requests/{cr}/preview",
        "the forecast of one change request's edits; an object and needs a request with edits",
    ),
    (
        "/api/admin/subjects/{subject}",
        "an administrator-only export of one person's data, an object and not a list",
    ),
    (
        "/api/plans/{pid}/issues/{issue_pid}",
        "one issue, an object and not a list; needs an issue this census does not create",
    ),
    (
        "/api/custom-fields/search",
        "needs a custom field defined and valued, which this census does not set up",
    ),
    (
        "/api/key-results/{pid}/check-ins",
        "needs a key result on a valid metric, which this census does not set up",
    ),
    (
        "/api/report-schedules/{pid}/latest",
        "needs a scheduled report that has run",
    ),
    (
        "/api/plans/{pid}/live",
        "a server-sent event stream: it never ends",
    ),
    (
        "/api/feeds/calendar.ics",
        "a token-authenticated calendar file, not JSON",
    ),
    ("/api/feeds/me.atom", "a token-authenticated feed, not JSON"),
    ("/api/plans/{pid}/export", "a file"),
    ("/api/attachments/{pid}/download", "a file"),
    ("/api/plans/export/{id}/download", "a file"),
    ("/api/plans/bulk-jobs/{id}/errors", "a file"),
    ("/api/board/pack.pdf", "a PDF"),
    ("/metrics.prom", "text, not JSON"),
    ("/swagger-ui", "an HTML page"),
    ("/", "a banner"),
    ("/api-docs/openapi.json", "the document itself"),
];

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test list_paging -- --ignored`"]
async fn every_list_pages_the_same_way() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let who = admin(&kid);
    request::<App, _, _>(|request, _ctx| async move {
        let post = |path: String, body: Value| {
            let (request, who) = (&request, who.clone());
            async move {
                let r = request.post(&path).add_header(who.0, who.1).json(&body).await;
                serde_json::from_str::<Value>(&r.text()).unwrap_or(Value::Null)
            }
        };
        let pid_of = |v: &Value| v["pid"].as_str().map(str::to_string);
        let person = "person:00000000-0000-4000-8000-0000000000c1";
        let plan = pid_of(&post("/api/plans".into(), json!({ "kind": "Project", "name": "Census" })).await).unwrap();
        let task = pid_of(&post(format!("/api/plans/{plan}/tasks"), json!({ "title": "T", "start_date": "2026-10-05", "duration_days": 2, "assignee_ref": person })).await).unwrap();
        let mut candidates: Vec<String> = vec![plan.clone(), task.clone()];
        let mut sprint = String::new();
        for (path, body, is_sprint) in [
            (format!("/api/plans/{plan}/milestones"), json!({ "name": "M", "due": "2026-10-20" }), false),
            (format!("/api/plans/{plan}/sprints"), json!({ "name": "S", "starts_on": "2026-10-05", "ends_on": "2026-10-19" }), true),
            (format!("/api/plans/{plan}/risks"), json!({ "title": "R", "probability": 3, "impact": 3 }), false),
            ("/api/kpis".to_string(), json!({ "name": "K", "direction": "higher_better", "target_value": 90, "red_value": 50, "plan_pid": plan }), false),
            ("/api/canvases".to_string(), json!({ "template": "lean", "title": "C", "plan_pid": plan }), false),
            (format!("/api/plans/{plan}/controls"), json!({ "name": "C", "timing": "feedforward", "metric": "gate_readiness", "target_value": 10_000, "comparator": "at_least", "cadence_days": 7 }), false),
            (format!("/api/plans/{plan}/onboarding/cases"), json!({ "kind": "plan_intake", "effective_date": "2026-10-12" }), false),
            ("/api/proposals".to_string(), json!({ "title": "P", "kind_target": "projects" }), false),
            (format!("/api/plans/{plan}/save-as-template"), json!({ "name": "Template" }), false),
            ("/api/comments".to_string(), json!({ "subject_kind": "plan", "subject_pid": plan, "body": "hello" }), false),
            ("/api/me/notification-sinks".to_string(), json!({ "kind": "webhook", "destination": "https://example.com/hook" }), false),
            ("/api/reports".to_string(), json!({ "name": "R1", "collection": "", "fields": ["name"] }), false),
        ] {
            if let Some(p) = pid_of(&post(path, body).await) {
                if is_sprint { sprint = p.clone(); }
                candidates.push(p);
            }
        }
        let line = post(format!("/api/plans/{plan}/budget-lines"), json!({ "category": "capex", "description": "spend", "currency": "GBP", "planned_minor": 100_000 })).await;
        let _ = line;
        // A controlled document.
        if let Some(doc) = pid_of(&post("/api/documents".into(), json!({ "title": "Policy", "body": "text" })).await) {
            candidates.push(doc);
        }
        // A process, and a run of it.
        if let Some(proc) = pid_of(&post("/api/processes".into(), json!({ "name": "Census", "steps": [{ "kind": "notify", "person_ref": person, "title": "Hello" }] })).await) {
            if let Some(run) = pid_of(&post(format!("/api/processes/{proc}/start"), json!({ "subject_kind": "plan", "subject_pid": plan })).await) {
                candidates.push(run);
            }
            candidates.push(proc);
        }
        let objective = pid_of(&post("/api/objectives".into(), json!({ "title": "O" })).await).unwrap();
        candidates.push(objective.clone());
        if let Some(kr) = pid_of(&post(format!("/api/objectives/{objective}/key-results"), json!({ "title": "KR", "metric": "m", "direction": "increase", "start_value": 0, "target_value": 10 })).await) { candidates.push(kr); }
        let scen_a = pid_of(&post("/api/scenarios".into(), json!({ "name": "A", "plan_pids": [plan] })).await).unwrap();
        let scen_b = pid_of(&post("/api/scenarios".into(), json!({ "name": "B", "plan_pids": [plan] })).await).unwrap();
        // A report on a schedule, and an export job.
        let report = candidates.last().cloned();
        let _ = report;
        let export = post("/api/plans/export".into(), json!({})).await;
        let export_id = export["job_id"].as_str().unwrap_or_default().to_string();
        // A finished import, for the job ids.
        let job = request.post("/api/plans/import?format=jsonl&on_keyless=create").add_header(who.0.clone(), who.1.clone())
            .content_type("text/plain").text(r#"{"name":"Imported","kind":"Project","code":"IMP-1"}"#).await;
        let job_id = job.json::<Value>()["job_id"].as_str().unwrap_or_default().to_string();
        // The values each named path parameter may take (`pid` is a plan or any other record).
        let pools: BTreeMap<&str, Vec<String>> = BTreeMap::from([
            ("pid", candidates.clone()),
            ("t_pid", vec![task.clone()]),
            ("s_pid", vec![sprint.clone()]),
            ("version", vec!["1".to_string()]),
            ("id", vec![job_id, export_id]),
        ]);
        // Query strings some lists need to answer at all.
        let extra: BTreeMap<&str, String> = BTreeMap::from([
            ("/api/attachments", format!("subject_kind=plan&subject_pid={plan}")),
            ("/api/attachments/search", "q=xx".to_string()),
            ("/api/comments", format!("subject_kind=plan&subject_pid={plan}")),
            ("/api/reactions", format!("subject_kind=plan&subject_pid={plan}")),
            ("/api/watchers", format!("subject_kind=plan&subject_pid={plan}")),
            ("/api/availability", format!("person_ref={person}")),
            ("/api/cost-rates/lookup", format!("person_ref={person}&date=2026-10-05")),
            ("/api/custom-fields/search", "applies_to=task&key=x&value=y".to_string()),
            ("/api/custom-fields/values", format!("applies_to=task&subject_pid={task}")),
            ("/api/financials/forecast", format!("plan={plan}")),
            ("/api/plans/search", "q=Census".to_string()),
            ("/api/time-reports", "group_by=plan".to_string()),
            ("/api/reviews/consensus", format!("subject_kind=plan&subject_pid={plan}")),
            ("/api/notifications", format!("recipient={person}")),
            ("/api/timesheets", "week=2026-W41".to_string()),
            ("/api/me/inbox", "tz=UTC".to_string()),
            ("/api/me/tasks", "tz=UTC".to_string()),
            ("/api/me/plan", "day=2026-10-06&tz=UTC".to_string()),
            ("/api/plans/{pid}/burndown", format!("sprint={sprint}")),
            ("/api/scenarios/compare", format!("a={scen_a}&b={scen_b}")),
        ]);
        let docs: Value = request.get("/api-docs/openapi.json").add_header(who.0.clone(), who.1.clone()).await.json();
        let (mut lists, mut unreached, mut broken, mut undocumented) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for (path, item) in docs["paths"].as_object().unwrap() {
            let Some(get) = item.get("get") else { continue };
            // Every combination of values the path's parameters may take (usually one name, a few values).
            let mut try_urls: Vec<String> = vec![path.clone()];
            while try_urls.iter().any(|u| u.contains('{')) {
                let mut next = Vec::new();
                for u in &try_urls {
                    let Some(a) = u.find('{') else { next.push(u.clone()); continue };
                    let b = u[a..].find('}').unwrap() + a;
                    let name = &u[a + 1..b];
                    for v in pools.get(name).map(Vec::as_slice).unwrap_or(&[]) {
                        let mut url = u.clone();
                        url.replace_range(a..=b, v);
                        next.push(url);
                    }
                }
                try_urls = next;
                if try_urls.is_empty() { break; }
            }
            let mut found = None;
            let mut last = String::new();
            for base in &try_urls {
                let url = match extra.get(path.as_str()) { Some(q) => format!("{base}?{q}"), None => base.clone() };
                let Ok(r) = tokio::time::timeout(std::time::Duration::from_secs(5), request.get(&url).add_header(who.0.clone(), who.1.clone())).await else { break };
                if r.status_code() == 200 {
                    found = Some((url, r));
                    break;
                }
                last = format!("{} {}", r.status_code(), r.text().chars().take(90).collect::<String>());
            }
            let Some((url, r)) = found else {
                if !UNREACHED.iter().any(|(p, _)| p == path) { unreached.push(format!("{path}  ({last})")); }
                continue;
            };
            let Ok(body) = serde_json::from_str::<Value>(&r.text()) else { continue };
            let Some(rows) = body.as_array() else { continue };
            lists.push(path.clone());
            let sep = if url.contains('?') { '&' } else { '?' };
            let total = r.maybe_header("x-total-count").and_then(|v| v.to_str().ok().and_then(|s| s.parse::<usize>().ok()));
            if total.is_none() || r.maybe_header("x-limit").is_none() || r.maybe_header("x-offset").is_none() {
                broken.push(format!("{path}: no X-Total-Count/X-Limit/X-Offset"));
                continue;
            }
            if total.is_none_or(|t| t < rows.len()) { broken.push(format!("{path}: X-Total-Count {total:?} but {} rows", rows.len())); }
            let one = request.get(&format!("{url}{sep}limit=1")).add_header(who.0.clone(), who.1.clone()).await;
            let one_rows = serde_json::from_str::<Value>(&one.text()).ok().and_then(|v| v.as_array().map(Vec::len));
            if one_rows.is_none_or(|n| n > 1) || one.maybe_header("x-limit").and_then(|v| v.to_str().ok().map(str::to_string)).as_deref() != Some("1") {
                broken.push(format!("{path}: limit=1 not honoured ({one_rows:?})"));
            }
            let past = request.get(&format!("{url}{sep}offset=9000")).add_header(who.0.clone(), who.1.clone()).await;
            let past_rows = serde_json::from_str::<Value>(&past.text()).ok().and_then(|v| v.as_array().map(Vec::len));
            if past_rows != Some(0) || past.maybe_header("x-total-count").is_none() {
                broken.push(format!("{path}: an offset past the end is not an empty page that still says the total ({past_rows:?})"));
            }
            let absurd = request.get(&format!("{url}{sep}offset=999999999")).add_header(who.0.clone(), who.1.clone()).await;
            if absurd.status_code() != 400 { broken.push(format!("{path}: an absurd offset is {}, not 400", absurd.status_code())); }
            let bad = request.get(&format!("{url}{sep}limit=abc")).add_header(who.0.clone(), who.1.clone()).await;
            if bad.status_code() != 400 { broken.push(format!("{path}: limit=abc is {}, not 400", bad.status_code())); }
            let params: Vec<&str> = get["parameters"].as_array().map(|a| a.iter().filter_map(|p| p["name"].as_str()).collect()).unwrap_or_default();
            if !(params.contains(&"limit") && params.contains(&"offset")) { undocumented.push(path.clone()); }
            let _ = get;
        }
        // The registry the spec uses is exactly what the census finds (plus the lists it cannot reach).
        let registry: std::collections::BTreeSet<&str> = project_portfolio_management::openapi::PAGED_LISTS.iter().copied().collect();
        let not_registered: Vec<&String> = lists.iter().filter(|l| !registry.contains(l.as_str())).collect();
        let unreached_ok: Vec<&str> = UNREACHED.iter().map(|(p, _)| *p).collect();
        let stale: Vec<&&str> = registry.iter().filter(|r| !lists.iter().any(|l| l == **r) && !unreached_ok.contains(*r)).collect();
        assert!(not_registered.is_empty(), "lists the census found that PAGED_LISTS does not name: {not_registered:?}");
        assert!(stale.is_empty(), "PAGED_LISTS names paths that are not lists (or not reachable and not allowlisted): {stale:?}");
        eprintln!("LISTS ({}): {lists:?}", lists.len());
        eprintln!("UNREACHED not allowlisted ({}):\n{}", unreached.len(), unreached.join("\n"));
        eprintln!("UNDOCUMENTED ({}):\n{}", undocumented.len(), undocumented.join("\n"));
        eprintln!("BROKEN ({}):\n{}", broken.len(), broken.join("\n"));
        assert!(unreached.is_empty(), "documented GETs the census could not call and nobody explained: {unreached:#?}");
        assert!(broken.is_empty(), "lists that do not page the standard way: {broken:#?}");
        assert!(undocumented.is_empty(), "lists whose OpenAPI entry does not document limit and offset: {undocumented:#?}");
    })
    .await;
}
