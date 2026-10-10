import { describe, expect, it } from "vitest";
import {
  isOverCapacityConflict,
  loadModel,
  spanSegments,
  type CapacityPerson,
} from "../../src/lib/capacity-view";

const person = (over: Partial<CapacityPerson> = {}): CapacityPerson => ({
  person_ref: "person:x",
  allocated_percent: 70,
  over_allocated: false,
  capacity: { source: "default", min_percent: 100 },
  peak_percent: 70,
  over_capacity: [],
  proposed: { percent: 0, allocations: 0, would_exceed_capacity: false },
  ...over,
});

describe("loadModel", () => {
  it("scales the meter to at least 100 and in steps of 25", () => {
    expect(loadModel(person()).scale).toBe(100);
    expect(
      loadModel(person({ allocated_percent: 130, over_allocated: true })).scale,
    ).toBe(150);
    expect(
      loadModel(
        person({
          allocated_percent: 90,
          proposed: {
            percent: 40,
            allocations: 1,
            would_exceed_capacity: true,
          },
        }),
      ).scale,
    ).toBe(150);
  });
  it("draws the firm bar and the capacity line, with the proposal as a separate extension", () => {
    const m = loadModel(
      person({
        allocated_percent: 60,
        capacity: { source: "availability", min_percent: 40 },
        proposed: { percent: 20, allocations: 1, would_exceed_capacity: true },
      }),
    );
    expect(m).toMatchObject({
      scale: 100,
      committedPct: 60,
      proposedPct: 20,
      capacityPct: 40,
    });
  });
  it("gives each state a marker besides colour", () => {
    const over = loadModel(
      person({ allocated_percent: 120, over_allocated: true }),
    );
    const maybe = loadModel(
      person({
        proposed: { percent: 50, allocations: 1, would_exceed_capacity: true },
      }),
    );
    const ok = loadModel(person());
    expect([over.marker, maybe.marker, ok.marker]).toEqual(["!", "◐", "○"]);
    expect([over.state, maybe.state, ok.state]).toEqual([
      "over",
      "proposed_over",
      "ok",
    ]);
  });
  it("says in words what is firm, what is over and what is only proposed", () => {
    const m = loadModel(
      person({
        allocated_percent: 80,
        over_allocated: true,
        capacity: { source: "availability", min_percent: 40 },
        over_capacity: [
          { from: "2026-10-12", to: "2026-10-18", demand: 80, capacity: 40 },
        ],
        proposed: { percent: 50, allocations: 1, would_exceed_capacity: true },
      }),
    );
    expect(m.label).toContain("80% committed against a capacity as low as 40%");
    expect(m.label).toContain(
      "over capacity from 2026-10-12 to 2026-10-18 (80% asked, 40% available)",
    );
    expect(m.label).toContain(
      "50% proposed, not counted; committing it would put them over capacity",
    );
  });
  it("never lets a proposal into the firm figure", () => {
    const m = loadModel(
      person({
        allocated_percent: 70,
        proposed: { percent: 50, allocations: 1, would_exceed_capacity: true },
      }),
    );
    expect(m.scale).toBe(125); // room for the proposal, drawn after the firm bar
    expect(m.committedPct).toBeCloseTo(56); // 70 of 125: the firm bar is never 120
    expect(m.label).toContain("70% committed");
  });
  it("clamps odd values rather than drawing off the meter", () => {
    const m = loadModel(
      person({
        allocated_percent: -5,
        capacity: { source: "default", min_percent: -3 },
      }),
    );
    expect(m.committedPct).toBe(0);
    expect(m.capacityPct).toBe(0);
  });
});

describe("spanSegments", () => {
  const spans = [
    { from: "2026-10-12", to: "2026-10-18", demand: 80, capacity: 40 },
  ];
  it("places a run on the window as percentages of its width", () => {
    const [s] = spanSegments("2026-10-05", "2026-10-18", spans);
    expect(s!.startPct).toBeCloseTo(50);
    expect(s!.widthPct).toBeCloseTo(50);
    expect(s!.label).toBe("2026-10-12 to 2026-10-18: 80% asked, 40% available");
  });
  it("clips a run that starts before the window and skips one outside it", () => {
    const [s] = spanSegments("2026-10-14", "2026-10-20", spans);
    expect(s!.startPct).toBe(0);
    expect(s!.widthPct).toBeCloseTo((5 / 7) * 100);
    expect(spanSegments("2026-11-01", "2026-11-10", spans)).toEqual([]);
  });
  it("is empty for a bad window", () => {
    expect(spanSegments("bad", "2026-10-18", spans)).toEqual([]);
    expect(spanSegments("2026-10-18", "2026-10-05", spans)).toEqual([]);
  });
  it("labels a one-day run without a range", () => {
    const [s] = spanSegments("2026-10-12", "2026-10-12", [
      { from: "2026-10-12", to: "2026-10-12", demand: 10, capacity: 0 },
    ]);
    expect(s!.label).toBe("2026-10-12: 10% asked, 0% available");
  });
});

describe("isOverCapacityConflict", () => {
  it("recognizes the confirmable 409 only", () => {
    expect(
      isOverCapacityConflict(409, { error: "over_capacity", description: "x" }),
    ).toBe(true);
    expect(
      isOverCapacityConflict(409, {
        error: "conflict",
        description: "already committed",
      }),
    ).toBe(false);
    expect(isOverCapacityConflict(404, { error: "over_capacity" })).toBe(false);
    expect(isOverCapacityConflict(undefined, undefined)).toBe(false);
  });
});
