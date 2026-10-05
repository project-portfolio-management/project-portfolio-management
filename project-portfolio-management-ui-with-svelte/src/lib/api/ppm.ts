// PPM catalogue client (Phases A–C): typed wrappers over the
// governance / visibility / strategy endpoints, mirroring the
// service's route table (spec 15-roadmap PPM-1..12). One class, one
// method per endpoint, the single source of PPM paths for the UI.
//
// PPM views ship English-first; extending the 13-locale catalogues to
// these strings is a documented follow-up.

import { API_BASE_URL } from "#lib/config.js";
import { ApiClient } from "./client";
import type { Collection } from "./types";

// ---- wire shapes (mirroring the service responses) ----

export interface Proposal {
  pid: string;
  title: string;
  summary: string | null;
  kind_target: Collection;
  sponsor_ref: string | null;
  strategic_rationale: string | null;
  requested_minor: number | null;
  currency: string | null;
  status:
    "draft" | "submitted" | "in_review" | "approved" | "rejected" | "promoted";
  promoted_plan_pid: string | null;
}

export interface DemandHit {
  source: "plan" | "proposal";
  pid: string;
  name: string;
  score: number;
}

export interface Idea {
  pid: string;
  title: string;
  pitch: string | null;
  tags: string[] | unknown;
  votes: number;
  status: "open" | "converted" | "dismissed";
  converted_proposal_pid: string | null;
}

export interface GateJourney {
  stage: string | null;
  next_gate: string | null;
  reviews: {
    pid: string;
    gate: string;
    decision: string;
    conditions: string | null;
    approver_ref: string | null;
    decided_at: string;
  }[];
}

export interface Risk {
  pid: string;
  title: string;
  status: string;
  probability: number;
  impact: number;
  exposure: number;
  owner_ref: string | null;
  mitigation: string | null;
  review_date: string | null;
  escalated_at: string | null;
  /** Simulation inputs (schedule risk and cost risk); one probability is shared. */
  task_pid?: string | null;
  schedule_impact_days?: number | null;
  probability_percent?: number | null;
  cost_impact_minor?: number | null;
  cost_currency?: string | null;
}

export interface BudgetBoard {
  lines: {
    pid: string;
    category: string;
    description: string;
    currency: string;
    planned_minor: number;
    actual_minor: number;
    /** Low and high final cost for cost risk; the planned amount is the likely one. */
    low_minor?: number | null;
    high_minor?: number | null;
  }[];
  totals: {
    currency: string;
    planned_minor: number;
    actual_minor: number;
    variance_minor: number;
  }[];
}

export interface GovernanceSummary {
  pid: string;
  name: string;
  stage: string | null;
  next_gate: string | null;
  gate_reviews: number;
  latest_review: { gate: string; decision: string; decided_at: string } | null;
  risks: {
    open: number;
    materialised: number;
    max_exposure: number | null;
    total_exposure: number;
  };
  budget: BudgetBoard["totals"];
}

export interface ScheduleView {
  portfolio_pid: string;
  items: {
    pid: string;
    kind: string;
    name: string;
    stage: string | null;
    start: string | null;
    end: string | null;
    on_critical_path: boolean;
  }[];
  edges: {
    pid: string;
    predecessor_pid: string;
    successor_pid: string;
    lag_days: number;
  }[];
  violations: {
    edge_pid: string;
    predecessor: string;
    successor: string;
    earliest_start: string;
    actual_start: string;
  }[];
  critical_path: string[];
  undated: string[];
}

export interface Milestone {
  pid: string;
  name: string;
  due: string;
  done: boolean;
  overdue: boolean;
}

// ---- Scheduling canon views (SC-6 .. SC-9) ----

/** One of the fourteen schedule health checks (SC-9). */
export interface HealthCheck {
  id: number;
  key: string;
  name: string;
  of_what: string;
  count: number;
  of: number;
  value: number | null;
  threshold: { op: string; limit: number | null; unit: "percent" | "milli" | "days" | "result" };
  /** Null when the check cannot be computed (never a pass). */
  pass: boolean | null;
  reason: string | null;
  offenders: { pid: string; title: string | null }[];
  note: string | null;
}

/** `GET /api/plans/{pid}/schedule-health`. */
export interface ScheduleHealth {
  name: string;
  disclaimer: string;
  preset: string;
  thresholds: { values: Record<string, number>; source: string };
  plan_pid: string;
  as_of: string;
  calendar: string;
  baseline: { version: number; taken: string } | null;
  tasks: number;
  checks: HealthCheck[];
  passed: number;
  failed: number;
  not_computable: number;
  all_pass: boolean;
  note: string;
}

export interface LevelingRun {
  person: string;
  person_ref: string | null;
  masked?: boolean;
  from: string;
  to: string;
  demand: number | null;
  capacity: number | null;
  tasks: { pid: string; title: string | null }[];
  reason?: string | null;
}

export interface LevelingMove {
  pid: string;
  title: string | null;
  person: string;
  person_ref: string | null;
  masked?: boolean;
  from_start: string;
  to_start: string;
  scheduled_start: string;
  from_finish: string;
  to_finish: string;
  delay_days: number;
  critical: boolean;
  float_before: number;
  float_after: number;
  because_of: string;
}

/** `GET /api/plans/{pid}/leveling` (advice only). */
export interface LevelingProposal {
  advisory: boolean;
  calendar: string;
  allow_critical: boolean;
  load_percent: number;
  moves: LevelingMove[];
  effects: { pid: string; title: string | null; moved: string; start_days: number; finish_days: number }[];
  over_allocated: LevelingRun[];
  unresolved: LevelingRun[];
  finish_before: string | null;
  finish_after: string | null;
  finish_delay_days: number;
  critical_moved: boolean;
  truncated: boolean;
  note: string;
}

export interface LevelingApplyResult {
  requested: number;
  ok: number;
  refused: number;
  results: { pid: string; result: "ok" | "refused"; note: string | null }[];
  finish_before: string | null;
  finish_after: string | null;
  finish_delay_days: number;
}

export interface ResourceRow {
  key: string;
  demand_fte: number;
  named_fte: number;
  placeholder_fte: number;
  proposed_fte: number;
  /** Null for skills: supply is not known for them. */
  supply_fte: number | null;
  gap_fte: number | null;
  headcount: number;
}

export interface ResourcePeriod {
  from: string;
  to: string;
  days: number;
  rows: ResourceRow[];
  total_demand_fte: number;
  total_placeholder_fte: number;
  total_proposed_fte: number;
}

/** `GET /api/resource-plan`. */
export interface ResourcePlan {
  scope: { kind: string; name?: string };
  from: string;
  to: string;
  period: "week" | "month";
  group_by: "role" | "skill";
  overlapping: boolean;
  periods: ResourcePeriod[];
  placeholders: { allocation_pid: string; plan_pid: string; role: string | null; percent: number; start_date: string | null; end_date: string | null; booking: string }[];
  people_by_role: Record<string, string[]> | null;
  masked: boolean;
  note: string;
}

export interface RiskTask {
  pid: string;
  title: string | null;
  optimistic: number;
  likely: number;
  pessimistic: number;
  source: "estimated" | "fallback" | "milestone";
  widened: boolean;
  criticality_bp: number;
  sensitivity: number | null;
}

/** `GET /api/plans/{pid}/schedule-risk`. */
export interface ScheduleRisk {
  plan_pid: string;
  calendar: string;
  assumptions: {
    seed: number;
    iterations: number;
    max_iterations: number;
    distribution: "triangular" | "pert";
    fallback_band_percent: number;
    correlation: string;
    independence: string;
    durations: string;
    tasks_using_fallback: string[];
    fallback: string;
  };
  cpm_finish: string | null;
  percentiles: { p: number; finish: string }[];
  cpm_finish_confidence_bp: number;
  earliest: string | null;
  latest: string | null;
  histogram: { date: string; count: number }[];
  tasks: RiskTask[];
  events: {
    used: { risk_pid: string; title: string; task_pid: string; probability_percent: number; impact_days: number }[];
    skipped: { risk_pid: string; title: string; reason: string }[];
    risks_without_schedule_fields: number;
  };
  unscheduled: { pid: string; title: string | null; reason: string }[];
  note: string;
}

// ---- Cost-risk and joint cost-and-schedule Monte Carlo ----

export interface CostLineResult {
  pid: string;
  plan: string;
  description?: string;
  category?: string;
  low: number;
  likely: number;
  high: number;
  actual: number;
  source: "estimated" | "fallback";
  widened: boolean;
  sensitivity: number | null;
}

export interface CostCurrencyResult {
  currency: string;
  budget_minor: number;
  spent_minor: number;
  mean_minor: number;
  percentiles: { p: number; minor: number }[];
  budget_confidence_bp: number;
  min_minor: number;
  max_minor: number;
  histogram: { from_minor: number; to_minor: number; count: number }[];
  lines: CostLineResult[];
  events: { risk: string; plan: string; title: string | null; probability_bp: number; impact_minor: number; hit_bp: number }[];
  plans: { plan: string; name: string | null; budget_minor: number; mean_minor: number }[];
}

export interface CostRiskAssumptions {
  seed: number;
  iterations: number;
  max_iterations: number;
  distribution: "triangular" | "pert";
  fallback_band_percent: number;
  correlation: string;
  independence: string;
  floor: string;
  currencies: string;
  budget: string;
  fallback: string;
  lines_using_fallback?: string[];
}

