//! The `search_reindex` task's rebuild, in process: every active plan is replayed into the index page by page, a row
//! whose payload will not read is skipped and counted, and the boot-time rebuild runs only over an empty index.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::models::plans::Model as PlanModel;
use project_portfolio_management::tasks::search::{reindex, reindex_if_empty};
use project_portfolio_management_matcher::Plan;
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with: cargo test -- --ignored"]
async fn a_rebuild_replays_every_active_plan_and_skips_an_unreadable_one() {
    super::isolate_search_index();
    request::<App, _, _>(|_request, ctx| async move {
        let pool = project_portfolio_management::db::pool(&ctx);
        let mut txn = project_portfolio_management::db::begin(&ctx).await.unwrap();
        PlanModel::create(&mut txn, &Plan::new("Reindex alpha"))
            .await
            .unwrap();
        PlanModel::create(&mut txn, &Plan::new("Reindex beta"))
            .await
            .unwrap();
        let broken = PlanModel::create(&mut txn, &Plan::new("Reindex broken"))
            .await
            .unwrap();
        txn.commit().await.unwrap();
        sqlx::query("UPDATE plans SET data = '{\"not\": \"a plan\"}'::jsonb WHERE pid = $1")
            .bind(broken.pid)
            .execute(pool)
            .await
            .unwrap();
        let active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM plans WHERE deleted_at IS NULL")
            .fetch_one(pool)
            .await
            .unwrap();
        let active = usize::try_from(active).unwrap();

        // A page of one row forces the loop through every page and the empty page that ends it.
        let report = reindex(&ctx, 1).await.unwrap();
        assert_eq!(report.scanned, active);
        assert!(
            report.skipped >= 1,
            "the unreadable row is skipped: {report:?}"
        );
        assert_eq!(report.indexed + report.skipped, report.scanned);
        // A batch of 0 is read as 1, not as an endless loop of empty pages.
        assert_eq!(reindex(&ctx, 0).await.unwrap(), report);

        // The index now holds documents, so the boot-time rebuild has nothing to do.
        assert_eq!(reindex_if_empty(&ctx).await.unwrap(), None);
        // Emptied, it rebuilds, with the same counts as the task.
        project_portfolio_management::search::engine()
            .unwrap()
            .clear()
            .unwrap();
        let rebuilt = reindex_if_empty(&ctx)
            .await
            .unwrap()
            .expect("an empty index over plans is rebuilt");
        assert_eq!(rebuilt.scanned, report.scanned);
        assert_eq!(rebuilt.indexed, report.indexed);

        sqlx::query("UPDATE plans SET deleted_at = now(), active = false WHERE pid = $1")
            .bind(broken.pid)
            .execute(pool)
            .await
            .unwrap();
    })
    .await;
}
