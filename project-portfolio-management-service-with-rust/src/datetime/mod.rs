//! The service's dates, instants and durations.
//!
//! `golden` (a test module) pins, byte for byte, how they are written (JSON, RFC 3339 text, `Display`,
//! `Debug`, strftime patterns), how request text is read, and how date arithmetic behaves. It stays in the
//! service rather than the domain crate because it also pins a row struct's JSON (`models::_entities`).

#[cfg(test)]
mod golden;
