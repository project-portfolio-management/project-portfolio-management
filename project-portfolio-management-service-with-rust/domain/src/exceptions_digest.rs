//! The weekly portfolio-exceptions digest (NC-17): what is outside tolerance, over a threshold or escalated, in one
//! short, ordered message. DB-free: the controller gathers the [`Item`]s and this module orders, counts and words
//! them, so the same facts always give the same digest.

use std::fmt::Write;

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

/// The most items a digest lists; the counts always cover all of them.
pub const MAX_ITEMS: usize = 20;

/// Where an exception comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// A red or amber tolerance dimension.
    Tolerance,
    /// An open exception opened by a pause / reset / stop threshold.
    Threshold,
    /// A live escalation.
    Escalation,
}

/// One exception.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Item {
    /// Where it comes from.
    pub kind: Kind,
    /// The plan it is about.
    pub plan_pid: Uuid,
    /// The plan's name.
    pub plan_name: String,
    /// What is wrong, in a line.
    pub summary: String,
    /// 0 is the worst (red, overdue); larger is milder.
    pub severity: u8,
}

/// How many of each kind there are.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    /// Tolerance items.
    pub tolerance: usize,
    /// Threshold items.
    pub threshold: usize,
    /// Escalation items.
    pub escalation: usize,
}

/// The digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Digest {
    /// The deployment's date it was made for.
    pub as_of: NaiveDate,
    /// Counts over every item, listed or not.
    pub counts: Counts,
    /// The worst [`MAX_ITEMS`] items, worst first.
    pub items: Vec<Item>,
    /// More items exist than are listed.
    pub truncated: bool,
}

/// Order, count and cap `items`: worst severity first, then kind, plan name and summary, so the order never
/// depends on the order the facts were gathered in.
#[must_use]
pub fn build(mut items: Vec<Item>, as_of: NaiveDate) -> Digest {
    items.sort_by(|a, b| {
        (a.severity, a.kind, &a.plan_name, &a.summary, a.plan_pid).cmp(&(
            b.severity,
            b.kind,
            &b.plan_name,
            &b.summary,
            b.plan_pid,
        ))
    });
    let mut counts = Counts::default();
    for i in &items {
        match i.kind {
            Kind::Tolerance => counts.tolerance += 1,
            Kind::Threshold => counts.threshold += 1,
            Kind::Escalation => counts.escalation += 1,
        }
    }
    let truncated = items.len() > MAX_ITEMS;
    items.truncate(MAX_ITEMS);
    Digest {
        as_of,
        counts,
        items,
        truncated,
    }
}

/// The digest as the text of a notification.
#[must_use]
pub fn message(d: &Digest) -> String {
    let c = d.counts;
    if c.tolerance + c.threshold + c.escalation == 0 {
        return format!("Portfolio exceptions for the week to {}: none.", d.as_of);
    }
    let mut out = format!(
        "Portfolio exceptions for the week to {}: {} tolerance, {} threshold, {} escalation.",
        d.as_of, c.tolerance, c.threshold, c.escalation
    );
    for i in &d.items {
        let _ = write!(out, "\n- {}: {}", i.plan_name, i.summary);
    }
    if d.truncated {
        out.push_str("\n- and more; open the portfolio exceptions view for the rest.");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(kind: Kind, name: &str, severity: u8) -> Item {
        Item {
            kind,
            plan_pid: Uuid::from_u128(1),
            plan_name: name.to_string(),
            summary: format!("{kind:?} on {name}"),
            severity,
        }
    }

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, 26).unwrap()
    }

    #[test]
    fn one_of_each_kind_is_counted_and_listed_worst_first() {
        let d = build(
            vec![
                item(Kind::Escalation, "B", 2),
                item(Kind::Tolerance, "A", 0),
                item(Kind::Threshold, "C", 1),
            ],
            day(),
        );
        assert_eq!(
            d.counts,
            Counts {
                tolerance: 1,
                threshold: 1,
                escalation: 1
            }
        );
        let order: Vec<Kind> = d.items.iter().map(|i| i.kind).collect();
        assert_eq!(
            order,
            vec![Kind::Tolerance, Kind::Threshold, Kind::Escalation]
        );
        assert!(!d.truncated);
        let text = message(&d);
        assert!(
            text.contains("1 tolerance, 1 threshold, 1 escalation"),
            "{text}"
        );
        assert!(text.contains("- A: Tolerance on A"), "{text}");
    }

    #[test]
    fn the_order_does_not_depend_on_how_the_facts_were_gathered() {
        let a = vec![
            item(Kind::Escalation, "B", 1),
            item(Kind::Tolerance, "A", 1),
            item(Kind::Threshold, "A", 1),
        ];
        let mut b = a.clone();
        b.reverse();
        assert_eq!(build(a, day()), build(b, day()));
    }

    #[test]
    fn a_long_list_is_capped_but_still_counted() {
        let items: Vec<Item> = (0..MAX_ITEMS + 5)
            .map(|n| item(Kind::Escalation, &format!("P{n:02}"), 2))
            .collect();
        let d = build(items, day());
        assert_eq!(d.items.len(), MAX_ITEMS);
        assert_eq!(d.counts.escalation, MAX_ITEMS + 5);
        assert!(d.truncated);
        assert!(message(&d).contains("and more"));
    }

    #[test]
    fn nothing_to_report_says_so() {
        let d = build(Vec::new(), day());
        assert_eq!(
            message(&d),
            "Portfolio exceptions for the week to 2026-10-26: none."
        );
    }
}
