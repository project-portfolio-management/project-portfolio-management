//! Pure rules for **economic viability**: net present value, payback and a verdict over a plan's dated cash flows. DB-free.
//!
//! A flow is a signed amount in minor units in one period (period 0 is now): negative is money out (an investment, such as the
//! cost of training people), positive is money in or avoided. The rate is per period, in basis points, and is **echoed in the
//! result**, because an NPV whose rate cannot be seen cannot be reviewed.
//!
//! The recurring rule of this crate holds: **no benefit recorded is `Unmeasured`, never `NotViable`.** A plan that has not yet
//! estimated its benefits has not failed. Flows are one currency; the caller withholds a mixed set before it gets here.

use serde::{Deserialize, Serialize};

/// Basis points in 100 percent.
const BP: i128 = 10_000;

/// The most periods a series may span, so a typo cannot ask for ten thousand years.
pub const MAX_PERIODS: u32 = 600;

/// One cash flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flow {
    /// The period it falls in; 0 is now.
    pub period: u32,
    /// Minor units; negative is out, positive is in.
    pub amount_minor: i64,
}

/// A point in time between whole periods: `periods` whole periods and `fraction_bp` of the next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payback {
    /// Whole periods after the last outflow's period.
    pub periods: u32,
    /// Basis points of the next period, 0 to 9999.
    pub fraction_bp: u32,
}

/// What the figures say.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// NPV above zero, and paid back inside the limit where there is one.
    Viable,
    /// NPV above zero but paid back later than the limit.
    TooSlow,
    /// NPV exactly zero.
    BreakEven,
    /// NPV below zero.
    NotViable,
    /// No investment or no benefit is recorded: nothing to judge.
    Unmeasured,
}

/// Why a figure is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Absent {
    /// No flow is an outflow, so there is no investment to recover.
    NoInvestment,
    /// No flow is an inflow, so nothing is recorded to recover it with.
    NoBenefit,
    /// The series is longer than [`MAX_PERIODS`] or the arithmetic overflowed.
    OutOfRange,
}

/// The result, with its inputs echoed so the arithmetic is checkable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viability {
    /// The discount rate used, basis points per period.
    pub rate_bp: u32,
    /// Net present value, or `None` with `absent`.
    pub npv_minor: Option<i64>,
    /// Total outflows (positive number).
    pub investment_minor: i64,
    /// Total inflows.
    pub benefit_minor: i64,
    /// When the undiscounted running total first reaches zero.
    pub payback: Option<Payback>,
    /// When the discounted running total first reaches zero.
    pub discounted_payback: Option<Payback>,
    /// The verdict.
    pub verdict: Verdict,
    /// Why the figures are absent, if they are.
    pub absent: Option<Absent>,
}

/// Present value of `amount` after `period` periods at `rate_bp`, half away from zero at each step.
fn present_value(amount: i64, period: u32, rate_bp: u32) -> Option<i128> {
    let mut value = i128::from(amount);
    let divisor = BP + i128::from(rate_bp);
    for _ in 0..period {
        let scaled = value.checked_mul(BP)?;
        let half = divisor / 2;
        value = if scaled >= 0 {
            (scaled + half) / divisor
        } else {
            (scaled - half) / divisor
        };
    }
    Some(value)
}

/// Where a running total, which was negative before `period`, first reaches zero: linear inside the period.
fn crossing(before: i128, flow: i128, period: u32, last_out: u32) -> Payback {
    // `before` < 0 and `before + flow` >= 0, so `flow` > 0.
    let need = -before;
    let fraction = need * BP / flow;
    let whole = period.saturating_sub(last_out).saturating_sub(1);
    let fraction_bp = u32::try_from(fraction).unwrap_or(0).min(9_999);
    Payback {
        periods: whole,
        fraction_bp,
    }
}

/// The first point the running total, adding the flows in period order (each discounted by `rate_bp`), reaches zero after money has
/// gone out. `None` when it never does.
fn payback_of(flows: &[Flow], rate_bp: u32, last_out: u32) -> Option<Payback> {
    let mut sorted: Vec<Flow> = flows.to_vec();
    sorted.sort_by_key(|f| f.period);
    let mut total: i128 = 0;
    for f in sorted {
        let value = present_value(f.amount_minor, f.period, rate_bp)?;
        let before = total;
        total += value;
        if before < 0 && total >= 0 && value > 0 && f.period > last_out {
            return Some(crossing(before, value, f.period, last_out));
        }
    }
    None
}