/** `GET /api/plans/{pid}/cost-risk` and `GET /api/cost-risk`. */
export interface CostRisk {
  scope: { kind: "plan" | "portfolio"; plans: number; plan_pid?: string };
  assumptions: CostRiskAssumptions;
  currencies: CostCurrencyResult[];
  events: {
    used: { risk_pid: string; plan_pid: string; title: string; probability_percent: number; cost_impact_minor: number; currency: string; also_a_schedule_event: boolean }[];
    skipped: { risk_pid: string; title: string; reason: string }[];
    risks_without_cost_fields: number;
  };
  note: string;
}

/** `GET /api/plans/{pid}/joint-risk`. */
export interface JointRisk {
  plan_pid: string;
  currency: string;
  other_currencies_not_simulated: string[];
  calendar: string;
  assumptions: CostRiskAssumptions & {
    delay_cost: { per_day_minor: number; currency: string; rule: string };
    shared_events: string[];
    shared_events_rule: string;
    durations?: string;
  };
  schedule: {
    cpm_finish: string | null;
    percentiles: { p: number; finish: string }[];
    earliest: string | null;
    latest: string | null;
    tasks: RiskTask[];
  };
  cost: {
    budget_minor: number;
    spent_minor: number;
    mean_minor: number;
    mean_delay_cost_minor: number;
    percentiles: { p: number; minor: number }[];
    lines: CostLineResult[];
  };
  confidence: { budget_bp: number; finish_bp: number; joint_bp: number };
  quadrants: { both: number; budget_only: number; time_only: number; neither: number };
  points: { days_late: number; cost_minor: number }[];
  events: {
    schedule: { used: unknown[]; skipped: { risk_pid: string; title: string; reason: string }[]; risks_without_schedule_fields: number };
    cost: { used: unknown[]; skipped: { risk_pid: string; title: string; reason: string }[]; risks_without_cost_fields: number };
  };
  note: string;
}

// ---- Milestones as a concept (portfolio view) ----

export type MilestoneStatus = "achieved" | "late" | "due_soon" | "upcoming";
export type MilestoneScheduleState = "on_track" | "at_risk" | "forecast_late" | "no_forecast";
export type MilestoneMode = "manual" | "linked";

export interface MilestoneSchedule {
  /** Null once achieved. */
  state: MilestoneScheduleState | null;
  forecast: string | null;
  /** Working days between the forecast and the due date; negative is late. */
  slack_days: number | null;
  /** Why there is no forecast (or `achieved`). */
  reason: string | null;
  partial: boolean;
  unforecast_tasks: string[];
}

export interface MilestoneLink {
  task_pid: string;
  title: string;
  complete: boolean;
  early_finish: string | null;
  total_float: number | null;
}

export interface MilestoneSlip {
  original_due: string;
  reschedules: number;
  later_moves: number;
  /** Calendar days from the original due date to now. */
  net_days: number;
}

/** A milestone with everything derived on read. */
export interface MilestoneView {
  pid: string;
  plan_pid: string;
  name: string;
  kind: string;
  due: string;
  completion_mode: MilestoneMode;
  achieved: boolean;
  achieved_by: "manual" | "tasks" | null;
  achieved_on: string | null;
  done_reason: string | null;
  status: MilestoneStatus;
  schedule: MilestoneSchedule;
  linked_tasks: number;
  open_tasks: number;
  links: MilestoneLink[];
  slip: MilestoneSlip;
  /** Only on `GET /api/milestones/{pid}`. */
  history?: { from: string; to: string; reason: string | null; actor: string | null; at: string }[];
}

/** `GET /api/milestones`. */
export interface MilestoneList {
  as_of: string;
  due_soon_days: number;
  at_risk_days: number;
  milestones: MilestoneView[];
  summary: {
    by_status: Record<MilestoneStatus, number>;
    by_schedule_state: Partial<Record<MilestoneScheduleState, number>>;
  };
  note: string;
}

export interface Allocation {
  pid: string;
  person_ref: string;
  role: string | null;
  percent: number;
  start_date: string | null;
  end_date: string | null;
  /** `committed` (firm) or `proposed` (hoped for; never counts toward utilisation). PJ-4. */
  booking: "committed" | "proposed";
  /** Asked of the resource manager. */
  requested: boolean;
}

/** A run of days where demand exceeded the capacity in force (PJ-3). */
export interface OverCapacitySpan {
  from: string;
  to: string;
  /** Summed percent asked for on each of those days. */
  demand: number;
  /** The capacity in force on each of those days. */
  capacity: number;
}

/** One availability range (PJ-3): the most of their time a person can commit between two dates. */
export interface AvailabilityRange {
  pid: string;
  person_ref: string;
  from: string;
  to: string;
  max_percent: number;
  note: string | null;
}

export interface CapacityView {
  from: string;
  to: string;
  people: {
    person_ref: string;
    /** Committed bookings only; a proposal never counts here. */
    allocated_percent: number;
    over_allocated: boolean;
    allocations: number;
    /** `default` (100) or `availability` (a range applies in the window). */
    capacity: { source: "default" | "availability"; min_percent: number };
    peak_percent: number;
    over_capacity: OverCapacitySpan[];
    /** The hoped-for bookings, kept apart from the firm figures. */
    proposed: { percent: number; allocations: number; would_exceed_capacity: boolean };
  }[];
}

export interface ReportDefinition {
  pid: string;
  name: string;
  collection: Collection;
  filters: Record<string, unknown>;
  fields: string[] | unknown;
}

export interface ReportRun {
  name: string;
  collection: string;
  rows: number;
  data: Record<string, string>[];
}

export interface Dashboard {
  as_of: string;
  collections: {
    collection: string;
    total: number;
    rag: { red: number; amber: number; green: number };
    stages: Record<string, number>;
  }[];
  site_tiles: {
    plans: number;
    proposals_open: number;
    materialised_risks: number;
    open_risk_exposure: number;
    schedule_violations: number;
    over_allocated_people: number;
  };
}

export interface Scenario {
  pid: string;
  name: string;
  description: string | null;
  status: "draft" | "committed";
  budget_cap_minor: number | null;
  currency: string | null;
}

export interface ScenarioEvaluation {
  pid: string;
  name: string;
  status: string;
  feasible: boolean;
  evaluation: {
    planned_by_currency: [string, number][];
    total_exposure: number;
    total_alignment: number;
    violations: string[];
  };
}

export interface Objective {
  pid: string;
  title: string;
  description: string | null;
  period: string | null;
}

// ---- OKRs (the portfolio view) and KPIs ----

export type KeyResultMetric = "number" | "percent" | "currency" | "boolean";
export type KeyResultDirection = "increase" | "decrease" | "maintain";

export interface KeyResult {
  pid: string;
  objective_pid: string;
  title: string;
  metric: KeyResultMetric;
  direction: KeyResultDirection;
  start_value: number;
  target_value: number;
  current_value: number;
  tolerance: number | null;
  unit: string | null;
  currency: string | null;
  owner_ref: string | null;
  due_date: string | null;
}

export interface OkrKeyResult {
  key_result: KeyResult;
  /** Null with `unmeasurable` saying why; never a zero standing in for "unknown". */
  progress_basis_points: number | null;
  unmeasurable: string | null;
  latest_check_in: {
    observed_at: string;
    value: number;
    /** 0-100; recorded beside the score, never blended into it. */
    confidence: number | null;
    note: string | null;
  } | null;
}

export interface OkrObjective {
  objective: Objective;
  score_basis_points: number | null;
  measured: boolean;
  key_results: OkrKeyResult[];
  single_currency: boolean;
}

/** `GET /api/okrs`. */
export interface OkrList {
  period: string | null;
  objectives: OkrObjective[];
  as_of: string;
  note: string;
}

export type KpiStatus = "green" | "amber" | "red" | "no_data";
export type KpiDirection = "higher_better" | "lower_better";
export type KpiCadence = "daily" | "weekly" | "monthly" | "quarterly";
export type KpiTrend = "improving" | "worsening" | "flat";

export interface Kpi {
  pid: string;
  plan_pid: string | null;
  objective_pid: string | null;
  name: string;
  description: string | null;
  unit: string | null;
  direction: KpiDirection;
  target_value: number;
  red_value: number;
  cadence: KpiCadence | null;
  owner_ref: string | null;
}

export interface KpiReading {
  pid: string;
  observed_on: string;
  value: number;
  note: string | null;
  actor: string | null;
}

/** A KPI with everything derived on read. */
export interface KpiView {
  kpi: Kpi;
  status: KpiStatus;
  latest: { value: number; observed_on: string; note: string | null } | null;
  trend: KpiTrend | null;
  /** Null when the KPI has no cadence. */
  stale: boolean | null;
  readings: number;
  /** Only on `GET /api/kpis/{pid}`. */
  history?: KpiReading[];
}

export interface KpiSummary {
  green: number;
  amber: number;
  red: number;
  no_data: number;
}

/** `GET /api/kpis`. */
export interface KpiList {
  as_of: string;
  kpis: KpiView[];
  summary: KpiSummary;
  note: string;
}

export interface Alignment {
  objective_pid: string;
  title: string;
  period: string | null;
  items: { pid: string; kind: string; name: string; weight: number }[];
  weight_by_collection: Record<string, number>;
  total_weight: number;
}

export interface BenefitBoard {
  benefits: {
    pid: string;
    title: string;
    category: string;
    currency: string | null;
    target_minor: number | null;
    realized_minor: number;
    target_note: string | null;
    status: string;
    expected_on: string | null;
  }[];
  totals: {
    currency: string;
    target_minor: number;
    realized_minor: number;
    spend_minor: number;
    roi_basis_points: number | null;
  }[];
}

