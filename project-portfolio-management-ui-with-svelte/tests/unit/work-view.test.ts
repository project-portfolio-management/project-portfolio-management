import { describe, expect, it } from "vitest";
import {
  assignmentBody,
  capacityText,
  customWeights,
  hoursText,
  minorFromMajor,
  minutesFromHours,
  resultText,
} from "../../src/lib/work-view";

const form = (o: object) =>
  ({
    rule: "fixed_units",
    changing: "work",
    duration: "",
    units: "",
    hours: "",
    first: false,
    ...o,
  }) as never;

describe("units of measure", () => {
  it("shows minutes as hours and reads hours back strictly", () => {
    expect(hoursText(90)).toBe("1.5 h");
    expect(hoursText(480)).toBe("8 h");
    expect(minutesFromHours("1.5")).toBe(90);
    expect(minutesFromHours(8)).toBe(480);
    for (const bad of ["", "0", "-1", "abc", null, undefined])
      expect(minutesFromHours(bad as never)).toBeNull();
  });
  it("reads money strictly and never turns bad text into zero", () => {
    expect(minorFromMajor("12.34")).toBe(1234);
    expect(minorFromMajor("0")).toBeNull();
    expect(minorFromMajor("0", true)).toBe(0);
    expect(minorFromMajor("-1")).toBeNull();
    expect(minorFromMajor("")).toBeNull();
    expect(minorFromMajor("abc")).toBeNull();
  });
});

describe("assignmentBody", () => {
  it("changing the work sends only the work and the rule", () => {
    expect(assignmentBody(form({ hours: "16" }))).toEqual({
      rule: "fixed_units",
      work_minutes: 960,
    });
  });
  it("changing duration or units sends only that one", () => {
    expect(
      assignmentBody(form({ changing: "duration", duration: "5" })),
    ).toEqual({ rule: "fixed_units", duration_days: 5 });
    expect(
      assignmentBody(
        form({ changing: "units", units: "50", rule: "fixed_work" }),
      ),
    ).toEqual({ rule: "fixed_work", units_percent: 50 });
  });
  it("a first assignment sends duration and units; work is derived", () => {
    expect(
      assignmentBody(
        form({ first: true, changing: "units", duration: "4", units: "100" }),
      ),
    ).toEqual({
      rule: "fixed_units",
      duration_days: 4,
      units_percent: 100,
    });
  });
  it("rejects values the server would", () => {
    for (const f of [
      form({ changing: "duration", duration: "0" }),
      form({ changing: "duration", duration: "2.5" }),
      form({ changing: "units", units: "1001" }),
      form({ changing: "units", units: "" }),
      form({ hours: "0" }),
    ])
      expect("error" in (assignmentBody(f) as object)).toBe(true);
  });
});

describe("words", () => {
  it("says what the rule did, and that rounding happened", () => {
    const r = {
      duration_days: 5,
      units_percent: 50,
      work_minutes: 1200,
      derived: "work",
      exact: true,
      capacity: { checked: false },
    } as never;
    expect(resultText(r)).toBe(
      "5 working days at 50% is 20 h. The work was worked out.",
    );
    expect(resultText({ ...(r as object), exact: false } as never)).toMatch(
      /Rounded/,
    );
  });
  it("reports capacity plainly", () => {
    expect(
      capacityText({
        checked: true,
        over_capacity: true,
        days_over: 1,
        from: "a",
        to: "b",
      }),
    ).toMatch(/Over capacity on 1 day /);
    expect(capacityText({ checked: true, over_capacity: false })).toMatch(
      /Within/,
    );
    expect(capacityText({ checked: false, reason: "unscheduled" })).toMatch(
      /no dates/,
    );
  });
  it("checks custom percentages", () => {
    expect(customWeights("10, 20, 70")).toEqual([10, 20, 70]);
    expect(customWeights("40 50")).toEqual({
      error: "The percentages add up to 90, not 100.",
    });
    expect("error" in (customWeights("a,b") as object)).toBe(true);
    expect("error" in (customWeights("") as object)).toBe(true);
  });
});
