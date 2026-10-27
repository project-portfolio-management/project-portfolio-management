// PPM catalogue client (Phases A–C): typed wrappers over the
// governance / visibility / strategy endpoints, mirroring the
// service's route table (spec 15-roadmap PPM-1..12). One class, one
// method per endpoint, the single source of PPM paths for the UI.
//
// The strings these views show come from the page catalogues: English and
// Welsh (machine-written, unreviewed); the other locales fall back to English.
import { API_BASE_URL } from "#lib/config.js";
import { ApiClient } from "./client";
import type { Collection } from "./types";
import { rowOf } from "#lib/row-of.js";
import { mspdiReport, simQuery } from "./ppm-types";
import type {
  AccessRecords,
  Alignment,
  AlignmentCoverage,
  Allocation,
  Announcement,
  AnnouncementInput,
  AnnouncementList,
  AssignmentBody,
  AssignmentResult,
  AuditTrail,
  AuditorFindings,
  AvailabilityRange,
  BenefitBoard,
  BlockedWork,
  BoardPack,
  BudgetBoard,
  BufferManagement,
  Burndown,
  CapacityView,
  ChainBuffers,
  ChangeRequestList,
  Closeout,
  ConformanceFindings,
  Contour,
  ControlledDocument,
  CostPlan,
  CostRateTable,
  CostRisk,
  Cpm,
  CreateAllocationBody,
  CreateBenefitBody,
  CreateBudgetLineBody,
  CreateIdeaBody,
  CreateKeyResultBody,
  CreateKpiBody,
  CreateMilestoneBody,
  CreateObjectiveBody,
  CreateProposalBody,
  CreateReportBody,
  CreateRiskBody,
  CreateScenarioBody,
  CreateSprintBody,
  CreateSprintNoteBody,
  CreateTaskBody,
  CriticalChain,
  Dashboard,
  DecisionEntry,
  DeliveryLinks,
  DemandHit,
  DependencyRisk,
  DevopsMetrics,
  DevopsReleases,
  DirectorPack,
  DirectorPackMeta,
  DocumentInForce,
  DocumentVersion,
  DrumView,
  EarnedValue,
  Escalation,
  EscalationRegister,
  EvForecast,
  EvRollup,
  EvSeries,
  ExecutiveBenefits,
  ExecutiveHealth,
  FinancialExposure,
  FinancialVariance,
  FlowMetrics,
  Framework,
  FrameworkFamily,
  FrameworkInForce,
  GateJourney,
  GovernanceSummary,
  Grain,
  HandoverPreview,
  Idea,
  Investment,
  IssueRegister,
  JointRisk,
  KeyResult,
  Kpi,
  KpiList,
  KpiStatus,
  KpiView,
  LabourCosts,
  LevelingApplyResult,
  LevelingProposal,
  Milestone,
  MilestoneCalendar,
  MilestoneList,
  MilestoneMode,
  MilestoneStatus,
  MilestoneView,
  MoscowView,
  MyWork,
  Objective,
  OkrList,
  OnboardingCase,
  OnboardingCaseSummary,
  OnboardingKind,
  Pipeline,
  PlanLink,
  PlanModules,
  ProcessDefinition,
  ProcessRun,
  Proposal,
  RaciFinding,
  RaciMatrix,
  RealizeBenefitBody,
  RegulatorExtract,
  ReportDefinition,
  ReportRun,
  ResourcePlan,
  ReviewGateBody,
  Risk,
  RiskHeatmap,
  RiskRegister,
  RoleTasks,
  SandboxCommitted,
  SandboxEdit,
  SandboxPreview,
  Scenario,
  ScenarioComparison,
  ScenarioEvaluation,
  ScheduleBaseline,
  ScheduleHealth,
  ScheduleMoved,
  ScheduleRisk,
  ScheduleVariance,
  ScheduleView,
  SimOptions,
  SkillPool,
  Sprint,
  SprintNote,
  Standup,
  Task,
  TaskCost,
  TaskScheduleBody,
  TechDebtRegister,
  TechnologyRadar,
  ToleranceBoard,
  TolerancePicture,
  TrackerImportReport,
  TrendSeries,
  UpdateKeyResultBody,
  UpdateKpiBody,
  Velocity,
  WorkCalendarView,
  WorkProfile,
} from "./ppm-types";