/** Format integer minor units as a major-unit string (e.g. 1_234_50 → "1,234.50"). */
export function money(minor: number, currency?: string | null): string {
  const sign = minor < 0 ? "-" : "";
  const abs = Math.abs(minor);
  const major = Math.floor(abs / 100).toLocaleString("en-GB");
  const cents = String(abs % 100).padStart(2, "0");
  return `${sign}${major}.${cents}${currency ? ` ${currency}` : ""}`;
}

/** The PPM endpoints, bound to the configured base URL. */

// ---- executive insight areas (CEO / CFO / CTO derived views) ----

/** `{pid, name, kind}` reference carried by the insight views. */
export interface InsightItemRef {
  pid: string;
  name: string;
  kind: string;
}

/** One per-currency variance row (minor units; currencies never merge). */
export interface VarianceRow {
  currency: string;
  planned_minor: number;
  actual_minor: number;
  remaining_minor: number;
  overrun: boolean;
  line_count: number;
}

export type AnnouncementSeverity = "info" | "important" | "urgent";

/** One announcement (`GET /api/announcements`). */
export interface Announcement {
  pid: string;
  title: string;
  body: string;
  severity: AnnouncementSeverity;
  pinned: boolean;
  publish_at: string;
  expires_at: string | null;
  author_ref: string | null;
  state: "scheduled" | "live" | "expired";
}

export interface AnnouncementList {
  announcements: Announcement[];
  total: number;
  truncated: boolean;
  as_of: string;
}

export interface AnnouncementInput {
  title: string;
  body: string;
  severity: AnnouncementSeverity;
  pinned: boolean;
  publish_at?: string | null;
  expires_at?: string | null;
}

export type SchedulingRule = "fixed_units" | "fixed_duration" | "fixed_work";
export type Contour = "flat" | "front_loaded" | "back_loaded" | "bell" | "turtle" | "custom";
export type Grain = "day" | "week" | "month";

export interface AssignmentBody {
  rule?: SchedulingRule;
  duration_days?: number;
  units_percent?: number;
  work_minutes?: number;
}

export interface AssignmentResult {
  task_pid: string;
  rule: SchedulingRule;
  duration_days: number;
  units_percent: number;
  work_minutes: number;
  derived: "duration" | "units" | "work" | null;
  exact: boolean | null;
  capacity: {
    checked: boolean;
    reason?: string;
    over_capacity?: boolean;
    days_over?: number;
    from?: string;
    to?: string;
  };
  note: string;
}

export interface Bucket {
  from: string;
  to: string;
  minutes: number;
}

export interface WorkProfile {
  plan_pid: string;
  from?: string;
  to?: string;
  period?: Grain;
  tasks: Array<{
    pid: string;
    title: string;
    contour: Contour;
    work_minutes: number;
    buckets: Bucket[];
    placed: boolean;
    assignee?: string | null;
  }>;
  totals: {
    buckets: Bucket[] | null;
    work_minutes: number;
    placed_work_minutes?: number;
    unplaced_work_minutes?: number;
    outside_window_minutes?: number;
  };
  unplaced: Array<{ pid: string; title: string; work_minutes: number; reason: string }>;
  reason?: string;
  note: string;
}

export interface CostRateTable {
  pid: string;
  name: string;
  person_ref: string | null;
  role: string | null;
  currency: string;
  rates: Array<{ effective_from: string; per_hour_minor: number }>;
}

export interface TaskCost {
  pid: string;
  kind: "material" | "fixed";
  description: string;
  amount_minor: number;
  currency: string;
  accrual: "start" | "prorated" | "end";
}

export interface MoneyBuckets {
  total_minor: number;
  buckets: Array<{ from: string; to: string; minor: number }>;
}

export interface CostPlan {
  plan_pid: string;
  from?: string;
  to?: string;
  currencies: Array<{ currency: string; labour: MoneyBuckets; fixed: MoneyBuckets; total: MoneyBuckets }>;
  unrated: Array<{ pid: string; title: string; work_minutes: number; reason: string }>;
  unassigned: Array<{ pid: string; title: string; work_minutes: number; reason: string }>;
  unplaced: Array<{ cost_pid: string; task_pid: string; amount_minor: number; currency: string; reason: string }>;
  reason?: string;
  note: string;
}

export interface LabourCosts {
  currencies: Array<{ currency: string; cost_minor: number; minutes: number; entries: number }>;
  unpriced: { entries: number; minutes: number; reason: string | null };
  note: string;
}

/** `GET /api/executive/health` response. */
export interface ExecutiveHealth {
  as_of: string;
  derivation: string;
  portfolios: Array<{
    portfolio: InsightItemRef | null;
    status: "red" | "amber" | "green";
    rag: { red: number; amber: number; green: number };
    members: number;
    overdue_milestones: number;
    escalated_risks: number;
    open_risk_exposure: number;
    overrun_currencies: string[];
    days_since_last_update: number;
  }>;
}

/** One decision-log entry (`GET /api/executive/decisions`). */
export interface DecisionEntry {
  kind: "gate_review" | "scenario_commit" | "proposal" | "merge";
  at: string;
  decision: string;
  subject: { pid: string; name: string | null };
  actor?: string | null;
  gate?: string;
  sponsor?: string | null;
  merged_from?: string;
  reason?: string | null;
  conditions?: string | null;
}

/** `GET /api/executive/benefits` response. */
export interface ExecutiveBenefits {
  as_of: string;
  note: string;
  portfolios: Array<{
    portfolio: InsightItemRef | null;
    benefits: number;
    non_financial: number;
    statuses: Record<string, number>;
    financial: Array<{
      currency: string;
      target_minor: number;
      realized_minor: number;
      realization_ratio: number | null;
    }>;
  }>;
}

/** `GET /api/financials/variance` response. */
export interface FinancialVariance {
  as_of: string;
  note: string;
  by_collection: Array<{ collection: string; variance: VarianceRow[] }>;
  by_category: Array<{ category: string; variance: VarianceRow[] }>;
  by_portfolio: Array<{
    portfolio: InsightItemRef | null;
    variance: VarianceRow[];
  }>;
}

/** `GET /api/financials/exposure` response. */
export interface FinancialExposure {
  as_of: string;
  note: string;
  currencies: Array<VarianceRow & { plans: number }>;
}

/** `GET /api/technology/dependency-risk` response. */
export interface DependencyRisk {
  as_of: string;
  edges: number;
  top_fan_out: Array<{
    item: InsightItemRef;
    dependents: number;
    rag: string | null;
  }>;
  cross_portfolio: Array<{
    edge: string;
    predecessor: InsightItemRef;
    successor: InsightItemRef;
  }>;
  red_predecessor_edges: Array<{
    edge: string;
    predecessor: InsightItemRef;
    successor: InsightItemRef;
  }>;
}

/** `GET /api/technology/radar` response. */
export interface TechnologyRadar {
  as_of: string;
  convention: string;
  technologies: Array<{
    technology: string;
    ring: "assess" | "trial" | "adopt" | "hold" | "unclassified";
    ring_votes: number;
    per_collection: Record<string, number>;
    items: InsightItemRef[];
  }>;
}

/** `GET /api/executive/alignment` response. */
export interface AlignmentCoverage {
  as_of: string;
  derivation: string;
  by_collection: Array<{
    collection: string;
    total: number;
    aligned: number;
    unaligned: number;
  }>;
  unaligned_spend: VarianceRow[];
  unaligned_items: Array<{
    item: InsightItemRef;
    planned: Array<{ currency: string; planned_minor: number }>;
  }>;
}

/** `GET /api/technology/debt` response. */
export interface TechDebtRegister {
  as_of: string;
  note: string;
  open_exposure: number;
  statuses: Record<string, number>;
  register: Array<{
    pid: string;
    title: string;
    status: string;
    exposure: number;
    escalated: boolean;
    owner_ref: string | null;
    item: InsightItemRef | null;
  }>;
}

/** `GET /api/technology/flow` response. */
export interface FlowMetrics {
  as_of: string;
  window_months: number;
  derivation: string;
  throughput_by_month: Record<string, number>;
  timed_completions: number;
  median_lead_days: number | null;
  undated_completions: number;
}

/** `GET /api/scenarios/compare` response. */
export interface ScenarioComparison {
  a: ScenarioSide;
  b: ScenarioSide;
  deltas: {
    planned_by_currency: Array<{
      currency: string;
      a_minor: number;
      b_minor: number;
      delta_minor: number;
    }>;
    exposure: number;
    alignment: number;
  };
  note: string;
}

/** One side of a scenario comparison. */
export interface ScenarioSide {
  pid: string;
  name: string;
  status: string;
  feasible: boolean;
  evaluation: {
    planned_by_currency: Array<[string, number]>;
    total_exposure: number;
    total_alignment: number;
    violations: string[];
  };
}

/** A risk-register row shared by the compliance / security views. */
export interface RegisterRow {
  pid: string;
  title: string;
  status: string;
  exposure: number;
  escalated: boolean;
  owner_ref: string | null;
  item: InsightItemRef | null;
}

/** A category risk register (`/compliance/register`, `/security/register`). */
export interface RiskRegister {
  as_of: string;
  note: string;
  open_exposure: number;
  statuses: Record<string, number>;
  register: RegisterRow[];
  unreviewed_at_late_stage?: {
    heuristic: string;
    items: Array<{ item: InsightItemRef; stage: string | null }>;
  };
}

