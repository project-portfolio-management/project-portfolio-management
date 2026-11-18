// NC-1 slice 12 part (h), LT-11 and RM-10: recording time and the time report. An entry's rules said first (a plan, a
// real date, whole hours and minutes from 1 minute to a day, a week known to be approved refused), every problem at
// once; the defaults and a blank note not sent; the caller's own entries kept from everyone's; the report's dates and
// person checked first and the defaults not sent; a group's key in words; and each refusal by action and status.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { TimeEntry } from "../../src/lib/api/ppm-types";
import { timeReportSearch } from "../../src/lib/api/ppm-types";
import {
  categoryKey,
  emptyEntryForm,
  entryMinutes,
  groupText,
  ownEntries,
  timeEntryBody,
  timeRefusalKey,
  timeReportQuery,
} from "../../src/lib/time-entry-view";

const ME = "person:6f1c2a3b-0000-4000-8000-000000000009";

describe("an entry's time", () => {
  it("is whole hours and minutes, from 1 minute to 1440 in all", () => {
    expect(entryMinutes(1, 30)).toBe(90);
    expect(entryMinutes(null, 45)).toBe(45);
    expect(entryMinutes(24, 0)).toBe(1440);
    expect(entryMinutes(24, 1)).toBeNull();
    expect(entryMinutes(0, 0)).toBeNull();
    expect(entryMinutes(undefined, undefined)).toBeNull();
    expect(entryMinutes(1.5, 0)).toBeNull();
    expect(entryMinutes(0, 60)).toBeNull();
    expect(entryMinutes(-1, 30)).toBeNull();
  });
});

describe("an entry's body", () => {
  const form = {
    ...emptyEntryForm("2026-11-11"),
    planPid: "p1",
    hours: 1,
    minutes: 30,
  };
  it("sends the caller's reference, the date and the minutes, leaving the defaults and a blank note out", () => {
    expect(timeEntryBody(form, ME)).toEqual({
      planPid: "p1",
      body: { actor_ref: ME, spent_on: "2026-11-11", minutes: 90 },
    });
  });

  it("sends a task, a category, billable and a trimmed note when given", () => {
    expect(
      timeEntryBody(
        {
          ...form,
          taskPid: "t1",
          category: "capex",
          billable: true,
          note: "  Design review  ",
        },
        ME,
      ),
    ).toEqual({
      planPid: "p1",
      body: {
        actor_ref: ME,
        spent_on: "2026-11-11",
        minutes: 90,
        task_pid: "t1",
        category: "capex",
        billable: true,
        note: "Design review",
      },
    });
  });

  it("says every problem at once", () => {
    expect(
      timeEntryBody(
        { ...emptyEntryForm("2026-02-30"), hours: 0, minutes: 0 },
        ME,
      ),
    ).toEqual({
      problems: [
        "p.timeentry.problem_plan",
        "p.timeentry.problem_date",
        "p.timeentry.problem_duration",
      ],
    });
  });

  it("refuses a date in a week the page knows is approved", () => {
    expect(timeEntryBody(form, ME, new Set(["2026-W46"]))).toEqual({
      problems: ["p.timeentry.problem_locked"],
    });
    expect("body" in timeEntryBody(form, ME, new Set(["2026-W47"]))).toBe(true);
  });
});

describe("the caller's own entries", () => {
  const entry = (pid: string, actor: string): TimeEntry => ({
    pid,
    plan_pid: "p1",
    task_pid: null,
    actor_ref: actor,
    spent_on: "2026-11-11",
    minutes: 30,
    category: "unclassified",
    billable: false,
    note: null,
    created_at: "2026-11-11T10:00:00Z",
  });
  it("keeps the caller's, whatever the prefix or case, in the service's order", () => {
    const list = [
      entry("e1", ME),
      entry("e2", "person:6f1c2a3b-0000-4000-8000-000000000001"),
      entry("e3", ME.toUpperCase().replace("PERSON", "worker")),
    ];
    expect(ownEntries(list, ME).map((e) => e.pid)).toEqual(["e1", "e3"]);
  });

  it("names a category in words, anything unknown as not classified", () => {
    expect(categoryKey("opex")).toBe("p.timeentry.category_opex");
    expect(categoryKey("other")).toBe("p.timeentry.category_unclassified");
  });
});

describe("the time report's query", () => {
  const base = {
    groupBy: "plan" as const,
    from: "",
    to: "",
    plan: "",
    person: "",
  };
  it("sends only what is given; the dates' defaults are the service's", () => {
    expect(timeReportQuery(base)).toEqual({ query: { groupBy: "plan" } });
    expect(
      timeReportQuery({
        ...base,
        groupBy: "month",
        from: "2026-09-01",
        to: "2026-11-30",
        plan: "p1",
        person: ` ${ME} `,
      }),
    ).toEqual({
      query: {
        groupBy: "month",
        from: "2026-09-01",
        to: "2026-11-30",
        plan: "p1",
        person: ME,
      },
    });
    expect(timeReportSearch({ groupBy: "person" })).toBe("?group_by=person");
    expect(
      timeReportSearch({ groupBy: "plan", from: "2026-11-01" }, true),
    ).toBe("?group_by=plan&from=2026-11-01&format=csv");
  });

  it("says every problem at once", () => {
    expect(
      timeReportQuery({ ...base, from: "2026-11-31", person: "alice" }),
    ).toEqual({
      problems: ["p.timereport.problem_from", "p.timereport.problem_person"],
    });
    expect(
      timeReportQuery({ ...base, from: "2026-12-01", to: "2026-11-01" }),
    ).toEqual({ problems: ["p.timereport.problem_order"] });
  });
});

describe("a group's key in words", () => {
  it("names a plan, a category, billable and a month, and keeps a person's reference", () => {
    expect(groupText("plan", "p1", { p1: "Website replatform" })).toBe(
      "Website replatform",
    );
    expect(groupText("plan", "22222222-2222", {})).toBe(
      "A plan not named here (22222222)",
    );
    expect(groupText("category", "capex", {})).toBe("Capital spending (capex)");
    expect(groupText("billable", "non_billable", {})).toBe("Not billable");
    expect(groupText("billable", "billable", {})).toBe("Billable");
    expect(groupText("month", "2026-11", {})).toBe("November 2026");
    expect(groupText("person", ME, {})).toBe(ME);
  });
});

describe("refusals", () => {
  const err = (status: number) =>
    new ApiError(status, "service text", { code: "conflict" });
  it("names each action's refusal by status", () => {
    expect(timeRefusalKey("record", err(409))).toBe(
      "p.timeentry.refused_locked",
    );
    expect(timeRefusalKey("record", err(422))).toBe(
      "p.timeentry.refused_invalid",
    );
    expect(timeRefusalKey("record", err(404))).toBe("p.timeentry.refused_plan");
    expect(timeRefusalKey("record", err(403))).toBe(
      "p.timeentry.refused_forbidden",
    );
    expect(timeRefusalKey("entries", err(404))).toBe(
      "p.timeentry.refused_plan",
    );
    expect(timeRefusalKey("report", err(422))).toBe("p.timereport.refused");
    expect(timeRefusalKey("report", err(404))).toBeNull();
    expect(timeRefusalKey("record", err(500))).toBeNull();
    expect(timeRefusalKey("record", new Error("network"))).toBeNull();
  });
});
