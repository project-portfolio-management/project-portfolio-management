//! Pure PPM Phase-C strategy rules (spec/15-roadmap PPM-2/4/5/11):
//! the idea funnel tokens, scenario evaluation arithmetic, OKR weight
//! bounds, and benefit/ROI math — DB-free and unit-tested.

use uuid::Uuid;

/// Idea funnel statuses (PPM-2).
pub const IDEA_STATUSES: &[&str] = &["open", "converted", "dismissed"];

/// Scenario statuses (PPM-4).
pub const SCENARIO_STATUSES: &[&str] = &["draft", "committed"];

/// Benefit categories (PPM-11).
pub const BENEFIT_CATEGORIES: &[&str] = &[
    "cost_saving",
    "revenue",
    "risk_reduction",
    "quality",
    "compliance",
    "other",
];

/// Benefit statuses (PPM-11).
pub const BENEFIT_STATUSES: &[&str] = &["planned", "on_track", "realized", "missed"];

/// Whether an OKR mapping weight is in bounds (1–5).
#[must_use]
pub fn valid_weight(weight: i32) -> bool {
    (1..=5).contains(&weight)
}

/// One scenario member's prepared facts (aggregated by the caller —
/// the evaluation itself stays pure).
#[derive(Debug, Clone)]
pub struct MemberFact {
    /// The member's pid (plan or proposal).
    pub pid: Uuid,
    /// Planned budget per currency (plans) or the requested
    /// funding (proposals), in minor units.
    pub planned_by_currency: Vec<(String, i64)>,
    /// Sum of open risk exposure (plans; proposals carry 0).
    pub open_exposure: i32,
    /// Sum of OKR mapping weights (plans; proposals carry 0).
    pub alignment_weight: i32,
}

/// The scenario's constraint knobs.
#[derive(Debug, Clone, Default)]
pub struct Constraints {
    /// Budget cap in minor units of [`Constraints::currency`].
    pub budget_cap_minor: Option<i64>,
    /// The cap's currency (required when the cap is set).
    pub currency: Option<String>,
    /// Pids that must appear in the membership.
    pub must_include: Vec<Uuid>,
}

/// One live row an evaluation read, with its own `updated_at` — so two
/// evaluations of the same scenario that disagree can point at which
/// input moved (T-28a). Populated by the controller (DB-touching); the
/// shape itself is pure data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct InputRead {
    /// What kind of row this is: `"budget_line"`, `"risk"`,
    /// `"objective_link"`, or `"proposal"`.
    pub kind: &'static str,
    /// The row's own pid.
    pub pid: Uuid,
    /// The row's own `updated_at`.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A scenario evaluation: totals + named constraint violations.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Evaluation {
    /// Total planned/requested spend per currency (minor units).
    pub planned_by_currency: Vec<(String, i64)>,
    /// Summed open risk exposure across members.
    pub total_exposure: i32,
    /// Summed OKR alignment weight across members.
    pub total_alignment: i32,
    /// Human-readable constraint violations (empty ⇒ feasible).
    pub violations: Vec<String>,
}

/// Evaluate a candidate portfolio: sum per-currency spend, exposure,
/// and alignment; check the budget cap (same-currency only) and the
/// must-include list. Pure arithmetic over prepared facts.
#[must_use]
pub fn evaluate_scenario(members: &[MemberFact], constraints: &Constraints) -> Evaluation {
    let mut planned: Vec<(String, i64)> = Vec::new();
    for member in members {
        for (currency, amount) in &member.planned_by_currency {
            match planned.iter_mut().find(|(c, _)| c == currency) {
                Some((_, total)) => *total = total.saturating_add(*amount),
                None => planned.push((currency.clone(), *amount)),
            }
        }
    }
    planned.sort();
    let mut violations = Vec::new();
    if let (Some(cap), Some(cap_currency)) = (
        constraints.budget_cap_minor,
        constraints.currency.as_deref(),
    ) {
        let total = planned
            .iter()
            .find(|(c, _)| c == cap_currency)
            .map_or(0, |(_, t)| *t);
        if total > cap {
            violations.push(format!(
                "budget cap exceeded: {total} > {cap} {cap_currency} (minor units)"
            ));
        }
    }
    let member_pids: std::collections::HashSet<Uuid> = members.iter().map(|m| m.pid).collect();
    for required in &constraints.must_include {
        if !member_pids.contains(required) {
            violations.push(format!("must-include member {required} is missing"));
        }
    }
    Evaluation {
        planned_by_currency: planned,
        total_exposure: members.iter().map(|m| m.open_exposure).sum(),
        total_alignment: members.iter().map(|m| m.alignment_weight).sum(),
        violations,
    }
}

