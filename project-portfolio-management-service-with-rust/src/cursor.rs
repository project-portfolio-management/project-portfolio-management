//! The key that signs list cursors (spec §9, FR-46).
//!
//! Every list pages with `?limit=` and an opaque `?cursor=` from `axa-cursor-pagination`. A cursor is signed with this
//! key over its position and its query, so a caller cannot make one up, edit one, or carry one to another list or filter.
//! Every replica must hold the same key, or a cursor from one is refused by another.

use std::sync::{Arc, OnceLock};

use crate::prelude::*;
use axa::Environment;
use axa::cursor_pagination::{CursorKey, MIN_KEY_LEN};

/// The setting that holds the key: at least [`MIN_KEY_LEN`] bytes of secret text.
pub const CURSOR_KEY: &str = "PROJECT_PORTFOLIO_MANAGEMENT_CURSOR_KEY";

/// The cursor key this service signs with.
#[derive(Debug, Clone)]
pub struct Cursors(pub CursorKey);

impl Cursors {
    /// The key from [`CURSOR_KEY`]. Where the environment opts out of the production guards and the setting is unset,
    /// a random key for this process, so cursors stop working on a restart, which is fine for development.
    ///
    /// # Errors
    ///
    /// When the setting is shorter than [`MIN_KEY_LEN`] bytes, or unset outside development and test.
    pub fn from_env(environment: &Environment) -> Result<Self> {
        Self::from_setting(
            std::env::var(CURSOR_KEY)
                .ok()
                .filter(|key| !key.trim().is_empty()),
            environment,
        )
    }

    /// As [`Cursors::from_env`], with the setting given.
    ///
    /// # Errors
    ///
    /// As [`Cursors::from_env`].
    pub fn from_setting(setting: Option<String>, environment: &Environment) -> Result<Self> {
        match setting {
            Some(secret) => CursorKey::new(secret.as_bytes())
                .map(Self)
                .map_err(|error| Error::Message(format!("{CURSOR_KEY}: {error}"))),
            None if crate::app::opts_out_of_guards(environment) => Self::random(),
            None => Err(Error::Message(format!(
                "{CURSOR_KEY} is required in this environment: at least {MIN_KEY_LEN} bytes, the same on every replica"
            ))),
        }
    }

    /// A key no one else holds, for this process only.
    ///
    /// # Errors
    ///
    /// When the key cannot be built. Two UUIDs are 32 bytes, which is [`MIN_KEY_LEN`], so this is not expected.
    pub fn random() -> Result<Self> {
        let mut secret = Vec::with_capacity(MIN_KEY_LEN);
        secret.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
        secret.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
        CursorKey::new(&secret)
            .map(Self)
            .map_err(|error| Error::Message(format!("a random cursor key: {error}")))
    }

    /// The key this service booted with. A context built without one gets one random key for the process.
    ///
    /// # Errors
    ///
    /// When there is no booted key and a random one cannot be built (see [`Cursors::random`]).
    pub fn of(ctx: &AppContext) -> Result<Arc<Self>> {
        static FALLBACK: OnceLock<Arc<Cursors>> = OnceLock::new();
        if let Some(booted) = ctx.shared_store.get_ref::<Self>() {
            return Ok(booted);
        }
        if let Some(fallback) = FALLBACK.get() {
            return Ok(fallback.clone());
        }
        let made = Arc::new(Self::random()?);
        Ok(FALLBACK.get_or_init(|| made).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_refuses_to_start_without_a_key() {
        let error = Cursors::from_setting(None, &Environment::Production).unwrap_err();
        assert!(error.to_string().contains(CURSOR_KEY), "{error}");
    }

    #[test]
    fn a_short_key_is_refused_everywhere() {
        for environment in [Environment::Production, Environment::Development] {
            assert!(Cursors::from_setting(Some("too short".into()), &environment).is_err());
        }
    }

    #[test]
    fn development_and_test_make_their_own_key() {
        assert!(Cursors::from_setting(None, &Environment::Development).is_ok());
        assert!(Cursors::from_setting(None, &Environment::Test).is_ok());
        assert!(
            Cursors::from_setting(Some("k".repeat(MIN_KEY_LEN)), &Environment::Production).is_ok()
        );
    }
}
