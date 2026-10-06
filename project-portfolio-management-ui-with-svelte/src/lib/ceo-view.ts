// The CEO dashboard: what each tile says, worked out from data the server
// already derived. Pure and tested. Nothing is recomputed here that the server
// owns; this only counts, orders and words what it was given.

import type {
  DecisionEntry,
  ExecutiveHealth,
  KpiView,
  MilestoneView,
} from "#lib/api/ppm.js";

export type Rag = "red" | "amber" | "green";

/** Portfolio RAG counts added up across portfolios (each count is a plan). */
export function ragTotals(health: ExecutiveHealth): Record<Rag, number> {
  const out = { red: 0, amber: 0, green: 0 };
  for (const p of health.portfolios) {
    out.red += p.rag.red;
    out.amber += p.rag.amber;
    out.green += p.rag.green;
  }
  return out;
}

/** The worst status among portfolios, or null when there are none. */
export function worstStatus(health: ExecutiveHealth): Rag | null {
  const order: Rag[] = ["red", "amber", "green"];
  for (const r of order)
    if (health.portfolios.some((p) => p.status === r)) return r;
  return null;
}

/** One plain sentence for the health tile. */
export function healthSentence(health: ExecutiveHealth): string {
  const t = ragTotals(health);
  const n = t.red + t.amber + t.green;
  if (health.portfolios.length === 0) return "No portfolios to report on yet.";
  if (n === 0) return "Portfolios exist but hold no plans yet.";
  return `${t.red} of ${n} plans are red, ${t.amber} amber, ${t.green} green.`;
}

export function escalatedRisks(health: ExecutiveHealth): number {
  return health.portfolios.reduce((a, p) => a + p.escalated_risks, 0);
}

export function overdueMilestones(health: ExecutiveHealth): number {
  return health.portfolios.reduce((a, p) => a + p.overdue_milestones, 0);
}

/** Portfolios that are over budget in some currency (their names, for the tile). */
export function overrunPortfolios(health: ExecutiveHealth): string[] {
  return health.portfolios
    .filter((p) => p.overrun_currencies.length > 0)
    .map((p) => p.portfolio?.name ?? "(unassigned)");
}

const KPI_RANK: Record<string, number> = {
  red: 0,
  amber: 1,
  no_data: 2,
  green: 3,
};

/** The `n` KPIs that most need a look (red, then amber, then no data), and how many more there are. */
export function needsAttention(
  kpis: KpiView[],
  n: number,
): { shown: KpiView[]; more: number } {
  const bad = kpis
    .filter((k) => k.status !== "green")
    .sort(
      (a, b) =>
        (KPI_RANK[a.status] ?? 9) - (KPI_RANK[b.status] ?? 9) ||
        a.kpi.name.localeCompare(b.kpi.name),
    );
  return { shown: bad.slice(0, n), more: Math.max(0, bad.length - n) };
}

const MS_RANK: Record<string, number> = {
  late: 0,
  due_soon: 1,
  upcoming: 2,
  achieved: 3,
};

/** The `n` milestones to watch: late first, then soonest due; achieved ones are left out. */
export function milestonesToWatch(
  milestones: MilestoneView[],
  n: number,
): { shown: MilestoneView[]; more: number } {
  const open = milestones
    .filter((m) => !m.achieved)
    .sort(
      (a, b) =>
        (MS_RANK[a.status] ?? 9) - (MS_RANK[b.status] ?? 9) ||
        a.due.localeCompare(b.due) ||
        a.name.localeCompare(b.name),
    );
  return { shown: open.slice(0, n), more: Math.max(0, open.length - n) };
}

const DECISION_TEXT: Record<string, string> = {
  gate_review: "Gate review",
  scenario_commit: "Scenario committed",
  proposal: "Proposal",
  merge: "Plans merged",
};

/** The `n` newest decisions as one line each ("Gate review: Approved — Apollo"). */
export function decisionLines(
  decisions: DecisionEntry[],
  n: number,
): { lines: { key: string; text: string; at: string }[]; more: number } {
  const sorted = [...decisions].sort((a, b) => b.at.localeCompare(a.at));
  return {
    lines: sorted.slice(0, n).map((d, i) => ({
      key: `${d.at}-${i}`,
      text: `${DECISION_TEXT[d.kind] ?? d.kind}: ${d.decision}${d.subject.name ? ` — ${d.subject.name}` : ""}`,
      at: d.at.slice(0, 10),
    })),
    more: Math.max(0, sorted.length - n),
  };
}
