// NC-1 slice 6 (LT-11): which ISO week a day is in, this week in the reader's time zone, moving between weeks, the
// checks on a typed week and person, the time in words, the steps each status allows, the bodies, and the refusal
// sentences. The rules mirror the service's `src/timesheets.rs` and `src/controllers/timesheets.rs`; the service
// still decides.

import { describe, expect, it } from "vitest";
import { i18n } from "../../src/lib/i18n.svelte";
import {
  canDecide,
  canSubmit,
  decideBody,
  decisionProblem,
  durationText,
  isoWeekOf,
  parseWeek,
  personProblem,
  samePerson,
  shiftWeek,
  statusOf,
  submitBody,
  thisWeek,
  timesheetRefusalKey,
  todayIn,
  weekMonday,
} from "../../src/lib/timesheet-view";

const ALICE = "person:6f1c2a3b-0000-4000-8000-000000000001";

describe("ISO weeks", () => {
  it("names the week a day is in, as the service's week_start reads it", () => {
    // The service's own cases: 2026-W40 starts on 28 September, W41 on 5 October.
    expect(isoWeekOf("2026-09-28")).toBe("2026-W40");
    expect(isoWeekOf("2026-10-04")).toBe("2026-W40");
    expect(isoWeekOf("2026-10-05")).toBe("2026-W41");
    expect(weekMonday("2026-W40")).toBe("2026-09-28");
    expect(weekMonday("2026-W41")).toBe("2026-10-05");
  });

  it("puts days near the new year in the week of their Thursday", () => {
    expect(isoWeekOf("2021-01-03")).toBe("2020-W53");
    expect(isoWeekOf("2024-12-30")).toBe("2025-W01");
    expect(isoWeekOf("2026-12-31")).toBe("2026-W53");
    expect(isoWeekOf("2027-01-03")).toBe("2026-W53");
    expect(isoWeekOf("2027-01-04")).toBe("2027-W01");
    expect(isoWeekOf("2026-02-30")).toBeNull();
  });

  it("reads a typed week, and refuses what the service would", () => {
    expect(parseWeek(" 2026-w46 ")).toBe("2026-W46");
    expect(parseWeek("2026-W5")).toBe("2026-W05");
    expect(parseWeek("2026-W53")).toBe("2026-W53");
    expect(parseWeek("2025-W53")).toBeNull(); // 2025 has 52 weeks
    expect(parseWeek("2026-W54")).toBeNull();
    expect(parseWeek("2026-W00")).toBeNull();
    expect(parseWeek("nope")).toBeNull();
    expect(parseWeek("2026-11-10")).toBeNull();
  });

  it("moves a week back and forward across the year's end", () => {
    expect(shiftWeek("2026-W46", 1)).toBe("2026-W47");
    expect(shiftWeek("2026-W53", 1)).toBe("2027-W01");
    expect(shiftWeek("2027-W01", -1)).toBe("2026-W53");
    expect(shiftWeek("bad", 1)).toBeNull();
  });

  it("takes this week from today in the reader's own time zone", () => {
    // Sunday 15 November 2026 at 23:30 UTC is already Monday 16 November (W47) in Auckland, still W46 in London
    // and in Los Angeles.
    const at = new Date("2026-11-15T23:30:00Z");
    expect(todayIn("Pacific/Auckland", at)).toBe("2026-11-16");
    expect(thisWeek("Pacific/Auckland", at)).toBe("2026-W47");
    expect(thisWeek("Europe/London", at)).toBe("2026-W46");
    expect(thisWeek("America/Los_Angeles", at)).toBe("2026-W46");
    expect(todayIn("Not/AZone", at)).toBe("2026-11-15");
  });
});

describe("people", () => {
  it("takes a person: or worker: reference with a UUID, as the service does", () => {
    expect(personProblem(ALICE)).toBeNull();
    expect(
      personProblem(" worker:6F1C2A3B-0000-4000-8000-000000000001 "),
    ).toBeNull();
    expect(personProblem("person:alice")).toBe("person_not_reference");
    expect(personProblem("6f1c2a3b-0000-4000-8000-000000000001")).toBe(
      "person_not_reference",
    );
  });

  it("compares two references on the id, ignoring the prefix and case", () => {
    expect(
      samePerson(ALICE, "worker:6F1C2A3B-0000-4000-8000-000000000001"),
    ).toBe(true);
    expect(
      samePerson(ALICE, "person:6f1c2a3b-0000-4000-8000-000000000002"),
    ).toBe(false);
    expect(samePerson("nonsense", "nonsense")).toBe(false);
  });
});

