//! Starter templates (BR-13): five ready-made plan trees in the template snapshot format, so a new deployment has
//! something to instantiate on day one. DB-free and unit-tested.
//!
//! A starter is **code, not a row**: it is listed after the stored templates, found by its fixed `pid`, and
//! instantiated like any template, but it cannot be deleted, revised or saved over by name. It has no people, no
//! identifiers and no history, only the shape (plans, components, tasks and milestones with day offsets from the start
//! date given at instantiation). Changing a starter changes it for every later instantiation and nothing already made.

use project_portfolio_management_matcher::{Plan, PlanKind};
use uuid::Uuid;

use crate::plan_templates::{Snapshot, TplMilestone, TplNode, TplTask};

/// One starter as listed.
pub struct Starter {
    /// Its fixed id.
    pub pid: Uuid,
    /// Its name; a saved template cannot take it.
    pub name: &'static str,
    /// What it is for.
    pub description: &'static str,
    /// The tree.
    pub snapshot: Snapshot,
}

fn task(
    key: usize,
    title: &str,
    points: i32,
    due: i64,
    component: &str,
    parent: Option<usize>,
) -> TplTask {
    TplTask {
        key,
        title: title.to_string(),
        description: None,
        points: Some(points),
        flow_type: None,
        due_offset_days: Some(due),
        component: Some(component.to_string()),
        parent,
    }
}

fn milestone(name: &str, kind: &str, offset_days: i64) -> TplMilestone {
    TplMilestone {
        name: name.to_string(),
        kind: Some(kind.to_string()),
        offset_days,
    }
}

#[expect(clippy::too_many_arguments)] // a literal tree, written once
fn node(
    key: usize,
    parent: Option<usize>,
    kind: PlanKind,
    name: &str,
    keywords: &[&str],
    target: i64,
    components: &[&str],
    tasks: Vec<TplTask>,
    milestones: Vec<TplMilestone>,
) -> TplNode {
    let mut plan = Plan::new(name);
    plan.kind = Some(kind);
    plan.keywords = keywords.iter().map(|k| (*k).to_string()).collect();
    plan.tags = vec!["starter".to_string()];
    TplNode {
        key,
        parent,
        plan,
        start_offset_days: Some(0),
        target_offset_days: Some(target),
        components: components.iter().map(|c| (*c).to_string()).collect(),
        tasks,
        milestones,
        workflow: None,
    }
}

fn snapshot(nodes: Vec<TplNode>) -> Snapshot {
    Snapshot {
        base_date: None,
        has_offsets: true,
        nodes,
    }
}

fn stage_gated_project() -> Snapshot {
    let c = ["Initiation", "Planning", "Delivery", "Closure"];
    snapshot(vec![node(
        0,
        None,
        PlanKind::Project,
        "Stage-gated project",
        &["stage gate", "project"],
        180,
        &c,
        vec![
            task(0, "Write the business case", 5, 14, "Initiation", None),
            task(
                1,
                "Name the sponsor and the board",
                2,
                7,
                "Initiation",
                None,
            ),
            task(
                2,
                "Agree scope and success measures",
                3,
                21,
                "Planning",
                None,
            ),
            task(
                3,
                "Build the plan, budget and risk register",
                8,
                35,
                "Planning",
                None,
            ),
            task(4, "Plan the build in stages", 3, 42, "Planning", Some(3)),
            task(5, "Deliver the first stage", 13, 90, "Delivery", None),
            task(6, "Deliver the remaining stages", 21, 150, "Delivery", None),
            task(
                7,
                "Test against the success measures",
                8,
                160,
                "Delivery",
                Some(6),
            ),
            task(8, "Hand over to operations", 5, 170, "Closure", None),
            task(9, "Record lessons learned", 2, 175, "Closure", None),
            task(
                10,
                "Close the project and release the team",
                2,
                180,
                "Closure",
                None,
            ),
        ],
        vec![
            milestone("Gate 0: concept accepted", "gate", 7),
            milestone("Gate 1: business case approved", "gate", 21),
            milestone("Gate 2: plan approved", "gate", 42),
            milestone("Gate 3: ready to go live", "gate", 160),
            milestone("Gate 4: handed over", "gate", 170),
            milestone("Gate 5: benefits reviewed", "gate", 180),
        ],
    )])
}

