//! The plan matcher — pairwise plan record matching.
//!
//! A *plan* is a named unit of intended work in a portfolio /
//! project-management registry, forming one recursive tree: any work
//! item may contain any other via `parent_ref`. The former `Portfolio` /
//! `Project` / `Product` / `Program` kinds were unified into this single
//! type; `kind` survives only as **optional descriptive metadata** and
//! no longer gates matching. The matcher takes two [`Plan`] records
//! and produces a [`MatchResult`] with a score in `[0.0, 1.0]`, a
//! [`Confidence`] band, an `is_match` boolean, and a per-component
//! [`MatchBreakdown`].
//!
//! ## No kind gate
//!
//! Because the kinds were unified, **any two plans may match**
//! regardless of their (optional) `kind`. The `MatchBreakdown` retains a
//! vestigial `kind_gate_blocked` field, now always `false`.
//!
//! ## Strategies
//!
//! - **Deterministic** — rule-based short-circuit: an exact match on a
//!   deterministic identifier scheme (the PM-tool/registry ids plus URI /
//!   UUID), a same-owner `code` match, or a `same_as` URL overlap pins
//!   the score to `1.0`.
//! - **Probabilistic** — a weighted fuzzy score over name (Jaro-Winkler +
//!   Soundex), goal titles (Jaccard), owner-scoped code, owner org,
//!   parent plan (`parent_ref`), timeframe (Gaussian date proximity),
//!   keywords, relationships (typed-set Jaccard), and tags; renormalized
//!   over the components both records carry.
//!
//! ## Usage
//!
//! ```
//! use project_portfolio_management::matcher::{Plan, MatchConfig, MatchingEngine};
//!
//! let engine = MatchingEngine::new(MatchConfig::default());
//! let a = Plan::new("Apollo platform migration");
//! let b = Plan::new("Apollo platform migrate");
//! let result = engine.match_plans(&a, &b);
//! assert!(result.score >= 0.0 && result.score <= 1.0);
//! ```
//!
//! ## Public types
//!
//! - [`Plan`], [`PlanKind`], [`PlanStatus`], [`PlanPhase`], [`Goal`],
//!   [`GoalStatus`], [`PlanIdentifier`], [`IdentifierScheme`],
//!   [`PlanRelationship`], [`RelationKind`].
//! - [`MatchingEngine`], [`MatchConfig`], [`MatchResult`],
//!   [`MatchBreakdown`], [`Confidence`].
//!
//! ## The caller must bound array sizes
//!
//! `goals`, `keywords`, `relationships`, and `tags` are each compared by
//! Jaccard over every element (`engine.rs`), an **unbounded O(n·m)**
//! operation — this module has no length cap of its own on any of the
//! four fields (unlike `Plan`'s consuming service, which validates
//! every payload before it ever reaches a matcher). Any new caller that accepts
//! these fields from untrusted input must cap array length and per-entry
//! length itself before calling [`MatchingEngine::match_plans`] — an
//! unbounded caller-supplied array is a denial-of-service vector
//! (`agents/share/security.md` invariant 3). The service's `src/validation.rs`
//! (`MAX_ARRAY_LEN` = 256, `MAX_ITEM_LEN` = 512) is the reference cap.

// ─── Modules ─────────────────────────────────────────────────────────
// One concern per module; `lib.rs` re-exports the stable surface below so
// callers never depend on module paths.

/// Tunable weights, timeframe σ, and threshold for the probabilistic
/// strategy.
pub mod config;
/// The [`MatchingEngine`] entry point and the matching algorithm.
pub mod engine;
/// Crate error type and its [`Result`] alias.
pub mod error;
/// Input normalization helpers (fold / code / URL / set / ISO date).
pub mod normalize;
/// Soundex phonetic encoder backing the name-component bonus.
pub mod phonetic;
/// Domain model: the [`Plan`] record and its categorical enums.
pub mod plan;
/// Match-result shape and the renormalized weighted-average helper.
pub mod scoring;

// ─── Public re-exports ───────────────────────────────────────────────
// The flat, stable API surface. These are the only paths downstream
// crates should name; the module layout above is free to evolve.

pub use config::MatchConfig;
pub use engine::MatchingEngine;
pub use error::{Error, Result};
pub use plan::{
    Goal, GoalStatus, IdentifierScheme, Plan, PlanIdentifier, PlanKind, PlanPhase,
    PlanRelationship, PlanStatus, RelationKind,
};
pub use scoring::{Confidence, MatchBreakdown, MatchResult};
