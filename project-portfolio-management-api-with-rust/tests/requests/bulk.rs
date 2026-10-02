//! Bulk import / export request tests (T-8): the acceptance cases of
//! `spec/bulk-import-export.md` §10 — idempotent re-import, the per-row error
//! report with reconciling counts, a keyless duplicate held for review,
//! masked versus full export, and a zero-row export that is still audited.
//!
//! The test profile runs workers inline (`ForegroundBlocking`), so a job is
//! finished by the time its `POST` returns. `#[ignore]`d: needs PostgreSQL.

use axum::body::Bytes;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use project_portfolio_management_service::models::_entities::audit_logs;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde_json::{Value, json};
use serial_test::serial;

async fn import(
    request: &axum_test::TestServer,
    format: &str,
    body: &str,
    key: Option<&str>,
    on_keyless: &str,
) -> (u16, Value) {
    let mut r = request
        .post(&format!(
            "/api/plans/import?format={format}&on_keyless={on_keyless}"
        ))
        .content_type("text/plain")
        .bytes(Bytes::copy_from_slice(body.as_bytes()));
    if let Some(k) = key {
        r = r.add_header(
            axum::http::HeaderName::from_static("idempotency-key"),
            axum::http::HeaderValue::from_str(k).unwrap(),
        );
    }
    let resp = r.await;
    (resp.status_code().as_u16(), resp.json())
}

async fn job(request: &axum_test::TestServer, id: &str) -> Value {
    request.get(&format!("/api/plans/import/{id}")).await.json()
}

