//! Per-plan feature modules (RM-7): the pure rules. DB-free, unit-tested.
//!
//! A plan exposes a **set of modules**; a plan-scoped route belonging to a
//! module that is off for that plan answers `404`. This is what keeps an
//! individual's personal plan down to tasks and time while a PMO plan shows
//! governance and finance — one service, two audiences.
//!
//! Rules that never bend:
//!
//! - **Disabling hides, never deletes.** Data written while a module was on
//!   is intact when it is switched back on.
//! - **No explicit set means the deployment default**, which is *all
//!   modules* unless `PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES` says
//!   otherwise — so a plan nobody configured behaves exactly as before.
//! - **Only plan-scoped paths are gated** (`/api/plans/{pid}/<segment>`).
//!   Portfolio-wide views (`/api/at-a-glance`, `/api/capacity`, …) read
//!   across plans and are not affected, and a path whose segment belongs to
//!   no module (the plan record itself, `modules`, `audit`, …) is always on.

use std::collections::BTreeSet;

use uuid::Uuid;

/// The environment variable naming the deployment default.
pub const DEFAULT_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES";

/// A module and the plan-scoped path segments it owns.
pub struct ModuleDef {
    /// Stable key (what `modules` lists).
    pub key: &'static str,
    /// The `/api/plans/{pid}/<segment>` segments this module owns.
    pub segments: &'static [&'static str],
}

/// Every module, in display order.
pub const MODULES: &[ModuleDef] = &[
    ModuleDef {
        key: "tasks",
        segments: &[
            "tasks",
            "components",
            "sprints",
            "burndown",
            "velocity",
            "standup",
            // The critical-path view of a plan's tasks (SC-1).
            "cpm",
            // Schedule baselines and variance against them (SC-3).
            "schedule-baselines",
            "schedule-variance",
            // The RACI matrix of a plan's tasks.
            "raci",
        ],
    },
    ModuleDef {
        key: "time",
        segments: &["time-entries", "effort", "time-analysis"],
    },
    ModuleDef {
        key: "risks",
        segments: &["risks", "issues"],
    },
    ModuleDef {
        key: "financials",
        segments: &[
            "budget-lines",
            "budget-baselines",
            "financials",
            // Earned value (SC-4).
            "earned-value",
            "earned-value-settings",
        ],
    },
    ModuleDef {
        key: "milestones",
        segments: &["milestones", "schedule", "calendar"],
    },
    ModuleDef {
        key: "governance",
        segments: &[
            "gate-reviews",
            "governance",
            "tpc",
            "controls",
            "change-requests",
            "director-pack",
        ],
    },
];

/// The modules a **personal** plan (one person's own work) exposes.
pub const PERSONAL: &[&str] = &["tasks", "time"];

/// Every module key.
#[must_use]
pub fn all_keys() -> BTreeSet<String> {
    MODULES.iter().map(|m| m.key.to_string()).collect()
}

/// The module that owns a plan-scoped path segment, if any.
#[must_use]
pub fn module_for_segment(segment: &str) -> Option<&'static str> {
    MODULES
        .iter()
        .find(|m| m.segments.contains(&segment))
        .map(|m| m.key)
}

/// Split `/api/plans/{uuid}/{segment}[/...]` into the plan pid and the
/// first segment after it. Anything else (including the bare plan record
/// and `/api/plans/{uuid}`) is `None` — never gated.
#[must_use]
pub fn parse_plan_path(path: &str) -> Option<(Uuid, &str)> {
    let rest = path.strip_prefix("/api/plans/")?;
    let mut parts = rest.split('/');
    let pid = Uuid::parse_str(parts.next()?).ok()?;
    let segment = parts.next().filter(|s| !s.is_empty())?;
    Some((pid, segment))
}

/// Every problem with a requested module set (unknown keys, by name).
#[must_use]
pub fn problems(requested: &[String]) -> Vec<String> {
    let known = all_keys();
    requested
        .iter()
        .filter(|k| !known.contains(k.as_str()))
        .map(|k| format!("unknown module {k:?} (allowed: {known:?})"))
        .collect()
}

/// Parse the deployment default: unset, blank or `all` mean every module;
/// otherwise a comma-separated subset. An unknown name is an error rather
/// than a silently smaller default.
///
/// # Errors
///
/// A message naming each unknown module.
pub fn parse_default(raw: Option<&str>) -> Result<BTreeSet<String>, String> {
    let raw = raw.map(str::trim).unwrap_or_default();
    if raw.is_empty() || raw.eq_ignore_ascii_case("all") {
        return Ok(all_keys());
    }
    let wanted: Vec<String> = raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let bad = problems(&wanted);
    if bad.is_empty() {
        Ok(wanted.into_iter().collect())
    } else {
        Err(bad.join("; "))
    }
}

