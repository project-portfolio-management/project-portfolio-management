import { describe, it, expect } from "vitest";
import { floatText } from "../../src/lib/my-work-view";

describe("floatText (SC-10)", () => {
  it("shows nothing for a task with no schedule data, never 0 days", () => {
    expect(floatText(null)).toBeNull();
    expect(floatText(undefined)).toBeNull();
    expect(
      floatText({ total_float: Number.NaN, critical: false, late: false }),
    ).toBeNull();
  });

  it("says a task with no float is on the critical path", () => {
    expect(floatText({ total_float: 0, critical: true, late: false })).toEqual({
      kind: "critical",
      text: "On the critical path",
    });
  });

  it("says N days of float, singular for one", () => {
    expect(floatText({ total_float: 3, critical: false, late: false })).toEqual(
      {
        kind: "float",
        text: "3 days of float",
      },
    );
    expect(
      floatText({ total_float: 1, critical: false, late: false })?.text,
    ).toBe("1 day of float");
  });

  it("puts negative float in a plain sentence", () => {
    const r = floatText({ total_float: -3, critical: true, late: true });
    expect(r?.kind).toBe("late");
    expect(r?.text).toBe(
      "This is already late for the plan (3 working days behind).",
    );
    expect(
      floatText({ total_float: -1, critical: true, late: true })?.text,
    ).toContain("1 working day behind");
  });

  it("never shows a bare number: every kind is a sentence", () => {
    for (const f of [-5, -1, 0, 1, 12]) {
      const r = floatText({ total_float: f, critical: f <= 0, late: f < 0 });
      expect(r?.text).toMatch(/[a-z]{3,}/);
    }
  });
});
