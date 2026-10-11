//! Watermelon measures (EX-9): green on the outside, red inside. Pure.
//!
//! Computed from **stored pack bodies only**: the body being drafted and the previous issued pack's body.
//! Nothing is re-derived from live records, so a measure can be checked against the packs that were read.
//! A measure whose inputs are missing is `null` with the reason; absence is never zero.
//!
//! * **self against independent**: programmes whose latest independent assessment disagrees with their own
//!   reading (the map already says so; this counts and names them);
//! * **late discovery**: a programme reported at amber-red or red this pack whose previous issued pack had
//!   it at green. It went from nothing to a crisis with no amber on the way;
//! * **milestones met with sign-off**: the met share of those delivered (EX-4);
//! * **red without a decision**: programmes at red whose exception has no recorded decision in the period
//!   and which are not paused or stopped.

use serde_json::{Value, json};

fn value_of(p: &Value) -> &str {
    p["confidence"]["value"].as_str().unwrap_or("not_assessed")
}

fn programmes(body: &Value) -> Vec<&Value> {
    body["portfolio_map"]["groups"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|g| g["programmes"].as_array().into_iter().flatten())
        .collect()
}

/// The measures for `body`, against the previous issued pack's body when there is one.
#[must_use]
pub fn measures(body: &Value, previous: Option<&Value>) -> Value {
    let progs = programmes(body);

    let gap = if progs
        .iter()
        .any(|p| p["disagreement"]["independent_vs_self"].is_boolean())
        && progs
            .iter()
            .any(|p| p["confidence"]["kind"] == "independent")
    {
        let names: Vec<&str> = progs
            .iter()
            .filter(|p| p["disagreement"]["independent_vs_self"] == true)
            .filter_map(|p| p["name"].as_str())
            .collect();
        json!({ "count": names.len(), "programmes": names })
    } else {
        Value::Null
    };

    let late = match previous {
        None => Value::Null,
        Some(prev) => {
            let before: std::collections::HashMap<&str, &str> = programmes(prev)
                .into_iter()
                .filter_map(|p| Some((p["plan_pid"].as_str()?, value_of(p))))
                .collect();
            let names: Vec<&str> = progs
                .iter()
                .filter(|p| matches!(value_of(p), "amber_red" | "red"))
                .filter(|p| p["plan_pid"].as_str().and_then(|id| before.get(id)) == Some(&"green"))
                .filter_map(|p| p["name"].as_str())
                .collect();
            json!({ "count": names.len(), "programmes": names })
        }
    };

    let share = &body["milestone_signoffs"]["totals"]["met_share_of_delivered_bp"];
    let signoff = if share.is_number() {
        json!({
            "met_share_of_delivered_bp": share,
            "delivered": body["milestone_signoffs"]["totals"]["delivered"],
            "met": body["milestone_signoffs"]["totals"]["met"],
        })
    } else {
        Value::Null
    };

    let decided: std::collections::HashSet<&str> = body["exceptions"]["closed_in_period"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|e| e["decision"].as_str().is_some_and(|d| !d.trim().is_empty()))
        .filter_map(|e| e["plan_pid"].as_str())
        .collect();
    let red = if progs.is_empty() {
        Value::Null
    } else {
        let names: Vec<&str> = progs
            .iter()
            .filter(|p| value_of(p) == "red")
            .filter(|p| !matches!(p["programme_status"].as_str(), Some("paused" | "stopped")))
            .filter(|p| {
                p["plan_pid"]
                    .as_str()
                    .is_none_or(|id| !decided.contains(id))
            })
            .filter_map(|p| p["name"].as_str())
            .collect();
        json!({ "count": names.len(), "programmes": names })
    };

    json!({
        "self_vs_independent": gap,
        "late_discovery": late,
        "milestones_met_with_signoff": signoff,
        "red_without_decision": red,
        "reasons_null": {
            "self_vs_independent": gap.is_null().then_some("no programme has an independent assessment on record"),
            "late_discovery": late.is_null().then_some("there is no earlier issued pack to compare with"),
            "milestones_met_with_signoff": signoff.is_null().then_some("no milestone that needs sign-off has been delivered"),
            "red_without_decision": red.is_null().then_some("the pack has no programmes"),
        },
        "note": "Computed from this pack and the previous issued pack as stored, never from live records. A measure with no inputs is null with the reason, not zero. Red without a decision looks only at decisions closed in the period; an older decision is not seen here.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prog(id: &str, name: &str, value: &str, kind: &str, gap: bool, status: &str) -> Value {
        json!({ "plan_pid": id, "name": name, "programme_status": status,
            "confidence": { "value": value, "kind": kind },
            "disagreement": { "with_derived": false, "independent_vs_self": gap } })
    }

    fn pack(progs: &[Value]) -> Value {
        json!({ "portfolio_map": { "groups": [{ "programmes": progs }] },
            "exceptions": { "closed_in_period": [] }, "milestone_signoffs": { "totals": { "met_share_of_delivered_bp": null } } })
    }

    #[test]
    fn late_discovery_is_red_now_and_green_before() {
        let before = pack(&[
            prog("a", "A", "green", "self", false, "active"),
            prog("b", "B", "amber", "self", false, "active"),
        ]);
        let now = pack(&[
            prog("a", "A", "red", "self", false, "active"),
            prog("b", "B", "red", "self", false, "active"),
            prog("c", "C", "red", "self", false, "active"),
        ]);
        let m = measures(&now, Some(&before));
        assert_eq!(
            m["late_discovery"]["programmes"],
            json!(["A"]),
            "B had amber on the way; C was not in the earlier pack"
        );
    }

    #[test]
    fn with_no_earlier_pack_late_discovery_is_null_with_a_reason() {
        let m = measures(
            &pack(&[prog("a", "A", "red", "self", false, "active")]),
            None,
        );
        assert!(m["late_discovery"].is_null());
        assert!(m["reasons_null"]["late_discovery"].is_string());
    }

    #[test]
    fn the_gap_needs_an_independent_assessment_to_exist() {
        let none = measures(
            &pack(&[prog("a", "A", "green", "self", false, "active")]),
            None,
        );
        assert!(none["self_vs_independent"].is_null());
        let some = measures(
            &pack(&[
                prog("a", "A", "green", "independent", true, "active"),
                prog("b", "B", "green", "independent", false, "active"),
            ]),
            None,
        );
        assert_eq!(some["self_vs_independent"]["programmes"], json!(["A"]));
    }

    #[test]
    fn red_without_a_decision_skips_paused_stopped_and_decided() {
        let mut now = pack(&[
            prog("a", "A", "red", "self", false, "active"),
            prog("b", "B", "red", "self", false, "paused"),
            prog("c", "C", "red", "self", false, "stopped"),
            prog("d", "D", "red", "self", false, "active"),
            prog("e", "E", "amber", "self", false, "active"),
        ]);
        now["exceptions"]["closed_in_period"] =
            json!([{ "plan_pid": "d", "decision": "Recovery plan approved" }]);
        let m = measures(&now, None);
        assert_eq!(m["red_without_decision"]["programmes"], json!(["A"]));
        assert!(
            measures(&json!({}), None)["red_without_decision"].is_null(),
            "no programmes: null, not zero"
        );
    }

    #[test]
    fn the_signoff_share_is_passed_through_or_null() {
        let mut p = pack(&[]);
        assert!(measures(&p, None)["milestones_met_with_signoff"].is_null());
        p["milestone_signoffs"]["totals"] =
            json!({ "met_share_of_delivered_bp": 5000, "delivered": 2, "met": 1 });
        assert_eq!(
            measures(&p, None)["milestones_met_with_signoff"]["met_share_of_delivered_bp"],
            5000
        );
    }
}
