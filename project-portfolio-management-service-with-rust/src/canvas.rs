//! Strategy canvases (LT-1): the built-in templates and the pure rules
//! around them. DB-free and unit-tested.
//!
//! A **template** is nothing but an ordered list of boxes. One generic
//! `canvases` / `canvas_items` table pair serves every type, so adding a
//! canvas type means adding a [`Template`] here — never a migration.
//!
//! The box sets are the conventional, widely published ones (Lean Canvas,
//! Business Model Canvas, SWOT, empathy map, logic model). The `risk` and
//! `goal` templates are this service's own design — a small box set sized
//! to the risk register and objectives it promotes into — and are not
//! claimed to match any other tool's.

use serde::Serialize;

/// One box of a canvas template.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct BoxDef {
    /// Stable machine key (what an item's `box` names).
    pub key: &'static str,
    /// Human label.
    pub title: &'static str,
}

/// A canvas template: a key, a title, and an ordered box list.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Template {
    /// Stable machine key (what a canvas's `template` names).
    pub key: &'static str,
    /// Human label.
    pub title: &'static str,
    /// Boxes in display order.
    pub boxes: &'static [BoxDef],
}

macro_rules! boxes {
    ($(($k:literal, $t:literal)),+ $(,)?) => {
        &[$(BoxDef { key: $k, title: $t }),+]
    };
}

/// Every built-in template, in display order.
pub const TEMPLATES: &[Template] = &[
    Template {
        key: "lean",
        title: "Lean Canvas",
        boxes: boxes![
            ("problem", "Problem"),
            ("solution", "Solution"),
            ("key_metrics", "Key metrics"),
            ("unique_value_proposition", "Unique value proposition"),
            ("unfair_advantage", "Unfair advantage"),
            ("channels", "Channels"),
            ("customer_segments", "Customer segments"),
            ("cost_structure", "Cost structure"),
            ("revenue_streams", "Revenue streams"),
        ],
    },
    Template {
        key: "business_model",
        title: "Business Model Canvas",
        boxes: boxes![
            ("key_partners", "Key partners"),
            ("key_activities", "Key activities"),
            ("key_resources", "Key resources"),
            ("value_propositions", "Value propositions"),
            ("customer_relationships", "Customer relationships"),
            ("channels", "Channels"),
            ("customer_segments", "Customer segments"),
            ("cost_structure", "Cost structure"),
            ("revenue_streams", "Revenue streams"),
        ],
    },
    Template {
        key: "swot",
        title: "SWOT analysis",
        boxes: boxes![
            ("strengths", "Strengths"),
            ("weaknesses", "Weaknesses"),
            ("opportunities", "Opportunities"),
            ("threats", "Threats"),
        ],
    },
    Template {
        key: "empathy",
        title: "Empathy map",
        boxes: boxes![
            ("says", "Says"),
            ("thinks", "Thinks"),
            ("does", "Does"),
            ("feels", "Feels"),
            ("pains", "Pains"),
            ("gains", "Gains"),
        ],
    },
    Template {
        key: "logic_model",
        title: "Logic model",
        boxes: boxes![
            ("inputs", "Inputs"),
            ("activities", "Activities"),
            ("outputs", "Outputs"),
            ("outcomes", "Outcomes"),
            ("impact", "Impact"),
        ],
    },
    Template {
        key: "risk",
        title: "Risk canvas",
        boxes: boxes![
            ("threats", "Threats"),
            ("vulnerabilities", "Vulnerabilities"),
            ("existing_controls", "Existing controls"),
            ("residual_risks", "Residual risks"),
        ],
    },
    Template {
        key: "goal",
        title: "Goal canvas",
        boxes: boxes![
            ("goal", "Goal"),
            ("success_measures", "Success measures"),
            ("actions", "Actions"),
            ("obstacles", "Obstacles"),
            ("resources", "Resources"),
        ],
    },
];

/// Cap on one item's body (a note, not a document).
pub const MAX_BODY_LEN: usize = 2_000;
/// Cap on a canvas title.
pub const MAX_TITLE_LEN: usize = 200;
/// Cap on items per canvas, so one canvas cannot grow without bound.
pub const MAX_ITEMS: usize = 500;

/// Look a template up by key.
#[must_use]
pub fn template(key: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.key == key)
}

/// Whether `box_key` is a box of `template_key`.
#[must_use]
pub fn valid_box(template_key: &str, box_key: &str) -> bool {
    template(template_key).is_some_and(|t| t.boxes.iter().any(|b| b.key == box_key))
}

/// Problems with a new canvas's shape (every one, not just the first).
#[must_use]
pub fn canvas_problems(template_key: &str, title: &str) -> Vec<String> {
    let mut out = Vec::new();
    if template(template_key).is_none() {
        let keys: Vec<&str> = TEMPLATES.iter().map(|t| t.key).collect();
        out.push(format!(
            "unknown template {template_key:?} (allowed: {keys:?})"
        ));
    }
    if title.trim().is_empty() {
        out.push("title is required".to_string());
    } else if title.chars().count() > MAX_TITLE_LEN {
        out.push(format!("title: exceeds {MAX_TITLE_LEN} characters"));
    }
    out
}

