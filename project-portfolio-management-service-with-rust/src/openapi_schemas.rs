//! Request and response schemas for the director-pack work: change requests (IR-4, IR-5), the pack
//! summary (IR-6), the monthly pack (DP-1), delivery confidence, exceptions and the portfolio map
//! (DP-2 to DP-4), gates and programme status (DP-5), funding and financial-year money (DP-6) and the
//! buffer fever chart (DP-7).
//!
//! [`schemas`] is merged into `components.schemas`; [`attach`] adds the `requestBody` and the response
//! `content` to operations already documented in [`crate::openapi_more`], so the prose there and the
//! shapes here cannot drift apart without a test noticing (see the tests: every `$ref` resolves, every
//! attachment names a real operation, every schema is used).

#![allow(clippy::needless_pass_by_value)] // small builders for JSON literals

use serde_json::{Value, json};

fn r(name: &str) -> Value {
    json!({ "$ref": format!("#/components/schemas/{name}") })
}
fn s() -> Value {
    json!({ "type": "string" })
}
fn s_null() -> Value {
    json!({ "type": "string", "nullable": true })
}
fn uuid() -> Value {
    json!({ "type": "string", "format": "uuid" })
}
fn uuid_null() -> Value {
    json!({ "type": "string", "format": "uuid", "nullable": true })
}
fn date() -> Value {
    json!({ "type": "string", "format": "date" })
}
fn date_null() -> Value {
    json!({ "type": "string", "format": "date", "nullable": true })
}
fn time() -> Value {
    json!({ "type": "string", "format": "date-time" })
}
fn time_null() -> Value {
    json!({ "type": "string", "format": "date-time", "nullable": true })
}
fn int() -> Value {
    json!({ "type": "integer", "format": "int64" })
}
fn int_null() -> Value {
    json!({ "type": "integer", "format": "int64", "nullable": true })
}
fn num() -> Value {
    json!({ "type": "number", "format": "double" })
}
fn boolean() -> Value {
    json!({ "type": "boolean" })
}
fn any_object() -> Value {
    json!({ "type": "object", "additionalProperties": true })
}
fn list(item: Value) -> Value {
    json!({ "type": "array", "items": item })
}
fn en(values: &[&str]) -> Value {
    json!({ "type": "string", "enum": values })
}
fn obj(required: &[&str], props: Value) -> Value {
    json!({ "type": "object", "required": required, "properties": props })
}

const CONFIDENCE: &[&str] = &["green", "amber_green", "amber", "amber_red", "red"];

