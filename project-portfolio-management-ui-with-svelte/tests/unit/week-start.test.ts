// WW-1.4 (UI): the first day of the week is the locale's by default and the person's choice when made.
import { describe, expect, it } from "vitest";
import {
  calendarLocale,
  firstDayOfWeek,
  parseWeekStart,
} from "../../src/lib/week-start";
import { parsePrefs } from "../../src/lib/preferences";

describe("week start", () => {
  it("follows the locale by default", () => {
    expect(firstDayOfWeek("en-001")).toBe(1);
    expect(firstDayOfWeek("de-de")).toBe(1);
    expect(firstDayOfWeek("en-US")).toBe(0);
    expect(firstDayOfWeek("ar-EG")).toBe(6);
  });
  it("a choice wins over the locale", () => {
    expect(firstDayOfWeek("en-001", "sun")).toBe(0);
    expect(firstDayOfWeek("en-US", "mon")).toBe(1);
    expect(firstDayOfWeek("de-001", "sat")).toBe(6);
    expect(firstDayOfWeek("en-US", "locale")).toBe(0);
  });
  it("reaches the calendar as the locale's fw extension, which the engine honours", () => {
    expect(calendarLocale("en-001")).toBe("en-001");
    expect(calendarLocale("en-001", "sun")).toBe("en-001-u-fw-sun");
    expect(calendarLocale("ar-001-u-nu-arab", "sat")).toBe("ar-001-u-fw-sat");
    expect(firstDayOfWeek(calendarLocale("en-001", "sun"))).toBe(0);
  });
  it("is kept with the display preferences, and anything else is dropped", () => {
    expect(parseWeekStart("sun")).toBe("sun");
    expect(parseWeekStart("locale")).toBeUndefined();
    expect(parseWeekStart("friday")).toBeUndefined();
    expect(parsePrefs({ weekStart: "sat", simple: true })).toEqual({
      simple: true,
      weekStart: "sat",
    });
    expect(parsePrefs({ weekStart: "tuesday" })).toEqual({});
  });
});
