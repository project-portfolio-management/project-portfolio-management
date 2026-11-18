// NC-1 slice 12 part (j), KO-1 and KO-2: a plan's goal score, a key result's progress words, and adoption checked
// first.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  adoptionBody,
  adoptionRefusalKey,
  emptyAdoptionForm,
  okrReadRefusalKey,
  scoredSplit,
  unmeasurableKey,
} from "../../src/lib/okr-reads-view";

describe("a plan's goal score", () => {
  it("leaves an unmeasured objective out, never as a zero", () => {
    const o = [
      { id: "a", measured: true },
      { id: "b", measured: false },
    ];
    expect(scoredSplit(o)).toEqual({
      counted: [{ id: "a", measured: true }],
      left_out: [{ id: "b", measured: false }],
    });
  });
  it("words why a key result has no progress", () => {
    expect(unmeasurableKey("no_range")).toBe("p.planokr.unmeasurable_no_range");
    expect(unmeasurableKey("no_tolerance")).toBe(
      "p.planokr.unmeasurable_no_tolerance",
    );
    expect(unmeasurableKey(null)).toBe("p.planokr.unmeasurable_other");
    expect(okrReadRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.planokr.refused_gone",
    );
    expect(okrReadRefusalKey(new ApiError(500, "service text"))).toBeNull();
  });
});

describe("adoption", () => {
  it("sends the four figures, the definition trimmed", () => {
    expect(
      adoptionBody({
        active: 120,
        target: 400,
        windowDays: 30,
        definition: " signed in this month ",
      }),
    ).toEqual({
      body: {
        active_users: 120,
        target_users: 400,
        window_days: 30,
        definition: "signed in this month",
      },
    });
    expect(
      adoptionBody({ active: 0, target: 1, windowDays: 1, definition: "x" }),
    ).toHaveProperty("body");
  });
  it("says every problem at once", () => {
    expect(adoptionBody(emptyAdoptionForm())).toEqual({
      problems: [
        "p.adoption.problem_active",
        "p.adoption.problem_target",
        "p.adoption.problem_definition",
      ],
    });
    expect(
      adoptionBody({ active: -1, target: 0, windowDays: 0, definition: " " }),
    ).toEqual({
      problems: [
        "p.adoption.problem_active",
        "p.adoption.problem_target",
        "p.adoption.problem_window",
        "p.adoption.problem_definition",
      ],
    });
    expect(
      adoptionBody({ active: 1.5, target: 2, windowDays: 30, definition: "x" }),
    ).toEqual({ problems: ["p.adoption.problem_active"] });
  });
  it("is a sentence by status", () => {
    const err = (s: number) => new ApiError(s, "service text");
    expect(adoptionRefusalKey(err(422))).toBe("p.adoption.refused");
    expect(adoptionRefusalKey(err(404))).toBe("p.adoption.refused_gone");
    expect(adoptionRefusalKey(err(403))).toBe("p.adoption.refused_forbidden");
    expect(adoptionRefusalKey(err(500))).toBeNull();
  });
});
