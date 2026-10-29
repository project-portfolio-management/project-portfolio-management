//! EVM analytics (SC-5): forecasts from earned value. Pure, DB-free, unit-tested.
//!
//! Where [`crate::earned_value`] says *where the plan is*, this says *where it is
//! heading*: **EAC** (estimate at completion), **ETC** (estimate to complete),
//! **VAC** (variance at completion, BAC − EAC), and **TCPI** (the cost efficiency
//! the remaining work would need to hit a target). Every forecast **names its
//! formula**; none is presented as *the* answer, because the right one depends on
//! whether past overruns will continue, and this module cannot know that.
//!
//! | variant | EAC | assumes |
//! |---|---|---|
//! | `budget_rate` | AC + (BAC − EV) | the overrun so far was a one-off |
//! | `cpi` | AC + (BAC − EV) ÷ CPI | today's cost efficiency continues |
//! | `cpi_spi` | AC + (BAC − EV) ÷ (CPI × SPI) | cost *and* schedule pressure both continue |
//! | `bottom_up` | AC + ETC from the plan's own estimate (T-28b) | someone re-estimated |
//!
//! Money is minor units in `i64`, ratios basis points, arithmetic `i128` and
//! integer. Undefined is `None` with a reason, never zero.

use std::collections::BTreeMap;

use serde::Serialize;

/// Why a figure is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Absent {
    /// Actual cost is not known.
    NoActualCost,
    /// The ratio it divides by is missing or zero.
    UndefinedDenominator,
    /// No one has supplied a bottom-up estimate (a TPC cost-estimate-to-complete).
    NoBottomUpEstimate,
    /// The budget is already spent (or overspent): there is no budget left to divide by.
    BudgetExhausted,
    /// Nothing remains to spend.
    NoRemainingCost,
}

/// An amount, or why there is none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Amount {
    /// Minor units.
    pub minor: Option<i64>,
    /// Why, when `minor` is `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent: Option<Absent>,
}

impl Amount {
    fn of(minor: i64) -> Self {
        Self {
            minor: Some(minor),
            absent: None,
        }
    }
    fn missing(why: Absent) -> Self {
        Self {
            minor: None,
            absent: Some(why),
        }
    }
}

/// One forecast of the final cost, named by its formula.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Eac {
    /// `budget_rate`, `cpi`, `cpi_spi` or `bottom_up`.
    pub variant: &'static str,
    /// The formula in words.
    pub formula: &'static str,
    /// The forecast.
    pub eac: Amount,
    /// EAC − AC.
    pub etc: Amount,
    /// BAC − EAC (negative is an overrun).
    pub vac: Amount,
}

/// Integer division rounded to nearest, halves away from zero. `d` must not be zero.
fn round_div(n: i128, d: i128) -> i128 {
    let (q, r) = (n / d, n % d);
    if r.abs() * 2 >= d.abs() {
        q + n.signum() * d.signum()
    } else {
        q
    }
}

fn eac_row(
    variant: &'static str,
    formula: &'static str,
    bac: i64,
    ac: Option<i64>,
    eac: Result<i64, Absent>,
) -> Eac {
    match (ac, eac) {
        (Some(ac), Ok(e)) => Eac {
            variant,
            formula,
            eac: Amount::of(e),
            etc: Amount::of(e - ac),
            vac: Amount::of(bac - e),
        },
        (_, Err(why)) => Eac {
            variant,
            formula,
            eac: Amount::missing(why),
            etc: Amount::missing(why),
            vac: Amount::missing(why),
        },
        (None, Ok(_)) => Eac {
            variant,
            formula,
            eac: Amount::missing(Absent::NoActualCost),
            etc: Amount::missing(Absent::NoActualCost),
            vac: Amount::missing(Absent::NoActualCost),
        },
    }
}

