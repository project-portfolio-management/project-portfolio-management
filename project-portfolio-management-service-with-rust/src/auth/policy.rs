//! The ABAC policy: where it is read from, the hot reload, and the watcher.

#[allow(
    clippy::wildcard_imports,
    reason = "the parent module holds the shared imports and the other parts"
)]
use super::*;

/// The process-wide ABAC policy, read once from
/// `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY` / `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE` and cached
/// (restart to change). Mirrors [`require_auth`]: a process-wide
/// `OnceLock` built from the environment via [`policy_from_env`].
#[must_use]
pub fn policy() -> &'static ReloadablePolicy {
    static POLICY: OnceLock<ReloadablePolicy> = OnceLock::new();
    POLICY.get_or_init(|| ReloadablePolicy::new(policy_from_env()))
}

/// Re-read the ABAC policy from the environment and swap it into the
/// live [`policy`] holder. A malformed policy falls back to the built-in
/// default, so a bad edit can never leave the service unprotected.
pub fn reload_policy() {
    policy().store(policy_from_env());
    tracing::info!("ABAC policy reloaded");
}

/// How often the policy-file watcher polls for a change (seconds).
pub(super) const POLICY_WATCH_SECS: u64 = 15;

/// Spawn the background task that hot-reloads the ABAC policy when
/// `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE` changes on disk, so an
/// operator can edit the policy and have it take effect without a
/// restart. Polls the file's mtime (not an OS notify — this stays
/// dependency-light).
///
/// A no-op when the variable is unset: an inline
/// `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY` (or the built-in default) has
/// nothing to watch, and environment variables do not change under a
/// running process.
pub fn spawn_policy_watcher() {
    let Some(path) = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE")
        .ok()
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
    else {
        return;
    };
    tokio::spawn(async move {
        let mut last = file_mtime(&path);
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(POLICY_WATCH_SECS));
        ticker.tick().await; // skip the immediate first tick
        loop {
            ticker.tick().await;
            let now = file_mtime(&path);
            if now != last {
                last = now;
                reload_policy();
            }
        }
    });
    tracing::info!(
        secs = POLICY_WATCH_SECS,
        "watching the ABAC policy file for changes"
    );
}

/// The last-modified time of `path`, or `None` when it cannot be read
/// (missing file, permission error, platform without mtime).
pub(super) fn file_mtime(path: &str) -> Option<std::time::SystemTime> {
    std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
}

/// Load the ABAC policy: `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY` (inline JSON) wins,
/// then `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE` (path to a JSON file), else the
/// built-in default policy. A present-but-unparsable policy (bad JSON,
/// unknown effect/action names, unreadable file) `tracing::warn!`s and
/// falls back to the default — the service always boots, matching the
/// key-fetch posture.
#[must_use]
pub fn policy_from_env() -> Policy {
    let source = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| {
            let path = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE")
                .ok()
                .filter(|v| !v.trim().is_empty())?;
            match std::fs::read_to_string(path.trim()) {
                Ok(contents) => Some(contents),
                Err(error) => {
                    tracing::warn!(%error, %path, "ABAC policy file unreadable; using the built-in default policy");
                    None
                }
            }
        });
    with_service_rules(match source {
        Some(json) => Policy::from_json(&json).unwrap_or_else(|error| {
            tracing::warn!(%error, "ABAC policy JSON invalid; using the built-in default policy");
            Policy::default_policy()
        }),
        None => Policy::default_policy(),
    })
}