describe("time in words", () => {
  it("writes whole minutes as hours and minutes, and no evidence as a dash", () => {
    i18n.set("en-001");
    expect(durationText(450)).toBe("7 hrs 30 mins");
    expect(durationText(45)).toBe("45 mins");
    expect(durationText(480)).toBe("8 hrs");
    expect(durationText(0)).toBe("0 mins");
    expect(durationText(61)).toBe("1 hr 1 min");
    expect(durationText(null)).toBe("—");
    i18n.set("cy-001");
    expect(durationText(450)).toBe("7 awr 30 munud");
    i18n.set("en-001");
  });
});

describe("steps and bodies", () => {
  it("offers submit on an open or sent-back week with time in it, and decide on a submitted one", () => {
    expect(canSubmit("open", 135)).toBe(true);
    expect(canSubmit("rejected", 135)).toBe(true);
    expect(canSubmit("open", null)).toBe(false);
    expect(canSubmit("submitted", 135)).toBe(false);
    expect(canSubmit("approved", 135)).toBe(false);
    expect(canDecide("submitted")).toBe(true);
    expect(canDecide("open")).toBe(false);
    expect(canDecide("approved")).toBe(false);
    expect(canDecide("rejected")).toBe(false);
    expect(statusOf("approved")).toBe("approved");
    expect(statusOf("something new")).toBe("open");
  });

  it("refuses a decision on one's own week and a rejection without a reason", () => {
    const own = "person:6f1c2a3b-0000-4000-8000-000000000009";
    expect(decisionProblem("approved", "", ALICE, own)).toBeNull();
    expect(decisionProblem("rejected", "  ", ALICE, own)).toBe(
      "comment_required",
    );
    expect(decisionProblem("rejected", "fix Monday", ALICE, own)).toBeNull();
    expect(decisionProblem("approved", "", own, own)).toBe("own_week");
    expect(decisionProblem("approved", "", ALICE, null)).toBeNull();
  });

  it("builds the bodies the service reads", () => {
    expect(submitBody("2026-W46")).toEqual({ week: "2026-W46" });
    expect(decideBody(` ${ALICE} `, "2026-W46", "approved", "  ")).toEqual({
      person: ALICE,
      week: "2026-W46",
      decision: "approved",
    });
    expect(decideBody(ALICE, "2026-W46", "rejected", " fix Monday ")).toEqual({
      person: ALICE,
      week: "2026-W46",
      decision: "rejected",
      comment: "fix Monday",
    });
  });
});

describe("refusals", () => {
  it("names each refusal by step and status", () => {
    expect(timesheetRefusalKey("read_other", 403)).toBe(
      "p.timesheet.refused_read_forbidden",
    );
    expect(timesheetRefusalKey("submit", 403)).toBe(
      "p.timesheet.refused_submit_forbidden",
    );
    expect(timesheetRefusalKey("decide", 403)).toBe(
      "p.timesheet.refused_decide_forbidden",
    );
    expect(timesheetRefusalKey("submit", 409)).toBe(
      "p.timesheet.refused_submit_conflict",
    );
    expect(timesheetRefusalKey("decide", 409)).toBe(
      "p.timesheet.refused_decide_conflict",
    );
    expect(timesheetRefusalKey("read_own", 409)).toBeNull();
    expect(timesheetRefusalKey("read_own", 422)).toBe(
      "p.timesheet.refused_not_addressable",
    );
    expect(timesheetRefusalKey("read_other", 422)).toBe(
      "p.timesheet.refused_read_invalid",
    );
    expect(timesheetRefusalKey("submit", 422)).toBe(
      "p.timesheet.refused_submit_invalid",
    );
    expect(timesheetRefusalKey("decide", 422)).toBe(
      "p.timesheet.refused_decide_invalid",
    );
    expect(timesheetRefusalKey("decide", 500)).toBeNull();
  });
});
