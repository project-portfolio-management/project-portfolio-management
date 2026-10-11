//! Register completeness (EX-1). Pure rules; the database work is in
//! [`crate::controllers::register`].
//!
//! A programme's register is complete when ten things are on record: who is responsible, the partners (where
//! the delivery model has any), the stage, a schedule baseline, a target date, a budget, a funding source for
//! each budget line, a delivery-confidence assessment, risks and dependencies. The score is the share of the
//! applicable ones that are present, and **each missing field is named**: a score never stands alone.
//!
//! The deployment may turn on "no register, no money": a budget-line release or a firm allocation on an
//! incomplete programme is then refused, naming the fields. It is off by default; with it off the action
//! succeeds and the warning is audited.

#![allow(missing_docs)]

use serde::Serialize;

/// The fields, in the order they are listed.
pub const FIELDS: [&str; 10] = [
    "sro",
    "partners",
    "stage",
    "baseline",
    "target_date",
    "budget",
    "funding_source",
    "confidence",
    "top_risks",
    "dependencies",
];

/// What is on record for a programme (and the plans under it).
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Facts {
    /// A lead is recorded on the programme.
    pub sro: bool,
    /// Partners are required (the delivery model has partners) and none is on record.
    pub partners_required: bool,
    pub has_partners: bool,
    pub stage: bool,
    pub baseline: bool,
    pub target_date: bool,
    /// Budget lines exist.
    pub budget: bool,
    /// Every budget line has a funding source; `None` when there is no budget line.
    pub all_funded: Option<bool>,
    pub confidence: bool,
    pub risks: bool,
    pub dependencies: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Assessment {
    /// Fields that apply.
    pub applicable: usize,
    pub present: usize,
    /// Present over applicable, in basis points.
    pub score_bp: u32,
    /// The fields that apply and are missing, by name.
    pub missing: Vec<&'static str>,
    /// The fields that do not apply, by name, with the reason in the answer.
    pub not_applicable: Vec<&'static str>,
}

impl Assessment {
    #[must_use]
    pub fn complete(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Assess one programme.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn assess(f: &Facts) -> Assessment {
    let mut missing = Vec::new();
    let mut not_applicable = Vec::new();
    let mut present = 0usize;
    let mut applicable = 0usize;
    let mut check = |name: &'static str, applies: bool, ok: bool| {
        if !applies {
            not_applicable.push(name);
            return;
        }
        applicable += 1;
        if ok {
            present += 1;
        } else {
            missing.push(name);
        }
    };
    check("sro", true, f.sro);
    check("partners", f.partners_required, f.has_partners);
    check("stage", true, f.stage);
    check("baseline", true, f.baseline);
    check("target_date", true, f.target_date);
    check("budget", true, f.budget);
    // Funding sources can only be missing where there is a budget line to fund.
    check("funding_source", f.all_funded.is_some(), f.all_funded.unwrap_or(false));
    check("confidence", true, f.confidence);
    check("top_risks", true, f.risks);
    check("dependencies", true, f.dependencies);
    let score_bp = if applicable == 0 { 10_000 } else { ((present as u64 * 10_000 + applicable as u64 / 2) / applicable as u64) as u32 };
    Assessment { applicable, present, score_bp, missing, not_applicable }
}

/// Whether the deployment refuses actions on an incomplete programme: `PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED`
/// is `on`, `1`, `true` or `yes`. Off by default.
#[must_use]
pub fn enforced() -> bool {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED")
        .is_ok_and(|v| matches!(v.trim().to_lowercase().as_str(), "on" | "1" | "true" | "yes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full() -> Facts {
        Facts {
            sro: true,
            partners_required: true,
            has_partners: true,
            stage: true,
            baseline: true,
            target_date: true,
            budget: true,
            all_funded: Some(true),
            confidence: true,
            risks: true,
            dependencies: true,
        }
    }

    #[test]
    fn a_full_register_is_complete_with_nothing_missing() {
        let a = assess(&full());
        assert!(a.complete());
        assert_eq!((a.applicable, a.present, a.score_bp), (10, 10, 10_000));
    }

    #[test]
    fn each_missing_field_is_named_and_the_score_is_the_share_present() {
        let a = assess(&Facts { sro: false, baseline: false, risks: false, ..full() });
        assert_eq!(a.missing, ["sro", "baseline", "top_risks"]);
        assert_eq!((a.present, a.applicable, a.score_bp), (7, 10, 7_000));
    }

    #[test]
    fn partners_apply_only_where_the_delivery_model_has_them() {
        let a = assess(&Facts { partners_required: false, has_partners: false, ..full() });
        assert!(a.complete() && a.not_applicable == ["partners"] && a.applicable == 9);
        let b = assess(&Facts { has_partners: false, ..full() });
        assert_eq!(b.missing, ["partners"]);
    }

    #[test]
    fn a_funding_source_is_asked_for_only_where_there_is_a_budget() {
        let none = assess(&Facts { budget: false, all_funded: None, ..full() });
        assert_eq!(none.missing, ["budget"], "no budget line, so only the budget is missing");
        assert_eq!(none.not_applicable, ["funding_source"]);
        let unfunded = assess(&Facts { all_funded: Some(false), ..full() });
        assert_eq!(unfunded.missing, ["funding_source"]);
    }

    #[test]
    fn an_empty_register_names_all_ten() {
        let a = assess(&Facts { partners_required: true, ..Facts::default() });
        assert_eq!(a.missing.len(), 9, "funding_source does not apply with no budget");
        assert_eq!(a.score_bp, 0);
    }
}
