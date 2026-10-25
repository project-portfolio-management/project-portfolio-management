//! The jobs the service schedules for itself (NC-5): two schedulers racing for a job give one run, a run is
//! recorded and counted, and with no job enabled nothing runs.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::_entities::{audit_logs, scheduled_job_runs};
use project_portfolio_management::scheduled_jobs::{Job, Ran, run_enabled, run_job};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn two_schedulers_run_a_job_once_and_a_disabled_scheduler_runs_nothing() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        // Nothing is enabled in the environment, so the ticker's pass runs nothing.
        run_enabled(&ctx).await;
        assert_eq!(
            scheduled_job_runs::Entity::find()
                .count(&ctx.db)
                .await
                .unwrap(),
            0
        );

        // Two schedulers reach for the same job at the same moment: one runs it, one is told it has run.
        let now = chrono::Utc::now();
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

        let rows = scheduled_job_runs::Entity::find()
            .all(&ctx.db)
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
            let n = audit_logs::Entity::find()
                .filter(audit_logs::Column::Action.eq(action))
                .filter(audit_logs::Column::EntityPid.eq(rows[0].pid))
                .count(&ctx.db)
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
        let later = now + chrono::Duration::minutes(Job::ExceptionBreaches.interval_minutes() + 1);
        assert!(matches!(
            run_job(&ctx, Job::ExceptionBreaches, later).await.unwrap(),
            Ran::Ok(_)
        ));
    })
    .await;
}
