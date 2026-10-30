// PPM wire shapes and the small pure helpers that format or reshape them (CQ-9: split out of ppm.ts).
// `ppm.ts` re-exports everything here, so `#lib/api/ppm.js` imports are unchanged.

import type { Collection } from "./types";
import { t } from "../i18n.svelte.js";

// ---- wire shapes (mirroring the service responses) ----

/** `POST /api/plans/{pid}/risks`; checked against the service's schema by `client-bodies.test.ts` (CQ-9). */
export interface CreateRiskBody {
  title: string;
  probability: number;
  impact: number;
  category?: string | null;
  description?: string | null;
  mitigation?: string | null;
  owner_ref?: string | null;
  review_date?: string | null;
}

/** `POST /api/plans/{pid}/tasks`. */
export interface CreateTaskBody {
  title: string;
  assignee_ref?: string | null;
  component_pid?: string | null;
  custom?: Record<string, unknown> | null;
  description?: string | null;
  due_date?: string | null;
  duration_days?: number | null;
  flow_type?: string | null;
  parent_task_pid?: string | null;
  points?: number | null;
  sprint_pid?: string | null;
  start_date?: string | null;
  status?: string | null;
}

/** `POST /api/plans/{pid}/milestones`. */
export interface CreateMilestoneBody {
  name: string;
  due: string;
  kind?: string | null;
}

/** `POST /api/scenarios`. */
export interface CreateScenarioBody {
  name: string;
  plan_pids?: string[];
  proposal_pids?: string[];
  must_include?: string[];
  budget_cap_minor?: number | null;
  currency?: string | null;
  description?: string | null;
}

/** `POST /api/proposals`. */
export interface CreateProposalBody {
  title: string;
  kind_target: string;
  currency?: string | null;
  external_ask?: boolean | null;
  requested_minor?: number | null;
  sponsor_ref?: string | null;
  strategic_rationale?: string | null;
  summary?: string | null;
  tradeoff?: string | null;
}

/** `POST /api/ideas`. */
export interface CreateIdeaBody {
  title: string;
  pitch?: string | null;
  tags?: string[] | null;
}

/** `POST /api/plans/{pid}/budget-lines`. */
export interface CreateBudgetLineBody {
  category: string;
  currency: string;
  description: string;
  planned_minor: number;
  funding_source_pid?: string | null;
  gate?: string | null;
  period_end?: string | null;
  period_start?: string | null;
}

/** `POST /api/plans/{pid}/benefits`. */
export interface CreateBenefitBody {
  title: string;
  category: string;
  currency?: string | null;
  expected_on?: string | null;
  target_minor?: number | null;
  target_note?: string | null;
}

/** `POST /api/plans/{pid}/allocations`. */
export interface CreateAllocationBody {
  percent: number;
  booking?: string | null;
  end_date?: string | null;
  person_ref?: string | null;
  placeholder?: boolean | null;
  requested?: boolean | null;
  role?: string | null;
  skills_required?: string[] | null;
  start_date?: string | null;
}

/** `POST /api/plans/{pid}/gate-reviews`. */
export interface ReviewGateBody {
  gate: string;
  decision: string;
  approver_ref?: string | null;
  conditions?: string | null;
}

/** `POST /api/objectives`. */
export interface CreateObjectiveBody {
  title: string;
  description?: string | null;
  period?: string | null;
}

/** `POST /api/objectives/{pid}/key-results`. */
export interface CreateKeyResultBody {
  title: string;
  metric: string;
  direction: string;
  start_value: number;
  target_value: number;
  currency?: string | null;
  owner_ref?: string | null;
  tolerance?: number | null;
  unit?: string | null;
}

/** `PUT /api/key-results/{pid}`. */
export interface UpdateKeyResultBody {
  title: string;
  target_value: number;
  due_date?: string | null;
  owner_ref?: string | null;
  tolerance?: number | null;
  unit?: string | null;
}

/** `PUT /api/kpis/{pid}`. */
export interface UpdateKpiBody {
  name: string;
  red_value: number;
  target_value: number;
  cadence?: string | null;
  description?: string | null;
  objective_pid?: string | null;
  owner_ref?: string | null;
  unit?: string | null;
}

/** `POST /api/kpis`. */
export interface CreateKpiBody extends UpdateKpiBody {
  direction: string;
  plan_pid?: string | null;
}

/** `POST /api/reports`. */
export interface CreateReportBody {
  name: string;
  collection: string;
  fields: string[];
  filters?: unknown;
  group_by?: string | null;
}

/** `POST /api/plans/{pid}/sprints`. */
export interface CreateSprintBody {
  name: string;
  starts_on: string;
  ends_on: string;
}

/** `POST /api/plans/{pid}/escalations`. */
export interface RaiseEscalationBody {
  title: string;
  reason_kind?: string;
  description?: string | null;
  tolerance_dimension?: string | null;
  level?: number | null;
  escalated_to?: string | null;
  owner_ref?: string | null;
  respond_by?: string | null;
}