/// One candidate's cost/score facts for the deterministic generator
/// (T-28h). Populated by the caller (Smart Score evidence + the
/// candidate's budget lines summed in the cap's currency); the
/// selection itself stays pure.
#[derive(Debug, Clone)]
pub struct GenerateCandidateFact {
    /// The candidate plan's pid.
    pub pid: Uuid,
    /// `None` when Smart Score found no evidence at all for this plan.
    pub score: Option<f64>,
    /// Cost in the cap's currency, minor units. `None` when the
    /// candidate carries budget lines but **none** in that currency
    /// (genuinely incomparable — `foreign_currency`); `Some(0)` when
    /// it carries no budget lines at all (a free candidate, ranked on
    /// score alone).
    pub cost_minor: Option<i64>,
}

/// Why a candidate did not make the generated scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludeReason {
    /// No Smart Score evidence at all — listed unranked, never scored
    /// zero.
    NoScore,
    /// Carries budget lines, but none in the cap's currency.
    ForeignCurrency,
    /// Scored and priced, but the remaining budget ran out first.
    OverCap,
    /// A `must_include` pid that names no known candidate.
    MustIncludeConflict,
}

/// One candidate's rationale row: included (with its score and cost)
/// or excluded (with its reason). Every candidate — and every
/// unresolvable `must_include` pid — appears in exactly one row.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GenerateRationale {
    /// The candidate's pid (or the unresolved `must_include` pid).
    pub pid: Uuid,
    /// Whether this pid made the generated scenario's membership.
    pub included: bool,
    /// The Smart Score, when the candidate had one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// The cost that was weighed, when one could be computed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_minor: Option<i64>,
    /// Why excluded — absent when `included`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ExcludeReason>,
}

/// Greedily select candidates by Smart Score per unit cost, within an
/// optional budget cap (`None` ⇒ no cap, every scored candidate
/// fits). `must_include` members are force-included first, regardless
/// of cap or cost — a mandatory member is never silently dropped; if
/// that alone exceeds the cap, the remaining budget goes negative and
/// every other candidate is reported `over_cap` in consequence (the
/// resulting scenario's own infeasibility then surfaces normally
/// through `evaluate`, exactly as a planner-built scenario's would).
///
/// Fully deterministic: candidates are sorted by pid before anything
/// else, so the result depends only on the **set** of facts passed
/// in, never on the caller's iteration order — same inputs always
/// produce byte-identical output.
#[must_use]
pub fn generate_scenario(
    mut candidates: Vec<GenerateCandidateFact>,
    budget_cap_minor: Option<i64>,
    must_include: &[Uuid],
) -> Vec<GenerateRationale> {
    candidates.sort_by_key(|c| c.pid);
    let must_include_set: std::collections::BTreeSet<Uuid> = must_include.iter().copied().collect();
    let known: std::collections::BTreeSet<Uuid> = candidates.iter().map(|c| c.pid).collect();

    let mut rationale = Vec::new();

    // A must-include pid with no matching candidate: a real conflict,
    // named once rather than silently dropped.
    for pid in &must_include_set {
        if !known.contains(pid) {
            rationale.push(GenerateRationale {
                pid: *pid,
                included: false,
                score: None,
                cost_minor: None,
                reason: Some(ExcludeReason::MustIncludeConflict),
            });
        }
    }

    // Force-include every must-include candidate, unconditionally.
    let mut remaining_cap = budget_cap_minor;
    for c in candidates
        .iter()
        .filter(|c| must_include_set.contains(&c.pid))
    {
        rationale.push(GenerateRationale {
            pid: c.pid,
            included: true,
            score: c.score,
            cost_minor: c.cost_minor,
            reason: None,
        });
        remaining_cap = remaining_cap.map(|cap| cap.saturating_sub(c.cost_minor.unwrap_or(0)));
    }

    // The rest: no-score and foreign-currency exclude outright; the
    // scored, priceable remainder ranks by score-per-unit-cost.
    let mut ranked: Vec<&GenerateCandidateFact> = Vec::new();
    for c in candidates
        .iter()
        .filter(|c| !must_include_set.contains(&c.pid))
    {
        match c.score {
            None => rationale.push(GenerateRationale {
                pid: c.pid,
                included: false,
                score: None,
                cost_minor: c.cost_minor,
                reason: Some(ExcludeReason::NoScore),
            }),
            Some(score) if budget_cap_minor.is_some() && c.cost_minor.is_none() => {
                rationale.push(GenerateRationale {
                    pid: c.pid,
                    included: false,
                    score: Some(score),
                    cost_minor: None,
                    reason: Some(ExcludeReason::ForeignCurrency),
                });
            }
            Some(_) => ranked.push(c),
        }
    }

    // Score per unit cost, descending; a zero/free cost ranks as the
    // best possible ratio. Ties break on score, then (via the earlier
    // pid sort + a stable sort here) on pid — so the order never
    // depends on anything but the facts themselves.
    ranked.sort_by(|a, b| {
        let ratio = |c: &GenerateCandidateFact| -> f64 {
            let cost = c.cost_minor.unwrap_or(0);
            let score = c.score.unwrap_or(0.0);
            if cost <= 0 {
                f64::INFINITY
            } else {
                // A ranking heuristic, not stored money: the ratio
                // only needs to order candidates, so `f64`'s ~15
                // significant digits of precision at typical minor-
                // unit magnitudes is not a correctness concern.
                #[expect(clippy::cast_precision_loss)]
                let cost = cost as f64;
                score / cost
            }
        };
        ratio(b)
            .total_cmp(&ratio(a))
            .then_with(|| b.score.unwrap_or(0.0).total_cmp(&a.score.unwrap_or(0.0)))
    });

    for c in ranked {
        let cost = c.cost_minor.unwrap_or(0);
        let fits = remaining_cap.is_none_or(|cap| cost <= cap);
        if fits {
            rationale.push(GenerateRationale {
                pid: c.pid,
                included: true,
                score: c.score,
                cost_minor: c.cost_minor,
                reason: None,
            });
            remaining_cap = remaining_cap.map(|cap| cap - cost);
        } else {
            rationale.push(GenerateRationale {
                pid: c.pid,
                included: false,
                score: c.score,
                cost_minor: c.cost_minor,
                reason: Some(ExcludeReason::OverCap),
            });
        }
    }

    rationale
}

