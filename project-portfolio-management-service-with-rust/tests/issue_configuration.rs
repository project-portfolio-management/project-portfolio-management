//! Organization-defined issue types, statuses and roles (OC-1), end to end with real PASETO identities. These are the
//! properties the feature must hold: who can change the catalogues and workflows (an administrator, `403` otherwise);
//! that a role **only ever narrows** what a token allows; that a custom status counts by its category, never its name; that a
//! workflow can differ by issue type and restrict a move to a role, as Redmine's matrix does; that a retired type stays
//! valid on the issues that carry it; and that a plan with nothing configured behaves as before.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test issue_configuration -- --ignored --test-threads=1`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::app::AppContext;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [71; 32];
const ISSUER: &str = "authentication-service";
const AUDIENCE: &str = "main-x-service";
const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000001a1";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000001ad";
const MANAGER: &str = "3f2c6b7e-0000-4000-8000-0000000001b2";

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

/// Each test starts from the migrated catalogue, with no leftover type, role or issue workflow.
async fn clean(ctx: &AppContext) {
    ctx.db
        .execute_unprepared(
            "DELETE FROM workflows WHERE applies_to = 'issue';
             DELETE FROM plan_role_members;
             DELETE FROM plan_issue_types;
             DELETE FROM organization_roles;
             DELETE FROM issues WHERE kind NOT IN
                 ('bug', 'risk', 'blocker', 'question', 'improvement', 'concern');
             DELETE FROM issue_types WHERE key NOT IN
                 ('bug', 'risk', 'blocker', 'question', 'improvement', 'concern');
             UPDATE issue_types SET retired_at = NULL;",
        )
        .await
        .expect("clean");
}

fn triage_workflow(name: &str, issue_type: Option<&str>, plan: Option<&str>) -> Value {
    json!({
        "name": name, "applies_to": "issue", "issue_type": issue_type, "plan_pid": plan,
        "is_default": plan.is_none(),
        "states": [
            { "key": "new", "label": "New", "category": "todo", "is_initial": true },
            { "key": "with_supplier", "label": "With supplier", "category": "waiting" },
            { "key": "fixed", "label": "Fixed", "category": "done" },
            { "key": "signed_off", "label": "Signed off", "category": "done", "is_terminal": true },
        ],
        "transitions": [
            { "from": "new", "to": "with_supplier" },
            { "from": "with_supplier", "to": "fixed" },
            { "from": "fixed", "to": "signed_off", "roles": ["change-manager"] },
        ],
    })
}