/** `POST /api/plans/{pid}/benefits/{b_pid}/realize`. */
export interface RealizeBenefitBody {
  amount_minor?: number | null;
  note?: string | null;
  status?: string | null;
}

/** `POST /api/plans/{pid}/sprints/{s_pid}/notes`. */
export interface CreateSprintNoteBody {
  category: string;
  body: string;
}

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
  /** Business days the risk adds to its task. */
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
    materialized: number;
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
    /** Business days; negative is a lead. */
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
  threshold: {
    op: string;
    limit: number | null;
    unit: "percent" | "milli" | "days" | "result";
  };
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
  /** Business days the start moves later. */
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
  effects: {
    pid: string;
    title: string | null;
    moved: string;
    /** Business days the start moved; positive is later. */
    start_days: number;
    /** Business days the finish moved; positive is later. */
    finish_days: number;
  }[];
  over_allocated: LevelingRun[];
  unresolved: LevelingRun[];
  finish_before: string | null;
  finish_after: string | null;
  /** Business days the project finish moves. */
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
  /** Business days the project finish moved. */
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
  /** Calendar days in the period, both ends included. */
  days: number;
  rows: ResourceRow[];
  total_demand_fte: number;
  total_placeholder_fte: number;
  total_proposed_fte: number;
}

/** A ways-of-working or governance framework record (`/api/ways-of-working`, `/api/governance-frameworks`). */
export interface Framework {
  pid: string;
  name: string;
  description: string | null;
  scope: "organization" | "plan";
  owner_plan_pid: string | null;
  plans_using: number;
  // Ways of working.
  methodology?: string;
  cadence_days?: number | null;
  wip_limit?: number | null;
  definition_of_ready?: string | null;
  definition_of_done?: string | null;
  ceremonies?: { name: string; cadence: string; purpose?: string | null }[];
  // Governance.
  decision_model?: string;
  review_cadence_days?: number | null;
  reporting_cadence_days?: number | null;
  escalation_path?: { level: number; body: string; trigger: string }[];
  decision_rights?: {
    decision: string;
    decides: string;
    consulted?: string | null;
  }[];
  forums?: { name: string; cadence: string; purpose?: string | null }[];
}

/** `GET /api/plans/{pid}/ways-of-working` and `…/governance-framework`: what is in force and where it came from. */
export interface FrameworkInForce {
  plan_pid: string;
  source: "own" | "inherited" | "none";
  from_plan_pid: string | null;
  ways_of_working?: Framework | null;
  governance_framework?: Framework | null;
  reason: string | null;
}

/** One link on a plan (`/api/plans/{pid}/links`). */
export interface PlanLink {
  pid: string;
  kind: string;
  url: string;
  label: string;
  system: string | null;
  external_id: string | null;
}

export type FrameworkFamily = "ways-of-working" | "governance-frameworks";

export type EscalationStatus =
  "open" | "acknowledged" | "resolved" | "withdrawn";

/** One escalation (`/api/plans/{pid}/escalations`, `/api/escalations`). */
export interface Escalation {
  pid: string;
  plan_pid: string;
  plan_name?: string | null;
  title: string;
  description: string | null;
  reason_kind: string;
  tolerance_dimension: string | null;
  level: number;
  escalated_to: string | null;
  owner_ref: string | null;
  status: EscalationStatus;
  raised_by: string | null;
  raised_on: string;
  respond_by: string | null;
  outcome: string | null;
  response: string | null;
  resolved_on: string | null;
  age_days: number;
  overdue: boolean;
  /** Only on the single-escalation read. */
  events?: {
    event: string;
    level: number;
    actor: string | null;
    note: string | null;
    at: string;
  }[];
}

/** `GET /api/escalations`. */
export interface EscalationRegister {
  escalations: Escalation[];
  counts: Record<string, number>;
  overdue: number;
}

export type ToleranceStatus = "green" | "amber" | "red" | "unknown" | "not_set";

/** One dimension of a plan's tolerances (`GET /api/plans/{pid}/tolerances`). */
export interface ToleranceDimension {
  dimension: "time" | "schedule" | "cost" | "risk" | "resources";
  unit: string;
  basis: string;
  status: ToleranceStatus;
  limit: number | null;
  amber_from_percent?: number;
  measured?: number | null;
  usage_percent?: number | null;
  reason?: string | null;
  source: "own" | "inherited" | "none";
  from_plan_pid?: string | null;
  note?: string | null;
  /** Live escalations that cite this dimension. */
  open_escalations?: number;
  /** Red, and nobody has escalated it. */
  escalation_needed?: boolean;
}

