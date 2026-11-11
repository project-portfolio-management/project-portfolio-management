//! Row — `total_project_control`. Devaux's Total Project
//! Control observations per plan. See entity spec §5.9.7 / FR-37 and
//! `spec/total-project-control/index.md`.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// ISO 4217. Rankings never cross it — this service does not
    /// convert currency anywhere.
    pub currency: String,
    pub observed_at: DateTimeWithTimeZone,
    /// The stored DIPP in basis points, which may carry TPC time-value
    /// terms that EMV alone does not. Divergence from EMV/CEC is
    /// reported as a finding, never silently resolved.
    pub total_project_control_dipp: Option<Decimal>,
    pub total_project_control_dipp_progress_index_numerator: Option<Decimal>,
    pub total_project_control_dipp_progress_index_denominator: Option<Decimal>,
    /// `GENERATED ALWAYS` in Postgres, so it cannot disagree with the
    /// two numbers beside it. Never written by this crate.
    pub total_project_control_dipp_progress_index_ratio: Option<Decimal>,
    /// Minor units. **May be negative**: a project can be worth less
    /// than nothing to finish.
    pub total_project_control_expected_monetary_value: Decimal,
    /// Minor units. Never negative (CHECK constraint).
    pub total_project_control_cost_estimate_to_complete: Decimal,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