/** `GET /api/board/pack` response. */
export interface BoardPack {
  as_of: string;
  window: { from: string; to: string };
  health_now: { portfolios: Record<string, number>; note: string };
  decisions: DecisionEntry[];
  benefits_realized: {
    events: number;
    per_currency_minor: Record<string, number>;
    unattributed_events: number;
  };
  milestones_completed: number;
  tranches_released: { count: number; per_currency: VarianceRow[] };
}

/** One board investment entry. */
export interface Investment {
  kind: "scenario_commit" | "tranche_release" | "proposal";
  at: string;
  name?: string;
  title?: string;
  description?: string;
  gate?: string | null;
  decision?: string;
  planned_minor?: number;
  requested_minor?: number | null;
  budget_cap_minor?: number | null;
  currency?: string | null;
  item?: InsightItemRef | null;
}

/** `GET /api/board/trends` response. */
export interface TrendSeries {
  as_of: string;
  note: string;
  series: Array<{ taken_at: string; body: Record<string, unknown> }>;
}

/** `GET /api/auditor/trail` response. */
export interface AuditTrail {
  as_of: string;
  returned: number;
  stats: {
    per_day: Record<string, number>;
    distinct_actors: number;
    actorless: number;
  };
  rows: Array<{
    created_at: string;
    actor: string | null;
    action: string;
    entity_pid: string;
  }>;
}

/** `GET /api/auditor/findings` response. */
export interface AuditorFindings {
  as_of: string;
  note: string;
  findings: Array<Record<string, unknown> & { rule: string; detail: string }>;
  actorless_actions: number;
}

/** `GET /api/compliance/findings` response. */
export interface ConformanceFindings {
  as_of: string;
  review_days: number;
  findings: Array<Record<string, unknown> & { rule: string; detail: string }>;
}

/** `GET /api/risk/heatmap` response. */
export interface RiskHeatmap {
  as_of: string;
  open_risks: number;
  estate_open_exposure: number;
  cells: Record<string, number>;
  top_risks: Array<{
    pid: string;
    title: string;
    exposure: number;
    category: string;
    item: InsightItemRef | null;
  }>;
  posture: Array<{
    portfolio: InsightItemRef | null;
    open_exposure: number;
    escalated: number;
    materialised: number;
  }>;
  concentration: Array<{ pid: string; title: string; exposure: number }>;
  overdue_reviews: Array<{
    pid: string;
    title: string;
    review_date: string | null;
  }>;
  appetite: {
    max_open_exposure: number | null;
    max_item_exposure: number | null;
  } | null;
  appetite_note: string;
  breaches: Array<Record<string, unknown> & { rule: string }>;
}

/** `GET /api/regulator/extract` response. */
export interface RegulatorExtract {
  as_of: string;
  masked: boolean;
  note: string;
  portfolios: Array<{
    pid: string;
    name: string | null;
    stage: string | null;
    members: Record<string, number>;
    gate_decisions: Record<string, number>;
    spend: Array<{
      currency: string;
      planned_minor: number;
      actual_minor: number;
    }>;
    benefits: Array<{
      currency: string;
      target_minor: number;
      realized_minor: number;
    }>;
  }>;
}

/** One task (the per-item Kanban sub-resource). */
export interface Task {
  pid: string;
  title: string;
  description: string | null;
  status: "todo" | "in_progress" | "in_review" | "done" | "blocked";
  assignee_ref: string | null;
  points: number | null;
  /** Due calendar date (`YYYY-MM-DD`, no zone), or null. */
  due_date: string | null;
  sprint_pid: string | null;
  created_at: string;
  status_changed_at: string;
  done_at: string | null;
  blocked_days: number | null;
}

/** One task in `GET /api/plans/{pid}/cpm` (SC-1, SC-2). Dates are `YYYY-MM-DD`. */
export interface CpmNode {
  pid: string;
  title: string | null;
  status: string | null;
  early_start: string;
  early_finish: string;
  late_start: string;
  late_finish: string;
  /** Working days it can slip before the project finish moves; negative = cannot meet a constraint or deadline. */
  total_float: number;
  /** Working days before any successor is delayed. */
  free_float: number;
  critical: boolean;
  near_critical: boolean;
  planned_start: string | null;
  /** The stored working-day duration (0 = milestone). */
  duration_days: number | null;
  /** Optimistic and pessimistic duration for schedule risk; the likely one is `duration_days`. */
  duration_optimistic_days?: number | null;
  duration_pessimistic_days?: number | null;
  /** Working days logic forces the start later than planned. */
  pushed_days: number;
  constraint: "snet" | "snlt" | "fnet" | "fnlt" | "mso" | "mfo" | null;
  /** PJ-5 / PJ-6: work in minutes, units as a percent of the assignee's day, and the rule and contour. */
  work_minutes?: number | null;
  units_percent?: number | null;
  scheduling_rule?: SchedulingRule | null;
  contour?: Contour | null;
  constraint_date: string | null;
  deadline: string | null;
  /** A hard constraint pins this task earlier than its predecessors allow. */
  hard_conflict: boolean;
  /** `plan`, or `plan+person` when the assignee's own calendar narrows it. */
  calendar: "plan" | "plan+person";
}

/** A dependency between two tasks. */
export interface CpmEdge {
  pid: string;
  predecessor_pid: string;
  successor_pid: string;
  kind: "fs" | "ss" | "ff" | "sf";
  /** Working days; negative is a lead. */
  lag_days: number;
}

/** `GET /api/plans/{pid}/cpm`. */
export interface Cpm {
  plan_pid: string;
  /** `plan` or `default` (all seven days work). */
  calendar: "plan" | "default";
  calendar_conflicts: string[];
  project_finish: string | null;
  nodes: CpmNode[];
  /** The dependencies used, so a diagram can draw them. */
  edges: CpmEdge[];
  unscheduled: { pid: string; title: string | null; reason: "no_duration" | "no_anchor" | "date_out_of_range" }[];
  ignored_edges: string[];
  critical_chains: string[][];
  chains_truncated: boolean;
  note: string;
}

/** A schedule baseline (SC-3): a numbered, append-only snapshot. */
export interface ScheduleBaseline {
  pid: string;
  version: number;
  reason: string;
  created_by: string | null;
  created_at: string;
  task_count: number;
  project_finish: string | null;
}

/** How a task compares with a baseline. */
export type VarianceStanding = "slipped" | "pulled_in" | "on_baseline" | "added" | "removed" | "unmeasurable";

export interface VarianceRow {
  task_pid: string;
  title: string;
  standing: VarianceStanding;
  baseline_start: string | null;
  baseline_finish: string | null;
  start: string | null;
  finish: string | null;
  /** Working days; positive is later; null when no variance can be stated. */
  start_variance_days: number | null;
  finish_variance_days: number | null;
  duration_variance_days: number | null;
  reason?: string;
}

/** `GET /api/plans/{pid}/schedule-variance`. */
export interface ScheduleVariance {
  /** `null` when no baseline has been taken. */
  baseline: ScheduleBaseline | null;
  reason?: "no_baseline";
  note: string;
  variance: {
    rows: VarianceRow[];
    summary: {
      baseline_tasks: number;
      current_tasks: number;
      matched: number;
      slipped: number;
      pulled_in: number;
      on_baseline: number;
      unmeasurable: number;
      added: number;
      removed: number;
      /** Working days the plan's own finish moved; null when either side has no finish. */
      plan_finish_slip_days: number | null;
    };
  } | null;
}

/** What is incomplete on a RACI row. */
export type RaciFinding = "no_accountable" | "no_responsible" | "many_accountable";

/** `GET /api/plans/{pid}/raci`. */
export interface RaciMatrix {
  plan_pid: string;
  /** Everyone who holds a letter anywhere, sorted. */
  people: string[];
  rows: { task_pid: string; title: string; cells: Record<string, string>; findings: RaciFinding[] }[];
  summary: { tasks: number; complete: number; missing_accountable: number; multiple_accountable: number; missing_responsible: number };
  workload: { person_ref: string; responsible: number; accountable: number; consulted: number; informed: number }[];
}

/** A basis-point ratio (10 000 = 1.0), or why there is none. */
export interface EvRatio {
  bp: number | null;
  absent?: "no_planned_value_yet" | "no_actuals_recorded" | "no_planned_value";
}

/** One currency's earned-value figures (SC-4). Money is minor units. */
export interface EvBlock {
  currency: string;
  baseline_version: number;
  bac_minor: number;
  pv_minor: number;
  ev_minor: number;
  /** `null` when actual cost cannot be stated. */
  ac_minor: number | null;
  sv_minor: number;
  cv_minor: number | null;
  spi: EvRatio;
  cpi: EvRatio;
  earned_schedule_days: number | null;
  actual_days: number;
  spi_t: EvRatio;
}

/** `GET /api/plans/{pid}/earned-value`. */
export interface EarnedValue {
  plan_pid: string;
  as_of: string;
  /** `null` with `absent` when there is nothing to show. */
  blocks: EvBlock[] | null;
  absent?: "no_baseline" | "no_weighted_tasks";
  reason?: string;
  percent_complete?: {
    percent_bp: number;
    default_method: "zero_hundred" | "fifty_fifty" | "points";
    weighting: "equal" | "points";
    tasks_counted: number;
    unweighted_tasks: string[];
    manual_tasks: string[];
    /** Any task is counted by a person-entered percent. */
    manual: boolean;
    tasks: { pid: string; weight: number; percent_bp: number; method: "zero_hundred" | "fifty_fifty" | "manual" }[];
  };
  unbaselined_currencies?: string[];
  ac_source?: string;
  note?: string;
}