/** A plan's tolerances in force, measured and coloured. */
export interface TolerancePicture {
  plan_pid: string;
  plan_name: string;
  overall: ToleranceStatus;
  /** Some red dimension has no live escalation. */
  escalation_needed?: boolean;
  as_of: string;
  dimensions: ToleranceDimension[];
  how_to_read: string;
}

/** `GET /api/tolerances`: every readable plan with a tolerance, worst first. */
export interface ToleranceBoard {
  plans: TolerancePicture[];
  truncated: boolean;
  measured_at_most: number;
  note: string;
}

/** An approval process definition (`GET /api/processes`). The list has no `steps`. */
export interface ProcessDefinition {
  pid: string;
  name: string;
  current_version: number;
  steps?: {
    kind: "approve" | "todo" | "notify";
    person_ref: string;
    title: string;
  }[];
}

/** A run of a process (`GET /api/process-executions`; the detail adds `steps` and `log`). */
export interface ProcessRun {
  pid: string;
  process_pid: string;
  process_name?: string | null;
  version: number;
  subject_kind: string;
  subject_pid: string;
  status: "running" | "completed" | "rejected" | "cancelled" | "failed";
  current_step: number;
  started_by: string;
  finished_at: string | null;
  note?: string | null;
  steps?: {
    index: number;
    kind: "approve" | "todo" | "notify";
    person_ref: string;
    title: string;
    state:
      "pending" | "waiting" | "completed" | "skipped" | "rejected" | "failed";
  }[];
  log?: {
    step: number | null;
    event: string;
    actor: string | null;
    note: string | null;
    at: string;
  }[];
}

/** `GET /api/resource-plan/role-tasks`: the open tasks of the people in a role. */
export interface RoleTasks {
  role: string;
  people: {
    person_ref: string;
    tasks: {
      pid: string;
      title: string;
      status: string;
      due_date: string | null;
      plan_pid: string;
      plan_name: string;
    }[];
  }[];
  tasks: number;
  /** Tasks in plans the reader cannot read: counted, never shown. */
  omitted_unreadable: number;
  truncated: boolean;
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
  placeholders: {
    allocation_pid: string;
    plan_pid: string;
    role: string | null;
    percent: number;
    start_date: string | null;
    end_date: string | null;
    booking: string;
  }[];
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
    used: {
      risk_pid: string;
      title: string;
      task_pid: string;
      probability_percent: number;
      /** Business days it adds. */
      impact_days: number;
    }[];
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
  events: {
    risk: string;
    plan: string;
    title: string | null;
    probability_bp: number;
    impact_minor: number;
    hit_bp: number;
  }[];
  plans: {
    plan: string;
    name: string | null;
    budget_minor: number;
    mean_minor: number;
  }[];
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
    used: {
      risk_pid: string;
      plan_pid: string;
      title: string;
      probability_percent: number;
      cost_impact_minor: number;
      currency: string;
      also_a_schedule_event: boolean;
    }[];
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
  quadrants: {
    both: number;
    budget_only: number;
    time_only: number;
    neither: number;
  };
  points: { days_late: number; cost_minor: number }[];
  events: {
    schedule: {
      used: unknown[];
      skipped: { risk_pid: string; title: string; reason: string }[];
      risks_without_schedule_fields: number;
    };
    cost: {
      used: unknown[];
      skipped: { risk_pid: string; title: string; reason: string }[];
      risks_without_cost_fields: number;
    };
  };
  note: string;
}

// ---- Milestones as a concept (portfolio view) ----

export type MilestoneStatus = "achieved" | "late" | "due_soon" | "upcoming";
export type MilestoneScheduleState =
  "on_track" | "at_risk" | "forecast_late" | "no_forecast";
export type MilestoneMode = "manual" | "linked";

export interface MilestoneSchedule {
  /** Null once achieved. */
  state: MilestoneScheduleState | null;
  forecast: string | null;
  /** Business days between the forecast and the due date; negative is late. */
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
  history?: {
    from: string;
    to: string;
    reason: string | null;
    actor: string | null;
    at: string;
  }[];
}

/** One issue (IR-1/IR-2). */
export interface Issue {
  pid: string;
  plan_pid: string;
  title: string;
  description: string | null;
  kind: "bug" | "risk" | "blocker" | "question" | "improvement" | "concern";
  severity: "low" | "med" | "high" | "critical";
  priority: string | null;
  status: "open" | "in_progress" | "resolved" | "closed";
  owner_ref: string | null;
  raised_by: string | null;
  raised_on: string;
  due_date: string | null;
  resolution: string | null;
  resolved_on: string | null;
  /** Calendar days since raised, until resolved. */
  age_calendar_days: number;
  overdue: boolean;
}

/** `GET /api/issues`. */
export interface IssueRegister {
  issues: Issue[];
  matching: number;
  truncated: boolean;
  summary: {
    by_severity: Record<string, number>;
    by_status: Record<string, number>;
    overdue: number;
    oldest_open: Issue | null;
  };
  as_of: string;
  note: string;
}

