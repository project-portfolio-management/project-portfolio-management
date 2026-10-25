//! Meetings (NC-14): an agenda before, minutes and outcomes after, linked to the plan's items. The rules and the
//! printable text. DB-free, unit-tested.
//!
//! A meeting is `planned`, then `held` or `cancelled`; a held or cancelled meeting is a record and is not edited.
//! The printable text states the meeting's own dates and never the moment it was rendered, so two renders of the same
//! meeting are byte-identical (the PDF writer is deterministic, [`crate::pdf`]).

use serde::{Deserialize, Serialize};

/// The most agenda items, and the most outcomes, one meeting holds.
pub const MAX_ITEMS: usize = 50;
/// The longest title, in characters.
pub const MAX_TITLE: usize = 200;
/// The longest agenda item or outcome text, in characters.
pub const MAX_TEXT: usize = 500;
/// The longest minutes, in characters.
pub const MAX_MINUTES: usize = 20_000;
/// What an outcome is.
pub const OUTCOME_KINDS: &[&str] = &["decision", "action", "note"];
/// The plan items an agenda item or outcome may point at. Each names a record of the same plan.
pub const LINK_KINDS: &[&str] = &[
    "task",
    "risk",
    "issue",
    "milestone",
    "change_request",
    "cutover_decision",
];

/// A pointer from an agenda item or outcome to a record of the plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
pub struct Link {
    /// One of [`LINK_KINDS`].
    pub kind: String,
    /// The record's `pid`.
    pub pid: uuid::Uuid,
}

/// One agenda item as given.
#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct AgendaIn {
    /// What is to be discussed.
    pub text: String,
    /// The plan record it concerns, if any.
    #[serde(default)]
    pub link: Option<Link>,
}

/// One outcome as given.
#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct OutcomeIn {
    /// `decision`, `action` or `note`.
    pub kind: String,
    /// What was decided, to be done, or noted.
    pub text: String,
    /// The plan record it concerns, if any.
    #[serde(default)]
    pub link: Option<Link>,
}

fn text_problem(what: &str, text: &str, out: &mut Vec<String>) {
    if text.trim().is_empty() || text.chars().count() > MAX_TEXT {
        out.push(format!(
            "{what} text is required (up to {MAX_TEXT} characters)"
        ));
    }
}

fn link_problem(what: &str, link: Option<&Link>, out: &mut Vec<String>) {
    if let Some(l) = link
        && !LINK_KINDS.contains(&l.kind.as_str())
    {
        out.push(format!(
            "{what} link kind is one of {}",
            LINK_KINDS.join(", ")
        ));
    }
}

/// Everything wrong with a new meeting.
#[must_use]
pub fn problems_planned(title: &str, agenda: &[AgendaIn]) -> Vec<String> {
    let mut out = Vec::new();
    if title.trim().is_empty() || title.chars().count() > MAX_TITLE {
        out.push(format!("title is required (up to {MAX_TITLE} characters)"));
    }
    if agenda.len() > MAX_ITEMS {
        out.push(format!("an agenda holds at most {MAX_ITEMS} items"));
    }
    for a in agenda {
        text_problem("an agenda item's", &a.text, &mut out);
        link_problem("an agenda item's", a.link.as_ref(), &mut out);
    }
    out
}

/// Everything wrong with the record of a meeting held.
#[must_use]
pub fn problems_held(minutes: &str, outcomes: &[OutcomeIn]) -> Vec<String> {
    let mut out = Vec::new();
    if minutes.trim().is_empty() || minutes.chars().count() > MAX_MINUTES {
        out.push(format!(
            "minutes are required (up to {MAX_MINUTES} characters)"
        ));
    }
    if outcomes.len() > MAX_ITEMS {
        out.push(format!("a meeting holds at most {MAX_ITEMS} outcomes"));
    }
    for o in outcomes {
        if !OUTCOME_KINDS.contains(&o.kind.as_str()) {
            out.push(format!(
                "an outcome's kind is one of {}",
                OUTCOME_KINDS.join(", ")
            ));
        }
        text_problem("an outcome's", &o.text, &mut out);
        link_problem("an outcome's", o.link.as_ref(), &mut out);
    }
    out.dedup();
    out
}

/// One printed line item: its text and what it points at, already described.
#[derive(Debug, Clone)]
pub struct Line {
    /// `decision`, `action`, `note` for an outcome; empty for an agenda item.
    pub kind: String,
    /// The text.
    pub text: String,
    /// A description of the linked record, e.g. `task: Build the API`.
    pub link: Option<String>,
}

/// What the printed meeting holds.
#[derive(Debug, Clone)]
pub struct Printable {
    /// The plan's name.
    pub plan: String,
    /// The meeting title.
    pub title: String,
    /// `planned`, `held` or `cancelled`.
    pub status: String,
    /// The date it was scheduled for.
    pub scheduled_on: String,
    /// The date it was held, when it was.
    pub held_on: Option<String>,
    /// The agenda, in order.
    pub agenda: Vec<Line>,
    /// The minutes, when held.
    pub minutes: Option<String>,
    /// The outcomes, in order.
    pub outcomes: Vec<Line>,
}

fn items(heading: &str, lines: &[Line], out: &mut Vec<String>) {
    out.push(String::new());
    out.push(heading.to_string());
    if lines.is_empty() {
        out.push("  (none)".to_string());
    }
    for (i, l) in lines.iter().enumerate() {
        let kind = if l.kind.is_empty() {
            String::new()
        } else {
            format!("[{}] ", l.kind)
        };
        out.push(format!("  {}. {kind}{}", i + 1, l.text));
        if let Some(link) = &l.link {
            out.push(format!("      about {link}"));
        }
    }
}