/// All the EAC variants for one currency's figures.
///
/// `cpi_bp` and `spi_bp` are basis points (`None` when undefined);
/// `bottom_up_eac` is the plan's own estimate at completion (T-28b) when one exists.
#[must_use]
pub fn eac_variants(
    bac: i64,
    ev: i64,
    ac: Option<i64>,
    cpi_bp: Option<i64>,
    spi_bp: Option<i64>,
    bottom_up_eac: Option<i64>,
) -> Vec<Eac> {
    let remaining = i128::from((bac - ev).max(0));
    let acv = ac.map(i128::from);
    let to_i64 = |v: i128| i64::try_from(v).unwrap_or(i64::MAX);
    let budget_rate = acv
        .map(|a| to_i64(a + remaining))
        .ok_or(Absent::NoActualCost);
    let cpi = match (acv, cpi_bp) {
        (None, _) => Err(Absent::NoActualCost),
        (Some(_), Some(c)) if c > 0 => Ok(to_i64(
            acv.unwrap_or(0) + round_div(remaining * 10_000, i128::from(c)),
        )),
        _ => Err(Absent::UndefinedDenominator),
    };
    let cpi_spi = match (acv, cpi_bp, spi_bp) {
        (None, _, _) => Err(Absent::NoActualCost),
        (Some(a), Some(c), Some(s)) if c > 0 && s > 0 => Ok(to_i64(
            a + round_div(remaining * 10_000 * 10_000, i128::from(c) * i128::from(s)),
        )),
        _ => Err(Absent::UndefinedDenominator),
    };
    vec![
        eac_row(
            "budget_rate",
            "AC + (BAC − EV): the overrun so far is a one-off",
            bac,
            ac,
            budget_rate,
        ),
        eac_row(
            "cpi",
            "AC + (BAC − EV) ÷ CPI: today's cost efficiency continues",
            bac,
            ac,
            cpi,
        ),
        eac_row(
            "cpi_spi",
            "AC + (BAC − EV) ÷ (CPI × SPI): cost and schedule pressure both continue",
            bac,
            ac,
            cpi_spi,
        ),
        eac_row(
            "bottom_up",
            "AC + the plan's own estimate to complete (T-28b)",
            bac,
            ac,
            bottom_up_eac.ok_or(Absent::NoBottomUpEstimate),
        ),
    ]
}

/// Thresholds for the traffic lights, per deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Thresholds {
    /// SPI or CPI at or above this is green (basis points).
    pub spi_cpi_amber_bp: i64,
    /// SPI or CPI below this is red; between red and amber is amber.
    pub spi_cpi_red_bp: i64,
    /// TCPI at or below this is green.
    pub tcpi_amber_bp: i64,
    /// TCPI above this is red; between amber and red is amber.
    pub tcpi_red_bp: i64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            spi_cpi_amber_bp: 9_500,
            spi_cpi_red_bp: 9_000,
            tcpi_amber_bp: 10_500,
            tcpi_red_bp: 12_500,
        }
    }
}

/// Parse `spi_cpi_amber=9500,spi_cpi_red=9000,tcpi_amber=10500,tcpi_red=12500`
/// (any subset, any order). A malformed or inconsistent value falls back to the
/// defaults; the bool says whether the defaults were used.
#[must_use]
pub fn parse_thresholds(raw: Option<&str>) -> (Thresholds, bool) {
    let mut t = Thresholds::default();
    let Some(raw) = raw.map(str::trim).filter(|r| !r.is_empty()) else {
        return (t, true);
    };
    for part in raw.split(',') {
        let Some((k, v)) = part.split_once('=') else {
            return (Thresholds::default(), true);
        };
        let Ok(n) = v.trim().parse::<i64>() else {
            return (Thresholds::default(), true);
        };
        match k.trim() {
            "spi_cpi_amber" => t.spi_cpi_amber_bp = n,
            "spi_cpi_red" => t.spi_cpi_red_bp = n,
            "tcpi_amber" => t.tcpi_amber_bp = n,
            "tcpi_red" => t.tcpi_red_bp = n,
            _ => return (Thresholds::default(), true),
        }
    }
    let ok = t.spi_cpi_red_bp <= t.spi_cpi_amber_bp
        && t.tcpi_amber_bp <= t.tcpi_red_bp
        && t.spi_cpi_red_bp > 0
        && t.tcpi_amber_bp > 0;
    if ok {
        (t, false)
    } else {
        (Thresholds::default(), true)
    }
}

/// A traffic light.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rag {
    /// Within tolerance.
    Green,
    /// Slipping.
    Amber,
    /// Off plan.
    Red,
    /// No value to judge.
    Unknown,
}

