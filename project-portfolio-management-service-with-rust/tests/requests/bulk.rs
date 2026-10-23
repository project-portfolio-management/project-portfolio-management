//! Bulk import / export request tests (T-8): the acceptance cases of
//! `spec/bulk-import-export.md` §10 — idempotent re-import, the per-row error
//! report with reconciling counts, a keyless duplicate held for review,
//! masked versus full export, and a zero-row export that is still audited.
//!
//! The test profile runs workers inline (`ForegroundBlocking`), so a job is
//! finished by the time its `POST` returns. `#[ignore]`d: needs PostgreSQL.

#![expect(
    clippy::items_after_statements,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]

use axum::body::Bytes;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::audit_logs;
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
        use project_portfolio_management::models::_entities::artifacts;
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
        let swept = project_portfolio_management::controllers::bulk::run_sweep(&ctx)
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

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn a_job_can_be_cancelled_keeps_what_it_finished_and_progress_never_goes_back() {
    use project_portfolio_management::controllers::bulk::run_job;
    use project_portfolio_management::models::_entities::bulk_jobs;
    use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait};
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: `#[serial]` — nothing else runs while this is set.
    unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path()) };
    request::<App, _, _>(|request, ctx| async move {
        use axum::http::StatusCode as S;
        let audits = |action: &'static str| {
            let db = ctx.db.clone();
            async move { audit_logs::Entity::find().filter(audit_logs::Column::Action.eq(action)).count(&db).await.unwrap() }
        };
        let cancel = |id: &str| {
            let (request, url) = (&request, format!("/api/plans/bulk-jobs/{id}/cancel"));
            async move { request.post(&url).await }
        };

        // Unknown and malformed ids are 404, not 500.
        cancel("00000000-0000-4000-8000-00000000ffff").await.assert_status(S::NOT_FOUND);
        cancel("not-a-uuid").await.assert_status(S::NOT_FOUND);

        // A finished job: cancelling it is a no-op, not an error, and is not audited.
        let rows: String = (0..25).map(|i| json!({ "name": format!("Plan {i}"), "kind": "Project", "code": format!("P-{i}") }).to_string()).collect::<Vec<_>>().join("\n");
        let (code, first) = import(&request, "jsonl", &rows, None, "create").await;
        assert_eq!(code, 202);
        let id = first["job_id"].as_str().unwrap().to_string();
        let done = job(&request, &id).await;
        assert_eq!((done["status"].as_str(), done["progress"].as_i64()), (Some("completed"), Some(100)), "{done}");
        let before = audits("bulk_job_cancel_requested").await + audits("bulk_job_cancelled").await;
        let r: Value = cancel(&id).await.json();
        assert_eq!((r["status"].as_str(), r["changed"].as_bool()), (Some("completed"), Some(false)));
        assert_eq!(audits("bulk_job_cancel_requested").await + audits("bulk_job_cancelled").await, before);
        assert_eq!(job(&request, &id).await["counts"]["created"], 25, "nothing was undone");

        // A queued job is cancelled before it ever runs, and the worker leaves it alone.
        let queued = bulk_jobs::ActiveModel {
            pid: ActiveValue::set(uuid::Uuid::new_v4()),
            entity: ActiveValue::set("plan".to_string()),
            kind: ActiveValue::set("import".to_string()),
            status: ActiveValue::set("queued".to_string()),
            format: ActiveValue::set("jsonl".to_string()),
            params: ActiveValue::set(json!({})),
            rows_total: ActiveValue::set(0), rows_created: ActiveValue::set(0), rows_updated: ActiveValue::set(0),
            rows_unchanged: ActiveValue::set(0), rows_queued_for_review: ActiveValue::set(0),
            rows_failed: ActiveValue::set(0), rows_omitted: ActiveValue::set(0),
            progress: ActiveValue::set(0),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();
        let qid = queued.pid.to_string();
        let r: Value = cancel(&qid).await.json();
        assert_eq!((r["status"].as_str(), r["changed"].as_bool()), (Some("cancelled"), Some(true)));
        assert_eq!(audits("bulk_job_cancelled").await, 1);
        run_job(&ctx, queued.pid).await.unwrap();
        let after = job(&request, &qid).await;
        assert_eq!((after["status"].as_str(), after["cancelled"].as_bool()), (Some("cancelled"), Some(true)), "the worker did not resurrect it");
        let again: Value = cancel(&qid).await.json();
        assert_eq!(again["changed"], false, "cancelling twice is harmless");
        assert_eq!(audits("bulk_job_cancelled").await, 1, "audited once");

        // A running job notices between rows, ends cancelled, keeps and counts what it did.
        // (Re-queue the finished import with the cancel flag already raised.)
        let plans_before = plan_count(&request).await;
        ctx.db.execute_unprepared(&format!(
            "UPDATE bulk_jobs SET status='queued', rows_total=0, rows_created=0, rows_updated=0, rows_unchanged=0, rows_queued_for_review=0, rows_failed=0, finished_at=NULL, cancel_requested_at=now() WHERE pid='{id}'"
        )).await.unwrap();
        run_job(&ctx, uuid::Uuid::parse_str(&id).unwrap()).await.unwrap();
        let c = job(&request, &id).await;
        assert_eq!(c["status"], "cancelled", "{c}");
        assert_eq!(c["counts"]["created"], 0, "it stopped before the first row");
        assert_eq!(c["rows_omitted"], 25, "and counted what it did not reach");
        assert!(c["note"].as_str().unwrap().contains("were not reached"));
        assert_eq!(plan_count(&request).await, plans_before, "no half-written rows");
        assert_eq!(audits("bulk_job_cancelled").await, 2);

        // Progress never goes backwards, whatever writes it.
        ctx.db.execute_unprepared(&format!("UPDATE bulk_jobs SET progress = 60 WHERE pid='{qid}'")).await.unwrap();
        ctx.db.execute_unprepared(&format!("UPDATE bulk_jobs SET progress = 10 WHERE pid='{qid}'")).await.unwrap();
        assert_eq!(job(&request, &qid).await["progress"], 60);
        assert!(ctx.db.execute_unprepared(&format!("UPDATE bulk_jobs SET progress = 101 WHERE pid='{qid}'")).await.is_err(), "out of range is refused");
    })
    .await;
}
