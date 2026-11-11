//! Governance model helpers (PPM Phase A): `ActiveModelBehavior`
//! impls for the proposals / gate-reviews / risks / budget-lines
//! sub-resources, and the path-pid parser. The finders moved to `sqlx`
//! in the governance controller (SQ-1).

use crate::prelude::*;
use uuid::Uuid;

use super::_entities::{budget_lines, gate_reviews, proposals, risks};

impl ActiveModelBehavior for proposals::ActiveModel {}
impl ActiveModelBehavior for gate_reviews::ActiveModel {}
impl ActiveModelBehavior for risks::ActiveModel {}
impl ActiveModelBehavior for budget_lines::ActiveModel {}

/// Parse a path `pid`, mapping failure to `404` (an unparsable pid
/// can never name a record; axa does not map
/// `ModelError::EntityNotFound`, so `Error::NotFound` directly).
///
/// # Errors
///
/// [`Error::NotFound`] when `pid` is not a UUID.
pub fn parse_pid(pid: &str) -> Result<Uuid> {
    Uuid::parse_str(pid).map_err(|_| Error::NotFound)
}
