//! The capacity heatmap by skill pool and month (DP-10). Pure rules; the database work is in
//! [`crate::controllers::capacity_heatmap`].
//!
//! Capacity is counted in thousandths of a full-time equivalent (milli-FTE), whole numbers. A cell
//! says what is asked (demand), what is on offer (supply: the organization's own people plus partners'
//! commitments) and the gap, and it says so with a **word and a shape** as well as a colour. A pool
//! with no supplier and no partner commitment is **no supply recorded**, not zero capacity.

#![allow(missing_docs)]

/// Milli-FTE from percent-days over `days` calendar days: `percent_days / (100 × days) × 1000`,
/// rounded to the nearest whole milli-FTE.
#[must_use]
pub fn milli_fte(percent_days: i64, days: i64) -> i64 {
    if days <= 0 {
        return 0;
    }
    let n = i128::from(percent_days) * 10;
    let d = i128::from(days);
    i64::try_from((n * 2 + d) / (d * 2)).unwrap_or(i64::MAX)
}

/// One cell's inputs, in milli-FTE.
#[derive(Debug, Clone, Default)]
pub struct Inputs {
    pub own_demand: i64,
    /// Work partners owe (commitments of kind `demand`).
    pub partner_demand: i64,
    /// What the organization's own people can give; none when no person has the pool's roles.
    pub own_supply: Option<i64>,
    /// What partners have committed; none when nobody has.
    pub partner_committed: Option<i64>,
    /// What partners have delivered, where recorded.
    pub partner_delivered: Option<i64>,
}

/// The reading of a cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// `shortfall`, `tight`, `enough`, `no_supply_recorded`, `no_demand` or `nothing_recorded`.
    pub state: &'static str,
    pub word: &'static str,
    /// A shape that carries the meaning without colour.
    pub shape: &'static str,
    pub colour: &'static str,
    pub demand: i64,
    /// None when no supply is recorded: that is not zero.
    pub supply: Option<i64>,
    pub gap: Option<i64>,
    /// Partners delivered less than they committed.
    pub partner_delivery_short: bool,
}

/// Read a cell. Supply under demand is a shortfall; supply under 110 % of demand is tight.
#[must_use]
pub fn verdict(i: &Inputs) -> Verdict {
    let demand = i.own_demand.saturating_add(i.partner_demand);
    let supply = if i.own_supply.is_none() && i.partner_committed.is_none() {
        None
    } else {
        Some(
            i.own_supply
                .unwrap_or(0)
                .saturating_add(i.partner_committed.unwrap_or(0)),
        )
    };
    let short = matches!((i.partner_committed, i.partner_delivered), (Some(c), Some(d)) if d < c);
    let (state, word, shape, colour) = match supply {
        None if demand == 0 => ("nothing_recorded", "nothing recorded", "blank", "none"),
        None => ("no_supply_recorded", "no supply recorded", "dash", "grey"),
        Some(_) if demand == 0 => ("no_demand", "no demand", "empty circle", "grey"),
        Some(s) if s < demand => ("shortfall", "shortfall", "triangle", "red"),
        Some(s) if i128::from(s) * 100 < i128::from(demand) * 110 => {
            ("tight", "tight", "diamond", "amber")
        }
        Some(_) => ("enough", "enough", "circle", "green"),
    };
    Verdict {
        state,
        word,
        shape,
        colour,
        demand,
        supply,
        gap: supply.map(|s| s - demand),
        partner_delivery_short: short,
    }
}

/// The first day of the month `n` months after `from` (a first of a month).
#[must_use]
pub fn add_months(from: chrono::NaiveDate, n: u32) -> chrono::NaiveDate {
    from.checked_add_months(chrono::Months::new(n))
        .unwrap_or(from)
}

