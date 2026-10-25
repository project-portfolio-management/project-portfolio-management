//! **WSJF** and **RICE** — two more ways to rank a plan, beside the Smart Score (NC-6). Pure and DB-free.
//!
//! Unlike the Smart Score, which is derived from rows the service already holds, these two need numbers a
//! person estimates, so they are stored as [`ModelInputs`] and scored here. The same honesty rules apply:
//!
//! - **Absent input is absent, not zero.** A model needs all of its inputs. With any missing, the score is
//!   `None` and every missing input is named with the reason it is needed; nothing is guessed or defaulted.
//! - **Every component is explained.** The result carries each component's value and the sentence that says
//!   what it means, and the formula, so a number that ranks somebody's work can say how it was reached.
//! - **Nothing is stored but the inputs.** The score is derived on read.
//!
//! **WSJF** (weighted shortest job first, as used by SAFe): `(business value + time criticality + risk
//! reduction or opportunity enablement) / job size`, each on a relative 1 to 20 scale. **RICE**:
//! `reach × impact × confidence / effort`; reach is people (or events) per period, impact is one of
//! 0.25, 0.5, 1, 2 or 3, confidence is 0 to 100 percent, and effort is person-months above zero.

use serde::{Deserialize, Serialize};

/// The impact values RICE allows: minimal, low, medium, high, massive.
pub const IMPACTS: [f64; 5] = [0.25, 0.5, 1.0, 2.0, 3.0];

/// Lowest and highest value on a WSJF relative scale.
pub const WSJF_MIN: i16 = 1;
/// Highest value on a WSJF relative scale.
pub const WSJF_MAX: i16 = 20;

/// The estimated inputs for both models. Every one is optional.
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ModelInputs {
    /// WSJF: relative value to the user or business, `1..=20`.
    #[serde(default)]
    pub business_value: Option<i16>,
    /// WSJF: how much the value decays with time, `1..=20`.
    #[serde(default)]
    pub time_criticality: Option<i16>,
    /// WSJF: risk reduction or opportunity enablement, `1..=20`.
    #[serde(default)]
    pub risk_reduction: Option<i16>,
    /// WSJF: relative size of the job, `1..=20`.
    #[serde(default)]
    pub job_size: Option<i16>,
    /// RICE: how many people or events it reaches in a period (a whole number, 0 or more).
    #[serde(default)]
    pub reach: Option<i32>,
    /// RICE: 0.25, 0.5, 1, 2 or 3.
    #[serde(default)]
    pub impact: Option<f64>,
    /// RICE: how sure the estimates are, `0..=100` percent.
    #[serde(default)]
    pub confidence_percent: Option<i16>,
    /// RICE: person-months of work, above 0.
    #[serde(default)]
    pub effort: Option<f64>,
}

/// One component of a model's score.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Component {
    /// The component's name.
    pub name: &'static str,
    /// Its value, or `None` when the input is missing.
    pub value: Option<f64>,
    /// What it means and how it is used.
    pub explanation: &'static str,
}

/// An input a model needs and does not have.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Missing {
    /// The input's name.
    pub input: &'static str,
    /// Why the model needs it.
    pub reason: &'static str,
}

/// A model's score with its derivation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelScore {
    /// `wsjf` or `rice`.
    pub model: &'static str,
    /// The score, or `None` when any input is missing.
    pub score: Option<f64>,
    /// The formula in words.
    pub formula: &'static str,
    /// Every component, present or not, in formula order.
    pub components: Vec<Component>,
    /// The inputs the score is waiting for, each with its reason.
    pub missing: Vec<Missing>,
}

