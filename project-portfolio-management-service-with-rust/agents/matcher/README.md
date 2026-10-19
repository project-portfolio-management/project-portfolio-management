# Plan matcher

> The matcher is the `matcher` module of the service crate (`src/matcher/`). It was a separate crate,
> `project-portfolio-management-matcher`, until it was folded in; version 0.2.0 of that crate stays on crates.io.
> Paths and commands below are relative to the service directory: `cargo test --lib matcher`,
> `cargo run --example matcher_demo`, `cargo bench --bench match_pair`.

Pairwise **plan record matching** for Rust, for deduplication across one
recursive plan collection. A *plan* may contain other plans (a
recursive tree) with an optional `kind` label (Portfolio / Project /
Product / Program / Practice / Process / Purpose / Pathway / Proposal)
that is descriptive metadata and does not gate matching. Combines
deterministic identifier short-circuits and explainable probabilistic
scoring. Dependency-light, no IO, no `unsafe`.

- Spec: [spec/index.md](../../spec/matcher.md)
- Agent guide: [AGENTS.md](./AGENTS.md)
- Entity domain model: [`../spec/index.md`](../../spec/matcher.md) §5

## Usage

```rust
use project_portfolio_management_matcher::{Plan, MatchConfig, MatchingEngine};

let engine = MatchingEngine::new(MatchConfig::default());
let a = Plan::new("Customer Onboarding Revamp");
let b = Plan::new("Customer Onboarding Revamp Initiative");
let r = engine.match_plans(&a, &b);
assert!((0.0..=1.0).contains(&r.score));
```

## Matching at a glance

There is **no kind gate**: all plans live in one recursive collection,
so any two plans may match regardless of their (optional) `kind`
(`Portfolio` / `Project` / `Product` / `Program` / `Practice` /
`Process` / `Purpose` / `Pathway` / `Proposal`), which is descriptive
metadata only and is never compared.

| Component | Weight | Algorithm |
|---|---:|---|
| Name | 0.30 | Jaro-Winkler + Soundex bonus |
| Goals | 0.15 | Jaccard over folded goal titles |
| Code | 0.15 | Same-owner equality (1.0 / 0.0) |
| Owner org | 0.10 | Case-folded exact (1.0 / 0.0) |
| Parent | 0.08 | Same parent `parent_ref` (1.0 / 0.0) |
| Timeframe | 0.07 | Date proximity (Gaussian decay) |
| Keywords | 0.05 | Jaccard |
| Relationships | 0.05 | Typed-set Jaccard over `(relation, plan_id)` |
| Tags | 0.05 | Set Jaccard over normalized tags |

Default threshold 0.85 (`MatchConfig::strict()` 0.95, `lenient()` 0.70).
Weights renormalize over the components both records carry. `phase` and
`status` are never scored.

**Deterministic short-circuit → 1.0** on a shared globally-unique
identifier (URI, UUID, Jira project key, Asana GID, Trello board id, MS
Project id, GitHub project id, Linear id), a same-owner code, or a
`same_as` URL overlap. Owner-scoped codes (`Code`/`LocalId`) and
`Custom` are evidence at most.

## Run the demo

```bash
cargo run
```

## Testing

```bash
cargo test                # unit, public_api, proptest property tests, doctests
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo bench               # benches/match_pair.rs
```

## Licence

Licence is any of these, or contact us for custom licence options.

* [MIT](https://opensource.org/license/mit) ([SPDX: MIT](https://spdx.org/licenses/MIT.html)), text in [`../LICENSE/MIT.txt`](../../../LICENSE/MIT.txt)
* [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([SPDX: Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)), text in [`../LICENSE/Apache-2.0.txt`](../../../LICENSE/Apache-2.0.txt)
* [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) ([SPDX: GPL-2.0-only](https://spdx.org/licenses/GPL-2.0-only.html)), text in [`../LICENSE/GPL-2.0-only.txt`](../../../LICENSE/GPL-2.0-only.txt)
* [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) ([SPDX: GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html)), text in [`../LICENSE/GPL-3.0-only.txt`](../../../LICENSE/GPL-3.0-only.txt)
* [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) ([SPDX: BSD-3-Clause](https://spdx.org/licenses/BSD-3-Clause.html)), text in [`../LICENSE/BSD-3-Clause.txt`](../../../LICENSE/BSD-3-Clause.txt)

SPDX expression: `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`.
