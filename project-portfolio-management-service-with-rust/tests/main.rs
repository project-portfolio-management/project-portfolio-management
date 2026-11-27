//! The service's one integration-test binary (`[[test]] name = "main"`; `autotests = false` in `Cargo.toml`).
//!
//! Each module below was a binary of its own, built and linked against the whole library: 69 links per test build.
//! They are modules of this one now. The request-level suites are the [`requests`] tree (the old `tests/mod.rs`
//! binary). A suite that changes process-wide state still runs each of its tests in a process of its own, of this
//! same executable: see [`own_process`].
//!
//! Running them:
//!
//! - `cargo test --test main` runs every test that needs no database or container (the rest are `#[ignore]`d);
//! - `cargo test --test main -- --ignored --test-threads=1 --skip load_test::` runs the database suites, as CI does
//!   (`load_test` needs a running service; `keycloak_container` and `fluvio_relay` build only with their features);
//! - one former binary is a module filter: `cargo test --test main approvals:: -- --ignored --test-threads=1`, and
//!   the request suites are `requests::`.
//!
//! A filter is a substring of the test's path, so a module's name is kept from appearing inside another's:
//! `tests/change_requests.rs` is the module `change_request_rights`, as `change_requests::` would also match
//! `requests::change_requests::`, and `requests::` would match `change_requests::`.

// One binary holding every suite is large: on macOS `ld` says its `__eh_frame` section is over 16 MB, too large for the
// compact unwind table, so unwinding a panic is slower. A note about speed, not a fault; Linux (CI) prints nothing.
#![allow(
    linker_messages,
    reason = "ld64's __eh_frame size note for one large test binary"
)]

/// Run one test in a process of its own, inside the one test binary: axa-test's `own_process!` (axa 0.21, TST-12), which
/// replaced this crate's own `tests/own_process.rs`.
///
/// The suites under `tests/` were 69 binaries; they are now modules of one (`tests/main.rs`), so Cargo links the
/// library into one test executable rather than 69. Some of them were separate binaries for a reason: they change
/// state that lives for the whole process and is read once. The service reads these once per process and keeps
/// them (each is a `OnceLock` in `src/`):
///
/// - whether authentication is required (`auth::enforcement::REQUIRE_AUTH`, from
///   `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`): most real-token suites turn it on, the request suites need it
///   off;
/// - the ABAC policy (`auth::policy`, from `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY`), which suites set to
///   different policies;
/// - the handling-mark scheme (`handling_marks::scheme`), which `handling_marks` and `public_reader` set to
///   different schemes;
/// - the integrity-MAC key set (`compliance::mac`), set by `webhook_redelivery`;
/// - the event transport (`streaming::transport`), set to `outbox` by `outbox_audit` and `fluvio_relay`;
/// - the `OpenID Connect` and Keycloak verifiers (`auth::oidc`, `auth::verifier::KEYCLOAK`);
/// - the full-text index directory (`search::ENGINE`), which `masking` and `list_search_check_duplicates_authz` set;
/// - the global `tracing` subscriber, which `trace_span_concurrency` and `otlp_middleware` install, because what they
///   test is the global dispatcher on a multi-threaded runtime (a thread-local `set_default` would not reach the
///   worker threads);
///
/// and they set environment variables (`std::env::set_var`) that other suites would then read.
///
/// `own_process!` keeps that isolation without a second binary. Written first in a test's body, it starts this same
/// executable again with `<the test's name> --exact --include-ignored --test-threads=1`, and `AXA_TEST_OWN_PROCESS`
/// naming the test; the child process runs that one test, and the parent passes or fails with it, showing the child's
/// output when it fails. The parent fails, too, unless the child reports exactly one test run and passed: a test name
/// that matched nothing (a rename the name derivation missed) must not pass silently.
macro_rules! own_process {
    () => {
        ::axa_test::own_process!()
    };
}

mod otlp_collector;
mod requests;

mod approvals;
mod availability_acl;
mod benefit_validation;
mod break_glass;
mod capacity_heatmap;
mod capture_scope;
#[path = "change_requests.rs"]
mod change_request_rights;
mod cli;
mod controlled_documents;
mod coverage_config;
mod cross_plan;
mod delivery_confidence;
mod director_packs;
mod drum_pipeline;
mod enforcement;
mod escalation_rights;
mod feeds;
mod fever;
mod finance_gate;
mod fluvio_relay;
mod framework_versions;
mod funding;
mod guests;
mod handling_marks;
mod hard_constraint;
mod hardening;
mod idempotency_callers;
mod issue_configuration;
mod keycloak_container;
mod keycloak_entra_broker;
mod keycloak_guard;
mod licence_files;
mod list_paging;
mod list_search_check_duplicates_authz;
mod load_test;
mod masking;
mod matching;
mod migrations;
mod my_work;
mod notification_prefs;
mod oidc_provider;
mod organizations;
mod otlp_export;
mod otlp_middleware;
mod outbound_hosts;
mod outbox_audit;
mod pack_rhythm;
mod personal_data_inventory;
mod plan_access;
mod planner;
mod portfolio_evm;
mod private_aggregates;
mod private_routes;
mod processes;
mod programme_gates;
mod public_files;
mod public_reader;
mod public_roadmap_privacy;
mod read_only_scope;
mod references_privacy;
mod report_schedules;
mod saved_views;
mod sinks;
mod sqlx_layer;
mod timesheets;
mod today_time_zone;
mod trace_span_concurrency;
mod watchers;
mod webhook_redelivery;
mod workflow_roles;
mod wpm_bridge;
