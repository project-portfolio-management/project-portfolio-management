// Portfolio balance (NC-9): value, risk and cost for each portfolio, and the imbalances worth a look.
// Pure and tested; the page only draws it.
//
// The three figures come from three existing reads, joined on the portfolio:
//   cost  = planned cost on budget lines  (`GET /api/financials/variance`, `by_portfolio`)
//   value = the benefits' target          (`GET /api/executive/benefits`, `portfolios`)
//   risk  = open risk exposure            (`GET /api/risk/heatmap`, `posture`)
// Cost and value are money, so they are compared in ONE currency at a time and never converted. Risk
// exposure is the service's own score and has no currency.
//
// A flag is a statement about shares of the whole, with its threshold written here, not a verdict: the
// page says what was compared and leaves the decision to the person.

import type {
  ExecutiveBenefits,
  FinancialVariance,
  InsightItemRef,
  RiskHeatmap,
} from "#lib/api/ppm.js";

/** One group (portfolio) holds at least this share of the total cost or the total risk. */
export const CONCENTRATION_SHARE = 0.5;
/** A group's share of cost (or of risk) exceeds its share of value by at least this much. */
export const AHEAD_OF_VALUE_GAP = 0.2;
/** Concentration means little with fewer groups than this, so it is not flagged. */
export const MIN_GROUPS_FOR_CONCENTRATION = 2;

export type FlagKind =
  | "cost_concentration"
  | "risk_concentration"
  | "cost_ahead_of_value"
  | "risk_ahead_of_value"
  | "no_value";

export interface BalanceFlag {
  kind: FlagKind;
  /** Whole percent for the sentence, `null` where the flag has no percentage. */
  percent: number | null;
}

export interface BalanceGroup {
  key: string;
  /** `null` for plans under no portfolio. */
  name: string | null;
  cost: number;
  value: number;
  risk: number;
  costShare: number;
  valueShare: number;
  riskShare: number;
  flags: BalanceFlag[];
}

/** The key of the group for items with no portfolio. */
export const UNASSIGNED = "unassigned";

interface Raw {
  key: string;
  name: string | null;
  cost: number;
  value: number;
  risk: number;
}

const keyOf = (p: InsightItemRef | null): string => p?.pid ?? UNASSIGNED;

/** Every currency named by the cost or value reads, sorted. */
export function currenciesOf(
  variance: FinancialVariance | null,
  benefits: ExecutiveBenefits | null,
): string[] {
  const set = new Set<string>();
  for (const g of variance?.by_portfolio ?? [])
    for (const v of g.variance) set.add(v.currency);
  for (const g of benefits?.portfolios ?? [])
    for (const f of g.financial) set.add(f.currency);
  return [...set].sort();
}

/** Shares are compared to the nearest tenth of a percent, so 0.3 - 0.1 counts as the 0.2 it is. */
const r3 = (x: number): number => Math.round(x * 1000) / 1000;

const share = (part: number, whole: number): number =>
  whole > 0 ? part / whole : 0;

/** Flags for one group, given the totals. Nothing is flagged on a total of zero. */
export function flagsFor(
  g: Pick<
    BalanceGroup,
    "cost" | "value" | "risk" | "costShare" | "valueShare" | "riskShare"
  >,
  groupCount: number,
  /** Whether any group has value in this currency. With none, "ahead of value" has nothing to compare to. */
  valueKnown = true,
): BalanceFlag[] {
  const out: BalanceFlag[] = [];
  const pct = (x: number) => Math.round(x * 100);
  if (groupCount >= MIN_GROUPS_FOR_CONCENTRATION) {
    if (g.cost > 0 && r3(g.costShare) >= CONCENTRATION_SHARE) {
      out.push({ kind: "cost_concentration", percent: pct(g.costShare) });
    }
    if (g.risk > 0 && r3(g.riskShare) >= CONCENTRATION_SHARE) {
      out.push({ kind: "risk_concentration", percent: pct(g.riskShare) });
    }
  }
  if (g.cost > 0 && g.value === 0) {
    out.push({ kind: "no_value", percent: null });
  } else if (
    valueKnown &&
    r3(g.costShare - g.valueShare) >= AHEAD_OF_VALUE_GAP
  ) {
    out.push({ kind: "cost_ahead_of_value", percent: pct(g.valueShare) });
  }
  if (
    valueKnown &&
    g.risk > 0 &&
    r3(g.riskShare - g.valueShare) >= AHEAD_OF_VALUE_GAP
  ) {
    out.push({ kind: "risk_ahead_of_value", percent: pct(g.valueShare) });
  }
  return out;
}

