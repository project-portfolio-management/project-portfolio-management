# Learning, upskilling and economic viability (proposed 2026-11-04)

> **Status: proposed; the pure core `viability` is built (UE-1), nothing else.** Tasks UE-1 to UE-8, decisions UE-D1 to UE-D3 (proposed, the owner's to confirm). The sister file in the workforce planning repository is `spec/learning-economics.md` there; the two share a vocabulary, not code.

## Where PPM stands

PPM measures a plan's **return** (transformation ROI from realized value against actual cost) and carries budget lines, variance, earned value and a business case. It has **no net present value, no payback and no viability verdict**: the specification promises an NPV with its rate echoed ([06-functional-requirements.md](06-functional-requirements.md), the metrics table), and nothing computes it. It has **no notion of learning at all**: a plan can need a skill nobody has, and the cost of building that capability is a budget line like any other, invisible as such.

| Holds today | Missing |
|---|---|
| ROI from realized value and actual cost, mixed currencies withheld | Net present value, payback, discounted payback, a verdict |
| Budget lines with a category, variance by currency | A way to see the **training and upskilling** a plan spends |
| A skill-gap view fed by the workforce planning service | That feed answers `unknown` today (FB-7), so no gap is priced |
| Value facts with a method (measured or asserted) | Dated flows per period to discount |

## The approach

1. **Viability as a pure rule** ([`viability.rs`](../project-portfolio-management-service-with-rust/src/viability.rs), built): signed flows by period, a discount rate in basis points per period **echoed**, NPV, payback and discounted payback (linear inside the period, in whole periods and basis points of the next), and a verdict: `viable`, `too_slow` (positive NPV, payback past the owner's limit), `break_even`, `not_viable` or `unmeasured`. **No benefit recorded is `unmeasured`, never `not_viable`**, the rule of the rest of the crate: a plan that has not estimated its benefits has not failed.
2. **Training as a category, not a module.** A budget line categorized `training` is the cost of building capability; the portfolio reports it per plan and rolls it up, per currency, never mixed.
3. **A plan's viability including its training.** Costs from the plan's budget lines (actual where recorded, else baseline, and the answer says which), benefits from its value facts and its business case's projected benefits, by period.
4. **A priced capability gap.** Where the workforce planning service answers a person's or a pool's skill gap and training hours, the plan shows the training time and, where WPM offers one, its cost. Nothing is priced from a guess.

## Tasks

See [`tasks.md`](../tasks.md), "Learning, upskilling and economic viability (UE-n)".

## Decisions (proposed)

- **UE-D1.** Viability follows one definition in this project and the workforce planning one: flows by period, the rate echoed, half-away-from-zero rounding at each step, a series capped at 600 periods, `unmeasured` where either side is absent.
- **UE-D2.** Training is a category of budget line, not a separate ledger.
- **UE-D3.** A benefit is a stated estimate with a source or a measured value; a skill gained is never converted into money.

## Not taken

- An internal rate of return. It has no unique answer for a series that changes sign more than once, and a figure that can be wrong in a way the reader cannot see is worse than none.
- A verdict across currencies. A mixed series is withheld, as ROI already is.
