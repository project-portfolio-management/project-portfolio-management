//! `project-portfolio-management-domain` — the pure domain logic of the project portfolio management service:
//! scheduling (critical path, leveling, schedule risk and health), earned value and cost risk, buffers and fever
//! charts, tolerances, prioritization, ledgers, imports (MSPDI, CSV, tracker exports) and validation.
//!
//! Nothing here touches HTTP, the database or the application context: the crate may not depend on the service,
//! `axum`, `axa` or `sqlx` (root spec §8.3c). The service re-exports every module under its old name, so
//! `project_portfolio_management::cpm` and `crate::cpm` in the service still name the same module; inside this
//! crate, `crate::x` is a sibling domain module. An edit to a controller no longer recompiles this crate.

// The OpenAPI entries in `openapi_more` are large nested `json!` trees.
#![recursion_limit = "512"]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Dates, instants, durations and IANA zones (the `project-portfolio-management-datetime` crate, over jiff), so
/// `crate::datetime::Date` reads the same here as in the service.
pub use project_portfolio_management_datetime as datetime;

pub mod access_review;
pub mod announcements;
pub mod approvals;
pub mod artifacts;
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
pub mod components;
/// Pure rules for the **Controlling process** — set a standard,
/// measure, compare, act — and the three control timings
/// (feedforward / concurrent / feedback) that fix what a failing
/// control may do.
pub mod controls;
pub mod cost_rates;
pub mod cost_risk;
pub mod cpm;
pub mod critical_chain;
pub mod csv_cell;
pub mod currency;
pub mod custom_fields;
pub mod cutover;
pub mod deescalation;
pub mod delivery_confidence;
pub mod delivery_model;
pub mod deployment_moves;
pub mod dhcw_import;
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
pub mod engineering;
pub mod escalations;
pub mod evm;
pub mod exceptions_digest;
pub mod extract;
pub mod feeds;
pub mod fever;
pub mod financial_year;
/// The phased budget baseline and the EAC/ETC forecast (T-28b).
pub mod financials;
pub mod frameworks;
pub mod gate_decisions;
/// Pure PPM governance rules (proposal pipeline, phase gates, risks, money).
pub mod governance;
pub mod guests;
/// Pure derivations for the executive insight areas (CEO / CFO / CTO).
pub mod hammock;
pub mod inherit;
pub mod insights;
pub mod issues;
pub mod joint_risk;
pub mod kpi;
pub mod ledger;
pub mod leveling;
/// Pure bird.s-eye lifecycle rules (the funnel + next-phase readiness).
pub mod lifecycle;
pub mod link_token;
pub mod meeting_actions;
pub mod meetings;
pub mod merge;
pub mod milestone_signoff;
pub mod milestones;
pub mod modules;
pub mod mspdi;
pub mod mspdi_batch;
pub mod my_work;
pub mod notification_prefs;
/// The OKR engine: key-result progress, objective scores, and the
/// alignment-weighted plan score — all derived on read.
pub mod okr;
/// The pure, explainable Smart Score behind data-driven prioritization.
/// Pure rules for the sequential project phase — Initiating through
/// Closing: one-step advancement, explicitly-reasoned regression, and
/// per-phase durations from the transition log.
pub mod onboarding;
pub mod openapi_more;
pub mod pdf;
pub mod phase;
pub mod plan_access;
pub mod plan_links;
pub mod plan_roles;
pub mod planner;
pub mod portfolio_resources;
pub mod prioritization;
/// Field masking (owner org, lead ref) + the data-subject access export envelope.
pub mod privacy;
pub mod processes;
pub mod product_flow;
/// Count of statements run on the app's connection, for query-count tests.
pub mod public_roadmap;
pub mod query_count;
pub mod raci;
pub mod register_completeness;
pub mod report_schedules;
pub mod sandbox;
pub mod saved_views;
pub mod schedule_health;
pub mod schedule_risk;
pub mod scheduling_rules;
pub mod scoring_models;
pub mod sinks;
pub mod start_control;
pub mod status_evidence;
pub mod status_updates;
/// Pure PPM strategy rules (scenario evaluation, OKR weights, ROI).
pub mod strategy;
pub mod subjects;
pub mod task_relations;
pub mod task_rollup;
pub mod thresholds;
pub mod timesheets;
pub mod tolerances;
/// Total Project Control (TPC): pure rules for Devaux's Index of
/// Project Performance (DIPP), Expected Monetary Value, and Cost
/// Estimate to Complete — is the value still to come worth the money
/// still to spend?
pub mod tpc;
pub mod tracker_import;
pub mod tz;
pub mod validation;
/// Realized gains and strategic performance: transformation ROI, value
/// realization rate, time to value, adoption, earned-value indices and
/// stakeholder sentiment — every one derived, never stored.
pub mod value;
/// Net present value, payback and a viability verdict over dated cash flows.
pub mod viability;
/// Pure PPM visibility rules (schedule math, RAG, capacity, CSV).
pub mod visibility;
pub mod watermelon;
pub mod work_profile;
/// Working-day length and the default working week, per deployment (WW-1.4).
pub mod working_time;
