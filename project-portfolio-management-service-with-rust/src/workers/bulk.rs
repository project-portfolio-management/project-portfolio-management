//! The bulk-job worker (T-8): a loco `BackgroundWorker` that runs one queued
//! import or export job to its terminal state. The work itself lives in
//! [`crate::controllers::bulk::run_job`] so it is the same code whether the
//! queue runs it in the background (production) or inline (the test profile's
//! `ForegroundBlocking`).

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The bulk-job worker.
pub struct BulkWorker {
    /// The shared application context.
    pub ctx: AppContext,
}

/// Arguments for a [`BulkWorker`] job.
#[derive(Deserialize, Debug, Serialize)]
pub struct BulkWorkerArgs {
    /// The `bulk_jobs.pid` to run.
    pub job_pid: Uuid,
}

#[async_trait]
impl BackgroundWorker<BulkWorkerArgs> for BulkWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: BulkWorkerArgs) -> Result<()> {
        crate::controllers::bulk::run_job(&self.ctx, args.job_pid).await
    }
}
