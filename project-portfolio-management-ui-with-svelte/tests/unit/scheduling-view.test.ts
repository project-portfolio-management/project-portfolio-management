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
