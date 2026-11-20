//! The jobs the service schedules for itself (NC-5): two schedulers racing for a job give one run, a run is
//! recorded and counted, and with no job enabled nothing runs.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::scheduled_job_runs;
use project_portfolio_management::scheduled_jobs::{Job, Ran, run_enabled, run_job};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn two_schedulers_run_a_job_once_and_a_disabled_scheduler_runs_nothing() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        // Nothing is enabled in the environment, so the ticker's pass runs nothing.
        run_enabled(&ctx).await;
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM scheduled_job_runs")
                .fetch_one(project_portfolio_management::db::pool(&ctx))
                .await
                .unwrap(),
            0
        );

        // Two schedulers reach for the same job at the same moment: one runs it, one is told it has run.
        let now = project_portfolio_management::datetime::Timestamp::now();
        let (a, b) = tokio::join!(
            run_job(&ctx, Job::ExceptionBreaches, now),
            run_job(&ctx, Job::ExceptionBreaches, now)
        );
        let (a, b) = (a.unwrap(), b.unwrap());
        let ran = [&a, &b].iter().filter(|r| matches!(r, Ran::Ok(_))).count();
        let skipped = [&a, &b]
            .iter()
            .filter(|r| matches!(r, Ran::Skipped))
            .count();
        assert_eq!((ran, skipped), (1, 1), "{a:?} {b:?}");

        let rows =
            sqlx::query_as::<_, scheduled_job_runs::Model>("SELECT * FROM scheduled_job_runs")
                .fetch_all(project_portfolio_management::db::pool(&ctx))
                .await
                .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            (rows[0].job.as_str(), rows[0].outcome.as_str()),
            ("exception_breaches", "ok")
        );
        assert!(rows[0].finished_at.is_some());

        // Started and finished are both audited, as the scheduler.
        for action in ["scheduled_job_started", "scheduled_job_finished"] {
            let n = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM audit_logs WHERE action = $1 AND entity_pid = $2",
            )
            .bind(action)
            .bind(rows[0].pid)
            .fetch_one(project_portfolio_management::db::pool(&ctx))
            .await
            .unwrap();
            assert_eq!(n, 1, "{action}");
        }

        // It is counted.
        let text = project_portfolio_management::metrics::Metrics::global().render();
        assert!(
            text.contains(r#"scheduled_job_runs_total{job="exception_breaches",outcome="ok"} 1"#),
            "{text}"
        );

        // Within its interval it does not run again; a different job is not held back by it.
        assert_eq!(
            run_job(&ctx, Job::ExceptionBreaches, now).await.unwrap(),
            Ran::Skipped
        );
        assert!(matches!(
            run_job(&ctx, Job::ThresholdSweep, now).await.unwrap(),
            Ran::Ok(_)
        ));
        // After the interval it runs again.
        let later = now
            + project_portfolio_management::datetime::Duration::minutes(
                Job::ExceptionBreaches.interval_minutes() + 1,
            );
        assert!(matches!(
            run_job(&ctx, Job::ExceptionBreaches, later).await.unwrap(),
            Ran::Ok(_)
        ));
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
// Keep this after the exact metrics assertion above: metrics are process-global.
// Exercise every opt-in maintenance job against an empty throwaway estate.
async fn z_every_scheduled_job_runs_once_and_an_empty_estate_refreshes_flow_metrics() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let flow = project_portfolio_management::flow_metrics::refresh_once(&ctx)
            .await
            .expect("refresh an empty estate");
        assert!(flow.rows.is_empty());
        let first_epoch = project_portfolio_management::response_cache::epoch(&ctx).await;
        assert_eq!(
            project_portfolio_management::response_cache::epoch(&ctx).await,
            first_epoch
        );
        project_portfolio_management::response_cache::bump(&ctx).await;
        assert_ne!(
            project_portfolio_management::response_cache::epoch(&ctx).await,
            first_epoch
        );

        let now = project_portfolio_management::datetime::Timestamp::now();
        for &job in project_portfolio_management::scheduled_jobs::ALL {
            let first = run_job(&ctx, job, now).await.unwrap();
            assert!(matches!(first, Ran::Ok(_)), "{}: {first:?}", job.name());
            assert_eq!(
                run_job(&ctx, job, now).await.unwrap(),
                Ran::Skipped,
                "{} must not run twice inside its interval",
                job.name()
            );
        }
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn the_ticker_starts_only_when_configured_and_one_pass_runs_every_sweep() {
    use project_portfolio_management::scheduler::{spawn_with, sweep_once};
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        // Off when unset, blank, zero or junk; on for a positive number of minutes (the loop then sleeps first).
        assert!(!spawn_with(ctx.clone(), None));
        assert!(!spawn_with(ctx.clone(), Some("  ")));
        assert!(!spawn_with(ctx.clone(), Some("0")));
        assert!(!spawn_with(ctx.clone(), Some("hourly")));
        assert!(spawn_with(ctx.clone(), Some("60")));
        // One pass over a healthy database: every sweep runs and none fails, and a second pass is just as quiet.
        assert_eq!(sweep_once(&ctx).await, 0);
        assert_eq!(sweep_once(&ctx).await, 0);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn flow_metrics_read_plans_with_tasks_and_count_a_column_over_its_limit() {
    use project_portfolio_management::flow_metrics::{refresh_with, spawn_every};
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let plan = |name: &'static str| {
            let request = &request;
            async move {
                let plan: serde_json::Value = request
                    .post("/api/plans")
                    .json(&serde_json::json!({ "kind": "Project", "name": name }))
                    .await
                    .json();
                plan["pid"].as_str().expect("pid").to_string()
            }
        };
        let busy = plan("Flow with tasks").await;
        let _empty = plan("Flow without tasks").await;
        for title in ["One", "Two", "Three"] {
            request
                .post(&format!("/api/plans/{busy}/tasks"))
                .json(&serde_json::json!({ "title": title }))
                .await
                .assert_status_success();
        }

        // With no limits, and with a cap of one on the column the three tasks sit in: both passes succeed and
        // read the same plans.
        let unlimited = refresh_with(&ctx, None).await.unwrap();
        let limits = std::collections::BTreeMap::from([("todo".to_string(), 1)]);
        let capped = refresh_with(&ctx, Some(&limits)).await.unwrap();
        assert_eq!(unlimited.suppressed_plans, capped.suppressed_plans);
        assert_eq!(unlimited.rows.len(), capped.rows.len());

        // The loop refreshes at once on start, then every period.
        let ticking = spawn_every(&ctx, std::time::Duration::from_millis(20));
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        assert!(!ticking.is_finished(), "the loop keeps running");
        ticking.abort();
    })
    .await;
}
