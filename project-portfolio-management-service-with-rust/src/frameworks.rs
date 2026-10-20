//! Frameworks: the pure rules. DB-free and deterministic.
//!
//! A *framework* is a named, reusable description a plan can follow. There are two families:
//!
//! * **ways of working** — how the team works: methodology, iteration length, WIP limit, definitions of ready and
//!   done, ceremonies;
//! * **governance** — how the work is governed: decision model, review and reporting cadence, escalation path,
//!   decision rights, forums.
//!
//! A plan follows at most one of each family, through a link; a plan with no link inherits the nearest ancestor's
//! ([`resolve`]). The family's own fields live in one JSON `body`, validated here ([`check`]); unknown fields are
//! refused, so a misspelt one is an error and not silently dropped.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

/// The two families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// How the team works.
    WaysOfWorking,
    /// How the work is governed.
    Governance,
}

impl Family {
    /// The stored token (also the `family` column's `CHECK`).
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::WaysOfWorking => "ways_of_working",
            Self::Governance => "governance",
        }
    }

    /// The key the effective answer for a plan carries the record under.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::WaysOfWorking => "ways_of_working",
            Self::Governance => "governance_framework",
        }
    }

    /// What the family is called in a sentence.
    #[must_use]
    pub const fn noun(self) -> &'static str {
        match self {
            Self::WaysOfWorking => "ways of working",
            Self::Governance => "governance framework",
        }
    }
}

/// The methodologies a ways-of-working record may name.
pub const METHODOLOGIES: &[&str] = &[
    "scrum",
    "kanban",
    "scrumban",
    "waterfall",
    "safe",
    "xp",
    "lean",
    "hybrid",
    "other",
];
/// The decision models a governance framework may name.
pub const DECISION_MODELS: &[&str] = &[
    "sponsor",
    "steering_committee",
    "product_owner",
    "delegated",
    "consensus",
    "other",
];
/// Longest name, in characters.
pub const NAME_MAX: usize = 120;
/// Longest description or definition, in characters.
pub const TEXT_MAX: usize = 10_000;
/// Most entries in any one list on a record.
pub const LIST_MAX: usize = 20;

/// A recurring practice: a standup, a retrospective, a planning session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ceremony {
    /// What it is called (up to 80 characters).
    pub name: String,
    /// How often, in words (up to 80 characters).
    pub cadence: String,
    /// Why it happens (optional, up to 300 characters).
    #[serde(default)]
    pub purpose: Option<String>,
}

/// The fields of a ways-of-working record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaysOfWorking {
    /// One of [`METHODOLOGIES`].
    #[serde(default = "other")]
    pub methodology: String,
    /// Iteration length in days (1 to 366).
    #[serde(default)]
    pub cadence_days: Option<i32>,
    /// Most items in progress at once (1 to 1000).
    #[serde(default)]
    pub wip_limit: Option<i32>,
    /// When work may start.
    #[serde(default)]
    pub definition_of_ready: Option<String>,
    /// When work is finished.
    #[serde(default)]
    pub definition_of_done: Option<String>,
    /// The recurring practices.
    #[serde(default)]
    pub ceremonies: Vec<Ceremony>,
}

/// One rung of an escalation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Escalation {
    /// 1 is the first place to go.
    pub level: u32,
    /// Who or what hears it (up to 120 characters).
    pub body: String,
    /// What sends something there (up to 300 characters).
    pub trigger: String,
}

/// Who decides what.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionRight {
    /// The kind of decision (up to 120 characters).
    pub decision: String,
    /// Who decides (up to 120 characters).
    pub decides: String,
    /// Who must be asked first (optional, up to 120 characters).
    #[serde(default)]
    pub consulted: Option<String>,
}

/// A standing meeting or board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Forum {
    /// Its name (up to 80 characters).
    pub name: String,
    /// How often, in words (up to 80 characters).
    pub cadence: String,
    /// What it is for (optional, up to 300 characters).
    #[serde(default)]
    pub purpose: Option<String>,
}

