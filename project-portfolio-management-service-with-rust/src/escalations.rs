//! Project escalations: the pure rules. DB-free and deterministic.
//!
//! An **escalation** is a project referring something upward because it cannot or should not settle it itself: a
//! tolerance exceeded, a risk or issue that needs a decision above the project, a blocking dependency. It is raised
//! at a **level** of the escalation path the project's governance framework declares (level 1 is the first place to
//! go), is acknowledged, may be raised to the next level if nobody answers, and ends resolved (with an outcome and a
//! written response) or withdrawn. Every step is kept in an append-only history.

use chrono::NaiveDate;

/// Why something is escalated.
pub const REASONS: &[&str] = &[
    "tolerance",
    "risk",
    "issue",
    "dependency",
    "decision",
    "resource",
    "other",
];
/// How a resolved escalation ended.
pub const OUTCOMES: &[&str] = &["decided", "guidance_given", "rejected"];
/// The states.
pub const STATUSES: &[&str] = &["open", "acknowledged", "resolved", "withdrawn"];
/// The highest level when there is no declared path to bound it.
pub const LEVEL_MAX: i32 = 9;
/// The shortest written response that counts as one.
pub const RESPONSE_MIN: usize = 10;

/// Whether an escalation in `status` is still awaiting an answer.
#[must_use]
pub fn is_live(status: &str) -> bool {
    matches!(status, "open" | "acknowledged")
}

/// A `person:` or `worker:` reference with something after the colon.
#[must_use]
pub fn is_person_ref(s: &str) -> bool {
    s.split_once(':')
        .is_some_and(|(k, rest)| matches!(k, "person" | "worker") && !rest.trim().is_empty())
}

/// An escalation as submitted.
#[derive(Debug, Clone)]
pub struct Draft<'a> {
    /// Short title.
    pub title: &'a str,
    /// What and why.
    pub description: Option<&'a str>,
    /// One of [`REASONS`].
    pub reason_kind: &'a str,
    /// The tolerance dimension it concerns, only for a `tolerance` reason.
    pub tolerance_dimension: Option<&'a str>,
    /// Who or what it goes to, in words.
    pub escalated_to: Option<&'a str>,
    /// A named person it is for.
    pub owner_ref: Option<&'a str>,
    /// When an answer is wanted.
    pub respond_by: Option<NaiveDate>,
    /// Today.
    pub today: NaiveDate,
}

/// Every problem with a new escalation (empty when fine).
#[must_use]
pub fn problems(d: &Draft<'_>) -> Vec<String> {
    let mut out = Vec::new();
    let n = |s: &str| s.trim().chars().count();
    if n(d.title) == 0 || n(d.title) > 200 {
        out.push("title is 1 to 200 characters".to_string());
    }
    if d.description.is_some_and(|x| x.chars().count() > 10_000) {
        out.push("description is longer than 10000 characters".to_string());
    }
    if !REASONS.contains(&d.reason_kind) {
        out.push(format!("reason_kind must be one of {REASONS:?}"));
    }
    match (d.reason_kind, d.tolerance_dimension) {
        ("tolerance", Some(dim)) if !matches!(dim, "time" | "cost" | "risk" | "resources") => {
            out.push("tolerance_dimension must be time, cost, risk or resources".to_string());
        }
        (kind, Some(_)) if kind != "tolerance" => {
            out.push("tolerance_dimension belongs only on a tolerance escalation".to_string());
        }
        _ => {}
    }
    if d.escalated_to.is_some_and(|x| n(x) == 0 || n(x) > 120) {
        out.push("escalated_to is 1 to 120 characters".to_string());
    }
    if d.owner_ref.is_some_and(|o| !is_person_ref(o)) {
        out.push("owner_ref must be a person: or worker: reference".to_string());
    }
    if d.respond_by.is_some_and(|r| r < d.today) {
        out.push("respond_by is in the past".to_string());
    }
    out
}

/// The path as `(level, body)`, lowest first.
pub type Path = [(u32, String)];

/// Check a level against the declared path. With a path, the level must be on it and the body there is returned as
/// the default `escalated_to`; with none, any level from 1 to [`LEVEL_MAX`] is allowed.
///
/// # Errors
///
/// A sentence for a `422`.
pub fn check_level(level: i32, path: &Path) -> Result<Option<String>, String> {
    if path.is_empty() {
        return if (1..=LEVEL_MAX).contains(&level) {
            Ok(None)
        } else {
            Err(format!("level is 1 to {LEVEL_MAX}"))
        };
    }
    path.iter()
        .find(|(l, _)| i64::from(*l) == i64::from(level))
        .map(|(_, body)| Some(body.clone()))
        .ok_or_else(|| {
            let levels: Vec<u32> = path.iter().map(|(l, _)| *l).collect();
            format!("level {level} is not on this project's escalation path (levels {levels:?})")
        })
}

/// The level above `current`, with the body there when a path declares one.
///
/// # Errors
///
/// A sentence for a `409` when `current` is already the top.
pub fn next_level(current: i32, path: &Path) -> Result<(i32, Option<String>), String> {
    if path.is_empty() {
        return if current < LEVEL_MAX {
            Ok((current + 1, None))
        } else {
            Err(format!("already at the highest level ({LEVEL_MAX})"))
        };
    }
    path.iter()
        .find(|(l, _)| i64::from(*l) > i64::from(current))
        .and_then(|(l, body)| i32::try_from(*l).ok().map(|l| (l, Some(body.clone()))))
        .ok_or_else(|| "already at the top of the escalation path".to_string())
}

