//! The RACI matrix (pure rules). DB-free, unit-tested.
//!
//! **R**esponsible does the work; **A**ccountable answers for it and signs it off
//! (one per task — two accountable people is nobody accountable); **C**onsulted is
//! asked before; **I**nformed is told after. A person may hold several letters on
//! a task (the common *A/R* is the same person doing the work and owning it).
//!
//! Two kinds of problem are kept apart, because they deserve different answers:
//!
//! - **refused**: more than one Accountable, an unknown letter, a duplicate, too
//!   many parties — the data would be wrong;
//! - **reported**: no Accountable, no Responsible — the row is *incomplete*, which
//!   is a normal state while a plan is being set up and is shown as a gap, not
//!   hidden and not blocked.

use std::collections::BTreeMap;

use serde::Serialize;
use uuid::Uuid;

/// The four roles, in the order they are written.
pub const ROLES: [char; 4] = ['R', 'A', 'C', 'I'];
/// Most people on one task's row.
pub const MAX_PARTIES: usize = 50;

/// What is wrong or missing on a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Finding {
    /// Nobody is Accountable.
    NoAccountable,
    /// Nobody is Responsible.
    NoResponsible,
    /// More than one Accountable (refused on write; reported if it is ever found).
    ManyAccountable,
}

/// One person's letters on a task, as submitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The person (`person:` / `worker:` URN).
    pub person_ref: String,
    /// Their letters.
    pub roles: Vec<char>,
}

/// Normalise a letters string like `"ar"` to `['A', 'R']`: upper-cased, in
/// R-A-C-I order, no duplicates. `None` when any character is not a RACI letter.
#[must_use]
pub fn parse_roles(s: &str) -> Option<Vec<char>> {
    let mut seen = [false; 4];
    for c in s.chars() {
        let i = ROLES.iter().position(|r| *r == c.to_ascii_uppercase())?;
        seen[i] = true;
    }
    Some(
        ROLES
            .iter()
            .zip(seen)
            .filter(|(_, s)| *s)
            .map(|(r, _)| *r)
            .collect(),
    )
}

/// Every reason a submitted row is **refused** (all of them, not the first).
#[must_use]
pub fn refusals(entries: &[Entry]) -> Vec<String> {
    let mut out = Vec::new();
    if entries.len() > MAX_PARTIES {
        out.push(format!("at most {MAX_PARTIES} people on one task"));
    }
    let mut people: Vec<&str> = entries.iter().map(|e| e.person_ref.as_str()).collect();
    people.sort_unstable();
    let n = people.len();
    people.dedup();
    if people.len() != n {
        out.push(
            "a person may appear once in a row (give them several letters instead)".to_string(),
        );
    }
    if entries.iter().any(|e| e.roles.is_empty()) {
        out.push("every person in the row needs at least one letter (R, A, C or I); leave them out to remove them".to_string());
    }
    let accountable = entries.iter().filter(|e| e.roles.contains(&'A')).count();
    if accountable > 1 {
        out.push(format!(
            "a task has one Accountable person, not {accountable}"
        ));
    }
    out
}

/// What is **incomplete** about a row (never refused).
#[must_use]
pub fn findings(entries: &[Entry]) -> Vec<Finding> {
    let mut out = Vec::new();
    let a = entries.iter().filter(|e| e.roles.contains(&'A')).count();
    if a == 0 {
        out.push(Finding::NoAccountable);
    }
    if a > 1 {
        out.push(Finding::ManyAccountable);
    }
    if !entries.iter().any(|e| e.roles.contains(&'R')) {
        out.push(Finding::NoResponsible);
    }
    out
}

/// One task's row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Row {
    /// The task.
    pub task_pid: Uuid,
    /// Its title.
    pub title: String,
    /// person → letters (`"AR"`), only people who hold one.
    pub cells: BTreeMap<String, String>,
    /// What is incomplete.
    pub findings: Vec<Finding>,
}

/// A person's load across the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Workload {
    /// The person.
    pub person_ref: String,
    /// Tasks they are Responsible for.
    pub responsible: usize,
    /// Tasks they are Accountable for.
    pub accountable: usize,
    /// Tasks they are Consulted on.
    pub consulted: usize,
    /// Tasks they are Informed of.
    pub informed: usize,
}

/// The counts, which reconcile with the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// Tasks in the matrix.
    pub tasks: usize,
    /// Tasks with exactly one Accountable and at least one Responsible.
    pub complete: usize,
    /// Tasks with no Accountable.
    pub missing_accountable: usize,
    /// Tasks with more than one Accountable.
    pub multiple_accountable: usize,
    /// Tasks with no Responsible.
    pub missing_responsible: usize,
}

