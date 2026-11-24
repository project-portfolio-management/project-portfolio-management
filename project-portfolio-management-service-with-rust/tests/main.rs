//! The service's one integration-test binary (`[[test]] name = "main"`; `autotests = false` in `Cargo.toml`).
//!
//! Each module below was a binary of its own, built and linked against the whole library: 69 links per test build.
//! They are modules of this one now. The request-level suites are the [`requests`] tree (the old `tests/mod.rs`
//! binary). A suite that changes process-wide state still runs each of its tests in a process of its own, of this
//! same executable: see `axa_test::own_process!` (axa-test 0.14).
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
