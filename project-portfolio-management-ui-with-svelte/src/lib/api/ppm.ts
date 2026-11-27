// PPM catalogue client (Phases A–C): typed wrappers over the
// governance / visibility / strategy endpoints, mirroring the
// service's route table (spec 15-roadmap PPM-1..12). One class, one
// method per endpoint, the single source of PPM paths for the UI.
//
// The strings these views show come from the page catalogues: English and
// Welsh (machine-written, unreviewed); the other locales fall back to English.
import { API_BASE_URL } from "#lib/config.js";
import { ApiClient } from "./client";
import type { Collection, Goal } from "./types";
import { rowOf } from "#lib/row-of.js";
import { mspdiReport, simQuery } from "./ppm-types";
import type { PeopleImpact } from "#lib/people-impact.js";
import type {
  Threshold,
  ThresholdAction,
  ThresholdKind,
  Triage,
  TriageOutcome,
} from "#lib/threshold-view.js";
import type { CapacityHeatmap } from "#lib/capacity-heatmap-view.js";
import type { DefBody } from "#lib/custom-field-view.js";
import type {
  CapacityCommitment,
  CommitmentInput,
} from "#lib/capacity-commitments-view.js";
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
  ChangeRequest,
  ChangeRequestBody,
  ChangeRequestDecisionBody,
  ChangeRequestList,
  ChangeRequestSubmitBody,
  ChangeRequestLinksBody,
  ChangeRequestPreview,
  ChangeRequestBaselined,
  PlanChangeRequestList,
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
  RaiseEscalationBody,
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
  EscalationAcknowledgement,
  AcknowledgeEscalationBody,
  EscalationResolution,
  ResolveEscalationBody,
  RegisterCompleteness,
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
  Issue,
  IssueBody,
  IssueOptions,
  IssueType,
  IssueTypeChange,
  NewIssueType,
  NewIssueWorkflow,
  IssueWorkflowChange,
  PlanIssueTypes,
  PlanRoleMember,
  PlanRoleMembers,
  WorkflowDetail,
  NewOrganizationRole,
  OrganizationRole,
  OrganizationRoleChange,
  PlanIssueList,
  WorkflowSummary,
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
  MyInbox,
  NotificationPrefBody,
  NotificationPrefResetBody,
  NotificationPrefs,
  NotificationPref,
  NotificationSink,
  NotificationSinkBody,
  SinkDelivery,
  TimesheetDecideBody,
  AccessReview,
  AccessReviewQuery,
  BreakGlassGrant,
  BreakGlassList,
  BreakGlassOpenBody,
  BreakGlassOpened,
  TimesheetSubmitBody,
  TimesheetWeek,
  TimesheetWeekState,
  Canvas,
  CanvasCreateBody,
  CanvasItem,
  CanvasItemBody,
  CanvasItemUpdateBody,
  CanvasPromoteBody,
  CanvasTemplate,
  CanvasView,
  ConfidenceBody,
  DeliveryException,
  PortfolioMap,
  FundingSource,
  AllocationBody,
  ReconcileReport,
  ProjectBufferBody,
  ProjectBufferView,
  BenefitClassifyBody,
  BenefitValidations,
  DeliveryProfile,
  PlannedGates,
  ProgrammeStatusBody,
  ProgrammeStatusView,
  ExceptionCompleteBody,
  ConfidenceView,
  ConfidenceAssessment,
  DirectorPackDraftBody,
  PackSchedule,
  InstantiateTemplateBody,
  InstantiatedTemplate,
  PlanTemplate,
  PlanTemplateSummary,
  SaveTemplateBody,
  SavedTemplate,
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
  RiskScoreRow,
  RiskUpdateBody,
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
  TaskRelationBody,
  Attachment,
  AttachmentUploaded,
  Comment,
  CommentBody,
  CommentEdit,
  CommentEditBody,
  DiscussionSubjectKind,
  ReactionBody,
  ReactionSummary,
  WatchBody,
  WatchStatus,
  TaskRelationList,
  TaskRelationRow,
  UpdateTaskBody,
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
  ThresholdSweepRow,
  DeescalationTracker,
  ScorecardRow,
  ScorecardView,
  TemplateMeasure,
  MilestoneSignoff,
  PlanStatus,
  StatusCadence,
  StatusDue,
  StatusMetrics,
  StatusUpdate,
  StatusUpdateBody,
  HeldMeetingBody,
  Lesson,
  LessonBody,
  Meeting,
  MeetingSummary,
  PlanMeetingBody,
  ProposalLessonMatch,
  RecordReferences,
  TaskProposalCommit,
  TaskProposalPreview,
  TaskProposalsBody,
  AcSource,
  BudgetBaseline,
  BudgetBaselineBody,
  BusinessCaseBody,
  ForecastRollup,
  PlanForecast,
  ValuePointBody,
  ValueRealization,
  EstimateModel,
  Frontier,
  GenerateScenarioBody,
  GeneratedScenario,
  IntakeForecast,
  ModelRanking,
  PlanModelScore,
  ScoringInputs,
  ScoringInputsView,
  DayPlan,
  FeedTokenIssued,
  MyWatching,
  PlannedSlot,
  PlannedSlotBody,
  PortfolioWorkProfile,
  TimeEntry,
  TimeEntryBody,
  TimeReport,
  TimeReportQuery,
  BufferPlan,
  CcpmEstimateBody,
  CcpmEstimates,
  DeliveryForecast,
  DeliveryForecastQuery,
  DependencyBody,
  DependencyRow,
  FlowDistribution,
  FlowDistributionQuery,
  HammockList,
  HammockSet,
  NonWorkingBody,
  PlanEffort,
  PlanFlowRollup,
  PlanPerformance,
  PlanWorkflow,
  TaskRollup,
  WorkingTimeBody,
  AdoptionBody,
  BulkEditBody,
  BulkResult,
  BulkTransferBody,
  CeremonyBody,
  KeyResultCheckIn,
  KpiReadings,
  ObjectiveKeyResult,
  PlanOkr,
  PlannedHours,
  PlannedHoursQuery,
  RateLookup,
  RateLookupQuery,
  ResolvePlaceholderBody,
  BulkJob,
  BulkJobCancel,
  BulkJobKind,
  BulkJobStarted,
  BulkJobStatus,
  ReviewQueueItem,
  CustomFieldDef,
  OnboardingTemplate,
  OnboardingTemplateBody,
  CustomFieldSearch,
  CustomFieldValues,
  CustomFieldValuesBody,
  GoalList,
  IndexedGoal,
  ResourceRole,
  Component,
  ComponentBody,
  ComponentSummary,
  MspdiBatchAnswer,
  MspdiBatchBody,
  StartExportBody,
  TemplateDiff,
  TemplatePlan,
  TemplateRevised,
  TemplateSyncBody,
  TemplateSynced,
  SkillGap,
  SprintCeremonies,
  SprintCommitment,
  Utilization,
  UtilizationQuery,
} from "./ppm-types";
import { querySearch, readingSearch, timeReportSearch } from "./ppm-types";

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
  /** One proposal as stored, read before editing it. */
  getProposal(pid: string): Promise<Proposal> {
    return this.http.get(`/api/proposals/${pid}`);
  }
  /** Edit a draft proposal: the body replaces every field (EX-13 tags one as an external ask here). */
  updateProposal(pid: string, body: CreateProposalBody): Promise<Proposal> {
    return this.http.put(`/api/proposals/${pid}`, { body });
  }
  /** How many proposals will be approved in the next periods, from the intake's own approvals (T-28i). The service's
   * defaults (4 periods, 7 days, 12 periods of history) are left out of the query. */
  intakeForecast(
    params: {
      periods?: number;
      periodDays?: number;
      historyPeriods?: number;
    } = {},
  ): Promise<IntakeForecast> {
    const q = new URLSearchParams();
    if (params.periods !== undefined) q.set("periods", String(params.periods));
    if (params.periodDays !== undefined)
      q.set("period_days", String(params.periodDays));
    if (params.historyPeriods !== undefined)
      q.set("history_periods", String(params.historyPeriods));
    const s = q.toString();
    return this.http.get(`/api/proposals/forecast${s ? `?${s}` : ""}`);
  }
  /** Name what an external ask displaces (EX-13), until the proposal is decided. */
  setProposalTradeoff(pid: string, tradeoff: string): Promise<Proposal> {
    return this.http.put(`/api/proposals/${pid}/tradeoff`, {
      body: { tradeoff },
    });
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
  /** Change a risk's status, scores, mitigation or review date (only the fields sent; 422 for a rule broken). */
  updateRisk(
    pid: string,
    riskPid: string,
    body: RiskUpdateBody,
  ): Promise<Risk> {
    return this.http.put(`/api/plans/${pid}/risks/${riskPid}`, { body });
  }
  /** A risk's score history, oldest first (OF-3). */
  riskScores(
    pid: string,
    riskPid: string,
  ): Promise<{ risk_pid: string; scores: RiskScoreRow[]; note: string }> {
    return this.http.get(`/api/plans/${pid}/risks/${riskPid}/scores`);
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
  /** One plan's change requests (IR-4), newest first. */
  planChangeRequests(pid: string): Promise<PlanChangeRequestList> {
    return this.http.get(`/api/plans/${pid}/change-requests`);
  }
  /** Draft a change request on a plan (IR-4). */
  draftChangeRequest(
    pid: string,
    body: ChangeRequestBody,
  ): Promise<ChangeRequest> {
    return this.http.post(`/api/plans/${pid}/change-requests`, { body });
  }
  /** Replace a draft's fields (IR-4); 409 once it is no longer a draft, 403 for anyone but its requester. */
  updateChangeRequest(
    pid: string,
    crPid: string,
    body: ChangeRequestBody,
  ): Promise<ChangeRequest> {
    return this.http.put(`/api/plans/${pid}/change-requests/${crPid}`, {
      body,
    });
  }
  /** Send a draft for decision, naming the approver (IR-4). */
  submitChangeRequest(
    pid: string,
    crPid: string,
    body: ChangeRequestSubmitBody,
  ): Promise<ChangeRequest> {
    return this.http.post(`/api/plans/${pid}/change-requests/${crPid}/submit`, {
      body,
    });
  }
  /** The named approver starts reviewing a submitted request (IR-4). */
  reviewChangeRequest(pid: string, crPid: string): Promise<ChangeRequest> {
    return this.http.post(`/api/plans/${pid}/change-requests/${crPid}/review`, {
      body: {},
    });
  }
  /** The named approver approves or rejects, with a reason (IR-4). */
  decideChangeRequest(
    pid: string,
    crPid: string,
    body: ChangeRequestDecisionBody,
  ): Promise<ChangeRequest> {
    return this.http.post(
      `/api/plans/${pid}/change-requests/${crPid}/decision`,
      { body },
    );
  }
  /** The requester (or an administrator) withdraws an open request (IR-4). */
  withdrawChangeRequest(pid: string, crPid: string): Promise<ChangeRequest> {
    return this.http.post(
      `/api/plans/${pid}/change-requests/${crPid}/withdraw`,
      { body: {} },
    );
  }
  /** One change request, with the issues and risks it answers (IR-4). */
  changeRequest(pid: string, crPid: string): Promise<ChangeRequest> {
    return this.http.get(`/api/plans/${pid}/change-requests/${crPid}`);
  }
  /** Mark an approved change as carried out (IR-4); 409 unless approved. */
  implementChangeRequest(pid: string, crPid: string): Promise<ChangeRequest> {
    return this.http.post(
      `/api/plans/${pid}/change-requests/${crPid}/implement`,
      { body: {} },
    );
  }
  /** Replace the issues and risks an open request answers (IR-4). */
  setChangeRequestLinks(
    pid: string,
    crPid: string,
    body: ChangeRequestLinksBody,
  ): Promise<ChangeRequest> {
    return this.http.put(`/api/plans/${pid}/change-requests/${crPid}/links`, {
      body,
    });
  }
  /** The forecast of the request's what-if edits (IR-5); 422 when it carries none. Writes nothing. */
  changeRequestPreview(
    pid: string,
    crPid: string,
  ): Promise<ChangeRequestPreview> {
    return this.http.get(`/api/plans/${pid}/change-requests/${crPid}/preview`);
  }
  /** Take a schedule baseline because of an approved or implemented request, once (IR-5). */
  baselineChangeRequest(
    pid: string,
    crPid: string,
  ): Promise<ChangeRequestBaselined> {
    return this.http.post(
      `/api/plans/${pid}/change-requests/${crPid}/baseline`,
      { body: {} },
    );
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
  /** A plan's latest status update and where it stands against its cadence (NC-4). */
  planStatus(pid: string): Promise<PlanStatus> {
    return this.http.get(`/api/plans/${pid}/status`);
  }
  /** A plan's status cadence, with who set it and when; a 404 when none is set (NC-4). */
  statusCadence(pid: string): Promise<StatusCadence> {
    return this.http.get(`/api/plans/${pid}/status-cadence`);
  }
  /** Set how often a plan expects a status update, 1 to 92 days; this restarts the clock (NC-4). */
  setStatusCadence(pid: string, cadenceDays: number): Promise<StatusCadence> {
    return this.http.put(`/api/plans/${pid}/status-cadence`, {
      body: { cadence_days: cadenceDays },
    });
  }
  /** A plan's status updates, newest first, each claim's sources resolved for the reader (NC-4, BR-2). */
  statusUpdates(pid: string): Promise<StatusUpdate[]> {
    return this.http.get(`/api/plans/${pid}/status-updates`);
  }
  /** Post a status update with its claims (NC-4, BR-2). */
  postStatusUpdate(pid: string, body: StatusUpdateBody): Promise<StatusUpdate> {
    return this.http.post(`/api/plans/${pid}/status-updates`, { body });
  }
  /** How each number a claim may cite is counted (BR-2). */
  statusMetrics(): Promise<StatusMetrics> {
    return this.http.get("/api/status-metrics");
  }
  /** The plans the caller can read whose status update is due, most overdue first (NC-4). */
  statusUpdatesDue(): Promise<StatusDue[]> {
    return this.http.get("/api/status-updates/due");
  }
  /** Replace the organizations whose sign-off a milestone needs (EX-4); an empty list clears it. */
  setSignoffRequired(
    planPid: string,
    milestonePid: string,
    organizationPids: string[],
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${planPid}/milestones/${milestonePid}/signoff-required`,
      {
        body: { organization_pids: organizationPids },
      },
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
  // ---- triage and thresholds (EX-7, NC-1 slice 11) ----
  /** A programme's latest triage and its history, newest first. */
  triage(
    planPid: string,
  ): Promise<{ triage: Triage | null; history: Triage[] }> {
    return this.http.get(`/api/plans/${planPid}/triage`);
  }
  /** Record today's triage; it changes nothing about the programme. */
  setTriage(
    planPid: string,
    outcome: TriageOutcome,
    reason: string,
  ): Promise<Triage> {
    return this.http.put(`/api/plans/${planPid}/triage`, {
      body: { outcome, reason },
    });
  }
  /** A programme's thresholds. */
  thresholds(planPid: string): Promise<{ thresholds: Threshold[] }> {
    return this.http.get(`/api/plans/${planPid}/thresholds`);
  }
  /** Set one threshold, or remove it with `remove: true`. */
  setThreshold(
    planPid: string,
    body: {
      kind: ThresholdKind;
      count?: number;
      date?: string;
      cap_minor?: number;
      currency?: string;
      remove?: boolean;
    },
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${planPid}/thresholds`, { body });
  }
  /** Check every threshold you can read; a crossing opens a delivery exception. Pauses, resets and stops nothing. */
  thresholdSweep(): Promise<{ results: ThresholdSweepRow[]; note: string }> {
    return this.http.post("/api/threshold-sweep", { body: {} });
  }
  /** Record pause, reset, stop or an override (with an expiry) on an exception a threshold opened; it closes it. */
  thresholdDecision(
    exceptionPid: string,
    body: { action: ThresholdAction; reason: string; expiry?: string },
  ): Promise<unknown> {
    return this.http.post(
      `/api/exceptions/${encodeURIComponent(exceptionPid)}/threshold-decision`,
      { body },
    );
  }
  // ---- scorecards and the de-escalation tracker (EX-10, NC-1 slice 11) ----
  /** Every scorecard, by name. */
  scorecards(): Promise<{ scorecards: ScorecardRow[] }> {
    return this.http.get("/api/scorecards");
  }
  /** Add a scorecard; `for_deescalation` makes it the one the tracker reads (at most one is). */
  createScorecard(body: {
    name: string;
    for_deescalation?: boolean;
  }): Promise<{ pid: string; name: string; for_deescalation: boolean }> {
    return this.http.post("/api/scorecards", { body });
  }
  /** A scorecard's KPIs by finished quarter (the service shows 4 unless asked). */
  scorecard(pid: string, quarters?: number): Promise<ScorecardView> {
    const q = quarters === undefined ? "" : `?quarters=${quarters}`;
    return this.http.get(`/api/scorecards/${encodeURIComponent(pid)}${q}`);
  }
  /** Replace a scorecard's KPIs (at most 60). */
  setScorecardKpis(
    pid: string,
    kpiPids: string[],
  ): Promise<{ scorecard_pid: string; kpi_pids: string[] }> {
    return this.http.put(`/api/scorecards/${encodeURIComponent(pid)}/kpis`, {
      body: { kpi_pids: kpiPids },
    });
  }
  /** The starter measures: names, directions and units, with no targets. Nothing is applied by reading it. */
  scorecardTemplate(): Promise<{ measures: TemplateMeasure[]; note?: string }> {
    return this.http.get("/api/scorecard-template");
  }
  /** Create the picked template measures as KPIs with your targets and red lines, and put them on the scorecard. */
  scorecardFromTemplate(
    pid: string,
    measures: Array<{ key: string; target_value: number; red_value: number }>,
  ): Promise<{ scorecard_pid: string; kpi_pids: string[] }> {
    return this.http.post(
      `/api/scorecards/${encodeURIComponent(pid)}/from-template`,
      { body: { measures } },
    );
  }
  /** The de-escalation tracker: the level as the sponsor set it, the run of quarters met, the criteria. */
  deescalation(): Promise<DeescalationTracker> {
    return this.http.get("/api/deescalation");
  }
  /** Record the escalation level the sponsor set (the sponsor or an administrator only). */
  setEscalationLevel(body: {
    level: string;
    note?: string;
  }): Promise<{ level: string; set_on: string }> {
    return this.http.put("/api/deescalation/level", { body });
  }
  /** Add a criterion for coming down; criteria are never edited or retired. */
  addDeescalationCriterion(
    criterion: string,
  ): Promise<{ pid: string; criterion: string }> {
    return this.http.post("/api/deescalation/criteria", {
      body: { criterion },
    });
  }
  /** Record whether a criterion was met in a finished quarter; the latest record speaks. */
  setCriterionEvidence(
    criterionPid: string,
    body: { quarter: string; met: boolean; note: string; link?: string },
  ): Promise<{ criterion_pid: string; quarter: string; met: boolean }> {
    return this.http.put(
      `/api/deescalation/criteria/${encodeURIComponent(criterionPid)}/evidence`,
      { body },
    );
  }
  /** Replace a draft decision's stop criteria, in order (EX-6, NC-1 slice 11). */
  replaceCutoverCriteria(
    planPid: string,
    did: string,
    criteria: string[],
  ): Promise<CutoverDecision> {
    return this.http.put(
      `/api/plans/${planPid}/cutover-decisions/${did}/criteria`,
      { body: { criteria } },
    );
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
  /** What a start (a move into executing) would meet now, with this programme's firm demand; moves nothing (EX-8). */
  startCheck(planPid: string): Promise<StartCheck> {
    return this.http.get(`/api/plans/${planPid}/start-check`);
  }
  /** A plan's phase, its transitions and the time spent in each phase. */
  phaseHistory(planPid: string): Promise<PhaseHistory> {
    return this.http.get(`/api/plans/${planPid}/phase-history`);
  }
  /** Move a plan's phase: one step forward, or back with a reason; a programme moving into executing meets the start
   * check, and `start_override_reason` starts a refused one anyway. */
  setPhase(
    planPid: string,
    body: {
      phase: PhaseToken;
      reason?: string;
      start_override_reason?: string;
    },
  ): Promise<{ phase: PhaseToken; next_phase: PhaseToken | null }> {
    return this.http.put(`/api/plans/${planPid}/phase`, { body });
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
  /** Add a pool (a name of 2 to 120 characters) with its roles and skills; 409 when the name, or a role or skill,
   *  is already taken (a role or skill is in at most one pool). DP-10, NC-1 slice 11. */
  createSkillPool(body: {
    name: string;
    roles: string[];
    skills: string[];
  }): Promise<SkillPool> {
    return this.http.post("/api/skill-pools", { body });
  }
  /** Retire or restore a pool, or replace its roles or skills; a field left out is kept. */
  updateSkillPool(
    pid: string,
    body: { active?: boolean; roles?: string[]; skills?: string[] },
  ): Promise<SkillPool> {
    return this.http.put(`/api/skill-pools/${encodeURIComponent(pid)}`, {
      body,
    });
  }
  /** FB-1: link a skill pool to the workforce service's pool (its id), or remove the link with null. */
  setSkillPoolPeerLink(
    pid: string,
    peerPoolRef: string | null,
  ): Promise<SkillPool> {
    return this.http.put(
      `/api/skill-pools/${encodeURIComponent(pid)}/peer-link`,
      {
        body: { peer_pool_ref: peerPoolRef },
      },
    );
  }
  /** DP-10: the capacity commitments on programmes the caller can read, each demand one with its latest send to
   *  the workforce service (`peer_demand_send`, FB-3/FB-5). */
  capacityCommitments(): Promise<{ commitments: CapacityCommitment[] }> {
    return this.http.get("/api/capacity-commitments");
  }
  /** DP-10 / FB-6: set a capacity commitment (one per pool, programme, organization, kind and month). */
  setCapacityCommitment(body: CommitmentInput): Promise<CapacityCommitment> {
    return this.http.put("/api/capacity-commitments", { body });
  }
  /** DP-10 / FB-2: demand and supply by skill pool and month, `from` `YYYY-MM`, 1 to 24 months. */
  capacityHeatmap(from?: string, months?: number): Promise<CapacityHeatmap> {
    const params = new URLSearchParams();
    if (from) params.set("from", from);
    if (months) params.set("months", String(months));
    const query = params.toString();
    return this.http.get(`/api/capacity-heatmap${query ? `?${query}` : ""}`);
  }
  /** The director packs the caller may read, without their bodies. */
  directorPacks(): Promise<{ packs: DirectorPackMeta[] }> {
    return this.http.get("/api/director-packs");
  }
  /** One director pack with its frozen body. */
  directorPack(pid: string): Promise<DirectorPack> {
    return this.http.get(`/api/director-packs/${encodeURIComponent(pid)}`);
  }
  /**
   * Draft the first version of a period's pack for an audience (DP-1): editors and issuers only (403); 409 when the
   * period already has a pack for that audience; 422 for a bad period, audience or cut-off.
   */
  draftDirectorPack(body: DirectorPackDraftBody): Promise<DirectorPackMeta> {
    return this.http.post("/api/director-packs", { body });
  }
  /** The portfolio office has looked over a draft (DP-12); needed before a scheduled draft is issued. Once; drafts only. */
  reviewDirectorPack(pid: string): Promise<DirectorPackMeta> {
    return this.http.post(
      `/api/director-packs/${encodeURIComponent(pid)}/review`,
      { body: {} },
    );
  }
  /** Freeze a draft (DP-1): issuers only (403); 409 when already issued, or a scheduled draft not yet reviewed. */
  issueDirectorPack(pid: string): Promise<DirectorPackMeta> {
    return this.http.post(
      `/api/director-packs/${encodeURIComponent(pid)}/issue`,
      { body: {} },
    );
  }
  /** The monthly pack schedules (DP-12); editors and issuers only (403). */
  packSchedules(): Promise<{ schedules: PackSchedule[] }> {
    return this.http.get("/api/director-pack-schedules");
  }
  /** Draft an audience's pack on a day (1 to 28) of each month; one schedule per audience (409 for a second). */
  createPackSchedule(body: {
    audience: string;
    day_of_month: number;
  }): Promise<PackSchedule> {
    return this.http.post("/api/director-pack-schedules", { body });
  }
  /** Switch a schedule on or off, or move its day; a field left out is kept. */
  updatePackSchedule(
    pid: string,
    body: { enabled?: boolean; day_of_month?: number },
  ): Promise<PackSchedule> {
    return this.http.put(
      `/api/director-pack-schedules/${encodeURIComponent(pid)}`,
      { body },
    );
  }
  /** Run the schedules that are due now: it drafts, never issues. */
  sweepPackSchedules(): Promise<{
    drafted: string[];
    skipped: number;
    issued: number;
  }> {
    return this.http.post("/api/director-pack-schedules/sweep", { body: {} });
  }
  /** The audiences the caller is told about when a pack is issued. */
  packSubscriptions(): Promise<{ subscribed_to: string[] }> {
    return this.http.get("/api/director-pack-subscriptions");
  }
  /** Be told, or stop being told, when a pack for an audience is issued (403 for an audience the caller cannot read). */
  setPackSubscription(body: {
    audience: string;
    subscribed: boolean;
  }): Promise<{ audience: string; subscribed: boolean }> {
    return this.http.put("/api/director-pack-subscriptions", { body });
  }

  /** Draft the next version of an issued pack with a change note of at least ten characters (DP-1); the latest only. */
  reissueDirectorPack(
    pid: string,
    changeNote: string,
  ): Promise<DirectorPackMeta> {
    return this.http.post(
      `/api/director-packs/${encodeURIComponent(pid)}/reissue`,
      {
        body: { change_note: changeNote },
      },
    );
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
  /** What a plan's issue form offers: every type with its workflow, and the caller's standing (OC-1, OC-3). */
  issueOptions(pid: string): Promise<IssueOptions> {
    return this.http.get(`/api/plans/${pid}/issues/options`);
  }
  /** The issue types, retired ones too if asked (OC-1). */
  issueTypes(includeRetired = false): Promise<IssueType[]> {
    return this.http.get(
      `/api/issue-types${includeRetired ? "?include_retired=true" : ""}`,
    );
  }
  /** Add an issue type (an administrator). */
  addIssueType(body: NewIssueType): Promise<IssueType> {
    return this.http.post(`/api/issue-types`, { body });
  }
  /** Rename, reorder, retire or restore an issue type (an administrator). */
  changeIssueType(key: string, body: IssueTypeChange): Promise<IssueType> {
    return this.http.put(`/api/issue-types/${key}`, { body });
  }
  /** The roles the organization recognizes, retired ones too if asked (OC-1). */
  organizationRoles(includeRetired = false): Promise<OrganizationRole[]> {
    return this.http.get(
      `/api/organization-roles${includeRetired ? "?include_retired=true" : ""}`,
    );
  }
  /** Add a role (an administrator). */
  addOrganizationRole(body: NewOrganizationRole): Promise<OrganizationRole> {
    return this.http.post(`/api/organization-roles`, { body });
  }
  /** Rename, reorder, retire or restore a role (an administrator). */
  changeOrganizationRole(
    key: string,
    body: OrganizationRoleChange,
  ): Promise<OrganizationRole> {
    return this.http.put(`/api/organization-roles/${key}`, { body });
  }
  /** Every registered workflow. */
  workflows(): Promise<WorkflowSummary[]> {
    return this.http.get(`/api/workflows`);
  }
  /** Register an issue workflow (an administrator). */
  addIssueWorkflow(body: NewIssueWorkflow): Promise<{ pid: string }> {
    return this.http.post(`/api/workflows`, { body });
  }
  /** One workflow with its definition. */
  workflowDetail(pid: string): Promise<WorkflowDetail> {
    return this.http.get(`/api/workflows/${pid}`);
  }
  /** Change an issue workflow in place (an administrator). */
  changeIssueWorkflow(
    pid: string,
    body: IssueWorkflowChange,
  ): Promise<{ pid: string }> {
    return this.http.put(`/api/workflows/${pid}`, { body });
  }
  /** Who holds which role on a plan (OC-3). */
  planRoleMembers(pid: string): Promise<PlanRoleMembers> {
    return this.http.get(`/api/plans/${pid}/role-members`);
  }
  /** Give someone a role on a plan (an administrator). The first member turns enforcement on. */
  addPlanRoleMember(
    pid: string,
    body: { person_ref: string; role_key: string },
  ): Promise<PlanRoleMember> {
    return this.http.post(`/api/plans/${pid}/role-members`, { body });
  }
  /** Take a role away (an administrator). */
  removePlanRoleMember(pid: string, member: string): Promise<void> {
    return this.http.delete(`/api/plans/${pid}/role-members/${member}`);
  }
  /** The issue types a plan limits itself to (OC-5). */
  planIssueTypes(pid: string): Promise<PlanIssueTypes> {
    return this.http.get(`/api/plans/${pid}/issue-types`);
  }
  /** Choose the types a plan offers (an administrator). Empty offers every type. */
  setPlanIssueTypes(pid: string, types: string[]): Promise<PlanIssueTypes> {
    return this.http.put(`/api/plans/${pid}/issue-types`, { body: { types } });
  }
  /** Withdraw a workflow. An issue workflow needs an administrator. */
  withdrawWorkflow(pid: string): Promise<void> {
    return this.http.delete(`/api/workflows/${pid}`);
  }
  /** One plan's issues (IR-2), most serious first. */
  planIssues(pid: string): Promise<PlanIssueList> {
    return this.http.get(`/api/plans/${pid}/issues`);
  }
  /** Raise an issue on a plan (IR-2). */
  raiseIssue(pid: string, body: IssueBody): Promise<Issue> {
    return this.http.post(`/api/plans/${pid}/issues`, { body });
  }
  /** Replace an issue's fields (IR-2). */
  updateIssue(pid: string, issuePid: string, body: IssueBody): Promise<Issue> {
    return this.http.put(`/api/plans/${pid}/issues/${issuePid}`, { body });
  }
  /** Raise an issue from a risk that has happened (IR-3); 409 when one is already open for it. */
  raiseIssueFromRisk(pid: string, riskPid: string): Promise<Issue> {
    return this.http.post(`/api/plans/${pid}/issues/from-risk/${riskPid}`, {
      body: {},
    });
  }
  /** Raise a blocker issue from a blocked task (IR-3); 422 unless blocked, 409 when one is already open for it. */
  raiseIssueFromTask(pid: string, taskPid: string): Promise<Issue> {
    return this.http.post(`/api/plans/${pid}/issues/from-task/${taskPid}`, {
      body: {},
    });
  }
  /** One issue, as stored now (IR-2). */
  issue(pid: string, issuePid: string): Promise<Issue> {
    return this.http.get(`/api/plans/${pid}/issues/${issuePid}`);
  }
  /** Remove an issue (IR-2; kept for the audit, gone from every list). */
  removeIssue(pid: string, issuePid: string): Promise<void> {
    return this.http.delete(`/api/plans/${pid}/issues/${issuePid}`);
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
  ): Promise<{
    pid: string;
    booking: string;
    over_capacity: boolean | null;
    days_capacity_unknown: number;
  }> {
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
  /** Un-commit a committed scenario: it becomes a draft again (T-28a). */
  rollbackScenario(pid: string): Promise<Scenario> {
    return this.http.post(`/api/scenarios/${pid}/rollback`, { body: {} });
  }
  /** Generate and save a draft scenario, chosen by Smart Score per unit cost within an optional cap (T-28h). */
  generateScenario(body: GenerateScenarioBody): Promise<GeneratedScenario> {
    return this.http.post("/api/scenarios/generate", { body });
  }
  /** The efficient frontier: the generator re-run over a sweep of caps; nothing is saved (NC-7). The caps are given,
   * or `steps` are spread evenly (left out for the service's 10). */
  scenarioFrontier(params: {
    currency: string;
    caps?: number[];
    steps?: number;
    mustInclude?: string[];
  }): Promise<Frontier> {
    const q = new URLSearchParams({ currency: params.currency });
    if (params.caps && params.caps.length > 0)
      q.set("caps", params.caps.join(","));
    else if (params.steps !== undefined) q.set("steps", String(params.steps));
    if (params.mustInclude && params.mustInclude.length > 0)
      q.set("must_include", params.mustInclude.join(","));
    return this.http.get(`/api/scenarios/frontier?${q}`);
  }

  // ---- WSJF and RICE (NC-6) ----
  /** A plan's estimated WSJF and RICE inputs. */
  scoringInputs(pid: string): Promise<ScoringInputsView> {
    return this.http.get(`/api/plans/${pid}/scoring-inputs`);
  }
  /** Replace a plan's estimated inputs (a `null` clears one). */
  setScoringInputs(
    pid: string,
    body: ScoringInputs,
  ): Promise<ScoringInputsView> {
    return this.http.put(`/api/plans/${pid}/scoring-inputs`, { body });
  }
  /** A plan's WSJF or RICE score, with every component and the inputs it waits for. */
  modelScore(pid: string, model: EstimateModel): Promise<PlanModelScore> {
    return this.http.get(`/api/plans/${pid}/score?model=${model}`);
  }
  /** The plans ranked by WSJF or RICE, highest first, unscored last. */
  rankByModel(model: EstimateModel): Promise<ModelRanking> {
    return this.http.get(`/api/prioritization?model=${model}`);
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
  raiseEscalation(pid: string, body: RaiseEscalationBody): Promise<Escalation> {
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
  /** The acknowledgement at the escalation's current level (CF-4); 404 when there is none yet. */
  escalationAcknowledgement(
    pid: string,
    epid: string,
  ): Promise<EscalationAcknowledgement> {
    return this.http.get(
      `/api/plans/${pid}/escalations/${epid}/acknowledgement`,
    );
  }
  /** Acknowledge it (CF-4); needs the acknowledgement right; repeating it answers the one already there. */
  acknowledgeEscalation(
    pid: string,
    epid: string,
    body: AcknowledgeEscalationBody,
  ): Promise<EscalationAcknowledgement> {
    return this.http.put(
      `/api/plans/${pid}/escalations/${epid}/acknowledgement`,
      { body },
    );
  }
  /** The resolution of a resolved escalation (CF-4); 404 until it is resolved. */
  escalationResolution(
    pid: string,
    epid: string,
  ): Promise<EscalationResolution> {
    return this.http.get(`/api/plans/${pid}/escalations/${epid}/resolution`);
  }
  /** Resolve it (CF-4); needs the resolution right; a resolution is never overwritten (409 for a different one). */
  resolveEscalation(
    pid: string,
    epid: string,
    body: ResolveEscalationBody,
  ): Promise<EscalationResolution> {
    return this.http.put(`/api/plans/${pid}/escalations/${epid}/resolution`, {
      body,
    });
  }
  /** Every readable programme's register completeness, and whether "no register, no money" is on (EX-1). */
  registerCompleteness(): Promise<RegisterCompleteness> {
    return this.http.get("/api/register-completeness");
  }

  // ---- meetings, lessons and references (NC-14, BR-3, NC-8, BR-4; NC-1 slice 12 part e) ----
  /** The plan's meetings, newest first, without their items. */
  meetings(pid: string): Promise<MeetingSummary[]> {
    return this.http.get(`/api/plans/${pid}/meetings`);
  }
  /** Plan a meeting with its agenda. */
  planMeeting(
    pid: string,
    body: PlanMeetingBody,
  ): Promise<{ pid: string; status: string }> {
    return this.http.post(`/api/plans/${pid}/meetings`, { body });
  }
  /** One meeting with its agenda, minutes and outcomes, each link named as the record is now. */
  meeting(pid: string, mid: string): Promise<Meeting> {
    return this.http.get(`/api/plans/${pid}/meetings/${mid}`);
  }
  /** Record a planned meeting as held, with its minutes and outcomes, once; 409 once held or cancelled. */
  holdMeeting(
    pid: string,
    mid: string,
    body: HeldMeetingBody,
  ): Promise<{ pid: string; status: string }> {
    return this.http.post(`/api/plans/${pid}/meetings/${mid}/held`, { body });
  }
  /** Cancel a planned meeting; 409 once held or cancelled. */
  cancelMeeting(
    pid: string,
    mid: string,
  ): Promise<{ pid: string; status: string }> {
    return this.http.post(`/api/plans/${pid}/meetings/${mid}/cancel`, {
      body: {},
    });
  }
  /** Judge proposed tasks against a held meeting's minutes and the plan's people; nothing is written (BR-3). */
  previewMeetingTasks(
    pid: string,
    mid: string,
    body: TaskProposalsBody,
  ): Promise<TaskProposalPreview> {
    return this.http.post(
      `/api/plans/${pid}/meetings/${mid}/task-proposals/preview`,
      { body },
    );
  }
  /** Create the proposed tasks, all or none, once per outcome; `key` is the idempotency key, kept across a retry. */
  commitMeetingTasks(
    pid: string,
    mid: string,
    body: TaskProposalsBody,
    key: string,
  ): Promise<TaskProposalCommit> {
    return this.http.post(
      `/api/plans/${pid}/meetings/${mid}/task-proposals/commit`,
      { body, idempotencyKey: key },
    );
  }
  /** The plan's lessons, oldest first; withdrawn ones are not listed (NC-8). */
  lessons(pid: string): Promise<Lesson[]> {
    return this.http.get(`/api/plans/${pid}/lessons`);
  }
  /** Record a lesson (append-only). */
  recordLesson(pid: string, body: LessonBody): Promise<Lesson> {
    return this.http.post(`/api/plans/${pid}/lessons`, { body });
  }
  /** Withdraw a lesson: it is no longer shown, and stays in the audit trail. */
  withdrawLesson(pid: string, lpid: string): Promise<{ withdrawn: string }> {
    return this.http.post(`/api/plans/${pid}/lessons/${lpid}/withdraw`, {
      body: {},
    });
  }
  /** The meetings and lessons that point at one record of the plan (BR-4). */
  references(
    pid: string,
    kind: string,
    recordPid: string,
  ): Promise<RecordReferences> {
    const q = new URLSearchParams({ kind, pid: recordPid });
    return this.http.get(`/api/plans/${pid}/references?${q}`);
  }
  /** Lessons from plans whose title is like the proposal's, best match first, with the match's evidence (NC-8). */
  proposalLessons(proposalPid: string): Promise<ProposalLessonMatch[]> {
    return this.http.get(`/api/proposals/${proposalPid}/lessons`);
  }
  // ---- programme money, value and benefits, read live (NC-1 slice 12 part f, DP-7, DP-8, DP-9) ----
  /** The fever chart of the programmes the reader can read, now: the shape of a pack's `buffer_fever` (DP-7). */
  bufferFever(): Promise<Record<string, unknown>> {
    return this.http.get("/api/buffer-fever");
  }
  /** Earned value over the threshold, now: the shape of a pack's `earned_value` (DP-8); no `ac` is the service's default. */
  portfolioEarnedValue(ac?: AcSource): Promise<Record<string, unknown>> {
    return this.http.get(
      `/api/portfolio-earned-value${ac ? `?${new URLSearchParams({ ac })}` : ""}`,
    );
  }
  /** Validated benefits, now: the shape of a pack's `benefits` (DP-9). */
  portfolioBenefits(): Promise<Record<string, unknown>> {
    return this.http.get("/api/portfolio-benefits");
  }
  /** Issues and change requests over the plans the reader can read, counted over `windowDays` (IR-6). */
  issuesAndChanges(windowDays?: number): Promise<Record<string, unknown>> {
    const q =
      windowDays === undefined
        ? ""
        : `?${new URLSearchParams({ window_days: String(windowDays) })}`;
    return this.http.get(`/api/director-pack/issues-and-changes${q}`);
  }
  /** The estimate at completion rolled up from `plan` over the plans beneath it, one row per currency. */
  forecastRollup(plan: string, depth?: number): Promise<ForecastRollup> {
    const q = new URLSearchParams({ plan });
    if (depth !== undefined) q.set("depth", String(depth));
    return this.http.get(`/api/financials/forecast?${q}`);
  }
  /** One plan's estimate at completion. */
  planForecast(pid: string): Promise<PlanForecast> {
    return this.http.get(`/api/plans/${pid}/financials/forecast`);
  }
  /** Every budget baseline version of a plan, newest first. */
  budgetBaselines(pid: string): Promise<BudgetBaseline[]> {
    return this.http.get(`/api/plans/${pid}/budget-baselines`);
  }
  /** Approve a new budget baseline version (frozen once approved; a later one needs a reason). */
  approveBudgetBaseline(
    pid: string,
    body: BudgetBaselineBody,
  ): Promise<{ pid: string; version: number }> {
    return this.http.post(`/api/plans/${pid}/budget-baselines`, { body });
  }
  /** Release a stage-gated budget line once its gate has passed (it cannot be held again). */
  releaseBudgetLine(pid: string, linePid: string): Promise<unknown> {
    return this.http.post(`/api/plans/${pid}/budget-lines/${linePid}/release`, {
      body: {},
    });
  }
  /** Record a promised business-case target; its approval date has no update path. */
  recordBusinessCase(
    pid: string,
    body: BusinessCaseBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/business-case`, { body });
  }
  /** The plan's realized gains: return on investment, time to value, adoption and its business case. */
  valueRealization(pid: string): Promise<ValueRealization> {
    return this.http.get(`/api/plans/${pid}/value-realization`);
  }
  /** Record an observed value. */
  recordValuePoint(
    pid: string,
    body: ValuePointBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/value-points`, { body });
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

  // ---- bulk import and export jobs (T-8; NC-1 slice 12 part k) ----
  /** The caller's bulk jobs (an administrator's: everyone's), newest first, at most 200; by kind and status. */
  bulkJobs(
    filter: { kind?: BulkJobKind; status?: BulkJobStatus } = {},
  ): Promise<BulkJob[]> {
    const q = new URLSearchParams();
    if (filter.kind) q.set("kind", filter.kind);
    if (filter.status) q.set("status", filter.status);
    const s = q.toString();
    return this.http.get(`/api/plans/bulk-jobs${s ? `?${s}` : ""}`);
  }
  /** One job read again, with its kept file's expiry. */
  bulkJob(kind: BulkJobKind, id: string): Promise<BulkJob> {
    return this.http.get(
      `/api/plans/${kind === "import" ? "import" : "export"}/${encodeURIComponent(id)}`,
    );
  }
  /** Stop a queued job, or ask a running one to stop after its current row. */
  cancelBulkJob(id: string): Promise<BulkJobCancel> {
    return this.http.post(
      `/api/plans/bulk-jobs/${encodeURIComponent(id)}/cancel`,
      { body: {} },
    );
  }
  /** Queue an export of the plans the caller may read. */
  startExport(body: StartExportBody): Promise<BulkJobStarted> {
    return this.http.post("/api/plans/export", { body });
  }
  /**
   * Queue an import of a file: JSON lines or CSV, sent as the raw body. The client's Idempotency-Key for the same
   * file means a retry is answered with the job it already started (`replayed`).
   */
  startImport(
    file: Blob | string,
    format: "jsonl" | "csv",
    onKeyless: "review" | "create",
  ): Promise<BulkJobStarted> {
    return this.http.post(
      `/api/plans/import?format=${format}&on_keyless=${onKeyless}`,
      { rawBody: file, headers: { "content-type": "text/plain" } },
    );
  }

  /** Rows held for a decision (pending ones, at most 500), all or one job's. */
  reviewQueue(job?: string): Promise<ReviewQueueItem[]> {
    return this.http.get(
      `/api/plans/review-queue${job ? `?job=${encodeURIComponent(job)}` : ""}`,
    );
  }
  /** A held row is new after all: create the plan. */
  confirmQueueItem(pid: string): Promise<ReviewQueueItem> {
    return this.http.post(
      `/api/plans/review-queue/${encodeURIComponent(pid)}/confirm`,
      { body: {} },
    );
  }
  /** A held row is a duplicate: discard it (nothing was created from it). */
  rejectQueueItem(pid: string): Promise<ReviewQueueItem> {
    return this.http.post(
      `/api/plans/review-queue/${encodeURIComponent(pid)}/reject`,
      { body: {} },
    );
  }

  // ---- blueprint sync (NC-12; NC-1 slice 12 part k) ----
  /** The plans made from a template that the caller can read, with how many changes each has waiting. */
  templatePlans(pid: string): Promise<TemplatePlan[]> {
    return this.http.get(
      `/api/plan-templates/${encodeURIComponent(pid)}/plans`,
    );
  }
  /** Re-capture a plan tree as the template's next version; plans made from it are not touched. */
  reviseTemplate(pid: string, fromPlan: string): Promise<TemplateRevised> {
    return this.http.post(
      `/api/plan-templates/${encodeURIComponent(pid)}/revise`,
      {
        body: { from_plan: fromPlan },
      },
    );
  }
  /** The template's changes since this plan was last brought up to date; 404 for a plan not made from one. */
  templateDiff(planPid: string): Promise<TemplateDiff> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(planPid)}/template-diff`,
    );
  }
  /** Apply the named changes; one to a task the plan edited needs naming in `overwrite_local` too. */
  templateSync(
    planPid: string,
    body: TemplateSyncBody,
  ): Promise<TemplateSynced> {
    return this.http.post(
      `/api/plans/${encodeURIComponent(planPid)}/template-sync`,
      { body },
    );
  }

  /** Import up to ten MS Project files at once; a dry run (the default) writes nothing. */
  importMspdiBatch(
    body: MspdiBatchBody,
    dryRun: boolean,
  ): Promise<MspdiBatchAnswer> {
    return this.http.post(`/api/import/mspdi-batch?dry_run=${dryRun}`, {
      body,
    });
  }

  // ---- task components (PL-11; NC-1 slice 12 part n) ----
  components(planPid: string): Promise<Component[]> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(planPid)}/components`,
    );
  }
  componentSummary(planPid: string): Promise<ComponentSummary> {
    return this.http.get(
      `/api/plans/${encodeURIComponent(planPid)}/components/summary`,
    );
  }
  createComponent(planPid: string, body: ComponentBody): Promise<Component> {
    return this.http.post(
      `/api/plans/${encodeURIComponent(planPid)}/components`,
      { body },
    );
  }
  updateComponent(
    planPid: string,
    pid: string,
    body: ComponentBody,
  ): Promise<Component> {
    return this.http.put(
      `/api/plans/${encodeURIComponent(planPid)}/components/${encodeURIComponent(pid)}`,
      { body },
    );
  }
  deleteComponent(planPid: string, pid: string): Promise<void> {
    return this.http.delete(
      `/api/plans/${encodeURIComponent(planPid)}/components/${encodeURIComponent(pid)}`,
    );
  }

  /**
   * A plan's goals with their positions, and the list's `ETag` (FR-12, CF-2). Every goal write answers the same, so
   * the tag in hand is always the last one the service gave.
   */
  async goals(planPid: string): Promise<GoalList> {
    let etag: string | null = null;
    const goals = await this.http.get<IndexedGoal[]>(
      `/api/plans/${encodeURIComponent(planPid)}/goals`,
      { onResponse: (r) => (etag = r.headers.get("etag")) },
    );
    return { goals: goals ?? [], etag };
  }
  /** Append a goal; answers with the whole list and its new tag. */
  async addGoal(planPid: string, goal: Goal): Promise<GoalList> {
    let etag: string | null = null;
    const goals = await this.http.post<IndexedGoal[]>(
      `/api/plans/${encodeURIComponent(planPid)}/goals`,
      { body: goal, onResponse: (r) => (etag = r.headers.get("etag")) },
    );
    return { goals: goals ?? [], etag };
  }
  /** Replace the goal at `index`, refused with 412 when the list changed since `etag` was read. */
  async replaceGoal(
    planPid: string,
    index: number,
    goal: Goal,
    etag: string | null,
  ): Promise<GoalList> {
    let next: string | null = null;
    const goals = await this.http.put<IndexedGoal[]>(
      `/api/plans/${encodeURIComponent(planPid)}/goals/${index}`,
      {
        body: goal,
        headers: etag ? { "If-Match": etag } : {},
        onResponse: (r) => (next = r.headers.get("etag")),
      },
    );
    return { goals: goals ?? [], etag: next };
  }
  /** Remove the goal at `index` (the ones after it move up), refused with 412 when the list changed since `etag`. */
  async removeGoal(
    planPid: string,
    index: number,
    etag: string | null,
  ): Promise<GoalList> {
    let next: string | null = null;
    const goals = await this.http.delete<IndexedGoal[]>(
      `/api/plans/${encodeURIComponent(planPid)}/goals/${index}`,
      {
        headers: etag ? { "If-Match": etag } : {},
        onResponse: (r) => (next = r.headers.get("etag")),
      },
    );
    return { goals: goals ?? [], etag: next };
  }

  /** Each kind's checklist: its custom template, or the built-in one. */
  async onboardingTemplates(): Promise<OnboardingTemplate[]> {
    const r = await this.http.get<{ templates: OnboardingTemplate[] }>(
      "/api/onboarding/templates",
    );
    return r?.templates ?? [];
  }
  /** Replace a kind's checklist (someone who may manage people; 403 otherwise). */
  setOnboardingTemplate(
    kind: OnboardingKind,
    body: OnboardingTemplateBody,
  ): Promise<OnboardingTemplate> {
    return this.http.put(
      `/api/onboarding/templates/${encodeURIComponent(kind)}`,
      { body },
    );
  }
  /** Go back to the built-in checklist for a kind. */
  resetOnboardingTemplate(kind: OnboardingKind): Promise<void> {
    return this.http.delete(
      `/api/onboarding/templates/${encodeURIComponent(kind)}`,
    );
  }

  /** Live custom field definitions, oldest first (RM-1). */
  customFields(): Promise<CustomFieldDef[]> {
    return this.http.get("/api/custom-fields");
  }
  /** Declare a field; 409 when its key is taken for the subject type. */
  declareCustomField(body: DefBody): Promise<CustomFieldDef> {
    return this.http.post("/api/custom-fields", { body });
  }
  /** Withdraw a definition; values already stored stay readable, marked withdrawn. */
  withdrawCustomField(pid: string): Promise<void> {
    return this.http.delete(`/api/custom-fields/${encodeURIComponent(pid)}`);
  }
  /** One plan's or task's values (withdrawn definitions marked) and the required fields still unset. */
  customFieldValues(
    appliesTo: "plan" | "task",
    subjectPid: string,
  ): Promise<CustomFieldValues> {
    const q = new URLSearchParams({
      applies_to: appliesTo,
      subject_pid: subjectPid,
    });
    return this.http.get(`/api/custom-fields/values?${q}`);
  }
  /** Set (or, with `null`, clear) values; atomic, so one invalid value refuses the whole call. */
  setCustomFieldValues(
    body: CustomFieldValuesBody,
  ): Promise<CustomFieldValues> {
    return this.http.put("/api/custom-fields/values", { body });
  }
  /** The plans or tasks whose `key` equals `value` (a multi-choice field: contains it), those the caller can read. */
  searchCustomFields(
    appliesTo: string,
    key: string,
    value: string,
  ): Promise<CustomFieldSearch> {
    const q = new URLSearchParams({ applies_to: appliesTo, key, value });
    return this.http.get(`/api/custom-fields/search?${q}`);
  }

  /** Everyone's primary role (a resource manager or an administrator), else just the caller's own. */
  resourceRoles(): Promise<ResourceRole[]> {
    return this.http.get("/api/resource-roles");
  }
  /** Set a person's primary role, or clear it with `null`; 403 without the right. */
  setResourceRole(body: ResourceRole): Promise<ResourceRole> {
    return this.http.put("/api/resource-roles", { body });
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
  /** FB-4: a programme's people impact from the workforce service, passed through (`source: "wpm"`). */
  planPeopleImpact(pid: string): Promise<PeopleImpact> {
    return this.http.get(`/api/plans/${pid}/people-impact`);
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
  /**
   * The caller's own inbox (PL-5, NC-1 slice 4), with their daily digests applied in `tz` (an IANA zone name).
   * `unread` keeps only lines with something unread. A 422 is a zone the service refused, or a sign-in identity
   * that cannot receive person-addressed notifications.
   */
  myInbox(tz: string, unread = false): Promise<MyInbox> {
    const q = new URLSearchParams({ tz });
    if (unread) q.set("unread", "true");
    return this.http.get(`/api/me/inbox?${q.toString()}`);
  }
  /** The caller's notification choices, the kinds, channels and digests there are, and the default (PL-5). */
  myNotificationPrefs(): Promise<NotificationPrefs> {
    return this.http.get(`/api/me/notification-prefs`);
  }
  /** Set one notification choice (PL-5); the service keeps one row per kind and channel. */
  setNotificationPref(body: NotificationPrefBody): Promise<NotificationPref> {
    return this.http.put(`/api/me/notification-prefs`, { body });
  }
  /** Back to the default for one kind and channel (PL-5). The POST twin: the default policy keeps DELETE for admins. */
  resetNotificationPref(body: NotificationPrefResetBody): Promise<void> {
    return this.http.post(`/api/me/notification-prefs/reset`, { body });
  }
  /** The caller's own notification destinations (LT-6), each destination masked. */
  myNotificationSinks(): Promise<NotificationSink[]> {
    return this.http.get(`/api/me/notification-sinks`);
  }
  /** Register a destination (LT-6, RM-11); 422 for one the service will not send to, or past ten. */
  addNotificationSink(body: NotificationSinkBody): Promise<NotificationSink> {
    return this.http.post(`/api/me/notification-sinks`, { body });
  }
  /** Remove one of the caller's destinations (LT-6); the POST twin of DELETE. Another person's is a 404. */
  removeNotificationSink(sinkPid: string): Promise<void> {
    return this.http.post(`/api/me/notification-sinks/${sinkPid}/delete`);
  }
  /** The recent deliveries to one of the caller's destinations (LT-6), newest first, at most 50. */
  sinkDeliveries(sinkPid: string): Promise<SinkDelivery[]> {
    return this.http.get(`/api/me/notification-sinks/${sinkPid}/deliveries`);
  }
  /**
   * One person's week by day and plan, and its approval state (LT-11, NC-1 slice 6). `week` is an ISO week such as
   * `2026-W46`; `person` (a `person:` or `worker:` reference) defaults to the caller, and reading someone else's
   * needs the approver right (403 otherwise). A 422 is a week or person the service does not read, or a sign-in
   * identity with no person reference.
   */
  timesheetWeek(week: string, person?: string): Promise<TimesheetWeek> {
    const q = new URLSearchParams({ week });
    if (person) q.set("person", person);
    return this.http.get(`/api/timesheets?${q.toString()}`);
  }
  /** Submit the caller's own week (LT-11): 422 when it has no entries, 409 when it is already submitted or approved. */
  submitTimesheet(body: TimesheetSubmitBody): Promise<TimesheetWeekState> {
    return this.http.post(`/api/timesheets/submit`, { body });
  }
  /** An approver approves or rejects someone else's submitted week (LT-11); approved is final. */
  decideTimesheet(body: TimesheetDecideBody): Promise<TimesheetWeekState> {
    return this.http.post(`/api/timesheets/decide`, { body });
  }
  /** Every person's time entries in a plan, newest first (LT-11); at most 5000. */
  timeEntries(pid: string): Promise<TimeEntry[]> {
    return this.http.get(`/api/plans/${pid}/time-entries`);
  }
  /** Record effort in a plan (LT-11): 409 when the person's week is approved and locked; 422 out of the rules. */
  recordTime(pid: string, body: TimeEntryBody): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/time-entries`, { body });
  }
  /** Time grouped by person, plan, category, billable or month (RM-10), over the plans the caller can read. */
  timeReport(q: TimeReportQuery): Promise<TimeReport> {
    return this.http.get(`/api/time-reports${timeReportSearch(q)}`);
  }
  /** Where the browser downloads the same time report as the service's CSV, through the proxy. */
  timeReportCsvUrl(q: TimeReportQuery): string {
    return `${API_BASE_URL}/api/time-reports${timeReportSearch(q, true)}`;
  }

  // ---- a person's own work (PL-4, LT-7, RM-3, PJ-5; NC-1 slice 12 part h) ----
  /** One local day of the caller's planned slots (PL-4), in `tz` (an IANA zone name; 422 for one refused). */
  myDayPlan(day: string, tz: string): Promise<DayPlan> {
    const q = new URLSearchParams({ day, tz });
    return this.http.get(`/api/me/plan?${q.toString()}`);
  }
  /** Block time for a task (PL-4): 422 out of the rules, 404 for a task the caller cannot read. */
  planSlot(body: PlannedSlotBody): Promise<PlannedSlot> {
    return this.http.post(`/api/me/planned-slots`, { body });
  }
  /** Remove one of the caller's slots (PL-4); the POST twin of DELETE. Another person's is a 404. */
  removeSlot(slotPid: string): Promise<void> {
    return this.http.post(`/api/me/planned-slots/${slotPid}/delete`);
  }
  /** Make a calendar feed token (LT-7), shown once; it replaces any earlier one. */
  issueFeedToken(): Promise<FeedTokenIssued> {
    return this.http.post(`/api/me/feed-token`);
  }
  /** Stop every feed token of the caller (LT-7); the POST twin of DELETE. */
  revokeFeedToken(): Promise<void> {
    return this.http.post(`/api/me/feed-token/revoke`);
  }
  /** What the caller follows (RM-3), newest first. */
  myWatching(): Promise<MyWatching> {
    return this.http.get(`/api/me/watching`);
  }
  /** The work of every plan the caller can read, spread over periods (PJ-5). A left-out period is weeks. */
  portfolioWorkProfile(
    params: { period?: Grain; from?: string; to?: string } = {},
  ): Promise<PortfolioWorkProfile> {
    const q = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) if (v) q.set(k, v);
    const qs = q.toString();
    return this.http.get(`/api/work-profile${qs ? `?${qs}` : ""}`);
  }

  // ---- scheduling, the rest (PJ-7, PJ-13, LT-10, CC-2, CC-3, RM-5, SC-4; NC-1 slice 12 part i) ----
  /** Each hammock of the plan with the span derived from its links (PJ-7). */
  hammocks(pid: string): Promise<HammockList> {
    return this.http.get(`/api/plans/${pid}/hammocks`);
  }
  /** Make a task a hammock, or a hammock a task again (which removes its links); 422 for a task that carries work. */
  setHammock(
    pid: string,
    taskPid: string,
    enabled: boolean,
  ): Promise<HammockSet> {
    return this.http.post(`/api/plans/${pid}/tasks/${taskPid}/hammock`, {
      body: { enabled },
    });
  }
  /** Link a hammock to a task (or a hammock) of the same plan; 422 for any of four refusals, told apart only in English. */
  linkHammock(
    pid: string,
    hammockPid: string,
    memberPid: string,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/hammocks/${hammockPid}/links`, {
      body: { member_pid: memberPid },
    });
  }
  /** Remove a hammock's link to `memberPid` (the task's pid, not the link's); 404 when there is none. */
  unlinkHammock(
    pid: string,
    hammockPid: string,
    memberPid: string,
  ): Promise<void> {
    return this.http.delete(
      `/api/plans/${pid}/hammocks/${hammockPid}/links/${memberPid}`,
    );
  }
  /** The links touching one task or plan (PJ-13); one whose other end the caller cannot read is opaque. */
  dependencies(item: string): Promise<DependencyRow[]> {
    return this.http.get(`/api/dependencies?item=${encodeURIComponent(item)}`);
  }
  /** Link two tasks, of any plans the caller can read both of; 404 when either is gone or unreadable. */
  addDependency(body: DependencyBody): Promise<{ pid: string }> {
    return this.http.post(`/api/dependencies`, { body });
  }
  /** Remove a link; 404 when it is gone or an end is unreadable. */
  removeDependency(depPid: string): Promise<void> {
    return this.http.delete(`/api/dependencies/${depPid}`);
  }
  /** A task's sub-tasks rolled up (RM-5), derived on read. */
  taskRollup(pid: string, taskPid: string): Promise<TaskRollup> {
    return this.http.get(`/api/plans/${pid}/tasks/${taskPid}/rollup`);
  }
  /** Flow across a plan and the plans it contains (TBA-9); a left-out depth is 32. */
  planFlowRollup(pid: string, depth?: number): Promise<PlanFlowRollup> {
    return this.http.get(`/api/plans/${pid}/rollup${readingSearch({ depth })}`);
  }
  /** The plan's recorded effort per plan, assignee and task, as people asserted it. */
  planEffort(pid: string): Promise<PlanEffort> {
    return this.http.get(`/api/plans/${pid}/effort`);
  }
  /** How long the plan's open work may take and how much may land (TBA-11); left-out values are the defaults. */
  deliveryForecast(
    pid: string,
    q: DeliveryForecastQuery = {},
  ): Promise<DeliveryForecast> {
    return this.http.get(
      `/api/plans/${pid}/forecast${readingSearch({ items: q.items, periods: q.periods, period_days: q.period_days, history_periods: q.history_periods })}`,
    );
  }
  /** Schedule and cost performance and the stakeholder score (SC-4). */
  planPerformance(pid: string): Promise<PlanPerformance> {
    return this.http.get(`/api/plans/${pid}/performance`);
  }
  /** The mix of finished work by kind; 90 days and no roll-up are the defaults, not sent. */
  flowDistribution(
    pid: string,
    q: FlowDistributionQuery = {},
  ): Promise<FlowDistribution> {
    return this.http.get(
      `/api/plans/${pid}/flow-distribution${readingSearch({ window_days: q.window_days, rollup: q.rollup, depth: q.depth })}`,
    );
  }
  /** The plan's task workflow: its states and the moves allowed. */
  planWorkflow(pid: string): Promise<PlanWorkflow> {
    return this.http.get(`/api/plans/${pid}/workflow`);
  }
  /** Each task's aggressive and safe estimates (CC-2), up to `limit` tasks (1 to 1000). */
  ccpmEstimates(pid: string, limit = 1000): Promise<CcpmEstimates> {
    return this.http.get(`/api/plans/${pid}/ccpm-estimates?limit=${limit}`);
  }
  /** Record a task's two estimates, or clear them with both null; 422 carries every problem in English. */
  setCcpmEstimate(
    pid: string,
    taskPid: string,
    body: CcpmEstimateBody,
  ): Promise<unknown> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}/ccpm-estimate`, {
      body,
    });
  }
  /** The latest frozen buffer plan (CC-3) and when each was taken. */
  bufferPlan(pid: string): Promise<BufferPlan> {
    return this.http.get(`/api/plans/${encodeURIComponent(pid)}/buffer-plan`);
  }
  /** Set a working day and week (an insert; 409 when the scope has one already). */
  setWorkingTime(body: WorkingTimeBody): Promise<{ pid: string }> {
    return this.http.post(`/api/working-time`, { body });
  }
  /** Record a person's leave or other time away from project work. */
  recordNonWorking(body: NonWorkingBody): Promise<{ pid: string }> {
    return this.http.post(`/api/non-working`, { body });
  }

  // ---- the board, sprints, resources and the OKR and KPI reads (RM-9, PL-3, SC-7, KO-1, KO-2; NC-1 slice 12 part j) ----
  /** Set the same fields on many tasks (up to 200); one result per task, a refusal's reason in English only. */
  bulkEditTasks(body: BulkEditBody): Promise<BulkResult> {
    return this.http.post(`/api/tasks/bulk`, { body });
  }
  /** Copy tasks into a plan, each a new task in that plan's first state. */
  copyTasks(body: BulkTransferBody): Promise<BulkResult> {
    return this.http.post(`/api/tasks/copy`, { body });
  }
  /** Move tasks to another plan; their sprint, component and parent are cleared, a status it lacks mapped. */
  moveTasks(body: BulkTransferBody): Promise<BulkResult> {
    return this.http.post(`/api/tasks/move`, { body });
  }
  /** Delete one task (kept, marked deleted, and audited). */
  deleteTask(pid: string, taskPid: string): Promise<void> {
    return this.http.delete(`/api/plans/${pid}/tasks/${taskPid}`);
  }
  /** The address of the plan's live board changes (`text/event-stream`), through the proxy. */
  liveUrl(pid: string): string {
    return `${API_BASE_URL}/api/plans/${encodeURIComponent(pid)}/live`;
  }
  /** A sprint's ceremonies, every kind counted (FR-29). */
  sprintCeremonies(sprintPid: string): Promise<SprintCeremonies> {
    return this.http.get(`/api/sprints/${sprintPid}/ceremonies`);
  }
  /** Record a ceremony held; a second planning or review is refused (422). */
  holdCeremony(
    sprintPid: string,
    body: CeremonyBody,
  ): Promise<{ pid: string }> {
    return this.http.post(`/api/sprints/${sprintPid}/ceremonies`, { body });
  }
  /** The tasks committed to a sprint beside those in it now. */
  sprintCommitment(sprintPid: string): Promise<SprintCommitment> {
    return this.http.get(`/api/sprints/${sprintPid}/commitment`);
  }
  /** Commit the sprint's tasks as they are now, once (a second is 422). */
  commitSprint(sprintPid: string): Promise<{ committed: number }> {
    return this.http.post(`/api/sprints/${sprintPid}/commit`, { body: {} });
  }
  /** Effort against the capacity declared; left-out values are the service's defaults. */
  utilization(q: UtilizationQuery = {}): Promise<Utilization> {
    return this.http.get(
      `/api/capacity/utilization${querySearch({ by: q.by, window_days: q.window_days, plan_pid: q.plan_pid })}`,
    );
  }
  /** Planned against recorded hours per plan, over a window. */
  plannedHours(q: PlannedHoursQuery = {}): Promise<PlannedHours> {
    return this.http.get(
      `/api/resource-plan/hours${querySearch({ from: q.from, to: q.to })}`,
    );
  }
  /** Each required skill of the plan's allocations against what the person holds. */
  skillGap(pid: string): Promise<SkillGap> {
    return this.http.get(`/api/plans/${pid}/skill-gap`);
  }
  /** The cost rate in force for a person on a day (finance only: anyone else is answered 404). */
  rateLookup(q: RateLookupQuery): Promise<RateLookup> {
    return this.http.get(
      `/api/cost-rates/lookup${querySearch({ person_ref: q.person_ref, date: q.date, table: q.table })}`,
    );
  }
  /** Give a placeholder a named person; 409 `over_capacity` unless `confirm`. */
  resolvePlaceholder(
    pid: string,
    allocationPid: string,
    body: ResolvePlaceholderBody,
  ): Promise<{ pid: string; person_ref: string; over_capacity: boolean }> {
    return this.http.post(
      `/api/plans/${pid}/allocations/${allocationPid}/resolve`,
      { body },
    );
  }
  /** Remove an allocation (kept, marked deleted, and audited). */
  removeAllocation(pid: string, allocationPid: string): Promise<void> {
    return this.http.delete(`/api/plans/${pid}/allocations/${allocationPid}`);
  }
  /** A plan's alignment-weighted score and the objectives behind it. */
  planOkr(pid: string): Promise<PlanOkr> {
    return this.http.get(`/api/plans/${pid}/okr`);
  }
  /** An objective's key results, each with its progress or why it has none. */
  objectiveKeyResults(objectivePid: string): Promise<ObjectiveKeyResult[]> {
    return this.http.get(`/api/objectives/${objectivePid}/key-results`);
  }
  /** A key result's check-ins, newest first. */
  keyResultCheckIns(keyResultPid: string): Promise<KeyResultCheckIn[]> {
    return this.http.get(`/api/key-results/${keyResultPid}/check-ins`);
  }
  /** Every reading of a KPI, newest first, each one a later reading that day replaced marked. */
  kpiReadings(kpiPid: string): Promise<KpiReadings> {
    return this.http.get(`/api/kpis/${kpiPid}/readings`);
  }
  /** Record how many people use what the plan delivered, against how many it is for. */
  recordAdoption(pid: string, body: AdoptionBody): Promise<{ pid: string }> {
    return this.http.post(`/api/plans/${pid}/adoption`, { body });
  }
  // ---- plan templates (LT-2, NC-1 slice 9) ----
  /** Saved templates the caller may see, then the built-in starters. */
  planTemplates(): Promise<PlanTemplateSummary[]> {
    return this.http.get(`/api/plan-templates`);
  }
  /** One template and its whole snapshot (404 when absent or of a plan hidden from the caller). */
  planTemplate(pid: string): Promise<PlanTemplate> {
    return this.http.get(`/api/plan-templates/${pid}`);
  }
  /** Save a plan and the plans under it that the caller can read as a template; 409 when the name is taken. */
  saveAsTemplate(
    planPid: string,
    body: SaveTemplateBody,
  ): Promise<SavedTemplate> {
    return this.http.post(`/api/plans/${planPid}/save-as-template`, { body });
  }
  /** Start new plans from a template, dates shifted to `start_date` (422 without one when the template has dates). */
  instantiateTemplate(
    pid: string,
    body: InstantiateTemplateBody,
  ): Promise<InstantiatedTemplate> {
    return this.http.post(`/api/plans/from-template/${pid}`, { body });
  }
  /** Remove a saved template (422 for a starter); plans already made from it stay. */
  deletePlanTemplate(pid: string): Promise<unknown> {
    return this.http.delete(`/api/plan-templates/${pid}`);
  }
  // ---- delivery confidence (DP-2, NC-1 slice 11) ----
  /** A plan's stated delivery confidence: what is shown and the whole history. */
  deliveryConfidence(planPid: string): Promise<ConfidenceView> {
    return this.http.get(`/api/plans/${planPid}/delivery-confidence`);
  }
  /**
   * Add a stated assessment; it never replaces one. An independent one needs `confidence=independent` or admin (403).
   * Amber/red or red opens an exception, or worsens the open one; the answer carries it.
   */
  assessDeliveryConfidence(
    planPid: string,
    body: ConfidenceBody,
  ): Promise<{
    assessment: ConfidenceAssessment;
    exception: { pid: string; window_hours: number } | null;
  }> {
    return this.http.put(`/api/plans/${planPid}/delivery-confidence`, { body });
  }
  // ---- delivery exceptions (DP-3, NC-1 slice 11) ----
  /** Exceptions across the plans the caller can read, newest first; `status` and `breached` filter. */
  deliveryExceptions(
    filter: { status?: "open" | "closed"; breached?: boolean } = {},
  ): Promise<{ exceptions: DeliveryException[]; window_hours: number }> {
    const q = new URLSearchParams();
    if (filter.status) q.set("status", filter.status);
    if (filter.breached !== undefined)
      q.set("breached", String(filter.breached));
    const qs = q.toString();
    return this.http.get(`/api/exceptions${qs ? `?${qs}` : ""}`);
  }
  /** Fill in an open exception's cause, decision needed, owner and decide-by date (409 when it is closed). */
  completeException(
    pid: string,
    body: ExceptionCompleteBody,
  ): Promise<DeliveryException> {
    return this.http.put(`/api/exceptions/${pid}`, { body });
  }
  /** Record the decision (ten characters or more), which closes it; 422 for one a threshold opened (EX-7). */
  decideException(pid: string, decision: string): Promise<DeliveryException> {
    return this.http.post(`/api/exceptions/${pid}/decision`, {
      body: { decision },
    });
  }
  // ---- strategic aims and the portfolio map (DP-4, NC-1 slice 11) ----
  /** The live portfolio map for a period `YYYY-MM` (the current month when left out). */
  portfolioMap(period?: string): Promise<PortfolioMap> {
    return this.http.get(
      `/api/portfolio-map${period ? `?${new URLSearchParams({ period }).toString()}` : ""}`,
    );
  }
  /** Mark or unmark an objective as a strategic aim; unmarking clears it as any programme's primary aim. */
  markStrategicAim(
    objectivePid: string,
    strategic: boolean,
  ): Promise<{ objective_pid: string; strategic: boolean }> {
    return this.http.put(`/api/objectives/${objectivePid}/strategic-aim`, {
      body: { strategic },
    });
  }
  /** Make a strategic aim a programme's one primary aim (422 for an objective that is not an aim). */
  setPrimaryAim(
    planPid: string,
    objectivePid: string,
  ): Promise<{ plan_pid: string; primary_aim: string }> {
    return this.http.put(`/api/plans/${planPid}/primary-aim`, {
      body: { objective_pid: objectivePid },
    });
  }
  // ---- planned gates and programme status (DP-5, NC-1 slice 11) ----
  /** The dates a plan expects its gate reviews at. */
  plannedGates(planPid: string): Promise<PlannedGates> {
    return this.http.get(`/api/plans/${planPid}/planned-gates`);
  }
  /** Set the date a plan expects its review at one gate. */
  setPlannedGate(
    planPid: string,
    gate: string,
    plannedOn: string,
  ): Promise<{ gate: string; planned_on: string }> {
    return this.http.put(`/api/plans/${planPid}/planned-gates/${gate}`, {
      body: { planned_on: plannedOn },
    });
  }
  /** A programme's status, its decisions and what it still holds. */
  programmeStatus(planPid: string): Promise<ProgrammeStatusView> {
    return this.http.get(`/api/plans/${planPid}/programme-status`);
  }
  /**
   * Pause, resume or stop a programme by a recorded decision: `programme=decider` or admin only (403); a plan of kind
   * Program only (422); 409 for the status it already has, or for any change once stopped.
   */
  decideProgrammeStatus(
    planPid: string,
    body: ProgrammeStatusBody,
  ): Promise<{ status: string; decision: string }> {
    return this.http.post(`/api/plans/${planPid}/programme-status`, { body });
  }
  // ---- funding sources (DP-6, NC-1 slice 11) ----
  /** The funding sources, by name, active and retired. */
  fundingSources(): Promise<{ funding_sources: FundingSource[] }> {
    return this.http.get("/api/funding-sources");
  }
  /** Add a funding source (a name of 2 to 120 characters; 409 when the name exists, in any case). */
  createFundingSource(name: string): Promise<FundingSource> {
    return this.http.post("/api/funding-sources", { body: { name } });
  }
  /** Retire (`false`) or restore (`true`) a funding source; lines keep a retired one. */
  setFundingSourceActive(pid: string, active: boolean): Promise<FundingSource> {
    return this.http.put(`/api/funding-sources/${pid}`, { body: { active } });
  }
  /** Set a budget line's funding source, or clear it with `null` (422 for one that is not active). */
  setLineFundingSource(
    planPid: string,
    linePid: string,
    sourcePid: string | null,
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${planPid}/budget-lines/${linePid}/funding-source`,
      {
        body: sourcePid ? { funding_source_pid: sourcePid } : {},
      },
    );
  }
  /** Programme money by funding source for a financial year (the year it starts in; the current one if left out). */
  portfolioMoney(
    fy?: number,
    combineClasses = false,
  ): Promise<Record<string, unknown>> {
    const q = new URLSearchParams();
    if (fy !== undefined) q.set("fy", String(fy));
    if (combineClasses) q.set("combine_classes", "true");
    const qs = q.toString();
    return this.http.get(`/api/portfolio-money${qs ? `?${qs}` : ""}`);
  }
  // ---- the ledger's writes (EX-12, NC-1 slice 11) ----
  /** Record money a funding source allocated (append-only): above 0, an ISO currency, not dated in the future. */
  recordAllocation(body: AllocationBody): Promise<unknown> {
    return this.http.post("/api/funding-allocations", { body });
  }
  /** Draw a budget line on an allocation of its own source and currency (422 otherwise), or detach it with `null`. */
  setLineAllocation(
    planPid: string,
    linePid: string,
    allocationPid: string | null,
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${planPid}/budget-lines/${linePid}/funding-allocation`,
      {
        body: allocationPid ? { funding_allocation_pid: allocationPid } : {},
      },
    );
  }
  /** The finance cost codes mapped to budget lines the caller may read. */
  costCodes(): Promise<{
    cost_codes: Array<{ cost_code: string; budget_line_pid: string }>;
  }> {
    return this.http.get("/api/cost-codes");
  }
  /** Map a finance cost code (1 to 60 characters; the same code in any case is moved) to a budget line. */
  mapCostCode(
    costCode: string,
    linePid: string,
  ): Promise<{ cost_code: string; budget_line_pid: string }> {
    return this.http.put("/api/cost-codes", {
      body: { cost_code: costCode, budget_line_pid: linePid },
    });
  }
  /** Reconcile a finance export (CSV with cost_code, amount_minor and currency; under 1 MB) against the register. */
  reconcileLedger(csv: string): Promise<ReconcileReport> {
    return this.http.post("/api/ledger/reconcile", {
      rawBody: csv,
      headers: { "content-type": "text/csv" },
    });
  }
  // ---- the project buffer (DP-7, NC-1 slice 11) ----
  /** A programme's project buffer: its declarations, the fever-chart reading now and the chart's lines. */
  projectBuffer(planPid: string): Promise<ProjectBufferView> {
    return this.http.get(`/api/plans/${planPid}/project-buffer`);
  }
  /** Declare a programme's buffer in business days (1 to 2000) or by the rule, which needs a schedule baseline (422). */
  declareProjectBuffer(
    planPid: string,
    body: ProjectBufferBody,
  ): Promise<{ buffer_days: number; source: string }> {
    return this.http.put(`/api/plans/${planPid}/project-buffer`, { body });
  }
  // ---- benefit classification and validation (DP-9, NC-1 slice 11) ----
  /** Set a benefit's recurrence, yearly amount, classification and KPI; a field left out is cleared. */
  classifyBenefit(
    planPid: string,
    benefitPid: string,
    body: BenefitClassifyBody,
  ): Promise<unknown> {
    return this.http.put(
      `/api/plans/${planPid}/benefits/${benefitPid}/classification`,
      { body },
    );
  }
  /** A benefit's validations: the claim, the latest validated amount, its standing and the history. */
  benefitValidations(
    planPid: string,
    benefitPid: string,
  ): Promise<BenefitValidations> {
    return this.http.get(
      `/api/plans/${planPid}/benefits/${benefitPid}/validations`,
    );
  }
  /** State the amount validated to date; refused (403) from a member of the programme; append-only. */
  validateBenefit(
    planPid: string,
    benefitPid: string,
    body: { amount_minor: number; note?: string },
  ): Promise<unknown> {
    return this.http.post(
      `/api/plans/${planPid}/benefits/${benefitPid}/validations`,
      { body },
    );
  }
  // ---- delivery model and reporting method (EX-2, EX-14, NC-1 slice 11) ----
  /** A programme's delivery model, owning organization and reporting method. */
  deliveryProfile(planPid: string): Promise<DeliveryProfile> {
    return this.http.get(`/api/plans/${planPid}/delivery-model`);
  }
  /** Set a programme's delivery model and owning organization; a `null` clears it, and a field left out is cleared too. */
  setDeliveryModel(
    planPid: string,
    body: {
      delivery_model: string | null;
      delivery_owner_organization_pid: string | null;
    },
  ): Promise<DeliveryProfile> {
    return this.http.put(`/api/plans/${planPid}/delivery-model`, { body });
  }
  /** How a programme reports in the pack: earned value, or burn-up and flow for a product programme. */
  setReportingMethod(
    planPid: string,
    method: "evm" | "product",
  ): Promise<{ reporting_method: string }> {
    return this.http.put(`/api/plans/${planPid}/reporting-method`, {
      body: { method },
    });
  }
  // ---- strategy canvases (LT-1, NC-1 slice 9) ----
  /** Every built-in canvas template and its boxes, in display order. */
  canvasTemplates(): Promise<CanvasTemplate[]> {
    return this.http.get(`/api/canvas-templates`);
  }
  /** Live canvases, of one plan or of every scope; a canvas on a plan the caller cannot read is left out. */
  listCanvases(plan?: string): Promise<Canvas[]> {
    const q = plan ? `?${new URLSearchParams({ plan }).toString()}` : "";
    return this.http.get(`/api/canvases${q}`);
  }
  /** One canvas, every box of its template, and its items in order (404 when absent or on an unreadable plan). */
  canvas(pid: string): Promise<CanvasView> {
    return this.http.get(`/api/canvases/${pid}`);
  }
  /** Create a canvas from a built-in template, on a plan or free-standing. */
  createCanvas(body: CanvasCreateBody): Promise<{ pid: string }> {
    return this.http.post(`/api/canvases`, { body });
  }
  /** Remove a canvas and its items (soft-deleted; an item already promoted keeps what it became). */
  deleteCanvas(pid: string): Promise<unknown> {
    return this.http.delete(`/api/canvases/${pid}`);
  }
  /** Add an item at the end of a box (a canvas holds at most 500). */
  addCanvasItem(pid: string, body: CanvasItemBody): Promise<{ pid: string }> {
    return this.http.post(`/api/canvases/${pid}/items`, { body });
  }
  /** Edit an item's text or move it to another box of the same template. */
  updateCanvasItem(
    pid: string,
    item: string,
    body: CanvasItemUpdateBody,
  ): Promise<CanvasItem> {
    return this.http.put(`/api/canvases/${pid}/items/${item}`, { body });
  }
  /** Remove one item. */
  deleteCanvasItem(pid: string, item: string): Promise<unknown> {
    return this.http.delete(`/api/canvases/${pid}/items/${item}`);
  }
  /** Turn an item into an idea, a risk (plan canvases only) or an objective; once only (409 after). */
  promoteCanvasItem(
    pid: string,
    item: string,
    body: CanvasPromoteBody,
  ): Promise<{ kind: "idea" | "risk" | "objective"; pid: string }> {
    return this.http.post(`/api/canvases/${pid}/items/${item}/promote`, {
      body,
    });
  }
  /**
   * The access grants that stand today, oldest first (RG-9, NC-1 slice 7): administrators only (403 otherwise), plans
   * the caller cannot read left out, and every read audited. A 422 is a negative age or a format the service does not
   * write.
   */
  accessReview(q: AccessReviewQuery = {}): Promise<AccessReview> {
    return this.http.get(`/api/access-review${accessReviewQuery(q, false)}`);
  }
  /**
   * Where the browser downloads the access review as a spreadsheet: the service's own CSV, through the proxy, with an
   * empty decision column for the reviewer, formulas defused and `Content-Disposition: attachment`. Audited like the
   * page's own read.
   */
  accessReviewCsvUrl(q: AccessReviewQuery = {}): string {
    return `${API_BASE_URL}/api/access-review${accessReviewQuery(q, true)}`;
  }
  /** The caller's break-glass grants, every grant for an administrator, newest first (RG-10, NC-1 slice 7). */
  breakGlassGrants(): Promise<BreakGlassList> {
    return this.http.get(`/api/break-glass`);
  }
  /**
   * Open a private plan for a limited time (RG-10): `breakglass=eligible` or an administrator (403), a reason and a
   * second person who is told in the same step (422 when they cannot be, or when the plan is not private), one live
   * grant per plan and person (409), five a day (429). Recorded and audited.
   */
  openBreakGlass(body: BreakGlassOpenBody): Promise<BreakGlassOpened> {
    return this.http.post(`/api/break-glass`, { body });
  }
  /** End a grant now (RG-10): its holder or an administrator; anyone else gets 404. Audited. */
  revokeBreakGlass(grantPid: string): Promise<BreakGlassGrant> {
    return this.http.post(`/api/break-glass/${grantPid}/revoke`);
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
  /** Replace a task's fields (NC-1 slice 3); every field is sent, as the service clears one left out. */
  updateTask(
    pid: string,
    taskPid: string,
    body: UpdateTaskBody,
  ): Promise<Task> {
    return this.http.put(`/api/plans/${pid}/tasks/${taskPid}`, { body });
  }
  /** A task's links to other tasks, both ways (LT-10). */
  taskRelations(pid: string, taskPid: string): Promise<TaskRelationList> {
    return this.http.get(`/api/plans/${pid}/tasks/${taskPid}/relations`);
  }
  /** Link this task to another (LT-10); 409 when the link exists, 422 for a cycle or a date it would break. */
  addTaskRelation(
    pid: string,
    taskPid: string,
    body: TaskRelationBody,
  ): Promise<TaskRelationRow> {
    return this.http.post(`/api/plans/${pid}/tasks/${taskPid}/relations`, {
      body,
    });
  }
  /** Remove one of a task's links (LT-10); neither task is removed. */
  removeTaskRelation(
    pid: string,
    taskPid: string,
    relationPid: string,
  ): Promise<void> {
    return this.http.delete(
      `/api/plans/${pid}/tasks/${taskPid}/relations/${relationPid}`,
    );
  }

  // ---- comments, reactions, watchers and attachments (LT-4, RM-3, LT-5; NC-1 slice 5) ----
  /** A subject's live comments, oldest first (LT-4). Every page is read, following the `Link` header. */
  async comments(
    kind: DiscussionSubjectKind,
    subjectPid: string,
  ): Promise<Comment[]> {
    const q = new URLSearchParams({
      subject_kind: kind,
      subject_pid: subjectPid,
    });
    const first = await this.http.getPage<Comment>(
      `/api/comments?${q.toString()}`,
    );
    return this.http.followPages(first);
  }
  /** Comment on a subject (LT-4); 422 for a blank one or one over 4000 characters. */
  addComment(body: CommentBody): Promise<{ pid: string }> {
    return this.http.post(`/api/comments`, { body });
  }
  /** The author changes their comment (LT-4); the previous text is kept. Anyone else is 403. */
  editComment(commentPid: string, body: CommentEditBody): Promise<Comment> {
    return this.http.put(`/api/comments/${commentPid}`, { body });
  }
  /** The author removes their comment (a soft delete); the `POST …/delete` twin, as the default policy keeps DELETE for admins. */
  deleteComment(commentPid: string): Promise<void> {
    return this.http.post(`/api/comments/${commentPid}/delete`);
  }
  /** A comment's earlier versions, oldest first (LT-4). */
  commentHistory(commentPid: string): Promise<CommentEdit[]> {
    return this.http.get(`/api/comments/${commentPid}/history`);
  }
  /** Reaction counts on a subject and the caller's own (LT-4). */
  reactions(
    kind: DiscussionSubjectKind,
    subjectPid: string,
  ): Promise<ReactionSummary> {
    const q = new URLSearchParams({
      subject_kind: kind,
      subject_pid: subjectPid,
    });
    return this.http.get(`/api/reactions?${q.toString()}`);
  }
  /** React (LT-4); a repeat is not an error and writes nothing. */
  addReaction(body: ReactionBody): Promise<{ created: boolean }> {
    return this.http.post(`/api/reactions`, { body });
  }
  /** Take back the caller's own reaction (LT-4); the `POST …/remove` twin of DELETE. */
  removeReaction(body: ReactionBody): Promise<void> {
    return this.http.post(`/api/reactions/remove`, { body });
  }
  /** How many follow a subject and whether the caller does (RM-3); 401 without a sign-in. */
  watchStatus(
    kind: DiscussionSubjectKind,
    subjectPid: string,
  ): Promise<WatchStatus> {
    const q = new URLSearchParams({
      subject_kind: kind,
      subject_pid: subjectPid,
    });
    return this.http.get(`/api/watchers?${q.toString()}`);
  }
  /** Follow a subject (RM-3); idempotent. */
  watch(body: WatchBody): Promise<{ watching: boolean; created: boolean }> {
    return this.http.post(`/api/watchers`, { body });
  }
  /** Stop following (RM-3); the `POST …/unwatch` twin of DELETE. */
  unwatch(body: WatchBody): Promise<void> {
    return this.http.post(`/api/watchers/unwatch`, { body });
  }
  /** A subject's live attachments, oldest first (LT-5). Every page is read. */
  async attachments(
    kind: DiscussionSubjectKind,
    subjectPid: string,
  ): Promise<Attachment[]> {
    const q = new URLSearchParams({
      subject_kind: kind,
      subject_pid: subjectPid,
    });
    const first = await this.http.getPage<Attachment>(
      `/api/attachments?${q.toString()}`,
    );
    return this.http.followPages(first);
  }
  /**
   * Attach a file (LT-5): the bytes as the raw body with `contentType`, the name in the query. The service refuses
   * (422) a type outside its list, an empty file, one over its limit, content that is not what the type says, or one
   * its malware scan refuses; 503 when a configured scanner does not answer.
   */
  uploadAttachment(
    kind: DiscussionSubjectKind,
    subjectPid: string,
    file: Blob,
    filename: string,
    contentType: string,
  ): Promise<AttachmentUploaded> {
    const q = new URLSearchParams({
      subject_kind: kind,
      subject_pid: subjectPid,
      filename,
    });
    return this.http.post(`/api/attachments?${q.toString()}`, {
      rawBody: file,
      headers: { "content-type": contentType },
    });
  }
  /** Remove an attachment (a soft delete; the uploader or an admin); the `POST …/delete` twin of DELETE. */
  removeAttachment(attachmentPid: string): Promise<void> {
    return this.http.post(`/api/attachments/${attachmentPid}/delete`);
  }
  /**
   * Where the browser downloads an attachment: the service's own response, through the proxy, with its
   * `Content-Disposition: attachment`, type and `nosniff`, so a file is saved and never shown as a page.
   */
  attachmentDownloadUrl(attachmentPid: string): string {
    return `${API_BASE_URL}/api/attachments/${attachmentPid}/download`;
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

/** A plan's phase (FR-30), in order. */
export type PhaseToken =
  "initiating" | "planning" | "executing" | "controlling" | "closing";

/** `GET /api/plans/{pid}/start-check` (EX-8). */
export interface StartCheck {
  plan_pid: string;
  wip: {
    limit: number | null;
    limit_setting_unusable: boolean;
    in_delivery: number;
  };
  constraint_pool: string | null;
  demand_by_pool: {
    pool: string;
    months: { month: string; demand_fte: number }[];
  }[];
  would_be_refused: boolean;
  refusals: StartRefusal[];
  queue_position: number | null;
  note: string;
}

/** `GET /api/plans/{pid}/phase-history`. */
export interface PhaseHistory {
  plan_pid: string;
  current_phase: PhaseToken | null;
  next_phase: PhaseToken | null;
  transitions: {
    pid: string;
    from_phase: PhaseToken | null;
    to_phase: PhaseToken;
    reason: string | null;
    occurred_at: string;
  }[];
  durations: {
    phase: PhaseToken;
    ms: number;
    visits: number;
    current: boolean;
  }[];
  as_of: string;
}

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
    signoff: MilestoneSignoff;
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
    /** The service's English; never shown (the page words `untraced_reason_code`). */
    untraced_reason: string | null;
    /** Why money is untraced, as a code (NC-1 slice 12 part a); null when every unit is traced. */
    untraced_reason_code?:
      "no_readable_line" | "not_on_a_line" | "lines_over_plan" | null;
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
    signed: {
      role: string;
      signed_on: string;
      /** When it was signed (EX-6a). */
      signed_at?: string;
      /** Digest of the marked criteria it was given on; none when signed before EX-6a. */
      evidence_digest?: string | null;
      /** Whether it is for the evidence as it stands now. */
      current?: boolean;
    }[];
    missing: string[];
  };
  go_would_be_refused_because: string[];
}

/** The access review's query string, with `format=csv` for the spreadsheet; empty values are left out. */
function accessReviewQuery(q: AccessReviewQuery, csv: boolean): string {
  const params = new URLSearchParams();
  if (q.plan) params.set("plan", q.plan);
  if (q.older_than_days !== undefined)
    params.set("older_than_days", String(q.older_than_days));
  if (csv) params.set("format", "csv");
  const qs = params.toString();
  return qs ? `?${qs}` : "";
}
