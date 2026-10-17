import { describe, expect, it } from "vitest";
import {
  dateOf,
  dayNumber,
  gridKeyTarget,
  kanbanGrid,
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

describe("kanbanGrid", () => {
  const cols = [
    { id: "todo", label: "Todo" },
    { id: "done", label: "Done" },
  ];
  it("puts each card in its column, keeping order, with empty cells as null", () => {
    const g = kanbanGrid(
      [
        { pid: "1", title: "One", status: "todo" },
        { pid: "2", title: "Two", status: "todo" },
        { pid: "3", title: "Three", status: "done" },
      ],
      cols,
    );
    expect(g.columns.map((c) => c.count)).toEqual([2, 1]);
    expect(g.rows.map((r) => r.map((c) => c?.pid ?? null))).toEqual([
      ["1", "3"],
      ["2", null],
    ]);
  });
  it("keeps a card with an unknown status visible instead of dropping it", () => {
    const g = kanbanGrid(
      [{ pid: "9", title: "Odd", status: "archived" }],
      cols,
    );
    expect(g.orphans.map((o) => o.pid)).toEqual(["9"]);
    expect(g.rows).toEqual([]);
  });
});

describe("gridKeyTarget (WAI-ARIA grid keys)", () => {
  const at = { row: 1, col: 1 };
  it("steps with the arrows and stops at the edges", () => {
    expect(gridKeyTarget(at, "ArrowRight", 3, 3)).toEqual({ row: 1, col: 2 });
    expect(gridKeyTarget({ row: 1, col: 2 }, "ArrowRight", 3, 3)).toEqual({
      row: 1,
      col: 2,
    });
    expect(gridKeyTarget({ row: 0, col: 0 }, "ArrowUp", 3, 3)).toEqual({
      row: 0,
      col: 0,
    });
    expect(gridKeyTarget(at, "ArrowDown", 3, 3)).toEqual({ row: 2, col: 1 });
    expect(gridKeyTarget(at, "ArrowLeft", 3, 3)).toEqual({ row: 1, col: 0 });
  });
  it("jumps with Home and End, and to the corners with Ctrl", () => {
    expect(gridKeyTarget(at, "Home", 3, 3)).toEqual({ row: 1, col: 0 });
    expect(gridKeyTarget(at, "End", 3, 3)).toEqual({ row: 1, col: 2 });
    expect(gridKeyTarget(at, "Home", 3, 3, true)).toEqual({ row: 0, col: 0 });
    expect(gridKeyTarget(at, "End", 3, 3, true)).toEqual({ row: 2, col: 2 });
  });
  it("ignores other keys and empty grids", () => {
    expect(gridKeyTarget(at, "a", 3, 3)).toBeNull();
    expect(gridKeyTarget(at, "ArrowDown", 0, 3)).toBeNull();
  });
});