/** `GET /api/plans/{pid}/issues`: one plan's issues, most serious first. */
export interface PlanIssueList {
  plan_pid: string;
  issues: Issue[];
  total: number;
  open: number;
  overdue: number;
  as_of: string;
}

/**
 * `POST /api/plans/{pid}/issues` and `PUT /api/plans/{pid}/issues/{issue_pid}` (same fields); checked against the
 * service's schema by `client-bodies.test.ts` (CQ-9). A closed issue needs a resolution.
 */
export interface IssueBody {
  title: string;
  description?: string | null;
  kind?: Issue["kind"];
  severity?: Issue["severity"];
  priority?: string | null;
  status?: Issue["status"];
  owner_ref?: string | null;
  raised_on?: string | null;
  due_date?: string | null;
  resolution?: string | null;
  source_risk_pid?: string | null;
  source_task_pid?: string | null;
}

/** One change request (IR-4). */
export interface ChangeRequest {
  pid: string;
  plan_pid: string;
  title: string;
  description: string;
  reason: string;
  kind: "scope" | "schedule" | "cost" | "quality" | "other";
  impact: {
    schedule_business_days: number | null;
    cost_minor: number | null;
    cost_currency: string | null;
    summary: string | null;
    has_forecast_edits: boolean;
  };
  status:
    | "draft"
    | "submitted"
    | "under_review"
    | "approved"
    | "rejected"
    | "implemented"
    | "withdrawn";
  requested_by: string;
  requested_on: string;
  /** Calendar days since requested. */
  age_calendar_days: number;
  approver_ref: string | null;
  decided_by: string | null;
  decided_on: string | null;
  decision_reason: string | null;
}

/** A director pack's row (DP-1). */
// ---- CC-6: the critical chain, its buffers, buffer management and the drum --------------------------

export interface TitledRef {
  pid: string;
  title: string | null;
}

export interface ChainTaskRow {
  pid: string;
  title: string | null;
  duration_days: number;
  start: string | null;
  finish: string | null;
  on_critical_path: boolean;
}

export interface ChainLink {
  from: string;
  to: string;
  kind: "dependency" | "resource";
  person?: string;
  person_ref?: string | null;
  masked?: boolean;
}

/** `GET /api/plans/{pid}/critical-chain`. */
export interface CriticalChain {
  advisory: boolean;
  state: "chain" | "no_critical_chain";
  reason?: string;
  calendar: string;
  critical_path: { tasks: TitledRef[]; finish: string | null; chains: number };
  levelled_finish: string | null;
  finish_delay_days: number;
  over_allocated_after: number;
  chain_days?: number;
  crosses_paths?: boolean;
  tied_chains?: boolean;
  tasks?: ChainTaskRow[];
  links?: ChainLink[];
  note: string;
}

export interface FeedingBuffer {
  joins: TitledRef;
  tasks: TitledRef[];
  duration_days: number;
  buffer_days: number | null;
  room_before_join_days?: number;
  would_push_chain?: boolean;
  push_days?: number;
  reason?: string;
  tasks_missing_estimates?: TitledRef[];
}

export interface ResourceBuffer {
  task: TitledRef;
  starts: string;
  alert_on: string;
  person: string;
  person_ref: string | null;
  masked?: boolean;
}

/** `GET /api/plans/{pid}/critical-chain-buffers`. */
export interface ChainBuffers {
  advisory: boolean;
  state: "buffers" | "no_buffers";
  reason?: string;
  buffer_rule: string;
  estimate_rule: string;
  resource_buffer_lead_days: number;
  chain_tasks_missing_estimates?: TitledRef[];
  scheduled_at_likely_duration?: TitledRef[];
  chain_days_at_aggressive?: number;
  chain?: TitledRef[];
  project_buffer?: {
    days: number;
    safety_removed_days: number;
    chain_finish: string;
    finish_with_buffer: string;
  };
  feeding_buffers?: FeedingBuffer[];
  resource_buffers?: ResourceBuffer[];
  note: string;
}

export interface FeverPoint {
  chain_completed_pct: number;
  buffer_consumed_pct: number;
  plotted_x_pct: number;
  plotted_y_pct: number;
  edge: "below" | "above" | null;
  zone: "green" | "amber" | "red";
  slip_days: number;
  buffer_days: number;
}

export interface FeverLines {
  green_amber: { at_start: number; at_end: number };
  amber_red: { at_start: number; at_end: number };
}

export interface BufferReading {
  key: string;
  kind: "project" | "feeding";
  buffer_days: number | null;
  chain_done_days: number;
  chain_total_days: number;
  slip_days: number | null;
  point: FeverPoint | null;
  no_point: string | null;
  no_point_reason?: string;
  chain: string[];
  joins: string | null;
}

