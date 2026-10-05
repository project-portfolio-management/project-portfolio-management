import { describe, it, expect } from "vitest";
import {
  STATE_TEXT,
  STATUS_ORDER,
  STATUS_SYMBOL,
  STATUS_TEXT,
  achievedText,
  completionNeedsReason,
  forecastText,
  openTasksText,
  slipText,
  statusParts,
  summaryText,
} from "../../src/lib/milestone-view";
import type { MilestoneSchedule, MilestoneView } from "../../src/lib/api/ppm";

const sched = (over: Partial<MilestoneSchedule> = {}): MilestoneSchedule => ({
  state: "on_track",
  forecast: "2026-10-12",
  slack_days: 8,
  reason: null,
  partial: false,
  unforecast_tasks: [],
  ...over,
});

describe("milestone words", () => {
  it("gives every status a word and a distinct shape, late first", () => {
    expect([...STATUS_ORDER]).toEqual([
      "late",
      "due_soon",
      "upcoming",
      "achieved",
    ]);
    expect(new Set(Object.values(STATUS_SYMBOL)).size).toBe(4);
    for (const s of STATUS_ORDER)
      expect(STATUS_TEXT[s].length).toBeGreaterThan(3);
  });

  it("states the forecast against the due date", () => {
    expect(forecastText(sched(), "2026-10-20")).toBe(
      "Forecast to finish 2026-10-12, 8 days before it is due (2026-10-20).",
    );
    expect(
      forecastText(sched({ slack_days: 0, state: "at_risk" }), "2026-10-12"),
    ).toBe("Forecast to finish on the due date, 2026-10-12.");
    expect(
      forecastText(
        sched({ slack_days: -1, state: "forecast_late" }),
        "2026-10-11",
      ),
    ).toBe(
      "Forecast to finish 2026-10-12, 1 day after it is due (2026-10-11).",
    );
    expect(forecastText(sched({ partial: true }), "2026-10-20")).toContain(
      "may be early",
    );
  });

  it("never calls a milestone with no forecast on track", () => {
    const none = sched({
      state: "no_forecast",
      forecast: null,
      slack_days: null,
      reason: "no_linked_tasks",
    });
    expect(forecastText(none, "2026-10-20")).toContain(
      "nothing to forecast from",
    );
    expect(STATE_TEXT.no_forecast).not.toMatch(/on track/i);
    expect(forecastText({ ...none, reason: "cycle" }, "x")).toContain("cycle");
    expect(
      forecastText({ ...none, reason: "no_scheduled_tasks" }, "x"),
    ).toContain("no dates");
    expect(forecastText({ ...none, reason: "mystery" }, "x")).toBe(
      "There is no forecast.",
    );
  });

  it("says nothing about a forecast once achieved", () => {
    expect(
      forecastText(
        sched({ state: null, forecast: null, reason: "achieved" }),
        "x",
      ),
    ).toBeNull();
  });

  it("describes slip from the original date, or nothing if it never moved", () => {
    expect(
      slipText({
        original_due: "2026-11-01",
        reschedules: 0,
        later_moves: 0,
        net_days: 0,
      }),
    ).toBeNull();
    expect(
      slipText({
        original_due: "2026-11-01",
        reschedules: 3,
        later_moves: 2,
        net_days: 19,
      }),
    ).toBe(
      "Originally due 2026-11-01; rescheduled 3 times, 2 of them later; now 19 days later than the original 2026-11-01.",
    );
    expect(
      slipText({
        original_due: "2026-11-20",
        reschedules: 1,
        later_moves: 0,
        net_days: -10,
      }),
    ).toContain("10 days earlier");
    expect(
      slipText({
        original_due: "2026-11-01",
        reschedules: 2,
        later_moves: 1,
        net_days: 0,
      }),
    ).toContain("back on its original date");
    expect(
      slipText({
        original_due: "2026-11-01",
        reschedules: 1,
        later_moves: 1,
        net_days: 1,
      }),
    ).toContain("1 day later");
  });

  it("says how a milestone was achieved", () => {
    expect(
      achievedText({
        achieved: false,
        achieved_by: null,
        achieved_on: null,
        done_reason: null,
      }),
    ).toBeNull();
    expect(
      achievedText({
        achieved: true,
        achieved_by: "tasks",
        achieved_on: "2026-10-14",
        done_reason: null,
      }),
    ).toBe("Achieved on 2026-10-14: every task it waits for is done.");
    expect(
      achievedText({
        achieved: true,
        achieved_by: "manual",
        achieved_on: "2026-10-14",
        done_reason: "scope cut",
      }),
    ).toBe("Marked done on 2026-10-14. Reason: scope cut");
  });

  it("knows when completing needs a reason", () => {
    expect(
      completionNeedsReason({
        completion_mode: "linked",
        open_tasks: 2,
        achieved: false,
      }),
    ).toBe(true);
    expect(
      completionNeedsReason({
        completion_mode: "linked",
        open_tasks: 0,
        achieved: false,
      }),
    ).toBe(false);
    expect(
      completionNeedsReason({
        completion_mode: "manual",
        open_tasks: 3,
        achieved: false,
      }),
    ).toBe(false);
    expect(
      completionNeedsReason({
        completion_mode: "linked",
        open_tasks: 2,
        achieved: true,
      }),
    ).toBe(false);
    expect(
      openTasksText({
        completion_mode: "linked",
        linked_tasks: 3,
        open_tasks: 1,
        achieved: false,
      }),
    ).toBe("1 of 3 tasks still open");
    expect(
      openTasksText({
        completion_mode: "manual",
        linked_tasks: 0,
        open_tasks: 0,
        achieved: false,
      }),
    ).toBeNull();
  });

  it("summarises a row and the counts", () => {
    const m = {
      name: "Beta",
      status: "due_soon",
      due: "2026-10-20",
      schedule: sched({ state: "at_risk" }),
    } as MilestoneView;
    expect(summaryText(m)).toBe("Beta: due soon, due 2026-10-20, at risk");
    expect(
      statusParts({ late: 1, due_soon: 2, upcoming: 3, achieved: 4 }).map(
        (p) => p.status,
      ),
    ).toEqual(["late", "due_soon", "upcoming", "achieved"]);
  });
});
