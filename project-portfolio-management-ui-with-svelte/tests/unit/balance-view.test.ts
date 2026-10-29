// NC-9: the portfolio balance flags, and the join of cost, value and risk on the portfolio.
import { describe, expect, it } from "vitest";
import type {
  ExecutiveBenefits,
  FinancialVariance,
  RiskHeatmap,
} from "../../src/lib/api/ppm";
import {
  AHEAD_OF_VALUE_GAP,
  CONCENTRATION_SHARE,
  UNASSIGNED,
  buildBalance,
  currenciesOf,
  flagsFor,
  layoutBubbles,
  BALANCE_BOX,
} from "../../src/lib/balance-view";

const ref = (pid: string) => ({ pid, name: `P-${pid}`, kind: "Portfolio" });
const row = (currency: string, planned_minor: number) => ({
  currency,
  planned_minor,
  actual_minor: 0,
  remaining_minor: planned_minor,
  overrun: false,
  line_count: 1,
});

const variance = (
  groups: Array<[string | null, Array<[string, number]>]>,
): FinancialVariance =>
  ({
    as_of: "2026-10-29",
    note: "",
    by_collection: [],
    by_category: [],
    by_portfolio: groups.map(([pid, rows]) => ({
      portfolio: pid ? ref(pid) : null,
      variance: rows.map(([c, n]) => row(c, n)),
    })),
  }) as FinancialVariance;

const benefits = (
  groups: Array<[string, Array<[string, number]>]>,
): ExecutiveBenefits => ({
  as_of: "2026-10-29",
  note: "",
  portfolios: groups.map(([pid, rows]) => ({
    portfolio: ref(pid),
    benefits: rows.length,
    non_financial: 0,
    statuses: {},
    financial: rows.map(([currency, target_minor]) => ({
      currency,
      target_minor,
      realized_minor: 0,
      realization_ratio: null,
    })),
  })),
});

const heat = (groups: Array<[string, number]>): RiskHeatmap =>
  ({
    as_of: "2026-10-29",
    open_risks: 0,
    estate_open_exposure: 0,
    cells: {},
    top_risks: [],
    posture: groups.map(([pid, open_exposure]) => ({
      portfolio: ref(pid),
      open_exposure,
      escalated: 0,
      materialized: 0,
    })),
    concentration: [],
    overdue_reviews: [],
  }) as unknown as RiskHeatmap;

const kinds = (flags: Array<{ kind: string }>) => flags.map((f) => f.kind);

describe("the thresholds are the ones the page states", () => {
  it("half for concentration, a fifth for running ahead of value", () => {
    expect(CONCENTRATION_SHARE).toBe(0.5);
    expect(AHEAD_OF_VALUE_GAP).toBe(0.2);
  });
});

describe("flagsFor", () => {
  const g = (over: Partial<Parameters<typeof flagsFor>[0]>) => ({
    cost: 10,
    value: 10,
    risk: 10,
    costShare: 0.3,
    valueShare: 0.3,
    riskShare: 0.3,
    ...over,
  });

  it("flags nothing for a group in proportion", () => {
    expect(flagsFor(g({}), 3)).toEqual([]);
  });

  it("flags a concentration of cost at exactly half, not just below", () => {
    expect(kinds(flagsFor(g({ costShare: 0.5, valueShare: 0.5 }), 2))).toEqual([
      "cost_concentration",
    ]);
    expect(flagsFor(g({ costShare: 0.49, valueShare: 0.49 }), 2)).toEqual([]);
  });

  it("says the percentage of the concentration", () => {
    expect(flagsFor(g({ riskShare: 0.72, valueShare: 0.72 }), 3)).toEqual([
      { kind: "risk_concentration", percent: 72 },
    ]);
  });

  it("does not call a lone group a concentration", () => {
    expect(
      flagsFor(g({ costShare: 1, valueShare: 1, riskShare: 1 }), 1),
    ).toEqual([]);
  });

  it("flags cost running a fifth or more ahead of value, and risk likewise", () => {
    expect(
      kinds(
        flagsFor(g({ costShare: 0.45, valueShare: 0.25, riskShare: 0.3 }), 4),
      ),
    ).toEqual(["cost_ahead_of_value"]);
    expect(
      kinds(
        flagsFor(g({ costShare: 0.3, valueShare: 0.1, riskShare: 0.3 }), 4),
      ),
    ).toEqual(["cost_ahead_of_value", "risk_ahead_of_value"]);
    expect(
      flagsFor(g({ costShare: 0.44, valueShare: 0.25, riskShare: 0.3 }), 4),
    ).toEqual([]);
  });

  it("flags cost with no value recorded, once, instead of the share gap", () => {
    expect(
      kinds(
        flagsFor(
          g({ value: 0, valueShare: 0, costShare: 0.3, riskShare: 0.1 }),
          4,
        ),
      ),
    ).toEqual(["no_value"]);
  });

  it("does not say anything is ahead of value when no group has value to compare with", () => {
    expect(
      kinds(
        flagsFor(
          g({ value: 5, valueShare: 0, costShare: 0.4, riskShare: 0.4 }),
          4,
          false,
        ),
      ),
    ).toEqual([]);
  });

  it("flags nothing for a group with no cost and no risk", () => {
    expect(
      flagsFor(
        {
          cost: 0,
          value: 5,
          risk: 0,
          costShare: 0,
          valueShare: 1,
          riskShare: 0,
        },
        3,
      ),
    ).toEqual([]);
  });
});