/// The lines of the printed meeting.
#[must_use]
pub fn lines(p: &Printable) -> Vec<String> {
    let mut out = vec![
        format!("Plan: {}", p.plan),
        format!("Status: {}", p.status),
        format!("Scheduled for: {}", p.scheduled_on),
    ];
    if let Some(h) = &p.held_on {
        out.push(format!("Held on: {h}"));
    }
    items("Agenda", &p.agenda, &mut out);
    if let Some(m) = &p.minutes {
        out.push(String::new());
        out.push("Minutes".to_string());
        out.extend(m.lines().map(|l| format!("  {l}")));
        items("Outcomes", &p.outcomes, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agenda(text: &str) -> AgendaIn {
        AgendaIn {
            text: text.into(),
            link: None,
        }
    }

    #[test]
    fn a_new_meeting_needs_a_title_and_agenda_items_with_text_and_a_known_link() {
        assert!(problems_planned("Kick-off", &[agenda("Scope")]).is_empty());
        assert!(
            problems_planned("Kick-off", &[]).is_empty(),
            "an agenda may be empty"
        );
        assert_eq!(problems_planned("  ", &[agenda("")]).len(), 2);
        let bad = AgendaIn {
            text: "x".into(),
            link: Some(Link {
                kind: "plan".into(),
                pid: uuid::Uuid::nil(),
            }),
        };
        assert!(problems_planned("T", &[bad])[0].contains("link kind"));
        let many: Vec<AgendaIn> = (0..=MAX_ITEMS).map(|_| agenda("x")).collect();
        assert!(problems_planned("T", &many)[0].contains("at most"));
    }

    #[test]
    fn a_held_meeting_needs_minutes_and_outcomes_of_a_known_kind() {
        let ok = OutcomeIn {
            kind: "decision".into(),
            text: "Go".into(),
            link: None,
        };
        assert!(problems_held("We met.", std::slice::from_ref(&ok)).is_empty());
        assert_eq!(problems_held("", &[]).len(), 1);
        let bad = OutcomeIn {
            kind: "vote".into(),
            ..ok
        };
        assert!(problems_held("m", &[bad])[0].contains("kind is one of"));
    }

    /// The meeting printed in the golden test: one of each part, and a link.
    fn golden_meeting() -> Printable {
        Printable {
            plan: "Apollo".into(),
            title: "Gate 2 review".into(),
            status: "held".into(),
            scheduled_on: "2026-11-02".into(),
            held_on: Some("2026-11-03".into()),
            agenda: vec![
                Line {
                    kind: String::new(),
                    text: "Is the budget still right?".into(),
                    link: Some("milestone: Beta".into()),
                },
                Line {
                    kind: String::new(),
                    text: "Open risks".into(),
                    link: None,
                },
            ],
            minutes: Some("The sponsor attended.\nThe budget holds for one more quarter.".into()),
            outcomes: vec![
                Line {
                    kind: "decision".into(),
                    text: "Continue to gate 3".into(),
                    link: Some("change_request: Extend scope".into()),
                },
                Line {
                    kind: "action".into(),
                    text: "Re-baseline the schedule (by the lead)".into(),
                    link: Some("task: Re-plan".into()),
                },
            ],
        }
    }

    /// The PDF of a fixed meeting equals the file in `tests/fixtures`, byte for byte. Regenerate with
    /// `UPDATE_GOLDEN=1 cargo test --lib meetings` after a deliberate change to the layout, and look at the file.
    #[test]
    fn the_printed_meeting_matches_the_golden_pdf() {
        let bytes = crate::pdf::render("Meeting: Gate 2 review", &lines(&golden_meeting()));
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/meeting-golden.pdf");
        if std::env::var_os("UPDATE_GOLDEN").is_some() {
            std::fs::write(&path, &bytes).expect("write the golden file");
        }
        let golden = std::fs::read(&path)
            .expect("tests/fixtures/meeting-golden.pdf; regenerate with UPDATE_GOLDEN=1");
        assert!(
            bytes == golden,
            "the rendered meeting differs from the golden file; if the change is deliberate, rerun with UPDATE_GOLDEN=1"
        );
        let text = crate::pdf::text_of(&golden);
        assert!(
            text.contains("Continue to gate 3") && text.contains("Held on: 2026-11-03"),
            "{text}"
        );
    }

    #[test]
    fn the_printed_lines_state_the_agenda_and_only_a_held_meeting_has_minutes_and_outcomes() {
        let mut p = Printable {
            plan: "Apollo".into(),
            title: "Kick-off".into(),
            status: "planned".into(),
            scheduled_on: "2026-11-02".into(),
            held_on: None,
            agenda: vec![Line {
                kind: String::new(),
                text: "Scope".into(),
                link: Some("task: Build".into()),
            }],
            minutes: None,
            outcomes: vec![],
        };
        let planned = lines(&p).join("\n");
        assert!(planned.contains("1. Scope") && planned.contains("about task: Build"));
        assert!(!planned.contains("Minutes") && !planned.contains("Outcomes"));
        p.status = "held".into();
        p.held_on = Some("2026-11-02".into());
        p.minutes = Some("We agreed.\nGo.".into());
        p.outcomes = vec![Line {
            kind: "decision".into(),
            text: "Go ahead".into(),
            link: None,
        }];
        let held = lines(&p).join("\n");
        assert!(held.contains("Held on: 2026-11-02") && held.contains("  Go."));
        assert!(held.contains("1. [decision] Go ahead"));
    }
}