fn programme_with_workstreams() -> Snapshot {
    let stream = |key, name: &str, kw: &str| {
        node(
            key,
            Some(0),
            PlanKind::Project,
            name,
            &[kw],
            270,
            &["Plan", "Do", "Review"],
            vec![
                task(
                    0,
                    "Agree the workstream's scope and owner",
                    3,
                    14,
                    "Plan",
                    None,
                ),
                task(1, "Break the work into deliverables", 5, 30, "Plan", None),
                task(2, "Deliver the first increment", 13, 120, "Do", None),
                task(
                    3,
                    "Report progress and risks to the programme",
                    2,
                    135,
                    "Review",
                    None,
                ),
            ],
            vec![milestone("Workstream plan agreed", "milestone", 30)],
        )
    };
    snapshot(vec![
        node(
            0,
            None,
            PlanKind::Program,
            "Programme with workstreams",
            &["programme", "workstreams"],
            360,
            &["Governance", "Dependencies"],
            vec![
                task(0, "Set up the programme board", 3, 14, "Governance", None),
                task(
                    1,
                    "Agree the vision and benefits",
                    5,
                    30,
                    "Governance",
                    None,
                ),
                task(
                    2,
                    "Map dependencies between workstreams",
                    5,
                    45,
                    "Dependencies",
                    None,
                ),
                task(
                    3,
                    "Review the programme at each quarter",
                    3,
                    90,
                    "Governance",
                    None,
                ),
                task(
                    4,
                    "Close the programme and confirm benefits",
                    5,
                    360,
                    "Governance",
                    None,
                ),
            ],
            vec![
                milestone("Programme mandate signed", "milestone", 14),
                milestone("Programme closed", "milestone", 360),
            ],
        ),
        stream(1, "Workstream: delivery", "delivery"),
        stream(2, "Workstream: change and adoption", "adoption"),
        stream(3, "Workstream: governance and reporting", "reporting"),
    ])
}

fn benefits_realisation_review() -> Snapshot {
    let c = ["Baseline", "Measure", "Report"];
    snapshot(vec![node(
        0,
        None,
        PlanKind::Process,
        "Benefits realisation review",
        &["benefits", "review"],
        60,
        &c,
        vec![
            task(
                0,
                "List the benefits the business case promised",
                3,
                5,
                "Baseline",
                None,
            ),
            task(
                1,
                "Find the baseline value of each measure",
                5,
                12,
                "Baseline",
                None,
            ),
            task(2, "Name an owner for each benefit", 2, 14, "Baseline", None),
            task(
                3,
                "Collect the current value of each measure",
                5,
                30,
                "Measure",
                None,
            ),
            task(
                4,
                "Ask owners what has changed and why",
                5,
                40,
                "Measure",
                None,
            ),
            task(
                5,
                "Compare what was promised with what happened",
                5,
                48,
                "Report",
                None,
            ),
            task(
                6,
                "Write the review and its recommendations",
                5,
                55,
                "Report",
                None,
            ),
            task(
                7,
                "Present to the board and agree the next review date",
                2,
                60,
                "Report",
                None,
            ),
        ],
        vec![
            milestone("Baseline agreed", "milestone", 14),
            milestone("Review presented", "milestone", 60),
        ],
    )])
}

fn change_request_process() -> Snapshot {
    let c = ["Intake", "Assessment", "Decision", "Delivery"];
    snapshot(vec![node(
        0,
        None,
        PlanKind::Process,
        "Change request process",
        &["change request", "change control"],
        30,
        &c,
        vec![
            task(
                0,
                "Log the change request and who asked",
                1,
                1,
                "Intake",
                None,
            ),
            task(
                1,
                "Describe the change and why it is wanted",
                2,
                3,
                "Intake",
                None,
            ),
            task(
                2,
                "Assess the effect on scope, cost and schedule",
                5,
                8,
                "Assessment",
                None,
            ),
            task(
                3,
                "Assess the effect on risks and dependencies",
                3,
                8,
                "Assessment",
                None,
            ),
            task(
                4,
                "Take the request to the change board",
                2,
                12,
                "Decision",
                None,
            ),
            task(
                5,
                "Record the decision and the reason",
                1,
                13,
                "Decision",
                Some(4),
            ),
            task(6, "Make the approved change", 8, 25, "Delivery", None),
            task(
                7,
                "Check the change did what it should and close the request",
                2,
                30,
                "Delivery",
                None,
            ),
        ],
        vec![
            milestone("Decision recorded", "milestone", 13),
            milestone("Request closed", "milestone", 30),
        ],
    )])
}

fn incident_response() -> Snapshot {
    let c = ["Detect", "Contain", "Recover", "Review"];
    snapshot(vec![node(
        0,
        None,
        PlanKind::Process,
        "Incident response",
        &["incident", "response"],
        14,
        &c,
        vec![
            task(
                0,
                "Declare the incident and name an incident lead",
                1,
                0,
                "Detect",
                None,
            ),
            task(
                1,
                "Record what is known and what is not",
                1,
                0,
                "Detect",
                None,
            ),
            task(2, "Contain the harm", 3, 1, "Contain", None),
            task(
                3,
                "Tell the people affected and the people who must be told",
                2,
                1,
                "Contain",
                None,
            ),
            task(4, "Find the cause", 5, 4, "Recover", None),
            task(5, "Restore normal service", 5, 5, "Recover", Some(4)),
            task(6, "Hold a blameless review", 3, 10, "Review", None),
            task(
                7,
                "Agree and assign the follow-up actions",
                2,
                14,
                "Review",
                Some(6),
            ),
        ],
        vec![
            milestone("Contained", "milestone", 1),
            milestone("Service restored", "milestone", 5),
            milestone("Review held", "milestone", 10),
        ],
    )])
}