/// One point of the efficient frontier: what the generator selects at one budget cap.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FrontierPoint {
    /// The budget cap this point was generated at, minor units.
    pub budget_cap_minor: i64,
    /// Sum of the Smart Scores of the included candidates.
    pub value: f64,
    /// Sum of the costs of the included candidates, minor units.
    pub cost_minor: i64,
    /// How many candidates are included.
    pub included: usize,
    /// True when a point at a lower cap reaches at least this value, so spending more here
    /// buys nothing (the greedy selection is not optimal, and says so).
    pub dominated: bool,
    /// The same explainable rows a single `generate_scenario` run at this cap returns.
    pub rationale: Vec<GenerateRationale>,
}

/// Run [`generate_scenario`] once for each cap (sorted, duplicates dropped) and total the value
/// and cost of what it selects. Each point is exactly the single-cap run's selection.
#[must_use]
pub fn frontier(
    candidates: &[GenerateCandidateFact],
    caps: &[i64],
    must_include: &[Uuid],
) -> Vec<FrontierPoint> {
    let mut caps = caps.to_vec();
    caps.sort_unstable();
    caps.dedup();
    let mut best = f64::NEG_INFINITY;
    caps.into_iter()
        .map(|cap| {
            let rationale = generate_scenario(candidates.to_vec(), Some(cap), must_include);
            let chosen = rationale.iter().filter(|r| r.included);
            let (value, cost_minor, included) = chosen.fold((0.0, 0_i64, 0), |(v, c, n), r| {
                (
                    v + r.score.unwrap_or(0.0),
                    c.saturating_add(r.cost_minor.unwrap_or(0)),
                    n + 1,
                )
            });
            let dominated = value <= best;
            best = best.max(value);
            FrontierPoint {
                budget_cap_minor: cap,
                value,
                cost_minor,
                included,
                dominated,
                rationale,
            }
        })
        .collect()
}