async fn new_plan(request: &axum_test::TestServer, who: &Header, name: &str) -> String {
    let created: Value = request
        .post("/api/plans")
        .add_header(who.0.clone(), who.1.clone())
        .json(&json!({ "kind": "Project", "name": name }))
        .await
        .json();
    created["pid"].as_str().expect("plan pid").to_string()
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn only_an_administrator_changes_the_catalogues_and_issue_workflows() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);

        // Reading is open to anyone signed in; the six built-in types are there.
        let types: Value = request
            .get("/api/issue-types")
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        assert_eq!(types.as_array().map(Vec::len), Some(6), "{types}");

        for (path, body) in [
            (
                "/api/issue-types",
                json!({ "key": "incident", "label": "Incident" }),
            ),
            (
                "/api/organization-roles",
                json!({ "key": "change-manager", "label": "Change manager" }),
            ),
            ("/api/workflows", triage_workflow("Mine", None, None)),
        ] {
            let refused = request
                .post(path)
                .add_header(writer.0.clone(), writer.1.clone())
                .json(&body)
                .await;
            assert_eq!(refused.status_code(), 403, "{path}");
        }
        let refused = request
            .put("/api/issue-types/bug")
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "label": "Mine" }))
            .await;
        assert_eq!(refused.status_code(), 403);

        // The administrator can, and a bad or repeated key is refused by name.
        let added = request
            .post("/api/issue-types")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "incident", "label": "Incident", "position": 9 }))
            .await;
        assert_eq!(added.status_code(), 201, "{}", added.text());
        let again = request
            .post("/api/issue-types")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "incident", "label": "Twice" }))
            .await;
        assert_eq!(again.status_code(), 409);
        let bad = request
            .post("/api/issue-types")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "Not A Key", "label": "Bad" }))
            .await;
        assert_eq!(bad.status_code(), 422);

        // An issue workflow cannot be withdrawn by a writer either.
        let made: Value = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Everyone", None, None))
            .await
            .json();
        let pid = made["pid"].as_str().expect("workflow pid");
        let refused = request
            .delete(&format!("/api/workflows/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .await;
        assert_eq!(refused.status_code(), 403);
        // A second default for the same scope would make "the workflow in force" ambiguous.
        let second = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Everyone, again", None, None))
            .await;
        assert_eq!(second.status_code(), 409);
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_role_makes_nobody_an_administrator() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        // A deployment calls a role `admin`. Holding that role must not make anyone an administrator.
        let named = request
            .post("/api/organization-roles")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "admin", "label": "Administrator" }))
            .await;
        assert_eq!(named.status_code(), 201);
        let role_holder = bearer(&kid, MANAGER, &[("access", "write"), ("role", "admin")]);
        let refused = request
            .post("/api/issue-types")
            .add_header(role_holder.0.clone(), role_holder.1.clone())
            .json(&json!({ "key": "escalated", "label": "Escalated" }))
            .await;
        assert_eq!(refused.status_code(), 403);
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_workflow_can_restrict_a_move_by_type_and_role_and_a_status_counts_by_its_category() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let manager = bearer(
            &kid,
            MANAGER,
            &[("access", "write"), ("role", "change-manager")],
        );
        let plan = new_plan(&request, &writer, "Configured").await;
        let issues = format!("/api/plans/{plan}/issues");

        // A role the catalogue does not hold cannot be named once the catalogue is in use.
        for (path, body) in [
            (
                "/api/organization-roles",
                json!({ "key": "reviewer", "label": "Reviewer" }),
            ),
            (
                "/api/issue-types",
                json!({ "key": "incident", "label": "Incident" }),
            ),
        ] {
            let ok = request
                .post(path)
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&body)
                .await;
            assert_eq!(ok.status_code(), 201, "{}", ok.text());
        }
        let misspelt = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Typo", Some("incident"), None))
            .await;
        assert_eq!(misspelt.status_code(), 422, "{}", misspelt.text());
        let ok = request
            .post("/api/organization-roles")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "change-manager", "label": "Change manager" }))
            .await;
        assert_eq!(ok.status_code(), 201);
        let ok = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Incidents", Some("incident"), None))
            .await;
        assert_eq!(ok.status_code(), 200, "{}", ok.text());

        // The incident workflow applies to incidents and to nothing else.
        let options: Value = request
            .get(&format!("{issues}/options"))
            .add_header(manager.0.clone(), manager.1.clone())
            .await
            .json();
        let of = |key: &str| {
            options["types"]
                .as_array()
                .and_then(|t| t.iter().find(|x| x["key"] == key))
                .cloned()
                .expect("type")
        };
        assert_eq!(of("incident")["workflow"]["states"][0]["key"], "new");
        assert_eq!(of("bug")["workflow"]["states"][0]["key"], "open");
        assert_eq!(options["caller_roles"], json!(["change-manager"]));

        let bug: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "An ordinary bug", "kind": "bug" }))
            .await
            .json();
        assert_eq!(bug["status"], "open");

        // A new issue starts where its workflow says, and cannot be raised past a gate.
        let past = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Skip it", "kind": "incident", "status": "fixed" }))
            .await;
        assert_eq!(past.status_code(), 422, "{}", past.text());
        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(
                &json!({ "title": "Signal failure", "kind": "incident", "due_date": "2020-01-01",
                           "raised_on": "2019-01-01" }),
            )
            .await
            .json();
        assert_eq!(raised["status"], "new", "{raised}");
        assert_eq!(raised["status_category"], "todo");
        let pid = raised["pid"].as_str().expect("issue pid");
        let one = format!("{issues}/{pid}");
        let body = |status: &str, resolution: Option<&str>| {
            json!({ "title": "Signal failure", "kind": "incident", "status": status,
                    "resolution": resolution, "due_date": "2020-01-01", "raised_on": "2019-01-01" })
        };

        // A move the workflow does not declare is refused.
        let skip = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&body("fixed", None))
            .await;
        assert_eq!(skip.status_code(), 422, "{}", skip.text());

        // `with_supplier` is the organization's own word for waiting: still open, still overdue.
        let moved: Value = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&body("with_supplier", None))
            .await
            .json();
        assert_eq!(moved["status_category"], "waiting", "{moved}");
        assert_eq!(moved["overdue"], true);
        let open: Value = request
            .get(&format!("{issues}?open=true"))
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        assert_eq!(open["issues"].as_array().map(Vec::len), Some(2), "{open}");

        // `fixed` means done, though it is not called resolved or closed.
        let fixed: Value = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&body("fixed", None))
            .await
            .json();
        assert_eq!(fixed["status_category"], "done", "{fixed}");
        assert_eq!(fixed["overdue"], false);
        assert!(fixed["resolved_on"].is_string(), "{fixed}");
        let open: Value = request
            .get(&format!("{issues}?open=true"))
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        assert_eq!(open["issues"].as_array().map(Vec::len), Some(1), "{open}");

        // The sign-off is the change manager's alone, and a terminal status needs a resolution.
        let refused = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&body("signed_off", Some("Replaced the relay")))
            .await;
        assert_eq!(refused.status_code(), 403, "{}", refused.text());
        let unresolved = request
            .put(&one)
            .add_header(manager.0.clone(), manager.1.clone())
            .json(&body("signed_off", None))
            .await;
        assert_eq!(unresolved.status_code(), 422, "{}", unresolved.text());
        let signed: Value = request
            .put(&one)
            .add_header(manager.0.clone(), manager.1.clone())
            .json(&body("signed_off", Some("Replaced the relay")))
            .await
            .json();
        assert_eq!(signed["status"], "signed_off", "{signed}");

        // A filter finds the organization's status, and a typo is still refused rather than answered with nothing.
        let found: Value = request
            .get(&format!("{issues}?status=signed_off"))
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        assert_eq!(found["issues"].as_array().map(Vec::len), Some(1), "{found}");
        for url in [
            format!("{issues}?status=signed_of"),
            "/api/issues?status=signed_of".to_string(),
        ] {
            let typo = request
                .get(&url)
                .add_header(writer.0.clone(), writer.1.clone())
                .await;
            assert_eq!(typo.status_code(), 422, "{url}");
        }

        // The register counts the organization's status, and the audit trail records each change.
        let register: Value = request
            .get("/api/issues")
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        let text = register.to_string();
        assert!(text.contains("signed_off"), "{text}");
        let audited = ctx
            .db
            .query_one_raw(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Postgres,
                format!(
                    "SELECT count(*)::bigint AS n FROM audit_logs WHERE action = 'issue_changed' \
                     AND entity_pid = '{pid}'"
                ),
            ))
            .await
            .expect("audit query")
            .expect("audit row");
        assert_eq!(audited.try_get::<i64>("", "n").ok(), Some(3));
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_retired_type_stays_on_its_issues_and_a_withdrawn_workflow_strands_nothing() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let plan = new_plan(&request, &writer, "Retired").await;
        let issues = format!("/api/plans/{plan}/issues");

        let made: Value = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Plan's own", None, Some(&plan)))
            .await
            .json();
        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Keeps its type", "kind": "question" }))
            .await
            .json();
        assert_eq!(raised["status"], "new", "{raised}");
        let pid = raised["pid"].as_str().expect("issue pid");

        let retired = request
            .put("/api/issue-types/question")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "label": "Question", "position": 3, "retired": true }))
            .await;
        assert_eq!(retired.status_code(), 200, "{}", retired.text());

        // It cannot be chosen again, and it is not in the default list, but its issue still saves.
        let refused = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "New one", "kind": "question" }))
            .await;
        assert_eq!(refused.status_code(), 422);
        let kept = request
            .put(&format!("{issues}/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Keeps its type, renamed", "kind": "question" }))
            .await;
        assert_eq!(kept.status_code(), 200, "{}", kept.text());

        // Its workflow goes: the status stays, keeps its meaning, and the issue can move into the built-in vocabulary.
        let wf = made["pid"].as_str().expect("workflow pid");
        let gone = request
            .delete(&format!("/api/workflows/{wf}"))
            .add_header(admin.0.clone(), admin.1.clone())
            .await;
        assert_eq!(gone.status_code(), 200, "{}", gone.text());
        let same: Value = request
            .put(&format!("{issues}/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Still here", "kind": "question" }))
            .await
            .json();
        assert_eq!(same["status"], "new", "{same}");
        assert_eq!(same["status_category"], "todo");
        let moved: Value = request
            .put(&format!("{issues}/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Still here", "kind": "question", "status": "in_progress" }))
            .await
            .json();
        assert_eq!(moved["status"], "in_progress", "{moved}");
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn with_nothing_configured_an_issue_behaves_as_before() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let plan = new_plan(&request, &writer, "Plain").await;
        let issues = format!("/api/plans/{plan}/issues");
        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Default" }))
            .await
            .json();
        assert_eq!(raised["kind"], "concern", "{raised}");
        assert_eq!(raised["status"], "open");
        assert_eq!(raised["status_category"], "todo");
        let pid = raised["pid"].as_str().expect("issue pid");
        // Any move is allowed, and closing still needs a resolution.
        let refused = request
            .put(&format!("{issues}/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Default", "status": "closed" }))
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        let closed: Value = request
            .put(&format!("{issues}/{pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Default", "status": "closed", "resolution": "Done" }))
            .await
            .json();
        assert_eq!(closed["status_category"], "done", "{closed}");
        let unknown = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Bad", "kind": "nonsense" }))
            .await;
        assert_eq!(unknown.status_code(), 422);
    })
    .await;
}

