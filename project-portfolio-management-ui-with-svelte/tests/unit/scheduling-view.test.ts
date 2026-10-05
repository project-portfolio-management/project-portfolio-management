import { describe, it, expect } from "vitest";
import {
  confidenceText,
  criticalityText,
  finishEffectText,
  gapCell,
  healthReason,
  healthResult,
  healthSummary,
  healthValue,
  histogramGeometry,
  levelingReason,
  maxAbsGap,
  moveText,
  offenderHref,
  percentileText,
  runText,
  sensitivityText,
  skippedReasonText,
  sourceText,
  thresholdSentence,
} from "../../src/lib/scheduling-view";
import type {
  HealthCheck,
  LevelingMove,
  LevelingRun,
} from "../../src/lib/api/ppm";

const check = (over: Partial<HealthCheck> = {}): HealthCheck => ({
  id: 1,
  key: "missing_logic",
  name: "Missing logic",
  of_what: "incomplete tasks",
  count: 1,
  of: 5,
  value: 20,
  threshold: { op: "<=", limit: 5, unit: "percent" },
  pass: false,
  reason: null,
  offenders: [],
  note: null,
  ...over,
});

describe("schedule health words", () => {
  it("never reads a check that cannot be computed as a pass", () => {
    expect(healthResult({ pass: true })).toMatchObject({
      kind: "pass",
      text: "Pass",
    });
    expect(healthResult({ pass: false })).toMatchObject({
      kind: "fail",
      text: "Fail",
    });
    const na = healthResult({ pass: null });
    expect(na.kind).toBe("na");
    expect(na.text).not.toMatch(/pass/i);
    expect(
      new Set(
        ["pass", "fail", "na"].map(
          (k) =>
            healthResult({
              pass: k === "pass" ? true : k === "fail" ? false : null,
            }).symbol,
        ),
      ).size,
    ).toBe(3);
  });
  it("says 'not enough data' with the reason", () => {
    expect(healthReason("no_baseline")).toBe(
      "Not enough data: No schedule baseline has been taken for this plan.",
    );
    expect(healthReason("cycle")).toContain("cycle");
    expect(healthReason("odd")).toBe("Not enough data: odd");
    expect(healthReason(null)).toBe("Not enough data.");
  });
  it("states the threshold and the finding", () => {
    expect(thresholdSentence(check())).toBe("At most 5%");
    expect(
      thresholdSentence(
        check({ threshold: { op: ">=", limit: 90, unit: "percent" } }),
      ),
    ).toBe("At least 90%");
    expect(
      thresholdSentence(
        check({ threshold: { op: "==", limit: 0, unit: "percent" } }),
      ),
    ).toBe("Exactly 0%");
    expect(
      thresholdSentence(
        check({ threshold: { op: ">=", limit: 950, unit: "milli" } }),
      ),
    ).toBe("At least 0.95");
    expect(
      thresholdSentence(
        check({ threshold: { op: "pass", limit: 600, unit: "days" } }),
      ),
    ).toContain("600");
    expect(healthValue(check())).toBe("20% (1 of 5 incomplete tasks)");
    expect(healthValue(check({ value: null }))).toBe("No figure");
    expect(
      healthValue(
        check({
          value: 0.75,
          count: 15,
          of: 20,
          threshold: { op: ">=", limit: 950, unit: "milli" },
        }),
      ),
    ).toBe("0.75 (15 of 20)");
  });
  it("summarises and links offenders to the task on the critical-path page", () => {
    expect(healthSummary({ passed: 12, failed: 1, not_computable: 1 })).toBe(
      "12 passed, 1 failed, 1 not computable",
    );
    expect(offenderHref("p1", "t9")).toBe("/plans/p1/critical-path#task-t9");
  });
});

const move: LevelingMove = {
  pid: "t1",
  title: "Build",
  person: "person-1",
  person_ref: "person:abc",
  from_start: "2026-10-05",
  to_start: "2026-10-07",
  scheduled_start: "2026-10-07",
  from_finish: "2026-10-06",
  to_finish: "2026-10-08",
  delay_days: 2,
  critical: false,
  float_before: 3,
  float_after: 1,
  because_of: "2026-10-05",
};

