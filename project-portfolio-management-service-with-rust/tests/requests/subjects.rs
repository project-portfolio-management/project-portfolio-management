//! Data-subject requests (RG-6): export everything held about a person; preview and apply a
//! pseudonymisation that keeps every form of the reference consistent, keeps other people's rows,
//! reaches the protected audit trail only inside its own transaction, and is idempotent.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use serde_json::{Value, json};
use serial_test::serial;

const ADA: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";

async fn count(db: &sea_orm::DatabaseConnection, sql: &str) -> i64 {
    db.query_one_raw(Statement::from_string(
        DatabaseBackend::Postgres,
        sql.to_string(),
    ))
    .await
    .expect("query")
    .expect("row")
    .try_get::<i64>("", "n")
    .expect("n")
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_person_is_exported_and_pseudonymised_everywhere_and_nobody_else_is_touched() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let db = &ctx.db;
        // A plan with a task assigned to Ada (person: form), a risk owned by Bob, and audit rows:
        // one by Ada (bare subject), one by Bob.
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Wire it", "assignee_ref": format!("person:{ADA}") }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/risks"))
            .json(&json!({ "title": "R", "probability": 2, "impact": 2,
                           "owner_ref": format!("person:{BOB}") }))
            .await
            .assert_status_ok();
        for (actor, action) in [(ADA, "ada_did_a_thing"), (BOB, "bob_did_a_thing")] {
            db.execute_unprepared(&format!(
                "INSERT INTO audit_logs (entity_pid, action, actor, created_at, updated_at) \
                 VALUES (gen_random_uuid(), '{action}', '{actor}', now(), now())"
            ))
            .await
            .expect("audit seed");
        }

        // Export finds Ada in both forms and not Bob.
        let export: Value = request
            .get(&format!("/api/admin/subjects/person:{ADA}"))
            .await
            .json();
        let tables: Vec<&str> = export["tables"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["table"].as_str().unwrap())
            .collect();
        assert!(tables.contains(&"tasks"), "{tables:?}");
        assert!(tables.contains(&"audit_logs"), "{tables:?}");
        assert!(!tables.contains(&"risks"), "Bob's risk is not Ada's: {tables:?}");
        assert!(export["total_rows"].as_i64().unwrap() >= 2);

        // A preview changes nothing.
        let preview: Value = request
            .post(&format!("/api/admin/subjects/{ADA}/pseudonymise"))
            .await
            .json();
        assert_eq!(preview["applied"], false);
        assert!(preview["total_rows"].as_i64().unwrap() >= 2);
        let pseudonym = preview["pseudonym"].as_str().unwrap().to_string();
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM tasks WHERE assignee_ref = 'person:{ADA}'")).await,
            1
        );

        // Applying replaces both forms, keeps each form's shape, leaves Bob alone.
        let done: Value = request
            .post(&format!("/api/admin/subjects/person:{ADA}/pseudonymise?apply=true"))
            .await
            .json();
        assert_eq!(done["applied"], true);
        assert_eq!(done["pseudonym"], pseudonym.as_str());
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM tasks WHERE assignee_ref = 'person:{pseudonym}'")).await,
            1
        );
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM audit_logs WHERE actor = '{pseudonym}' AND action = 'ada_did_a_thing'")).await,
            1,
            "the audit actor is pseudonymised in its bare form"
        );
        assert_eq!(count(db, &format!("SELECT COUNT(*) AS n FROM audit_logs WHERE actor = '{ADA}'")).await, 0);
        assert_eq!(count(db, &format!("SELECT COUNT(*) AS n FROM risks WHERE owner_ref = 'person:{BOB}'")).await, 1);
        assert_eq!(count(db, &format!("SELECT COUNT(*) AS n FROM audit_logs WHERE actor = '{BOB}'")).await, 1);

        // The original reference no longer appears in the audit row that records this very act.
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM audit_logs WHERE snapshot::text LIKE '%{ADA}%' AND action LIKE 'subject_%'")).await,
            0
        );
        // The audit guard is back on: an ordinary update is still refused.
        assert!(db
            .execute_unprepared("UPDATE audit_logs SET action = 'x' WHERE action = 'bob_did_a_thing'")
            .await
            .is_err());

        // Idempotent: nothing of Ada's is left, and the pseudonym itself is refused as input.
        let again: Value = request
            .post(&format!("/api/admin/subjects/{ADA}/pseudonymise?apply=true"))
            .await
            .json();
        assert_eq!(again["total_rows"], 0);
        assert_eq!(
            request
                .post(&format!("/api/admin/subjects/{pseudonym}/pseudonymise?apply=true"))
                .await
                .status_code(),
            422
        );
        // The export of the old reference is now empty.
        let after: Value = request.get(&format!("/api/admin/subjects/{ADA}")).await.json();
        assert_eq!(after["total_rows"], 0);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_blocked_change_rolls_back_everything_and_says_so() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let db = &ctx.db;
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().unwrap().to_string();
        // Ada is assigned a task, and holds "R" on another task in a RACI row.
        let assigned: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Assigned", "assignee_ref": format!("person:{ADA}") }))
            .await
            .json();
        let assigned = assigned["pid"].as_str().unwrap().to_string();
        let shared: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Shared" }))
            .await
            .json();
        let shared = shared["pid"].as_str().unwrap().to_string();
        // Someone already holds the pseudonym Ada would become, with the same letter on the same task:
        // the change would break the one-row-per-person-and-letter rule.
        let preview: Value = request
            .post(&format!("/api/admin/subjects/{ADA}/pseudonymise"))
            .await
            .json();
        let pseudonym = preview["pseudonym"].as_str().unwrap().to_string();
        request
            .put(&format!("/api/plans/{p}/tasks/{shared}/raci"))
            .json(&json!({ "assignments": [
                { "person_ref": format!("person:{ADA}"), "roles": "R" },
                { "person_ref": format!("person:{pseudonym}"), "roles": "R" },
            ] }))
            .await
            .assert_status_ok();

        let refused = request
            .post(&format!("/api/admin/subjects/{ADA}/pseudonymise?apply=true"))
            .await;
        assert_eq!(refused.status_code(), 409, "{}", refused.text());

        // All or nothing: Ada's other rows were not changed either, and no audit row says it happened.
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM tasks WHERE pid = '{assigned}' AND assignee_ref = 'person:{ADA}'")).await,
            1
        );
        assert_eq!(
            count(db, &format!("SELECT COUNT(*) AS n FROM raci_assignments WHERE person_ref = 'person:{ADA}' AND deleted_at IS NULL")).await,
            1
        );
        assert_eq!(
            count(db, "SELECT COUNT(*) AS n FROM audit_logs WHERE action = 'subject_pseudonymised'").await,
            0
        );
    })
    .await;
}