/// The light for an SPI or CPI.
#[must_use]
pub fn rag_ratio(bp: Option<i64>, t: &Thresholds) -> Rag {
    match bp {
        None => Rag::Unknown,
        Some(v) if v >= t.spi_cpi_amber_bp => Rag::Green,
        Some(v) if v >= t.spi_cpi_red_bp => Rag::Amber,
        Some(_) => Rag::Red,
    }
}

/// The light for a TCPI (higher is worse).
#[must_use]
pub fn rag_tcpi(bp: Option<i64>, t: &Thresholds) -> Rag {
    match bp {
        None => Rag::Unknown,
        Some(v) if v <= t.tcpi_amber_bp => Rag::Green,
        Some(v) if v <= t.tcpi_red_bp => Rag::Amber,
        Some(_) => Rag::Red,
    }
}

/// A TCPI: the cost efficiency the remaining work needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Tcpi {
    /// Basis points; `None` when undefined.
    pub bp: Option<i64>,
    /// Why, when `bp` is `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent: Option<Absent>,
    /// More than 1.0 means the remaining work must be done **more efficiently than
    /// the work so far**.
    pub needs_improved_efficiency: bool,
}

/// TCPI against a target: `(BAC − EV) ÷ (target − AC)`, with BAC or an EAC as the
/// target. A target already spent (or overspent) is `budget_exhausted`, and a
/// target equal to AC with work remaining is `no_remaining_cost`: not infinity.
#[must_use]
pub fn tcpi(bac: i64, ev: i64, ac: Option<i64>, target: i64) -> Tcpi {
    let Some(ac) = ac else {
        return Tcpi {
            bp: None,
            absent: Some(Absent::NoActualCost),
            needs_improved_efficiency: false,
        };
    };
    let work = i128::from((bac - ev).max(0));
    let funds = i128::from(target) - i128::from(ac);
    if funds < 0 {
        return Tcpi {
            bp: None,
            absent: Some(Absent::BudgetExhausted),
            needs_improved_efficiency: work > 0,
        };
    }
    if funds == 0 {
        return if work == 0 {
            Tcpi {
                bp: Some(10_000),
                absent: None,
                needs_improved_efficiency: false,
            }
        } else {
            Tcpi {
                bp: None,
                absent: Some(Absent::NoRemainingCost),
                needs_improved_efficiency: true,
            }
        };
    }
    let v = i64::try_from(round_div(work * 10_000, funds)).unwrap_or(i64::MAX);
    Tcpi {
        bp: Some(v),
        absent: None,
        needs_improved_efficiency: v > 10_000,
    }
}

/// Which variant to lead with, and why — a rule, stated, never a hidden choice.
///
/// Cost efficiency continuing (`cpi`) when the schedule is on track; both pressures
/// (`cpi_spi`) when the schedule is behind the amber line; the one-off reading
/// (`budget_rate`) when there is no CPI to extrapolate. Whatever it picks, the other
/// variants are still returned beside it.
#[must_use]
pub fn recommended(
    cpi_bp: Option<i64>,
    spi_bp: Option<i64>,
    t: &Thresholds,
) -> (&'static str, &'static str) {
    match (cpi_bp, spi_bp) {
        (Some(c), _) if c <= 0 => (
            "budget_rate",
            "CPI is not usable, so the overrun so far is read as a one-off",
        ),
        (None, _) => (
            "budget_rate",
            "no CPI is defined yet, so the overrun so far is read as a one-off",
        ),
        (Some(_), Some(s)) if s > 0 && s < t.spi_cpi_amber_bp => (
            "cpi_spi",
            "the schedule is behind the amber line, so cost and schedule pressure are both assumed to continue",
        ),
        (Some(_), _) => ("cpi", "cost efficiency so far is assumed to continue"),
    }
}

/// The budget-at-completion inclusion thresholds, per currency, in minor units:
/// `PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS` as JSON, e.g. `{"GBP":100000000,"EUR":120000000}`.
/// Unset means no threshold: every programme with figures is included. The flag is true when a value was
/// set but unusable (not an object of non-negative whole numbers keyed by a three-letter code), in which
/// case no threshold is applied and the pack says so.
#[must_use]
pub fn parse_bac_thresholds(raw: Option<&str>) -> (BTreeMap<String, i64>, bool) {
    let Some(raw) = raw.map(str::trim).filter(|r| !r.is_empty()) else {
        return (BTreeMap::new(), false);
    };
    let Ok(map) = serde_json::from_str::<BTreeMap<String, i64>>(raw) else {
        return (BTreeMap::new(), true);
    };
    let ok = map
        .iter()
        .all(|(k, v)| crate::currency::is_code(k) && *v >= 0);
    if ok {
        (map, false)
    } else {
        (BTreeMap::new(), true)
    }
}