export type EvRag = "green" | "amber" | "red" | "unknown";

/** An amount, or why there is none (EVM analytics, SC-5). */
export interface EvAmount {
  minor: number | null;
  absent?: "no_actual_cost" | "undefined_denominator" | "no_bottom_up_estimate" | "budget_exhausted" | "no_remaining_cost";
}

export interface EvTcpi {
  bp: number | null;
  absent?: EvAmount["absent"];
  needs_improved_efficiency: boolean;
}

/** `GET /api/plans/{pid}/earned-value/forecast`. */
export interface EvForecast {
  plan_pid: string;
  as_of: string;
  /** `null` with `absent` when there is nothing to forecast against. */
  forecasts: {
    currency: string;
    baseline_version: number;
    spi: { bp: number | null; rag: EvRag };
    cpi: { bp: number | null; rag: EvRag };
    variants: { variant: "budget_rate" | "cpi" | "cpi_spi" | "bottom_up"; formula: string; eac: EvAmount; etc: EvAmount; vac: EvAmount }[];
    recommended: { variant: string; why: string; eac_minor: number | null };
    tcpi_to_budget: { tcpi: EvTcpi; rag: EvRag };
    tcpi_to_recommended_eac: { tcpi: EvTcpi; rag: EvRag } | null;
    bac_minor: number;
    ev_minor: number;
    ac_minor: number | null;
  }[] | null;
  absent?: "no_baseline" | "no_weighted_tasks";
  reason?: string;
  thresholds?: { values: Record<string, number>; source: string };
  note?: string;
}

/** `GET /api/plans/{pid}/earned-value/series`. */
export interface EvSeries {
  plan_pid: string;
  curves: {
    currency: string;
    baseline_version: number;
    bac_minor: number;
    snapshots: number;
    /** Earned value and actual cost are null except on the days a snapshot was recorded. */
    points: { date: string; pv_minor: number; ev_minor: number | null; ac_minor: number | null }[];
  }[];
  note: string;
}

/** `GET /api/plans/{pid}/earned-value/rollup`. */
export interface EvRollup {
  root_plan_pid: string;
  as_of: string;
  currencies: {
    currency: string;
    totals: { bac_minor: number; pv_minor: number; ev_minor: number; ac_minor: number; plans: number; plans_without_ac: number };
    spi: { bp: number | null; rag: EvRag };
    cpi: { bp: number | null; rag: EvRag; absent?: string | null };
    eac: { minor: number | null; plans_counted: number; plans_total: number; note: string };
    vac_minor: number | null;
  }[];
  plans_without_figures: number;
  note: string;
}

/** `PUT …/tasks/{pid}/schedule` body; an absent field clears it. */
export interface TaskScheduleBody {
  start_date?: string | null;
  duration_days?: number | null;
  constraint_kind?: string | null;
  constraint_date?: string | null;
  deadline?: string | null;
  /** Required for a hard constraint (`mso`, `mfo`). */
  reason?: string | null;
}

/** What moved across the network after a schedule edit (PJ-8). */
export interface ScheduleMoved {
  task: { pid: string };
  moved: { pid: string; moved: "start" | "finish" | "both"; start_days: number; finish_days: number }[] | null;
  note: string | null;
}

/** A working calendar (SC-2): seven `0`/`1` characters, Monday first, plus holidays. */
export interface WorkCalendarView {
  source: "default" | "plan" | "person";
  weekdays: string;
  holidays: string[];
  note?: string;
}

/** `GET /api/plans/{pid}/modules` (RM-7): the feature modules a plan exposes. */
export interface PlanModules {
  plan_pid: string;
  /** `plan` when someone chose a set for this plan, `default` otherwise. */
  source: "plan" | "default";
  /** The enabled module keys. */
  modules: string[];
  available: Array<{ key: string; enabled: boolean; segments: string[] }>;
  default: string[];
  personal_preset: string[];
}

/** The five due-date buckets of `GET /api/me/tasks`, in display order. */
export const MY_WORK_BUCKETS = [
  "overdue",
  "today",
  "this_week",
  "later",
  "undated",
] as const;
export type MyWorkBucket = (typeof MY_WORK_BUCKETS)[number];

/**
 * A task's own schedule figure (SC-10): the critical-path total float in
 * working days. Only ever the caller's own task; absent (`null` on the task)
 * when the engine has no schedule data for it.
 */
export interface MyTaskSchedule {
  total_float: number;
  critical: boolean;
  late: boolean;
}

/** One task on the "My work" view. */
export interface MyTask {
  pid: string;
  title: string;
  status: string;
  due_date: string | null;
  points: number | null;
  plan_pid: string;
  plan_name: string;
  /** Null when there is no schedule data; `schedule_reason` says why. */
  schedule?: MyTaskSchedule | null;
  schedule_reason?: string | null;
}

/** `GET /api/me/tasks` response (LT-3). Every bucket is always present. */
export interface MyWork {
  tz: string;
  as_of: string;
  /** The caller's local calendar date in `tz`. */
  today: string;
  buckets: Record<MyWorkBucket, MyTask[]>;
  counts: Record<MyWorkBucket, number>;
  total: number;
  /** Tasks left out because their plan is not readable by this caller. */
  omitted_unreadable: number;
  truncated: boolean;
  /** Set (e.g. `sub_not_addressable`) when the buckets are empty for a stated reason. */
  reason: string | null;
}

/** A time-boxed sprint. */
export interface Sprint {
  pid: string;
  name: string;
  starts_on: string;
  ends_on: string;
}

/** `GET .../burndown` response. */
export interface Burndown {
  as_of: string;
  sprint: { pid: string; name: string; starts_on: string; ends_on: string };
  total_tasks: number;
  derivation: string;
  points: Array<{ date: string; remaining: number }>;
}

/** `GET .../standup` response. */
export interface Standup {
  as_of: string;
  since: string;
  item: InsightItemRef;
  tasks_created: Array<{
    at: string;
    task: string | null;
    actor: string | null;
  }>;
  tasks_moved: Array<{
    at: string;
    task: string | null;
    actor: string | null;
    move: unknown;
  }>;
  blocked_now: Task[];
  risks_raised_estate_wide: number;
}

/** `GET /api/engineering/blocked` response. */
export interface BlockedWork {
  as_of: string;
  derivation: string;
  blocked: Array<Task & { item: InsightItemRef | null }>;
}

/** `GET /api/engineering/moscow` response. */
export interface MoscowView {
  as_of: string;
  convention: string;
  bands: Record<string, InsightItemRef[]>;
  untagged: number;
}

/** `GET /api/engineering/delivery-links` response. */
export interface DeliveryLinks {
  as_of: string;
  schemes: string[];
  tracked: Array<{
    item: InsightItemRef;
    links: Array<{ scheme: string; value: string }>;
  }>;
  untracked: InsightItemRef[];
}

/** `GET /api/engineering/milestone-calendar` response. */
export interface MilestoneCalendar {
  as_of: string;
  kinds: string[];
  milestones: Array<{
    pid: string;
    name: string;
    kind: string;
    due: string;
    done: boolean;
    item: InsightItemRef | null;
  }>;
}

/** One sprint retro / feedback note. */
export interface SprintNote {
  pid: string;
  sprint_pid: string;
  category: "went_well" | "improve" | "action" | "feedback";
  body: string;
  task_pid: string | null;
}

/** `GET .../velocity` response. */
export interface Velocity {
  as_of: string;
  note: string;
  sprints: Array<{
    sprint: { pid: string; name: string; starts_on: string; ends_on: string };
    tasks_done: number;
    points_done: number;
    unpointed_done: number;
  }>;
}

/** `GET /api/devops/metrics` response. */
export interface DevopsMetrics {
  as_of: string;
  window_months: number;
  derivation: string;
  deploys: number;
  deploys_by_month: Record<string, number>;
  deploys_by_environment: Record<string, number>;
  incidents: number;
  incidents_by_month: Record<string, number>;
  resolved_incidents: number;
  unresolved_incidents: number;
  median_recovery_hours: number | null;
  declared_cause_incidents: number;
  change_failure_rate: number | null;
}

/** `GET /api/devops/releases` response. */
export interface DevopsReleases {
  as_of: string;
  releases: Array<{
    pid: string;
    occurred_at: string;
    environment: string | null;
    version: string | null;
    reference: string | null;
    item: InsightItemRef | null;
  }>;
}

/** Options shared by the Monte Carlo endpoints. */
export interface SimOptions {
  iterations?: number;
  seed?: number;
  distribution?: "triangular" | "pert";
  fallbackBandPercent?: number;
  currency?: string;
  delayCostPerDayMinor?: number;
}

function simQuery(o: SimOptions): string {
  const q = new URLSearchParams();
  if (o.iterations) q.set("iterations", String(o.iterations));
  if (o.seed !== undefined) q.set("seed", String(o.seed));
  if (o.distribution) q.set("distribution", o.distribution);
  if (o.fallbackBandPercent !== undefined) q.set("fallback_band_percent", String(o.fallbackBandPercent));
  if (o.currency) q.set("currency", o.currency);
  if (o.delayCostPerDayMinor !== undefined) q.set("delay_cost_per_day_minor", String(o.delayCostPerDayMinor));
  const qs = q.toString();
  return qs ? `?${qs}` : "";
}

