//! Visibility model helpers (PPM Phase B): `ActiveModelBehavior`
//! impls and finders for dependencies / milestones / allocations /
//! report definitions. The finders take the context and read the pool
//! ([`crate::db::pool`]).

use crate::prelude::*;
use uuid::Uuid;

use super::_entities::{allocations, milestones, plan_dependencies, report_definitions};

impl ActiveModelBehavior for plan_dependencies::ActiveModel {}
impl ActiveModelBehavior for milestones::ActiveModel {}
impl ActiveModelBehavior for allocations::ActiveModel {}
impl ActiveModelBehavior for report_definitions::ActiveModel {}

/// Every dependency edge (the cycle check and the schedule view both
/// want the full set — edge volume is operator-scale).
///
/// # Errors
///
/// When the query fails.
pub async fn all_dependencies(ctx: &AppContext) -> Result<Vec<plan_dependencies::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM plan_dependencies ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// One dependency by pid.
///
/// # Errors
///
/// [`Error::NotFound`] when absent.
pub async fn find_dependency(ctx: &AppContext, pid: Uuid) -> Result<plan_dependencies::Model> {
    sqlx::query_as("SELECT * FROM plan_dependencies WHERE pid = $1 LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)
}

/// Generate a `find_<x>` finder over an active (not soft-deleted) row.
macro_rules! find_active {
    ($fn_name:ident, $module:ident) => {
        /// Find the active row by public id.
        ///
        /// # Errors
        ///
        /// [`Error::NotFound`] when absent or soft-deleted.
        pub async fn $fn_name(ctx: &AppContext, pid: Uuid) -> Result<$module::Model> {
            sqlx::query_as(concat!(
                "SELECT * FROM ",
                stringify!($module),
                " WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
            ))
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)
        }
    };
}

find_active!(find_milestone, milestones);
find_active!(find_allocation, allocations);
find_active!(find_report, report_definitions);

/// A plan's active milestones, due-date order.
///
/// # Errors
///
/// When the query fails.
pub async fn milestones_for(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<milestones::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM milestones WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY due",
    )
    .bind(plan_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// A plan's active allocations, oldest first.
///
/// # Errors
///
/// When the query fails.
pub async fn allocations_for(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<allocations::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM allocations WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Every active allocation (the capacity rollup input).
///
/// # Errors
///
/// When the query fails.
pub async fn all_allocations(ctx: &AppContext) -> Result<Vec<allocations::Model>> {
    // A placeholder (SC-7) is role demand with no person yet: it is not a
    // person's load, so it never enters per-person capacity. In the database's
    // order, as before (no `ORDER BY`).
    Ok(sqlx::query_as(
        "SELECT * FROM allocations WHERE deleted_at IS NULL AND booking = 'committed' \
         AND person_ref NOT LIKE 'placeholder:%'",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Every live allocation, **proposed ones included** (PJ-4). The committed view
/// above is what utilization and over-allocation use: a proposal never counts.
///
/// # Errors
///
/// When the query fails.
pub async fn all_allocations_with_proposed(ctx: &AppContext) -> Result<Vec<allocations::Model>> {
    // In the database's order, as before (no `ORDER BY`).
    Ok(sqlx::query_as(
        "SELECT * FROM allocations WHERE deleted_at IS NULL AND person_ref NOT LIKE 'placeholder:%'",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// Every live allocation, proposed and **placeholders** included (SC-7): the
/// portfolio resource view's demand input.
///
/// # Errors
///
/// When the query fails.
pub async fn all_allocations_with_placeholders(
    ctx: &AppContext,
) -> Result<Vec<allocations::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM allocations WHERE deleted_at IS NULL ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}
