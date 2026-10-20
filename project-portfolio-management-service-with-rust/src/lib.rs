//! `project-portfolio-management-service` — a loco.rs registry for **plan**
//! records in one recursive collection, with CRUD + matching.
//!
//! The API DTO is `project_portfolio_management_matcher::Plan` itself: the service stores
//! it verbatim (JSONB in the `plans` table) and matches with the
//! canonical [`project_portfolio_management_matcher`] engine, so there is no separate domain
//! model to drift. Every plan lives in one `plans` table; `kind` is an
//! optional descriptive label and matching is kind-agnostic (there is
//! no kind gate).
//!
//! ## Modules
//!
//! - [`app`] — loco `Hooks` wiring (routes, workers, truncate/seed).
//! - [`controllers`] — Axum controllers: CRUD, `match`, `check-duplicates`.
//! - [`models`] — `SeaORM` entity + CRUD helpers over the stored payload.
//! - [`workers`] — background workers (loco `BackgroundWorker`).
//! - [`tasks`], [`initializers`], [`data`] — loco extension points.
//!
//! See `spec/index.md` for the living specification.

#![warn(clippy::pedantic)]
// Rust 1.99 clippy flags `assert!(x.is_empty())` in tests and suggests `assert_eq!(x, [] as [T; 0])`, which reads
// worse; over a hundred unit-test assertions use the first form, so it is allowed in test builds only.
#![cfg_attr(test, allow(clippy::assert_is_empty))]
// The hand-written OpenAPI document is one large nested `json!` tree.
#![recursion_limit = "512"]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod access_review;
pub mod announcements;
pub mod app;
pub mod approvals;
pub mod artifacts;
pub mod auth;
/// Pure workflow-automation rules (trigger matching, action shapes, due-ness).
pub mod automation;
pub mod availability;
pub mod baselines;
pub mod benefit_validation;
pub mod break_glass;
pub mod buffer_management;
pub mod buffers;
pub mod bulk;
pub mod calendars;
/// Per-user saved views: route-scoped filter/sort/column presets,
/// keyed by the token `sub` and nothing else identity-shaped (T-28l).
pub mod canvas;
pub mod capacity_heatmap;
pub mod ccpm_estimates;
pub mod change_requests;
/// Pure collaborative-review + assignee-workload rules.
pub mod collaboration;
/// Compliance controls: row-level integrity and keyed MACs.
pub mod compliance;
pub mod components;
pub mod content_check;
pub mod controllers;
/// Pure rules for the **Controlling process** — set a standard,
/// measure, compare, act — and the three control timings
/// (feedforward / concurrent / feedback) that fix what a failing
/// control may do.
pub mod controls;
pub mod cost_rates;
pub mod cost_risk;
pub mod cpm;
pub mod critical_chain;
pub mod custom_fields;
pub mod cutover;
pub mod data;
pub mod deescalation;
pub mod delivery_confidence;
pub mod delivery_model;
pub mod deployment_moves;
pub mod director_pack;
pub mod director_pack_text;
pub mod director_summary;
pub mod discussion;
/// Pure rules for the engineering-team features (tasks / burndown / MoSCoW).
/// Flow Distribution: the mix of work types completed — feature,
/// defect, risk, debt, and the honest fifth, `unclassified`.
pub mod distribution;
/// Recorded effort and utilization — including per person, under the
/// five obligations in `agents/share/time-based-analysis.md` §7.1.
pub mod documents;
pub mod drum;
pub mod earned_value;
pub mod effort;
pub mod email;
pub mod engineering;
pub mod escalations;
pub mod evm;
pub mod extract;
pub mod feeds;
pub mod fever;
pub mod financial_year;
/// The phased budget baseline and the EAC/ETC forecast (T-28b).
pub mod financials;
/// The time-based-analysis flow-gauge refresh loop: default-off,
/// bounded by a series cap, and suppressed below a board-size floor.
pub mod flow_metrics;
pub mod frameworks;
pub mod gate_decisions;
/// Pure PPM governance rules (proposal pipeline, phase gates, risks, money).
pub mod governance;
pub mod guests;
/// Pure derivations for the executive insight areas (CEO / CFO / CTO).
pub mod hammock;
pub mod handling_marks;
pub mod http_security;
pub mod inherit;
pub mod initializers;
pub mod insights;
pub mod issues;
pub mod joint_risk;
pub mod kpi;
pub mod ledger;
pub mod leveling;
/// Pure bird.s-eye lifecycle rules (the funnel + next-phase readiness).
pub mod lifecycle;
pub mod live;
pub mod merge;
pub mod metrics;
pub mod milestone_signoff;
pub mod milestones;
pub mod models;
pub mod modules;
pub mod mspdi;
pub mod my_work;
pub mod notification_prefs;
/// Structured logging + real OpenTelemetry OTLP export (repo `tasks.md`
/// PRO-H12, slice 7 of 7 — the last). Installed via
/// [`app::App::init_logger`]; [`observability::trace_mw`] is layered in
/// `App::after_routes`.
pub mod observability;
/// The OKR engine: key-result progress, objective scores, and the
/// alignment-weighted plan score — all derived on read.
pub mod okr;
/// The pure, explainable Smart Score behind data-driven prioritization.
/// Pure rules for the sequential project phase — Initiating through
/// Closing: one-step advancement, explicitly-reasoned regression, and
/// per-phase durations from the transition log.
pub mod onboarding;
pub mod openapi;
pub mod openapi_more;
pub mod openapi_schemas;
pub mod pdf;
pub mod phase;
pub mod plan_access;
pub mod plan_links;
pub mod plan_templates;
pub mod planner;
pub mod portfolio_resources;
pub mod prioritization;
/// Field masking (owner org, lead ref) + the GDPR right-of-access export envelope.
pub mod privacy;
pub mod processes;
pub mod product_flow;
pub mod raci;
/// Durable event bus Phase 3: the outbox relay loop + retention purge.
pub mod rate_limit;
pub mod register_completeness;
pub mod relay;
pub mod report_schedules;
pub mod response_cache;
pub mod retention;
pub mod sandbox;
pub mod saved_views;
pub mod schedule_health;
pub mod schedule_risk;
/// The set-and-forget ticker: the optional scheduled-action sweep loop.
pub mod scheduler;
pub mod scheduling_rules;
/// Tantivy full-text search: index schema, engine, and query surface.
pub mod search;
pub mod sinks;
/// Point-in-time estate snapshots (board / CRO trends).
pub mod snapshots;
pub mod start_control;
/// Pure PPM strategy rules (scenario evaluation, OKR weights, ROI).
pub mod strategy;
pub mod streaming;
pub mod subjects;
pub mod task_bulk;
pub mod task_relations;
pub mod task_rollup;
pub mod tasks;
/// Time-based analysis (TBA): pure computation over the task
/// status-transition log — cycle versus lead time, flow efficiency,
/// rework and first-pass yield, the service level expectation,
/// constraint ranking, and queueing-theory flow.
pub mod tba;
pub mod thresholds;
pub mod timesheets;
pub mod tolerances;
/// Total Project Control (TPC): pure rules for Devaux's Index of
/// Project Performance (DIPP), Expected Monetary Value, and Cost
/// Estimate to Complete — is the value still to come worth the money
/// still to spend?
pub mod tpc;
pub mod tracker_import;
pub mod trusted_proxy;
pub mod tz;
pub mod validation;
/// Realized gains and strategic performance: transformation ROI, value
/// realization rate, time to value, adoption, earned-value indices and
/// stakeholder sentiment — every one derived, never stored.
pub mod value;
/// Header-based API versioning (`Accepts-version`) for the REST surface.
pub mod version;
/// Pure PPM visibility rules (schedule math, RAG, capacity, CSV).
pub mod visibility;
pub mod watermelon;
/// Outbound webhook delivery — the `WebhookSink` family contract
/// (`agents/share/event-bus.md` §12, repo `tasks.md` EV-3).
pub mod webhooks;
pub mod work_profile;
pub mod workers;
/// Live worker-skill resolution over the worker service by
/// `EntityRef`, TTL-cached, never persisted (T-28c).
pub mod workers_client;
/// Custom workflows: configurable task and issue state vocabularies,
/// each state declaring the category every derived view computes from.
pub mod workflow;