export class PpmClient {
  constructor(private readonly http: ApiClient) {}

  /** Wire an {@link ApiClient} at {@link API_BASE_URL}. */
  static withFetch(fetchFn?: typeof fetch): PpmClient {
    return new PpmClient(
      new ApiClient({ baseUrl: API_BASE_URL, fetch: fetchFn }),
    );
  }

  // ---- dashboard ----
  dashboard(): Promise<Dashboard> {
    return this.http.get("/api/at-a-glance");
  }

  // ---- proposals (PPM-1) ----
  listProposals(status?: string): Promise<Proposal[]> {
    return this.http.get(`/api/proposals${status ? `?status=${status}` : ""}`);
  }
  createProposal(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/proposals", { body });
  }
  proposalAction(
    pid: string,
    action: "submit" | "review" | "approve" | "reject",
  ): Promise<Proposal> {
    return this.http.post(`/api/proposals/${pid}/${action}`, { body: {} });
  }
  promoteProposal(
    pid: string,
  ): Promise<{ plan_pid: string; kind_target: string | null }> {
    return this.http.post(`/api/proposals/${pid}/promote`, { body: {} });
  }
  proposalDuplicates(pid: string): Promise<DemandHit[]> {
    return this.http.get(`/api/proposals/${pid}/duplicates`);
  }

  // ---- ideas (PPM-2) ----
  listIdeas(status?: string): Promise<Idea[]> {
    return this.http.get(`/api/ideas${status ? `?status=${status}` : ""}`);
  }
  createIdea(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/ideas", { body });
  }
  voteIdea(pid: string): Promise<Idea> {
    return this.http.post(`/api/ideas/${pid}/vote`, { body: {} });
  }
  dismissIdea(pid: string): Promise<Idea> {
    return this.http.post(`/api/ideas/${pid}/dismiss`, { body: {} });
  }
  convertIdea(
    pid: string,
    kindTarget: Collection,
  ): Promise<{ proposal_pid: string }> {
    return this.http.post(`/api/ideas/${pid}/convert`, {
      body: { kind_target: kindTarget },
    });
  }

