import { describe, expect, it } from "vitest";
import {
  bands,
  daysBetween,
  FEVER_BOX,
  layoutBars,
  markerPath,
  outcomeLook,
  toPixel,
  zoneLook,
} from "../../src/lib/chain-view";

const LINES = {
  green_amber: { at_start: 30, at_end: 70 },
  amber_red: { at_start: 60, at_end: 100 },
};

describe("zone and outcome looks", () => {
  it("gives every zone a word, a shape and a colour, and a missing zone a dash", () => {
    expect(zoneLook("green")).toMatchObject({ symbol: "●", tone: "green" });
    expect(zoneLook("amber")).toMatchObject({ symbol: "◆", tone: "amber" });
    expect(zoneLook("red")).toMatchObject({ symbol: "▲", tone: "red" });
    expect(zoneLook(null)).toMatchObject({ symbol: "–", tone: "grey" });
    const shapes = new Set(
      ["green", "amber", "red"].map((z) => zoneLook(z as never).symbol),
    );
    expect(shapes.size).toBe(3);
  });
  it("makes a stagger an amber diamond and an impossible project a red triangle", () => {
    expect(outcomeLook("as_planned").symbol).toBe("●");
    expect(outcomeLook("staggered")).toMatchObject({
      symbol: "◆",
      tone: "amber",
    });
    expect(outcomeLook("exceeds_drum")).toMatchObject({
      symbol: "▲",
      tone: "red",
    });
    expect(outcomeLook("no_slot")).toMatchObject({ symbol: "▲", tone: "red" });
  });
});

describe("the fever chart", () => {
  const w = FEVER_BOX.width - FEVER_BOX.left - FEVER_BOX.right;
  const h = FEVER_BOX.height - FEVER_BOX.top - FEVER_BOX.bottom;
  it("puts 0,0 at the bottom left and 100,100 at the top right of the plot", () => {
    expect(toPixel(FEVER_BOX, 0, 0)).toEqual([
      FEVER_BOX.left,
      FEVER_BOX.top + h,
    ]);
    expect(toPixel(FEVER_BOX, 100, 100)).toEqual([
      FEVER_BOX.left + w,
      FEVER_BOX.top,
    ]);
  });
  it("draws a value outside 0 to 100 at the edge", () => {
    expect(toPixel(FEVER_BOX, 140, -20)).toEqual(toPixel(FEVER_BOX, 100, 0));
  });
  it("draws the bands from the configured lines and they share their edges", () => {
    const b = bands(FEVER_BOX, LINES);
    const [x0, y0] = toPixel(FEVER_BOX, 0, 30);
    expect(b.green).toContain(`${x0},${y0}`);
    expect(b.amber).toContain(`${x0},${y0}`);
    const [x1, y1] = toPixel(FEVER_BOX, 100, 100);
    expect(b.red).toContain(`${x1},${y1}`);
  });
  it("gives each zone its own marker shape", () => {
    const shapes = new Set(
      ["green", "amber", "red"].map((z) => markerPath(z as never, 10, 10)),
    );
    expect(shapes.size).toBe(3);
  });
});

describe("timeline bars", () => {
  it("counts days between ISO dates, either way", () => {
    expect(daysBetween("2026-10-05", "2026-10-09")).toBe(4);
    expect(daysBetween("2026-10-09T00:00:00Z", "2026-10-05")).toBe(-4);
  });
  it("places spans on one scale and never lets a bar vanish or leave the edge", () => {
    const bars = layoutBars([
      { start: "2026-10-01", end: "2026-10-10" },
      { start: "2026-10-11", end: "2026-10-20" },
      { start: "2026-10-20", end: "2026-10-20" },
    ]);
    expect(bars[0].left).toBe(0);
    expect(bars[1].left).toBe(50);
    expect(bars[1].width).toBe(50);
    expect(bars[2].width).toBeGreaterThanOrEqual(1.5);
    for (const b of bars) expect(b.left + b.width).toBeLessThanOrEqual(100.01);
  });
  it("has nothing to place when there is nothing", () => {
    expect(layoutBars([])).toEqual([]);
  });
});