/// The whole matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Matrix {
    /// The columns: everyone who holds a letter anywhere, sorted.
    pub people: Vec<String>,
    /// One row per task given (a task with no assignments is a row with no cells).
    pub rows: Vec<Row>,
    /// Counts.
    pub summary: Summary,
    /// Each person's load.
    pub workload: Vec<Workload>,
}

/// Build the matrix from `tasks` (pid, title, in the order to show) and every
/// `(task, person, letter)` assignment.
#[must_use]
pub fn matrix(tasks: &[(Uuid, String)], assignments: &[(Uuid, String, char)]) -> Matrix {
    let mut people: Vec<String> = assignments.iter().map(|(_, p, _)| p.clone()).collect();
    people.sort();
    people.dedup();
    let mut rows = Vec::with_capacity(tasks.len());
    for (pid, title) in tasks {
        let mut by_person: BTreeMap<String, Vec<char>> = BTreeMap::new();
        for (t, p, r) in assignments {
            if t == pid {
                by_person.entry(p.clone()).or_default().push(*r);
            }
        }
        let entries: Vec<Entry> = by_person
            .iter()
            .map(|(p, rs)| Entry {
                person_ref: p.clone(),
                roles: parse_roles(&rs.iter().collect::<String>()).unwrap_or_default(),
            })
            .collect();
        rows.push(Row {
            task_pid: *pid,
            title: title.clone(),
            cells: entries
                .iter()
                .map(|e| (e.person_ref.clone(), e.roles.iter().collect()))
                .collect(),
            findings: findings(&entries),
        });
    }
    let count = |f: Finding| rows.iter().filter(|r| r.findings.contains(&f)).count();
    let summary = Summary {
        tasks: rows.len(),
        complete: rows.iter().filter(|r| r.findings.is_empty()).count(),
        missing_accountable: count(Finding::NoAccountable),
        multiple_accountable: count(Finding::ManyAccountable),
        missing_responsible: count(Finding::NoResponsible),
    };
    let workload = people
        .iter()
        .map(|p| {
            let has = |c: char| {
                rows.iter()
                    .filter(|r| r.cells.get(p).is_some_and(|l| l.contains(c)))
                    .count()
            };
            Workload {
                person_ref: p.clone(),
                responsible: has('R'),
                accountable: has('A'),
                consulted: has('C'),
                informed: has('I'),
            }
        })
        .collect();
    Matrix {
        people,
        rows,
        summary,
        workload,
    }
}

