import { describe, expect, it } from "vitest";
import {
  dateOf,
  dayNumber,
  ganttGrid,
  gridKeyTarget,
  kanbanGrid,
  type GanttItem,
} from "../../src/lib/lily-grids";

const item = (pid: string, start: string | null, end: string | null, critical = false): GanttItem => ({
  pid,
  name: pid,
  start,
  end,
  critical,
});

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

describe("ganttGrid", () => {
  it("marks the run of weeks a task spans, Monday-based", () => {
    // 2026-10-05 is a Monday.
    const g = ganttGrid([item("a", "2026-10-06", "2026-10-14")]);
    expect(g.unit).toBe("week");
    expect(g.columns.map((c) => c.from)).toEqual(["2026-10-05", "2026-10-12"]);
    expect(g.rows[0]!.cells).toEqual([
      { active: true, ends: false },
      { active: true, ends: true },
    ]);
  });
  it("lines several tasks up on one shared timeline", () => {
    const g = ganttGrid([
      item("a", "2026-10-05", "2026-10-11"),
      item("b", "2026-10-19", "2026-10-25", true),
    ]);
    expect(g.columns).toHaveLength(3);
    expect(g.rows[0]!.cells.map((c) => c.active)).toEqual([true, false, false]);
    expect(g.rows[1]!.cells.map((c) => c.active)).toEqual([false, false, true]);
    expect(g.rows[1]!.critical).toBe(true);
  });
  it("never invents an undated or reversed task onto the timeline", () => {
    const g = ganttGrid([
      item("ok", "2026-10-05", "2026-10-06"),
      item("none", null, null),
      item("half", "2026-10-05", null),
      item("back", "2026-10-09", "2026-10-01"),
    ]);
    expect(g.rows.map((r) => r.pid)).toEqual(["ok"]);
    expect(g.undated.map((u) => u.pid)).toEqual(["none", "half", "back"]);
  });
  it("coarsens the unit instead of drawing hundreds of columns, and says so", () => {
    const g = ganttGrid([item("long", "2026-01-01", "2026-12-31")], { unit: "day", maxColumns: 60 });
    expect(g.coarsened).toBe(true);
    expect(g.unit).toBe("week");
    const years = ganttGrid([item("y", "2020-01-01", "2026-12-31")], { unit: "week", maxColumns: 60 });
    expect(years.unit).toBe("month");
    expect(years.columns.length).toBeLessThanOrEqual(84);
    const fine = ganttGrid([item("s", "2026-10-01", "2026-10-03")], { unit: "day" });
    expect(fine.coarsened).toBe(false);
    expect(fine.columns).toHaveLength(3);
  });
  it("builds whole calendar months with correct last days", () => {
    const g = ganttGrid([item("m", "2028-01-15", "2028-03-02")], { unit: "month" });
    expect(g.columns.map((c) => [c.from, c.to])).toEqual([
      ["2028-01-01", "2028-01-31"],
      ["2028-02-01", "2028-02-29"],
      ["2028-03-01", "2028-03-31"],
    ]);
    expect(g.rows[0]!.cells.map((c) => c.ends)).toEqual([false, false, true]);
  });
  it("is empty, not broken, with nothing dated", () => {
    const g = ganttGrid([item("x", null, null)]);
    expect(g.columns).toEqual([]);
    expect(g.rows).toEqual([]);
    expect(g.undated).toHaveLength(1);
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
    const g = kanbanGrid([{ pid: "9", title: "Odd", status: "archived" }], cols);
    expect(g.orphans.map((o) => o.pid)).toEqual(["9"]);
    expect(g.rows).toEqual([]);
  });
});

describe("gridKeyTarget (WAI-ARIA grid keys)", () => {
  const at = { row: 1, col: 1 };
  it("steps with the arrows and stops at the edges", () => {
    expect(gridKeyTarget(at, "ArrowRight", 3, 3)).toEqual({ row: 1, col: 2 });
    expect(gridKeyTarget({ row: 1, col: 2 }, "ArrowRight", 3, 3)).toEqual({ row: 1, col: 2 });
    expect(gridKeyTarget({ row: 0, col: 0 }, "ArrowUp", 3, 3)).toEqual({ row: 0, col: 0 });
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
