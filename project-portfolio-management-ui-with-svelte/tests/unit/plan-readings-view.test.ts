// NC-1 slice 12 part (i), RM-5 and SC-4: a plan's readings' queries (checked first, defaults not sent) and words.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  absentKey,
  derivedKey,
  distributionQuery,
  flowTypeKey,
  forecastQuery,
  readingRefusalKey,
  rollupDepth,
  stateName,
} from "../../src/lib/plan-readings-view";

describe("the flow roll-up's depth", () => {
  it("leaves out an empty box and the default of 32", () => {
    expect(rollupDepth(null)).toEqual({ depth: undefined });
    expect(rollupDepth(32)).toEqual({ depth: undefined });
    expect(rollupDepth(3)).toEqual({ depth: 3 });
  });
  it("refuses anything but a whole number from 1 to 32", () => {
    for (const d of [0, 33, 2.5])
      expect(rollupDepth(d)).toEqual({ problem: "p.readings.problem_depth" });
  });
});

describe("the forecast's query", () => {
  const empty = {
    items: null,
    periods: null,
    periodDays: null,
    historyPeriods: null,
  };
  it("sends nothing for the defaults", () => {
    expect(forecastQuery(empty)).toEqual({ query: {} });
    expect(
      forecastQuery({
        items: null,
        periods: 4,
        periodDays: 7,
        historyPeriods: 12,
      }),
    ).toEqual({ query: {} });
  });
  it("sends what differs", () => {
    expect(
      forecastQuery({
        items: 20,
        periods: 8,
        periodDays: 14,
        historyPeriods: 26,
      }),
    ).toEqual({
      query: { items: 20, periods: 8, period_days: 14, history_periods: 26 },
    });
  });
  it("says every problem at once", () => {
    expect(
      forecastQuery({
        items: 0,
        periods: 521,
        periodDays: 91,
        historyPeriods: 5,
      }),
    ).toEqual({
      problems: [
        "p.readings.problem_items",
        "p.readings.problem_periods",
        "p.readings.problem_period_days",
        "p.readings.problem_history",
      ],
    });
  });
});

describe("the mix of finished work's query", () => {
  it("sends nothing for 90 days and no roll-up", () => {
    expect(
      distributionQuery({ windowDays: null, rollup: false, depth: 5 }),
    ).toEqual({ query: {} });
    expect(
      distributionQuery({ windowDays: 90, rollup: false, depth: null }),
    ).toEqual({ query: {} });
  });
  it("sends a window, a roll-up and its depth only when asked", () => {
    expect(
      distributionQuery({ windowDays: 30, rollup: true, depth: null }),
    ).toEqual({
      query: { window_days: 30, rollup: true },
    });
    expect(
      distributionQuery({ windowDays: null, rollup: true, depth: 0 }),
    ).toEqual({
      query: { rollup: true, depth: 0 },
    });
  });
  it("refuses a window the service would quietly read as 90 days", () => {
    expect(
      distributionQuery({ windowDays: 0, rollup: true, depth: 33 }),
    ).toEqual({
      problems: ["p.readings.problem_window", "p.readings.problem_depth_zero"],
    });
    expect(
      distributionQuery({ windowDays: 3651, rollup: false, depth: 33 }),
    ).toEqual({
      problems: ["p.readings.problem_window"],
    });
  });
});

describe("words", () => {
  it("names each kind of work, and an unknown one", () => {
    expect(flowTypeKey("defect")).toBe("p.readings.flow_defect");
    expect(flowTypeKey("spike")).toBe("p.readings.flow_other");
  });
  it("says why performance is absent", () => {
    expect(absentKey("no_baseline")).toBe("p.readings.absent_no_baseline");
    expect(absentKey("several_currencies")).toBe(
      "p.readings.absent_several_currencies",
    );
    expect(absentKey(null)).toBe("p.readings.absent_other");
  });
  it("names a built-in state in the catalogue and a plan's own in its words", () => {
    const tt = (k: string) => `[${k}]`;
    expect(
      stateName(
        { key: "in_review", label: "In review" },
        "built_in_or_default",
        "cy",
        tt,
      ),
    ).toBe("[p.task.s_in_review]");
    expect(
      stateName(
        { key: "todo", label: "Backlog", labels: { cy: "Ôl-groniad" } },
        "plan",
        "cy",
        tt,
      ),
    ).toBe("Ôl-groniad");
    expect(stateName({ key: "todo", label: "Backlog" }, "plan", "de", tt)).toBe(
      "Backlog",
    );
  });
  it("says why a rolled-up figure is absent", () => {
    expect(derivedKey({ value: 3, reason: null })).toBeNull();
    expect(derivedKey({ value: null, reason: "no_estimates" })).toBe(
      "p.taskrollup.no_estimates",
    );
    expect(derivedKey({ value: null, reason: "no_dated_children" })).toBe(
      "p.taskrollup.no_dated_children",
    );
    expect(derivedKey({ value: null, reason: null })).toBe(
      "p.taskrollup.absent",
    );
  });
});

describe("a refused reading", () => {
  it("tells a module switched off from a plan gone, by the service's code", () => {
    expect(
      readingRefusalKey(
        "effort",
        new ApiError(404, "x", { code: "module_disabled" }),
      ),
    ).toBe("p.readings.refused_module");
    expect(
      readingRefusalKey(
        "effort",
        new ApiError(404, "x", { code: "not_found" }),
      ),
    ).toBe("p.readings.refused_gone");
    expect(readingRefusalKey("task_rollup", new ApiError(404, "x", {}))).toBe(
      "p.taskrollup.refused_gone",
    );
    expect(readingRefusalKey("forecast", new ApiError(422, "x", {}))).toBe(
      "p.readings.refused_query",
    );
    expect(
      readingRefusalKey("forecast", new ApiError(500, "x", {})),
    ).toBeNull();
  });
});