/// Whether a currency's summed BAC reaches its threshold. A BAC equal to the threshold is included; a
/// currency with no threshold is always included.
#[must_use]
pub fn meets_bac_threshold(
    bac_minor: i64,
    currency: &str,
    thresholds: &BTreeMap<String, i64>,
) -> bool {
    thresholds.get(currency).is_none_or(|t| bac_minor >= *t)
}

/// Totals for one currency across plans.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Totals {
    /// Σ BAC.
    pub bac_minor: i64,
    /// Σ PV.
    pub pv_minor: i64,
    /// Σ EV.
    pub ev_minor: i64,
    /// Σ AC over plans whose AC is known.
    pub ac_minor: i64,
    /// Plans that contributed.
    pub plans: usize,
    /// Plans whose actual cost is unknown (so Σ AC and CPI are partial).
    pub plans_without_ac: usize,
}

/// Add one plan's figures into the currency's totals — **never across currencies**
/// (the map is keyed by currency).
pub fn add(
    totals: &mut BTreeMap<String, Totals>,
    currency: &str,
    bac: i64,
    pv: i64,
    ev: i64,
    ac: Option<i64>,
) {
    let t = totals.entry(currency.to_string()).or_default();
    t.bac_minor = t.bac_minor.saturating_add(bac);
    t.pv_minor = t.pv_minor.saturating_add(pv);
    t.ev_minor = t.ev_minor.saturating_add(ev);
    t.plans += 1;
    match ac {
        Some(a) => t.ac_minor = t.ac_minor.saturating_add(a),
        None => t.plans_without_ac += 1,
    }
}