export interface RecoveryAction {
  pid: string;
  buffer_key: string;
  action: string;
  due_date: string | null;
  status: "open" | "done" | "dropped";
  closed_on: string | null;
  owner: string | null;
  owner_ref: string | null;
}

/** `GET /api/plans/{pid}/buffer-management`. */
export interface BufferManagement {
  state: "readings" | "no_buffer_plan";
  reason?: string;
  buffer_plan?: { pid: string; taken_on: string };
  lines?: FeverLines;
  readings?: BufferReading[];
  alerts?: {
    kind: string;
    buffer_key: string;
    chain: string[];
    joins: string | null;
    buffer_consumed_pct: number | null;
  }[];
  red_without_open_recovery_action?: string[];
  recovery_actions?: RecoveryAction[];
  snapshots?: {
    pid: string;
    taken_on: string;
    zones: Record<string, string | null>;
  }[];
  note?: string;
}

export interface DrumView {
  state: "drum" | "no_drum";
  skill_pool_pid?: string;
  name?: string;
  source?: "named" | "most_overloaded";
  why?: string;
  reason?: string;
}

export interface PipelineProject {
  plan_pid: string;
  name: string;
  outcome: "as_planned" | "staggered" | "exceeds_drum" | "no_slot";
  first_drum_use: string;
  last_drum_use: string;
  peak_fte: number;
  delay_days: number;
  proposed_first_drum_use: string;
  explanation: string;
}

/** `GET /api/portfolio-pipeline`. */
export interface Pipeline {
  state: "pipeline" | "no_drum" | "no_capacity";
  reason?: string;
  why?: string;
  drum?: { skill_pool_pid: string; name: string; source: string; why: string };
  capacity_fte?: number;
  capacity_buffer_days: number;
  from: string;
  months: number;
  projects?: PipelineProject[];
  staggered?: number;
  note: string;
}

export interface SkillPool {
  pid: string;
  name: string;
  active: boolean;
  roles: string[];
  skills: string[];
  /** FB-1: the workforce service's pool id this pool is linked to, or null. Older answers may leave it out. */
  peer_pool_ref?: string | null;
}

export interface DirectorPackMeta {
  pid: string;
  period: string;
  audience: string;
  version: number;
  status: "draft" | "issued";
  cut_off: string;
  change_note: string | null;
  supersedes_pid: string | null;
  issued_at: string | null;
  issued_by: string | null;
}

/** `GET /api/director-packs/{pid}`: the pack row and its frozen body, as stored. */
export interface DirectorPack {
  pack: DirectorPackMeta;
  body: Record<string, unknown>;
}

/** `GET /api/change-requests`. */
export interface ChangeRequestList {
  change_requests: ChangeRequest[];
  summary: {
    by_status: Record<string, number>;
    awaiting_decision: number;
    oldest_open: ChangeRequest | null;
  };
  as_of: string;
}

/** `GET /api/milestones`. */
export interface MilestoneList {
  as_of: string;
  /** The "due soon" window, in calendar days. */
  due_soon_days: number;
  /** Business days of slack under which a forecast is at risk. */
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
  /** `committed` (firm) or `proposed` (hoped for; never counts toward utilization). PJ-4. */
  booking: "committed" | "proposed";
  /** Asked of the resource manager. */
  requested: boolean;
}

/** A run of calendar days where demand exceeded the capacity in force (PJ-3). */
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
    proposed: {
      percent: number;
      allocations: number;
      would_exceed_capacity: boolean;
    };
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
    materialized_risks: number;
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

/** Money in minor units, in the reader's locale with the currency's own exponent (see `money.ts`). */
export { money } from "../money.js";

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

/** One thing a tracker import left out, with the reason. */
export interface ImportUnmapped {
  what: string;
  key: string | null;
  reason: string;
}

/** `POST /api/plans/{pid}/import/{jira|asana}`. */
export interface TrackerImportReport {
  dry_run: boolean;
  source: "jira" | "asana" | "mspdi";
  items_in_file: number;
  tasks_created: number;
  tasks_updated: number;
  links_in_file: number;
  links_created: number;
  links_already_present: number;
  unmapped: ImportUnmapped[];
  note: string;
}

/**
 * The MS Project report, shaped like a tracker report so one screen shows both: its task
 * `uid` is the item key, and its project calendar (imported or not) is one more line.
 */
export function mspdiReport(r: Record<string, unknown>): TrackerImportReport {
  const un = Array.isArray(r.unmapped)
    ? (r.unmapped as { what?: string; uid?: number | null; reason?: string }[])
    : [];
  return {
    dry_run: r.dry_run === true,
    source: "mspdi",
    items_in_file: Number(r.tasks_in_file ?? 0),
    tasks_created: Number(r.tasks_created ?? 0),
    tasks_updated: Number(r.tasks_updated ?? 0),
    links_in_file: Number(r.links_in_file ?? 0),
    links_created: Number(r.links_created ?? 0),
    links_already_present: Number(r.links_already_present ?? 0),
    unmapped: un.map((u) => ({
      what: u.what ?? "field",
      key:
        u.uid === null || u.uid === undefined
          ? null
          : t("l.api_ppm.task", { uid: u.uid }),
      reason: u.reason ?? "",
    })),
    note:
      String(r.note ?? "") +
      (r.calendar_imported
        ? t("l.api_ppm.the_project_calendar_comes_across")
        : ""),
  };
}

