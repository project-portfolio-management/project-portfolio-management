// NC-1 slice 12 part (i), CC-2: a task's aggressive and safe estimates, checked first; nothing sent for no change.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { CcpmTask } from "../../src/lib/api/ppm-types";
import {
  estimable,
  estimateBody,
  estimateFormOf,
  estimateRefusalKey,
  needsAttention,
  stateKey,
} from "../../src/lib/ccpm-estimate-view";

const task = (over: Partial<CcpmTask> = {}): CcpmTask => ({
  pid: "t1",
  title: "Build",
  likely_days: 5,
  aggressive: { days: 3, source: "recorded" },
  safe: { days: 6, source: "recorded" },
  state: "complete",
  ...over,
});

describe("the form", () => {
  it("is filled from the recorded figures only", () => {
    expect(estimateFormOf(task())).toEqual({ aggressive: 3, safe: 6 });
    expect(
      estimateFormOf(
        task({
          aggressive: { days: 2, source: "rule" },
          safe: { days: 4, source: "risk_range" },
        }),
      ),
    ).toEqual({ aggressive: null, safe: null });
    expect(estimateFormOf(null)).toEqual({ aggressive: null, safe: null });
  });
  it("offers no milestone and no task without a duration", () => {
    expect(estimable(task({ state: "milestone" }))).toBe(false);
    expect(estimable(task({ state: "no_duration" }))).toBe(false);
    expect(estimable(task({ state: "missing" }))).toBe(true);
  });
});

describe("the body", () => {
  it("is nothing for no change", () => {
    expect(estimateBody({ aggressive: 3, safe: 6 }, task())).toBeNull();
  });
  it("sends both figures, and says when it clears what is recorded", () => {
    expect(estimateBody({ aggressive: 2, safe: 6 }, task())).toEqual({
      body: { aggressive_days: 2, safe_days: 6 },
      clears: false,
    });
    expect(estimateBody({ aggressive: null, safe: undefined }, task())).toEqual(
      {
        body: { aggressive_days: null, safe_days: null },
        clears: true,
      },
    );
  });
  it("says every problem at once: whole days from 1 to 3650, aggressive not above safe", () => {
    expect(estimateBody({ aggressive: 0, safe: 3651 }, task())).toEqual({
      problems: ["p.ccpm.problem_aggressive", "p.ccpm.problem_safe"],
    });
    expect(estimateBody({ aggressive: 1.5, safe: 3 }, task())).toEqual({
      problems: ["p.ccpm.problem_aggressive"],
    });
    expect(estimateBody({ aggressive: 7, safe: 6 }, task())).toEqual({
      problems: ["p.ccpm.problem_order"],
    });
    expect(estimateBody({ aggressive: 6, safe: 6 }, task())).toHaveProperty(
      "body",
    );
  });
});

describe("standings and refusals", () => {
  it("marks what needs someone to act, and words an unknown standing", () => {
    expect(needsAttention("missing")).toBe(true);
    expect(needsAttention("inconsistent")).toBe(true);
    expect(needsAttention("derived")).toBe(false);
    expect(stateKey("complete")).toBe("p.ccpm.state_complete");
    expect(stateKey("new_one")).toBe("p.ccpm.state_other");
  });
  it("is the catalogue's sentence, never the service's", () => {
    const e = (s: number) =>
      new ApiError(s, "service text", {
        code: "unprocessable_entity",
        detail: "service text",
      });
    expect(estimateRefusalKey(e(422))).toBe("p.ccpm.refused_invalid");
    expect(estimateRefusalKey(e(404))).toBe("p.ccpm.refused_gone");
    expect(estimateRefusalKey(e(409))).toBeNull();
  });
});