async fn plan_count(request: &axum_test::TestServer) -> usize {
    let list: Value = request.get("/api/plans?limit=500").await.json();
    list.as_array().unwrap().len()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn import_is_idempotent_reports_every_bad_row_and_reconciles() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: `#[serial]` — nothing else runs while this is set.
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path()) };
    request::<App, _, _>(|request, _ctx| async move {
        let before = plan_count(&request).await;
        let keyed = [
            json!({ "name": "Alpha", "kind": "Project",
                    "identifiers": [{ "scheme": "JiraProjectKey", "value": "ALPHA" }] }),
            json!({ "name": "Beta", "kind": "Project", "code": "B-1",
                    "owner_org_id": "organization:3f2c6b7e-0000-4000-8000-0000000000aa" }),
        ]
        .map(|v| v.to_string())
        .join("\n");

        // First import creates both and the counts reconcile.
        let (code, first) = import(&request, "jsonl", &keyed, Some("run-1"), "review").await;
        assert_eq!(code, 202);
        let id = first["job_id"].as_str().expect("job id").to_string();
        let j = job(&request, &id).await;
        let why = request.get(&format!("/api/plans/bulk-jobs/{id}/errors")).await.text();
        assert_eq!(j["status"], "completed", "{j}\n{why}");
        assert_eq!(j["counts"]["created"], 2);
        assert_eq!(j["reconciles"], true);
        assert_eq!(plan_count(&request).await, before + 2);

        // The same Idempotency-Key replays the original job.
        let (code, replay) = import(&request, "jsonl", &keyed, Some("run-1"), "review").await;
        assert_eq!(code, 200);
        assert_eq!(replay["job_id"], id);
        assert_eq!(replay["replayed"], true);
        assert_eq!(plan_count(&request).await, before + 2, "a replay creates nothing");

        // Re-importing the same file under a new key is idempotent: all unchanged.
        let (_, again) = import(&request, "jsonl", &keyed, Some("run-2"), "review").await;
        let j2 = job(&request, again["job_id"].as_str().unwrap()).await;
        assert_eq!(j2["counts"]["unchanged"], 2, "{j2}");
        assert_eq!(j2["counts"]["created"], 0);
        assert_eq!(plan_count(&request).await, before + 2, "no duplicates");

        // An edited keyed row updates in place rather than duplicating.
        let edited = json!({ "name": "Alpha (renamed)", "kind": "Project",
                             "identifiers": [{ "scheme": "JiraProjectKey", "value": "ALPHA" }] })
            .to_string();
        let (_, upd) = import(&request, "jsonl", &edited, None, "review").await;
        let j3 = job(&request, upd["job_id"].as_str().unwrap()).await;
        assert_eq!(j3["counts"]["updated"], 1);
        assert_eq!(plan_count(&request).await, before + 2);

        // ---- the per-row error report: one bad row never aborts the load.
        let mixed = [
            "{ not json".to_string(),
            json!({ "name": "Good one", "kind": "Project",
                    "identifiers": [{ "scheme": "JiraProjectKey", "value": "GOOD" }] }).to_string(),
            json!({ "name": "   ", "kind": "Project" }).to_string(),
            json!({ "pid": "3f2c6b7e-0000-4000-8000-000000000bad", "name": "Ghost" }).to_string(),
        ]
        .join("\n");
        let (_, m) = import(&request, "jsonl", &mixed, None, "review").await;
        let jm = job(&request, m["job_id"].as_str().unwrap()).await;
        assert_eq!(jm["status"], "completed_with_errors");
        assert_eq!(jm["counts"]["total"], 4);
        assert_eq!(jm["counts"]["created"], 1, "the good row landed");
        assert_eq!(jm["counts"]["failed"], 3);
        assert_eq!(jm["reconciles"], true);
        let report = request
            .get(&format!("/api/plans/bulk-jobs/{}/errors", m["job_id"].as_str().unwrap()))
            .await
            .text();
        assert!(report.starts_with("row_number,source_line,field,code,message\n"), "{report}");
        assert!(report.contains("1,1,,decode,"), "the bad line is row 1, line 1: {report}");
        assert!(report.contains(",pid,unknown_pid,"), "{report}");
        assert!(report.contains("validation"), "{report}");
        assert!(!report.contains("Ghost"), "the report never echoes a source value");

        // ---- CSV: JSON-in-cell, a multi-line quoted cell, an unmapped column.
        let csv = "name,kind,tags,description_extra\r\n\"Gamma, the \"\"third\"\"\nproject\",Project,\"[\"\"x\"\",\"\"y\"\"]\",oops\r\nDelta,Project,\"[\"\"z\"\"]\",\r\n";
        let (_, c) = import(&request, "csv", csv, None, "create").await;
        let jc = job(&request, c["job_id"].as_str().unwrap()).await;
        assert_eq!(jc["counts"]["total"], 2);
        assert_eq!(jc["counts"]["created"], 1, "Delta is fine; Gamma named a column the format lacks");
        assert_eq!(jc["counts"]["failed"], 1);
        let creport = request
            .get(&format!("/api/plans/bulk-jobs/{}/errors", c["job_id"].as_str().unwrap()))
            .await
            .text();
        assert!(creport.contains("description_extra,unmapped_field"), "{creport}");
        assert!(creport.contains("1,2,"), "row 1 starts on physical line 2 (after the header): {creport}");
    })
    .await;
    unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR") };
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_keyless_duplicate_is_held_for_review_and_never_created_or_merged() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path()) };
    request::<App, _, _>(|request, _ctx| async move {
        request
            .post("/api/plans")
            .json(
                &json!({ "kind": "Project", "name": "Apollo platform migration programme",
                           "keywords": ["platform", "migration"] }),
            )
            .await
            .assert_status_ok();
        let before = plan_count(&request).await;
        let row = json!({ "kind": "Project", "name": "Apollo platform migration programme",
                          "keywords": ["platform", "migration"] })
        .to_string();
        let (_, j) = import(&request, "jsonl", &row, None, "review").await;
        let jid = j["job_id"].as_str().unwrap().to_string();
        let status = job(&request, &jid).await;
        assert_eq!(status["counts"]["queued_for_review"], 1, "{status}");
        assert_eq!(status["counts"]["created"], 0);
        assert_eq!(
            plan_count(&request).await,
            before,
            "nothing was created or merged"
        );
        assert!(status["review_url"].is_string());

        let queue: Value = request
            .get(&format!("/api/plans/review-queue?job={jid}"))
            .await
            .json();
        let item = &queue[0];
        assert_eq!(item["provenance"], "import");
        assert_eq!(item["status"], "pending");
        assert!(!item["matches"].as_array().unwrap().is_empty());
        let pid = item["pid"].as_str().unwrap().to_string();

        // A reviewer decides it is new after all.
        request
            .post(&format!("/api/plans/review-queue/{pid}/confirm"))
            .await
            .assert_status_ok();
        assert_eq!(plan_count(&request).await, before + 1);
        assert_eq!(
            request
                .post(&format!("/api/plans/review-queue/{pid}/confirm"))
                .await
                .status_code(),
            409
        );
        assert_eq!(
            request
                .post(&format!("/api/plans/review-queue/{pid}/reject"))
                .await
                .status_code(),
            409
        );

        // The same keyless row with on_keyless=create skips the check.
        let (_, j2) = import(&request, "jsonl", &row, None, "create").await;
        assert_eq!(
            job(&request, j2["job_id"].as_str().unwrap()).await["counts"]["created"],
            1
        );
    })
    .await;
    unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR") };
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn export_masks_by_default_and_a_zero_row_export_is_still_audited() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path()) };
    request::<App, _, _>(|request, ctx| async move {
        request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Exportable plan",
                           "lead_ref": "person:3f2c6b7e-0000-4000-8000-000000000001",
                           "owner_org_id": "organization:3f2c6b7e-0000-4000-8000-0000000000aa",
                           "owner_org_name": "Acme Corporation Ltd" }))
            .await
            .assert_status_ok();
        let export = |body: Value| {
            let request = &request;
            async move {
                let r = request.post("/api/plans/export").json(&body).await;
                let code = r.status_code().as_u16();
                let text = r.text();
                (
                    code,
                    serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "raw": text })),
                )
            }
        };
        let download = |id: &str| {
            let (request, id) = (&request, id.to_string());
            async move {
                request
                    .get(&format!("/api/plans/export/{id}/download"))
                    .await
                    .text()
            }
        };

        // Masked (the default): the people reference is gone, the org name partly hidden.
        let (code, m) = export(json!({ "name_like": "Exportable" })).await;
        assert_eq!(code, 202, "{m}");
        let mid = m["job_id"].as_str().unwrap().to_string();
        let st: Value = request
            .get(&format!("/api/plans/export/{mid}"))
            .await
            .json();
        assert_eq!(st["status"], "completed");
        assert_eq!(st["counts"]["total"], 1);
        assert!(st["expires_at"].is_string() && st["download_url"].is_string());
        let masked = download(&mid).await;
        assert!(
            !masked.contains("person:3f2c6b7e"),
            "the lead is not exported: {masked}"
        );
        assert!(
            !masked.contains("Acme Corporation Ltd"),
            "the org name is partly hidden: {masked}"
        );
        assert!(
            masked.contains("Exportable plan"),
            "operational content is not masked"
        );

        // Full: the same plan carries its people reference.
        let (_, f) = export(json!({ "name_like": "Exportable", "masking_profile": "full" })).await;
        let full = download(f["job_id"].as_str().unwrap()).await;
        assert!(
            full.contains("person:3f2c6b7e-0000-4000-8000-000000000001"),
            "{full}"
        );
        assert!(full.contains("Acme Corporation Ltd"));

        // Deterministic: two exports of unchanged data are byte-identical.
        let (_, again) = export(json!({ "name_like": "Exportable" })).await;
        assert_eq!(download(again["job_id"].as_str().unwrap()).await, masked);

        // CSV carries the same plan, with the pid column.
        let (_, c) = export(json!({ "name_like": "Exportable", "format": "csv" })).await;
        let csv = download(c["job_id"].as_str().unwrap()).await;
        assert!(csv.starts_with("pid,kind,name,"), "{csv}");

        // A zero-row export is still audited, with its profile and row count.
        let (_, z) = export(json!({ "name_like": "zzz-matches-nothing-zzz" })).await;
        let zid = z["job_id"].as_str().unwrap().to_string();
        let zs: Value = request
            .get(&format!("/api/plans/export/{zid}"))
            .await
            .json();
        assert_eq!(zs["status"], "completed");
        assert_eq!(zs["counts"]["total"], 0);
        let audited = audit_logs::Entity::find()
            .filter(audit_logs::Column::EntityPid.eq(uuid::Uuid::parse_str(&zid).unwrap()))
            .filter(audit_logs::Column::Action.eq("bulk_export"))
            .count(&ctx.db)
            .await
            .expect("count");
        assert_eq!(audited, 1, "a zero-row export writes its audit record");

        // Bad parameters.
        for body in [
            json!({ "format": "xml" }),
            json!({ "masking_profile": "everything" }),
        ] {
            assert_eq!(export(body).await.0, 422);
        }
        // An unknown job is 404; the jobs list shows the exports.
        assert_eq!(
            request
                .get("/api/plans/export/3f2c6b7e-0000-4000-8000-000000000bad")
                .await
                .status_code(),
            404
        );
        let jobs: Value = request.get("/api/plans/bulk-jobs?kind=export").await.json();
        assert!(jobs.as_array().unwrap().len() >= 5);

        // The sweep deletes an expired artifact's file and leaves its row.
        use project_portfolio_management_service::models::_entities::artifacts;
        use sea_orm::{ActiveModelTrait, ActiveValue};
        let one = artifacts::Entity::find()
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        let mut a: artifacts::ActiveModel = one.into();
        a.expires_at = ActiveValue::set(Some(
            (chrono::Utc::now() - chrono::Duration::hours(1)).into(),
        ));
        let expired = a.update(&ctx.db).await.unwrap();
        let swept = project_portfolio_management_service::controllers::bulk::run_sweep(&ctx)
            .await
            .unwrap();
        assert!(swept["artifacts_deleted"].as_u64().unwrap() >= 1);
        let after = artifacts::Entity::find_by_id(expired.id)
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        assert!(
            after.storage_key.is_none(),
            "the file is gone, the row remains as a tombstone"
        );
    })
    .await;
    unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR") };
}