/// Parse `YYYY-MM` to the first day of that month.
#[must_use]
pub fn parse_month(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(&format!("{s}-01"), "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(od: i64, pd: i64, os: Option<i64>, pc: Option<i64>, pdl: Option<i64>) -> Inputs {
        Inputs {
            own_demand: od,
            partner_demand: pd,
            own_supply: os,
            partner_committed: pc,
            partner_delivered: pdl,
        }
    }

    #[test]
    fn milli_fte_rounds_to_the_nearest_thousandth() {
        assert_eq!(
            milli_fte(3100, 31),
            1000,
            "100 % for a 31-day month is one FTE"
        );
        assert_eq!(milli_fte(1550, 31), 500);
        assert_eq!(
            milli_fte(1, 3),
            3,
            "1 percent-day over 3 days is 0.0033 FTE"
        );
        assert_eq!(milli_fte(0, 30), 0);
        assert_eq!(milli_fte(10, 0), 0);
    }

    #[test]
    fn a_pool_with_no_supplier_is_no_supply_recorded_not_zero_capacity() {
        let v = verdict(&i(2000, 0, None, None, None));
        assert_eq!(
            (v.state, v.word, v.shape, v.colour),
            ("no_supply_recorded", "no supply recorded", "dash", "grey")
        );
        assert_eq!((v.supply, v.gap), (None, None), "no zero is invented");
        // A recorded supply of zero is a real answer: a shortfall.
        let z = verdict(&i(2000, 0, Some(0), None, None));
        assert_eq!((z.state, z.gap), ("shortfall", Some(-2000)));
    }

    #[test]
    fn the_lines_between_shortfall_tight_and_enough_are_pinned() {
        // Demand 1000: 999 is short, 1000 is tight (not short), 1099 is tight, 1100 is enough.
        assert_eq!(
            verdict(&i(1000, 0, Some(999), None, None)).state,
            "shortfall"
        );
        assert_eq!(verdict(&i(1000, 0, Some(1000), None, None)).state, "tight");
        assert_eq!(verdict(&i(1000, 0, Some(1099), None, None)).state, "tight");
        assert_eq!(verdict(&i(1000, 0, Some(1100), None, None)).state, "enough");
        assert_eq!(verdict(&i(1000, 0, Some(5000), None, None)).state, "enough");
    }

    #[test]
    fn every_state_has_a_word_and_a_shape_as_well_as_a_colour() {
        let all = [
            verdict(&i(1000, 0, Some(0), None, None)),
            verdict(&i(1000, 0, Some(1000), None, None)),
            verdict(&i(1000, 0, Some(2000), None, None)),
            verdict(&i(1000, 0, None, None, None)),
            verdict(&i(0, 0, Some(500), None, None)),
            verdict(&i(0, 0, None, None, None)),
        ];
        let shapes: std::collections::BTreeSet<_> = all.iter().map(|v| v.shape).collect();
        let words: std::collections::BTreeSet<_> = all.iter().map(|v| v.word).collect();
        assert_eq!(shapes.len(), all.len(), "no two states share a shape");
        assert_eq!(words.len(), all.len());
    }

    #[test]
    fn partner_supply_adds_to_own_and_partner_demand_to_demand() {
        let v = verdict(&i(1000, 500, Some(800), Some(700), Some(400)));
        assert_eq!((v.demand, v.supply, v.gap), (1500, Some(1500), Some(0)));
        assert_eq!(v.state, "tight");
        assert!(v.partner_delivery_short, "400 delivered of 700 committed");
        assert!(!verdict(&i(1000, 0, Some(1000), Some(700), Some(700))).partner_delivery_short);
        assert!(
            !verdict(&i(1000, 0, Some(1000), Some(700), None)).partner_delivery_short,
            "not recorded is not short"
        );
        // Partner supply alone is supply.
        assert_eq!(verdict(&i(500, 0, None, Some(600), None)).state, "enough");
    }

    #[test]
    fn months_are_whole_months() {
        let m = parse_month("2026-10").unwrap();
        assert_eq!(add_months(m, 1).to_string(), "2026-11-01");
        assert_eq!(
            add_months(parse_month("2026-12").unwrap(), 1).to_string(),
            "2027-01-01"
        );
        assert!(parse_month("2026-13").is_none());
        assert!(parse_month("soon").is_none());
    }
}
