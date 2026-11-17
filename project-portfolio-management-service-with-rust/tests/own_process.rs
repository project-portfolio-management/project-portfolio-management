//! Run one test in a process of its own, inside the one test binary.
//!
//! The suites under `tests/` were 69 binaries; they are now modules of one (`tests/main.rs`), so Cargo links the
//! library into one test executable rather than 69. Some of them were separate binaries for a reason: they change
//! state that lives for the whole process and is read once. The service reads these once per process and keeps
//! them (each is a `OnceLock` in `src/`):
//!
//! - whether authentication is required (`auth::enforcement::REQUIRE_AUTH`, from
//!   `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`): most real-token suites turn it on, the request suites need it
//!   off;
//! - the ABAC policy (`auth::policy`, from `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY`), which suites set to
//!   different policies;
//! - the handling-mark scheme (`handling_marks::scheme`), which `handling_marks` and `public_reader` set to
//!   different schemes;
//! - the integrity-MAC key set (`compliance::mac`), set by `webhook_redelivery`;
//! - the event transport (`streaming::transport`), set to `outbox` by `outbox_audit` and `fluvio_relay`;
//! - the `OpenID Connect` and Keycloak verifiers (`auth::oidc`, `auth::verifier::KEYCLOAK`);
//! - the full-text index directory (`search::ENGINE`), which `masking` and `list_search_check_duplicates_authz` set;
//! - the global `tracing` subscriber, which `trace_span_concurrency` and `otlp_middleware` install, because what they
//!   test is the global dispatcher on a multi-threaded runtime (a thread-local `set_default` would not reach the
//!   worker threads);
//!
//! and they set environment variables (`std::env::set_var`) that other suites would then read.
//!
//! [`own_process!`] keeps that isolation without a second binary. Written first in a test's body, it starts this same
//! executable again with `<the test's name> --exact --include-ignored --test-threads=1`, and a marker variable naming
//! the test; the child process runs that one test, and the parent passes or fails with it. The child's output is
//! printed through the parent's captured output, so `--nocapture` and a failure report show it as before.
//!
//! The parent fails, too, unless the child reports exactly one test run and passed: a test name that matched nothing
//! (a rename the name derivation missed) must not pass silently.

use std::process::Command;

/// The variable that tells a child process which test it is to run for real.
const CHILD: &str = "PROJECT_PORTFOLIO_MANAGEMENT_TEST_OWN_PROCESS";

/// Run the enclosing test in a process of its own (see the module docs). Write it as the first statement of the
/// test's body; it returns from the test in the parent once the child has passed.
macro_rules! own_process {
    () => {
        if !$crate::own_process::run({
            fn here() {}
            ::std::any::type_name_of_val(&here)
        }) {
            return;
        }
    };
}

/// The test's name as libtest knows it, from the type name of a function declared in its body:
/// `main::approvals::the_test::{{closure}}::here` is `approvals::the_test`.
fn test_name(type_name: &str) -> String {
    let path = type_name
        .split_once("::")
        .map_or(type_name, |(_crate, rest)| rest);
    path.split("::")
        .filter(|segment| !segment.starts_with('{'))
        .collect::<Vec<_>>()
        .split_last()
        .map_or_else(String::new, |(_here, test)| test.join("::"))
}

/// `true` in the child, where the test body is to run; in the parent, runs the child, fails unless it passed, and
/// returns `false` so the parent's body returns.
///
/// # Panics
///
/// In the parent, when the child cannot be started, fails, or does not report one passed test.
pub fn run(type_name: &str) -> bool {
    let name = test_name(type_name);
    if std::env::var_os(CHILD).is_some_and(|v| v == name.as_str()) {
        return true;
    }
    let exe = std::env::current_exe().expect("the test executable's path");
    let output = Command::new(&exe)
        .args([
            name.as_str(),
            "--exact",
            "--include-ignored",
            "--test-threads=1",
            "--nocapture",
        ])
        .env(CHILD, &name)
        .output()
        .unwrap_or_else(|e| panic!("start {} for {name}: {e}", exe.display()));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{stdout}");
    eprint!("{stderr}");
    assert!(
        output.status.success(),
        "{name}, in its own process, failed ({})",
        output.status
    );
    assert!(
        stdout.contains("test result: ok. 1 passed;"),
        "{name}, in its own process, did not run exactly one test: is the name right?"
    );
    false
}
