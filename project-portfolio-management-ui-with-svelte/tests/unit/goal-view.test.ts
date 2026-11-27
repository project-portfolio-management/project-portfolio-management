// The goals page's rules (FR-12, CF-2; NC-1 slice 12 part n).

import { describe, expect, it } from "vitest";
import {
  EMPTY_FORM,
  TITLE_MAX,
  formOf,
  goalBody,
  refusalKey,
  sameGoal,
  statusLabel,
} from "../../src/lib/goal-view";

describe("goalBody", () => {
  it("trims, and sends a blank field as null", () => {
    expect(
      goalBody({ ...EMPTY_FORM, title: "  Ship v1 ", description: "  " }),
    ).toEqual({
      body: {
        title: "Ship v1",
        description: null,
        target_date: null,
        status: null,
      },
    });
  });
  it("refuses a blank or an over-long title", () => {
    expect(goalBody({ ...EMPTY_FORM, title: "   " })).toEqual({
      problem: "p.goals.title_required",
    });
    expect(
      goalBody({ ...EMPTY_FORM, title: "x".repeat(TITLE_MAX) }),
    ).toHaveProperty("body");
    expect(
      goalBody({ ...EMPTY_FORM, title: "x".repeat(TITLE_MAX + 1) }),
    ).toEqual({
      problem: "p.goals.title_too_long",
    });
  });
  it("counts characters, not UTF-16 units", () => {
    expect(
      goalBody({ ...EMPTY_FORM, title: "😀".repeat(TITLE_MAX) }),
    ).toHaveProperty("body");
  });
  it("takes only a real day as the target date", () => {
    expect(
      goalBody({ ...EMPTY_FORM, title: "a", target_date: "2027-02-28" }),
    ).toHaveProperty("body.target_date", "2027-02-28");
    for (const bad of ["2027-02-30", "2027-13-01", "27-02-01", "soon"])
      expect(goalBody({ ...EMPTY_FORM, title: "a", target_date: bad })).toEqual(
        {
          problem: "p.goals.date_bad",
        },
      );
  });
  it("sends a named status as itself and a custom one by its name", () => {
    expect(
      goalBody({ ...EMPTY_FORM, title: "a", status: "Achieved" }),
    ).toHaveProperty("body.status", "Achieved");
    expect(
      goalBody({
        ...EMPTY_FORM,
        title: "a",
        status: "Custom",
        custom: " Paused ",
      }),
    ).toHaveProperty("body.status", { Custom: "Paused" });
    expect(
      goalBody({ ...EMPTY_FORM, title: "a", status: "Custom", custom: " " }),
    ).toEqual({
      problem: "p.goals.custom_required",
    });
  });
});

describe("formOf and sameGoal", () => {
  it("round-trips a stored goal through its form", () => {
    for (const g of [
      { title: "a" },
      {
        title: "a",
        description: "d",
        target_date: "2027-01-01",
        status: "Missed" as const,
      },
      { title: "a", status: { Custom: "Paused" } },
    ]) {
      const r = goalBody(formOf(g));
      expect("body" in r && sameGoal(r.body, g)).toBe(true);
    }
  });
  it("tells a change apart", () => {
    expect(sameGoal({ title: "a" }, { title: "a", description: null })).toBe(
      true,
    );
    expect(sameGoal({ title: "a" }, { title: "b" })).toBe(false);
    expect(
      sameGoal(
        { title: "a", status: { Custom: "x" } },
        { title: "a", status: { Custom: "y" } },
      ),
    ).toBe(false);
    expect(sameGoal({ title: "a", status: "Achieved" }, { title: "a" })).toBe(
      false,
    );
  });
});

describe("statusLabel and refusalKey", () => {
  it("reads a status", () => {
    expect(statusLabel(null)).toBeNull();
    expect(statusLabel("InProgress")).toEqual({ key: "p.goals.st.InProgress" });
    expect(statusLabel({ Custom: "Paused" })).toEqual({ custom: "Paused" });
  });
  it("says a changed list and a missing goal in their own words", () => {
    expect(refusalKey("save", 412)).toBe("p.goals.changed");
    expect(refusalKey("remove", 412)).toBe("p.goals.changed");
    expect(refusalKey("remove", 404)).toBe("p.goals.gone");
    expect(refusalKey("save", 403)).toBe("p.goals.could_not_save");
    expect(refusalKey("remove", 500)).toBe("p.goals.could_not_remove");
  });
});