/// The fields of a governance framework.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Governance {
    /// One of [`DECISION_MODELS`].
    #[serde(default = "other")]
    pub decision_model: String,
    /// How often the plan is reviewed, in days (1 to 366).
    #[serde(default)]
    pub review_cadence_days: Option<i32>,
    /// How often it reports upward, in days (1 to 366).
    #[serde(default)]
    pub reporting_cadence_days: Option<i32>,
    /// Where things go when they cannot be settled.
    #[serde(default)]
    pub escalation_path: Vec<Escalation>,
    /// Who decides what.
    #[serde(default)]
    pub decision_rights: Vec<DecisionRight>,
    /// The standing meetings and boards.
    #[serde(default)]
    pub forums: Vec<Forum>,
}

fn other() -> String {
    "other".to_string()
}

fn len(s: &str) -> usize {
    s.trim().chars().count()
}

fn within(s: &str, max: usize) -> bool {
    (1..=max).contains(&len(s))
}

fn days(out: &mut Vec<String>, field: &str, v: Option<i32>, max: i32) {
    if v.is_some_and(|d| !(1..=max).contains(&d)) {
        out.push(format!("{field} is 1 to {max}"));
    }
}

fn text(out: &mut Vec<String>, field: &str, v: Option<&str>) {
    if v.is_some_and(|t| t.chars().count() > TEXT_MAX) {
        out.push(format!("{field} is longer than {TEXT_MAX} characters"));
    }
}

fn listed(out: &mut Vec<String>, field: &str, n: usize) {
    if n > LIST_MAX {
        out.push(format!("at most {LIST_MAX} {field}"));
    }
}

fn wow_problems(w: &WaysOfWorking) -> Vec<String> {
    let mut out = Vec::new();
    if !METHODOLOGIES.contains(&w.methodology.as_str()) {
        out.push(format!("methodology must be one of {METHODOLOGIES:?}"));
    }
    days(&mut out, "cadence_days", w.cadence_days, 366);
    days(&mut out, "wip_limit", w.wip_limit, 1000);
    text(
        &mut out,
        "definition_of_ready",
        w.definition_of_ready.as_deref(),
    );
    text(
        &mut out,
        "definition_of_done",
        w.definition_of_done.as_deref(),
    );
    listed(&mut out, "ceremonies", w.ceremonies.len());
    for (i, c) in w.ceremonies.iter().enumerate() {
        let at = |m: &str| format!("ceremony {}: {m}", i + 1);
        if !within(&c.name, 80) {
            out.push(at("name is 1 to 80 characters"));
        }
        if !within(&c.cadence, 80) {
            out.push(at("cadence is 1 to 80 characters"));
        }
        if c.purpose
            .as_deref()
            .is_some_and(|p| p.chars().count() > 300)
        {
            out.push(at("purpose is longer than 300 characters"));
        }
    }
    out
}

fn governance_problems(g: &Governance) -> Vec<String> {
    let mut out = Vec::new();
    if !DECISION_MODELS.contains(&g.decision_model.as_str()) {
        out.push(format!("decision_model must be one of {DECISION_MODELS:?}"));
    }
    days(&mut out, "review_cadence_days", g.review_cadence_days, 366);
    days(
        &mut out,
        "reporting_cadence_days",
        g.reporting_cadence_days,
        366,
    );
    listed(&mut out, "escalation_path steps", g.escalation_path.len());
    listed(&mut out, "decision_rights", g.decision_rights.len());
    listed(&mut out, "forums", g.forums.len());
    let mut levels: Vec<u32> = g.escalation_path.iter().map(|e| e.level).collect();
    levels.sort_unstable();
    if levels.windows(2).any(|w| w[0] == w[1]) {
        out.push("escalation_path levels must be distinct".to_string());
    }
    for (i, e) in g.escalation_path.iter().enumerate() {
        let at = |m: &str| format!("escalation step {}: {m}", i + 1);
        if e.level == 0 {
            out.push(at("level starts at 1"));
        }
        if !within(&e.body, 120) {
            out.push(at("body is 1 to 120 characters"));
        }
        if !within(&e.trigger, 300) {
            out.push(at("trigger is 1 to 300 characters"));
        }
    }
    for (i, d) in g.decision_rights.iter().enumerate() {
        let at = |m: &str| format!("decision right {}: {m}", i + 1);
        if !within(&d.decision, 120) {
            out.push(at("decision is 1 to 120 characters"));
        }
        if !within(&d.decides, 120) {
            out.push(at("decides is 1 to 120 characters"));
        }
        if d.consulted
            .as_deref()
            .is_some_and(|c| c.chars().count() > 120)
        {
            out.push(at("consulted is longer than 120 characters"));
        }
    }
    for (i, f) in g.forums.iter().enumerate() {
        let at = |m: &str| format!("forum {}: {m}", i + 1);
        if !within(&f.name, 80) {
            out.push(at("name is 1 to 80 characters"));
        }
        if !within(&f.cadence, 80) {
            out.push(at("cadence is 1 to 80 characters"));
        }
        if f.purpose
            .as_deref()
            .is_some_and(|p| p.chars().count() > 300)
        {
            out.push(at("purpose is longer than 300 characters"));
        }
    }
    out
}

