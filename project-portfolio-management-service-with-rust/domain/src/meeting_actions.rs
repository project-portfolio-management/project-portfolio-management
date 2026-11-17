//! Action items from minutes to tasks (BR-3): propose, verify, create once. The rules. DB-free, unit-tested.
//!
//! The service runs no model. A person or an external agent proposes tasks from a held meeting's minutes; each proposal
//! quotes an `excerpt` that must be found in the minutes, names the `outcome` (an action of the meeting) it belongs
//! to, may name an assignee (a `person:` or `worker:` URN that must already be one of the plan's people; the service
//! holds no names, so turning a name in the minutes into a URN is the proposer's job) and a due date (inside a window
//! around the meeting). [`judge`] says whether a proposal may be created; nothing here writes.

use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

/// The most proposals one request carries. More is refused, never silently cut.
pub const MAX_PROPOSALS: usize = 20;
/// The shortest excerpt that counts as evidence, in characters (after trimming and collapsing whitespace).
pub const MIN_EXCERPT: usize = 8;
/// The longest task title, in characters.
pub const MAX_TITLE: usize = 200;
/// How many days before the meeting a due date may fall.
pub const WINDOW_BEFORE_DAYS: u64 = 7;
/// How many days after the meeting a due date may fall.
pub const WINDOW_AFTER_DAYS: u64 = 365;

/// One proposed task as given.
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ProposalIn {
    /// The task's title.
    pub title: String,
    /// A passage of the minutes that supports the task; it must appear in them (case and spacing are ignored).
    pub excerpt: String,
    /// Which outcome of the meeting this is: the 0-based position among its outcomes, which must be an `action`.
    pub outcome: usize,
    /// Who it is assigned to: a URN already on the plan's tasks or RACI rows.
    #[serde(default)]
    pub assignee_ref: Option<String>,
    /// When it is due.
    #[serde(default)]
    pub due_on: Option<NaiveDate>,
    /// Story points, if the plan estimates in them.
    #[serde(default)]
    pub points: Option<i32>,
}

/// What may be done with a proposal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// It may be created, with this assignee (if any).
    Ready(Option<String>),
    /// It is refused, for this reason.
    Dropped(&'static str),
}

/// Lower-case and collapse runs of whitespace, so a quote matches across line breaks and capitals.
#[must_use]
pub fn squash(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Whether `excerpt` is long enough to be evidence and appears in `minutes`.
#[must_use]
pub fn excerpt_found(minutes: &str, excerpt: &str) -> bool {
    let wanted = squash(excerpt);
    wanted.chars().count() >= MIN_EXCERPT && squash(minutes).contains(&wanted)
}

/// Whether `due` falls in the window around the meeting date.
#[must_use]
pub fn due_in_window(due: NaiveDate, meeting: NaiveDate) -> bool {
    let first = meeting
        .checked_sub_days(Days::new(WINDOW_BEFORE_DAYS))
        .unwrap_or(NaiveDate::MIN);
    let last = meeting
        .checked_add_days(Days::new(WINDOW_AFTER_DAYS))
        .unwrap_or(NaiveDate::MAX);
    (first..=last).contains(&due)
}

/// What the meeting offers a proposal to attach to.
pub struct Meeting<'a> {
    /// The minutes.
    pub minutes: &'a str,
    /// The date the meeting was held (or scheduled).
    pub on: NaiveDate,
    /// The kind of each outcome, in order.
    pub outcome_kinds: &'a [String],
    /// Which outcomes already have a task.
    pub taken: &'a [usize],
}