  // ---- governance panel (PPM-3/10/12 + benefits/objectives) ----
  governance(pid: string): Promise<GovernanceSummary> {
    return this.http.get(`/api/plans/${pid}/governance`);
  }
  gateJourney(pid: string): Promise<GateJourney> {
    return this.http.get(`/api/plans/${pid}/gate-reviews`);
  }
  reviewGate(pid: string, body: unknown): Promise<{ stage: string | null }> {
    return this.http.post(`/api/plans/${pid}/gate-reviews`, { body });
  }
  listRisks(pid: string): Promise<Risk[]> {
    return this.http.get(`/api/plans/${pid}/risks`);
  }
  createRisk(pid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/risks`, { body });
  }
  escalateRisk(pid: string, riskPid: string): Promise<Risk> {
    return this.http.post(`/api/plans/${pid}/risks/${riskPid}/escalate`, {
      body: {},
    });
  }
  budget(pid: string): Promise<BudgetBoard> {
    return this.http.get(`/api/plans/${pid}/budget-lines`);
  }
  createBudgetLine(pid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/budget-lines`, { body });
  }
  recordActual(
    pid: string,
    linePid: string,
    amountMinor: number,
  ): Promise<unknown> {
    return this.http.post(`/api/plans/${pid}/budget-lines/${linePid}/actual`, {
      body: { amount_minor: amountMinor },
    });
  }
  benefits(pid: string): Promise<BenefitBoard> {
    return this.http.get(`/api/plans/${pid}/benefits`);
  }
  createBenefit(pid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/benefits`, { body });
  }
  realizeBenefit(
    pid: string,
    benefitPid: string,
    body: unknown,
  ): Promise<unknown> {
    return this.http.post(`/api/plans/${pid}/benefits/${benefitPid}/realize`, {
      body,
    });
  }
  itemObjectives(
    pid: string,
  ): Promise<{ objective_pid: string; title: string; weight: number }[]> {
    return this.http.get(`/api/plans/${pid}/objectives`);
  }
  linkObjective(
    pid: string,
    objectivePid: string,
    weight: number,
  ): Promise<unknown> {
    return this.http.post(`/api/plans/${pid}/objectives`, {
      body: { objective_pid: objectivePid, weight },
    });
  }
  milestones(pid: string): Promise<Milestone[]> {
    return this.http.get(`/api/plans/${pid}/milestones`);
  }
  createMilestone(pid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/milestones`, { body });
  }
  /** A linked milestone with open tasks needs a `reason` (422 without one). */
  completeMilestone(pid: string, milestonePid: string, reason?: string): Promise<unknown> {
    return this.http.post(
      `/api/plans/${pid}/milestones/${milestonePid}/complete`,
      { body: reason ? { reason } : {} },
    );
  }
  portfolioMilestones(params: { status?: MilestoneStatus; plan?: string; kind?: string } = {}): Promise<MilestoneList> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/milestones${qs ? `?${qs}` : ""}`);
  }
  milestone(pid: string): Promise<MilestoneView> {
    return this.http.get(`/api/milestones/${pid}`);
  }
  rescheduleMilestone(pid: string, milestonePid: string, due: string, reason?: string): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/milestones/${milestonePid}`, { body: { due, reason: reason || undefined } });
  }
  reopenMilestone(pid: string, milestonePid: string, reason: string): Promise<unknown> {
    return this.http.post(`/api/plans/${pid}/milestones/${milestonePid}/reopen`, { body: { reason } });
  }
  setMilestoneLinks(pid: string, milestonePid: string, taskPids: string[], completionMode?: MilestoneMode): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/milestones/${milestonePid}/tasks`, {
      body: { task_pids: taskPids, completion_mode: completionMode },
    });
  }
  allocations(pid: string): Promise<Allocation[]> {
    return this.http.get(`/api/plans/${pid}/allocations`);
  }
  /** Make a proposed booking firm; a 409 `over_capacity` needs `confirm` to go ahead. */
  commitAllocation(pid: string, allocationPid: string, confirm = false): Promise<{ pid: string; booking: string; over_capacity: boolean }> {
    return this.http.post(
      `/api/plans/${pid}/allocations/${allocationPid}/commit${confirm ? "?confirm=true" : ""}`,
    );
  }
  availability(personRef?: string): Promise<AvailabilityRange[]> {
    const q = personRef ? `?person_ref=${encodeURIComponent(personRef)}` : "";
    return this.http.get(`/api/availability${q}`);
  }
  setAvailability(body: { person_ref?: string; from: string; to: string; max_percent: number; note?: string }): Promise<AvailabilityRange> {
    return this.http.post("/api/availability", { body });
  }
  removeAvailability(pid: string): Promise<unknown> {
    return this.http.post(`/api/availability/${pid}/delete`);
  }
  createAllocation(pid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/allocations`, { body });
  }

  // ---- schedule (PPM-6) ----
  schedule(portfolioPid: string): Promise<ScheduleView> {
    return this.http.get(`/api/plans/${portfolioPid}/schedule`);
  }

  // ---- capacity (PPM-8) ----
  capacity(from?: string, to?: string): Promise<CapacityView> {
    const params = new URLSearchParams();
    if (from) params.set("from", from);
    if (to) params.set("to", to);
    const query = params.toString();
    return this.http.get(`/api/capacity${query ? `?${query}` : ""}`);
  }

  // ---- scenarios (PPM-4) ----
  listScenarios(): Promise<Scenario[]> {
    return this.http.get("/api/scenarios");
  }
  createScenario(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/scenarios", { body });
  }
  evaluateScenario(pid: string): Promise<ScenarioEvaluation> {
    return this.http.get(`/api/scenarios/${pid}/evaluate`);
  }
  commitScenario(pid: string): Promise<Scenario> {
    return this.http.post(`/api/scenarios/${pid}/commit`, { body: {} });
  }

  // ---- objectives (PPM-5) ----
  listObjectives(): Promise<Objective[]> {
    return this.http.get("/api/objectives");
  }
  createObjective(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/objectives", { body });
  }
  alignment(pid: string): Promise<Alignment> {
    return this.http.get(`/api/objectives/${pid}/alignment`);
  }

  // ---- scheduling canon (SC-6 .. SC-9) ----
  scheduleHealth(pid: string, asOf?: string): Promise<ScheduleHealth> {
    return this.http.get(`/api/plans/${pid}/schedule-health${asOf ? `?as_of=${asOf}` : ""}`);
  }
  levelingProposal(pid: string, opts: { allowCritical?: boolean; loadPercent?: number } = {}): Promise<LevelingProposal> {
    const q = new URLSearchParams();
    if (opts.allowCritical) q.set("allow_critical", "true");
    if (opts.loadPercent) q.set("load_percent", String(opts.loadPercent));
    const qs = q.toString();
    return this.http.get(`/api/plans/${pid}/leveling${qs ? `?${qs}` : ""}`);
  }
  applyLeveling(
    pid: string,
    body: { moves: { task_pid: string; start_date: string }[]; confirm: boolean; allow_critical?: boolean; load_percent?: number; reason?: string },
  ): Promise<LevelingApplyResult> {
    return this.http.post(`/api/plans/${pid}/leveling/apply`, { body });
  }
  resourcePlan(params: { from?: string; to?: string; period?: "week" | "month"; group_by?: "role" | "skill"; scenario?: string } = {}): Promise<ResourcePlan> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/resource-plan${qs ? `?${qs}` : ""}`);
  }
  scheduleRisk(pid: string, opts: { iterations?: number; seed?: number; distribution?: "triangular" | "pert"; fallbackBandPercent?: number } = {}): Promise<ScheduleRisk> {
    const q = new URLSearchParams();
    if (opts.iterations) q.set("iterations", String(opts.iterations));
    if (opts.seed !== undefined) q.set("seed", String(opts.seed));
    if (opts.distribution) q.set("distribution", opts.distribution);
    if (opts.fallbackBandPercent !== undefined) q.set("fallback_band_percent", String(opts.fallbackBandPercent));
    const qs = q.toString();
    return this.http.get(`/api/plans/${pid}/schedule-risk${qs ? `?${qs}` : ""}`);
  }

  // ---- saved views (the user's own; also holds display preferences) ----
  listSavedViews(route?: string): Promise<{ pid: string; route: string; name: string; filter: unknown }[]> {
    return this.http.get(`/api/saved-views${route ? `?route=${encodeURIComponent(route)}` : ""}`);
  }
  createSavedView(body: { route: string; name: string; filter?: unknown; sort?: unknown; columns?: string[] }): Promise<{ pid: string }> {
    return this.http.post("/api/saved-views", { body });
  }
  deleteSavedView(pid: string): Promise<void> {
    return this.http.delete(`/api/saved-views/${pid}`);
  }

  // ---- simulation inputs ----
  setTaskEstimate(pid: string, taskPid: string, body: { optimistic_days: number | null; pessimistic_days: number | null }): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/estimate`, { body });
  }
  setLineEstimate(pid: string, linePid: string, body: { low_minor: number | null; high_minor: number | null }): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/budget-lines/${linePid}/estimate`, { body });
  }
  setRiskScheduleImpact(pid: string, riskPid: string, body: { task_pid: string | null; impact_days: number | null; probability_percent: number | null }): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/risks/${riskPid}/schedule-impact`, { body });
  }
  setRiskCostImpact(pid: string, riskPid: string, body: { cost_impact_minor: number | null; cost_currency: string | null; probability_percent?: number }): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/risks/${riskPid}/cost-impact`, { body });
  }

  // ---- cost-risk and joint Monte Carlo ----
  costRisk(pid: string, opts: SimOptions = {}): Promise<CostRisk> {
    return this.http.get(`/api/plans/${pid}/cost-risk${simQuery(opts)}`);
  }
  portfolioCostRisk(opts: SimOptions = {}): Promise<CostRisk> {
    return this.http.get(`/api/cost-risk${simQuery(opts)}`);
  }
  jointRisk(pid: string, opts: SimOptions & { currency?: string; delayCostPerDayMinor?: number } = {}): Promise<JointRisk> {
    return this.http.get(`/api/plans/${pid}/joint-risk${simQuery(opts)}`);
  }

  // ---- OKRs ----
  okrs(period?: string): Promise<OkrList> {
    const q = period ? `?period=${encodeURIComponent(period)}` : "";
    return this.http.get(`/api/okrs${q}`);
  }
  updateObjective(pid: string, body: { title: string; description?: string | null; period?: string | null }): Promise<Objective> {
    return this.http.put(`/api/objectives/${pid}`, { body });
  }
  deleteObjective(pid: string): Promise<void> {
    return this.http.delete(`/api/objectives/${pid}`);
  }
  createKeyResult(objectivePid: string, body: unknown): Promise<{ pid: string }> {
    return this.http.post(`/api/objectives/${objectivePid}/key-results`, { body });
  }
  updateKeyResult(pid: string, body: unknown): Promise<KeyResult> {
    return this.http.put(`/api/key-results/${pid}`, { body });
  }
  deleteKeyResult(pid: string): Promise<void> {
    return this.http.delete(`/api/key-results/${pid}`);
  }
  checkIn(keyResultPid: string, body: { value: number; confidence?: number | null; note?: string | null }): Promise<unknown> {
    return this.http.post(`/api/key-results/${keyResultPid}/check-ins`, { body });
  }

  // ---- KPIs ----
  kpis(params: { plan?: string; objective?: string; status?: KpiStatus } = {}): Promise<KpiList> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/kpis${qs ? `?${qs}` : ""}`);
  }
  kpi(pid: string): Promise<KpiView> {
    return this.http.get(`/api/kpis/${pid}`);
  }
  createKpi(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/kpis", { body });
  }
  updateKpi(pid: string, body: unknown): Promise<Kpi> {
    return this.http.put(`/api/kpis/${pid}`, { body });
  }
  deleteKpi(pid: string): Promise<void> {
    return this.http.delete(`/api/kpis/${pid}`);
  }
  addKpiReading(pid: string, body: { value: number; observed_on?: string; note?: string | null }): Promise<{ reading_pid: string; kpi: KpiView }> {
    return this.http.post(`/api/kpis/${pid}/readings`, { body });
  }

  // ---- reports (PPM-9) ----
  listReports(): Promise<ReportDefinition[]> {
    return this.http.get("/api/reports");
  }
  createReport(body: unknown): Promise<{ pid: string }> {
    return this.http.post("/api/reports", { body });
  }
  runReport(pid: string): Promise<ReportRun> {
    return this.http.get(`/api/reports/${pid}/run`);
  }

  // ---- executive insight areas (CEO / CFO / CTO) ----
  announcements(params: { include?: "all"; limit?: number } = {}): Promise<AnnouncementList> {
    const q = new URLSearchParams();
    if (params.include) q.set("include", params.include);
    if (params.limit) q.set("limit", String(params.limit));
    const qs = q.toString();
    return this.http.get(`/api/announcements${qs ? `?${qs}` : ""}`);
  }
  publishAnnouncement(body: AnnouncementInput): Promise<Announcement> {
    return this.http.post("/api/announcements", { body });
  }
  retractAnnouncement(pid: string): Promise<void> {
    return this.http.delete(`/api/announcements/${pid}`);
  }
  executiveHealth(): Promise<ExecutiveHealth> {
    return this.http.get("/api/executive/health");
  }
  executiveDecisions(
    limit?: number,
  ): Promise<{ decisions: DecisionEntry[]; total: number; as_of: string }> {
    return this.http.get(
      `/api/executive/decisions${limit ? `?limit=${limit}` : ""}`,
    );
  }
  executiveBenefits(): Promise<ExecutiveBenefits> {
    return this.http.get("/api/executive/benefits");
  }
  financialVariance(): Promise<FinancialVariance> {
    return this.http.get("/api/financials/variance");
  }
  financialExposure(): Promise<FinancialExposure> {
    return this.http.get("/api/financials/exposure");
  }
  technologyDependencyRisk(): Promise<DependencyRisk> {
    return this.http.get("/api/technology/dependency-risk");
  }
  technologyRadar(): Promise<TechnologyRadar> {
    return this.http.get("/api/technology/radar");
  }
  executiveAlignment(): Promise<AlignmentCoverage> {
    return this.http.get("/api/executive/alignment");
  }
  technologyDebt(): Promise<TechDebtRegister> {
    return this.http.get("/api/technology/debt");
  }
  technologyFlow(months?: number): Promise<FlowMetrics> {
    return this.http.get(
      `/api/technology/flow${months ? `?months=${months}` : ""}`,
    );
  }
  compareScenarios(a: string, b: string): Promise<ScenarioComparison> {
    return this.http.get(
      `/api/scenarios/compare?a=${encodeURIComponent(a)}&b=${encodeURIComponent(b)}`,
    );
  }
  boardPack(from?: string, to?: string): Promise<BoardPack> {
    const params = new URLSearchParams();
    if (from) params.set("from", from);
    if (to) params.set("to", to);
    const query = params.toString();
    return this.http.get(`/api/board/pack${query ? `?${query}` : ""}`);
  }
  boardInvestments(): Promise<{ investments: Investment[]; as_of: string }> {
    return this.http.get("/api/board/investments");
  }
  takeSnapshot(): Promise<{ id: number }> {
    return this.http.post("/api/board/snapshots", { body: {} });
  }
  boardTrends(): Promise<TrendSeries> {
    return this.http.get("/api/board/trends");
  }
  auditorTrail(filter?: {
    actor?: string;
    action?: string;
  }): Promise<AuditTrail> {
    const params = new URLSearchParams();
    if (filter?.actor) params.set("actor", filter.actor);
    if (filter?.action) params.set("action", filter.action);
    const query = params.toString();
    return this.http.get(`/api/auditor/trail${query ? `?${query}` : ""}`);
  }
  auditorFindings(): Promise<AuditorFindings> {
    return this.http.get("/api/auditor/findings");
  }
  evidencePackUrl(): string {
    return "/api/proxy/api/auditor/evidence-pack?format=csv";
  }
  complianceRegister(): Promise<RiskRegister> {
    return this.http.get("/api/compliance/register");
  }
  complianceFindings(): Promise<ConformanceFindings> {
    return this.http.get("/api/compliance/findings");
  }
  riskHeatmap(): Promise<RiskHeatmap> {
    return this.http.get("/api/risk/heatmap");
  }
  securityRegister(): Promise<RiskRegister> {
    return this.http.get("/api/security/register");
  }
  regulatorExtract(): Promise<RegulatorExtract> {
    return this.http.get("/api/regulator/extract");
  }
  // ---- critical path and calendars (SC-1, SC-2) ----
  /** Float, critical chains and constraints for a plan's tasks. */
  setAssignment(pid: string, taskPid: string, body: AssignmentBody): Promise<AssignmentResult> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/assignment`, { body });
  }
  setContour(pid: string, taskPid: string, contour: Contour, custom?: number[]): Promise<{ contour: Contour }> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/contour`, { body: { contour, custom } });
  }
  workProfile(pid: string, params: { period?: Grain; from?: string; to?: string } = {}): Promise<WorkProfile> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/plans/${pid}/work-profile${qs ? `?${qs}` : ""}`);
  }
  costRateTables(): Promise<{ tables: CostRateTable[] }> {
    return this.http.get("/api/cost-rate-tables");
  }
  createCostRateTable(body: { name: string; person_ref?: string; role?: string; currency: string }): Promise<{ pid: string }> {
    return this.http.post("/api/cost-rate-tables", { body });
  }
  addCostRate(tablePid: string, body: { effective_from: string; per_hour_minor: number }): Promise<unknown> {
    return this.http.post(`/api/cost-rate-tables/${tablePid}/rates`, { body });
  }
  taskCosts(pid: string, taskPid: string): Promise<{ costs: TaskCost[] }> {
    return this.http.get(`/api/plans/${pid}/tasks/${taskPid}/costs`);
  }
  addTaskCost(pid: string, taskPid: string, body: Omit<TaskCost, "pid">): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/tasks/${taskPid}/costs`, { body });
  }
  removeTaskCost(pid: string, taskPid: string, costPid: string): Promise<void> {
    return this.http.delete(`/api/plans/${pid}/tasks/${taskPid}/costs/${costPid}`);
  }
  costPlan(pid: string, params: { period?: Grain } = {}): Promise<CostPlan> {
    const qs = params.period ? `?period=${params.period}` : "";
    return this.http.get(`/api/plans/${pid}/cost-plan${qs}`);
  }
  recordLabourCosts(pid: string): Promise<{ recorded: number; already_recorded: number; no_rate: unknown[] }> {
    return this.http.post(`/api/plans/${pid}/labour-costs/record`, { body: {} });
  }
  labourCosts(pid: string): Promise<LabourCosts> {
    return this.http.get(`/api/plans/${pid}/labour-costs`);
  }
  planCpm(pid: string, near?: number): Promise<Cpm> {
    const q = near === undefined ? "" : `?near=${near}`;
    return this.http.get(`/api/plans/${pid}/cpm${q}`);
  }
  /** Set a task's planned start, duration, constraint and deadline; answers with what moved. */
  setTaskSchedule(pid: string, taskPid: string, body: TaskScheduleBody): Promise<ScheduleMoved> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/schedule`, { body });
  }
  scheduleBaselines(pid: string): Promise<ScheduleBaseline[]> {
    return this.http.get(`/api/plans/${pid}/schedule-baselines`);
  }
  /** Take the next baseline (append-only); a reason is required. */
  takeScheduleBaseline(pid: string, reason: string): Promise<ScheduleBaseline> {
    return this.http.post(`/api/plans/${pid}/schedule-baselines`, { body: { reason } });
  }
  scheduleVariance(pid: string, baseline?: number): Promise<ScheduleVariance> {
    const q = baseline === undefined ? "" : `?baseline=${baseline}`;
    return this.http.get(`/api/plans/${pid}/schedule-variance${q}`);
  }
  planRaci(pid: string): Promise<RaciMatrix> {
    return this.http.get(`/api/plans/${pid}/raci`);
  }
  /** Replace a task's RACI row; `roles` is letters such as `"AR"`. */
  setRaciRow(
    pid: string,
    taskPid: string,
    assignments: { person_ref: string; roles: string }[],
  ): Promise<{ task_pid: string; cells: Record<string, string>; findings: RaciFinding[] }> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/raci`, { body: { assignments } });
  }
  earnedValue(pid: string): Promise<EarnedValue> {
    return this.http.get(`/api/plans/${pid}/earned-value`);
  }
  setEarnedValueMethod(pid: string, defaultMethod: "zero_hundred" | "fifty_fifty" | "points"): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/earned-value-settings`, { body: { default_method: defaultMethod } });
  }
  /** Count one task another way; no `method` clears the override. `manual` needs `percent` and `note`. */
  setTaskPercent(
    pid: string,
    taskPid: string,
    body: { method?: "zero_hundred" | "fifty_fifty" | "manual"; percent?: number; note?: string },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/percent-complete`, { body });
  }
  earnedValueForecast(pid: string): Promise<EvForecast> {
    return this.http.get(`/api/plans/${pid}/earned-value/forecast`);
  }
  earnedValueSeries(pid: string): Promise<EvSeries> {
    return this.http.get(`/api/plans/${pid}/earned-value/series`);
  }
  /** Record today's figures now (the scheduler also does this daily). */
  snapshotEarnedValue(pid: string): Promise<{ snapshotted: number }> {
    return this.http.post(`/api/plans/${pid}/earned-value/snapshot`);
  }
  earnedValueRollup(pid: string): Promise<EvRollup> {
    return this.http.get(`/api/plans/${pid}/earned-value/rollup`);
  }
  planCalendar(pid: string): Promise<WorkCalendarView> {
    return this.http.get(`/api/plans/${pid}/calendar`);
  }
  setPlanCalendar(pid: string, body: { weekdays: string; holidays: string[] }): Promise<WorkCalendarView> {
    return this.http.put(`/api/plans/${pid}/calendar`, { body });
  }
  meCalendar(): Promise<WorkCalendarView> {
    return this.http.get("/api/me/calendar");
  }
  setMeCalendar(body: { weekdays: string; holidays: string[] }): Promise<WorkCalendarView> {
    return this.http.put("/api/me/calendar", { body });
  }
  /** Which feature modules this plan exposes (RM-7). */
  planModules(pid: string): Promise<PlanModules> {
    return this.http.get(`/api/plans/${pid}/modules`);
  }
  /** Set a plan's modules, apply the personal preset, or reset to default. */
  setPlanModules(
    pid: string,
    body: { modules: string[] } | { preset: "personal" } | { reset: true },
  ): Promise<PlanModules> {
    return this.http.put(`/api/plans/${pid}/modules`, { body });
  }
  /**
   * The caller's open tasks by due date in `tz` (an IANA zone name; the
   * server refuses fixed offsets). `asOf` is for "what will my day look
   * like at …" and tests.
   */
  myTasks(tz: string, asOf?: string): Promise<MyWork> {
    const q = new URLSearchParams({ tz });
    if (asOf) q.set("as_of", asOf);
    return this.http.get(`/api/me/tasks?${q.toString()}`);
  }
  listTasks(
    pid: string,
  ): Promise<{ tasks: Task[]; counts: Record<string, number> }> {
    return this.http.get(`/api/plans/${pid}/tasks`);
  }
  createTask(pid: string, body: unknown): Promise<Task> {
    return this.http.post(`/api/plans/${pid}/tasks`, { body });
  }
  moveTask(pid: string, taskPid: string, status: string): Promise<Task> {
    return this.http.patch(`/api/plans/${pid}/tasks/${taskPid}`, {
      body: { status },
    });
  }
  listSprints(pid: string): Promise<Sprint[]> {
    return this.http.get(`/api/plans/${pid}/sprints`);
  }
  createSprint(pid: string, body: unknown): Promise<Sprint> {
    return this.http.post(`/api/plans/${pid}/sprints`, { body });
  }
  burndown(pid: string, sprintPid: string): Promise<Burndown> {
    return this.http.get(`/api/plans/${pid}/burndown?sprint=${sprintPid}`);
  }
  standup(pid: string): Promise<Standup> {
    return this.http.get(`/api/plans/${pid}/standup`);
  }
  engineeringBlocked(): Promise<BlockedWork> {
    return this.http.get("/api/engineering/blocked");
  }
  engineeringMoscow(): Promise<MoscowView> {
    return this.http.get("/api/engineering/moscow");
  }
  engineeringDeliveryLinks(): Promise<DeliveryLinks> {
    return this.http.get("/api/engineering/delivery-links");
  }
  milestoneCalendar(kind?: string): Promise<MilestoneCalendar> {
    return this.http.get(
      `/api/engineering/milestone-calendar${kind ? `?kind=${kind}` : ""}`,
    );
  }
  velocity(pid: string): Promise<Velocity> {
    return this.http.get(`/api/plans/${pid}/velocity`);
  }
  listNotes(pid: string, sprintPid: string): Promise<SprintNote[]> {
    return this.http.get(`/api/plans/${pid}/sprints/${sprintPid}/notes`);
  }
  createNote(
    pid: string,
    sprintPid: string,
    body: unknown,
  ): Promise<SprintNote> {
    return this.http.post(`/api/plans/${pid}/sprints/${sprintPid}/notes`, {
      body,
    });
  }
  convertNote(
    pid: string,
    sprintPid: string,
    notePid: string,
  ): Promise<{ note: SprintNote; task: Task }> {
    return this.http.post(
      `/api/plans/${pid}/sprints/${sprintPid}/notes/${notePid}/convert`,
      { body: {} },
    );
  }
  devopsMetrics(months?: number): Promise<DevopsMetrics> {
    return this.http.get(
      `/api/devops/metrics${months ? `?months=${months}` : ""}`,
    );
  }
  devopsReleases(): Promise<DevopsReleases> {
    return this.http.get("/api/devops/releases");
  }
  /** The CSV download URL for a saved report (same-origin proxy). */
  reportCsvUrl(pid: string): string {
    return `${API_BASE_URL}/api/reports/${pid}/run?format=csv`;
  }
}