/// The schemas, by name.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn schemas() -> Value {
    json!({
        // ── change requests ──
        "ChangeRequestInput": obj(&["title", "description", "reason"], json!({
            "title": s(), "description": s(), "reason": s(),
            "kind": en(&["scope", "schedule", "cost", "quality", "other"]),
            "impact_schedule_days": { "type": "integer", "format": "int64", "nullable": true, "description": "Business days; negative pulls the finish in." },
            "impact_cost_minor": int_null(), "cost_currency": { "type": "string", "nullable": true, "description": "ISO 4217; required with impact_cost_minor." },
            "impact_summary": s_null(),
            "edits": { "type": "array", "nullable": true, "items": any_object(), "description": "The what-if sandbox's edit list." }
        })),
        "ChangeRequestImpact": obj(&[], json!({
            "schedule_business_days": int_null(), "cost_minor": int_null(), "cost_currency": s_null(),
            "summary": s_null(), "has_forecast_edits": boolean() })),
        "ChangeRequest": obj(&["pid", "plan_pid", "title", "status", "kind"], json!({
            "pid": uuid(), "plan_pid": uuid(), "title": s(), "description": s(), "reason": s(),
            "kind": en(&["scope", "schedule", "cost", "quality", "other"]),
            "impact": r("ChangeRequestImpact"), "edits": { "nullable": true, "type": "array", "items": any_object() },
            "status": en(&["draft", "submitted", "under_review", "approved", "rejected", "implemented", "withdrawn"]),
            "requested_by": s_null(), "requested_on": date(), "age_calendar_days": int(),
            "approver_ref": s_null(), "decided_by": s_null(), "decided_on": date_null(),
            "decision_reason": s_null(), "implemented_on": date_null(), "baseline_pid": uuid_null(),
            "issue_pids": { "type": "array", "nullable": true, "items": uuid() },
            "risk_pids": { "type": "array", "nullable": true, "items": uuid() }
        })),
        "ChangeRequestSubmitInput": obj(&["approver_ref"], json!({
            "approver_ref": { "type": "string", "description": "person:<uuid> or worker:<uuid>; never the requester." } })),
        "ChangeRequestDecisionInput": obj(&["decision", "reason"], json!({
            "decision": en(&["approve", "reject"]),
            "reason": { "type": "string", "minLength": 10 } })),
        "ChangeRequestLinksInput": obj(&[], json!({
            "issue_pids": list(uuid()), "risk_pids": list(uuid()) })),
        "ChangeRequestList": obj(&["plan_pid", "change_requests", "total"], json!({
            "plan_pid": uuid(), "change_requests": list(r("ChangeRequest")), "total": int() })),
        "ChangeRequestPortfolio": obj(&["change_requests", "summary", "as_of"], json!({
            "change_requests": list(r("ChangeRequest")),
            "summary": obj(&["by_status", "awaiting_decision"], json!({
                "by_status": { "type": "object", "additionalProperties": int() },
                "awaiting_decision": int(),
                "oldest_open": { "nullable": true, "allOf": [r("ChangeRequest")] } })),
            "as_of": date(), "note": s() })),
        "ChangeRequestPreview": obj(&["change_request_pid", "forecast"], json!({
            "change_request_pid": uuid(), "label": s(), "note": s(),
            "stated_impact": r("ChangeRequestImpact"), "forecast": any_object() })),
        "ChangeRequestBaselined": obj(&["change_request_pid", "baseline_pid", "baseline_version"], json!({
            "change_request_pid": uuid(), "baseline_pid": uuid(), "baseline_version": int() })),

        // ── the pack summary and the pack ──
        "DirectorSummary": obj(&["available", "as_of", "window_days"], json!({
            "available": boolean(), "reason": s_null(), "window_days": int(), "as_of": date(),
            "issues": any_object(), "change_requests": any_object() })),
        "DirectorPackDraftInput": obj(&["period", "audience"], json!({
            "period": { "type": "string", "pattern": "^[0-9]{4}-(0[1-9]|1[0-2])$", "description": "YYYY-MM" },
            "audience": s(),
            "cut_off": { "type": "string", "format": "date-time", "nullable": true, "description": "Defaults to now; not in the future and not more than a year old." } })),
        "DirectorPackReissueInput": obj(&["change_note"], json!({ "change_note": s() })),
        "DirectorPackMeta": obj(&["pid", "period", "audience", "version", "status"], json!({
            "pid": uuid(), "period": s(), "audience": s(), "version": int(),
            "status": en(&["draft", "issued"]), "cut_off": time(), "change_note": s_null(),
            "supersedes_pid": uuid_null(), "issued_at": time_null(), "issued_by": s_null() })),
        "DirectorPack": json!({ "allOf": [r("DirectorPackMeta"), obj(&["body"], json!({
            "body": r("DirectorPackBody") }))] }),
        "DirectorPackBody": obj(&["cover", "data_quality"], json!({
            "cover": any_object(), "what_changed": list(any_object()), "basis": s(),
            "data_quality": obj(&["sections_included", "sections_not_yet_built"], json!({
                "plans_included": int(), "plans_left_out_private_or_marked": int(),
                "sections_included": list(s()), "sections_not_yet_built": list(s()), "note": s() })),
            "portfolio_map": r("PortfolioMap"), "exceptions": any_object(),
            "gate_decisions": r("GateDecisions"), "funding": r("PortfolioMoney"),
            "buffer_fever": r("BufferFever"), "earned_value": r("PortfolioEarnedValue"), "benefits": r("PortfolioBenefits"), "issues": any_object(), "change_requests": any_object() })),
        "DirectorPackList": obj(&["packs"], json!({ "packs": list(r("DirectorPackMeta")) })),

        // ── delivery confidence, exceptions, aims, the map ──
        "DeliveryConfidenceInput": obj(&["value", "kind", "rationale"], json!({
            "value": en(CONFIDENCE), "kind": en(&["self", "independent"]),
            "rationale": { "type": "string", "minLength": 10 },
            "assessed_on": { "type": "string", "format": "date", "nullable": true, "description": "Defaults to today; not in the future." } })),
        "DeliveryConfidenceAssessment": obj(&["pid", "plan_pid", "value", "kind"], json!({
            "pid": uuid(), "plan_pid": uuid(), "value": en(CONFIDENCE), "kind": en(&["self", "independent"]),
            "rationale": s(), "assessor": s_null(), "assessed_on": date() })),
        "DeliveryConfidenceShown": obj(&["value"], json!({
            "value": { "type": "string", "enum": ["green", "amber_green", "amber", "amber_red", "red", "not_assessed"] },
            "kind": en(&["self", "independent"]), "assessed_on": date(), "stale": boolean(), "marked_self": boolean() })),
        "DeliveryConfidenceHistory": obj(&["plan_pid", "shown", "history"], json!({
            "plan_pid": uuid(), "shown": r("DeliveryConfidenceShown"),
            "history": list(r("DeliveryConfidenceAssessment")), "note": s() })),
        "DeliveryConfidenceRecorded": obj(&["assessment"], json!({
            "assessment": r("DeliveryConfidenceAssessment"),
            "exception": { "nullable": true, "allOf": [r("DeliveryException")] } })),
        "DeliveryException": obj(&["pid", "plan_pid", "status"], json!({
            "pid": uuid(), "plan_pid": uuid(), "status": en(&["open", "closed"]), "opened_at": time(),
            "opened_value": en(&["amber_red", "red"]), "worst_value": en(&["amber_red", "red"]),
            "worsened_count": int(), "cause": s_null(), "decision_needed": s_null(),
            "owner_ref": s_null(), "owner_role": s_null(), "decide_by": date_null(),
            "completed_at": time_null(), "complete": boolean(), "missing": list(s()),
            "breached": boolean(), "window_hours": int(), "hours_open": int(),
            "closed_at": time_null(), "decision": s_null(), "decided_by": s_null() })),
        "DeliveryExceptionList": obj(&["exceptions", "window_hours"], json!({
            "exceptions": list(r("DeliveryException")), "window_hours": int() })),
        "ExceptionCompleteInput": obj(&[], json!({
            "cause": s_null(), "decision_needed": s_null(), "owner_ref": s_null(),
            "owner_role": { "type": "string", "nullable": true, "description": "What a pack shows instead of the person." },
            "decide_by": date_null() })),
        "ExceptionDecisionInput": obj(&["decision"], json!({ "decision": { "type": "string", "minLength": 10 } })),
        "StrategicAimInput": obj(&["strategic"], json!({ "strategic": boolean() })),
        "StrategicAimMarked": obj(&["objective_pid", "strategic"], json!({ "objective_pid": uuid(), "strategic": boolean() })),
        "PrimaryAimInput": obj(&["objective_pid"], json!({ "objective_pid": uuid() })),
        "PrimaryAimSet": obj(&["plan_pid", "primary_aim"], json!({ "plan_pid": uuid(), "primary_aim": uuid() })),
        "PortfolioMapProgramme": obj(&["plan_pid", "name", "confidence", "trend"], json!({
            "plan_pid": uuid(), "name": s(), "stage": s_null(), "phase": s_null(),
            "programme_status": en(&["active", "paused", "stopped"]),
            "confidence": r("DeliveryConfidenceShown"), "derived_rag": any_object(),
            "trend": en(&["up", "down", "same", "new", "not_assessed", "no_earlier_pack"]),
            "disagreement": any_object(), "also_serves": list(s()), "reason_no_primary": s_null() })),
        "PortfolioMapGroup": obj(&["aim", "programmes"], json!({
            "aim_pid": uuid_null(), "aim": s(), "programmes": list(r("PortfolioMapProgramme")) })),
        "PortfolioMap": obj(&["period", "programmes_total", "groups"], json!({
            "period": s(), "programmes_total": int(), "groups": list(r("PortfolioMapGroup")), "note": s() })),

        // ── gates and programme status ──
        "PlannedGateInput": obj(&["planned_on"], json!({ "planned_on": date() })),
        "PlannedGate": obj(&["gate", "planned_on"], json!({
            "gate": en(&["g0_concept", "g1_feasibility", "g2_definition", "g3_delivery", "g4_launch", "g5_benefits"]),
            "planned_on": date() })),
        "PlannedGateSet": obj(&["plan_pid", "gate", "planned_on"], json!({
            "plan_pid": uuid(), "gate": s(), "planned_on": date() })),
        "PlannedGates": obj(&["plan_pid", "planned_gates"], json!({
            "plan_pid": uuid(), "planned_gates": list(r("PlannedGate")) })),
        "ProgrammeStatusInput": obj(&["status", "reason"], json!({
            "status": en(&["active", "paused", "stopped"]), "reason": { "type": "string", "minLength": 10 },
            "gate_review_pid": { "type": "string", "format": "uuid", "nullable": true, "description": "A gate review of this programme." },
            "exception_pid": { "type": "string", "format": "uuid", "nullable": true, "description": "A delivery exception of this programme." } })),
        "ProgrammeStatusDecided": obj(&["plan_pid", "status", "decision"], json!({
            "plan_pid": uuid(), "status": en(&["active", "paused", "stopped"]), "decision": uuid() })),
        "ProgrammeStatusDecision": obj(&["status", "reason", "decided_on"], json!({
            "status": en(&["active", "paused", "stopped"]), "reason": s(), "decided_by": s_null(),
            "decided_on": date(), "gate_review_pid": uuid_null(), "exception_pid": uuid_null() })),
        "ProgrammeStillHeld": obj(&["allocations_flagged", "money_by_currency", "benefits"], json!({
            "allocations_flagged": int(),
            "money_by_currency": list(obj(&["currency"], json!({ "currency": s(), "planned_minor": int(), "actual_minor": int() }))),
            "benefits": int() })),
        "ProgrammeStatus": obj(&["plan_pid", "status", "history"], json!({
            "plan_pid": uuid(), "status": en(&["active", "paused", "stopped"]),
            "history": list(r("ProgrammeStatusDecision")),
            "still_held": { "nullable": true, "allOf": [r("ProgrammeStillHeld")] }, "note": s() })),
        "GateLine": obj(&["plan_pid", "plan", "gate", "outcome"], json!({
            "plan_pid": uuid(), "plan": s(), "gate": s(), "outcome": en(&["passed", "failed", "deferred"]),
            "decided_on": date_null(), "planned_on": date_null(), "reason": s_null(), "days_late": int_null() })),
        "GateDecisions": obj(&["period", "passed", "failed", "deferred", "paused", "stopped"], json!({
            "period": s(), "passed": list(r("GateLine")), "failed": list(r("GateLine")), "deferred": list(r("GateLine")),
            "paused": list(any_object()), "stopped": list(any_object()),
            "status_changes_in_period": list(any_object()), "note": s() })),

        // ── funding and financial-year money ──
        "FundingSourceInput": obj(&["name"], json!({ "name": { "type": "string", "minLength": 2, "maxLength": 120 } })),
        "FundingSourceActiveInput": obj(&["active"], json!({ "active": boolean() })),
        "FundingSource": obj(&["pid", "name", "active"], json!({ "pid": uuid(), "name": s(), "active": boolean() })),
        "FundingSourceList": obj(&["funding_sources"], json!({ "funding_sources": list(r("FundingSource")) })),
        "LineFundingSourceInput": obj(&[], json!({
            "funding_source_pid": { "type": "string", "format": "uuid", "nullable": true, "description": "An active source, or null to clear." } })),
        "LineFundingSourceSet": obj(&["line"], json!({ "line": uuid(), "funding_source_pid": uuid_null() })),
        "MoneyRow": obj(&["funding_source", "class", "currency", "budget_minor", "spend_minor", "variance_minor"], json!({
            "funding_source_pid": uuid_null(), "funding_source": { "type": "string", "description": "\"no funding source\" when the line has none." },
            "class": en(&["capex", "opex", "all"]), "currency": s(),
            "budget_minor": int(), "spend_minor": int(), "variance_minor": int(), "lines": int() })),
        "PortfolioMoney": obj(&["financial_year", "programmes", "data_quality"], json!({
            "financial_year": obj(&["label", "start", "end"], json!({ "label": s(), "start": date(), "end": date(), "starts_in_month": int() })),
            "classes_combined": boolean(),
            "programmes": list(obj(&["plan_pid", "name", "by_funding_source"], json!({
                "plan_pid": uuid(), "name": s(), "by_funding_source": list(r("MoneyRow")),
                "whole_life_forecast": any_object() }))),
            "not_under_a_programme": list(r("MoneyRow")),
            "data_quality": obj(&["lines_with_no_funding_source"], json!({
                "lines_with_no_funding_source": int(), "listed_lines_with_no_funding_source": list(any_object()),
                "lines_with_no_dates": int(), "lines_in_other_financial_years": int(), "lines_running_past_the_year_end": int() })),
            "note": s() })),

        // ── validated benefits ──
        "BenefitClassificationInput": obj(&[], json!({
            "recurrence": { "type": "string", "enum": ["recurring", "one_off"], "nullable": true },
            "annualized_minor": { "type": "integer", "format": "int64", "nullable": true, "description": "Yearly amount of a recurring, financial benefit." },
            "classification": { "type": "string", "enum": ["cash_releasing", "non_cash_releasing", "societal"], "nullable": true },
            "kpi_pid": uuid_null() })),
        "BenefitClassification": obj(&["benefit"], json!({
            "benefit": uuid(), "recurrence": s_null(), "annualized_minor": int_null(),
            "classification": s_null(), "kpi_pid": uuid_null() })),
        "BenefitValidationInput": obj(&["amount_minor"], json!({
            "amount_minor": { "type": "integer", "format": "int64", "minimum": 0, "description": "Validated to date, in the benefit's currency." },
            "evidence_attachment_pid": uuid_null(), "note": s_null() })),
        "BenefitValidated": obj(&["validation", "benefit", "amount_minor", "currency"], json!({
            "validation": uuid(), "benefit": uuid(), "amount_minor": int(), "currency": s(), "validated_on": date() })),
        "BenefitValidations": obj(&["benefit", "claimed_minor", "standing", "history"], json!({
            "benefit": uuid(), "claimed_minor": int(), "validated_minor": int_null(),
            "standing": en(&["not_claimed", "claimed", "partly_validated", "validated"]),
            "history": list(obj(&["amount_minor", "currency"], json!({
                "amount_minor": int(), "currency": s(), "validator": s_null(), "validated_on": date(),
                "evidence_attachment_pid": uuid_null(), "note": s_null() }))),
            "note": s() })),
        "BenefitRow": obj(&["currency", "recurrence", "classification", "planned_minor", "claimed_minor", "validated_minor"], json!({
            "currency": s(), "recurrence": en(&["recurring", "one_off", "not_stated"]),
            "classification": s(), "planned_minor": int(), "claimed_minor": int(), "validated_minor": int(),
            "unvalidated_claim_minor": int(), "benefits": int(), "with_no_planned_amount": int(), "claimed_only": int() })),
        "NonFinancialBenefit": obj(&["title", "kpi"], json!({
            "plan": s_null(), "title": s(), "classification": s_null(),
            "kpi": obj(&["state"], json!({ "state": s(), "kpi": s(), "latest": int_null(), "reason": s() })) })),
        "PortfolioBenefits": obj(&["programmes", "totals"], json!({
            "programmes": list(obj(&["plan_pid", "name", "financial", "non_financial"], json!({
                "plan_pid": uuid(), "name": s(), "financial": list(r("BenefitRow")),
                "non_financial": list(r("NonFinancialBenefit")) }))),
            "not_under_a_programme": obj(&[], json!({
                "financial": list(r("BenefitRow")), "non_financial": list(r("NonFinancialBenefit")) })),
            "totals": list(r("BenefitRow")), "note": s() })),

        // ── portfolio earned value ──
        "EarnedValueCurrency": obj(&["currency", "state"], json!({
            "currency": s(), "state": en(&["included", "below EVM threshold"]),
            "totals": any_object(), "ac_source": s(), "spi": any_object(), "cpi": any_object(),
            "eac": any_object(), "vac_minor": int_null(),
            "bac_minor": int(), "threshold_minor": int_null(), "reason": s() })),
        "EarnedValueProgramme": obj(&["plan_pid", "name"], json!({
            "plan_pid": uuid(), "name": s(), "plans_without_figures": int(),
            "state": en(&["no_baseline"]), "reason": s(),
            "currencies": list(r("EarnedValueCurrency")) })),
        "PortfolioEarnedValue": obj(&["as_of", "ac_source", "bac_inclusion_threshold", "programmes"], json!({
            "as_of": date(), "ac_source": s(),
            "bac_inclusion_threshold": obj(&["minor_by_currency", "configured"], json!({
                "minor_by_currency": { "type": "object", "additionalProperties": int() },
                "configured": boolean(), "setting_unusable_so_none_applied": boolean(), "note": s() })),
            "ratio_thresholds": any_object(), "programmes": list(r("EarnedValueProgramme")), "note": s() })),

        // ── the buffer fever chart ──
        "ProjectBufferInput": json!({ "type": "object", "description": "Either buffer_days or rule, not both.", "properties": {
            "buffer_days": { "type": "integer", "minimum": 1, "maximum": 2000, "description": "Business days." },
            "rule": en(&["half_of_critical_duration"]), "note": s_null() } }),
        "ProjectBufferSet": obj(&["plan_pid", "buffer_days", "source"], json!({
            "plan_pid": uuid(), "buffer_days": int(), "source": en(&["declared", "rule"]) })),
        "FeverLine": obj(&["at_start", "at_end"], json!({ "at_start": num(), "at_end": num() })),
        "FeverLines": obj(&["green_amber", "amber_red"], json!({ "green_amber": r("FeverLine"), "amber_red": r("FeverLine") })),
        "FeverPoint": obj(&["chain_completed_pct", "buffer_consumed_pct", "zone"], json!({
            "chain_completed_pct": num(), "buffer_consumed_pct": num(),
            "plotted_x_pct": num(), "plotted_y_pct": num(),
            "edge": { "type": "string", "enum": ["below", "above"], "nullable": true },
            "zone": en(&["green", "amber", "red"]), "slip_days": int(), "buffer_days": int() })),
        "FeverProgramme": obj(&["plan_pid", "plan"], json!({
            "plan_pid": uuid(), "plan": s(), "point": { "nullable": true, "allOf": [r("FeverPoint")] },
            "no_fever_chart_point": obj(&["code", "reason"], json!({
                "code": en(&["no_buffer", "no_baseline", "no_chain", "no_forecast"]), "reason": s() })),
            "previous": { "nullable": true, "type": "object", "description": "The point in the last issued pack.", "additionalProperties": true },
            "buffer": any_object(), "baseline_version": int(), "chain": any_object() })),
        "BufferFever": obj(&["lines", "programmes", "table"], json!({
            "lines": r("FeverLines"), "configured_lines_unusable_default_used": boolean(),
            "programmes": list(r("FeverProgramme")), "table": list(any_object()), "note": s() })),
        "ProjectBufferView": obj(&["plan_pid", "history", "now", "lines"], json!({
            "plan_pid": uuid(), "history": list(any_object()), "now": r("FeverProgramme"),
            "lines": r("FeverLines"), "configured_lines_unusable": boolean() }))
    })
}