/// Check a submitted record and return its canonical `body` (defaults filled in).
///
/// `fields` is everything in the request except `name`, `description` and `plan_pid`. Problems with the shape
/// (an unknown field, a wrong type) and with the values are all reported, not just the first.
///
/// # Errors
///
/// Every problem, one sentence each.
pub fn check(
    family: Family,
    name: &str,
    description: Option<&str>,
    fields: Map<String, Value>,
) -> Result<Value, Vec<String>> {
    let mut out = Vec::new();
    if !within(name, NAME_MAX) {
        out.push(format!("name is 1 to {NAME_MAX} characters"));
    }
    text(&mut out, "description", description);
    let body = match family {
        Family::WaysOfWorking => serde_json::from_value::<WaysOfWorking>(Value::Object(fields))
            .map(|w| {
                out.extend(wow_problems(&w));
                serde_json::to_value(w)
            }),
        Family::Governance => {
            serde_json::from_value::<Governance>(Value::Object(fields)).map(|g| {
                out.extend(governance_problems(&g));
                serde_json::to_value(g)
            })
        }
    };
    match body {
        Err(e) => {
            out.push(e.to_string());
            Err(out)
        }
        Ok(_) if !out.is_empty() => Err(out),
        Ok(value) => value.map_err(|e| vec![e.to_string()]),
    }
}

/// Where a plan's framework comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// The plan's own link.
    Own,
    /// The nearest ancestor's link.
    Inherited,
    /// Neither: nothing is declared.
    None,
}

/// The outcome of [`resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    /// Where it came from.
    pub source: Source,
    /// The framework followed, when there is one.
    pub framework: Option<Uuid>,
    /// The plan whose link it is (the plan itself for [`Source::Own`]).
    pub from_plan: Option<Uuid>,
}