/// The five starters, in the order they are listed.
#[must_use]
pub fn starters() -> Vec<Starter> {
    let make = |id: u128, name, description, snapshot| Starter {
        pid: Uuid::from_u128(0x5747_a000_0000_4000_8000_0000_0000_0000 | id),
        name,
        description,
        snapshot,
    };
    vec![
        make(
            1,
            "Stage-gated project",
            "A project with four stages and six gates, from business case to benefits review.",
            stage_gated_project(),
        ),
        make(
            2,
            "Programme with workstreams",
            "A programme with a board and three workstreams, each its own plan.",
            programme_with_workstreams(),
        ),
        make(
            3,
            "Benefits realisation review",
            "Compare what a project promised with what happened, and report it.",
            benefits_realisation_review(),
        ),
        make(
            4,
            "Change request process",
            "From logging a request to closing it, with the decision recorded.",
            change_request_process(),
        ),
        make(
            5,
            "Incident response",
            "Declare, contain, recover and review an incident.",
            incident_response(),
        ),
    ]
}

/// The starter with this id, if it is one.
#[must_use]
pub fn find(pid: Uuid) -> Option<Starter> {
    starters().into_iter().find(|s| s.pid == pid)
}

/// Whether a template name is a starter's, ignoring case.
#[must_use]
pub fn is_starter_name(name: &str) -> bool {
    starters()
        .iter()
        .any(|s| s.name.to_lowercase() == name.trim().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn there_are_five_with_distinct_ids_and_names() {
        let all = starters();
        assert_eq!(all.len(), 5);
        assert_eq!(all.iter().map(|s| s.pid).collect::<HashSet<_>>().len(), 5);
        assert_eq!(
            all.iter()
                .map(|s| s.name.to_lowercase())
                .collect::<HashSet<_>>()
                .len(),
            5
        );
        assert!(find(all[2].pid).is_some() && find(Uuid::new_v4()).is_none());
        assert!(is_starter_name("  incident RESPONSE "));
    }

    #[test]
    fn each_is_a_well_formed_tree_in_the_template_model() {
        for s in starters() {
            let n = &s.snapshot.nodes;
            assert!(
                !n.is_empty() && n.len() <= crate::plan_templates::MAX_PLANS,
                "{}",
                s.name
            );
            assert!(n[0].parent.is_none(), "{}: the root comes first", s.name);
            for (i, node) in n.iter().enumerate() {
                assert_eq!(node.key, i, "{}: keys are positions", s.name);
                assert!(
                    node.parent.is_none_or(|p| p < i),
                    "{}: parents come before children",
                    s.name
                );
                assert!(
                    crate::validation::problems(&node.plan).is_empty(),
                    "{}: {}",
                    s.name,
                    node.plan.name
                );
                assert!(
                    node.plan.lead_ref.is_none()
                        && node.plan.identifiers.is_empty()
                        && node.plan.code.is_none()
                );
                for (k, t) in node.tasks.iter().enumerate() {
                    assert_eq!(t.key, k, "{}: task keys are positions", s.name);
                    assert!(t.parent.is_none_or(|p| p < k), "{}: {}", s.name, t.title);
                    let c = t.component.as_deref().expect("a component");
                    assert!(
                        node.components.iter().any(|x| x == c),
                        "{}: {c} is declared",
                        s.name
                    );
                    assert!(
                        t.due_offset_days.unwrap() <= node.target_offset_days.unwrap(),
                        "{}: {}",
                        s.name,
                        t.title
                    );
                }
                for m in &node.milestones {
                    assert!(
                        m.offset_days <= node.target_offset_days.unwrap(),
                        "{}: {}",
                        s.name,
                        m.name
                    );
                }
            }
            assert!(s.snapshot.has_offsets);
        }
    }

    #[test]
    fn the_shapes_are_what_the_names_say() {
        let all = starters();
        let by = |name: &str| all.iter().find(|s| s.name == name).unwrap();
        assert_eq!(
            by("Stage-gated project").snapshot.nodes[0].milestones.len(),
            6,
            "gates 0 to 5"
        );
        assert_eq!(
            by("Programme with workstreams").snapshot.nodes.len(),
            4,
            "a programme and three workstreams"
        );
        for name in [
            "Benefits realisation review",
            "Change request process",
            "Incident response",
        ] {
            assert_eq!(by(name).snapshot.nodes.len(), 1);
        }
    }

    #[test]
    fn a_snapshot_survives_the_json_round_trip_the_instantiator_uses() {
        for s in starters() {
            let v = serde_json::to_value(&s.snapshot).unwrap();
            let back: Snapshot = serde_json::from_value(v).unwrap();
            assert_eq!(back.nodes.len(), s.snapshot.nodes.len());
        }
    }
}