/** PL-6: where a version of a controlled document is in its life. */
export type DocumentStatus =
  "draft" | "in_review" | "approved" | "effective" | "superseded";

/** One version of a controlled document (text only on a single-version read). */
export interface DocumentVersion {
  pid: string;
  version: number;
  status: DocumentStatus;
  change_summary: string | null;
  approval_pid: string | null;
  effective_from: string | null;
  superseded_on: string | null;
  superseded_by: string | null;
  created_by: string | null;
  body?: string;
}

/** `GET /api/documents[/{pid}]`. */
export interface ControlledDocument {
  pid: string;
  title: string;
  plan_pid: string | null;
  created_by: string | null;
  /** The version number in force today, or null. */
  in_force_today: number | null;
  versions: DocumentVersion[];
}

/** `GET /api/documents/{pid}/in-force`. */
export interface DocumentInForce {
  document_pid: string;
  on: string;
  in_force: DocumentVersion | null;
}

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
export type Contour =
  "flat" | "front_loaded" | "back_loaded" | "bell" | "turtle" | "custom";
export type Grain = "day" | "week" | "month";

export interface AssignmentBody {
  rule?: SchedulingRule;
  /** Duration in business days. */
  duration_days?: number;
  units_percent?: number;
  work_minutes?: number;
}

