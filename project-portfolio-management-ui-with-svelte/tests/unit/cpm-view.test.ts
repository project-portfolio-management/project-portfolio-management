import { describe, expect, it } from "vitest";
import {
  cpmBars,
  floatLabel,
  floatState,
  layoutNetwork,
  maskToWeekdays,
  monthGrid,
  shiftMonth,
  toggleHoliday,
  unscheduledReason,
  weekdaysToMask,
  weekdaysValid,
} from "../../src/lib/cpm-view";

describe("weekday masks", () => {
  it("round-trips and treats junk as all-working", () => {
    expect(maskToWeekdays("1111100")).toEqual([true, true, true, true, true, false, false]);
    expect(weekdaysToMask(maskToWeekdays("1010101"))).toBe("1010101");
    expect(maskToWeekdays("nope")).toEqual(Array(7).fill(true));
    expect(maskToWeekdays("111110")).toEqual(Array(7).fill(true));
  });
  it("needs at least one working day to be savable", () => {
    expect(weekdaysValid(maskToWeekdays("0000000"))).toBe(false);
    expect(weekdaysValid(maskToWeekdays("0000010"))).toBe(true);
  });
});

describe("holidays", () => {
  it("toggles in and out, sorted and unique", () => {
    let list = toggleHoliday([], "2026-12-25");
    list = toggleHoliday(list, "2026-01-01");
    expect(list).toEqual(["2026-01-01", "2026-12-25"]);
    expect(toggleHoliday(list, "2026-12-25")).toEqual(["2026-01-01"]);
  });
});

describe("monthGrid", () => {
  it("is Monday-first, padded to whole weeks, with the right days in the month", () => {
    // October 2026: the 1st is a Thursday.
    const g = monthGrid(2026, 10, "1111100", []);
    expect(g.every((r) => r.length === 7)).toBe(true);
    expect(g[0]![0]!.date).toBe("2026-09-28");
    expect(g[0]![3]).toMatchObject({ date: "2026-10-01", inMonth: true, working: true });
    expect(g.flat().filter((c) => c.inMonth)).toHaveLength(31);
    expect(g[0]![0]!.inMonth).toBe(false);
    expect(g).toHaveLength(5);
  });
  it("marks weekends, holidays and absences, each separately", () => {
    const g = monthGrid(2026, 10, "1111100", ["2026-10-12"], ["2026-10-14"]);
    const cell = (d: string) => g.flat().find((c) => c.date === d)!;
    expect(cell("2026-10-03")).toMatchObject({ offDay: true, working: false });
    expect(cell("2026-10-12")).toMatchObject({ holiday: true, working: false, offDay: false });
    expect(cell("2026-10-14")).toMatchObject({ absent: true, working: false });
    expect(cell("2026-10-13").working).toBe(true);
  });
  it("handles a month that needs six rows, December and leap February", () => {
    expect(monthGrid(2026, 8, "1111111", [])).toHaveLength(6); // Aug 2026 starts on a Saturday
    expect(monthGrid(2026, 12, "1111100", []).flat().filter((c) => c.inMonth)).toHaveLength(31);
    expect(monthGrid(2028, 2, "1111100", []).flat().filter((c) => c.inMonth)).toHaveLength(29);
  });
  it("moves between months across a year boundary", () => {
    expect(shiftMonth(2026, 12, 1)).toEqual({ year: 2027, month: 1 });
    expect(shiftMonth(2026, 1, -1)).toEqual({ year: 2025, month: 12 });
    expect(shiftMonth(2026, 10, 14)).toEqual({ year: 2027, month: 12 });
  });
});

describe("float wording and state", () => {
  const n = (total_float: number, critical = total_float <= 0, near_critical = false) => ({
    total_float,
    critical,
    near_critical,
  });
  it("says it in words, and never leaves a bare minus sign", () => {
    expect(floatLabel(n(0))).toMatch(/critical path/);
    expect(floatLabel(n(1))).toBe("1 working day of float");
    expect(floatLabel(n(3))).toBe("3 working days of float");
    expect(floatLabel(n(2, false, true))).toBe("2 working days of float (near-critical)");
    expect(floatLabel(n(-1))).toBe("Cannot meet its constraint or deadline: 1 working day short");
    expect(floatLabel(n(-4))).toContain("4 working days short");
  });
  it("gives every state a marker besides colour", () => {
    const markers = [n(-1), n(0), n(2, false, true), n(9, false, false)].map((x) => floatState(x).marker);
    expect(new Set(markers).size).toBe(4);
    expect(floatState(n(-1)).key).toBe("late");
  });
});

