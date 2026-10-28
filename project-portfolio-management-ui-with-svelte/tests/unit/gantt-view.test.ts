import { describe, expect, it } from "vitest";
import {
  chartRange,
  chartTasks,
  holidayRanges,
  nonWorkingWeekdays,
  parseGanttColumns,
  toggleGanttColumn,
} from "../../src/lib/gantt-view";

const item = (
  pid: string,
  start: string | null,
  end: string | null,
  on_critical_path = false,
) => ({
  pid,
  kind: "project",
  name: pid.toUpperCase(),
  stage: null,
  start,
  end,
  on_critical_path,
});
const edge = (pid: string, predecessor_pid: string, successor_pid: string) => ({
  pid,
  predecessor_pid,
  successor_pid,
  lag_days: 0,
});

describe("chartTasks", () => {
  it("passes dates, critical flag and predecessors through", () => {
    const tasks = chartTasks({
      items: [
        item("a", "2026-10-05", "2026-10-09", true),
        item("b", "2026-10-12", "2026-10-16"),
      ],
      edges: [edge("e1", "a", "b")],
    });
    expect(tasks).toEqual([
      expect.objectContaining({
        id: "a",
        label: "A",
        start: "2026-10-05",
        end: "2026-10-09",
        critical: true,
      }),
      expect.objectContaining({
        id: "b",
        label: "B",
        start: "2026-10-12",
        end: "2026-10-16",
        critical: false,
        dependsOn: ["a"],
      }),
    ]);
  });
  it("lists undated and reversed items without a bar, never inventing dates", () => {
    const [none, reversed] = chartTasks({
      items: [item("x", null, null), item("y", "2026-10-09", "2026-10-01")],
      edges: [],
    });
    expect(none).not.toHaveProperty("start");
    expect(reversed).not.toHaveProperty("end");
  });
  it("drops a dependency on an unknown or the same task", () => {
    const [a] = chartTasks({
      items: [item("a", "2026-10-05", "2026-10-06")],
      edges: [edge("e1", "ghost", "a"), edge("e2", "a", "a")],
    });
    expect(a).not.toHaveProperty("dependsOn");
  });
});

describe("chartRange", () => {
  const t = (start: string, end: string) => ({
    id: "t",
    label: "T",
    start,
    end,
  });
  it("pads the dated span", () => {
    expect(chartRange([t("2026-10-05", "2026-10-20")])).toEqual({
      start: "2026-10-02",
      end: "2026-10-23",
    });
  });
  it("includes a nearby today but not a faraway one", () => {
    expect(chartRange([t("2026-10-05", "2026-10-20")], "2026-10-25")).toEqual({
      start: "2026-10-02",
      end: "2026-10-28",
    });
    expect(chartRange([t("2026-10-05", "2026-10-20")], "2030-01-01")).toEqual({
      start: "2026-10-02",
      end: "2026-10-23",
    });
  });
  it("centres on today with nothing dated, and is null with neither", () => {
    expect(chartRange([{ id: "x", label: "X" }], "2026-10-15")).toEqual({
      start: "2026-10-01",
      end: "2026-10-29",
    });
    expect(chartRange([])).toBeNull();
  });
});

describe("calendar shading", () => {
  it("numbers days off from Sunday, whatever the mask", () => {
    expect(nonWorkingWeekdays("1111100")).toEqual([0, 6]);
    expect(nonWorkingWeekdays("1111001")).toEqual([5, 6]);
    expect(nonWorkingWeekdays("1111111")).toEqual([]);
  });
  it("shades nothing for a malformed mask", () => {
    expect(nonWorkingWeekdays("")).toEqual([]);
    expect(nonWorkingWeekdays("11111")).toEqual([]);
    expect(nonWorkingWeekdays("11111xx")).toEqual([]);
  });
  it("turns holidays into labelled holiday days", () => {
    expect(holidayRanges(["2026-12-25"], "Holiday")).toEqual([
      { start: "2026-12-25", kind: "holiday", label: "Holiday" },
    ]);
  });
});

describe("optional columns", () => {
  it("gives each task the text its columns show, dates only when it has a bar", () => {
    const [dated, undated] = chartTasks({
      items: [item("a", "2026-10-05", "2026-10-09"), item("x", null, null)],
      edges: [],
    });
    expect(dated?.values).toMatchObject({
      kind: "project",
      start: "2026-10-05",
      end: "2026-10-09",
    });
    expect(undated?.values).not.toHaveProperty("start");
  });
  it("switches a column on or off and keeps the columns in their own order", () => {
    expect(toggleGanttColumn(["start", "end"], "kind", true)).toEqual([
      "kind",
      "start",
      "end",
    ]);
    expect(toggleGanttColumn(["kind", "start"], "kind", false)).toEqual([
      "start",
    ]);
    expect(toggleGanttColumn([], "end", false)).toEqual([]);
  });
  it("reads a remembered choice, and falls back to the dates for anything else", () => {
    expect(parseGanttColumns('["end","kind"]')).toEqual(["kind", "end"]);
    expect(parseGanttColumns("[]")).toEqual([]);
    for (const bad of [null, "", "nope", '["colour"]', '{"a":1}', "3"])
      expect(parseGanttColumns(bad)).toEqual(["start", "end"]);
  });
});