describe("leveling words", () => {
  it("writes a move, naming the person only when not masked", () => {
    expect(moveText(move)).toBe(
      "Build (person:abc) starts 2026-10-05 → 2026-10-07, 2 working days later; finishes 2026-10-08 instead of 2026-10-06.",
    );
    expect(moveText({ ...move, masked: true, person_ref: null })).toContain(
      "(person-1)",
    );
    expect(moveText({ ...move, critical: true })).toContain(
      "project finish moves",
    );
    expect(moveText({ ...move, delay_days: 1 })).toContain(
      "1 working day later",
    );
  });
  it("writes a run without numbers for a masked caller", () => {
    const run: LevelingRun = {
      person: "person-1",
      person_ref: "person:abc",
      from: "2026-10-05",
      to: "2026-10-06",
      demand: 200,
      capacity: 100,
      tasks: [],
    };
    expect(runText(run)).toBe(
      "person:abc is asked for 200% on 2026-10-05 to 2026-10-06 and can give 100%.",
    );
    expect(
      runText({
        ...run,
        masked: true,
        person_ref: null,
        demand: null,
        capacity: null,
      }),
    ).toBe("person-1 is over capacity on 2026-10-05 to 2026-10-06.");
    expect(runText({ ...run, to: run.from })).toContain("on 2026-10-05 and");
  });
  it("explains why something is left unresolved, and the finish effect", () => {
    expect(levelingReason("critical_task")).toContain("critical");
    expect(levelingReason("no_float")).toContain("negative float");
    expect(levelingReason(null)).toBe("Not resolved.");
    expect(finishEffectText("2026-10-09", "2026-10-09", 0)).toBe(
      "The project finish stays 2026-10-09.",
    );
    expect(finishEffectText("2026-10-09", "2026-10-16", 5)).toBe(
      "The project finish moves from 2026-10-09 to 2026-10-16 (5 working days later).",
    );
    expect(finishEffectText(null, null, 0)).toBe(
      "No project finish can be computed.",
    );
  });
});

describe("resource heatmap cells", () => {
  it("states a shortage in FTE with a word and a symbol, never just colour", () => {
    const rows = [
      { gap_fte: -0.5, supply_fte: 2, demand_fte: 2.5 },
      { gap_fte: 1, supply_fte: 3, demand_fte: 2 },
    ];
    const max = maxAbsGap(rows);
    expect(max).toBe(1);
    expect(gapCell(rows[0], max)).toEqual({
      kind: "short",
      symbol: "▼",
      text: "Short 0.5 FTE",
      level: 0.5,
    });
    expect(gapCell(rows[1], max)).toMatchObject({
      kind: "spare",
      symbol: "▲",
      text: "Spare 1 FTE",
      level: 1,
    });
    expect(
      gapCell({ gap_fte: 0, supply_fte: 1, demand_fte: 1 }, max),
    ).toMatchObject({ kind: "balanced", level: 0 });
  });
  it("does not invent a gap where supply is unknown or absent", () => {
    expect(
      gapCell({ gap_fte: null, supply_fte: null, demand_fte: 1.5 }, 1),
    ).toMatchObject({
      kind: "unknown",
      text: "Demand 1.5 FTE; supply not known",
    });
    expect(gapCell(undefined, 1)).toMatchObject({
      kind: "unknown",
      text: "No demand or supply",
    });
    expect(maxAbsGap([])).toBe(0);
  });
});

describe("risk histogram", () => {
  const hist = [
    { date: "2026-10-12", count: 10 },
    { date: "2026-10-13", count: 40 },
    { date: "2026-10-15", count: 20 },
  ];
  const pcts = [
    { p: 10, finish: "2026-10-12" },
    { p: 50, finish: "2026-10-13" },
    { p: 80, finish: "2026-10-15" },
    { p: 90, finish: "2026-10-15" },
  ];
  it("draws a bar only where there are counts, tallest for the biggest", () => {
    const g = histogramGeometry(hist, pcts, "2026-10-13");
    expect(g).not.toBeNull();
    expect(g?.bars).toHaveLength(3);
    const tallest = [...(g?.bars ?? [])].sort((a, b) => b.h - a.h)[0];
    expect(tallest?.date).toBe("2026-10-13");
    expect(g?.bars[2]?.x).toBeGreaterThan(
      (g?.bars[1]?.x ?? 0) + (g?.bars[1]?.w ?? 0),
    );
  });
  it("marks P50, P80 and P90 (not P10) and the critical-path finish", () => {
    const g = histogramGeometry(hist, pcts, "2026-10-13");
    expect(g?.markers.map((m) => m.p)).toEqual([50, 80, 90]);
    expect(g?.cpmX).not.toBeNull();
    expect(histogramGeometry(hist, pcts, "2027-01-01")?.cpmX).toBeNull();
    expect(histogramGeometry(hist, pcts, null)?.cpmX).toBeNull();
  });
  it("returns nothing for an empty histogram", () => {
    expect(histogramGeometry([], [], null)).toBeNull();
  });
  it("says things in words", () => {
    expect(confidenceText(6200, "2026-10-13")).toBe(
      "62% of the simulated outcomes finish on or before 2026-10-13.",
    );
    expect(percentileText({ p: 80, finish: "2026-10-15" })).toBe(
      "P80: 2026-10-15",
    );
    expect(criticalityText(3333)).toBe("33%");
    expect(sensitivityText(null)).toBe("No variation");
    expect(sensitivityText(0.81)).toBe("0.81 (strong)");
    expect(sensitivityText(-0.5)).toBe("-0.50 (moderate)");
    expect(sensitivityText(0.1)).toContain("weak");
    expect(sourceText({ source: "fallback", widened: false })).toContain(
      "No estimate",
    );
    expect(sourceText({ source: "estimated", widened: true })).toContain(
      "widened",
    );
    expect(sourceText({ source: "milestone", widened: false })).toContain(
      "no spread",
    );
    expect(skippedReasonText("no_probability")).toBe("no probability");
    expect(skippedReasonText("zzz")).toBe("zzz");
  });
});