/// The deployment default, read from the environment. A malformed value
/// falls back to *all modules* (fail open on presentation, never hiding
/// features because of a typo); the guard logs nothing here because this is
/// called per request — [`parse_default`] is the place to validate at boot.
#[must_use]
pub fn deployment_default() -> BTreeSet<String> {
    parse_default(std::env::var(DEFAULT_ENV).ok().as_deref()).unwrap_or_else(|_| all_keys())
}

/// The modules in force for a plan: its own set if it has one, else the
/// deployment default.
#[must_use]
pub fn effective(
    explicit: Option<&BTreeSet<String>>,
    default: &BTreeSet<String>,
) -> BTreeSet<String> {
    explicit.cloned().unwrap_or_else(|| default.clone())
}

/// Whether a plan-scoped path segment is reachable under `modules`. A
/// segment no module owns is always reachable.
#[must_use]
pub fn segment_enabled(modules: &BTreeSet<String>, segment: &str) -> bool {
    module_for_segment(segment).is_none_or(|m| modules.contains(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(keys: &[&str]) -> BTreeSet<String> {
        keys.iter().map(|k| (*k).to_string()).collect()
    }

    const PID: &str = "3f2c6b7e-0000-4000-8000-000000000001";

    #[test]
    fn every_segment_belongs_to_at_most_one_module() {
        let mut seen = std::collections::HashSet::new();
        for m in MODULES {
            for s in m.segments {
                assert!(seen.insert(*s), "segment {s} owned by two modules");
            }
        }
    }

    #[test]
    fn the_personal_preset_names_only_real_modules() {
        let personal: Vec<String> = PERSONAL.iter().map(|s| (*s).to_string()).collect();
        assert!(problems(&personal).is_empty());
    }

    #[test]
    fn plan_paths_parse_only_when_a_segment_follows_a_uuid() {
        assert_eq!(
            parse_plan_path(&format!("/api/plans/{PID}/risks")),
            Some((Uuid::parse_str(PID).unwrap(), "risks"))
        );
        assert_eq!(
            parse_plan_path(&format!("/api/plans/{PID}/risks/abc")).map(|(_, s)| s),
            Some("risks")
        );
        assert_eq!(
            parse_plan_path(&format!("/api/plans/{PID}")),
            None,
            "the record itself"
        );
        assert_eq!(parse_plan_path(&format!("/api/plans/{PID}/")), None);
        assert_eq!(parse_plan_path("/api/plans/search"), None, "not a uuid");
        assert_eq!(parse_plan_path("/api/capacity"), None);
        assert_eq!(parse_plan_path("/api/plans"), None);
    }

    #[test]
    fn a_disabled_module_hides_its_segments_and_nothing_else() {
        let m = set(&["tasks", "time"]);
        assert!(segment_enabled(&m, "tasks"));
        assert!(segment_enabled(&m, "time-entries"));
        assert!(!segment_enabled(&m, "risks"));
        assert!(!segment_enabled(&m, "budget-lines"));
        assert!(
            segment_enabled(&m, "audit"),
            "unowned segments are always on"
        );
        assert!(
            segment_enabled(&m, "modules"),
            "the toggle itself can never be hidden"
        );
    }

    #[test]
    fn the_default_is_all_modules_unless_the_deployment_narrows_it() {
        assert_eq!(parse_default(None).unwrap(), all_keys());
        assert_eq!(parse_default(Some("  ")).unwrap(), all_keys());
        assert_eq!(parse_default(Some("ALL")).unwrap(), all_keys());
        assert_eq!(
            parse_default(Some("tasks, time")).unwrap(),
            set(&["tasks", "time"])
        );
        let err = parse_default(Some("tasks,nope")).unwrap_err();
        assert!(
            err.contains("nope"),
            "an unknown name is named, not dropped"
        );
    }

    #[test]
    fn an_explicit_set_beats_the_default_and_none_falls_back() {
        let default = all_keys();
        let own = set(&["tasks"]);
        assert_eq!(effective(Some(&own), &default), own);
        assert_eq!(effective(None, &default), default);
        assert_eq!(
            effective(Some(&BTreeSet::new()), &default),
            BTreeSet::new(),
            "an empty set is a real choice"
        );
    }

    #[test]
    fn unknown_modules_are_each_reported() {
        assert_eq!(problems(&["tasks".into(), "x".into(), "y".into()]).len(), 2);
    }
}
