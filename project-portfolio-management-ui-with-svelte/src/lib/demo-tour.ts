// The demonstration tour: which screens it points at, by group. The words are in the page catalogue
// (`p.demo.g_<group>`, `p.demo.f_<id>_t`, `p.demo.f_<id>_d`). `{N}` is the North Line plan.

export const GROUPS = ["overview", "delivery", "risk", "money", "people", "strategy", "escalation", "chain", "tools"] as const;

export interface DemoFeature {
  id: string;
  group: (typeof GROUPS)[number];
  href: string;
}

export const FEATURES: DemoFeature[] = [
  {
    "id": "plans",
    "group": "overview",
    "href": "/plans"
  },
  {
    "id": "dashboard",
    "group": "overview",
    "href": "/dashboard"
  },
  {
    "id": "gantt",
    "group": "overview",
    "href": "/gantt"
  },
  {
    "id": "milestones",
    "group": "overview",
    "href": "/milestones"
  },
  {
    "id": "calendar",
    "group": "overview",
    "href": "/calendar"
  },
  {
    "id": "board",
    "group": "delivery",
    "href": "/plans/{N}/board"
  },
  {
    "id": "critical-path",
    "group": "delivery",
    "href": "/plans/{N}/critical-path"
  },
  {
    "id": "earned-value",
    "group": "delivery",
    "href": "/plans/{N}/earned-value"
  },
  {
    "id": "schedule-health",
    "group": "delivery",
    "href": "/plans/{N}/schedule-health"
  },
  {
    "id": "wbs",
    "group": "delivery",
    "href": "/plans/{N}/wbs"
  },
  {
    "id": "risk",
    "group": "risk",
    "href": "/risk"
  },
  {
    "id": "issues",
    "group": "risk",
    "href": "/issues"
  },
  {
    "id": "change-requests",
    "group": "risk",
    "href": "/change-requests"
  },
  {
    "id": "joint-risk",
    "group": "risk",
    "href": "/plans/{N}/joint-risk"
  },
  {
    "id": "financials",
    "group": "money",
    "href": "/financials"
  },
  {
    "id": "cost-risk",
    "group": "money",
    "href": "/cost-risk"
  },
  {
    "id": "ledger",
    "group": "money",
    "href": "/ledger"
  },
  {
    "id": "costs",
    "group": "money",
    "href": "/plans/{N}/costs"
  },
  {
    "id": "resources",
    "group": "people",
    "href": "/resources"
  },
  {
    "id": "capacity",
    "group": "people",
    "href": "/capacity"
  },
  {
    "id": "key-persons",
    "group": "people",
    "href": "/key-persons"
  },
  {
    "id": "objectives",
    "group": "strategy",
    "href": "/objectives"
  },
  {
    "id": "okrs",
    "group": "strategy",
    "href": "/okrs"
  },
  {
    "id": "kpis",
    "group": "strategy",
    "href": "/kpis"
  },
  {
    "id": "proposals",
    "group": "strategy",
    "href": "/proposals"
  },
  {
    "id": "ideas",
    "group": "strategy",
    "href": "/ideas"
  },
  {
    "id": "scenarios",
    "group": "strategy",
    "href": "/scenarios"
  },
  {
    "id": "director-packs",
    "group": "escalation",
    "href": "/director-packs"
  },
  {
    "id": "organizations",
    "group": "escalation",
    "href": "/organizations"
  },
  {
    "id": "milestone-signoffs",
    "group": "escalation",
    "href": "/milestone-signoffs"
  },
  {
    "id": "cutover",
    "group": "escalation",
    "href": "/plans/{N}/cutover"
  },
  {
    "id": "deployment-moves",
    "group": "escalation",
    "href": "/plans/{N}/deployment-moves"
  },
  {
    "id": "start-queue",
    "group": "escalation",
    "href": "/start-queue"
  },
  {
    "id": "critical-chain",
    "group": "chain",
    "href": "/plans/{N}/critical-chain"
  },
  {
    "id": "buffers",
    "group": "chain",
    "href": "/plans/{N}/buffers"
  },
  {
    "id": "pipeline",
    "group": "chain",
    "href": "/pipeline"
  },
  {
    "id": "search",
    "group": "tools",
    "href": "/search?Signalling"
  },
  {
    "id": "email-deliveries",
    "group": "tools",
    "href": "/email-deliveries"
  },
  {
    "id": "display",
    "group": "tools",
    "href": "/display"
  }
];

/** The href for a feature, or null when it needs the North Line plan and that is not known. */
export function demoHref(f: DemoFeature, northPid: string | null): string | null {
  if (!f.href.includes("{N}")) return f.href;
  return northPid ? f.href.replace("{N}", northPid) : null;
}