/// Decide the framework for `plan`: its own link, else the first ancestor with one, else none. `ancestors` is
/// nearest first, each with that plan's link (if any).
#[must_use]
pub fn resolve(plan: Uuid, own: Option<Uuid>, ancestors: &[(Uuid, Option<Uuid>)]) -> Resolution {
    if let Some(w) = own {
        return Resolution {
            source: Source::Own,
            framework: Some(w),
            from_plan: Some(plan),
        };
    }
    ancestors
        .iter()
        .find_map(|(p, link)| link.map(|w| (*p, w)))
        .map_or(
            Resolution {
                source: Source::None,
                framework: None,
                from_plan: None,
            },
            |(p, w)| Resolution {
                source: Source::Inherited,
                framework: Some(w),
                from_plan: Some(p),
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fields(v: Value) -> Map<String, Value> {
        match v {
            Value::Object(m) => m,
            other => panic!("not an object: {other}"),
        }
    }

    #[test]
    fn a_good_record_of_each_family_gets_its_defaults_filled_in() {
        let w = check(
            Family::WaysOfWorking,
            "Scrum",
            None,
            fields(json!({ "methodology": "scrum", "cadence_days": 14 })),
        )
        .expect("valid");
        assert_eq!(w["methodology"], "scrum");
        assert_eq!(w["ceremonies"], json!([]));
        let g = check(Family::Governance, "Steering", None, fields(json!({}))).expect("valid");
        assert_eq!(g["decision_model"], "other", "defaults to other");
        assert_eq!(g["forums"], json!([]));
    }

    #[test]
    fn every_bad_field_of_a_ways_of_working_record_is_named() {
        let long = "x".repeat(TEXT_MAX + 1);
        let many: Vec<Value> = (0..=LIST_MAX)
            .map(|_| json!({ "name": "a", "cadence": "b" }))
            .collect();
        let e = check(
            Family::WaysOfWorking,
            " ",
            Some(&long),
            fields(json!({
                "methodology": "chaos", "cadence_days": 0, "wip_limit": 1001,
                "definition_of_ready": long, "definition_of_done": long, "ceremonies": many
            })),
        )
        .expect_err("invalid");
        for needle in [
            "name is",
            "description",
            "methodology",
            "cadence_days",
            "wip_limit",
            "definition_of_ready",
            "definition_of_done",
            "at most",
        ] {
            assert!(e.iter().any(|p| p.contains(needle)), "{needle}: {e:?}");
        }
        let c = check(Family::WaysOfWorking, "x", None, fields(json!({ "ceremonies": [{ "name": " ", "cadence": "d".repeat(81), "purpose": "p".repeat(301) }] }))).expect_err("invalid");
        for needle in [
            "ceremony 1: name",
            "ceremony 1: cadence",
            "ceremony 1: purpose",
        ] {
            assert!(c.iter().any(|p| p.contains(needle)), "{needle}: {c:?}");
        }
    }

    #[test]
    fn every_bad_field_of_a_governance_framework_is_named() {
        let e = check(
            Family::Governance,
            "x",
            None,
            fields(json!({
                "decision_model": "coin_toss", "review_cadence_days": 0, "reporting_cadence_days": 400,
                "escalation_path": [{ "level": 0, "body": "", "trigger": "t" }, { "level": 0, "body": "b", "trigger": "t" }],
                "decision_rights": [{ "decision": "", "decides": "d".repeat(121), "consulted": "c".repeat(121) }],
                "forums": [{ "name": "", "cadence": "", "purpose": "p".repeat(301) }]
            })),
        )
        .expect_err("invalid");
        for needle in [
            "decision_model",
            "review_cadence_days",
            "reporting_cadence_days",
            "levels must be distinct",
            "escalation step 1: level",
            "escalation step 1: body",
            "decision right 1: decision",
            "decision right 1: decides",
            "decision right 1: consulted",
            "forum 1: name",
            "forum 1: cadence",
            "forum 1: purpose",
        ] {
            assert!(e.iter().any(|p| p.contains(needle)), "{needle}: {e:?}");
        }
    }

    #[test]
    fn a_misspelt_or_foreign_field_is_an_error_not_silently_dropped() {
        let e = check(
            Family::WaysOfWorking,
            "x",
            None,
            fields(json!({ "methodolgy": "scrum" })),
        )
        .expect_err("unknown field");
        assert!(e.iter().any(|p| p.contains("methodolgy")), "{e:?}");
        assert!(
            check(
                Family::WaysOfWorking,
                "x",
                None,
                fields(json!({ "decision_model": "sponsor" }))
            )
            .is_err(),
            "a governance field on a ways of working"
        );
        assert!(
            check(
                Family::Governance,
                "x",
                None,
                fields(json!({ "wip_limit": 3 }))
            )
            .is_err()
        );
    }

    #[test]
    fn a_plan_follows_its_own_then_the_nearest_ancestors_then_nothing() {
        let (plan, parent, grand) = (Uuid::from_u128(1), Uuid::from_u128(2), Uuid::from_u128(3));
        let (mine, theirs, top) = (
            Uuid::from_u128(10),
            Uuid::from_u128(11),
            Uuid::from_u128(12),
        );
        let own = resolve(plan, Some(mine), &[(parent, Some(theirs))]);
        assert_eq!(
            (own.source, own.framework, own.from_plan),
            (Source::Own, Some(mine), Some(plan))
        );
        let near = resolve(plan, None, &[(parent, Some(theirs)), (grand, Some(top))]);
        assert_eq!(
            (near.source, near.framework, near.from_plan),
            (Source::Inherited, Some(theirs), Some(parent))
        );
        let far = resolve(plan, None, &[(parent, None), (grand, Some(top))]);
        assert_eq!(
            (far.framework, far.from_plan),
            (Some(top), Some(grand)),
            "skips an ancestor with none"
        );
        let none = resolve(plan, None, &[(parent, None), (grand, None)]);
        assert_eq!(
            (none.source, none.framework, none.from_plan),
            (Source::None, None, None)
        );
    }
}