/** The groups for `currency`, largest cost first, each with its shares and flags. */
export function buildBalance(
  variance: FinancialVariance | null,
  benefits: ExecutiveBenefits | null,
  heatmap: RiskHeatmap | null,
  currency: string,
): BalanceGroup[] {
  const raw = new Map<string, Raw>();
  const at = (p: InsightItemRef | null): Raw => {
    const key = keyOf(p);
    let r = raw.get(key);
    if (!r) {
      r = { key, name: p?.name ?? null, cost: 0, value: 0, risk: 0 };
      raw.set(key, r);
    }
    return r;
  };
  for (const g of variance?.by_portfolio ?? []) {
    const row = g.variance.find((v) => v.currency === currency);
    at(g.portfolio).cost += row?.planned_minor ?? 0;
  }
  for (const g of benefits?.portfolios ?? []) {
    const row = g.financial.find((f) => f.currency === currency);
    at(g.portfolio).value += row?.target_minor ?? 0;
  }
  for (const g of heatmap?.posture ?? []) {
    at(g.portfolio).risk += g.open_exposure;
  }
  const all = [...raw.values()];
  const total = (f: (r: Raw) => number) => all.reduce((s, r) => s + f(r), 0);
  const costTotal = total((r) => r.cost);
  const valueTotal = total((r) => r.value);
  const riskTotal = total((r) => r.risk);
  return all
    .map((r): BalanceGroup => {
      const base = {
        ...r,
        costShare: share(r.cost, costTotal),
        valueShare: share(r.value, valueTotal),
        riskShare: share(r.risk, riskTotal),
      };
      return { ...base, flags: flagsFor(base, all.length, valueTotal > 0) };
    })
    .sort((a, b) => b.cost - a.cost || a.key.localeCompare(b.key));
}

/** Where a bubble sits in the chart, in the chart's own pixel space. */
export interface Bubble {
  key: string;
  cx: number;
  cy: number;
  r: number;
}

export interface ChartBox {
  width: number;
  height: number;
  left: number;
  right: number;
  top: number;
  bottom: number;
  maxRadius: number;
  minRadius: number;
}

export const BALANCE_BOX: ChartBox = {
  width: 480,
  height: 320,
  left: 56,
  right: 40,
  top: 36,
  bottom: 44,
  maxRadius: 28,
  minRadius: 5,
};

/**
 * x is cost, y is value (up is more), area is risk. Every axis starts at zero, so a bubble's position
 * is a true proportion of the largest. A group with no risk gets the smallest bubble, still drawn.
 */
export function layoutBubbles(
  groups: readonly BalanceGroup[],
  box: ChartBox = BALANCE_BOX,
): Bubble[] {
  const maxCost = Math.max(0, ...groups.map((g) => g.cost));
  const maxValue = Math.max(0, ...groups.map((g) => g.value));
  const maxRisk = Math.max(0, ...groups.map((g) => g.risk));
  const w = box.width - box.left - box.right;
  const h = box.height - box.top - box.bottom;
  return groups.map((g) => ({
    key: g.key,
    cx: box.left + (maxCost > 0 ? (g.cost / maxCost) * w : 0),
    cy: box.top + h - (maxValue > 0 ? (g.value / maxValue) * h : 0),
    r:
      maxRisk > 0 && g.risk > 0
        ? Math.max(box.minRadius, Math.sqrt(g.risk / maxRisk) * box.maxRadius)
        : box.minRadius,
  }));
}
