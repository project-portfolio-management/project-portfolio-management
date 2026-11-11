//! Point-in-time estate snapshots behind the board / CRO **trend**
//! views. A snapshot captures the current portfolio RAG counts, the
//! estate's open risk exposure, and the per-currency money totals into
//! `insight_snapshots`; trends read the stored series back — history
//! is only ever what was actually captured, never re-derived.
//!
//! Capture runs two ways: explicitly (`POST /api/board/snapshots`) or
//! via `spawn` — an optional in-process ticker gated by
//! `PROJECT_PORTFOLIO_MANAGEMENT_SNAPSHOT_HOURS` (unset/0 ⇒ off, the
//! default; parse failure ⇒ off with a warning).

use crate::prelude::*;

use crate::insights;
use crate::models::_entities::{budget_lines, insight_snapshots, plans, risks};

/// Capture one `estate` snapshot row from live data and insert it, on a transaction or the pool
/// ([`crate::db::Session`]).
///
/// # Errors
///
/// When a query or the insert fails (`sqlx`'s error, so an outage is `503`).
pub async fn capture_on(db: &mut impl crate::db::Session) -> Result<insight_snapshots::Model> {
    let items: Vec<plans::Model> = sqlx::query_as("SELECT * FROM plans WHERE deleted_at IS NULL")
        .fetch_all(db.ex())
        .await?;
    let risk_rows: Vec<risks::Model> =
        sqlx::query_as("SELECT * FROM risks WHERE deleted_at IS NULL")
            .fetch_all(db.ex())
            .await?;
    let budget_rows: Vec<budget_lines::Model> =
        sqlx::query_as("SELECT * FROM budget_lines WHERE deleted_at IS NULL")
            .fetch_all(db.ex())
            .await?;

    let open_exposure: i32 = risk_rows
        .iter()
        .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
        .map(|r| r.probability * r.impact)
        .sum();
    let lines: Vec<insights::MoneyLine> = budget_rows
        .iter()
        .map(|b| insights::MoneyLine {
            currency: b.currency.clone(),
            planned_minor: b.planned_minor,
            actual_minor: b.actual_minor,
        })
        .collect();
    let money = insights::variance_by_currency(&lines);
    let portfolios = items
        .iter()
        .filter(|i| i.kind.as_deref() == Some("Portfolio"))
        .count();
    let body = serde_json::json!({
        "plans": items.len(),
        "portfolios": portfolios,
        "open_exposure": open_exposure,
        "money": money,
    });
    // The columns `SeaORM`'s insert set, no more (`taken_at` is the table's default).
    Ok(sqlx::query_as(
        "INSERT INTO insight_snapshots (kind, body) VALUES ('estate', $1) RETURNING *",
    )
    .bind(body)
    .fetch_one(db.ex())
    .await?)
}

/// Start the optional snapshot ticker: every
/// `PROJECT_PORTFOLIO_MANAGEMENT_SNAPSHOT_HOURS` hours, capture one
/// estate snapshot. Unset / `0` / unparsable ⇒ no ticker (the
/// default); capture failures are warned and the loop continues.
pub fn spawn(ctx: AppContext) {
    let hours = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_SNAPSHOT_HOURS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .unwrap_or(0);
    if hours == 0 {
        return;
    }
    tokio::spawn(async move {
        let period = std::time::Duration::from_secs(hours.saturating_mul(3600));
        loop {
            tokio::time::sleep(period).await;
            if let Err(err) = capture_on(&mut crate::db::pool(&ctx)).await {
                tracing::warn!("estate snapshot capture failed: {err}");
            }
        }
    });
}
