// Pure helpers for RM-7 (per-plan feature modules): which of a plan's
// workspace links to show, given the modules the service says are on.
// Extracted from `routes/plans/[pid]/+page.svelte` so the rule is
// unit-testable without mounting a component (the `nav.ts` pattern).
//
// Fail OPEN: when the module list is unknown (not loaded yet, or the call
// failed) every link is shown. Hiding a feature because of a transient error
// would be worse than showing one the server would then refuse with 404.

/** The plan workspace links, in display order, and the module each needs. */
export const PLAN_LINKS = [
  { key: "governance", module: "governance" },
  // `delivery-confidence` is owned by no module in the service (always on); it is a governance reading.
  { key: "confidence", module: "governance" },
  // `planned-gates` and `programme-status` are owned by no module (always on); they are governance.
  { key: "programme", module: "governance" },
  // `status`, `status-cadence` and `status-updates` are owned by no module in the service (always on); a status update
  // is a governance reading, so it sits beside the programme page (NC-4, NC-1 slice 12 part b).
  { key: "status", module: "governance" },
  // `meetings`, `lessons` and `references` are owned by no module in the service (always on); meetings and lessons are
  // governance records, so they sit beside the status page (NC-14, NC-8, NC-1 slice 12 part e).
  { key: "meetings", module: "governance" },
  { key: "lessons", module: "governance" },
  { key: "tolerances", module: "governance" },
  { key: "escalations", module: "governance" },
  // The service puts `/api/plans/{pid}/issues` in the risks module, not governance.
  { key: "issues", module: "risks" },
  { key: "change-requests", module: "governance" },
  { key: "ways-of-working", module: "governance" },
  { key: "governance-framework", module: "governance" },
  { key: "links", module: "governance" },
  { key: "board", module: "tasks" },
  { key: "schedule", module: "milestones" },
  { key: "flow", module: "tasks" },
  { key: "critical-path", module: "tasks" },
  { key: "raci", module: "tasks" },
  { key: "earned-value", module: "financials" },
  { key: "calendar", module: "milestones" },
  { key: "schedule-health", module: "tasks" },
  { key: "leveling", module: "tasks" },
  { key: "critical-chain", module: "tasks" },
  { key: "buffers", module: "tasks" },
  { key: "deployment-moves", module: "tasks" },
  { key: "cutover", module: "governance" },
  { key: "schedule-risk", module: "tasks" },
  { key: "cost-risk", module: "financials" },
  { key: "joint-risk", module: "financials" },
  { key: "work", module: "tasks" },
  { key: "what-if", module: "tasks" },
  { key: "onboarding", module: "tasks" },
  { key: "handover", module: "tasks" },
  { key: "wbs", module: "tasks" },
  { key: "import", module: "tasks" },
  // `template-diff` and `template-sync` are owned by no module in the service (always on); a template's changes are
  // tasks, so the page sits with import and export (NC-12, NC-1 slice 12 part k).
  { key: "template", module: "tasks" },
  // The service files `export` under no module (always on); it sits beside import, which is filed under tasks.
  { key: "export", module: "tasks" },
  { key: "costs", module: "financials" },
  // `benefits` is owned by no module in the service (always on); it sits with the money, under financials.
  { key: "benefits", module: "financials" },
] as const;

export type PlanLinkKey = (typeof PLAN_LINKS)[number]["key"];

/**
 * The link keys to show. `modules` is the enabled set from
 * `GET /api/plans/{pid}/modules`, or `null` when it is unknown.
 */
export function visiblePlanLinks(
  modules: readonly string[] | null,
): PlanLinkKey[] {
  return PLAN_LINKS.filter(
    (l) => modules === null || modules.includes(l.module),
  ).map((l) => l.key);
}
