//! Governance model helpers (PPM Phase A): the path-pid parser. The
//! finders are in the governance controller.

use crate::prelude::*;
use uuid::Uuid;

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