import {
  budgetConfidenceText,
  costHistogramGeometry,
  costSkippedText,
  delayCostText,
  eventHitText,
  jointConfidenceText,
  lineSourceText,
  quadrantRows,
  scatterGeometry,
} from "../../src/lib/scheduling-view";

describe("cost risk words and geometry", () => {
  const bins = [
    { from_minor: 1000, to_minor: 2000, count: 10 },
    { from_minor: 2000, to_minor: 3000, count: 40 },
    { from_minor: 3000, to_minor: 4000, count: 0 },
  ];
  const pcts = [
    { p: 10, minor: 1200 },
    { p: 50, minor: 2400 },
    { p: 80, minor: 2900 },
    { p: 90, minor: 3100 },
  ];
  it("draws every bin, tallest for the biggest, and marks P50 / P80 / P90 and the budget", () => {
    const g = costHistogramGeometry(bins, pcts, 2500);
    expect(g?.bars).toHaveLength(3);
    expect(g?.bars[1]?.h).toBeGreaterThan(g?.bars[0]?.h ?? 0);
    expect(g?.bars[2]?.h).toBe(0);
    expect(g?.markers.map((m) => m.p)).toEqual([50, 80, 90]);
    expect(g?.budgetX).not.toBeNull();
    expect(costHistogramGeometry(bins, pcts, 9999)?.budgetX).toBeNull();
    expect(costHistogramGeometry([], [], 0)).toBeNull();
  });
  it("states the budget confidence and the events in words", () => {
    expect(budgetConfidenceText(6200, "150.00 GBP")).toBe(
      "62% of the simulated outcomes cost no more than the budget (150.00 GBP).",
    );
    expect(eventHitText(5000, 4987)).toBe(
      "stated 50%, struck in 49.9% of runs",
    );
    expect(costSkippedText("no_probability")).toBe("no probability");
    expect(costSkippedText("no_budget_in_currency")).toContain(
      "no budget line",
    );
    expect(costSkippedText("zzz")).toBe("zzz");
    expect(lineSourceText({ source: "fallback", widened: false })).toContain(
      "No estimate",
    );
    expect(lineSourceText({ source: "estimated", widened: true })).toContain(
      "widened",
    );
    expect(lineSourceText({ source: "estimated", widened: false })).toBe(
      "Estimated",
    );
  });
});

describe("joint risk words and geometry", () => {
  it("lists the four quadrants with shares that add up", () => {
    const rows = quadrantRows({
      both: 50,
      budget_only: 20,
      time_only: 10,
      neither: 20,
    });
    expect(rows.map((r) => r.key)).toEqual([
      "both",
      "budget_only",
      "time_only",
      "neither",
    ]);
    expect(rows.map((r) => r.share)).toEqual([
      "50.0%",
      "20.0%",
      "10.0%",
      "20.0%",
    ]);
    expect(new Set(rows.map((r) => r.label)).size).toBe(4);
    expect(
      quadrantRows({ both: 0, budget_only: 0, time_only: 0, neither: 0 })[0]
        ?.share,
    ).toBe("—");
  });
  it("states the joint confidence beside both marginals", () => {
    expect(
      jointConfidenceText({ budget_bp: 7000, finish_bp: 6000, joint_bp: 4500 }),
    ).toBe(
      "Both within budget and on time: 45% (within budget alone 70%; on time alone 60%).",
    );
  });
  it("places dots with an on-time line at zero days and a budget line", () => {
    const s = scatterGeometry(
      [
        { days_late: -2, cost_minor: 900 },
        { days_late: 0, cost_minor: 1000 },
        { days_late: 6, cost_minor: 1400 },
      ],
      1000,
    );
    expect(s?.dots).toHaveLength(3);
    expect(s?.onTimeX).toBeGreaterThan(s?.dots[0]?.x ?? 0);
    expect(s?.onTimeX).toBeLessThan(s?.dots[2]?.x ?? 0);
    // Higher cost is higher on the page (a smaller y).
    expect((s?.dots[2]?.y ?? 0) < (s?.dots[0]?.y ?? 0)).toBe(true);
    expect(s?.budgetY).toBeCloseTo(s?.dots[1]?.y ?? -1);
    expect(scatterGeometry([], 0)).toBeNull();
    expect(
      scatterGeometry([{ days_late: 0, cost_minor: 5 }], 5),
    ).not.toBeNull();
  });
  it("says what a late day costs", () => {
    const f = (m: number, c: string) => `${m / 100} ${c}`;
    expect(delayCostText(0, "GBP", f)).toContain("costs nothing extra");
    expect(delayCostText(50_000, "GBP", f)).toBe(
      "Each working day late adds 500 GBP; finishing early earns nothing back.",
    );
  });
});