export interface AssignmentResult {
  task_pid: string;
  rule: SchedulingRule;
  /** Duration in business days. */
  duration_days: number;
  units_percent: number;
  work_minutes: number;
  derived: "duration" | "units" | "work" | null;
  exact: boolean | null;
  capacity: {
    checked: boolean;
    reason?: string;
    over_capacity?: boolean;
    /** Calendar days on which demand exceeds capacity. */
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
  unplaced: Array<{
    pid: string;
    title: string;
    work_minutes: number;
    reason: string;
  }>;
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
  currencies: Array<{
    currency: string;
    labour: MoneyBuckets;
    fixed: MoneyBuckets;
    total: MoneyBuckets;
  }>;
  unrated: Array<{
    pid: string;
    title: string;
    work_minutes: number;
    reason: string;
  }>;
  unassigned: Array<{
    pid: string;
    title: string;
    work_minutes: number;
    reason: string;
  }>;
  unplaced: Array<{
    cost_pid: string;
    task_pid: string;
    amount_minor: number;
    currency: string;
    reason: string;
  }>;
  reason?: string;
  note: string;
}

export interface LabourCosts {
  currencies: Array<{
    currency: string;
    cost_minor: number;
    minutes: number;
    entries: number;
  }>;
  unpriced: { entries: number; minutes: number; reason: string | null };
  note: string;
}

export type SandboxEdit =
  | { op: "move"; task_pid: string; start_date: string }
  | { op: "duration"; task_pid: string; duration_days: number }
  | {
      op: "constraint";
      task_pid: string;
      constraint_kind: string | null;
      constraint_date: string | null;
      reason?: string;
    }
  | {
      op: "add_link";
      predecessor_pid: string;
      successor_pid: string;
      kind?: string;
      /** Business days; negative is a lead. */
      lag_days?: number;
    }
  | { op: "remove_link"; link_pid: string };

export interface SandboxDates {
  early_start: string;
  early_finish: string;
  late_finish: string;
  total_float: number;
  critical: boolean;
}

export interface SandboxPreview {
  plan_pid: string;
  plan_etag: string;
  result_etag: string;
  changes: Array<{ item: "task" | "link_added" | "link_removed"; pid: string }>;
  finish: { before: string | null; after: string | null };
  tasks: Array<{
    pid: string;
    title: string;
    assignee?: string | null;
    before: SandboxDates | null;
    after: SandboxDates | null;
  }>;
  moved: Array<{
    pid: string;
    moved: string;
    /** Business days the start moved; positive is later. */
    start_days: number;
    /** Business days the finish moved; positive is later. */
    finish_days: number;
  }>;
  critical_path: {
    before: string[];
    after: string[];
    added: string[];
    removed: string[];
  };
  assignments_recomputed: Array<{
    task_pid: string;
    derived: string | null;
    exact: boolean | null;
    units_percent: number;
    work_minutes: number;
  }>;
  cost_impact:
    | { available: false; reason: string }
    | {
        available: true;
        labour: Array<{
          currency: string;
          before_minor: number;
          after_minor: number;
          delta_minor: number;
        }>;
        unrated_minutes_before: number;
        unrated_minutes_after: number;
      };
  masked: boolean;
  note: string;
}

export interface SandboxCommitted {
  committed: boolean;
  audit_entries: number;
  finish: { before: string | null; after: string | null };
}

export type OnboardingKind =
  "person_join" | "person_leave" | "plan_intake" | "plan_closeout";

export interface OnboardingProgress {
  total: number;
  done: number;
  skipped: number;
  open: number;
  overdue: number;
  resolved_bp: number | null;
}

export interface OnboardingItem {
  pid: string;
  position: number;
  title: string;
  owner_ref: string | null;
  due_date: string | null;
  resolution: "done" | "skipped" | null;
  note: string | null;
  overdue: boolean;
}

export interface OnboardingCase {
  pid: string;
  kind: OnboardingKind;
  plan_pid: string;
  title: string;
  subject_ref: string | null;
  effective_date: string;
  status: "open" | "completed" | "cancelled";
  closed_reason: string | null;
  progress: OnboardingProgress;
  items: OnboardingItem[];
  masked: boolean;
}

export interface OnboardingCaseSummary {
  pid: string;
  kind: OnboardingKind;
  title: string;
  status: "open" | "completed" | "cancelled";
  subject_ref: string | null;
  effective_date: string;
  progress: OnboardingProgress;
}

export interface Closeout {
  plan_pid: string;
  ready: boolean;
  counts: {
    open_tasks: number;
    open_risks: number;
    active_allocations: number;
    outstanding_access: number;
    open_cases: number;
  };
  findings: Array<{ code: string; count: number; text: string }>;
}

export interface AccessRecords {
  outstanding: Array<{
    person_ref: string;
    access: string;
    granted_on: string;
  }>;
  records: Array<{
    pid: string;
    person_ref: string;
    access: string;
    action: "granted" | "revoked";
    recorded_on: string;
    note: string | null;
  }>;
  note: string;
}

export interface HandoverPreview {
  plan_pid: string;
  from_person: string;
  to_person: string | null;
  handover_etag: string;
  action: "reassign" | "release";
  holdings: Array<{
    kind: "task" | "allocation" | "raci" | "risk";
    pid: string;
    label: string;
  }>;
  counts: Record<string, number>;
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
    /** Calendar days since the last update. */
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
  /** Median lead time in calendar days; null with no timed completions. */
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
  /** Gate-review recency threshold, in calendar days. */
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
    materialized: number;
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
  /** The deadline (`YYYY-MM-DD`), or null: what an import or a paste sets; the board does not show it. */
  deadline?: string | null;
  sprint_pid: string | null;
  created_at: string;
  status_changed_at: string;
  done_at: string | null;
  /** Calendar days since the task entered blocked; null when not blocked. */
  blocked_days: number | null;
  /** The parent task (a sub-task's), or null for a top-level task. */
  parent_task_pid?: string | null;
  /** Position among the siblings; null until a reorder sets it. */
  sibling_position?: number | null;
  is_hammock?: boolean;
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
  /** Business days it can slip before the project finish moves; negative = cannot meet a constraint or deadline. */
  total_float: number;
  /** Business days before any successor is delayed. */
  free_float: number;
  critical: boolean;
  near_critical: boolean;
  planned_start: string | null;
  /** The stored business-day duration (0 = milestone). */
  duration_days: number | null;
  /** Optimistic duration in business days for schedule risk; the likely one is `duration_days`. */
  duration_optimistic_days?: number | null;
  /** Pessimistic duration in business days. */
  duration_pessimistic_days?: number | null;
  /** Business-day logic forces the start later than planned. */
  pushed_days: number;
  constraint: "snet" | "snlt" | "fnet" | "fnlt" | "mso" | "mfo" | null;
  /** PJ-5 / PJ-6: work in minutes, units as a percent of the assignee's day, and the rule and contour. */
  work_minutes?: number | null;
  units_percent?: number | null;
  scheduling_rule?: SchedulingRule | null;
  contour?: Contour | null;
  constraint_date: string | null;
  deadline: string | null;
  /** PJ-13: a task of another plan, in this plan's network because a link joins them. */
  external?: boolean;
  plan_pid?: string;
  plan_name?: string;
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
  /** Business days; negative is a lead. */
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
  unscheduled: {
    pid: string;
    title: string | null;
    reason: "no_duration" | "no_anchor" | "date_out_of_range";
  }[];
  ignored_edges: string[];
  critical_chains: string[][];
  chains_truncated: boolean;
  /** The latest finish across the plan and what it is linked to. */
  network_finish?: string | null;
  /** Dependencies to a plan the caller may not read: counted, never named. */
  restricted_edges?: {
    pid: string;
    task_pid: string;
    side: "predecessor" | "successor";
    label: string;
  }[];
  external_truncated?: boolean;
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
export type VarianceStanding =
  | "slipped"
  | "pulled_in"
  | "on_baseline"
  | "added"
  | "removed"
  | "unmeasurable";

export interface VarianceRow {
  task_pid: string;
  title: string;
  standing: VarianceStanding;
  baseline_start: string | null;
  baseline_finish: string | null;
  start: string | null;
  finish: string | null;
  /** Business days; positive is later; null when no variance can be stated. */
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
      /** Business days the plan's own finish moved; null when either side has no finish. */
      plan_finish_slip_days: number | null;
    };
  } | null;
}