/// Problems with an item's shape against its canvas's template.
#[must_use]
pub fn item_problems(template_key: &str, box_key: &str, body: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !valid_box(template_key, box_key) {
        let allowed: Vec<&str> = template(template_key)
            .map(|t| t.boxes.iter().map(|b| b.key).collect())
            .unwrap_or_default();
        out.push(format!(
            "unknown box {box_key:?} for template {template_key:?} (allowed: {allowed:?})"
        ));
    }
    if body.trim().is_empty() {
        out.push("body is required".to_string());
    } else if body.chars().count() > MAX_BODY_LEN {
        out.push(format!("body: exceeds {MAX_BODY_LEN} characters"));
    }
    out
}

/// Derive a short title from an item's body: its first non-blank line,
/// trimmed and cut to `max` characters on a character boundary.
#[must_use]
pub fn title_from_body(body: &str, max: usize) -> String {
    body.lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or_default()
        .chars()
        .take(max)
        .collect()
}

/// One box of a rendered canvas: the definition plus its item count.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BoxSummary {
    /// Box key.
    pub key: &'static str,
    /// Box label.
    pub title: &'static str,
    /// Live items in the box.
    pub count: usize,
}

/// Count items per box. **Every box of the template is reported, even at
/// zero** — an empty box is itself the finding on a canvas ("we have no
/// answer for Unfair advantage"), so it must never silently disappear.
#[must_use]
pub fn summarize<'a>(
    template_key: &str,
    item_boxes: impl Iterator<Item = &'a str>,
) -> Vec<BoxSummary> {
    let Some(t) = template(template_key) else {
        return Vec::new();
    };
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for b in item_boxes {
        *counts.entry(b).or_default() += 1;
    }
    t.boxes
        .iter()
        .map(|b| BoxSummary {
            key: b.key,
            title: b.title,
            count: counts.get(b.key).copied().unwrap_or(0),
        })
        .collect()
}

/// What an item may be promoted into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromoteTarget {
    /// An idea on the idea board.
    Idea,
    /// A risk on a plan's register (needs a plan-scoped canvas).
    Risk,
    /// An objective in the OKR registry.
    Objective,
}

impl PromoteTarget {
    /// Parse the wire token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "idea" => Some(Self::Idea),
            "risk" => Some(Self::Risk),
            "objective" => Some(Self::Objective),
            _ => None,
        }
    }

    /// The wire token, as stored in `promoted_to_kind`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idea => "idea",
            Self::Risk => "risk",
            Self::Objective => "objective",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_has_unique_nonempty_boxes() {
        let mut keys = std::collections::HashSet::new();
        for t in TEMPLATES {
            assert!(keys.insert(t.key), "duplicate template {}", t.key);
            assert!(!t.boxes.is_empty(), "{} has no boxes", t.key);
            let mut seen = std::collections::HashSet::new();
            for b in t.boxes {
                assert!(seen.insert(b.key), "{}: duplicate box {}", t.key, b.key);
                assert!(!b.title.is_empty());
            }
        }
    }

    #[test]
    fn the_published_box_counts_hold() {
        let n = |k: &str| template(k).unwrap().boxes.len();
        assert_eq!(n("lean"), 9);
        assert_eq!(n("business_model"), 9);
        assert_eq!(n("swot"), 4);
        assert_eq!(n("empathy"), 6);
        assert_eq!(n("logic_model"), 5);
    }

    #[test]
    fn an_unknown_box_or_template_is_refused() {
        assert!(valid_box("swot", "strengths"));
        assert!(!valid_box("swot", "problem"));
        assert!(!valid_box("nope", "strengths"));
        assert!(!item_problems("swot", "problem", "x").is_empty());
        assert!(!canvas_problems("nope", "t").is_empty());
    }

    #[test]
    fn shape_problems_are_all_reported() {
        assert_eq!(canvas_problems("nope", "  ").len(), 2);
        assert_eq!(item_problems("swot", "bad", " ").len(), 2);
        assert!(item_problems("swot", "threats", "ok").is_empty());
        assert!(!item_problems("swot", "threats", &"x".repeat(MAX_BODY_LEN + 1)).is_empty());
    }

    #[test]
    fn an_empty_box_is_reported_at_zero() {
        let s = summarize("swot", ["strengths", "strengths", "threats"].into_iter());
        assert_eq!(s.len(), 4, "every box present");
        assert_eq!(s[0].count, 2);
        assert_eq!(s[1].count, 0, "weaknesses reported empty, not omitted");
        assert_eq!(s[3].count, 1);
        assert!(summarize("nope", std::iter::empty()).is_empty());
    }

    #[test]
    fn title_takes_the_first_nonblank_line_on_a_char_boundary() {
        assert_eq!(title_from_body("\n\n  Ship it  \nmore", 50), "Ship it");
        assert_eq!(title_from_body("héllo wörld", 5), "héllo");
        assert_eq!(title_from_body("   ", 5), "");
    }

    #[test]
    fn promote_targets_round_trip() {
        for t in [
            PromoteTarget::Idea,
            PromoteTarget::Risk,
            PromoteTarget::Objective,
        ] {
            assert_eq!(PromoteTarget::parse(t.as_str()), Some(t));
        }
        assert_eq!(PromoteTarget::parse("proposal"), None);
    }
}
