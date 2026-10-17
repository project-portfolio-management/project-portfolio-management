import { describe, expect, it } from "vitest";
import {
  dateOf,
  dayNumber,
} from "../../src/lib/lily-grids";

describe("dates", () => {
  it("round-trips calendar dates and rejects junk", () => {
    expect(dateOf(dayNumber("2026-10-03") as number)).toBe("2026-10-03");
    expect(dayNumber("2026-10-04")! - dayNumber("2026-10-03")!).toBe(1);
    expect(dayNumber("nope")).toBeNull();
    expect(dayNumber(null)).toBeNull();
    expect(dayNumber("2026-13-45")).toBeNull();
  });
  it("is unmoved by a daylight-saving change", () => {
    // US clocks changed on 2026-03-08: the day before and after are still one apart.
    expect(dayNumber("2026-03-09")! - dayNumber("2026-03-08")!).toBe(1);
    expect(dayNumber("2026-03-08")! - dayNumber("2026-03-07")!).toBe(1);
  });
});
