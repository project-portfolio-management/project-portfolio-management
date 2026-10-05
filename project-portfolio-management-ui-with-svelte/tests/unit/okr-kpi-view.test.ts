import { describe, it, expect } from "vitest";
import {
  confidenceText,
  keyResultProgress,
  percentText,
  progressValue,
  unmeasurableText,
  valueText as okrValue,
} from "../../src/lib/okr-view";
import {
  STATUS_ORDER,
  STATUS_SYMBOL,
  STATUS_TEXT,
  staleText,
  summaryParts,
  summaryText,
  thresholdText,
  trendText,
  valueText,
} from "../../src/lib/kpi-view";
import type { KpiView, OkrKeyResult } from "../../src/lib/api/ppm";

describe("okr view", () => {
  it("never turns not-measured into 0%", () => {
    expect(percentText(null)).toBe("Not measured");
    expect(percentText(undefined)).toBe("Not measured");
    expect(percentText(0)).toBe("0%");
    expect(progressValue(null)).toBeNull();
    expect(progressValue(0)).toBe(0);
  });
  it("formats basis points as a percentage and clamps the bar", () => {
    expect(percentText(4000)).toBe("40%");
    expect(percentText(3333)).toBe("33.3%");
    expect(progressValue(10_000)).toBe(100);
    expect(progressValue(25_000)).toBe(100);
    expect(progressValue(-5)).toBe(0);
  });
  it("says why a key result cannot be scored", () => {
    expect(unmeasurableText("NoRange")).toContain("no distance to travel");
    expect(unmeasurableText("NoTolerance")).toContain("tolerance");
    expect(unmeasurableText("Weird")).toContain("Weird");
    const kr = {
      key_result: {} as never,
      progress_basis_points: null,
      unmeasurable: "NoRange",
      latest_check_in: null,
    } satisfies OkrKeyResult;
    expect(keyResultProgress(kr)).toContain("Not measurable");
    expect(keyResultProgress({ ...kr, progress_basis_points: 5000 })).toBe(
      "50%",
    );
  });
  it("says confidence is not part of the score", () => {
    expect(confidenceText(70)).toBe("Confidence 70% (not part of the score)");
    expect(confidenceText(null)).toBeNull();
    expect(okrValue(40, 200, "users")).toBe("40 of 200 users");
    expect(okrValue(1, 2, null)).toBe("1 of 2");
  });
});

describe("kpi view", () => {
  const kpi = {
    pid: "k",
    plan_pid: null,
    objective_pid: null,
    name: "Uptime",
    description: null,
    unit: "percent",
    direction: "higher_better",
    target_value: 95,
    red_value: 80,
    cadence: "weekly",
    owner_ref: null,
  } as const;
  const view = (over: Partial<KpiView> = {}): KpiView => ({
    kpi,
    status: "amber",
    latest: { value: 90, observed_on: "2026-10-01", note: null },
    trend: "worsening",
    stale: false,
    readings: 2,
    ...over,
  });

  it("gives every status a word and a symbol, and orders red first", () => {
    expect([...STATUS_ORDER]).toEqual(["red", "amber", "no_data", "green"]);
    for (const s of STATUS_ORDER) {
      expect(STATUS_TEXT[s].length).toBeGreaterThan(2);
      expect(STATUS_SYMBOL[s].length).toBeGreaterThan(0);
    }
    expect(new Set(Object.values(STATUS_SYMBOL)).size).toBe(4);
    expect(STATUS_TEXT.no_data).not.toMatch(/target/i);
  });
  it("states the thresholds in the direction's own words", () => {
    expect(thresholdText(kpi)).toBe(
      "Target 95 or more; red at 80 or less percent",
    );
    expect(
      thresholdText({
        ...kpi,
        direction: "lower_better",
        target_value: 2,
        red_value: 10,
        unit: null,
      }),
    ).toBe("Target 2 or less; red at 10 or more");
  });
  it("writes trend, staleness and a spoken summary", () => {
    expect(trendText("improving")).toBe("Improving");
    expect(trendText("worsening")).toBe("Getting worse");
    expect(trendText(null)).toBeNull();
    expect(staleText(view({ stale: true }))).toContain("weekly");
    expect(staleText(view({ stale: false }))).toBeNull();
    expect(staleText(view({ stale: null }))).toBeNull();
    expect(summaryText(view())).toBe("Uptime: Watch, 90 percent on 2026-10-01");
    expect(summaryText(view({ status: "no_data", latest: null }))).toBe(
      "Uptime: No data yet, no reading yet",
    );
    expect(valueText(3, null)).toBe("3");
  });
  it("lists the summary red first", () => {
    const parts = summaryParts({ green: 5, amber: 2, red: 1, no_data: 3 });
    expect(parts.map((p) => p.status)).toEqual([
      "red",
      "amber",
      "no_data",
      "green",
    ]);
    expect(parts[0]).toEqual({ status: "red", label: "Off target", count: 1 });
  });
});
