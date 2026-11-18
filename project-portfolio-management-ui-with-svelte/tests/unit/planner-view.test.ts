// NC-1 slice 12 part (h), PL-4: the day's plan. A day moved back and forward; a typed time on the shown day read in the
// reader's IANA zone and sent as UTC, across both clock changes (a skipped time refused, a repeated one the first); a
// slot's rules checked before anything is sent, every problem at once; the clashes a slot is in; and each refusal.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  SLOT_MAX_MINUTES,
  clashCount,
  isDay,
  localToUtc,
  plannerRefusalKey,
  shiftDay,
  slotBody,
} from "../../src/lib/planner-view";

describe("the shown day", () => {
  it("is a real calendar date, moved a day at a time across months and years", () => {
    expect(isDay("2026-11-18")).toBe(true);
    expect(isDay("2026-02-30")).toBe(false);
    expect(isDay("18/11/2026")).toBe(false);
    expect(shiftDay("2026-11-30", 1)).toBe("2026-12-01");
    expect(shiftDay("2026-01-01", -1)).toBe("2025-12-31");
    expect(shiftDay("not a day", 1)).toBeNull();
  });
});

describe("a time of day in the reader's zone, as UTC", () => {
  it("reads the time in the zone, not in the browser's or in UTC", () => {
    expect(localToUtc("2026-11-18", "09:00", "Europe/London")).toBe(
      "2026-11-18T09:00:00Z",
    );
    expect(localToUtc("2026-07-01", "09:00", "Europe/London")).toBe(
      "2026-07-01T08:00:00Z",
    );
    expect(localToUtc("2026-11-18", "09:30", "America/Los_Angeles")).toBe(
      "2026-11-18T17:30:00Z",
    );
    expect(localToUtc("2026-11-18", "08:00", "Pacific/Auckland")).toBe(
      "2026-11-17T19:00:00Z",
    );
    expect(localToUtc("2026-11-18", "09:15", "Asia/Kolkata")).toBe(
      "2026-11-18T03:45:00Z",
    );
  });

  it("refuses a time the clocks skip, and takes the first of a time that happens twice", () => {
    // London springs forward at 01:00 on 29 March 2026 and falls back at 02:00 on 25 October 2026.
    expect(localToUtc("2026-03-29", "01:30", "Europe/London")).toBe("skipped");
    expect(localToUtc("2026-10-25", "01:30", "Europe/London")).toBe(
      "2026-10-25T00:30:00Z",
    );
  });

  it("is null for a time or day that is not one, or an unknown zone", () => {
    expect(localToUtc("2026-11-18", "25:00", "Europe/London")).toBeNull();
    expect(localToUtc("2026-11-18", "", "Europe/London")).toBeNull();
    expect(localToUtc("2026-13-01", "09:00", "Europe/London")).toBeNull();
    expect(localToUtc("2026-11-18", "09:00", "Not/AZone")).toBeNull();
  });
});

describe("a slot's body", () => {
  const zone = "Europe/London";
  it("sends the task and both instants in UTC", () => {
    expect(
      slotBody(
        { taskPid: "t1", start: "09:00", end: "10:30" },
        "2026-11-18",
        zone,
      ),
    ).toEqual({
      body: {
        task_pid: "t1",
        starts_at: "2026-11-18T09:00:00Z",
        ends_at: "2026-11-18T10:30:00Z",
      },
    });
  });

  it("says every problem at once, and nothing is sent", () => {
    expect(
      slotBody({ taskPid: "", start: "", end: "" }, "2026-11-18", zone),
    ).toEqual({ problems: ["task_needed", "start_needed", "end_needed"] });
    expect(
      slotBody(
        { taskPid: "t1", start: "10:00", end: "10:00" },
        "2026-11-18",
        zone,
      ),
    ).toEqual({ problems: ["end_not_after_start"] });
    expect(
      slotBody(
        { taskPid: "t1", start: "07:00", end: "19:01" },
        "2026-11-18",
        zone,
      ),
    ).toEqual({ problems: ["too_long"] });
    expect(
      slotBody(
        { taskPid: "t1", start: "01:15", end: "03:00" },
        "2026-03-29",
        zone,
      ),
    ).toEqual({ problems: ["start_skipped"] });
  });

  it("allows exactly 12 hours, counted in real time across a clock change", () => {
    expect(SLOT_MAX_MINUTES).toBe(720);
    expect(
      "body" in
        slotBody(
          { taskPid: "t1", start: "07:00", end: "19:00" },
          "2026-11-18",
          zone,
        ),
    ).toBe(true);
    // 00:00 to 12:00 on the day the clocks go forward is 11 real hours; on the day they go back, 13.
    expect(
      "body" in
        slotBody(
          { taskPid: "t1", start: "00:00", end: "12:00" },
          "2026-03-29",
          zone,
        ),
    ).toBe(true);
    expect(
      slotBody(
        { taskPid: "t1", start: "00:00", end: "12:00" },
        "2026-10-25",
        zone,
      ),
    ).toEqual({ problems: ["too_long"] });
  });
});

describe("clashes", () => {
  it("counts the other slots in a slot's clash group, and none for a slot in none", () => {
    const groups = [
      ["a", "b", "c"],
      ["d", "e"],
    ];
    expect(clashCount("a", groups)).toBe(2);
    expect(clashCount("e", groups)).toBe(1);
    expect(clashCount("f", groups)).toBe(0);
    expect(clashCount("a", [])).toBe(0);
  });
});

describe("refusals", () => {
  const err = (status: number) =>
    new ApiError(status, "service text", {
      code: "unprocessable",
      detail: "service text",
    });
  it("names each action's refusal by status, and leaves the rest to the general rule", () => {
    expect(plannerRefusalKey("read", err(422))).toBe("p.planner.refused_zone");
    expect(plannerRefusalKey("plan", err(422))).toBe("p.planner.refused_slot");
    expect(plannerRefusalKey("plan", err(404))).toBe("p.planner.refused_task");
    expect(plannerRefusalKey("remove", err(404))).toBe(
      "p.planner.refused_gone",
    );
    expect(plannerRefusalKey("read", err(404))).toBeNull();
    expect(plannerRefusalKey("remove", err(422))).toBeNull();
    expect(plannerRefusalKey("plan", err(500))).toBeNull();
    expect(plannerRefusalKey("plan", new Error("network"))).toBeNull();
  });
});