/// The bypass an independent review found: a workflow's gate must hold when an issue changes type.
#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn changing_an_issues_type_cannot_get_round_a_gate() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let manager = bearer(
            &kid,
            MANAGER,
            &[("access", "write"), ("role", "change-manager")],
        );
        for (path, body) in [
            (
                "/api/organization-roles",
                json!({ "key": "change-manager", "label": "Change manager" }),
            ),
            (
                "/api/issue-types",
                json!({ "key": "incident", "label": "Incident" }),
            ),
        ] {
            let ok = request
                .post(path)
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&body)
                .await;
            assert_eq!(ok.status_code(), 201, "{}", ok.text());
        }
        let ok = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Incidents", Some("incident"), None))
            .await;
        assert_eq!(ok.status_code(), 200, "{}", ok.text());
        let plan = new_plan(&request, &writer, "Gated").await;
        let issues = format!("/api/plans/{plan}/issues");

        // An incident reaches `fixed`, one step short of the sign-off.
        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Gate", "kind": "incident" }))
            .await
            .json();
        let pid = raised["pid"].as_str().expect("issue pid");
        let one = format!("{issues}/{pid}");
        for status in ["with_supplier", "fixed"] {
            let moved = request
                .put(&one)
                .add_header(writer.0.clone(), writer.1.clone())
                .json(&json!({ "title": "Gate", "kind": "incident", "status": status }))
                .await;
            assert_eq!(moved.status_code(), 200, "{}", moved.text());
        }

        // Out to a type with no gate, in one request.
        let closed = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Gate", "kind": "bug", "status": "closed", "resolution": "x" }))
            .await;
        assert_eq!(closed.status_code(), 403, "{}", closed.text());

        // Out to a type with no gate, then back in, past the gate.
        let out = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Gate", "kind": "bug", "status": "in_progress" }))
            .await;
        assert_eq!(out.status_code(), 403, "{}", out.text());

        // In from a type with no gate: raised as finished, then made an incident.
        let bug: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Raised done", "kind": "bug", "status": "resolved" }))
            .await
            .json();
        let bug_pid = bug["pid"].as_str().expect("bug pid");
        let into = request
            .put(&format!("{issues}/{bug_pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(
                &json!({ "title": "Raised done", "kind": "incident", "status": "signed_off",
                           "resolution": "x" }),
            )
            .await;
        assert_eq!(into.status_code(), 403, "{}", into.text());

        // A body that leaves the type out does not change it.
        let same: Value = request
            .put(&one)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Renamed" }))
            .await
            .json();
        assert_eq!(same["kind"], "incident", "{same}");
        assert_eq!(same["status"], "fixed", "{same}");

        // An administrator may retype, and a status outside the workflow still cannot skip its role.
        let retyped: Value = request
            .put(&format!("{issues}/{bug_pid}"))
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "title": "Raised done", "kind": "incident" }))
            .await
            .json();
        assert_eq!(retyped["kind"], "incident", "{retyped}");
        assert_eq!(retyped["status"], "resolved", "{retyped}");
        let body = json!({ "title": "Raised done", "kind": "incident", "status": "signed_off",
                           "resolution": "x" });
        let skipped = request
            .put(&format!("{issues}/{bug_pid}"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&body)
            .await;
        assert_eq!(skipped.status_code(), 403, "{}", skipped.text());
        let signed = request
            .put(&format!("{issues}/{bug_pid}"))
            .add_header(manager.0.clone(), manager.1.clone())
            .json(&body)
            .await;
        assert_eq!(signed.status_code(), 200, "{}", signed.text());
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn the_catalogues_and_workflows_keep_what_they_should() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let owner = bearer(&kid, WRITER, &[("access", "write")]);
        let stranger = bearer(&kid, MANAGER, &[("access", "write")]);

        // A shape the issue table could not hold is refused in words, not by a database error.
        let mut closed_is_not_terminal = triage_workflow("Shape", None, None);
        closed_is_not_terminal["states"][2]["key"] = json!("closed");
        closed_is_not_terminal["transitions"] = json!([]);
        let mut starts_finished = triage_workflow("Shape", None, None);
        starts_finished["states"][0]["category"] = json!("done");
        for body in [closed_is_not_terminal, starts_finished] {
            let refused = request
                .post("/api/workflows")
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&body)
                .await;
            assert_eq!(refused.status_code(), 422, "{}", refused.text());
        }

        // Renaming a retired type does not restore it, or move it.
        for body in [
            json!({ "label": "Question", "position": 7, "retired": true }),
            json!({ "label": "Query" }),
        ] {
            let ok = request
                .put("/api/issue-types/question")
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&body)
                .await;
            assert_eq!(ok.status_code(), 200, "{}", ok.text());
        }
        let all: Value = request
            .get("/api/issue-types?include_retired=true")
            .add_header(owner.0.clone(), owner.1.clone())
            .await
            .json();
        let question = all
            .as_array()
            .and_then(|t| t.iter().find(|x| x["key"] == "question"))
            .expect("question");
        assert_eq!(question["retired"], true, "{question}");
        assert_eq!(question["position"], 7, "{question}");
        assert_eq!(question["label"], "Query", "{question}");

        // A private plan's workflow and options stay private, and the audit trail says which workflow changed.
        let plan = new_plan(&request, &owner, "Private").await;
        let private = request
            .put(&format!("/api/plans/{plan}/access"))
            .add_header(owner.0.clone(), owner.1.clone())
            .json(&json!({ "private": true, "members": [] }))
            .await;
        assert_eq!(private.status_code(), 200, "{}", private.text());
        let made: Value = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Secret", None, Some(&plan)))
            .await
            .json();
        let wf = made["pid"].as_str().expect("workflow pid").to_string();
        let hidden = request
            .get(&format!("/api/plans/{plan}/issues/options"))
            .add_header(stranger.0.clone(), stranger.1.clone())
            .await;
        assert_eq!(hidden.status_code(), 404, "{}", hidden.text());
        let seen = request
            .get(&format!("/api/plans/{plan}/issues/options"))
            .add_header(owner.0.clone(), owner.1.clone())
            .await;
        assert_eq!(seen.status_code(), 200, "{}", seen.text());
        let listed = |who: Header| {
            let request = &request;
            async move {
                request
                    .get("/api/workflows?limit=200")
                    .add_header(who.0.clone(), who.1.clone())
                    .await
                    .text()
            }
        };
        assert!(
            !listed(stranger.clone()).await.contains(&wf),
            "shown to a stranger"
        );
        assert!(
            listed(owner.clone()).await.contains(&wf),
            "hidden from its member"
        );
        let gone = request
            .delete(&format!("/api/workflows/{wf}"))
            .add_header(admin.0.clone(), admin.1.clone())
            .await;
        assert_eq!(gone.status_code(), 200, "{}", gone.text());
        for action in ["workflow_registered", "workflow_withdrawn"] {
            let row = ctx
                .db
                .query_one_raw(sea_orm::Statement::from_string(
                    sea_orm::DbBackend::Postgres,
                    format!(
                        "SELECT count(*)::bigint AS n FROM audit_logs WHERE action = '{action}' \
                         AND entity_pid = '{plan}' AND snapshot::text LIKE '%{wf}%' \
                         AND snapshot::text LIKE '%change-manager%'"
                    ),
                ))
                .await
                .expect("audit query")
                .expect("audit row");
            assert_eq!(row.try_get::<i64>("", "n").ok(), Some(1), "{action}");
        }
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_status_filter_confirms_no_word_only_a_hidden_plan_uses() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let outsider = bearer(&kid, WRITER, &[("access", "write")]);
        let plan = new_plan(&request, &admin, "Hidden words").await;
        let open = new_plan(&request, &admin, "Open plan").await;
        let mut body = triage_workflow("Hidden", None, Some(&plan));
        body["states"][1]["key"] = json!("project_nightingale");
        body["transitions"] = json!([]);
        let made = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&body)
            .await;
        assert_eq!(made.status_code(), 200, "{}", made.text());
        let private = request
            .put(&format!("/api/plans/{plan}/access"))
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "private": true, "members": [] }))
            .await;
        assert_eq!(private.status_code(), 200, "{}", private.text());

        // To an outsider the hidden word and an unused word get the same answer, on both lists.
        for word in ["project_nightingale", "unused_word"] {
            for path in [
                format!("/api/issues?status={word}"),
                format!("/api/plans/{open}/issues?status={word}"),
            ] {
                let answer = request
                    .get(&path)
                    .add_header(outsider.0.clone(), outsider.1.clone())
                    .await;
                assert_eq!(answer.status_code(), 422, "{path}: {}", answer.text());
            }
        }
        // Someone who can read the plan can still filter by it, across plans and on the plan.
        for path in [
            "/api/issues?status=project_nightingale".to_string(),
            format!("/api/plans/{plan}/issues?status=project_nightingale"),
        ] {
            let answer = request
                .get(&path)
                .add_header(admin.0.clone(), admin.1.clone())
                .await;
            assert_eq!(answer.status_code(), 200, "{path}: {}", answer.text());
        }
        // Another plan's word is not this plan's.
        let elsewhere = request
            .get(&format!(
                "/api/plans/{open}/issues?status=project_nightingale"
            ))
            .add_header(admin.0.clone(), admin.1.clone())
            .await;
        assert_eq!(elsewhere.status_code(), 422);
    })
    .await;
}

