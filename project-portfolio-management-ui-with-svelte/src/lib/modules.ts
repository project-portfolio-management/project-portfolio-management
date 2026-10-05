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
  { key: "board", module: "tasks" },
  { key: "schedule", module: "milestones" },
  { key: "flow", module: "tasks" },
  { key: "critical-path", module: "tasks" },
  { key: "raci", module: "tasks" },
  { key: "earned-value", module: "financials" },
  { key: "calendar", module: "milestones" },
  { key: "schedule-health", module: "tasks" },
  { key: "leveling", module: "tasks" },
  { key: "schedule-risk", module: "tasks" },
  { key: "cost-risk", module: "financials" },
  { key: "joint-risk", module: "financials" },
  { key: "work", module: "tasks" },
  { key: "what-if", module: "tasks" },
  { key: "onboarding", module: "tasks" },
  { key: "handover", module: "tasks" },
  { key: "costs", module: "financials" },
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
