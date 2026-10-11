//! Milestones met only with partner sign-off (EX-4). Pure rules; the database work is in
//! [`crate::controllers::milestone_signoffs`].
//!
//! A milestone may require sign-off from named organizations. **Delivered** is what the milestone already
//! means (the delivery organization says it is done: marked done, or its linked tasks are all done) and
//! keeps its date. **Met** is delivered *and* every required organization currently signs it, and has its
//! own date: the later of the delivery and the last sign-off. A milestone with an outstanding sign-off is
//! delivered-not-met. A sign-off can be withdrawn (with a reason, audited), which makes it outstanding
//! again.

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

/// One required organization and whether it currently signs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Required {
    pub organization: Uuid,
    pub name: String,
    /// The date of the sign-off now in force; `None` is outstanding.
    pub signed_on: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    /// No organization is required: delivered is met, as it always was.
    NotRequired,
    /// Not yet delivered.
    NotDelivered,
    /// Delivered, with a sign-off outstanding.
    DeliveredNotMet,
    Met,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Assessment {
    pub state: State,
    pub delivered_on: Option<NaiveDate>,
    pub met_on: Option<NaiveDate>,
    pub outstanding: Vec<String>,
}

/// Assess a milestone from who is required and the date it was delivered (if it was).
#[must_use]
pub fn assess(required: &[Required], delivered_on: Option<NaiveDate>) -> Assessment {
    let outstanding: Vec<String> = required
        .iter()
        .filter(|r| r.signed_on.is_none())
        .map(|r| r.name.clone())
        .collect();
    if required.is_empty() {
        return Assessment {
            state: State::NotRequired,
            delivered_on,
            met_on: delivered_on,
            outstanding,
        };
    }
    let Some(delivered) = delivered_on else {
        return Assessment {
            state: State::NotDelivered,
            delivered_on: None,
            met_on: None,
            outstanding,
        };
    };
    if !outstanding.is_empty() {
        return Assessment {
            state: State::DeliveredNotMet,
            delivered_on,
            met_on: None,
            outstanding,
        };
    }
    let met_on = required
        .iter()
        .filter_map(|r| r.signed_on)
        .fold(delivered, NaiveDate::max);
    Assessment {
        state: State::Met,
        delivered_on,
        met_on: Some(met_on),
        outstanding,
    }
}

/// Totals over milestones that require sign-off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub struct Totals {
    pub requiring_signoff: u64,
    pub delivered: u64,
    pub met: u64,
    pub delivered_not_met: u64,
    /// Met as a share of those delivered, in basis points; `None` when none is delivered.
    pub met_share_of_delivered_bp: Option<u64>,
}

#[must_use]
pub fn totals(states: &[State]) -> Totals {
    let mut t = Totals::default();
    for s in states {
        match s {
            State::NotRequired => continue,
            State::NotDelivered => {}
            State::DeliveredNotMet => {
                t.delivered += 1;
                t.delivered_not_met += 1;
            }
            State::Met => {
                t.delivered += 1;
                t.met += 1;
            }
        }
        t.requiring_signoff += 1;
    }
    t.met_share_of_delivered_bp = (t.delivered > 0).then(|| (t.met * 10_000 + t.delivered / 2) / t.delivered);
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, n).unwrap()
    }

    fn req(name: &str, on: Option<u32>) -> Required {
        Required {
            organization: Uuid::new_v4(),
            name: name.into(),
            signed_on: on.map(d),
        }
    }

    #[test]
    fn no_requirement_means_delivered_is_met() {
        let a = assess(&[], Some(d(5)));
        assert_eq!((a.state, a.met_on), (State::NotRequired, Some(d(5))));
    }

    #[test]
    fn an_outstanding_signoff_is_delivered_not_met_and_has_no_met_date() {
        let a = assess(&[req("Acme", Some(9)), req("Other", None)], Some(d(5)));
        assert_eq!(a.state, State::DeliveredNotMet);
        assert_eq!(a.outstanding, ["Other"]);
        assert_eq!((a.delivered_on, a.met_on), (Some(d(5)), None));
    }

    #[test]
    fn met_is_the_later_of_delivery_and_the_last_signoff() {
        let a = assess(&[req("A", Some(9)), req("B", Some(7))], Some(d(5)));
        assert_eq!((a.state, a.delivered_on, a.met_on), (State::Met, Some(d(5)), Some(d(9))));
        let early = assess(&[req("A", Some(2))], Some(d(5)));
        assert_eq!(early.met_on, Some(d(5)), "signed before delivery: met when delivered");
    }

    #[test]
    fn a_signoff_does_not_make_an_undelivered_milestone_met() {
        let a = assess(&[req("A", Some(2))], None);
        assert_eq!((a.state, a.met_on), (State::NotDelivered, None));
    }

    #[test]
    fn the_totals_give_the_met_share_and_the_gap() {
        let t = totals(&[State::Met, State::DeliveredNotMet, State::DeliveredNotMet, State::NotDelivered, State::NotRequired]);
        assert_eq!((t.requiring_signoff, t.delivered, t.met, t.delivered_not_met), (4, 3, 1, 2));
        assert_eq!(t.met_share_of_delivered_bp, Some(3333));
        assert_eq!(totals(&[State::NotDelivered]).met_share_of_delivered_bp, None, "nothing delivered: no share");
    }
}
