//! Pure derivations for the executive insight areas (CEO / CFO / CTO)
//! — the money aggregation, technology-radar tag convention, benefit
//! realization ratio, and dependency fan-out counting behind
//! `controllers::insights`. No I/O; every number a served view shows is
//! derived here (or in [`crate::visibility`]) so the derivations are
//! unit-testable and the honesty rules are pinned in one place:
//! per-currency lines never merge, ratios carry their numerator and
//! denominator, and absent data is `None`, never `0`.

use std::collections::BTreeMap;

use serde::Serialize;
use uuid::Uuid;

/// One money observation: a currency plus planned/actual minor units.
#[derive(Debug, Clone)]
pub struct MoneyLine {
    /// ISO-4217 currency code.
    pub currency: String,
    /// Planned (committed) amount in minor units.
    pub planned_minor: i64,
    /// Actual (spent) amount in minor units.
    pub actual_minor: i64,
}

/// Per-currency variance rollup. Currencies are **never** merged and
/// there is **no** FX conversion — one row per currency.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VarianceRow {
    /// ISO-4217 currency code.
    pub currency: String,
    /// Sum of planned minor units in this currency.
    pub planned_minor: i64,
    /// Sum of actual minor units in this currency.
    pub actual_minor: i64,
    /// `planned - actual` (negative = overrun), minor units.
    pub remaining_minor: i64,
    /// Whether actual exceeds planned in this currency.
    pub overrun: bool,
    /// How many lines contributed to this row.
    pub line_count: usize,
}

/// The portfolio a plan rolls up to: the plan itself when it is a portfolio, else the nearest ancestor that
/// is one (a project under a programme under a portfolio rolls up to the portfolio), else `None`. Walks at most
/// 64 steps, so a cycle in bad data ends instead of looping.
#[must_use]
pub fn portfolio_of(
    pid: Uuid,
    kind_of: &BTreeMap<Uuid, &str>,
    parent_of: &BTreeMap<Uuid, Option<Uuid>>,
) -> Option<Uuid> {
    let mut cur = Some(pid);
    for _ in 0..64 {
        let p = cur?;
        if kind_of.get(&p) == Some(&"Portfolio") {
            return Some(p);
        }
        cur = parent_of.get(&p).copied().flatten();
    }
    None
}

/// Aggregate money lines into one variance row per currency
/// (deterministic order: currency code ascending).
#[must_use]
pub fn variance_by_currency(lines: &[MoneyLine]) -> Vec<VarianceRow> {
    let mut per: BTreeMap<&str, (i64, i64, usize)> = BTreeMap::new();
    for line in lines {
        let entry = per.entry(line.currency.as_str()).or_default();
        entry.0 = entry.0.saturating_add(line.planned_minor);
        entry.1 = entry.1.saturating_add(line.actual_minor);
        entry.2 += 1;
    }
    per.into_iter()
        .map(|(currency, (planned, actual, count))| VarianceRow {
            currency: currency.to_string(),
            planned_minor: planned,
            actual_minor: actual,
            remaining_minor: planned.saturating_sub(actual),
            overrun: actual > planned,
            line_count: count,
        })
        .collect()
}

/// One group of a grouped report: the group value, how many plans fell
/// in it, and its money lines rolled up **per currency** — never one
/// cross-currency sum (T-28e).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ReportGroup {
    /// The grouped field's value (`unspecified` when the plan has none).
    pub value: String,
    /// Plans in the group.
    pub count: usize,
    /// One variance row per currency; empty when the group has no lines.
    pub money: Vec<VarianceRow>,
}

/// Group `(group value, money lines)` entries — one per selected plan —
/// into [`ReportGroup`]s, ordered by group value ascending.
#[must_use]
pub fn group_report(entries: Vec<(String, Vec<MoneyLine>)>) -> Vec<ReportGroup> {
    let mut per: BTreeMap<String, (usize, Vec<MoneyLine>)> = BTreeMap::new();
    for (value, lines) in entries {
        let entry = per.entry(value).or_default();
        entry.0 += 1;
        entry.1.extend(lines);
    }
    per.into_iter()
        .map(|(value, (count, lines))| ReportGroup {
            value,
            count,
            money: variance_by_currency(&lines),
        })
        .collect()
}

/// The four technology-radar rings, outermost first.
pub const RADAR_RINGS: [&str; 4] = ["assess", "trial", "adopt", "hold"];

/// Parse one plan tag against the radar convention:
/// `tech:<name>` or `tech:<name>:<ring>` (ring ∈ [`RADAR_RINGS`]).
/// The name is lowercased and trimmed; blank names and unknown rings
/// are rejected (`None`) rather than guessed.
#[must_use]
pub fn parse_tech_tag(tag: &str) -> Option<(String, Option<String>)> {
    let rest = tag.trim().strip_prefix("tech:")?;
    let (name, ring) = match rest.split_once(':') {
        Some((name, ring)) => {
            let ring = ring.trim().to_lowercase();
            if !RADAR_RINGS.contains(&ring.as_str()) {
                return None;
            }
            (name, Some(ring))
        }
        None => (rest, None),
    };
    let name = name.trim().to_lowercase();
    if name.is_empty() {
        return None;
    }
    Some((name, ring))
}