describe("buildBalance", () => {
  const v = variance([
    [
      "a",
      [
        ["GBP", 7000],
        ["EUR", 100],
      ],
    ],
    ["b", [["GBP", 2000]]],
    ["c", [["GBP", 1000]]],
    [null, [["GBP", 500]]],
  ]);
  const b = benefits([
    ["a", [["GBP", 1000]]],
    ["b", [["GBP", 8000]]],
    ["c", [["GBP", 1000]]],
  ]);
  const h = heat([
    ["a", 30],
    ["b", 10],
  ]);

  it("lists the currencies named by cost or value, sorted, never merged", () => {
    expect(currenciesOf(v, b)).toEqual(["EUR", "GBP"]);
    expect(currenciesOf(null, null)).toEqual([]);
  });

  it("joins the three reads on the portfolio, largest cost first, in one currency", () => {
    const groups = buildBalance(v, b, h, "GBP");
    expect(groups.map((x) => x.key)).toEqual(["a", "b", "c", UNASSIGNED]);
    const a = groups[0]!;
    expect([a.cost, a.value, a.risk]).toEqual([7000, 1000, 30]);
    expect(a.costShare).toBeCloseTo(7000 / 10500);
    expect(groups[3]?.name).toBeNull();
  });

  it("states the imbalances the figures contain", () => {
    const groups = buildBalance(v, b, h, "GBP");
    const by = Object.fromEntries(groups.map((x) => [x.key, kinds(x.flags)]));
    expect(by.a).toEqual([
      "cost_concentration",
      "risk_concentration",
      "cost_ahead_of_value",
      "risk_ahead_of_value",
    ]);
    expect(by.b).toEqual([]);
    expect(by.c).toEqual([]);
    expect(by[UNASSIGNED]).toEqual(["no_value"]);
  });

  it("uses only the chosen currency, and compares nothing to value where none is recorded", () => {
    const groups = buildBalance(v, b, h, "EUR");
    expect(groups.find((x) => x.key === "a")?.cost).toBe(100);
    expect(groups.find((x) => x.key === "b")?.cost).toBe(0);
    const eur = benefits([["a", [["GBP", 1]]]]);
    expect(
      kinds(buildBalance(v, eur, h, "EUR").find((x) => x.key === "a")!.flags),
    ).toEqual(["cost_concentration", "risk_concentration", "no_value"]);
  });

  it("is empty with nothing loaded", () => {
    expect(buildBalance(null, null, null, "GBP")).toEqual([]);
  });
});

describe("layoutBubbles", () => {
  const groups = buildBalance(
    variance([
      ["a", [["GBP", 100]]],
      ["b", [["GBP", 50]]],
      ["c", [["GBP", 0]]],
    ]),
    benefits([
      ["a", [["GBP", 10]]],
      ["b", [["GBP", 20]]],
    ]),
    heat([
      ["a", 4],
      ["b", 1],
    ]),
    "GBP",
  );
  const bubbles = layoutBubbles(groups);

  it("starts both axes at zero and puts the largest at the far edge", () => {
    const a = bubbles.find((x) => x.key === "a")!;
    const c = bubbles.find((x) => x.key === "c")!;
    expect(a.cx).toBe(BALANCE_BOX.width - BALANCE_BOX.right);
    expect(c.cx).toBe(BALANCE_BOX.left);
    expect(c.cy).toBe(BALANCE_BOX.height - BALANCE_BOX.bottom);
  });

  it("sizes by the square root of risk, and still draws a group with none", () => {
    const a = bubbles.find((x) => x.key === "a")!;
    const b = bubbles.find((x) => x.key === "b")!;
    const c = bubbles.find((x) => x.key === "c")!;
    expect(a.r).toBe(BALANCE_BOX.maxRadius);
    expect(b.r).toBeCloseTo(BALANCE_BOX.maxRadius / 2);
    expect(c.r).toBe(BALANCE_BOX.minRadius);
  });

  it("does not divide by zero when every figure is zero", () => {
    const z = layoutBubbles(
      buildBalance(variance([["a", [["GBP", 0]]]]), null, null, "GBP"),
    );
    expect(z[0]?.cx).toBe(BALANCE_BOX.left);
    expect(Number.isFinite(z[0]?.cy)).toBe(true);
  });
});
