//! The service's dates, instants, durations and time zones: the `project-portfolio-management-datetime` crate
//! (`datetime/`, over jiff 1.0), re-exported so `crate::datetime::Timestamp` here and in the domain crate name
//! the same type.
//!
//! `golden` (a test module) pins, byte for byte, how they are written (JSON, RFC 3339 text, `Display`,
//! `Debug`, strftime patterns), how request text is read, and how date arithmetic behaves. It stays in the
//! service rather than the domain crate because it also pins a row struct's JSON (`models::_entities`).

pub use project_portfolio_management_datetime::*;

#[cfg(test)]
mod golden;
