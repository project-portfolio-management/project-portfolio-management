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
/// live `policy` holder. A malformed policy falls back to the built-in
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
    watch_file(
        path,
        std::time::Duration::from_secs(POLICY_WATCH_SECS),
        reload_policy,
    );
    tracing::info!(
        secs = POLICY_WATCH_SECS,
        "watching the ABAC policy file for changes"
    );
}

/// Poll `path`'s last-modified time every `every` and call `on_change` each time it differs from the last poll (a file
/// that appears, disappears or is rewritten). The first poll only records the time, so starting the watcher does not
/// itself count as a change. Runs until the returned task is aborted.
pub(super) fn watch_file(
    path: String,
    every: std::time::Duration,
    on_change: impl Fn() + Send + 'static,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut last = file_mtime(&path);
        let mut ticker = tokio::time::interval(every);
        ticker.tick().await; // skip the immediate first tick
        loop {
            ticker.tick().await;
            let now = file_mtime(&path);
            if now != last {
                last = now;
                on_change();
            }
        }
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_NAME: &str =
        "auth::policy::tests::environment_policy_sources_are_exercised_in_child_processes";

    #[tokio::test]
    async fn the_watcher_calls_back_once_per_change_and_not_at_start() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::time::{Duration, SystemTime};

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("policy.json");
        std::fs::write(&file, r#"{"rules":[]}"#).unwrap();
        let changes = Arc::new(AtomicUsize::new(0));
        let counted = changes.clone();
        let watcher = watch_file(
            file.to_string_lossy().to_string(),
            Duration::from_millis(10),
            move || {
                counted.fetch_add(1, Ordering::SeqCst);
            },
        );
        let wait_for = |n: usize| {
            let changes = changes.clone();
            async move {
                for _ in 0..200 {
                    if changes.load(Ordering::SeqCst) >= n {
                        return;
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            }
        };

        // Started, unchanged: no call.
        tokio::time::sleep(Duration::from_millis(60)).await;
        assert_eq!(changes.load(Ordering::SeqCst), 0);
        // A rewrite (a later modified time): one call, and quiet again after it.
        std::fs::File::options()
            .write(true)
            .open(&file)
            .unwrap()
            .set_modified(SystemTime::now() + Duration::from_secs(60))
            .unwrap();
        wait_for(1).await;
        tokio::time::sleep(Duration::from_millis(60)).await;
        assert_eq!(changes.load(Ordering::SeqCst), 1);
        // The file removed: that is a change too.
        std::fs::remove_file(&file).unwrap();
        wait_for(2).await;
        assert_eq!(changes.load(Ordering::SeqCst), 2);
        watcher.abort();
    }

    #[test]
    fn environment_policy_sources_are_exercised_in_child_processes() {
        if let Ok(case) = std::env::var("PPM_POLICY_TEST_CASE") {
            let _loaded = policy_from_env();
            let _ = policy();
            reload_policy();
            match case.as_str() {
                "file" => assert!(
                    file_mtime(
                        &std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE").unwrap()
                    )
                    .is_some()
                ),
                "missing" => assert!(file_mtime("/definitely/not/a/policy.json").is_none()),
                _ => {}
            }
            return;
        }

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("policy.json");
        std::fs::write(&file, r#"{"rules":[]}"#).unwrap();
        let executable = std::env::current_exe().unwrap();
        for (case, inline, path) in [
            ("default", None, None),
            ("inline", Some(r#"{"rules":[]}"#), None),
            ("invalid", Some("{not json"), None),
            ("file", None, file.to_str()),
            ("missing", None, Some("/definitely/not/a/policy.json")),
        ] {
            let mut child = std::process::Command::new(&executable);
            child
                .arg("--exact")
                .arg(TEST_NAME)
                .arg("--nocapture")
                .env("PPM_POLICY_TEST_CASE", case)
                .env_remove("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY")
                .env_remove("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE");
            if let Some(value) = inline {
                child.env("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY", value);
            }
            if let Some(value) = path {
                child.env("PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE", value);
            }
            assert!(child.status().unwrap().success(), "policy child {case}");
        }
    }
}
