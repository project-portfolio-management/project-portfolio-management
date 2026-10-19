# AGENTS.md — Working Guide for AI Coding Agents

Entry point for AI coding agents working in the plan matcher (the service's `src/matcher/` module).

> If you only read one file, read [`spec/index.md`](../../spec/matcher.md) —
> the living specification.

## Quick orientation

| Question | Answer |
|---|---|
| What does the crate do? | Pairwise plan record matching, deterministic + probabilistic, for dedup. Plans live in one recursive tree; the optional `kind` label (Portfolio / Project / Product / Program / Practice / Process / Purpose / Pathway / Proposal) is descriptive metadata and does not gate matching. v0.2.0. |
| Canonical spec? | [`spec/index.md`](../../spec/matcher.md). |
| Entity domain model? | [`../spec/index.md`](../../spec/matcher.md) §5 (entity-level umbrella). |
| Build / test / lint / fmt | `cargo build` · `cargo test` (unit, `tests/public_api.rs`, `tests/property_tests.rs` with proptest, doctests) · `cargo clippy --all-targets --all-features -- -D warnings` · `cargo fmt --check` · `cargo bench` (`benches/match_pair.rs`) |
| Run the demo | `cargo run` (`src/main.rs`; not SemVer surface). |
| Public types | `src/lib.rs` re-exports from `src/{plan,matcher,scoring,config,normalize,phonetic,error}.rs`. |
| Match gate | **None.** The former kind gate was removed; any two plans may match. `MatchBreakdown.kind_gate_blocked` is vestigial (always `false`). |
| Deterministic schemes (→ 1.0) | URI, UUID, Jira project key, Asana GID, Trello board id, MS Project id, GitHub project id, Linear id; plus same-owner code (R-1) and `same_as` URL overlap (R-2). |
| Probabilistic components | name (Jaro-Winkler + Soundex), goals (Jaccard over titles), owner-scoped code, owner org, parent (parent plan exact via `parent_ref`), timeframe (Gaussian decay), keywords (Jaccard), relationships (typed-set Jaccard), tags (set Jaccard). |
| Presets | `MatchConfig::default()` threshold 0.85; `strict()` 0.95; `lenient()` 0.70 (weights unchanged; nine weights sum to 1.0). |
| Phase | `Plan.phase: Option<PlanPhase>` (Initiating/Planning/Executing/Controlling/Closing) is data only and **never scored** (pinned by `phase_is_not_scored`). |
| Fuzzing | `fuzz/` (`match_plans`, `normalize` targets). |
| Public API shape | `MatchingEngine::new(MatchConfig::default()).match_plans(&a, &b) -> MatchResult`. `Plan::new(name)` (kind defaults `None`). |

## Golden rules

1. **Spec-first.** Behavioural change ⇒ update `spec/index.md`.
2. **No kind gate.** `kind` is optional descriptive metadata and never
   gates matching. Any two plans may match.
3. **Pure library.** No IO, no logging, no global state in `src/`
   (excluding `src/main.rs`); `#![forbid(unsafe_code)]` in `lib.rs`.
4. **No `unsafe`. No `unwrap`/`expect`/`panic!`** in library code.
5. **Deterministic.** No clocks, RNGs, or environment variables.
6. **Explainability.** Every match returns a per-component breakdown.
7. **Diacritic-correct.**
8. **Bound `goals`/`keywords`/`relationships`/`tags` before calling
   in.** This crate has no length cap of its own on any of the four —
   each is scored by Jaccard over every element (unbounded O(n·m)).
   `project-portfolio-management-service` validates every payload
   before it reaches this crate (`src/validation.rs`, `MAX_ARRAY_LEN` =
   256, `MAX_ITEM_LEN` = 512); a standalone integrator that skips an
   equivalent cap is exposing a denial-of-service vector, not merely a
   slow path.

## What not to do

- Do not reintroduce a kind gate — `kind` is descriptive metadata, and
  two plans with different kinds may still be the same identity.
- Do not short-circuit on owner-scoped (`Code`/`LocalId`) or `Custom`
  schemes — they are not globally unique.
- Do not score a `code` across differing owners.
- Do not match on `status` (it drifts between duplicate records).
- Do not score `phase` (or `status`); both drift between duplicates.
- Do not change default weights/threshold without updating
  `spec/index.md §7` and `CHANGELOG.md`.

## Detailed guides

- [agents/matching-algorithm.md](matching-algorithm.md)
- [agents/normalization.md](normalization.md)
- [agents/spec-driven-development.md](spec-driven-development.md)
- [agents/testing.md](testing.md)

## File layout

```
src/
├── lib.rs         public re-exports
├── main.rs        demo binary (not SemVer surface)
├── plan.rs        domain types (Plan, PlanKind [optional label], PlanPhase, Goal, GoalStatus, PlanStatus, IdentifierScheme, PlanIdentifier, PlanRelationship, RelationKind)
├── matcher.rs     MatchingEngine + per-component fns + deterministic rules (no kind gate)
├── scoring.rs     MatchResult, MatchBreakdown, Confidence, weighted_average
├── normalize.rs   fold, code, url, fold_set, iso_date_to_days
├── phonetic.rs    Soundex (name component bonus)
├── config.rs      MatchConfig (weights + threshold)
└── error.rs       error type
```