export * from "./ppm-types";

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
  createProposal(body: CreateProposalBody): Promise<{ pid: string }> {
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
  createIdea(body: CreateIdeaBody): Promise<{ pid: string }> {
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
  reviewGate(
    pid: string,
    body: ReviewGateBody,
  ): Promise<{ stage: string | null }> {
    return this.http.post(`/api/plans/${pid}/gate-reviews`, { body });
  }
  listRisks(pid: string): Promise<Risk[]> {
    return this.http.get(`/api/plans/${pid}/risks`);
  }
  createRisk(pid: string, body: CreateRiskBody): Promise<{ pid: string }> {
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
  createBudgetLine(
    pid: string,
    body: CreateBudgetLineBody,
  ): Promise<{ pid: string }> {
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
  createBenefit(
    pid: string,
    body: CreateBenefitBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/benefits`, { body });
  }
  realizeBenefit(
    pid: string,
    benefitPid: string,
    body: RealizeBenefitBody,
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
  createMilestone(
    pid: string,
    body: CreateMilestoneBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/milestones`, { body });
  }
  /** A linked milestone with open tasks needs a `reason` (422 without one). */
  completeMilestone(
    pid: string,
    milestonePid: string,
    reason?: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${pid}/milestones/${milestonePid}/complete`,
      { body: reason ? { reason } : {} },
    );
  }
  portfolioMilestones(
    params: { status?: MilestoneStatus; plan?: string; kind?: string } = {},
  ): Promise<MilestoneList> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/milestones${qs ? `?${qs}` : ""}`);
  }
  /** Change requests across the plans the caller can read. */
  changeRequests(
    params: { status?: string; open?: boolean } = {},
  ): Promise<ChangeRequestList> {
    const q = new URLSearchParams();
    if (params.status) q.set("status", params.status);
    if (params.open) q.set("open", "true");
    const qs = q.toString();
    return this.http.get(`/api/change-requests${qs ? `?${qs}` : ""}`);
  }
  /** The critical chain beside the critical path (advice only). */
  criticalChain(pid: string): Promise<CriticalChain> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(pid)}/critical-chain`,
    );
  }
  /** Project, feeding and resource buffers for the chain (advice only). */
  chainBuffers(pid: string): Promise<ChainBuffers> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(pid)}/critical-chain-buffers`,
    );
  }
  /** Freeze the chain and buffers as planned. */
  freezeBufferPlan(pid: string): Promise<unknown> {
    return this.http.post(`/api/plans/${encodeURIComponent(pid)}/buffer-plan`, {
      body: {},
    });
  }
  /** Every buffer against the live schedule. */
  bufferManagement(pid: string): Promise<BufferManagement> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(pid)}/buffer-management`,
    );
  }
  /** Record every buffer's reading now. */
  takeBufferSnapshot(pid: string): Promise<unknown> {
    return this.http.post(
      `/api/plans/${encodeURIComponent(pid)}/buffer-management/snapshots`,
      { body: {} },
    );
  }
  /** Record what was agreed to do about a buffer. */
  addRecoveryAction(
    pid: string,
    body: {
      buffer_key: string;
      action: string;
      owner_ref: string;
      due_date?: string;
    },
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${encodeURIComponent(pid)}/buffer-recovery-actions`,
      { body },
    );
  }
  /** Close an open recovery action as done or dropped. */
  closeRecoveryAction(
    pid: string,
    actionPid: string,
    status: "done" | "dropped",
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${encodeURIComponent(pid)}/buffer-recovery-actions/${encodeURIComponent(actionPid)}/status`,
      { body: { status } },
    );
  }
  /** The cutover go/no-go decisions of a plan, newest first. */
  cutoverDecisions(
    planPid: string,
  ): Promise<{ decisions: CutoverDecision[]; required_cosigners: string[] }> {
    return this.http.get(`/api/plans/${planPid}/cutover-decisions`);
  }
  createCutoverDecision(
    planPid: string,
    title: string,
    criteria: string[],
  ): Promise<unknown> {
    return this.http.post(`/api/plans/${planPid}/cutover-decisions`, {
      body: { title, criteria },
    });
  }
  /** One step on a decision: publish, open or new-version. */
  cutoverStep(
    planPid: string,
    did: string,
    step: "publish" | "open" | "new-version",
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/cutover-decisions/${did}/${step}`,
      { body: {} },
    );
  }
  markCutoverCriterion(
    planPid: string,
    did: string,
    n: number,
    met: boolean,
    evidence: string,
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${planPid}/cutover-decisions/${did}/criteria/${n}/mark`,
      { body: { met, evidence } },
    );
  }
  cosignCutover(planPid: string, did: string, role: string): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/cutover-decisions/${did}/cosign`,
      { body: { role } },
    );
  }
  decideCutover(
    planPid: string,
    did: string,
    outcome: string,
    reason: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/cutover-decisions/${did}/decide`,
      { body: { outcome, reason } },
    );
  }
  /** Requests to move a task's start, with the knock-on worked out when each was asked. */
  deploymentMoves(planPid: string): Promise<DeploymentMoves> {
    return this.http.get(`/api/plans/${planPid}/deployment-moves`);
  }
  requestDeploymentMove(
    planPid: string,
    body: {
      task_pid: string;
      new_start: string;
      reason: string;
      approver_ref?: string;
    },
  ): Promise<unknown> {
    return this.http.post(`/api/plans/${planPid}/deployment-moves`, { body });
  }
  decideDeploymentMove(
    planPid: string,
    movePid: string,
    decision: "approve" | "refuse",
    reason: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/deployment-moves/${movePid}/decision`,
      { body: { decision, reason } },
    );
  }
  /** The funding allocations, each with the money on lines that draw on it and what is untraced. */
  fundingAllocations(): Promise<{ allocations: FundingAllocation[] }> {
    return this.http.get("/api/funding-allocations");
  }
  /** What one allocation bought: lines, gates, milestones met and benefits validated. */
  traceAllocation(pid: string): Promise<AllocationTrace> {
    return this.http.get(`/api/funding-allocations/${pid}/trace`);
  }
  /** Milestones that need partner sign-off, and where each stands. */
  milestoneSignoffs(): Promise<MilestoneSignoffs> {
    return this.http.get("/api/milestone-signoffs");
  }
  /** An organization signs a milestone off, or withdraws its sign-off. */
  actOnSignoff(
    planPid: string,
    milestonePid: string,
    organizationPid: string,
    action: "sign" | "withdraw",
    reason?: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/milestones/${milestonePid}/signoffs`,
      {
        body: {
          organization_pid: organizationPid,
          action,
          ...(reason ? { reason } : {}),
        },
      },
    );
  }
  /** Key people and roles, and the contingent workers (resource managers only). */
  keyPersons(): Promise<KeyPersons> {
    return this.http.get("/api/key-persons");
  }
  recordKeyPerson(entry: {
    subject_kind: string;
    subject: string;
    successor_ref?: string;
    readiness: string;
    note?: string;
  }): Promise<unknown> {
    return this.http.put("/api/key-persons", { body: entry });
  }
  removeKeyPerson(pid: string): Promise<unknown> {
    return this.http.delete(`/api/key-persons/${pid}`);
  }
  setContingent(personRef: string, contingent: boolean): Promise<unknown> {
    return this.http.put("/api/key-persons/contingent", {
      body: { person_ref: personRef, contingent },
    });
  }
  /** The partner organizations. */
  organizations(): Promise<{ organizations: Organization[] }> {
    return this.http.get("/api/organizations");
  }
  addOrganization(name: string, kind: string): Promise<unknown> {
    return this.http.post("/api/organizations", { body: { name, kind } });
  }
  /** Rename, change kind, retire (`active: false`) or restore an organization. */
  changeOrganization(
    pid: string,
    change: { name?: string; kind?: string; active?: boolean },
  ): Promise<unknown> {
    return this.http.put(`/api/organizations/${pid}`, { body: change });
  }
  /** Email delivery rows, one for each recipient (administrators only). */
  emailDeliveries(status?: string): Promise<EmailDeliveries> {
    return this.http.get(
      `/api/email-deliveries${status ? `?status=${encodeURIComponent(status)}` : ""}`,
    );
  }
  /** Programmes refused a start, in the order they asked. */
  startQueue(): Promise<StartQueue> {
    return this.http.get("/api/start-queue");
  }
  /** Take a programme out of the start queue. */
  withdrawFromStartQueue(planPid: string): Promise<unknown> {
    return this.http.post(`/api/plans/${planPid}/start-queue/withdraw`, {
      body: {},
    });
  }
  /** Which skill pool is the portfolio's drum, and why. */
  portfolioDrum(): Promise<{
    drum: DrumView;
    history: { skill_pool_pid: string | null; declared_on: string }[];
  }> {
    return this.http.get("/api/portfolio-drum");
  }
  /** Name the drum, or clear the naming with null. */
  setPortfolioDrum(skillPoolPid: string | null): Promise<unknown> {
    return this.http.put("/api/portfolio-drum", {
      body: { skill_pool_pid: skillPoolPid },
    });
  }
  /** Proposed staggered starts for the projects that need the drum. */
  portfolioPipeline(): Promise<Pipeline> {
    return this.http.get("/api/portfolio-pipeline");
  }
  /** The skill pools. */
  skillPools(): Promise<{ skill_pools: SkillPool[] }> {
    return this.http.get("/api/skill-pools");
  }
  /** The director packs the caller may read, without their bodies. */
  directorPacks(): Promise<{ packs: DirectorPackMeta[] }> {
    return this.http.get("/api/director-packs");
  }
  /** One director pack with its frozen body. */
  directorPack(pid: string): Promise<DirectorPack> {
    return this.http.get(`/api/director-packs/${encodeURIComponent(pid)}`);
  }
  /** The issues register across the plans the caller can read. */
  issues(
    params: {
      plan?: string;
      status?: string;
      severity?: string;
      owner?: string;
      open?: boolean;
      min_age_days?: number;
    } = {},
  ): Promise<IssueRegister> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params))
      if (v !== undefined && v !== "" && v !== false) q.set(k, String(v));
    const qs = q.toString();
    return this.http.get(`/api/issues${qs ? `?${qs}` : ""}`);
  }
  milestone(pid: string): Promise<MilestoneView> {
    return this.http.get(`/api/milestones/${pid}`);
  }
  rescheduleMilestone(
    pid: string,
    milestonePid: string,
    due: string,
    reason?: string,
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/milestones/${milestonePid}`, {
      body: { due, reason: reason || undefined },
    });
  }
  reopenMilestone(
    pid: string,
    milestonePid: string,
    reason: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${pid}/milestones/${milestonePid}/reopen`,
      { body: { reason } },
    );
  }
  setMilestoneLinks(
    pid: string,
    milestonePid: string,
    taskPids: string[],
    completionMode?: MilestoneMode,
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/milestones/${milestonePid}/tasks`, {
      body: { task_pids: taskPids, completion_mode: completionMode },
    });
  }
  allocations(pid: string): Promise<Allocation[]> {
    return this.http.get(`/api/plans/${pid}/allocations`);
  }
  /** Make a proposed booking firm; a 409 `over_capacity` needs `confirm` to go ahead. */
  commitAllocation(
    pid: string,
    allocationPid: string,
    confirm = false,
  ): Promise<{ pid: string; booking: string; over_capacity: boolean }> {
    return this.http.post(
      `/api/plans/${pid}/allocations/${allocationPid}/commit${confirm ? "?confirm=true" : ""}`,
    );
  }
  availability(personRef?: string): Promise<AvailabilityRange[]> {
    const q = personRef ? `?person_ref=${encodeURIComponent(personRef)}` : "";
    return this.http.get(`/api/availability${q}`);
  }
  setAvailability(body: {
    person_ref?: string;
    from: string;
    to: string;
    max_percent: number;
    note?: string;
  }): Promise<AvailabilityRange> {
    return this.http.post("/api/availability", { body });
  }
  removeAvailability(pid: string): Promise<unknown> {
    return this.http.post(`/api/availability/${pid}/delete`);
  }
  createAllocation(
    pid: string,
    body: CreateAllocationBody,
  ): Promise<{ pid: string }> {
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
  createScenario(body: CreateScenarioBody): Promise<{ pid: string }> {
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
  createObjective(body: CreateObjectiveBody): Promise<{ pid: string }> {
    return this.http.post("/api/objectives", { body });
  }
  alignment(pid: string): Promise<Alignment> {
    return this.http.get(`/api/objectives/${pid}/alignment`);
  }

  // ---- scheduling canon (SC-6 .. SC-9) ----
  scheduleHealth(pid: string, asOf?: string): Promise<ScheduleHealth> {
    return this.http.get(
      `/api/plans/${pid}/schedule-health${asOf ? `?as_of=${asOf}` : ""}`,
    );
  }
  levelingProposal(
    pid: string,
    opts: { allowCritical?: boolean; loadPercent?: number } = {},
  ): Promise<LevelingProposal> {
    const q = new URLSearchParams();
    if (opts.allowCritical) q.set("allow_critical", "true");
    if (opts.loadPercent) q.set("load_percent", String(opts.loadPercent));
    const qs = q.toString();
    return this.http.get(`/api/plans/${pid}/leveling${qs ? `?${qs}` : ""}`);
  }
  applyLeveling(
    pid: string,
    body: {
      moves: { task_pid: string; start_date: string }[];
      confirm: boolean;
      allow_critical?: boolean;
      load_percent?: number;
      reason?: string;
    },
  ): Promise<LevelingApplyResult> {
    return this.http.post(`/api/plans/${pid}/leveling/apply`, { body });
  }
  planEscalations(
    pid: string,
    status?: string,
  ): Promise<{ plan_pid: string; escalations: Escalation[] }> {
    return this.http.get(
      `/api/plans/${pid}/escalations${status ? `?status=${status}` : ""}`,
    );
  }
  escalation(pid: string, epid: string): Promise<Escalation> {
    return this.http.get(`/api/plans/${pid}/escalations/${epid}`);
  }
  raiseEscalation(
    pid: string,
    body: Record<string, unknown>,
  ): Promise<Escalation> {
    return this.http.post(`/api/plans/${pid}/escalations`, { body });
  }
  escalationAction(
    pid: string,
    epid: string,
    action: "acknowledge" | "raise-level" | "resolve" | "withdraw",
    body: Record<string, unknown> = {},
  ): Promise<Escalation> {
    return this.http.post(`/api/plans/${pid}/escalations/${epid}/${action}`, {
      body,
    });
  }
  escalationRegister(
    params: { status?: string; overdue?: boolean } = {},
  ): Promise<EscalationRegister> {
    const q = new URLSearchParams();
    if (params.status) q.set("status", params.status);
    if (params.overdue) q.set("overdue", "true");
    const qs = q.toString();
    return this.http.get(`/api/escalations${qs ? `?${qs}` : ""}`);
  }
  frameworks(family: FrameworkFamily): Promise<Framework[]> {
    return this.http.get(`/api/${family}`);
  }
  createFramework(
    family: FrameworkFamily,
    body: Record<string, unknown>,
  ): Promise<Framework> {
    return this.http.post(`/api/${family}`, { body });
  }
  updateFramework(
    family: FrameworkFamily,
    pid: string,
    body: Record<string, unknown>,
  ): Promise<Framework> {
    return this.http.put(`/api/${family}/${pid}`, { body });
  }
  deleteFramework(family: FrameworkFamily, pid: string): Promise<unknown> {
    return this.http.post(`/api/${family}/${pid}/delete`, { body: {} });
  }
  frameworkInForce(
    family: FrameworkFamily,
    pid: string,
  ): Promise<FrameworkInForce> {
    return this.http.get(
      `/api/plans/${pid}/${family === "ways-of-working" ? "ways-of-working" : "governance-framework"}`,
    );
  }
  /** Follow `frameworkPid`, or follow none of its own (the plan then inherits) with null. */
  linkFramework(
    family: FrameworkFamily,
    pid: string,
    frameworkPid: string | null,
  ): Promise<FrameworkInForce> {
    return this.http.put(
      `/api/plans/${pid}/${family === "ways-of-working" ? "ways-of-working" : "governance-framework"}`,
      {
        body: { framework_pid: frameworkPid },
      },
    );
  }
  /** A plan's links, or a task's when `taskPid` is given. */
  planLinks(pid: string, taskPid?: string): Promise<PlanLink[]> {
    return this.http.get(
      `/api/plans/${pid}${taskPid ? `/tasks/${taskPid}` : ""}/links`,
    );
  }
  /** How many live links each of a plan's tasks has (tasks with none are absent). */
  taskLinkCounts(
    pid: string,
  ): Promise<{ plan_pid: string; counts: Record<string, number> }> {
    return this.http.get(`/api/plans/${pid}/task-link-counts`);
  }
  addPlanLink(
    pid: string,
    body: {
      kind: string;
      url: string;
      label: string;
      system?: string;
      external_id?: string;
    },
    taskPid?: string,
  ): Promise<PlanLink> {
    return this.http.post(
      `/api/plans/${pid}${taskPid ? `/tasks/${taskPid}` : ""}/links`,
      { body },
    );
  }
  removePlanLink(
    pid: string,
    lpid: string,
    taskPid?: string,
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${pid}${taskPid ? `/tasks/${taskPid}` : ""}/links/${lpid}/delete`,
      { body: {} },
    );
  }
  getTask(pid: string, taskPid: string): Promise<Task> {
    return this.http.get(`/api/plans/${pid}/tasks/${taskPid}`);
  }
  planTolerances(pid: string): Promise<TolerancePicture> {
    return this.http.get(`/api/plans/${pid}/tolerances`);
  }
  setTolerance(
    pid: string,
    dimension: string,
    body: { limit: number; amber_from_percent?: number; note?: string | null },
  ): Promise<TolerancePicture> {
    return this.http.put(`/api/plans/${pid}/tolerances/${dimension}`, { body });
  }
  /** Raise one escalation for every red dimension that has no live one; asking again raises nothing new. */
  escalateTolerances(pid: string): Promise<{
    raised: { pid: string; tolerance_dimension: string | null }[];
    tolerances: TolerancePicture;
  }> {
    return this.http.post(`/api/plans/${pid}/tolerances/escalate`, {
      body: {},
    });
  }
  removeTolerance(pid: string, dimension: string): Promise<TolerancePicture> {
    return this.http.post(`/api/plans/${pid}/tolerances/${dimension}/delete`, {
      body: {},
    });
  }
  toleranceBoard(
    params: { status?: string; dimension?: string } = {},
  ): Promise<ToleranceBoard> {
    const q = new URLSearchParams();
    if (params.status) q.set("status", params.status);
    if (params.dimension) q.set("dimension", params.dimension);
    const qs = q.toString();
    return this.http.get(`/api/tolerances${qs ? `?${qs}` : ""}`);
  }
  listProcesses(): Promise<ProcessDefinition[]> {
    return this.http.get("/api/processes");
  }
  createProcess(body: {
    name: string;
    steps: { kind: string; person_ref: string; title: string }[];
  }): Promise<ProcessDefinition> {
    return this.http.post("/api/processes", { body });
  }
  startProcess(
    pid: string,
    body: { subject_kind: string; subject_pid: string },
  ): Promise<ProcessRun> {
    return this.http.post(`/api/processes/${pid}/start`, { body });
  }
  listProcessRuns(): Promise<ProcessRun[]> {
    return this.http.get("/api/process-executions");
  }
  processRun(pid: string): Promise<ProcessRun> {
    return this.http.get(`/api/process-executions/${pid}`);
  }
  completeProcessStep(pid: string, index: number): Promise<ProcessRun> {
    return this.http.post(
      `/api/process-executions/${pid}/steps/${index}/complete`,
      { body: {} },
    );
  }
  cancelProcessRun(pid: string): Promise<ProcessRun> {
    return this.http.post(`/api/process-executions/${pid}/cancel`, {
      body: {},
    });
  }
  resumeProcessRun(pid: string): Promise<ProcessRun> {
    return this.http.post(`/api/process-executions/${pid}/resume`, {
      body: {},
    });
  }
  /** What the people in a role are working on (needs the right to manage people). */
  roleTasks(role: string): Promise<RoleTasks> {
    return this.http.get(
      `/api/resource-plan/role-tasks?role=${encodeURIComponent(role)}`,
    );
  }
  resourcePlan(
    params: {
      from?: string;
      to?: string;
      period?: "week" | "month";
      group_by?: "role" | "skill";
      scenario?: string;
    } = {},
  ): Promise<ResourcePlan> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/resource-plan${qs ? `?${qs}` : ""}`);
  }
  scheduleRisk(
    pid: string,
    opts: {
      iterations?: number;
      seed?: number;
      distribution?: "triangular" | "pert";
      fallbackBandPercent?: number;
    } = {},
  ): Promise<ScheduleRisk> {
    const q = new URLSearchParams();
    if (opts.iterations) q.set("iterations", String(opts.iterations));
    if (opts.seed !== undefined) q.set("seed", String(opts.seed));
    if (opts.distribution) q.set("distribution", opts.distribution);
    if (opts.fallbackBandPercent !== undefined)
      q.set("fallback_band_percent", String(opts.fallbackBandPercent));
    const qs = q.toString();
    return this.http.get(
      `/api/plans/${pid}/schedule-risk${qs ? `?${qs}` : ""}`,
    );
  }

  // ---- saved views (the user's own; also holds display preferences) ----
  listSavedViews(
    route?: string,
  ): Promise<{ pid: string; route: string; name: string; filter: unknown }[]> {
    return this.http.get(
      `/api/saved-views${route ? `?route=${encodeURIComponent(route)}` : ""}`,
    );
  }
  createSavedView(body: {
    route: string;
    name: string;
    filter?: unknown;
    sort?: unknown;
    columns?: string[];
  }): Promise<{ pid: string }> {
    return this.http.post("/api/saved-views", { body });
  }
  deleteSavedView(pid: string): Promise<void> {
    return this.http.delete(`/api/saved-views/${pid}`);
  }

  // ---- simulation inputs ----
  setTaskEstimate(
    pid: string,
    taskPid: string,
    body: { optimistic_days: number | null; pessimistic_days: number | null },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/estimate`, {
      body,
    });
  }
  setLineEstimate(
    pid: string,
    linePid: string,
    body: { low_minor: number | null; high_minor: number | null },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/budget-lines/${linePid}/estimate`, {
      body,
    });
  }
  setRiskScheduleImpact(
    pid: string,
    riskPid: string,
    body: {
      task_pid: string | null;
      /** Business days it adds. */
      impact_days: number | null;
      probability_percent: number | null;
    },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/risks/${riskPid}/schedule-impact`, {
      body,
    });
  }
  setRiskCostImpact(
    pid: string,
    riskPid: string,
    body: {
      cost_impact_minor: number | null;
      cost_currency: string | null;
      probability_percent?: number;
    },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/risks/${riskPid}/cost-impact`, {
      body,
    });
  }

  // ---- cost-risk and joint Monte Carlo ----
  costRisk(pid: string, opts: SimOptions = {}): Promise<CostRisk> {
    return this.http.get(`/api/plans/${pid}/cost-risk${simQuery(opts)}`);
  }
  portfolioCostRisk(opts: SimOptions = {}): Promise<CostRisk> {
    return this.http.get(`/api/cost-risk${simQuery(opts)}`);
  }
  jointRisk(
    pid: string,
    opts: SimOptions & {
      currency?: string;
      delayCostPerDayMinor?: number;
    } = {},
  ): Promise<JointRisk> {
    return this.http.get(`/api/plans/${pid}/joint-risk${simQuery(opts)}`);
  }

  // ---- OKRs ----
  okrs(period?: string): Promise<OkrList> {
    const q = period ? `?period=${encodeURIComponent(period)}` : "";
    return this.http.get(`/api/okrs${q}`);
  }
  updateObjective(
    pid: string,
    body: {
      title: string;
      description?: string | null;
      period?: string | null;
    },
  ): Promise<Objective> {
    return this.http.put(`/api/objectives/${pid}`, { body });
  }
  deleteObjective(pid: string): Promise<void> {
    return this.http.delete(`/api/objectives/${pid}`);
  }
  createKeyResult(
    objectivePid: string,
    body: CreateKeyResultBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/objectives/${objectivePid}/key-results`, {
      body,
    });
  }
  updateKeyResult(pid: string, body: UpdateKeyResultBody): Promise<KeyResult> {
    return this.http.put(`/api/key-results/${pid}`, { body });
  }
  deleteKeyResult(pid: string): Promise<void> {
    return this.http.delete(`/api/key-results/${pid}`);
  }
  checkIn(
    keyResultPid: string,
    body: { value: number; confidence?: number | null; note?: string | null },
  ): Promise<unknown> {
    return this.http.post(`/api/key-results/${keyResultPid}/check-ins`, {
      body,
    });
  }

  // ---- KPIs ----
  kpis(
    params: { plan?: string; objective?: string; status?: KpiStatus } = {},
  ): Promise<KpiList> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/kpis${qs ? `?${qs}` : ""}`);
  }
  kpi(pid: string): Promise<KpiView> {
    return this.http.get(`/api/kpis/${pid}`);
  }
  createKpi(body: CreateKpiBody): Promise<{ pid: string }> {
    return this.http.post("/api/kpis", { body });
  }
  updateKpi(pid: string, body: UpdateKpiBody): Promise<Kpi> {
    return this.http.put(`/api/kpis/${pid}`, { body });
  }
  deleteKpi(pid: string): Promise<void> {
    return this.http.delete(`/api/kpis/${pid}`);
  }
  addKpiReading(
    pid: string,
    body: { value: number; observed_on?: string; note?: string | null },
  ): Promise<{ reading_pid: string; kpi: KpiView }> {
    return this.http.post(`/api/kpis/${pid}/readings`, { body });
  }

  // ---- reports (PPM-9) ----
  listReports(): Promise<ReportDefinition[]> {
    return this.http.get("/api/reports");
  }
  createReport(body: CreateReportBody): Promise<{ pid: string }> {
    return this.http.post("/api/reports", { body });
  }
  runReport(pid: string): Promise<ReportRun> {
    return this.http.get(`/api/reports/${pid}/run`);
  }

  // ---- tracker import (T-28n) ----
  async importTracker(
    pid: string,
    source: "jira" | "asana" | "mspdi",
    file: string,
    dryRun: boolean,
  ): Promise<TrackerImportReport> {
    const url = `/api/plans/${pid}/import/${source}?dry_run=${dryRun}`;
    const type = source === "mspdi" ? "application/xml" : "text/csv";
    const raw = await this.http.post<Record<string, unknown>>(url, {
      rawBody: file,
      headers: { "content-type": type },
    });
    return source === "mspdi"
      ? mspdiReport(raw)
      : rowOf<TrackerImportReport>(raw);
  }

  // ---- controlled documents (PL-6) ----
  documents(): Promise<ControlledDocument[]> {
    return this.http.get("/api/documents");
  }
  document(pid: string): Promise<ControlledDocument> {
    return this.http.get(`/api/documents/${pid}`);
  }
  documentVersion(pid: string, version: number): Promise<DocumentVersion> {
    return this.http.get(`/api/documents/${pid}/versions/${version}`);
  }
  documentInForce(pid: string, on?: string): Promise<DocumentInForce> {
    return this.http.get(
      `/api/documents/${pid}/in-force${on ? `?on=${encodeURIComponent(on)}` : ""}`,
    );
  }
  createDocument(body: {
    title: string;
    body?: string;
    plan_pid?: string;
    change_summary?: string;
  }): Promise<ControlledDocument> {
    return this.http.post("/api/documents", { body });
  }
  newDocumentVersion(
    pid: string,
    body: { body?: string; change_summary?: string } = {},
  ): Promise<DocumentVersion> {
    return this.http.post(`/api/documents/${pid}/versions`, { body });
  }
  editDocumentVersion(
    pid: string,
    version: number,
    body: { body?: string; change_summary?: string },
  ): Promise<DocumentVersion> {
    return this.http.put(`/api/documents/${pid}/versions/${version}`, { body });
  }
  submitDocumentVersion(
    pid: string,
    version: number,
    body: { approver_ref: string; message?: string },
  ): Promise<DocumentVersion> {
    return this.http.post(`/api/documents/${pid}/versions/${version}/submit`, {
      body,
    });
  }
  makeDocumentVersionEffective(
    pid: string,
    version: number,
  ): Promise<DocumentVersion> {
    return this.http.post(
      `/api/documents/${pid}/versions/${version}/make-effective`,
      { body: {} },
    );
  }
  decideApproval(
    pid: string,
    body: { decision: "approved" | "rejected"; comment?: string },
  ): Promise<unknown> {
    return this.http.post(`/api/approvals/${pid}/decide`, { body });
  }

  // ---- executive insight areas (CEO / CFO / CTO) ----
  announcements(
    params: { include?: "all"; limit?: number } = {},
  ): Promise<AnnouncementList> {
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
  setAssignment(
    pid: string,
    taskPid: string,
    body: AssignmentBody,
  ): Promise<AssignmentResult> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/assignment`, {
      body,
    });
  }
  setContour(
    pid: string,
    taskPid: string,
    contour: Contour,
    custom?: number[],
  ): Promise<{ contour: Contour }> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/contour`, {
      body: { contour, custom },
    });
  }
  workProfile(
    pid: string,
    params: { period?: Grain; from?: string; to?: string } = {},
  ): Promise<WorkProfile> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/plans/${pid}/work-profile${qs ? `?${qs}` : ""}`);
  }
  costRateTables(): Promise<{ tables: CostRateTable[] }> {
    return this.http.get("/api/cost-rate-tables");
  }
  createCostRateTable(body: {
    name: string;
    person_ref?: string;
    role?: string;
    currency: string;
  }): Promise<{ pid: string }> {
    return this.http.post("/api/cost-rate-tables", { body });
  }
  addCostRate(
    tablePid: string,
    body: { effective_from: string; per_hour_minor: number },
  ): Promise<unknown> {
    return this.http.post(`/api/cost-rate-tables/${tablePid}/rates`, { body });
  }
  taskCosts(pid: string, taskPid: string): Promise<{ costs: TaskCost[] }> {
    return this.http.get(`/api/plans/${pid}/tasks/${taskPid}/costs`);
  }
  addTaskCost(
    pid: string,
    taskPid: string,
    body: Omit<TaskCost, "pid">,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/tasks/${taskPid}/costs`, { body });
  }
  removeTaskCost(pid: string, taskPid: string, costPid: string): Promise<void> {
    return this.http.delete(
      `/api/plans/${pid}/tasks/${taskPid}/costs/${costPid}`,
    );
  }
  costPlan(pid: string, params: { period?: Grain } = {}): Promise<CostPlan> {
    const qs = params.period ? `?period=${params.period}` : "";
    return this.http.get(`/api/plans/${pid}/cost-plan${qs}`);
  }
  recordLabourCosts(pid: string): Promise<{
    recorded: number;
    already_recorded: number;
    no_rate: unknown[];
  }> {
    return this.http.post(`/api/plans/${pid}/labour-costs/record`, {
      body: {},
    });
  }
  labourCosts(pid: string): Promise<LabourCosts> {
    return this.http.get(`/api/plans/${pid}/labour-costs`);
  }
  sandboxPreview(pid: string, edits: SandboxEdit[]): Promise<SandboxPreview> {
    return this.http.post(`/api/plans/${pid}/sandbox/preview`, {
      body: { edits },
    });
  }
  sandboxCommit(
    pid: string,
    edits: SandboxEdit[],
    planEtag: string,
    reason?: string,
  ): Promise<SandboxCommitted> {
    return this.http.post(`/api/plans/${pid}/sandbox/commit`, {
      body: { edits, reason: reason || undefined },
      headers: { "If-Match": planEtag },
    });
  }
  onboardingCases(
    pid: string,
    status?: string,
  ): Promise<{
    cases: OnboardingCaseSummary[];
    hidden_people_cases: number;
    masked: boolean;
  }> {
    return this.http.get(
      `/api/plans/${pid}/onboarding/cases${status ? `?status=${status}` : ""}`,
    );
  }
  startOnboarding(
    pid: string,
    body: {
      kind: OnboardingKind;
      subject_ref?: string;
      effective_date: string;
      title?: string;
    },
  ): Promise<OnboardingCase> {
    return this.http.post(`/api/plans/${pid}/onboarding/cases`, { body });
  }
  onboardingCase(casePid: string): Promise<OnboardingCase> {
    return this.http.get(`/api/onboarding/cases/${casePid}`);
  }
  updateOnboardingItem(
    casePid: string,
    itemPid: string,
    body: {
      owner_ref?: string;
      due_date?: string;
      resolution?: "done" | "skipped" | "open";
      note?: string;
    },
  ): Promise<OnboardingCase> {
    return this.http.put(`/api/onboarding/cases/${casePid}/items/${itemPid}`, {
      body,
    });
  }
  completeOnboarding(casePid: string): Promise<OnboardingCase> {
    return this.http.post(`/api/onboarding/cases/${casePid}/complete`, {
      body: {},
    });
  }
  cancelOnboarding(casePid: string, reason: string): Promise<OnboardingCase> {
    return this.http.post(`/api/onboarding/cases/${casePid}/cancel`, {
      body: { reason },
    });
  }
  closeout(pid: string): Promise<Closeout> {
    return this.http.get(`/api/plans/${pid}/closeout`);
  }
  accessRecords(pid: string): Promise<AccessRecords> {
    return this.http.get(`/api/plans/${pid}/access-records`);
  }
  recordAccess(
    pid: string,
    body: {
      person_ref: string;
      access: string;
      action: "granted" | "revoked";
      note?: string;
      case_pid?: string;
    },
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/access-records`, { body });
  }
  handoverPreview(
    pid: string,
    body: { from_person: string; to_person: string | null },
  ): Promise<HandoverPreview> {
    return this.http.post(`/api/plans/${pid}/handover/preview`, { body });
  }
  handoverCommit(
    pid: string,
    body: { from_person: string; to_person: string | null; reason?: string },
    etag: string,
  ): Promise<{ committed: boolean; audit_entries: number }> {
    return this.http.post(`/api/plans/${pid}/handover/commit`, {
      body,
      headers: { "If-Match": etag },
    });
  }
  planCpm(pid: string, near?: number, only?: "critical"): Promise<Cpm> {
    const params = [
      near === undefined ? "" : `near=${near}`,
      only === undefined ? "" : `only=${only}`,
    ].filter(Boolean);
    const q = params.length ? `?${params.join("&")}` : "";
    return this.http.get(`/api/plans/${pid}/cpm${q}`);
  }
  /** Set a task's planned start, duration, constraint and deadline; answers with what moved. */
  setTaskSchedule(
    pid: string,
    taskPid: string,
    body: TaskScheduleBody,
  ): Promise<ScheduleMoved> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/schedule`, {
      body,
    });
  }
  scheduleBaselines(pid: string): Promise<ScheduleBaseline[]> {
    return this.http.get(`/api/plans/${pid}/schedule-baselines`);
  }
  /** Take the next baseline (append-only); a reason is required. */
  takeScheduleBaseline(pid: string, reason: string): Promise<ScheduleBaseline> {
    return this.http.post(`/api/plans/${pid}/schedule-baselines`, {
      body: { reason },
    });
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
  ): Promise<{
    task_pid: string;
    cells: Record<string, string>;
    findings: RaciFinding[];
  }> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/raci`, {
      body: { assignments },
    });
  }
  earnedValue(pid: string): Promise<EarnedValue> {
    return this.http.get(`/api/plans/${pid}/earned-value`);
  }
  setEarnedValueMethod(
    pid: string,
    defaultMethod: "zero_hundred" | "fifty_fifty" | "points",
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/earned-value-settings`, {
      body: { default_method: defaultMethod },
    });
  }
  /** Count one task another way; no `method` clears the override. `manual` needs `percent` and `note`. */
  setTaskPercent(
    pid: string,
    taskPid: string,
    body: {
      method?: "zero_hundred" | "fifty_fifty" | "manual";
      percent?: number;
      note?: string;
    },
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${pid}/tasks/${taskPid}/percent-complete`,
      { body },
    );
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
  setPlanCalendar(
    pid: string,
    body: { weekdays: string; holidays: string[] },
  ): Promise<WorkCalendarView> {
    return this.http.put(`/api/plans/${pid}/calendar`, { body });
  }
  meCalendar(): Promise<WorkCalendarView> {
    return this.http.get("/api/me/calendar");
  }
  setMeCalendar(body: {
    weekdays: string;
    holidays: string[];
  }): Promise<WorkCalendarView> {
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
  /** A plan's handling mark (RG-7). `scheme` is empty when the deployment has none. */
  planHandlingMark(pid: string): Promise<{
    plan_pid: string;
    scheme: string[];
    mark: string | null;
    effective: string | null;
  }> {
    return this.http.get(`/api/plans/${pid}/handling-mark`);
  }
  listTasks(pid: string): Promise<{
    tasks: Task[];
    counts: Record<string, number>;
    /** Every task that matches, not just those returned. */
    total?: number;
    /** More tasks exist than were returned (a very large plan). */
    truncated?: boolean;
  }> {
    return this.http.get(`/api/plans/${pid}/tasks`);
  }
  /** Set the order of one parent's subtasks (`null` for the top level); `order` names every child. */
  setTaskOrder(
    pid: string,
    parentTaskPid: string | null,
    order: string[],
  ): Promise<{ parent_task_pid: string | null; order: string[] }> {
    return this.http.put(`/api/plans/${pid}/task-order`, {
      body: { parent_task_pid: parentTaskPid, order },
    });
  }
  createTask(pid: string, body: CreateTaskBody): Promise<Task> {
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
  createSprint(pid: string, body: CreateSprintBody): Promise<Sprint> {
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
    body: CreateSprintNoteBody,
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

/** Why a programme was refused a start (EX-8). */
export type StartRefusal =
  | { reason: "limit_reached"; in_delivery: number; limit: number }
  | { reason: "constraint_pool_over"; pool: string; months: string[] };

export interface StartQueue {
  queue: {
    position: number;
    plan_pid: string;
    name: string | null;
    queued_on: string;
    refusals: StartRefusal[];
  }[];
  note: string;
}

export interface EmailDeliveries {
  deliveries: {
    pid: string;
    kind: string;
    subject_ref: string;
    recipient: string;
    subject: string;
    status: "queued" | "retry" | "sent" | "failed";
    attempts: number;
    last_error: string | null;
    sender: string | null;
    next_attempt_at: string | null;
    sent_at: string | null;
  }[];
  counts: { retry: number; sent: number; failed: number };
}

export interface Organization {
  pid: string;
  name: string;
  kind: "delivery" | "partner" | "sponsor" | "supplier";
  active: boolean;
  /** The pack audience that addresses this organization (`org:<pid>`). */
  audience: string;
}

export interface KeyPersons {
  entries: {
    pid: string;
    subject_kind: "person" | "role";
    subject: string;
    successor_ref: string | null;
    readiness: "ready" | "partly" | "none";
    skill_pool_pid: string | null;
    note: string | null;
  }[];
  contingent: string[];
}

export interface MilestoneSignoffs {
  totals: {
    requiring_signoff: number;
    delivered: number;
    met: number;
    delivered_not_met: number;
    met_share_of_delivered_bp: number | null;
  };
  milestones: {
    milestone_pid: string;
    plan_pid: string;
    name: string;
    due: string | null;
    signoff: {
      state: "met" | "delivered_not_met" | "not_delivered" | "not_required";
      delivered_on: string | null;
      met_on: string | null;
      outstanding: string[];
    };
  }[];
  note: string;
}

export interface FundingAllocation {
  pid: string;
  funding_source_pid: string;
  funding_source: string;
  amount_minor: number;
  currency: string;
  allocated_on: string;
  reference: string | null;
  planned_on_lines_minor: number;
  untraced_minor: number;
  lines: number;
}

export interface AllocationTrace {
  allocation: FundingAllocation;
  currency: string;
  lines: {
    line_pid: string;
    plan_pid: string;
    plan: string | null;
    description: string;
    category: string;
    planned_minor: number;
    actual_minor: number;
    gate: string | null;
    released: boolean;
  }[];
  money: {
    allocated_minor: number;
    planned_on_lines_minor: number;
    spent_minor: number;
    untraced_minor: number;
    untraced_reason: string | null;
  };
  gates: { lines_with_a_gate: number; released: number };
  milestones: {
    met: number;
    total: number;
    items: {
      plan: string | null;
      name: string;
      due: string | null;
      met: boolean;
      met_on: string | null;
    }[];
  };
  benefits: {
    validated_by_currency: { currency: string; validated_minor: number }[];
    planned_by_currency: { currency: string; planned_minor: number }[];
    benefits: number;
  };
  note: string;
}

export interface DeploymentMove {
  pid: string;
  plan_pid: string;
  task_pid: string;
  from_start: string;
  to_start: string;
  reason: string;
  requested_by: string | null;
  approver_ref: string;
  status: "requested" | "approved" | "refused";
  decided_by: string | null;
  decided_on: string | null;
  decision_reason: string | null;
  knock_on: {
    task_pid: string;
    title: string;
    plan_pid: string;
    other_deployment: boolean;
    moved: boolean;
    start_days: number;
    finish_days: number;
  }[];
  knock_on_cost: { labour?: { currency: string; delta_minor: number }[] };
}

export interface DeploymentMoves {
  moves: DeploymentMove[];
  cost_by_requester: {
    requested_by: string;
    requests: number;
    knock_on_labour: { currency: string; delta_minor: number }[];
  }[];
  note: string;
}

export interface CutoverDecision {
  pid: string;
  plan_pid: string;
  title: string;
  version: number;
  supersedes_pid: string | null;
  status: "draft" | "published" | "open" | "decided" | "superseded";
  published_on: string | null;
  opened_on: string | null;
  outcome: "go" | "no_go" | "delay_for_safety" | null;
  decided_on: string | null;
  decision_reason: string | null;
  criteria: {
    n: number;
    criterion: string;
    met: boolean;
    evidence: string | null;
    marked_on: string | null;
  }[];
  cosignatures: {
    required: string[];
    signed: { role: string; signed_on: string }[];
    missing: string[];
  };
  go_would_be_refused_because: string[];
}
