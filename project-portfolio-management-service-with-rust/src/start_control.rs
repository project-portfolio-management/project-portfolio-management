//! Start control (EX-8): a work-in-progress limit on programmes in active delivery, and a capacity check on
//! the constraint pool (the drum, CC-5). Pure rules; the database work is in
//! [`crate::controllers::start_control`].
//!
//! A programme is **in active delivery** when its phase is executing or controlling **and** its status is
//! active: a paused or stopped programme does not use a place. A programme about to start (move into
//! executing) is refused when the places are full, or when the constraint pool is already short of supply
//! in the months ahead; or it starts only with a recorded override that gives a reason. Programmes refused
//! wait in a queue in the order they asked, and the order never changes.

#![allow(missing_docs)]

/// Whether a programme with this phase and status is in active delivery.
#[must_use]
pub fn in_delivery(phase: Option<&str>, status: &str) -> bool {
    status == "active" && matches!(phase, Some("executing" | "controlling"))
}

/// The limit in force: `PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT`, a whole number from 1 to 1000. Unset or
/// unusable means no limit; the flag says an unusable value was set.
#[must_use]
pub fn limit() -> (Option<usize>, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT") {
        Err(_) => (None, false),
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if (1..=1000).contains(&n) => (Some(n), false),
            _ => (None, true),
        },
    }
}

/// Why a start is refused.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum Refusal {
    /// All `limit` places are taken by programmes in active delivery.
    LimitReached { in_delivery: usize, limit: usize },
    /// The constraint pool is short of supply in these months (`YYYY-MM`).
    ConstraintPoolOver { pool: String, months: Vec<String> },
}

/// The refusals for a start, given the programmes now in delivery and the constraint pool's short months.
/// A programme already in delivery is never refused for being there: pass the count without it.
#[must_use]
pub fn refusals(
    in_delivery_now: usize,
    limit: Option<usize>,
    pool: Option<(&str, &[String])>,
) -> Vec<Refusal> {
    let mut out = Vec::new();
    if let Some(l) = limit
        && in_delivery_now >= l
    {
        out.push(Refusal::LimitReached {
            in_delivery: in_delivery_now,
            limit: l,
        });
    }
    if let Some((name, short)) = pool
        && !short.is_empty()
    {
        out.push(Refusal::ConstraintPoolOver {
            pool: name.to_string(),
            months: short.to_vec(),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_active_programme_in_executing_or_controlling_uses_a_place() {
        assert!(in_delivery(Some("executing"), "active"));
        assert!(in_delivery(Some("controlling"), "active"));
        assert!(
            !in_delivery(Some("executing"), "paused"),
            "paused does not count"
        );
        assert!(!in_delivery(Some("executing"), "stopped"));
        assert!(!in_delivery(Some("planning"), "active"));
        assert!(!in_delivery(None, "active"));
        assert!(!in_delivery(Some("closing"), "active"));
    }

    #[test]
    fn the_limit_is_reached_at_the_limit_and_not_below_it() {
        assert!(
            refusals(2, Some(3), None).is_empty(),
            "two of three places: room"
        );
        assert_eq!(
            refusals(3, Some(3), None),
            [Refusal::LimitReached {
                in_delivery: 3,
                limit: 3
            }]
        );
        assert!(refusals(99, None, None).is_empty(), "no limit set");
    }

    #[test]
    fn a_short_constraint_pool_refuses_and_names_the_months() {
        let months = vec!["2026-11".to_string(), "2026-12".to_string()];
        let r = refusals(0, Some(5), Some(("Engineering", &months)));
        assert_eq!(
            r,
            [Refusal::ConstraintPoolOver {
                pool: "Engineering".into(),
                months
            }]
        );
        assert!(
            refusals(0, None, Some(("Engineering", &[]))).is_empty(),
            "no short month, no refusal"
        );
    }

    #[test]
    fn both_can_refuse_at_once() {
        let m = vec!["2026-11".to_string()];
        assert_eq!(refusals(3, Some(3), Some(("X", &m))).len(), 2);
    }
}
