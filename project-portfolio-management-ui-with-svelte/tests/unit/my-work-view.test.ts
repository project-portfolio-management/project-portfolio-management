import { describe, it, expect } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  feedAddress,
  floatText,
  isWatchKind,
  meRefusalKey,
  shortRef,
  watchKindKey,
  workProfileQuery,
} from "../../src/lib/my-work-view";

describe("floatText (SC-10)", () => {
  it("shows nothing for a task with no schedule data, never 0 business days", () => {
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

  it("says N business days of float, singular for one", () => {
    expect(floatText({ total_float: 3, critical: false, late: false })).toEqual(
      {
        kind: "float",
        text: "3 business days of float",
      },
    );
    expect(
      floatText({ total_float: 1, critical: false, late: false })?.text,
    ).toBe("1 business day of float");
  });

  it("puts negative float in a plain sentence", () => {
    const r = floatText({ total_float: -3, critical: true, late: true });
    expect(r?.kind).toBe("late");
    expect(r?.text).toBe(
      "This is already late for the plan (3 business days behind).",
    );
    expect(
      floatText({ total_float: -1, critical: true, late: true })?.text,
    ).toContain("1 business day behind");
  });

  it("never shows a bare number: every kind is a sentence", () => {
    for (const f of [-5, -1, 0, 1, 12]) {
      const r = floatText({ total_float: f, critical: f <= 0, late: f < 0 });
      expect(r?.text).toMatch(/[a-z]{3,}/);
    }
  });
});

// NC-1 slice 12 part (h): the calendar feed (LT-7), what I follow (RM-3) and the work over time (PJ-5).
describe("the calendar feed's address", () => {
  it("is the service's feed address through this site's proxy, and nothing for anything else", () => {
    expect(
      feedAddress(
        "https://ppm.example/api/proxy",
        "/api/feeds/calendar.ics?token=abc",
      ),
    ).toBe("https://ppm.example/api/proxy/api/feeds/calendar.ics?token=abc");
    expect(
      feedAddress("https://ppm.example/api/proxy", "https://elsewhere/x"),
    ).toBeNull();
    expect(feedAddress("https://ppm.example/api/proxy", "")).toBeNull();
  });
});

describe("what I follow", () => {
  it("names each kind the service lists, and one it does not know as something else", () => {
    expect(watchKindKey("task")).toBe("p.watching.kind_task");
    expect(watchKindKey("document_version")).toBe(
      "p.watching.kind_document_version",
    );
    expect(watchKindKey("comment")).toBe("p.watching.kind_other");
    expect(isWatchKind("plan")).toBe(true);
    expect(isWatchKind("comment")).toBe(false);
    expect(shortRef("6f1c2a3b-0000-4000-8000-000000000009")).toBe("6f1c2a3b");
  });
});

describe("the work over time's query", () => {
  it("leaves weeks and empty dates out", () => {
    expect(workProfileQuery({ period: "", from: "", to: "" })).toEqual({
      query: {},
    });
    expect(
      workProfileQuery({ period: "month", from: "2026-01-01", to: "" }),
    ).toEqual({ query: { period: "month", from: "2026-01-01" } });
  });

  it("checks the dates, their order, ten years and 120 periods first", () => {
    expect(
      workProfileQuery({ period: "", from: "2026-02-30", to: "x" }),
    ).toEqual({
      problems: ["p.workprofile.problem_from", "p.workprofile.problem_to"],
    });
    expect(
      workProfileQuery({ period: "", from: "2026-12-01", to: "2026-11-01" }),
    ).toEqual({ problems: ["p.workprofile.problem_order"] });
    expect(
      workProfileQuery({
        period: "month",
        from: "2020-01-01",
        to: "2031-01-01",
      }),
    ).toEqual({ problems: ["p.workprofile.problem_too_long"] });
    // 120 days is the most a daily view holds; 121 is one too many.
    expect(
      "query" in
        workProfileQuery({
          period: "day",
          from: "2026-01-01",
          to: "2026-04-30",
        }),
    ).toBe(true);
    expect(
      workProfileQuery({ period: "day", from: "2026-01-01", to: "2026-05-01" }),
    ).toEqual({ problems: ["p.workprofile.problem_periods"] });
    // Weeks are Monday to Sunday: a window from a Sunday touches one more week.
    expect(
      "query" in
        workProfileQuery({ period: "", from: "2026-01-05", to: "2028-04-23" }),
    ).toBe(true);
    expect(
      workProfileQuery({ period: "", from: "2026-01-04", to: "2028-04-23" }),
    ).toEqual({ problems: ["p.workprofile.problem_periods"] });
  });
});

describe("refusals on my own work", () => {
  const err = (status: number) => new ApiError(status, "service text");
  it("names each action's refusal by status", () => {
    expect(meRefusalKey("issue_feed", err(403))).toBe(
      "p.feed.refused_forbidden",
    );
    expect(meRefusalKey("revoke_feed", err(403))).toBe(
      "p.feed.refused_forbidden",
    );
    expect(meRefusalKey("unwatch", err(422))).toBe("p.watching.refused");
    expect(meRefusalKey("work_profile", err(422))).toBe(
      "p.workprofile.refused",
    );
    expect(meRefusalKey("issue_feed", err(401))).toBeNull();
    expect(meRefusalKey("work_profile", new Error("x"))).toBeNull();
  });
});
