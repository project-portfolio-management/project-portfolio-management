//! Pause, reset and stop thresholds, and triage (EX-7). Pure rules; the database work is in
//! [`crate::controllers::thresholds`].
//!
//! A programme can carry up to three thresholds. Crossing one **opens an exception** that needs a decision:
//! pause, reset, stop, or a recorded **override** that names who decided and when it expires. Nothing is
//! ever paused, reset or stopped automatically: the service reports, a person decides.
//!
//! * `confidence_red_packs` N: the programme's confidence was red in each of the last N issued packs
//!   (one per period); one pack that is not red breaks the run;
//! * `forecast_past` date: the forecast finish is **after** the date (on the date is not past it);
//! * `p80_cap` amount: the P80 cost is **above** the cap (at the cap is not over it).

use chrono::NaiveDate;

/// The threshold kinds.
pub const KINDS: [&str; 3] = ["confidence_red_packs", "forecast_past", "p80_cap"];
/// The decisions that answer a crossing.
pub const ACTIONS: [&str; 4] = ["pause", "reset", "stop", "override"];
/// The triage outcomes.
pub const OUTCOMES: [&str; 4] = ["finish", "fix", "pause", "stop"];

/// Red in each of the last `n` periods. `latest_first` holds one reading per issued period, newest first
/// (`true` = red). Fewer than `n` periods is not a crossing: there is not yet a run.
#[must_use]
pub fn confidence_crossed(latest_first: &[bool], n: usize) -> bool {
    n >= 1 && latest_first.len() >= n && latest_first[..n].iter().all(|r| *r)
}

/// The forecast finish is after `limit`; no forecast is not a crossing.
#[must_use]
pub fn forecast_crossed(forecast: Option<NaiveDate>, limit: NaiveDate) -> bool {
    forecast.is_some_and(|f| f > limit)
}

/// The P80 cost is above `cap`; no figure is not a crossing.
#[must_use]
pub fn p80_crossed(p80_minor: Option<i64>, cap_minor: i64) -> bool {
    p80_minor.is_some_and(|p| p > cap_minor)
}

/// Why a decision cannot be recorded, in words.
#[must_use]
pub fn decision_problems(
    action: &str,
    reason: &str,
    expiry: Option<NaiveDate>,
    today: NaiveDate,
) -> Vec<String> {
    let mut out = Vec::new();
    if !ACTIONS.contains(&action) {
        out.push(format!("action must be one of {ACTIONS:?}"));
    }
    if reason.trim().chars().count() < 10 {
        out.push("a reason of at least 10 characters is required".to_string());
    }
    match (action, expiry) {
        ("override", None) => out.push("an override needs an expiry date".to_string()),
        ("override", Some(e)) if e <= today => {
            out.push("an override's expiry must be in the future".to_string());
        }
        ("override", Some(e)) if (e - today).num_days() > 366 => {
            out.push("an override may last at most a year".to_string());
        }
        (a, Some(_)) if a != "override" => out.push("only an override has an expiry".to_string()),
        _ => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }

    #[test]
    fn confidence_is_crossed_exactly_at_n_red_packs_in_a_row() {
        assert!(
            !confidence_crossed(&[true, true], 3),
            "two packs are not three"
        );
        assert!(confidence_crossed(&[true, true, true], 3), "at the line");
        assert!(
            !confidence_crossed(&[true, false, true], 3),
            "a pack that is not red breaks the run"
        );
        assert!(
            confidence_crossed(&[true, true, true, false], 3),
            "older packs do not matter"
        );
        assert!(!confidence_crossed(&[], 1));
        assert!(
            !confidence_crossed(&[true], 0),
            "a count of zero is no threshold"
        );
    }

    #[test]
    fn the_forecast_is_crossed_one_day_after_the_date_and_not_on_it() {
        assert!(!forecast_crossed(Some(d(3, 31)), d(3, 31)));
        assert!(forecast_crossed(Some(d(4, 1)), d(3, 31)));
        assert!(
            !forecast_crossed(None, d(3, 31)),
            "no forecast, no crossing"
        );
    }

    #[test]
    fn p80_is_crossed_one_unit_above_the_cap_and_not_at_it() {
        assert!(!p80_crossed(Some(1_000_000), 1_000_000));
        assert!(p80_crossed(Some(1_000_001), 1_000_000));
        assert!(!p80_crossed(None, 1_000_000));
    }

    #[test]
    fn an_override_names_an_expiry_in_the_future_and_nothing_else_has_one() {
        let today = d(6, 1);
        assert!(decision_problems("pause", "Safety case is not ready", None, today).is_empty());
        assert!(
            decision_problems("override", "Sponsor accepts the risk", Some(d(9, 1)), today)
                .is_empty()
        );
        assert_eq!(
            decision_problems("override", "Sponsor accepts the risk", None, today),
            ["an override needs an expiry date"]
        );
        assert_eq!(
            decision_problems("override", "Sponsor accepts the risk", Some(today), today),
            ["an override's expiry must be in the future"]
        );
        assert_eq!(
            decision_problems(
                "override",
                "Sponsor accepts the risk",
                Some(d(6, 2) + chrono::Duration::days(400)),
                today
            ),
            ["an override may last at most a year"]
        );
        assert_eq!(
            decision_problems("stop", "Not worth the cost now", Some(d(9, 1)), today),
            ["only an override has an expiry"]
        );
        assert_eq!(decision_problems("maybe", "short", None, today).len(), 2);
    }
}