/** What is incomplete on a RACI row. */
export type RaciFinding =
  "no_accountable" | "no_responsible" | "many_accountable";

/** `GET /api/plans/{pid}/raci`. */
export interface RaciMatrix {
  plan_pid: string;
  /** Everyone who holds a letter anywhere, sorted. */
  people: string[];
  rows: {
    task_pid: string;
    title: string;
    cells: Record<string, string>;
    findings: RaciFinding[];
  }[];
  summary: {
    tasks: number;
    complete: number;
    missing_accountable: number;
    multiple_accountable: number;
    missing_responsible: number;
  };
  workload: {
    person_ref: string;
    responsible: number;
    accountable: number;
    consulted: number;
    informed: number;
  }[];
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
  /** Earned schedule: the calendar days elapsed at which the plan should have earned what has been earned. */
  earned_schedule_days: number | null;
  /** Calendar days elapsed since the baseline started (the actual time). */
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
    tasks: {
      pid: string;
      weight: number;
      percent_bp: number;
      method: "zero_hundred" | "fifty_fifty" | "manual";
    }[];
  };
  unbaselined_currencies?: string[];
  ac_source?: string;
  note?: string;
}

export type EvRag = "green" | "amber" | "red" | "unknown";

/** An amount, or why there is none (EVM analytics, SC-5). */
export interface EvAmount {
  minor: number | null;
  absent?:
    | "no_actual_cost"
    | "undefined_denominator"
    | "no_bottom_up_estimate"
    | "budget_exhausted"
    | "no_remaining_cost";
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
  forecasts:
    | {
        currency: string;
        baseline_version: number;
        spi: { bp: number | null; rag: EvRag };
        cpi: { bp: number | null; rag: EvRag };
        variants: {
          variant: "budget_rate" | "cpi" | "cpi_spi" | "bottom_up";
          formula: string;
          eac: EvAmount;
          etc: EvAmount;
          vac: EvAmount;
        }[];
        recommended: { variant: string; why: string; eac_minor: number | null };
        tcpi_to_budget: { tcpi: EvTcpi; rag: EvRag };
        tcpi_to_recommended_eac: { tcpi: EvTcpi; rag: EvRag } | null;
        bac_minor: number;
        ev_minor: number;
        ac_minor: number | null;
      }[]
    | null;
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
    points: {
      date: string;
      pv_minor: number;
      ev_minor: number | null;
      ac_minor: number | null;
    }[];
  }[];
  note: string;
}

/** `GET /api/plans/{pid}/earned-value/rollup`. */
export interface EvRollup {
  root_plan_pid: string;
  as_of: string;
  currencies: {
    currency: string;
    totals: {
      bac_minor: number;
      pv_minor: number;
      ev_minor: number;
      ac_minor: number;
      plans: number;
      plans_without_ac: number;
    };
    spi: { bp: number | null; rag: EvRag };
    cpi: { bp: number | null; rag: EvRag; absent?: string | null };
    eac: {
      minor: number | null;
      plans_counted: number;
      plans_total: number;
      note: string;
    };
    vac_minor: number | null;
  }[];
  plans_without_figures: number;
  note: string;
}

/** `PUT …/tasks/{pid}/schedule` body; an absent field clears it. */
export interface TaskScheduleBody {
  start_date?: string | null;
  /** Duration in business days. */
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
  moved:
    | {
        pid: string;
        moved: "start" | "finish" | "both";
        /** Business days the start moved; positive is later. */
        start_days: number;
        /** Business days the finish moved; positive is later. */
        finish_days: number;
      }[]
    | null;
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
 * business days. Only ever the caller's own task; absent (`null` on the task)
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

export function simQuery(o: SimOptions): string {
  const q = new URLSearchParams();
  if (o.iterations) q.set("iterations", String(o.iterations));
  if (o.seed !== undefined) q.set("seed", String(o.seed));
  if (o.distribution) q.set("distribution", o.distribution);
  if (o.fallbackBandPercent !== undefined)
    q.set("fallback_band_percent", String(o.fallbackBandPercent));
  if (o.currency) q.set("currency", o.currency);
  if (o.delayCostPerDayMinor !== undefined)
    q.set("delay_cost_per_day_minor", String(o.delayCostPerDayMinor));
  const qs = q.toString();
  return qs ? `?${qs}` : "";
}