/// Check the inputs; every problem is named.
///
/// # Errors
///
/// One sentence per input out of range.
pub fn validate(inputs: &ModelInputs) -> Result<(), Vec<String>> {
    let mut problems = Vec::new();
    for (name, v) in [
        ("business_value", inputs.business_value),
        ("time_criticality", inputs.time_criticality),
        ("risk_reduction", inputs.risk_reduction),
        ("job_size", inputs.job_size),
    ] {
        if v.is_some_and(|value| !(WSJF_MIN..=WSJF_MAX).contains(&value)) {
            problems.push(format!("{name} is {WSJF_MIN} to {WSJF_MAX}"));
        }
    }
    if inputs.reach.is_some_and(|reach| reach < 0) {
        problems.push("reach cannot be negative".to_string());
    }
    if inputs
        .impact
        .is_some_and(|impact| !IMPACTS.contains(&impact))
    {
        problems.push("impact is one of 0.25, 0.5, 1, 2 or 3".to_string());
    }
    if inputs
        .confidence_percent
        .is_some_and(|pct| !(0..=100).contains(&pct))
    {
        problems.push("confidence_percent is 0 to 100".to_string());
    }
    if inputs
        .effort
        .is_some_and(|months| !(months > 0.0 && months.is_finite()))
    {
        problems.push("effort is above 0 (person-months)".to_string());
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

fn comp(name: &'static str, value: Option<f64>, explanation: &'static str) -> Component {
    Component {
        name,
        value,
        explanation,
    }
}

/// WSJF: `(business value + time criticality + risk reduction) / job size`.
#[must_use]
pub fn wsjf(inputs: &ModelInputs) -> ModelScore {
    let widen = |v: Option<i16>| v.map(f64::from);
    let (bv, tc, rr, js) = (
        widen(inputs.business_value),
        widen(inputs.time_criticality),
        widen(inputs.risk_reduction),
        widen(inputs.job_size),
    );
    let cost_of_delay = match (bv, tc, rr) {
        (Some(value), Some(time), Some(risk)) => Some(value + time + risk),
        _ => None,
    };
    let mut missing = Vec::new();
    for (input, v, reason) in [
        (
            "business_value",
            bv,
            "it is the first part of the cost of delay",
        ),
        (
            "time_criticality",
            tc,
            "it is the second part of the cost of delay",
        ),
        (
            "risk_reduction",
            rr,
            "it is the third part of the cost of delay",
        ),
        (
            "job_size",
            js,
            "it is the divisor: without a size there is no value per unit of work",
        ),
    ] {
        if v.is_none() {
            missing.push(Missing { input, reason });
        }
    }
    let score = match (cost_of_delay, js) {
        (Some(delay), Some(size)) => Some(delay / size),
        _ => None,
    };
    ModelScore {
        model: "wsjf",
        score,
        formula: "(business value + time criticality + risk reduction or opportunity enablement) / job size",
        components: vec![
            comp(
                "business_value",
                bv,
                "relative value to the user or the business, 1 to 20",
            ),
            comp(
                "time_criticality",
                tc,
                "how fast the value decays if it is late, 1 to 20",
            ),
            comp(
                "risk_reduction",
                rr,
                "risk it removes or opportunity it opens, 1 to 20",
            ),
            comp(
                "cost_of_delay",
                cost_of_delay,
                "the sum of the three above: what a delay costs",
            ),
            comp(
                "job_size",
                js,
                "relative size of the work, 1 to 20; the divisor",
            ),
        ],
        missing,
    }
}

/// RICE: `reach × impact × confidence / effort`.
#[must_use]
pub fn rice(inputs: &ModelInputs) -> ModelScore {
    let reach = inputs.reach.map(f64::from);
    let impact = inputs.impact;
    let confidence = inputs.confidence_percent.map(|pct| f64::from(pct) / 100.0);
    let effort = inputs.effort;
    let mut missing = Vec::new();
    for (input, v, reason) in [
        (
            "reach",
            reach,
            "it says how many people the work touches in a period",
        ),
        (
            "impact",
            impact,
            "it says how much it matters to each of them",
        ),
        (
            "confidence_percent",
            confidence,
            "it discounts the estimates by how sure they are",
        ),
        (
            "effort",
            effort,
            "it is the divisor: without effort there is no value per unit of work",
        ),
    ] {
        if v.is_none() {
            missing.push(Missing { input, reason });
        }
    }
    let score = match (reach, impact, confidence, effort) {
        (Some(people), Some(size), Some(sure), Some(months)) => Some(people * size * sure / months),
        _ => None,
    };
    ModelScore {
        model: "rice",
        score,
        formula: "reach x impact x confidence / effort",
        components: vec![
            comp("reach", reach, "people or events reached in a period"),
            comp(
                "impact",
                impact,
                "0.25 minimal, 0.5 low, 1 medium, 2 high, 3 massive",
            ),
            comp(
                "confidence",
                confidence,
                "how sure the estimates are, as a fraction of 1",
            ),
            comp("effort", effort, "person-months of work; the divisor"),
        ],
        missing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full() -> ModelInputs {
        ModelInputs {
            business_value: Some(8),
            time_criticality: Some(5),
            risk_reduction: Some(3),
            job_size: Some(4),
            reach: Some(2000),
            impact: Some(2.0),
            confidence_percent: Some(80),
            effort: Some(4.0),
        }
    }

    #[test]
    fn wsjf_is_the_cost_of_delay_over_the_job_size() {
        let s = wsjf(&full());
        assert_eq!(s.score, Some(4.0), "(8 + 5 + 3) / 4");
        let cod = s
            .components
            .iter()
            .find(|c| c.name == "cost_of_delay")
            .unwrap();
        assert_eq!(cod.value, Some(16.0));
        assert!(s.missing.is_empty());
    }

    #[test]
    fn a_smaller_job_scores_higher_with_the_same_delay_cost() {
        let mut small = full();
        small.job_size = Some(2);
        assert!(wsjf(&small).score > wsjf(&full()).score);
    }

    #[test]
    fn rice_is_reach_times_impact_times_confidence_over_effort() {
        let s = rice(&full());
        assert_eq!(s.score, Some(800.0), "2000 x 2 x 0.8 / 4");
        assert!(s.missing.is_empty());
    }

    #[test]
    fn zero_confidence_scores_zero_but_a_missing_one_scores_nothing() {
        let mut i = full();
        i.confidence_percent = Some(0);
        assert_eq!(
            rice(&i).score,
            Some(0.0),
            "zero confidence is a stated answer"
        );
        i.confidence_percent = None;
        let s = rice(&i);
        assert_eq!(s.score, None, "an absent confidence is not zero");
        assert_eq!(s.missing.len(), 1);
        assert_eq!(s.missing[0].input, "confidence_percent");
        assert!(!s.missing[0].reason.is_empty());
    }

    #[test]
    fn every_missing_input_is_named_with_its_reason() {
        let s = wsjf(&ModelInputs {
            business_value: Some(3),
            ..ModelInputs::default()
        });
        assert_eq!(s.score, None);
        let names: Vec<&str> = s.missing.iter().map(|m| m.input).collect();
        assert_eq!(names, ["time_criticality", "risk_reduction", "job_size"]);
        let c = s
            .components
            .iter()
            .find(|c| c.name == "cost_of_delay")
            .unwrap();
        assert_eq!(
            c.value, None,
            "a partial cost of delay is not shown as a total"
        );
    }

    #[test]
    fn a_model_is_unaffected_by_the_other_models_inputs() {
        let only_rice = ModelInputs {
            reach: Some(10),
            impact: Some(1.0),
            confidence_percent: Some(100),
            effort: Some(1.0),
            ..ModelInputs::default()
        };
        assert_eq!(rice(&only_rice).score, Some(10.0));
        assert_eq!(wsjf(&only_rice).score, None);
    }

    #[test]
    fn validation_names_every_problem() {
        assert!(validate(&full()).is_ok());
        let bad = ModelInputs {
            business_value: Some(0),
            job_size: Some(21),
            reach: Some(-1),
            impact: Some(1.5),
            confidence_percent: Some(101),
            effort: Some(0.0),
            ..ModelInputs::default()
        };
        let problems = validate(&bad).unwrap_err();
        assert_eq!(problems.len(), 6, "{problems:?}");
        assert!(
            validate(&ModelInputs::default()).is_ok(),
            "no input at all is allowed"
        );
        let nan = ModelInputs {
            effort: Some(f64::NAN),
            ..ModelInputs::default()
        };
        assert!(validate(&nan).is_err());
    }
}