/// The matrix as CSV: a header of people, one row per task, cells as letters
/// (`AR`), fields quoted when they need it.
#[must_use]
pub fn to_csv(m: &Matrix) -> String {
    let esc = |s: &str| {
        if s.contains([',', '"', '\n', '\r']) {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    };
    let mut out = String::from("task");
    for p in &m.people {
        out.push(',');
        out.push_str(&esc(p));
    }
    out.push_str(",gaps\n");
    for r in &m.rows {
        out.push_str(&esc(&r.title));
        for p in &m.people {
            out.push(',');
            out.push_str(r.cells.get(p).map_or("", String::as_str));
        }
        let gaps: Vec<&str> = r
            .findings
            .iter()
            .map(|f| match f {
                Finding::NoAccountable => "no accountable",
                Finding::NoResponsible => "no responsible",
                Finding::ManyAccountable => "several accountable",
            })
            .collect();
        out.push(',');
        out.push_str(&esc(&gaps.join("; ")));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn e(p: &str, letters: &str) -> Entry {
        Entry {
            person_ref: p.to_string(),
            roles: parse_roles(letters).unwrap(),
        }
    }

    #[test]
    fn letters_are_normalised_ordered_and_deduplicated() {
        assert_eq!(
            parse_roles("ar"),
            Some(vec!['R', 'A']),
            "R-A-C-I order, upper-cased"
        );
        assert_eq!(parse_roles("IIAA"), Some(vec!['A', 'I']));
        assert_eq!(parse_roles(""), Some(vec![]));
        assert_eq!(parse_roles("RX"), None);
        assert_eq!(parse_roles("R A"), None, "spaces are not letters");
    }

    #[test]
    fn two_accountable_people_are_refused_and_none_is_only_reported() {
        assert!(
            refusals(&[e("person:a", "AR"), e("person:b", "C")]).is_empty(),
            "A/R on one person is fine"
        );
        let r = refusals(&[e("person:a", "A"), e("person:b", "A")]);
        assert_eq!(
            r,
            vec!["a task has one Accountable person, not 2".to_string()]
        );
        assert!(
            refusals(&[e("person:a", "R")]).is_empty(),
            "no Accountable is allowed, just flagged"
        );
        assert_eq!(
            findings(&[e("person:a", "R")]),
            vec![Finding::NoAccountable]
        );
    }

    #[test]
    fn every_refusal_is_listed_not_just_the_first() {
        let many: Vec<Entry> = (0..=MAX_PARTIES)
            .map(|i| e(&format!("person:{i}"), "C"))
            .collect();
        assert_eq!(refusals(&many).len(), 1);
        let bad = vec![
            e("person:a", "A"),
            e("person:a", "R"),
            Entry {
                person_ref: "person:b".into(),
                roles: vec![],
            },
            e("person:c", "A"),
        ];
        assert_eq!(
            refusals(&bad).len(),
            3,
            "duplicate person, empty letters, two accountable"
        );
    }

    #[test]
    fn incomplete_rows_are_reported_by_what_is_missing() {
        assert_eq!(
            findings(&[]),
            vec![Finding::NoAccountable, Finding::NoResponsible]
        );
        assert_eq!(
            findings(&[e("person:a", "A")]),
            vec![Finding::NoResponsible]
        );
        assert_eq!(findings(&[e("person:a", "AR")]), vec![]);
        assert_eq!(
            findings(&[e("person:a", "A"), e("person:b", "A"), e("person:b", "R")]).len(),
            1
        );
    }

    #[test]
    fn the_matrix_lays_people_across_and_tasks_down() {
        let tasks = vec![
            (id(1), "Design".to_string()),
            (id(2), "Build".to_string()),
            (id(3), "Untouched".to_string()),
        ];
        let a = vec![
            (id(1), "person:ann".to_string(), 'A'),
            (id(1), "person:ann".to_string(), 'R'),
            (id(1), "person:bob".to_string(), 'C'),
            (id(2), "person:bob".to_string(), 'R'),
            (id(2), "person:cy".to_string(), 'I'),
        ];
        let m = matrix(&tasks, &a);
        assert_eq!(m.people, vec!["person:ann", "person:bob", "person:cy"]);
        assert_eq!(
            m.rows[0].cells.get("person:ann").map(String::as_str),
            Some("RA")
        );
        assert_eq!(
            m.rows[0].cells.get("person:cy"),
            None,
            "a person with no letter on a task has no cell"
        );
        assert!(
            m.rows[2].cells.is_empty(),
            "a task with no assignments is still a row"
        );
        assert_eq!(
            m.rows[2].findings,
            vec![Finding::NoAccountable, Finding::NoResponsible]
        );
        assert_eq!(
            m.summary,
            Summary {
                tasks: 3,
                complete: 1,
                missing_accountable: 2,
                multiple_accountable: 0,
                missing_responsible: 1
            }
        );
        let bob = m
            .workload
            .iter()
            .find(|w| w.person_ref == "person:bob")
            .unwrap();
        assert_eq!(
            (
                bob.responsible,
                bob.accountable,
                bob.consulted,
                bob.informed
            ),
            (1, 0, 1, 0)
        );
    }

    #[test]
    fn the_summary_reconciles_with_the_rows() {
        let tasks: Vec<(Uuid, String)> = (1..=5).map(|n| (id(n), format!("T{n}"))).collect();
        let a = vec![
            (id(1), "p".to_string(), 'A'),
            (id(1), "p".to_string(), 'R'),
            (id(2), "p".to_string(), 'R'),
        ];
        let m = matrix(&tasks, &a);
        assert_eq!(m.summary.tasks, m.rows.len());
        assert_eq!(
            m.summary.complete + m.rows.iter().filter(|r| !r.findings.is_empty()).count(),
            m.summary.tasks
        );
        assert_eq!(m.summary.complete, 1);
    }

    #[test]
    fn the_csv_has_people_as_columns_letters_as_cells_and_the_gaps_in_words() {
        let tasks = vec![
            (id(1), "Design, v2".to_string()),
            (id(2), "Build".to_string()),
        ];
        let a = vec![
            (id(1), "person:ann".to_string(), 'A'),
            (id(1), "person:ann".to_string(), 'R'),
        ];
        let csv = to_csv(&matrix(&tasks, &a));
        assert_eq!(
            csv,
            "task,person:ann,gaps\n\"Design, v2\",RA,\nBuild,,no accountable; no responsible\n"
        );
    }
}