describe("cpmBars", () => {
  const node = (pid: string, es: string, ef: string, lf: string, total_float: number) => ({
    pid,
    title: pid,
    early_start: es,
    early_finish: ef,
    late_finish: lf,
    total_float,
    critical: total_float <= 0,
    near_critical: false,
  });
  it("draws a bar from early start to early finish and the float after it", () => {
    const c = cpmBars([node("a", "2026-10-05", "2026-10-07", "2026-10-07", 0), node("b", "2026-10-05", "2026-10-06", "2026-10-09", 3)])!;
    expect(c.from).toBe("2026-10-05");
    expect(c.to).toBe("2026-10-09");
    expect(c.days).toBe(5);
    const [a, b] = c.bars;
    expect(a).toMatchObject({ startPct: 0, widthPct: 60, floatPct: 0, state: "critical" });
    expect(b).toMatchObject({ startPct: 0, widthPct: 40, floatPct: 60, state: "float" });
    expect(a!.label).toContain("critical path");
  });
  it("marks where the latest finish sits when float is negative", () => {
    const c = cpmBars([node("a", "2026-10-05", "2026-10-09", "2026-10-07", -2), node("b", "2026-10-05", "2026-10-09", "2026-10-09", 0)])!;
    expect(c.bars[0]!.limitPct).toBeCloseTo(60);
    expect(c.bars[0]!.floatPct).toBe(0);
    expect(c.bars[0]!.state).toBe("late");
    expect(c.bars[1]!.limitPct).toBeNull();
  });
  it("uses weekly ticks for short spans and monthly for long ones", () => {
    const short = cpmBars([node("a", "2026-10-05", "2026-10-30", "2026-10-30", 0)])!;
    expect(short.ticks.map((t) => t.label)).toEqual(["10-05", "10-12", "10-19", "10-26"]);
    const long = cpmBars([node("a", "2026-10-05", "2027-02-10", "2027-02-10", 0)])!;
    expect(long.ticks.map((t) => t.label)).toEqual(["2026-11", "2026-12", "2027-01", "2027-02"]);
  });
  it("is null with nothing to draw and skips malformed dates", () => {
    expect(cpmBars([])).toBeNull();
    expect(cpmBars([node("a", "bad", "2026-10-05", "2026-10-05", 0)])).toBeNull();
  });
});

describe("layoutNetwork", () => {
  const ids = ["a", "b", "c", "d"];
  const edges = [
    { from: "a", to: "b" },
    { from: "a", to: "c" },
    { from: "b", to: "d" },
    { from: "c", to: "d" },
  ];
  it("puts each node one layer after its longest predecessor chain", () => {
    const l = layoutNetwork(ids, edges);
    const layer = (p: string) => l.nodes.find((n) => n.pid === p)!.layer;
    expect([layer("a"), layer("b"), layer("c"), layer("d")]).toEqual([0, 1, 1, 2]);
    expect(l.layers).toBe(3);
  });
  it("keeps rows within a layer distinct", () => {
    const l = layoutNetwork(ids, edges);
    const rows = l.nodes.filter((n) => n.layer === 1).map((n) => n.row);
    expect(new Set(rows).size).toBe(2);
  });
  it("flags edges that lie on a critical chain", () => {
    const l = layoutNetwork(ids, edges, [["a", "b", "d"]]);
    const crit = l.edges.filter((e) => e.critical).map((e) => `${e.from}>${e.to}`);
    expect(crit.sort()).toEqual(["a>b", "b>d"]);
  });
  it("ignores unknown ends and self-edges, and survives a cycle", () => {
    const l = layoutNetwork(["a", "b"], [{ from: "a", to: "x" }, { from: "a", to: "a" }, { from: "a", to: "b" }, { from: "b", to: "a" }]);
    expect(l.edges.every((e) => ["a", "b"].includes(e.from) && ["a", "b"].includes(e.to))).toBe(true);
    expect(l.nodes).toHaveLength(2);
  });
  it("is deterministic", () => {
    expect(layoutNetwork(ids, edges)).toEqual(layoutNetwork(ids, edges));
  });
  it("lays out an empty network", () => {
    expect(layoutNetwork([], [])).toEqual({ nodes: [], edges: [], layers: 1 });
  });
});

describe("unscheduledReason", () => {
  it("explains each reason in words", () => {
    expect(unscheduledReason("no_duration")).toMatch(/no duration/);
    expect(unscheduledReason("no_anchor")).toMatch(/no start date/);
    expect(unscheduledReason("weird")).toBe("weird");
  });
});