/// Sends `body` to `path` as `who`, and returns the status.
async fn put_status(
    request: &axum_test::TestServer,
    who: &Header,
    path: &str,
    body: &Value,
) -> u16 {
    request
        .put(path)
        .add_header(who.0.clone(), who.1.clone())
        .json(body)
        .await
        .status_code()
        .as_u16()
}

async fn add_member(
    request: &axum_test::TestServer,
    admin: &Header,
    plan: &str,
    person: &str,
    role: &str,
) -> Value {
    let added = request
        .post(&format!("/api/plans/{plan}/role-members"))
        .add_header(admin.0.clone(), admin.1.clone())
        .json(&json!({ "person_ref": person, "role_key": role }))
        .await;
    assert_eq!(added.status_code(), 201, "{}", added.text());
    added.json()
}

async fn add_role(
    request: &axum_test::TestServer,
    admin: &Header,
    key: &str,
    permissions: &[&str],
) {
    let added = request
        .post("/api/organization-roles")
        .add_header(admin.0.clone(), admin.1.clone())
        .json(&json!({ "key": key, "label": key, "permissions": permissions }))
        .await;
    assert_eq!(added.status_code(), 201, "{}", added.text());
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn roles_on_a_plan_decide_what_its_members_may_do_to_issues() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let member = bearer(&kid, WRITER, &[("access", "write")]);
        let other = bearer(&kid, MANAGER, &[("access", "write")]);
        add_role(&request, &admin, "triager", &["issue_add", "issue_edit"]).await;
        add_role(&request, &admin, "closer", &["issue_move"]).await;
        let plan = new_plan(&request, &admin, "Governed").await;
        let free = new_plan(&request, &admin, "Ungoverned").await;
        let issues = format!("/api/plans/{plan}/issues");

        // A word outside the vocabulary is refused, and a role takes its permissions from the catalogue.
        let bad = request
            .post("/api/organization-roles")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "key": "wizard", "label": "Wizard", "permissions": ["fly"] }))
            .await;
        assert_eq!(bad.status_code(), 422, "{}", bad.text());

        // With no members nothing is enforced, so this is everyone's plan as it always was.
        let before: Value = request
            .post(&issues)
            .add_header(other.0.clone(), other.1.clone())
            .json(&json!({ "title": "Before members" }))
            .await
            .json();
        let issue = format!("{issues}/{}", before["pid"].as_str().expect("pid"));

        // Only an administrator changes membership.
        let refused = request
            .post(&format!("/api/plans/{plan}/role-members"))
            .add_header(member.0.clone(), member.1.clone())
            .json(&json!({ "person_ref": WRITER, "role_key": "triager" }))
            .await;
        assert_eq!(refused.status_code(), 403);

        // The first member turns enforcement on: the person with no role keeps reading and loses the writes.
        let held = add_member(&request, &admin, &plan, WRITER, "triager").await;
        let again = request
            .post(&format!("/api/plans/{plan}/role-members"))
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "person_ref": WRITER, "role_key": "triager" }))
            .await;
        assert_eq!(again.status_code(), 409);
        for bad in [
            json!({ "person_ref": "not-a-uuid", "role_key": "triager" }),
            json!({ "person_ref": MANAGER, "role_key": "nobody" }),
        ] {
            let refused = request
                .post(&format!("/api/plans/{plan}/role-members"))
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&bad)
                .await;
            assert_eq!(refused.status_code(), 422, "{bad}");
        }
        let read = request
            .get(&issues)
            .add_header(other.0.clone(), other.1.clone())
            .await;
        assert_eq!(read.status_code(), 200);
        let refused = request
            .post(&issues)
            .add_header(other.0.clone(), other.1.clone())
            .json(&json!({ "title": "No role" }))
            .await;
        assert_eq!(refused.status_code(), 403, "{}", refused.text());
        assert_eq!(
            put_status(&request, &other, &issue, &json!({ "title": "Changed" })).await,
            403
        );
        let refused = request
            .delete(&issue)
            .add_header(other.0.clone(), other.1.clone())
            .await;
        assert_eq!(refused.status_code(), 403);
        // Another plan is not affected.
        let elsewhere = request
            .post(&format!("/api/plans/{free}/issues"))
            .add_header(other.0.clone(), other.1.clone())
            .json(&json!({ "title": "Free" }))
            .await;
        assert_eq!(elsewhere.status_code(), 200);

        // The role's permissions, and no others.
        assert_eq!(
            put_status(&request, &member, &issue, &json!({ "title": "Edited" })).await,
            200
        );
        let body = json!({ "title": "Edited", "status": "in_progress" });
        assert_eq!(put_status(&request, &member, &issue, &body).await, 403);
        let refused = request
            .delete(&issue)
            .add_header(member.0.clone(), member.1.clone())
            .await;
        assert_eq!(refused.status_code(), 403);

        // Several roles combine, and the options say what the form may offer.
        add_member(&request, &admin, &plan, WRITER, "closer").await;
        assert_eq!(put_status(&request, &member, &issue, &body).await, 200);
        let options: Value = request
            .get(&format!("{issues}/options"))
            .add_header(member.0.clone(), member.1.clone())
            .await
            .json();
        assert_eq!(options["enforced"], true);
        assert_eq!(
            options["caller_permissions"],
            json!(["issue_add", "issue_edit", "issue_move"])
        );
        assert_eq!(options["caller_roles"], json!(["closer", "triager"]));

        // A role carried by the token counts too, and an administrator passes without one.
        let carried = bearer(&kid, MANAGER, &[("access", "write"), ("role", "triager")]);
        let raised = request
            .post(&issues)
            .add_header(carried.0.clone(), carried.1.clone())
            .json(&json!({ "title": "By token role" }))
            .await;
        assert_eq!(raised.status_code(), 200, "{}", raised.text());
        let by_admin = request
            .post(&issues)
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "title": "By administrator" }))
            .await;
        assert_eq!(by_admin.status_code(), 200);

        // **The token stays the ceiling:** a member with a read-only token still cannot write.
        let read_only = bearer(&kid, WRITER, &[("access", "read")]);
        let refused = request
            .post(&issues)
            .add_header(read_only.0.clone(), read_only.1.clone())
            .json(&json!({ "title": "Read only" }))
            .await;
        assert_eq!(refused.status_code(), 403, "{}", refused.text());

        // A retired role carries nothing, on the plan or in a token.
        let retired = put_status(
            &request,
            &admin,
            "/api/organization-roles/triager",
            &json!({ "label": "triager", "retired": true }),
        )
        .await;
        assert_eq!(retired, 200);
        assert_eq!(
            put_status(
                &request,
                &member,
                &issue,
                &json!({ "title": "Too late", "status": "in_progress" })
            )
            .await,
            403
        );
        let refused = request
            .post(&issues)
            .add_header(carried.0.clone(), carried.1.clone())
            .json(&json!({ "title": "Retired" }))
            .await;
        assert_eq!(refused.status_code(), 403);
        let restored = put_status(
            &request,
            &admin,
            "/api/organization-roles/triager",
            &json!({ "label": "triager", "retired": false }),
        )
        .await;
        assert_eq!(restored, 200);

        // Removing the last member turns enforcement off again, and both changes are in the audit trail.
        let listed: Value = request
            .get(&format!("/api/plans/{plan}/role-members"))
            .add_header(other.0.clone(), other.1.clone())
            .await
            .json();
        assert_eq!(listed["enforced"], true);
        assert_eq!(listed["members"].as_array().map(Vec::len), Some(2));
        for m in listed["members"].as_array().expect("members") {
            let path = format!(
                "/api/plans/{plan}/role-members/{}",
                m["pid"].as_str().expect("pid")
            );
            let refused = request
                .delete(&path)
                .add_header(member.0.clone(), member.1.clone())
                .await;
            assert_eq!(refused.status_code(), 403);
            let gone = request
                .delete(&path)
                .add_header(admin.0.clone(), admin.1.clone())
                .await;
            assert_eq!(gone.status_code(), 200);
        }
        let open_again = request
            .post(&issues)
            .add_header(other.0.clone(), other.1.clone())
            .json(&json!({ "title": "Open again" }))
            .await;
        assert_eq!(open_again.status_code(), 200);
        let audit: Value = request
            .get(&format!("/api/plans/{plan}/audit"))
            .add_header(admin.0.clone(), admin.1.clone())
            .await
            .json();
        let text = audit.to_string();
        assert!(text.contains("plan_role_added"), "{text}");
        assert!(text.contains("plan_role_removed"), "{text}");
        assert!(held["added_by"] == ADMIN, "{held}");
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_role_held_on_one_plan_opens_that_plans_gates_and_no_others_and_no_private_plan() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let member = bearer(&kid, WRITER, &[("access", "write")]);
        add_role(&request, &admin, "change-manager", &["issue_move"]).await;
        let gated = new_plan(&request, &admin, "Gated").await;
        let elsewhere = new_plan(&request, &admin, "Gated elsewhere").await;
        let hidden = new_plan(&request, &admin, "Hidden").await;
        let made = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&triage_workflow("Everywhere", None, None))
            .await;
        assert_eq!(made.status_code(), 200, "{}", made.text());

        // Walks an issue up to the gate, as an administrator.
        let mut paths = Vec::new();
        for plan in [&gated, &elsewhere] {
            let issues = format!("/api/plans/{plan}/issues");
            let raised: Value = request
                .post(&issues)
                .add_header(admin.0.clone(), admin.1.clone())
                .json(&json!({ "title": "Gate" }))
                .await
                .json();
            let path = format!("{issues}/{}", raised["pid"].as_str().expect("pid"));
            for status in ["with_supplier", "fixed"] {
                let body = json!({ "title": "Gate", "status": status });
                assert_eq!(put_status(&request, &admin, &path, &body).await, 200);
            }
            paths.push(path);
        }
        add_member(&request, &admin, &gated, WRITER, "change-manager").await;
        let close = json!({ "title": "Gate", "status": "signed_off", "resolution": "Done" });
        assert_eq!(put_status(&request, &member, &paths[0], &close).await, 200);
        assert_eq!(
            put_status(&request, &member, &paths[1], &close).await,
            403,
            "the role is held on the other plan only"
        );

        // A role does not open a private plan, so a membership there is refused rather than left to do nothing.
        let private = put_status(
            &request,
            &admin,
            &format!("/api/plans/{hidden}/access"),
            &json!({ "private": true, "members": [] }),
        )
        .await;
        assert_eq!(private, 200);
        let refused = request
            .post(&format!("/api/plans/{hidden}/role-members"))
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "person_ref": WRITER, "role_key": "change-manager" }))
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        let unseen = request
            .get(&format!("/api/plans/{hidden}/role-members"))
            .add_header(member.0.clone(), member.1.clone())
            .await;
        assert_eq!(unseen.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_status_can_require_a_field_or_lock_one_for_named_roles() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let guest = bearer(
            &kid,
            MANAGER,
            &[("access", "write"), ("role", "contractor")],
        );
        let plan = new_plan(&request, &admin, "Field rules").await;
        let issues = format!("/api/plans/{plan}/issues");
        let mut body = triage_workflow("Rules", None, Some(&plan));
        body["transitions"] = json!([]);
        body["states"][0]["field_rules"] = json!([
            { "field": "description", "rule": "required" },
        ]);
        body["states"][1]["field_rules"] = json!([
            { "field": "owner_ref", "rule": "required" },
            { "field": "due_date", "rule": "required", "roles": ["contractor"] },
            { "field": "title", "rule": "read_only" },
            { "field": "priority", "rule": "read_only", "roles": ["contractor"] },
        ]);

        // A field outside the list and a rule on a task workflow are refused.
        let mut bad = body.clone();
        bad["states"][1]["field_rules"] = json!([{ "field": "colour", "rule": "required" }]);
        let refused = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&bad)
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        let mut task = body.clone();
        task["applies_to"] = json!("task");
        task["plan_pid"] = json!(new_plan(&request, &admin, "Task").await);
        let refused = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&task)
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());

        let made = request
            .post("/api/workflows")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&body)
            .await;
        assert_eq!(made.status_code(), 200, "{}", made.text());

        // The initial state's rule binds a person raising an issue; a save names what is missing.
        let refused = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "No description" }))
            .await;
        assert_eq!(refused.status_code(), 422);
        assert!(refused.text().contains("description"), "{}", refused.text());
        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Rules", "description": "Why" }))
            .await
            .json();
        let issue = format!("{issues}/{}", raised["pid"].as_str().expect("pid"));

        // Entering a state needs what it requires, for whoever the rule binds.
        let base = json!({ "title": "Rules", "description": "Why", "status": "with_supplier" });
        assert_eq!(put_status(&request, &writer, &issue, &base).await, 422);
        let mut owned = base.clone();
        owned["owner_ref"] = json!("person:a");
        assert_eq!(
            put_status(&request, &guest, &issue, &owned).await,
            422,
            "a contractor must also give a due date"
        );
        assert_eq!(put_status(&request, &writer, &issue, &owned).await, 200);

        // Read-only holds in that state: for everyone, or for the roles named. A blank is a change too.
        let mut retitled = owned.clone();
        retitled["title"] = json!("Changed");
        assert_eq!(put_status(&request, &writer, &issue, &retitled).await, 403);
        let mut prioritised = owned.clone();
        prioritised["priority"] = json!("P1");
        assert_eq!(
            put_status(&request, &writer, &issue, &prioritised).await,
            200
        );
        prioritised["priority"] = json!("P2");
        prioritised["due_date"] = json!("2099-01-01");
        assert_eq!(
            put_status(&request, &guest, &issue, &prioritised).await,
            403,
            "read-only for the contractor role"
        );
        prioritised["priority"] = json!("P1");
        assert_eq!(
            put_status(&request, &guest, &issue, &prioritised).await,
            200
        );
        // An administrator is not bound by read-only.
        prioritised["title"] = json!("Changed");
        assert_eq!(
            put_status(&request, &admin, &issue, &prioritised).await,
            200
        );

        // A type change cannot leave a state's rules behind.
        prioritised["kind"] = json!("bug");
        assert_eq!(
            put_status(&request, &writer, &issue, &prioritised).await,
            403
        );
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
#[ignore = "requires PostgreSQL (config/test.yaml)"]
async fn a_plan_can_limit_itself_to_some_issue_types() {
    let (keys, kid) = keys_and_kid();
    set_up(&keys);
    request::<App, _, _>(|request, ctx| async move {
        clean(&ctx).await;
        let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
        let writer = bearer(&kid, WRITER, &[("access", "write")]);
        let plan = new_plan(&request, &admin, "Few types").await;
        let free = new_plan(&request, &admin, "All types").await;
        let chooser = format!("/api/plans/{plan}/issue-types");
        let issues = format!("/api/plans/{plan}/issues");

        let raised: Value = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "A question", "kind": "question" }))
            .await
            .json();
        let issue = format!("{issues}/{}", raised["pid"].as_str().expect("pid"));

        let choice = json!({ "types": ["concern", "bug"] });
        assert_eq!(put_status(&request, &writer, &chooser, &choice).await, 403);
        assert_eq!(
            put_status(
                &request,
                &admin,
                &chooser,
                &json!({ "types": ["nonsense"] })
            )
            .await,
            422
        );
        assert_eq!(put_status(&request, &admin, &chooser, &choice).await, 200);
        let read: Value = request
            .get(&chooser)
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        assert_eq!(read["types"], json!(["bug", "concern"]));
        assert_eq!(read["limited"], true);

        // Only what the plan offers can be chosen, on this plan only.
        let refused = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Off the list", "kind": "risk" }))
            .await;
        assert_eq!(refused.status_code(), 422, "{}", refused.text());
        let allowed = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "On the list", "kind": "bug" }))
            .await;
        assert_eq!(allowed.status_code(), 200, "{}", allowed.text());
        let elsewhere = request
            .post(&format!("/api/plans/{free}/issues"))
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Elsewhere", "kind": "risk" }))
            .await;
        assert_eq!(elsewhere.status_code(), 200);

        // An issue that already carries a type left out keeps it and can be saved, but cannot be changed to another.
        let keep = json!({ "title": "Still a question" });
        assert_eq!(put_status(&request, &writer, &issue, &keep).await, 200);
        let retype = json!({ "title": "Still a question", "kind": "risk" });
        assert_eq!(put_status(&request, &writer, &issue, &retype).await, 422);

        // The options mark what can be chosen, and an empty list offers everything again.
        let options: Value = request
            .get(&format!("{issues}/options"))
            .add_header(writer.0.clone(), writer.1.clone())
            .await
            .json();
        let offered: Vec<&str> = options["types"]
            .as_array()
            .expect("types")
            .iter()
            .filter(|t| t["offered"] == true)
            .filter_map(|t| t["key"].as_str())
            .collect();
        assert_eq!(offered, ["bug", "concern"]);
        assert_eq!(
            options["types"].as_array().map(Vec::len),
            Some(6),
            "the rest stay, so an issue can show its type's name"
        );
        assert_eq!(
            put_status(&request, &admin, &chooser, &json!({ "types": [] })).await,
            200
        );
        let open = request
            .post(&issues)
            .add_header(writer.0.clone(), writer.1.clone())
            .json(&json!({ "title": "Any type", "kind": "risk" }))
            .await;
        assert_eq!(open.status_code(), 200);
    })
    .await;
}