/// What can be done to a live escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Someone has seen it.
    Acknowledge,
    /// Send it to the next level up.
    RaiseLevel,
    /// Answer it.
    Resolve,
    /// Take it back.
    Withdraw,
}

/// The status after `action`.
///
/// # Errors
///
/// A sentence for a `409` when the action does not apply.
pub fn apply(status: &str, action: Action) -> Result<&'static str, String> {
    if !is_live(status) {
        return Err(format!("this escalation is already {status}"));
    }
    match (status, action) {
        ("open", Action::Acknowledge) => Ok("acknowledged"),
        ("acknowledged", Action::Acknowledge) => {
            Err("this escalation is already acknowledged".to_string())
        }
        (_, Action::RaiseLevel) => Ok("open"),
        (_, Action::Resolve) => Ok("resolved"),
        (_, Action::Withdraw) => Ok("withdrawn"),
        _ => Err("that does not apply".to_string()),
    }
}

/// Whole days between `from` and `to`, never negative.
#[must_use]
pub fn age_days(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days().max(0)
}

/// Whether a live escalation has passed the date an answer was wanted.
#[must_use]
pub fn is_overdue(status: &str, respond_by: Option<NaiveDate>, today: NaiveDate) -> bool {
    is_live(status) && respond_by.is_some_and(|r| r < today)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("date")
    }

    fn ok() -> Draft<'static> {
        Draft {
            title: "Cost forecast is beyond tolerance",
            description: None,
            reason_kind: "tolerance",
            tolerance_dimension: Some("cost"),
            escalated_to: None,
            owner_ref: Some("person:abc"),
            respond_by: Some(d(2026, 11, 1)),
            today: d(2026, 10, 20),
        }
    }

    #[test]
    fn a_good_escalation_has_no_problems_and_every_bad_field_is_named() {
        assert!(problems(&ok()).is_empty());
        let bad = Draft {
            title: " ",
            description: None,
            reason_kind: "mood",
            tolerance_dimension: Some("cost"),
            escalated_to: Some(""),
            owner_ref: Some("nobody"),
            respond_by: Some(d(2026, 10, 1)),
            today: d(2026, 10, 20),
        };
        let out = problems(&bad);
        for needle in [
            "title",
            "reason_kind",
            "tolerance_dimension",
            "escalated_to",
            "owner_ref",
            "respond_by",
        ] {
            assert!(out.iter().any(|p| p.contains(needle)), "{needle}: {out:?}");
        }
        let wrong_dim = Draft {
            tolerance_dimension: Some("scope"),
            ..ok()
        };
        assert!(
            problems(&wrong_dim)
                .iter()
                .any(|p| p.contains("time, cost, risk or resources"))
        );
        let dim_on_risk = Draft {
            reason_kind: "risk",
            ..ok()
        };
        assert!(
            problems(&dim_on_risk)
                .iter()
                .any(|p| p.contains("only on a tolerance"))
        );
    }

    #[test]
    fn a_level_must_be_on_the_declared_path_and_the_next_level_follows_it() {
        let path = vec![
            (1, "Programme manager".to_string()),
            (2, "Sponsor".to_string()),
            (4, "Board".to_string()),
        ];
        assert_eq!(check_level(2, &path), Ok(Some("Sponsor".to_string())));
        assert!(
            check_level(3, &path)
                .expect_err("not on the path")
                .contains("levels [1, 2, 4]")
        );
        assert_eq!(next_level(1, &path), Ok((2, Some("Sponsor".to_string()))));
        assert_eq!(
            next_level(2, &path),
            Ok((4, Some("Board".to_string()))),
            "skips a level the path does not have"
        );
        assert!(next_level(4, &path).is_err(), "the top");
        // No declared path: any level up to the cap.
        assert_eq!(check_level(5, &[]), Ok(None));
        assert!(check_level(0, &[]).is_err() && check_level(10, &[]).is_err());
        assert_eq!(next_level(8, &[]), Ok((9, None)));
        assert!(next_level(9, &[]).is_err());
    }

    #[test]
    fn the_states_move_one_way_and_a_finished_escalation_stays_finished() {
        assert_eq!(apply("open", Action::Acknowledge), Ok("acknowledged"));
        assert!(apply("acknowledged", Action::Acknowledge).is_err());
        assert_eq!(
            apply("acknowledged", Action::RaiseLevel),
            Ok("open"),
            "the next level has not seen it yet"
        );
        assert_eq!(apply("open", Action::Resolve), Ok("resolved"));
        assert_eq!(apply("acknowledged", Action::Withdraw), Ok("withdrawn"));
        for done in ["resolved", "withdrawn"] {
            for a in [
                Action::Acknowledge,
                Action::RaiseLevel,
                Action::Resolve,
                Action::Withdraw,
            ] {
                assert!(apply(done, a).is_err(), "{done} {a:?}");
            }
        }
    }

    #[test]
    fn only_a_live_escalation_can_be_overdue() {
        let today = d(2026, 10, 20);
        assert!(is_overdue("open", Some(d(2026, 10, 19)), today));
        assert!(
            !is_overdue("open", Some(d(2026, 10, 20)), today),
            "due today is not late yet"
        );
        assert!(!is_overdue("open", None, today));
        assert!(!is_overdue("resolved", Some(d(2026, 1, 1)), today));
        assert_eq!(age_days(d(2026, 10, 10), today), 10);
        assert_eq!(age_days(d(2026, 11, 1), today), 0);
        assert!(is_person_ref("worker:1") && !is_person_ref("worker:") && !is_person_ref("team:x"));
    }
}