/// Judge one proposal. `people` are the plan's people.
#[must_use]
pub fn judge(p: &ProposalIn, m: &Meeting<'_>, people: &[String]) -> Verdict {
    let title = p.title.trim();
    if title.is_empty() || title.chars().count() > MAX_TITLE {
        return Verdict::Dropped("title_invalid");
    }
    if !excerpt_found(m.minutes, &p.excerpt) {
        return Verdict::Dropped("excerpt_not_in_minutes");
    }
    match m.outcome_kinds.get(p.outcome) {
        None => return Verdict::Dropped("outcome_not_found"),
        Some(kind) if kind != "action" => return Verdict::Dropped("outcome_not_an_action"),
        Some(_) => {}
    }
    if m.taken.contains(&p.outcome) {
        return Verdict::Dropped("outcome_already_has_a_task");
    }
    if p.due_on.is_some_and(|d| !due_in_window(d, m.on)) {
        return Verdict::Dropped("due_outside_window");
    }
    if p.points.is_some_and(|n| !(0..=1000).contains(&n)) {
        return Verdict::Dropped("points_invalid");
    }
    match p.assignee_ref.as_deref() {
        None => Verdict::Ready(None),
        Some(who) if people.iter().any(|x| x == who) => Verdict::Ready(Some(who.to_string())),
        Some(_) => Verdict::Dropped("assignee_not_in_plan"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    fn people() -> Vec<String> {
        [
            "person:11111111-1111-1111-1111-111111111111",
            "person:22222222-2222-2222-2222-222222222222",
        ]
        .map(String::from)
        .to_vec()
    }

    fn proposal() -> ProposalIn {
        ProposalIn {
            title: "Order the racks".into(),
            excerpt: "Priya will order the racks".into(),
            outcome: 0,
            assignee_ref: Some("person:11111111-1111-1111-1111-111111111111".into()),
            due_on: Some(day("2026-11-10")),
            points: None,
        }
    }

    fn judged(p: &ProposalIn, taken: &[usize]) -> Verdict {
        let kinds = vec!["action".to_string(), "note".to_string()];
        let m = Meeting {
            minutes: "We agreed.\nPriya  will ORDER the racks by the tenth.",
            on: day("2026-11-03"),
            outcome_kinds: &kinds,
            taken,
        };
        judge(p, &m, &people())
    }

    #[test]
    fn a_quote_is_found_across_case_and_line_breaks_and_a_made_up_one_is_not() {
        let minutes = "We agreed.\nPriya  will ORDER the racks by the tenth.";
        assert!(excerpt_found(minutes, "priya will order the racks"));
        assert!(!excerpt_found(minutes, "Priya will order the servers"));
        assert!(
            !excerpt_found(minutes, "agreed"),
            "too short to be evidence"
        );
    }

    #[test]
    fn the_due_window_is_a_week_before_to_a_year_after() {
        let m = day("2026-11-03");
        assert!(due_in_window(day("2026-10-27"), m));
        assert!(!due_in_window(day("2026-10-26"), m));
        assert!(due_in_window(day("2027-11-03"), m));
        assert!(!due_in_window(day("2027-11-04"), m));
    }

    #[test]
    fn a_proposal_is_ready_or_dropped_with_a_reason() {
        assert_eq!(
            judged(&proposal(), &[]),
            Verdict::Ready(Some("person:11111111-1111-1111-1111-111111111111".into()))
        );
        let mut p = proposal();
        p.excerpt = "Priya will order the servers".into();
        assert_eq!(judged(&p, &[]), Verdict::Dropped("excerpt_not_in_minutes"));
        let mut p = proposal();
        p.outcome = 1;
        assert_eq!(judged(&p, &[]), Verdict::Dropped("outcome_not_an_action"));
        p.outcome = 7;
        assert_eq!(judged(&p, &[]), Verdict::Dropped("outcome_not_found"));
        assert_eq!(
            judged(&proposal(), &[0]),
            Verdict::Dropped("outcome_already_has_a_task")
        );
        let mut p = proposal();
        p.due_on = Some(day("2030-01-01"));
        assert_eq!(judged(&p, &[]), Verdict::Dropped("due_outside_window"));
    }

    #[test]
    fn an_assignee_must_already_be_a_person_of_the_plan_and_none_is_allowed() {
        let mut p = proposal();
        p.assignee_ref = Some("person:99999999-9999-9999-9999-999999999999".into());
        assert_eq!(judged(&p, &[]), Verdict::Dropped("assignee_not_in_plan"));
        p.assignee_ref = None;
        assert_eq!(judged(&p, &[]), Verdict::Ready(None));
    }
}
