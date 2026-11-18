// NC-1 slice 12 part (i): the working day and week, and a person's time away, checked first.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  emptyNonWorkingForm,
  emptyWorkingTimeForm,
  nonWorkingBody,
  workingTimeBody,
  workingTimeRefusalKey,
} from "../../src/lib/working-time-view";

const PERSON = "person:0b8a7c1e-1111-4222-8333-444455556666";

describe("the working day and week", () => {
  it("is this plan's by default, in minutes", () => {
    expect(workingTimeBody(emptyWorkingTimeForm(), "p1")).toEqual({
      body: {
        scope_ref: "plan:p1",
        minutes_per_day: 450,
        working_days_per_week: 5,
      },
    });
  });
  it("leaves out the scope for the organization's default", () => {
    expect(
      workingTimeBody(
        { scope: "default", hours: 8, minutes: null, days: 4 },
        "p1",
      ),
    ).toEqual({
      body: { minutes_per_day: 480, working_days_per_week: 4 },
    });
  });
  it("says every problem at once", () => {
    expect(
      workingTimeBody({ scope: "plan", hours: 25, minutes: 60, days: 8 }, "p1"),
    ).toEqual({
      problems: [
        "p.workingtime.problem_hours",
        "p.workingtime.problem_minutes",
        "p.workingtime.problem_week",
      ],
    });
    expect(
      workingTimeBody(
        { scope: "plan", hours: 0, minutes: 0, days: null },
        "p1",
      ),
    ).toEqual({
      problems: ["p.workingtime.problem_day", "p.workingtime.problem_week"],
    });
    expect(
      workingTimeBody({ scope: "plan", hours: 24, minutes: 1, days: 5 }, "p1"),
    ).toEqual({
      problems: ["p.workingtime.problem_day"],
    });
    expect(
      workingTimeBody({ scope: "plan", hours: 24, minutes: 0, days: 7 }, "p1"),
    ).toHaveProperty("body");
  });
});

describe("time away", () => {
  const ok = {
    ...emptyNonWorkingForm(),
    person: ` ${PERSON} `,
    startsOn: "2026-12-21",
    endsOn: "2026-12-24",
  };
  it("trims the person and leaves out a blank note", () => {
    expect(nonWorkingBody({ ...ok, note: "  " })).toEqual({
      body: {
        person_ref: PERSON,
        starts_on: "2026-12-21",
        ends_on: "2026-12-24",
        kind: "leave",
      },
    });
    expect(
      nonWorkingBody({ ...ok, kind: "study_leave", note: " Course " }),
    ).toEqual({
      body: {
        person_ref: PERSON,
        starts_on: "2026-12-21",
        ends_on: "2026-12-24",
        kind: "study_leave",
        note: "Course",
      },
    });
  });
  it("says every problem at once", () => {
    expect(nonWorkingBody(emptyNonWorkingForm())).toEqual({
      problems: [
        "p.workingtime.problem_person",
        "p.workingtime.problem_starts",
        "p.workingtime.problem_ends",
      ],
    });
    expect(
      nonWorkingBody({ ...ok, person: "ada", endsOn: "2026-12-20" }),
    ).toEqual({
      problems: ["p.workingtime.problem_person", "p.workingtime.problem_order"],
    });
    expect(nonWorkingBody({ ...ok, startsOn: "2026-02-30" })).toEqual({
      problems: ["p.workingtime.problem_starts"],
    });
    expect(
      nonWorkingBody({ ...ok, endsOn: "2026-12-21", startsOn: "2026-12-21" }),
    ).toHaveProperty("body");
  });
});

describe("a refusal", () => {
  const e = (s: number) =>
    new ApiError(s, "service text", { code: "conflict" });
  it("is the catalogue's sentence by action and status", () => {
    expect(workingTimeRefusalKey("working_time", e(409))).toBe(
      "p.workingtime.refused_exists",
    );
    expect(workingTimeRefusalKey("working_time", e(422))).toBe(
      "p.workingtime.refused_working",
    );
    expect(workingTimeRefusalKey("non_working", e(422))).toBe(
      "p.workingtime.refused_non_working",
    );
    expect(workingTimeRefusalKey("non_working", e(403))).toBe(
      "p.workingtime.refused_forbidden",
    );
    expect(workingTimeRefusalKey("non_working", e(409))).toBeNull();
  });
});
