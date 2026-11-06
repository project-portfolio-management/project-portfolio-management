//! The response cache's store: Redis where a deployment names one, shared by every replica, and memory otherwise.
//!
//! Production requires Redis ([`Cache::from_env`]). A value is stored as JSON. Every entry has an expiry, so a failed
//! write only costs a recomputed answer. [`crate::response_cache`] says what is cached and when it stops being valid.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use axa::{AppContext, Environment, Error, Result};
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

/// Most entries memory holds. Past this, expired entries go, and if that frees nothing, everything goes: the cache only
/// ever saves work, and a deployment with enough load to reach this runs Redis.
const MEMORY_ENTRIES: usize = 4096;

/// Where cached values live.
pub enum Cache {
    /// In this process only.
    Memory(Mutex<HashMap<String, (Instant, String)>>),
    /// In Redis.
    Redis(redis::aio::ConnectionManager),
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Memory(_) => "Cache::Memory",
            Self::Redis(_) => "Cache::Redis",
        })
    }
}

impl Cache {
    /// An empty cache in memory.
    #[must_use]
    pub fn memory() -> Self {
        Self::Memory(Mutex::new(HashMap::new()))
    }

    /// The cache for this environment. Production, and any name not set up for development, needs
    /// `PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL` and refuses to boot without it, so replicas cannot silently stop sharing.
    /// In development, tests and `container-dev` it is optional.
    ///
    /// # Errors
    ///
    /// When such an environment has no URL, or the URL is malformed. An unreachable Redis is not an error here:
    /// the connection is retried, and `/_readiness` reports it.
    pub fn from_env(environment: &Environment) -> Result<Self> {
        let url = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL")
            .ok()
            .filter(|u| !u.trim().is_empty());
        match url {
            Some(url) => {
                let client = redis::Client::open(url).map_err(Error::wrap)?;
                // Lazy, so a Redis that is down costs cached answers and a `503` on `/_readiness`, never a service
                // that hangs at boot.
                let manager = redis::aio::ConnectionManager::new_lazy_with_config(
                    client,
                    redis::aio::ConnectionManagerConfig::new()
                        .set_number_of_retries(1)
                        .set_connection_timeout(Some(Duration::from_secs(1)))
                        .set_response_timeout(Some(Duration::from_secs(1))),
                )
                .map_err(Error::wrap)?;
                Ok(Self::Redis(manager))
            }
            None if crate::app::opts_out_of_guards(environment) => Ok(Self::memory()),
            None => Err(Error::string(
                "PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL is required in this environment",
            )),
        }
    }

    /// The cache this service booted with. A context built without one gets a shared memory cache.
    #[must_use]
    pub fn of(ctx: &AppContext) -> Arc<Self> {
        static FALLBACK: OnceLock<Arc<Cache>> = OnceLock::new();
        ctx.shared_store
            .get_ref::<Self>()
            .unwrap_or_else(|| FALLBACK.get_or_init(|| Arc::new(Self::memory())).clone())
    }

    /// The value under `key`, if there is a live one.
    ///
    /// # Errors
    ///
    /// When Redis fails or the stored value does not decode.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let text: Option<String> = match self {
            Self::Memory(entries) => {
                let mut entries = entries.lock().unwrap_or_else(PoisonError::into_inner);
                match entries.get(key) {
                    Some((until, _)) if *until <= Instant::now() => {
                        entries.remove(key);
                        None
                    }
                    Some((_, value)) => Some(value.clone()),
                    None => None,
                }
            }
            Self::Redis(manager) => manager.clone().get(key).await.map_err(Error::wrap)?,
        };
        text.map(|t| serde_json::from_str(&t).map_err(Error::from))
            .transpose()
    }

    /// Stores `value` under `key` for `expiry`.
    ///
    /// # Errors
    ///
    /// When Redis fails or the value does not encode.
    pub async fn insert_with_expiry<T: Serialize + Sync>(
        &self,
        key: &str,
        value: &T,
        expiry: Duration,
    ) -> Result<()> {
        let text = serde_json::to_string(value)?;
        match self {
            Self::Memory(entries) => {
                let mut entries = entries.lock().unwrap_or_else(PoisonError::into_inner);
                let now = Instant::now();
                if entries.len() >= MEMORY_ENTRIES {
                    entries.retain(|_, (until, _)| *until > now);
                }
                if entries.len() >= MEMORY_ENTRIES {
                    entries.clear();
                }
                entries.insert(key.to_string(), (now + expiry, text));
                Ok(())
            }
            Self::Redis(manager) => manager
                .clone()
                .set_ex(key, text, expiry.as_secs().max(1))
                .await
                .map_err(Error::wrap),
        }
    }

    /// Whether the store answers, for `/_readiness`.
    pub async fn is_reachable(&self) -> bool {
        match self {
            Self::Memory(_) => true,
            Self::Redis(manager) => redis::cmd("PING")
                .query_async::<String>(&mut manager.clone())
                .await
                .inspect_err(|error| tracing::error!(%error, "readiness_cache_ping_error"))
                .is_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn memory_returns_what_was_stored_until_it_expires() {
        let cache = Cache::memory();
        cache
            .insert_with_expiry("a", &"one".to_string(), Duration::from_secs(30))
            .await
            .unwrap();
        cache
            .insert_with_expiry("b", &"two".to_string(), Duration::ZERO)
            .await
            .unwrap();
        assert_eq!(
            cache.get::<String>("a").await.unwrap().as_deref(),
            Some("one")
        );
        assert_eq!(cache.get::<String>("b").await.unwrap(), None);
        assert_eq!(cache.get::<String>("c").await.unwrap(), None);
    }

    #[tokio::test]
    async fn memory_stays_within_its_bound() {
        let cache = Cache::memory();
        for i in 0..=MEMORY_ENTRIES {
            cache
                .insert_with_expiry(&i.to_string(), &i, Duration::from_secs(30))
                .await
                .unwrap();
        }
        let Cache::Memory(entries) = &cache else {
            unreachable!()
        };
        assert!(entries.lock().unwrap().len() <= MEMORY_ENTRIES);
    }

    #[tokio::test]
    async fn production_refuses_to_boot_with_no_redis() {
        if std::env::var_os("PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL").is_none() {
            assert!(Cache::from_env(&Environment::Production).is_err());
            assert!(Cache::from_env(&Environment::Other("prod-eu".to_string())).is_err());
            assert!(Cache::from_env(&Environment::Test).is_ok());
            assert!(Cache::from_env(&Environment::Other("container-dev".to_string())).is_ok());
        }
    }

    #[tokio::test]
    async fn a_redis_that_is_down_is_not_ready_and_never_hangs() {
        let client = redis::Client::open("redis://127.0.0.1:1/0").unwrap();
        let manager = redis::aio::ConnectionManager::new_lazy_with_config(
            client,
            redis::aio::ConnectionManagerConfig::new()
                .set_number_of_retries(1)
                .set_connection_timeout(Some(Duration::from_millis(100))),
        )
        .unwrap();
        let cache = Cache::Redis(manager);
        assert!(!cache.is_reachable().await);
        assert!(cache.get::<String>("a").await.is_err());
    }
}