/// Simple ROI in basis points: `(realized − cost) / cost × 10_000`.
/// `None` when the cost is zero or negative (undefined, never a
/// divide-by-zero panic).
#[must_use]
pub fn roi_basis_points(realized_minor: i64, cost_minor: i64) -> Option<i64> {
    if cost_minor <= 0 {
        return None;
    }
    let delta = realized_minor.checked_sub(cost_minor)?;
    delta.checked_mul(10_000).map(|scaled| scaled / cost_minor)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn member(n: u128, planned: &[(&str, i64)], exposure: i32, alignment: i32) -> MemberFact {
        MemberFact {
            pid: pid(n),
            planned_by_currency: planned
                .iter()
                .map(|(c, a)| ((*c).to_string(), *a))
                .collect(),
            open_exposure: exposure,
            alignment_weight: alignment,
        }
    }

    /// Totals sum per currency; cap and must-include violations name
    /// themselves; a feasible scenario has none.
    #[test]
    fn scenario_evaluation() {
        let members = vec![
            member(1, &[("GBP", 500_000)], 12, 8),
            member(2, &[("GBP", 300_000), ("EUR", 100_000)], 6, 3),
        ];
        let feasible = evaluate_scenario(
            &members,
            &Constraints {
                budget_cap_minor: Some(1_000_000),
                currency: Some("GBP".to_string()),
                must_include: vec![pid(1)],
            },
        );
        assert_eq!(
            feasible.planned_by_currency,
            vec![("EUR".to_string(), 100_000), ("GBP".to_string(), 800_000)]
        );
        assert_eq!(feasible.total_exposure, 18);
        assert_eq!(feasible.total_alignment, 11);
        assert!(feasible.violations.is_empty());

        let violated = evaluate_scenario(
            &members,
            &Constraints {
                budget_cap_minor: Some(700_000),
                currency: Some("GBP".to_string()),
                must_include: vec![pid(9)],
            },
        );
        assert_eq!(violated.violations.len(), 2);
        assert!(violated.violations[0].contains("budget cap exceeded"));
        assert!(violated.violations[1].contains("missing"));
        // A cap in a currency nobody spends is not violated.
        let other = evaluate_scenario(
            &members,
            &Constraints {
                budget_cap_minor: Some(1),
                currency: Some("USD".to_string()),
                must_include: vec![],
            },
        );
        assert!(other.violations.is_empty());
    }

    /// ROI: positive, negative, and the undefined zero-cost case.
    #[test]
    fn roi_math() {
        assert_eq!(roi_basis_points(15_000, 10_000), Some(5_000)); // +50%
        assert_eq!(roi_basis_points(5_000, 10_000), Some(-5_000)); // −50%
        assert_eq!(roi_basis_points(10_000, 10_000), Some(0));
        assert_eq!(roi_basis_points(1, 0), None);
        assert_eq!(roi_basis_points(1, -5), None);
    }

    /// Weight bounds.
    #[test]
    fn weights() {
        assert!(valid_weight(1));
        assert!(valid_weight(5));
        assert!(!valid_weight(0));
        assert!(!valid_weight(6));
    }

    fn candidate(n: u128, score: Option<f64>, cost: Option<i64>) -> GenerateCandidateFact {
        GenerateCandidateFact {
            pid: pid(n),
            score,
            cost_minor: cost,
        }
    }

    fn rationale_of(rows: &[GenerateRationale], p: Uuid) -> &GenerateRationale {
        rows.iter()
            .find(|r| r.pid == p)
            .unwrap_or_else(|| panic!("no rationale row for {p}"))
    }

    /// The generator (T-28h): higher score-per-cost wins within the
    /// cap; a no-score candidate is excluded `no_score`, never scored
    /// zero; a candidate priced in another currency is
    /// `foreign_currency`; a free (no budget lines) candidate ranks on
    /// score alone; every candidate appears exactly once.
    #[test]
    fn generator_ranks_by_score_per_cost_within_cap() {
        let candidates = vec![
            candidate(1, Some(80.0), Some(400_000)), // 0.0002 / minor
            candidate(2, Some(40.0), Some(100_000)), // 0.0004 / minor — best ratio
            candidate(3, None, Some(50_000)),        // no_score
            candidate(4, Some(90.0), None),          // foreign_currency (capped run)
            candidate(5, Some(10.0), Some(0)),       // free: ranks on score alone
        ];
        let rows = generate_scenario(candidates, Some(150_000), &[]);
        assert_eq!(rows.len(), 5, "every candidate appears exactly once");

        // Free candidate (5) and best-ratio candidate (2) both fit
        // 150_000; candidate 1 does not (would need 400_000 more).
        assert!(rationale_of(&rows, pid(5)).included);
        assert!(rationale_of(&rows, pid(2)).included);
        assert!(!rationale_of(&rows, pid(1)).included);
        assert_eq!(
            rationale_of(&rows, pid(1)).reason,
            Some(ExcludeReason::OverCap)
        );
        assert_eq!(
            rationale_of(&rows, pid(3)).reason,
            Some(ExcludeReason::NoScore)
        );
        assert!(rationale_of(&rows, pid(3)).score.is_none());
        assert_eq!(
            rationale_of(&rows, pid(4)).reason,
            Some(ExcludeReason::ForeignCurrency)
        );
    }

    /// A must-include candidate is force-included even when it alone
    /// exceeds the cap — never silently dropped — and consequently
    /// blocks every other candidate via `over_cap`, not silence.
    #[test]
    fn must_include_is_never_silently_dropped_even_over_cap() {
        let candidates = vec![
            candidate(1, Some(50.0), Some(900_000)), // must-include, alone over cap
            candidate(2, Some(99.0), Some(1_000)),   // would otherwise easily win
        ];
        let rows = generate_scenario(candidates, Some(100_000), &[pid(1)]);
        assert!(
            rationale_of(&rows, pid(1)).included,
            "forced in regardless of cap"
        );
        assert!(
            !rationale_of(&rows, pid(2)).included,
            "the cap is already blown by the mandatory member"
        );
        assert_eq!(
            rationale_of(&rows, pid(2)).reason,
            Some(ExcludeReason::OverCap)
        );
    }

    /// A `must_include` pid naming no real candidate is reported, not
    /// silently absent.
    #[test]
    fn an_unresolvable_must_include_is_named_as_a_conflict() {
        let candidates = vec![candidate(1, Some(50.0), Some(1_000))];
        let ghost = pid(999);
        let rows = generate_scenario(candidates, None, &[ghost]);
        assert_eq!(
            rationale_of(&rows, ghost).reason,
            Some(ExcludeReason::MustIncludeConflict)
        );
    }

    /// No cap at all ⇒ every scored candidate is included regardless
    /// of cost; only `no_score` excludes.
    #[test]
    fn no_cap_includes_every_scored_candidate() {
        let candidates = vec![
            candidate(1, Some(10.0), Some(10_000_000)),
            candidate(2, None, Some(1)),
        ];
        let rows = generate_scenario(candidates, None, &[]);
        assert!(rationale_of(&rows, pid(1)).included);
        assert_eq!(
            rationale_of(&rows, pid(2)).reason,
            Some(ExcludeReason::NoScore)
        );
    }

    /// Same facts, different starting order (simulating a `HashMap`'s
    /// unordered iteration) ⇒ byte-identical output.
    #[test]
    fn same_inputs_produce_byte_identical_output_regardless_of_input_order() {
        let a = vec![
            candidate(3, Some(20.0), Some(30_000)),
            candidate(1, Some(80.0), Some(10_000)),
            candidate(2, Some(50.0), Some(20_000)),
        ];
        let mut b = a.clone();
        b.reverse();
        let rows_a = generate_scenario(a, Some(50_000), &[]);
        let rows_b = generate_scenario(b, Some(50_000), &[]);
        let json_a = serde_json::to_string(&rows_a).unwrap();
        let json_b = serde_json::to_string(&rows_b).unwrap();
        assert_eq!(json_a, json_b);
    }

    fn fixture() -> Vec<GenerateCandidateFact> {
        vec![
            candidate(1, Some(80.0), Some(10_000)),
            candidate(2, Some(50.0), Some(20_000)),
            candidate(3, Some(20.0), Some(30_000)),
        ]
    }

    #[test]
    fn the_frontier_rises_with_the_cap_and_each_point_is_a_single_cap_run() {
        let caps = [60_000, 0, 10_000, 30_000, 10_000];
        let points = frontier(&fixture(), &caps, &[]);
        let at: Vec<i64> = points.iter().map(|p| p.budget_cap_minor).collect();
        assert_eq!(at, vec![0, 10_000, 30_000, 60_000], "sorted, deduplicated");
        let values: Vec<f64> = points.iter().map(|p| p.value).collect();
        assert!(values.windows(2).all(|w| w[0] <= w[1]), "{values:?}");
        assert_eq!(values, vec![0.0, 80.0, 130.0, 150.0]);
        assert_eq!(points[2].cost_minor, 30_000);
        assert_eq!(points[2].included, 2);
        for p in &points {
            let single = generate_scenario(fixture(), Some(p.budget_cap_minor), &[]);
            assert_eq!(p.rationale, single);
            assert!(!p.dominated);
        }
    }

    #[test]
    fn a_point_that_buys_nothing_more_is_marked_dominated() {
        let points = frontier(&fixture(), &[60_000, 600_000], &[]);
        assert!(!points[0].dominated);
        assert!(points[1].dominated, "ten times the money, the same value");
        assert_eq!(points[0].value, points[1].value);
    }
}