/// Assess a series of flows at `rate_bp` per period; `max_payback_periods` is the longest payback the owner accepts, if any.
#[must_use]
pub fn assess(flows: &[Flow], rate_bp: u32, max_payback_periods: Option<u32>) -> Viability {
    let investment: i128 = flows
        .iter()
        .filter(|f| f.amount_minor < 0)
        .map(|f| -i128::from(f.amount_minor))
        .sum();
    let benefit: i128 = flows
        .iter()
        .filter(|f| f.amount_minor > 0)
        .map(|f| i128::from(f.amount_minor))
        .sum();
    let to_i64 = |n: i128| i64::try_from(n).ok();
    let absent_result = |absent: Absent, verdict: Verdict| Viability {
        rate_bp,
        npv_minor: None,
        investment_minor: to_i64(investment).unwrap_or(i64::MAX),
        benefit_minor: to_i64(benefit).unwrap_or(i64::MAX),
        payback: None,
        discounted_payback: None,
        verdict,
        absent: Some(absent),
    };
    if flows.iter().any(|f| f.period > MAX_PERIODS) {
        return absent_result(Absent::OutOfRange, Verdict::Unmeasured);
    }
    if investment == 0 {
        return absent_result(Absent::NoInvestment, Verdict::Unmeasured);
    }
    if benefit == 0 {
        return absent_result(Absent::NoBenefit, Verdict::Unmeasured);
    }
    let mut npv: i128 = 0;
    for f in flows {
        match present_value(f.amount_minor, f.period, rate_bp) {
            Some(v) => npv += v,
            None => return absent_result(Absent::OutOfRange, Verdict::Unmeasured),
        }
    }
    let Some(npv_minor) = to_i64(npv) else {
        return absent_result(Absent::OutOfRange, Verdict::Unmeasured);
    };
    let last_out = flows
        .iter()
        .filter(|f| f.amount_minor < 0)
        .map(|f| f.period)
        .max()
        .unwrap_or(0);
    let payback = payback_of(flows, 0, last_out);
    let discounted_payback = payback_of(flows, rate_bp, last_out);
    let within = |p: Option<Payback>| match (p, max_payback_periods) {
        (Some(p), Some(max)) => p.periods < max || (p.periods == max && p.fraction_bp == 0),
        (None, Some(_)) => false,
        (_, None) => true,
    };
    let verdict = match npv_minor {
        n if n < 0 => Verdict::NotViable,
        0 => Verdict::BreakEven,
        _ if within(payback) => Verdict::Viable,
        _ => Verdict::TooSlow,
    };
    Viability {
        rate_bp,
        npv_minor: Some(npv_minor),
        investment_minor: to_i64(investment).unwrap_or(i64::MAX),
        benefit_minor: to_i64(benefit).unwrap_or(i64::MAX),
        payback,
        discounted_payback,
        verdict,
        absent: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flow(period: u32, amount_minor: i64) -> Flow {
        Flow {
            period,
            amount_minor,
        }
    }

    #[test]
    fn npv_discounts_each_period_and_the_rate_is_echoed() {
        // 1000 out now, 600 in after one period and 600 after two, at 10 percent a period.
        let v = assess(&[flow(0, -1000), flow(1, 600), flow(2, 600)], 1_000, None);
        assert_eq!(v.rate_bp, 1_000);
        // Each period divides by 1.1 and rounds: 600 becomes 545, then that becomes 495. 545 + 495 - 1000 = 40.
        assert_eq!(v.npv_minor, Some(40));
        assert_eq!(v.investment_minor, 1000);
        assert_eq!(v.benefit_minor, 1200);
        assert_eq!(v.verdict, Verdict::Viable);
    }

    #[test]
    fn payback_is_linear_inside_the_period_and_discounting_makes_it_later() {
        let v = assess(&[flow(0, -1000), flow(1, 600), flow(2, 600)], 1_000, None);
        // The undiscounted total reaches zero two thirds of the way into the second inflow period.
        assert_eq!(
            v.payback,
            Some(Payback {
                periods: 1,
                fraction_bp: 6_666
            })
        );
        let d = v.discounted_payback.unwrap();
        assert!(d.periods > 1 || d.fraction_bp > 6_666, "{d:?}");
    }

    #[test]
    fn no_benefit_recorded_is_unmeasured_never_not_viable() {
        let v = assess(&[flow(0, -5000)], 800, None);
        assert_eq!(v.verdict, Verdict::Unmeasured);
        assert_eq!(v.absent, Some(Absent::NoBenefit));
        assert_eq!(v.npv_minor, None);
    }

    #[test]
    fn no_investment_is_unmeasured_too() {
        let v = assess(&[flow(1, 500)], 800, None);
        assert_eq!(v.verdict, Verdict::Unmeasured);
        assert_eq!(v.absent, Some(Absent::NoInvestment));
    }

    #[test]
    fn a_benefit_smaller_than_the_investment_is_not_viable() {
        let v = assess(&[flow(0, -1000), flow(1, 400)], 0, None);
        assert_eq!(v.npv_minor, Some(-600));
        assert_eq!(v.verdict, Verdict::NotViable);
        assert_eq!(v.payback, None);
    }

    #[test]
    fn a_zero_net_is_break_even() {
        let v = assess(&[flow(0, -1000), flow(2, 1000)], 0, None);
        assert_eq!(v.verdict, Verdict::BreakEven);
    }

    #[test]
    fn a_positive_npv_that_pays_back_too_late_is_too_slow() {
        let flows = [flow(0, -1000), flow(5, 1500)];
        assert_eq!(assess(&flows, 0, None).verdict, Verdict::Viable);
        assert_eq!(assess(&flows, 0, Some(6)).verdict, Verdict::Viable);
        assert_eq!(assess(&flows, 0, Some(3)).verdict, Verdict::TooSlow);
    }

    #[test]
    fn a_series_past_the_limit_is_out_of_range_not_a_guess() {
        let v = assess(&[flow(0, -1), flow(MAX_PERIODS + 1, 5)], 100, None);
        assert_eq!(v.absent, Some(Absent::OutOfRange));
        assert_eq!(v.verdict, Verdict::Unmeasured);
    }

    #[test]
    fn flows_in_any_order_give_the_same_answer() {
        let a = assess(&[flow(0, -1000), flow(1, 600), flow(2, 600)], 500, None);
        let b = assess(&[flow(2, 600), flow(0, -1000), flow(1, 600)], 500, None);
        assert_eq!(a, b);
    }
}