/// The consensus ring for a technology from its declared rings: the
/// most-declared ring wins; ties break toward the more cautious ring
/// (earlier in [`RADAR_RINGS`]); no declarations ⇒ `unclassified`.
#[must_use]
pub fn ring_consensus(rings: &[String]) -> &'static str {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for ring in rings {
        if let Some(known) = RADAR_RINGS.iter().find(|r| **r == ring.as_str()) {
            *counts.entry(known).or_default() += 1;
        }
    }
    let best = RADAR_RINGS
        .iter()
        .filter_map(|ring| counts.get(ring).map(|n| (*ring, *n)))
        .max_by(|a, b| a.1.cmp(&b.1)); // stable: first max in ring order wins ties
    match best {
        Some((ring, _)) => RADAR_RINGS
            .iter()
            .filter(|r| counts.get(**r) == counts.get(ring))
            .copied()
            .next()
            .unwrap_or("unclassified"),
        None => "unclassified",
    }
}

/// Benefit realization ratio: `realized / target`, or `None` when no
/// positive target exists (a ratio against zero would be an invented
/// number).
#[must_use]
pub fn realization_ratio(target_minor: i64, realized_minor: i64) -> Option<f64> {
    if target_minor <= 0 {
        return None;
    }
    #[allow(clippy::cast_precision_loss)] // display ratio, not money math
    Some(realized_minor as f64 / target_minor as f64)
}

/// Dependents per predecessor (fan-out), most-depended-on first, then
/// by pid for determinism. An item many others depend on is a
/// single-point-of-failure candidate.
#[must_use]
pub fn fan_out(edges: &[(Uuid, Uuid)]) -> Vec<(Uuid, usize)> {
    let mut per: BTreeMap<Uuid, usize> = BTreeMap::new();
    for (predecessor, _successor) in edges {
        *per.entry(*predecessor).or_default() += 1;
    }
    let mut out: Vec<(Uuid, usize)> = per.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

/// Heatmap cell counts over `(probability, impact)` pairs, keyed
/// `"p{P}i{I}"` (both clamped to 1–5 for display stability).
#[must_use]
pub fn heatmap_cells(pairs: &[(i32, i32)]) -> BTreeMap<String, usize> {
    let mut cells = BTreeMap::new();
    for (probability, impact) in pairs {
        let key = format!("p{}i{}", probability.clamp(&1, &5), impact.clamp(&1, &5));
        *cells.entry(key).or_default() += 1;
    }
    cells
}

/// A deployment's declared risk appetite (absent ⇒ no thresholds).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RiskAppetite {
    /// Maximum tolerated estate-wide open exposure.
    pub max_open_exposure: Option<i32>,
    /// Maximum tolerated single-risk exposure.
    pub max_item_exposure: Option<i32>,
}

