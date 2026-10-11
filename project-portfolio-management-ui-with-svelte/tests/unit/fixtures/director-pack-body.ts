// A representative issued pack body, as the service stores it: every section, with the shapes that
// need a word (a self-assessed reading, a stale one, a hold, a pause, no funding source, recovery,
// below threshold, no baseline, no supply recorded).
export const BODY = {
  cover: { pack_pid: "p1", version: 2, period: "2026-10", audience: "board", previous_issued_pack: { pid: "p0", period: "2026-09", version: 1 } },
  what_changed: ["Open issues: 9, was 4", "Change requests waiting for a decision: 5, unchanged"],
  basis: "Records dated on or before the cut-off.",
  data_quality: { plans_included: 17, plans_left_out_private_or_marked: 3, sections_included: ["issues"], sections_not_yet_built: [] },
  portfolio_map: {
    period: "2026-10", programmes_total: 3,
    groups: [
      { aim_pid: "a1", aim: "Safer care", programmes: [
        { plan_pid: "x1", name: "Care", stage: "g2_definition", phase: "execution", programme_status: "active",
          confidence: { value: "amber_red", kind: "self", marked_self: true, stale: true }, trend: "down",
          delivery: { delivery_model: "local_programme_central_standards", delivery_owner: { name: "Acme Council" }, delivered_by_organization: false } },
        { plan_pid: "x2", name: "Records", stage: null, phase: null, programme_status: "paused",
          confidence: { value: "not_assessed" }, trend: "not_assessed" },
      ] },
      { aim_pid: null, aim: "no strategic aim", programmes: [
        { plan_pid: "x3", name: "Loose", stage: "g0_concept", phase: null, programme_status: "stopped",
          confidence: { value: "green", kind: "independent", marked_self: false, stale: false }, trend: "no_earlier_pack" },
      ] },
    ],
  },
  exceptions: {
    window_hours: 48,
    open: [{ plan: "Care", worst_value: "red", decision_needed: "Approve the extra supplier", owner_role: "Programme director", decide_by: "2026-10-20", hours_open: 70, breached: true }],
    breached: [{ plan: "Care", worst_value: "red", decision_needed: "Approve the extra supplier", owner_role: "Programme director", decide_by: "2026-10-20", hours_open: 70 }],
    closed_in_period: [],
  },
  gate_decisions: {
    passed: [{ plan: "Care", gate: "g1_feasibility", decided_on: "2026-10-05", planned_on: "2026-10-01", reason: null, days_late: null }],
    failed: [],
    deferred: [
      { plan: "Records", gate: "g2_definition", decided_on: "2026-10-06", planned_on: "2026-09-30", reason: "reviewed as hold", days_late: 11 },
      { plan: "Loose", gate: "g0_concept", decided_on: null, planned_on: "2026-09-01", reason: "planned date passed without a review", days_late: 40 },
    ],
    paused: [{ plan: "Records", since: "2026-10-02", reason: "Funding under review", still_held: { allocations_flagged: 2 } }],
    stopped: [{ plan: "Loose", since: "2026-09-15", reason: "Business case no longer holds", still_held: { allocations_flagged: 0 } }],
  },
  funding: {
    financial_year: { label: "2026/27", start: "2026-04-01", end: "2027-03-31" },
    programmes: [{ plan_pid: "x1", name: "Care",
      by_funding_source: [
        { funding_source_pid: "f1", funding_source: "Departmental grant", class: "capex", currency: "GBP", budget_minor: 10_000_000, spend_minor: 2_500_000, variance_minor: 7_500_000 },
        { funding_source_pid: null, funding_source: "no funding source", class: "opex", currency: "JPY", budget_minor: 123_456, spend_minor: 0, variance_minor: 123_456 },
      ],
      whole_life_forecast: { rows: [{ currency: "GBP", actual_minor: 2_500_000, estimate_at_completion_minor: 11_000_000 }] } }],
    data_quality: { lines_with_no_funding_source: 1, lines_with_no_dates: 2 },
  },
  buffer_fever: {
    lines: { green_amber: { at_start: 30, at_end: 70 }, amber_red: { at_start: 60, at_end: 100 } },
    programmes: [
      { plan_pid: "x1", plan: "Care", point: { chain_completed_pct: 40, buffer_consumed_pct: 60, zone: "amber", edge: null, slip_days: 6, buffer_days: 10 }, previous: { zone: "green" } },
      { plan_pid: "x2", plan: "Records", point: { chain_completed_pct: 10, buffer_consumed_pct: -20, zone: "green", edge: "below", slip_days: -2, buffer_days: 10 }, previous: null },
      { plan_pid: "x3", plan: "Loose", point: null, no_fever_chart_point: { code: "no_baseline", reason: "x" } },
    ],
  },
  earned_value: {
    programmes: [
      { plan_pid: "x1", name: "Care", currencies: [
        { currency: "GBP", state: "included", totals: { bac_minor: 400_000, ev_minor: 200_000, ac_minor: 250_000 }, spi: { bp: 20_000 }, cpi: { bp: 8_000 } },
        { currency: "EUR", state: "below EVM threshold", bac_minor: 5_000 },
      ] },
      { plan_pid: "x3", name: "Loose", state: "no_baseline" },
      { plan_pid: "x4", name: "Portal", method: "product", state: "product", flow: { scope: 12, done: 7, open: 5, throughput_per_week: 1.75, burn_up: [{ done: 2 }, { done: 4 }, { done: 7 }] } },
      { plan_pid: "x5", name: "Empty product", method: "product", state: "no_flow_data" },
    ],
  },
  benefits: { totals: [
    { currency: "GBP", recurrence: "recurring", classification: "cash_releasing", planned_minor: 100_000, claimed_minor: 60_000, validated_minor: 20_000, unvalidated_claim_minor: 40_000 },
    { currency: "GBP", recurrence: "one_off", classification: "not_stated", planned_minor: 500_000, claimed_minor: 0, validated_minor: 0, unvalidated_claim_minor: 0 },
  ] },
  milestone_signoffs: {
    totals: { requiring_signoff: 3, delivered: 2, met: 1, delivered_not_met: 1, met_share_of_delivered_bp: 5000 },
    milestones: [
      { name: "Cutover", due: "2026-12-01", signoff: { state: "delivered_not_met", delivered_on: "2026-11-28", met_on: null, outstanding: ["Acme Council"] } },
      { name: "Training", due: "2026-11-01", signoff: { state: "met", delivered_on: "2026-10-30", met_on: "2026-11-02", outstanding: [] } },
    ],
  },
  external_asks: { period: "2026-10", without_named_tradeoff: 1, asks: [
    { title: "A new portal", status: "approved", tradeoff: "Displaces the records migration", named_tradeoff: true },
    { title: "Faster reports", status: "in_review", tradeoff: null, named_tradeoff: false },
  ] },
  capacity_heatmap: { months: ["2026-10"], pools: [
    { pool_pid: "q1", name: "Engineering", cells: [
      { month: "2026-10", state: "shortfall", demand_fte: 2, supply_fte: 1, gap_fte: -1, partner_delivery_short: true },
      { month: "2026-11", state: "no_supply_recorded", demand_fte: 1, supply_fte: null, gap_fte: null, partner_delivery_short: false },
    ] },
    { pool_pid: null, name: "no pool assigned", cells: [{ month: "2026-10", state: "enough", demand_fte: 1, supply_fte: 2.5, gap_fte: 1.5, partner_delivery_short: false }] },
  ] },
  issues: { open: 9 },
  change_requests: { awaiting_decision: 5 },
};