/// An SPI or CPI from totals, in basis points; `None` when the denominator is zero.
#[must_use]
pub fn ratio_bp(num: i64, den: i64) -> Option<i64> {
    if den == 0 {
        None
    } else {
        i64::try_from(round_div(i128::from(num) * 10_000, i128::from(den))).ok()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_bac_threshold_includes_the_line_and_names_a_bad_setting() {
        use super::{meets_bac_threshold, parse_bac_thresholds};
        let (t, bad) = parse_bac_thresholds(Some(r#"{"GBP":1000,"EUR":0}"#));
        assert!(!bad);
        assert!(!meets_bac_threshold(999, "GBP", &t));
        assert!(
            meets_bac_threshold(1000, "GBP", &t),
            "exactly the threshold is in"
        );
        assert!(meets_bac_threshold(0, "EUR", &t));
        assert!(
            meets_bac_threshold(1, "USD", &t),
            "no threshold for a currency means in"
        );
        assert_eq!(parse_bac_thresholds(None), (BTreeMap::default(), false));
        assert_eq!(
            parse_bac_thresholds(Some("  ")),
            (BTreeMap::default(), false)
        );
        for junk in [
            "gbp",
            "{\"gbp\":5}",
            "{\"GBP\":-1}",
            "{\"GBP\":1.5}",
            "[1]",
            "{\"GBPX\":5}",
        ] {
            assert_eq!(
                parse_bac_thresholds(Some(junk)),
                (BTreeMap::default(), true),
                "{junk}"
            );
        }
    }

    use super::*;

    fn amount(e: &Eac) -> Option<i64> {
        e.eac.minor
    }
    fn by<'a>(v: &'a [Eac], name: &str) -> &'a Eac {
        v.iter().find(|e| e.variant == name).unwrap()
    }

    // The textbook plan: BAC 10 000, EV 4 000, AC 5 000, PV 5 000 => CPI 0.80, SPI 0.80.
    fn textbook() -> Vec<Eac> {
        eac_variants(10_000, 4_000, Some(5_000), Some(8_000), Some(8_000), None)
    }

    #[test]
    fn each_eac_formula_is_pinned_against_hand_worked_numbers() {
        let v = textbook();
        assert_eq!(
            amount(by(&v, "budget_rate")),
            Some(11_000),
            "5 000 + (10 000 − 4 000)"
        );
        assert_eq!(
            amount(by(&v, "cpi")),
            Some(12_500),
            "10 000 / 0.80 = 12 500 (= AC + 6 000 / 0.80)"
        );
        assert_eq!(
            amount(by(&v, "cpi_spi")),
            Some(14_375),
            "5 000 + 6 000 / (0.80 x 0.80)"
        );
    }

    #[test]
    fn etc_and_vac_follow_from_eac() {
        let v = textbook();
        let cpi = by(&v, "cpi");
        assert_eq!(cpi.etc.minor, Some(7_500), "EAC − AC");
        assert_eq!(cpi.vac.minor, Some(-2_500), "BAC − EAC: a 2 500 overrun");
        assert_eq!(by(&v, "budget_rate").vac.minor, Some(-1_000));
    }

    #[test]
    fn a_missing_denominator_names_itself_and_never_becomes_zero() {
        let none_cpi = eac_variants(10_000, 4_000, Some(5_000), None, Some(8_000), None);
        assert_eq!(
            by(&none_cpi, "cpi").eac,
            Amount {
                minor: None,
                absent: Some(Absent::UndefinedDenominator)
            }
        );
        assert_eq!(
            by(&none_cpi, "cpi_spi").eac.absent,
            Some(Absent::UndefinedDenominator)
        );
        assert_eq!(
            amount(by(&none_cpi, "budget_rate")),
            Some(11_000),
            "the one that needs no ratio still answers"
        );
        let zero = eac_variants(10_000, 0, Some(5_000), Some(0), Some(0), None);
        assert_eq!(
            by(&zero, "cpi").eac.absent,
            Some(Absent::UndefinedDenominator),
            "CPI = 0 is not infinity"
        );
        let no_ac = eac_variants(10_000, 4_000, None, None, None, None);
        assert!(
            no_ac
                .iter()
                .filter(|e| e.variant != "bottom_up")
                .all(|e| e.eac.absent == Some(Absent::NoActualCost))
        );
        assert_eq!(
            by(&textbook(), "bottom_up").eac.absent,
            Some(Absent::NoBottomUpEstimate)
        );
    }

    #[test]
    fn a_bottom_up_estimate_is_carried_as_given() {
        let v = eac_variants(
            10_000,
            4_000,
            Some(5_000),
            Some(8_000),
            Some(8_000),
            Some(13_000),
        );
        let b = by(&v, "bottom_up");
        assert_eq!(
            (b.eac.minor, b.etc.minor, b.vac.minor),
            (Some(13_000), Some(8_000), Some(-3_000))
        );
    }

    #[test]
    fn finished_work_forecasts_its_actual_cost() {
        let v = eac_variants(
            10_000,
            10_000,
            Some(9_000),
            Some(11_111),
            Some(10_000),
            None,
        );
        assert_eq!(
            amount(by(&v, "budget_rate")),
            Some(9_000),
            "nothing remains"
        );
        assert_eq!(amount(by(&v, "cpi")), Some(9_000));
        assert_eq!(by(&v, "cpi").vac.minor, Some(1_000), "under budget");
    }

    #[test]
    fn tcpi_to_the_budget_and_to_the_forecast() {
        // (BAC − EV) / (BAC − AC) = 6 000 / 5 000 = 1.20; against an EAC of 12 500: 6 000 / 7 500 = 0.80.
        let against_budget = tcpi(10_000, 4_000, Some(5_000), 10_000);
        assert_eq!(against_budget.bp, Some(12_000));
        assert!(
            against_budget.needs_improved_efficiency,
            "above 1.0: the rest must be done better than the work so far"
        );
        let against_eac = tcpi(10_000, 4_000, Some(5_000), 12_500);
        assert_eq!(against_eac.bp, Some(8_000));
        assert!(!against_eac.needs_improved_efficiency);
    }

    #[test]
    fn a_target_already_spent_is_stated_not_hidden_or_infinite() {
        let over = tcpi(10_000, 4_000, Some(11_000), 10_000);
        assert_eq!(
            (over.bp, over.absent, over.needs_improved_efficiency),
            (None, Some(Absent::BudgetExhausted), true)
        );
        let exact = tcpi(10_000, 4_000, Some(10_000), 10_000);
        assert_eq!(
            (exact.bp, exact.absent),
            (None, Some(Absent::NoRemainingCost))
        );
        let done = tcpi(10_000, 10_000, Some(10_000), 10_000);
        assert_eq!(
            done.bp,
            Some(10_000),
            "no work and no money left is exactly on target"
        );
        assert_eq!(
            tcpi(10_000, 4_000, None, 10_000).absent,
            Some(Absent::NoActualCost)
        );
    }

    #[test]
    fn the_recommended_variant_is_a_stated_rule_not_a_hidden_choice() {
        let t = Thresholds::default();
        assert_eq!(
            recommended(Some(8_000), Some(10_000), &t).0,
            "cpi",
            "schedule fine: cost efficiency continues"
        );
        assert_eq!(
            recommended(Some(8_000), Some(8_000), &t).0,
            "cpi_spi",
            "schedule behind the amber line"
        );
        assert_eq!(recommended(None, Some(8_000), &t).0, "budget_rate");
        assert_eq!(recommended(Some(0), Some(8_000), &t).0, "budget_rate");
        assert!(
            recommended(Some(8_000), Some(8_000), &t)
                .1
                .contains("schedule is behind")
        );
    }

    #[test]
    fn thresholds_parse_leniently_and_fall_back_to_the_defaults_when_wrong() {
        let (d, used) = parse_thresholds(None);
        assert!(used);
        assert_eq!(d, Thresholds::default());
        let (t, used) = parse_thresholds(Some("spi_cpi_amber=9800, spi_cpi_red=9200"));
        assert!(!used);
        assert_eq!(
            (t.spi_cpi_amber_bp, t.spi_cpi_red_bp, t.tcpi_amber_bp),
            (9_800, 9_200, 10_500)
        );
        for bad in [
            "nonsense",
            "spi_cpi_amber=abc",
            "spi_cpi_amber=9000,spi_cpi_red=9500",
            "tcpi_amber=13000,tcpi_red=11000",
            "bogus=1",
            "spi_cpi_red=0",
        ] {
            assert_eq!(
                parse_thresholds(Some(bad)),
                (Thresholds::default(), true),
                "{bad}"
            );
        }
    }

    #[test]
    fn traffic_lights_follow_the_thresholds_and_say_unknown_when_there_is_no_value() {
        let t = Thresholds::default();
        assert_eq!(
            [
                rag_ratio(Some(10_000), &t),
                rag_ratio(Some(9_500), &t),
                rag_ratio(Some(9_499), &t),
                rag_ratio(Some(9_000), &t),
                rag_ratio(Some(8_999), &t),
                rag_ratio(None, &t)
            ],
            [
                Rag::Green,
                Rag::Green,
                Rag::Amber,
                Rag::Amber,
                Rag::Red,
                Rag::Unknown
            ]
        );
        assert_eq!(
            [
                rag_tcpi(Some(10_000), &t),
                rag_tcpi(Some(10_500), &t),
                rag_tcpi(Some(10_501), &t),
                rag_tcpi(Some(12_500), &t),
                rag_tcpi(Some(12_501), &t),
                rag_tcpi(None, &t)
            ],
            [
                Rag::Green,
                Rag::Green,
                Rag::Amber,
                Rag::Amber,
                Rag::Red,
                Rag::Unknown
            ]
        );
    }

    #[test]
    fn totals_never_cross_currencies_and_say_when_cost_is_partial() {
        let mut t = BTreeMap::new();
        add(&mut t, "GBP", 1_000, 400, 300, Some(350));
        add(&mut t, "GBP", 2_000, 800, 900, None);
        add(&mut t, "USD", 5_000, 1_000, 500, Some(600));
        let gbp = &t["GBP"];
        assert_eq!(
            (gbp.bac_minor, gbp.pv_minor, gbp.ev_minor, gbp.ac_minor),
            (3_000, 1_200, 1_200, 350)
        );
        assert_eq!(
            (gbp.plans, gbp.plans_without_ac),
            (2, 1),
            "AC is partial and says so"
        );
        assert_eq!(t["USD"].bac_minor, 5_000, "USD is its own total");
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn a_ratio_from_totals_is_integer_and_none_on_zero() {
        assert_eq!(ratio_bp(1_200, 1_200), Some(10_000));
        assert_eq!(ratio_bp(2, 3), Some(6_667));
        assert_eq!(ratio_bp(5, 0), None);
    }
}