/// Parse the `PROJECT_PORTFOLIO_MANAGEMENT_RISK_APPETITE` JSON. Absent
/// / blank / unparsable ⇒ `None` (no appetite configured; views say so
/// rather than inventing thresholds).
#[must_use]
pub fn parse_appetite(raw: Option<&str>) -> Option<RiskAppetite> {
    let raw = raw?.trim();
    if raw.is_empty() {
        return None;
    }
    serde_json::from_str(raw).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(currency: &str, planned: i64, actual: i64) -> MoneyLine {
        MoneyLine {
            currency: currency.into(),
            planned_minor: planned,
            actual_minor: actual,
        }
    }

    #[test]
    fn grouped_report_never_sums_across_currencies() {
        let groups = group_report(vec![
            (
                "active".into(),
                vec![line("GBP", 100, 40), line("EUR", 200, 50)],
            ),
            ("active".into(), vec![line("GBP", 10, 5)]),
            ("closed".into(), vec![]),
        ]);
        assert_eq!(groups.len(), 2);
        let active = &groups[0];
        assert_eq!((active.value.as_str(), active.count), ("active", 2));
        // Two currencies in the group → two rows, never one sum.
        assert_eq!(active.money.len(), 2);
        assert_eq!(active.money[0].currency, "EUR");
        assert_eq!(active.money[1].planned_minor, 110);
        // A group with no lines reports empty money, not a zero row.
        assert_eq!(groups[1].count, 1);
        assert!(groups[1].money.is_empty());
    }

    #[test]
    fn variance_keeps_currencies_apart_and_flags_overrun() {
        let rows = variance_by_currency(&[
            MoneyLine {
                currency: "GBP".into(),
                planned_minor: 10_000,
                actual_minor: 12_000,
            },
            MoneyLine {
                currency: "GBP".into(),
                planned_minor: 5_000,
                actual_minor: 1_000,
            },
            MoneyLine {
                currency: "USD".into(),
                planned_minor: 7_000,
                actual_minor: 7_000,
            },
        ]);
        assert_eq!(rows.len(), 2, "one row per currency, never merged");
        let gbp = &rows[0];
        assert_eq!(
            (gbp.currency.as_str(), gbp.planned_minor, gbp.actual_minor),
            ("GBP", 15_000, 13_000)
        );
        assert_eq!(gbp.remaining_minor, 2_000);
        assert!(!gbp.overrun);
        assert_eq!(gbp.line_count, 2);
        let usd = &rows[1];
        assert!(!usd.overrun, "actual == planned is not an overrun");

        let overrun = variance_by_currency(&[MoneyLine {
            currency: "EUR".into(),
            planned_minor: 100,
            actual_minor: 101,
        }]);
        assert!(overrun[0].overrun);
        assert_eq!(overrun[0].remaining_minor, -1);
    }

    #[test]
    fn tech_tags_parse_and_reject() {
        assert_eq!(parse_tech_tag("tech:rust"), Some(("rust".into(), None)));
        assert_eq!(
            parse_tech_tag("tech:Rust:Adopt"),
            Some(("rust".into(), Some("adopt".into())))
        );
        assert_eq!(
            parse_tech_tag("tech: postgres :trial"),
            Some(("postgres".into(), Some("trial".into())))
        );
        assert_eq!(parse_tech_tag("tech:"), None, "blank name refused");
        assert_eq!(
            parse_tech_tag("tech:x:sideways"),
            None,
            "unknown ring refused"
        );
        assert_eq!(parse_tech_tag("owner:alice"), None, "non-tech tag ignored");
    }

    #[test]
    fn ring_consensus_majority_ties_and_default() {
        let r =
            |xs: &[&str]| ring_consensus(&xs.iter().map(|s| (*s).to_string()).collect::<Vec<_>>());
        assert_eq!(r(&["adopt", "adopt", "trial"]), "adopt");
        assert_eq!(r(&["assess", "adopt"]), "assess", "tie breaks cautious");
        assert_eq!(r(&[]), "unclassified");
        assert_eq!(
            r(&["nonsense"]),
            "unclassified",
            "unknown rings never count"
        );
    }

    #[test]
    fn realization_ratio_never_divides_by_nothing() {
        assert_eq!(realization_ratio(0, 500), None);
        assert_eq!(realization_ratio(-1, 500), None);
        assert!((realization_ratio(1_000, 250).unwrap() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn fan_out_orders_by_dependents_then_pid() {
        let a = Uuid::from_u128(1);
        let b = Uuid::from_u128(2);
        let c = Uuid::from_u128(3);
        let out = fan_out(&[(a, c), (b, c), (a, b)]);
        assert_eq!(out[0], (a, 2));
        assert_eq!(out[1], (b, 1));
    }
    #[test]
    fn heatmap_clamps_and_counts() {
        let cells = heatmap_cells(&[(1, 1), (1, 1), (9, 0)]);
        assert_eq!(cells["p1i1"], 2);
        assert_eq!(cells["p5i1"], 1, "out-of-range clamps for display");
    }

    #[test]
    fn a_project_rolls_up_through_its_programme_to_the_portfolio() {
        let (pf, pg, pj, orphan) = (
            Uuid::from_u128(1),
            Uuid::from_u128(2),
            Uuid::from_u128(3),
            Uuid::from_u128(4),
        );
        let kinds: BTreeMap<Uuid, &str> = BTreeMap::from([
            (pf, "Portfolio"),
            (pg, "Program"),
            (pj, "Project"),
            (orphan, "Project"),
        ]);
        let parents: BTreeMap<Uuid, Option<Uuid>> =
            BTreeMap::from([(pf, None), (pg, Some(pf)), (pj, Some(pg)), (orphan, None)]);
        assert_eq!(
            portfolio_of(pf, &kinds, &parents),
            Some(pf),
            "a portfolio is its own"
        );
        assert_eq!(portfolio_of(pg, &kinds, &parents), Some(pf));
        assert_eq!(
            portfolio_of(pj, &kinds, &parents),
            Some(pf),
            "two levels up"
        );
        assert_eq!(
            portfolio_of(orphan, &kinds, &parents),
            None,
            "no portfolio above"
        );
    }

    #[test]
    fn a_cycle_in_the_parents_ends_the_walk() {
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        let kinds: BTreeMap<Uuid, &str> = BTreeMap::from([(a, "Project"), (b, "Project")]);
        let parents: BTreeMap<Uuid, Option<Uuid>> = BTreeMap::from([(a, Some(b)), (b, Some(a))]);
        assert_eq!(portfolio_of(a, &kinds, &parents), None);
    }

    #[test]
    fn appetite_parses_or_declines() {
        assert_eq!(parse_appetite(None), None);
        assert_eq!(parse_appetite(Some("  ")), None);
        assert_eq!(parse_appetite(Some("not json")), None);
        let a = parse_appetite(Some(r#"{"max_open_exposure": 60}"#)).unwrap();
        assert_eq!(a.max_open_exposure, Some(60));
        assert_eq!(a.max_item_exposure, None);
    }
}