/// One attachment: method, path, request schema, and `(status, response schema)`.
type Attach = (
    &'static str,
    &'static str,
    Option<&'static str>,
    &'static [(&'static str, &'static str)],
);

/// What to attach to which documented operation.
pub const ATTACH: &[Attach] = &[
    (
        "get",
        "/api/change-requests",
        None,
        &[("200", "ChangeRequestPortfolio")],
    ),
    (
        "get",
        "/api/plans/{pid}/change-requests",
        None,
        &[("200", "ChangeRequestList")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests",
        Some("ChangeRequestInput"),
        &[("200", "ChangeRequest")],
    ),
    (
        "get",
        "/api/plans/{pid}/change-requests/{cr}",
        None,
        &[("200", "ChangeRequest")],
    ),
    (
        "put",
        "/api/plans/{pid}/change-requests/{cr}",
        Some("ChangeRequestInput"),
        &[("200", "ChangeRequest")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/submit",
        Some("ChangeRequestSubmitInput"),
        &[("200", "ChangeRequest")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/review",
        None,
        &[("200", "ChangeRequest")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/decision",
        Some("ChangeRequestDecisionInput"),
        &[("200", "ChangeRequest")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/implement",
        None,
        &[("200", "ChangeRequest")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/withdraw",
        None,
        &[("200", "ChangeRequest")],
    ),
    (
        "put",
        "/api/plans/{pid}/change-requests/{cr}/links",
        Some("ChangeRequestLinksInput"),
        &[("200", "ChangeRequest")],
    ),
    (
        "get",
        "/api/plans/{pid}/change-requests/{cr}/preview",
        None,
        &[("200", "ChangeRequestPreview")],
    ),
    (
        "post",
        "/api/plans/{pid}/change-requests/{cr}/baseline",
        None,
        &[("200", "ChangeRequestBaselined")],
    ),
    (
        "get",
        "/api/director-pack/issues-and-changes",
        None,
        &[("200", "DirectorSummary")],
    ),
    (
        "get",
        "/api/director-packs",
        None,
        &[("200", "DirectorPackList")],
    ),
    (
        "post",
        "/api/director-packs",
        Some("DirectorPackDraftInput"),
        &[("201", "DirectorPackMeta")],
    ),
    (
        "get",
        "/api/director-packs/{pid}",
        None,
        &[("200", "DirectorPack")],
    ),
    (
        "post",
        "/api/director-packs/{pid}/issue",
        None,
        &[("200", "DirectorPackMeta")],
    ),
    (
        "post",
        "/api/director-packs/{pid}/reissue",
        Some("DirectorPackReissueInput"),
        &[("201", "DirectorPackMeta")],
    ),
    (
        "put",
        "/api/plans/{pid}/delivery-confidence",
        Some("DeliveryConfidenceInput"),
        &[("200", "DeliveryConfidenceRecorded")],
    ),
    (
        "get",
        "/api/plans/{pid}/delivery-confidence",
        None,
        &[("200", "DeliveryConfidenceHistory")],
    ),
    (
        "get",
        "/api/exceptions",
        None,
        &[("200", "DeliveryExceptionList")],
    ),
    (
        "put",
        "/api/exceptions/{pid}",
        Some("ExceptionCompleteInput"),
        &[("200", "DeliveryException")],
    ),
    (
        "post",
        "/api/exceptions/{pid}/decision",
        Some("ExceptionDecisionInput"),
        &[("200", "DeliveryException")],
    ),
    (
        "put",
        "/api/objectives/{pid}/strategic-aim",
        Some("StrategicAimInput"),
        &[("200", "StrategicAimMarked")],
    ),
    (
        "put",
        "/api/plans/{pid}/primary-aim",
        Some("PrimaryAimInput"),
        &[("200", "PrimaryAimSet")],
    ),
    (
        "get",
        "/api/portfolio-map",
        None,
        &[("200", "PortfolioMap")],
    ),
    (
        "get",
        "/api/plans/{pid}/planned-gates",
        None,
        &[("200", "PlannedGates")],
    ),
    (
        "put",
        "/api/plans/{pid}/planned-gates/{gate}",
        Some("PlannedGateInput"),
        &[("200", "PlannedGateSet")],
    ),
    (
        "get",
        "/api/plans/{pid}/programme-status",
        None,
        &[("200", "ProgrammeStatus")],
    ),
    (
        "post",
        "/api/plans/{pid}/programme-status",
        Some("ProgrammeStatusInput"),
        &[("200", "ProgrammeStatusDecided")],
    ),
    (
        "get",
        "/api/funding-sources",
        None,
        &[("200", "FundingSourceList")],
    ),
    (
        "post",
        "/api/funding-sources",
        Some("FundingSourceInput"),
        &[("200", "FundingSource")],
    ),
    (
        "put",
        "/api/funding-sources/{pid}",
        Some("FundingSourceActiveInput"),
        &[("200", "FundingSource")],
    ),
    (
        "put",
        "/api/plans/{pid}/budget-lines/{line}/funding-source",
        Some("LineFundingSourceInput"),
        &[("200", "LineFundingSourceSet")],
    ),
    (
        "get",
        "/api/portfolio-money",
        None,
        &[("200", "PortfolioMoney")],
    ),
    (
        "put",
        "/api/plans/{pid}/project-buffer",
        Some("ProjectBufferInput"),
        &[("200", "ProjectBufferSet")],
    ),
    (
        "get",
        "/api/plans/{pid}/project-buffer",
        None,
        &[("200", "ProjectBufferView")],
    ),
    ("get", "/api/buffer-fever", None, &[("200", "BufferFever")]),
    (
        "get",
        "/api/portfolio-earned-value",
        None,
        &[("200", "PortfolioEarnedValue")],
    ),
    (
        "put",
        "/api/plans/{pid}/benefits/{benefit}/classification",
        Some("BenefitClassificationInput"),
        &[("200", "BenefitClassification")],
    ),
    (
        "post",
        "/api/plans/{pid}/benefits/{benefit}/validations",
        Some("BenefitValidationInput"),
        &[("200", "BenefitValidated")],
    ),
    (
        "get",
        "/api/plans/{pid}/benefits/{benefit}/validations",
        None,
        &[("200", "BenefitValidations")],
    ),
    (
        "get",
        "/api/portfolio-benefits",
        None,
        &[("200", "PortfolioBenefits")],
    ),
];

/// Add the request bodies and response schemas in [`ATTACH`] to the documented operations. An entry that
/// names an operation or a response that is not documented is skipped here and caught by a test.
pub fn attach(paths: &mut Value) {
    for (method, path, request, responses) in ATTACH {
        let Some(op) = paths.get_mut(*path).and_then(|p| p.get_mut(*method)) else {
            continue;
        };
        if let Some(name) = request {
            op["requestBody"] = json!({ "required": true, "content": { "application/json": { "schema": r(name) } } });
        }
        for (status, name) in *responses {
            if let Some(resp) = op.get_mut("responses").and_then(|x| x.get_mut(*status)) {
                resp["content"] = json!({ "application/json": { "schema": r(name) } });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn refs(v: &Value, out: &mut BTreeSet<String>) {
        match v {
            Value::Object(m) => {
                for (k, x) in m {
                    if k == "$ref" {
                        if let Some(n) = x
                            .as_str()
                            .and_then(|s| s.strip_prefix("#/components/schemas/"))
                        {
                            out.insert(n.to_string());
                        }
                    } else {
                        refs(x, out);
                    }
                }
            }
            Value::Array(a) => a.iter().for_each(|x| refs(x, out)),
            _ => {}
        }
    }

    #[test]
    fn every_attachment_names_a_documented_operation_and_response() {
        let doc = crate::openapi::spec();
        let mut faults = Vec::new();
        for (method, path, request, responses) in ATTACH {
            let op = &doc["paths"][*path][*method];
            if !op.is_object() {
                faults.push(format!("{method} {path} is not documented"));
                continue;
            }
            if op.get("requestBody").is_some() != request.is_some() {
                faults.push(format!("{method} {path}: request body"));
            }
            for (status, name) in *responses {
                if !op["responses"][*status].is_object() {
                    let have: Vec<_> = op["responses"]
                        .as_object()
                        .map(|m| m.keys().cloned().collect())
                        .unwrap_or_default();
                    faults.push(format!(
                        "{method} {path} has no {status} response (has {have:?})"
                    ));
                } else if op["responses"][*status]["content"]["application/json"]["schema"]["$ref"]
                    != format!("#/components/schemas/{name}")
                {
                    faults.push(format!("{method} {path} {status} does not point at {name}"));
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    #[test]
    fn every_reference_resolves_and_every_schema_is_used() {
        let doc = crate::openapi::spec();
        let defined: BTreeSet<String> = doc["components"]["schemas"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        let mut used = BTreeSet::new();
        refs(&doc, &mut used);
        let dangling: Vec<_> = used.difference(&defined).collect();
        assert!(
            dangling.is_empty(),
            "references to schemas that do not exist: {dangling:?}"
        );
        // The schemas in this file are all reachable: nothing documented that nothing points at.
        let mine: BTreeSet<String> = schemas().as_object().unwrap().keys().cloned().collect();
        let unused: Vec<_> = mine.difference(&used).collect();
        assert!(unused.is_empty(), "schemas nothing refers to: {unused:?}");
    }

    #[test]
    fn nullable_and_required_are_well_formed() {
        for (name, schema) in schemas().as_object().unwrap() {
            if let Some(req) = schema.get("required").and_then(Value::as_array) {
                let props = schema["properties"]
                    .as_object()
                    .unwrap_or_else(|| panic!("{name}: required without properties"));
                for k in req {
                    assert!(
                        props.contains_key(k.as_str().unwrap()),
                        "{name}: required {k} is not a property"
                    );
                }
            }
        }
    }

    #[test]
    fn a_schema_name_is_not_defined_twice() {
        let older = crate::openapi::components();
        let older = older["schemas"].as_object().unwrap();
        let clash: Vec<_> = schemas()
            .as_object()
            .unwrap()
            .keys()
            .filter(|k| older.contains_key(*k))
            .cloned()
            .collect();
        assert!(clash.is_empty(), "defined in both places: {clash:?}");
        assert!(
            crate::openapi::spec()["components"]["schemas"]["Plan"].is_object(),
            "the older schemas are still there"
        );
    }
}
