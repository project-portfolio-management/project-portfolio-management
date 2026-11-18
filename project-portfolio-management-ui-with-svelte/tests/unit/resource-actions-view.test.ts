// NC-1 slice 12 part (j), SC-7 and PJ-3: placeholders, hours, skill gaps, the rate in force and effort against
// capacity, checked first, the defaults not sent.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  figuresOf,
  hoursOf,
  hoursQuery,
  hoursRefusalKey,
  isOverCapacity,
  placeholderRefusalKey,
  rateQuery,
  rateRefusalKey,
  resolveBody,
  skillCounts,
  unavailableKey,
  utilizationQuery,
  utilizationRefusalKey,
} from "../../src/lib/resource-actions-view";

const PERSON = "person:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";
const err = (status: number, code = "unprocessable") =>
  new ApiError(status, "service text", { code, detail: "service text" });

describe("a placeholder", () => {
  it("is given a person by reference, confirm only after agreeing to go over", () => {
    expect(resolveBody(` ${PERSON} `, false)).toEqual({
      body: { person_ref: PERSON },
    });
    expect(resolveBody(PERSON, true)).toEqual({
      body: { person_ref: PERSON, confirm: true },
    });
    expect(resolveBody("Ada Lovelace", false)).toEqual({
      problems: ["p.placeholder.problem_person"],
    });
  });
  it("tells going over capacity from a person named already", () => {
    expect(isOverCapacity(err(409, "over_capacity"))).toBe(true);
    expect(isOverCapacity(err(409, "conflict"))).toBe(false);
    expect(placeholderRefusalKey("resolve", err(409, "conflict"))).toBe(
      "p.placeholder.refused_named",
    );
    expect(placeholderRefusalKey("resolve", err(403, "forbidden"))).toBe(
      "p.placeholder.refused_forbidden",
    );
    expect(placeholderRefusalKey("resolve", err(422))).toBe(
      "p.placeholder.refused_person",
    );
    expect(placeholderRefusalKey("remove", err(404, "not_found"))).toBe(
      "p.placeholder.refused_gone",
    );
    expect(placeholderRefusalKey("remove", err(409))).toBeNull();
  });
});

describe("hours", () => {
  it("leaves a blank day out and checks the window first", () => {
    expect(hoursQuery("", "")).toEqual({ query: {} });
    expect(hoursQuery("2026-11-01", "2026-11-28")).toEqual({
      query: { from: "2026-11-01", to: "2026-11-28" },
    });
    expect(hoursQuery("2026-11-28", "2026-11-01")).toEqual({
      problems: ["p.hours.problem_order"],
    });
    expect(hoursQuery("2016-01-01", "2026-12-31")).toEqual({
      problems: ["p.hours.problem_span"],
    });
    expect(hoursQuery("2026-13-01", "nope")).toEqual({
      problems: ["p.hours.problem_from", "p.hours.problem_to"],
    });
    expect(hoursOf(90)).toBe(1.5);
    expect(hoursRefusalKey(err(422))).toBe("p.hours.refused");
  });
});

describe("skill gaps", () => {
  it("counts each standing", () => {
    expect(
      skillCounts([
        {
          allocation_pid: "a",
          person_ref: PERSON,
          tag: "rust",
          status: "covered",
        },
        {
          allocation_pid: "a",
          person_ref: PERSON,
          tag: "sql",
          status: "missing",
        },
        {
          allocation_pid: "b",
          person_ref: PERSON,
          tag: "go",
          status: "unknown",
          reason: "service text",
        },
      ]),
    ).toEqual({ covered: 1, missing: 1, unknown: 1 });
  });
});

describe("the rate in force", () => {
  it("leaves the standard table out and checks first", () => {
    expect(rateQuery(PERSON, "2026-11-18", "standard")).toEqual({
      query: { person_ref: PERSON, date: "2026-11-18" },
    });
    expect(rateQuery(PERSON, "2026-11-18", " contractors ")).toEqual({
      query: { person_ref: PERSON, date: "2026-11-18", table: "contractors" },
    });
    expect(rateQuery(" ", "18/11/2026", "")).toEqual({
      problems: ["p.rate.problem_person", "p.rate.problem_date"],
    });
  });
  it("says a 404 is a rate kept from the reader", () => {
    expect(rateRefusalKey(err(404, "not_found"))).toBe("p.rate.refused_hidden");
    expect(rateRefusalKey(err(400))).toBe("p.rate.refused_query");
  });
});

describe("effort against capacity", () => {
  it("leaves the team and 28 days out, and refuses a window the service would quietly replace", () => {
    expect(utilizationQuery("team", 28, "")).toEqual({ query: {} });
    expect(utilizationQuery("person", 90, "p1")).toEqual({
      query: { by: "person", window_days: 90, plan_pid: "p1" },
    });
    expect(utilizationQuery("team", null, "")).toEqual({ query: {} });
    for (const bad of [0, 367, 2.5])
      expect(utilizationQuery("team", bad, "")).toEqual({
        problems: ["p.util.problem_window"],
      });
  });
  it("words why a figure is absent, and reads one figure or many", () => {
    expect(unavailableKey("small_group")).toBe(
      "p.util.unavailable_small_group",
    );
    expect(unavailableKey("all_non_working")).toBe(
      "p.util.unavailable_all_away",
    );
    expect(unavailableKey("new_reason")).toBe("p.util.unavailable_other");
    const f = {
      actor_ref: "team",
      basis_points: 8000,
      unavailable: null,
      effort_minutes: 1,
      available_minutes: 1,
      declared_minutes: 1,
      non_working_minutes: 0,
      asserted: true,
      at_or_over_capacity: false,
    };
    expect(figuresOf(f)).toEqual([f]);
    expect(figuresOf([f, f])).toHaveLength(2);
    expect(utilizationRefusalKey(err(422))).toBe("p.util.refused");
  });
});
