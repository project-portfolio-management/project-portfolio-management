import { describe, expect, it, vi } from "vitest";
import { flushSync, mount, unmount } from "svelte";
import CalendarMonth from "../../src/lib/components/CalendarMonth.svelte";
import CpmGantt from "../../src/lib/components/CpmGantt.svelte";
import CpmTable from "../../src/lib/components/CpmTable.svelte";
import NetworkDiagram from "../../src/lib/components/NetworkDiagram.svelte";
import type { CpmEdge, CpmNode } from "../../src/lib/api/ppm";

/** Mount into jsdom (the suite compiles for the client); returns the root and a cleanup. */
function render(component: unknown, props: Record<string, unknown>): { root: HTMLElement; done: () => void } {
  const root = document.createElement("div");
  document.body.append(root);
  const instance = mount(component as never, { target: root, props });
  flushSync();
  return {
    root,
    done: () => {
      void unmount(instance);
      root.remove();
    },
  };
}

const node = (pid: string, title: string, over: Partial<CpmNode> = {}): CpmNode => ({
  pid,
  title,
  status: "todo",
  early_start: "2026-10-05",
  early_finish: "2026-10-07",
  late_start: "2026-10-05",
  late_finish: "2026-10-07",
  total_float: 0,
  free_float: 0,
  critical: true,
  near_critical: false,
  planned_start: "2026-10-05",
  duration_days: 3,
  pushed_days: 0,
  constraint: null,
  constraint_date: null,
  deadline: null,
  hard_conflict: false,
  calendar: "plan",
  ...over,
});

const nodes: CpmNode[] = [
  node("a", "Design"),
  node("b", "Build", { total_float: 2, critical: false, late_finish: "2026-10-09", early_start: "2026-10-08", early_finish: "2026-10-09", calendar: "plan+person" }),
  node("c", "Ship", { total_float: -1, critical: true, constraint: "fnlt", constraint_date: "2026-10-06", late_finish: "2026-10-06", hard_conflict: true }),
];
const edges: CpmEdge[] = [
  { pid: "e1", predecessor_pid: "a", successor_pid: "b", kind: "fs", lag_days: 0 },
  { pid: "e2", predecessor_pid: "b", successor_pid: "c", kind: "ss", lag_days: 2 },
];

describe("CpmGantt", () => {
  it("gives every row a marker and a sentence, so state is never colour alone", () => {
    const { root, done } = render(CpmGantt, { nodes });
    const text = root.textContent ?? "";
    expect(root.querySelectorAll("li.row")).toHaveLength(3);
    expect(text).toContain("On the critical path");
    expect(text).toContain("2 working days of float");
    expect(text).toContain("short");
    for (const m of ["●", "○", "!"]) expect(text).toContain(m);
    expect(root.querySelector(".float")).not.toBeNull();
    expect(root.querySelector(".limit")).not.toBeNull();
    done();
  });
  it("reports a selection through the row's button", () => {
    const onselect = vi.fn();
    const { root, done } = render(CpmGantt, { nodes, onselect });
    (root.querySelectorAll("button.name")[1] as HTMLButtonElement).click();
    expect(onselect).toHaveBeenCalledWith("b");
    done();
  });
  it("says so when nothing is scheduled", () => {
    const { root, done } = render(CpmGantt, { nodes: [] });
    expect(root.textContent).toContain("No scheduled tasks");
    done();
  });
});

describe("CpmTable", () => {
  it("is a real table with a caption, row headers and the columns the chart cannot show", () => {
    const { root, done } = render(CpmTable, { nodes });
    expect(root.querySelector("caption")).not.toBeNull();
    expect([...root.querySelectorAll("thead th")].map((h) => h.textContent)).toEqual([
      "Task", "Starts", "Finishes", "Total float", "Free float", "State", "Constraint", "Deadline", "Calendar",
    ]);
    expect(root.querySelectorAll("tbody th[scope=row]")).toHaveLength(3);
    const text = root.textContent ?? "";
    expect(text).toContain("FNLT 2026-10-06");
    expect(text).toContain("overrides its predecessors");
    expect(text).toContain("plan + assignee");
    done();
  });
});

describe("NetworkDiagram", () => {
  it("draws a labelled group with a keyboard-reachable box per task and labels non-trivial links", () => {
    const { root, done } = render(NetworkDiagram, { nodes, edges, chains: [["a", "b"]] });
    expect(root.querySelector("svg")?.getAttribute("role")).toBe("group");
    const boxes = root.querySelectorAll("g.node");
    expect(boxes).toHaveLength(3);
    expect([...boxes].every((b) => b.getAttribute("tabindex") === "0" && b.getAttribute("role") === "button")).toBe(true);
    expect(root.textContent).toContain("SS +2d");
    expect(root.querySelectorAll("path.edge.critical")).toHaveLength(1);
    expect(boxes[0]?.getAttribute("aria-label")).toContain("on the critical path");
    done();
  });
  it("lists the same network in words", () => {
    const { root, done } = render(NetworkDiagram, { nodes, edges, chains: [] });
    const list = root.querySelector("details")?.textContent ?? "";
    expect(list).toContain("Design → Build");
    expect(list).toContain("SS, lag 2");
    done();
  });
  it("selects with Enter", () => {
    const onselect = vi.fn();
    const { root, done } = render(NetworkDiagram, { nodes, edges, chains: [], onselect });
    root.querySelector("g.node")?.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onselect).toHaveBeenCalledWith("a");
    done();
  });
});

describe("CalendarMonth", () => {
  it("shows a symbol for every non-working state, besides colour", () => {
    const { root, done } = render(CalendarMonth, { year: 2026, month: 10, mask: "1111100", holidays: ["2026-10-12"], absences: ["2026-10-14"] });
    const text = root.textContent ?? "";
    expect(text).toContain("October 2026");
    expect(text).toContain("✕");
    expect(text).toContain("A");
    expect(text).toContain("–");
    const label = (d: string) => [...root.querySelectorAll("[aria-label]")].find((e) => e.getAttribute("aria-label")?.startsWith(d))?.getAttribute("aria-label");
    expect(label("2026-10-12")).toBe("2026-10-12, Monday: holiday");
    expect(label("2026-10-03")).toBe("2026-10-03, Saturday: not a working day");
    expect(label("2026-10-13")).toBe("2026-10-13, Tuesday: working day");
    done();
  });
  it("toggles a holiday only on a working day of this month when editable", () => {
    const ontoggle = vi.fn();
    const { root, done } = render(CalendarMonth, { year: 2026, month: 10, mask: "1111100", holidays: [], editable: true, ontoggle });
    const buttons = [...root.querySelectorAll("td button")] as HTMLButtonElement[];
    expect(buttons).toHaveLength(22); // 22 weekdays in October 2026
    buttons.find((b) => b.getAttribute("aria-label")?.startsWith("2026-10-12"))!.click();
    expect(ontoggle).toHaveBeenCalledWith("2026-10-12");
    done();
  });
  it("is read-only by default", () => {
    const { root, done } = render(CalendarMonth, { year: 2026, month: 10, mask: "1111100", holidays: [] });
    expect(root.querySelectorAll("td button")).toHaveLength(0);
    done();
  });
  it("moves between months", () => {
    const { root, done } = render(CalendarMonth, { year: 2026, month: 12, mask: "1111111", holidays: [] });
    (root.querySelector('button[aria-label="Next month"]') as HTMLButtonElement).click();
    flushSync();
    expect(root.textContent).toContain("January 2027");
    done();
  });
});

import CapacityMeter from "../../src/lib/components/CapacityMeter.svelte";

describe("CapacityMeter", () => {
  const person = {
    person_ref: "person:x",
    allocated_percent: 80,
    over_allocated: true,
    allocations: 1,
    capacity: { source: "availability" as const, min_percent: 40 },
    peak_percent: 80,
    over_capacity: [{ from: "2026-10-12", to: "2026-10-18", demand: 80, capacity: 40 }],
    proposed: { percent: 30, allocations: 1, would_exceed_capacity: true },
  };
  it("draws the firm bar, the capacity line, the proposal apart and the over-capacity strip, with words", () => {
    const { root, done } = render(CapacityMeter, { person, from: "2026-10-05", to: "2026-10-18" });
    expect(root.querySelector(".fill")).not.toBeNull();
    expect(root.querySelector(".proposed")).not.toBeNull();
    expect(root.querySelector(".cap")).not.toBeNull();
    expect(root.querySelectorAll(".seg")).toHaveLength(1);
    const text = root.textContent ?? "";
    expect(text).toContain("80% committed against a capacity as low as 40%");
    expect(text).toContain("30% proposed, not counted");
    expect(text).toContain("!");
    expect(root.querySelector("[role=img]")?.getAttribute("aria-label")).toContain("committed");
    done();
  });
  it("omits the strip and the proposal when there are none", () => {
    const quiet = { ...person, over_allocated: false, over_capacity: [], proposed: { percent: 0, allocations: 0, would_exceed_capacity: false } };
    const { root, done } = render(CapacityMeter, { person: quiet, from: "2026-10-05", to: "2026-10-18" });
    expect(root.querySelector(".proposed")).toBeNull();
    expect(root.querySelector(".strip")).toBeNull();
    expect(root.textContent).toContain("○");
    done();
  });
});

import VarianceTable from "../../src/lib/components/VarianceTable.svelte";

describe("VarianceTable", () => {
  const variance = {
    rows: [
      { task_pid: "a", title: "Design", standing: "slipped" as const, baseline_start: "2026-10-05", baseline_finish: "2026-10-07", start: "2026-10-05", finish: "2026-10-09", start_variance_days: 0, finish_variance_days: 2, duration_variance_days: 2 },
      { task_pid: "b", title: "New thing", standing: "added" as const, baseline_start: null, baseline_finish: null, start: "2026-10-12", finish: "2026-10-14", start_variance_days: null, finish_variance_days: null, duration_variance_days: null },
      { task_pid: "c", title: "Dropped", standing: "removed" as const, baseline_start: "2026-10-05", baseline_finish: "2026-10-30", start: null, finish: null, start_variance_days: null, finish_variance_days: null, duration_variance_days: null },
    ],
    summary: { baseline_tasks: 2, current_tasks: 2, matched: 1, slipped: 1, pulled_in: 0, on_baseline: 0, unmeasurable: 0, added: 1, removed: 1, plan_finish_slip_days: -3 },
  };
  it("states slip, added and removed apart, in words, and warns when a removal makes the plan look better", () => {
    const { root, done } = render(VarianceTable, { variance });
    const text = (root.textContent ?? "").replace(/\s+/g, " ");
    expect(text).toContain("1 of 2 baselined tasks are still in the plan");
    expect(text).toContain("1 added and 1 removed since");
    expect(text).toContain("3 working days earlier than baselined");
    expect(text).toContain("deleting work is not an improvement");
    expect(text).toContain("Slipped: finishes 2 working days later");
    expect(text).toContain("Added: not in the baseline, so it has no slip");
    expect(text).toContain("Removed: was in the baseline and is gone");
    expect(root.querySelector("caption")).not.toBeNull();
    expect([...root.querySelectorAll("tbody th[scope=row]")]).toHaveLength(3);
    done();
  });
  it("says the plan finish cannot be compared when it cannot", () => {
    const { root, done } = render(VarianceTable, { variance: { ...variance, summary: { ...variance.summary, plan_finish_slip_days: null, removed: 0 } } });
    expect((root.textContent ?? "").replace(/\s+/g, " ")).toContain("cannot be compared");
    done();
  });
});

import RaciMatrixTable from "../../src/lib/components/RaciMatrix.svelte";

describe("RaciMatrix", () => {
  const ann = "person:3f2c6b7e-0000-4000-8000-0000000000a1";
  const bob = "person:3f2c6b7e-0000-4000-8000-0000000000a2";
  const matrix = {
    plan_pid: "p",
    people: [ann, bob],
    rows: [
      { task_pid: "t1", title: "Design", cells: { [ann]: "RA", [bob]: "C" }, findings: [] },
      { task_pid: "t2", title: "Build", cells: { [bob]: "R" }, findings: ["no_accountable" as const] },
    ],
    summary: { tasks: 2, complete: 1, missing_accountable: 1, multiple_accountable: 0, missing_responsible: 0 },
    workload: [],
  };
  it("shows the four letters as pressed buttons with a sentence each, and names a row's gap in words", () => {
    const { root, done } = render(RaciMatrixTable, { matrix, people: [ann, bob], onchange: () => undefined });
    expect(root.querySelector("caption")).not.toBeNull();
    expect(root.querySelectorAll("tbody tr")).toHaveLength(2);
    const buttons = [...root.querySelectorAll("button.l")] as HTMLButtonElement[];
    expect(buttons).toHaveLength(2 * 2 * 4);
    const pressed = buttons.filter((b) => b.getAttribute("aria-pressed") === "true");
    expect(pressed.map((b) => b.textContent)).toEqual(["R", "A", "C", "R"]);
    expect(buttons[0]!.getAttribute("aria-label")).toBe("Responsible"); // the state is aria-pressed
    expect(buttons[0]!.parentElement?.getAttribute("aria-label")).toBe("Design: 3f2c6b7e… is Responsible and Accountable");
    expect(root.textContent).toContain("Nobody is accountable");
    expect(root.textContent).toContain("✓");
    done();
  });
  it("reports a toggle with the new row, and says when Accountable moves", () => {
    const calls: Array<[string, Record<string, string>, string | null]> = [];
    const { root, done } = render(RaciMatrixTable, { matrix, people: [ann, bob], onchange: (t: string, c: Record<string, string>, n: string | null) => calls.push([t, c, n]) });
    // Bob's A on Design (row 1, second person, second letter).
    const rowButtons = [...root.querySelectorAll("tbody tr")][0]!.querySelectorAll("button.l");
    (rowButtons[4 + 1] as HTMLButtonElement).click();
    expect(calls).toHaveLength(1);
    expect(calls[0]![0]).toBe("t1");
    expect(calls[0]![1]).toEqual({ [ann]: "R", [bob]: "AC" });
    expect(calls[0]![2]).toContain("Accountable moved from 3f2c6b7e… to 3f2c6b7e…");
    done();
  });
  it("shows a column for a person with no letters yet", () => {
    const carol = "person:3f2c6b7e-0000-4000-8000-0000000000a3";
    const { root, done } = render(RaciMatrixTable, { matrix, people: [ann, bob, carol], onchange: () => undefined });
    expect(root.querySelectorAll("thead th")).toHaveLength(1 + 3 + 1);
    expect(root.querySelectorAll("button.l[aria-pressed=true]")).toHaveLength(4);
    done();
  });
});

import EvBars from "../../src/lib/components/EvBars.svelte";

describe("EvBars", () => {
  const block = {
    currency: "GBP", baseline_version: 1, bac_minor: 400_000, pv_minor: 100_000, ev_minor: 200_000, ac_minor: 250_000,
    sv_minor: 100_000, cv_minor: -50_000, spi: { bp: 20_000 }, cpi: { bp: 8000 }, earned_schedule_days: 14, actual_days: 10, spi_t: { bp: 14_000 },
  };
  it("shows planned, earned and actual with their amounts as text, against the budget line", () => {
    const { root, done } = render(EvBars, { block });
    const text = root.textContent ?? "";
    expect(root.querySelectorAll(".fill")).toHaveLength(3);
    expect(root.querySelectorAll(".bac")).toHaveLength(3);
    expect(text).toContain("Planned value");
    expect(text).toContain("1,000.00 GBP");
    expect(text).toContain("2,000.00 GBP");
    expect(text).toContain("2,500.00 GBP");
    expect(text).toContain("budget at completion: 4,000.00 GBP");
    done();
  });
  it("calls an unrecorded actual cost unknown, never zero", () => {
    const { root, done } = render(EvBars, { block: { ...block, ac_minor: null, cv_minor: null } });
    expect(root.textContent).toContain("unknown");
    expect(root.textContent).not.toContain("0.00 GBP\n");
    done();
  });
});

import SCurve from "../../src/lib/components/SCurve.svelte";

describe("SCurve", () => {
  const curve = {
    currency: "GBP", baseline_version: 1, bac_minor: 400_000, snapshots: 1,
    points: [
      { date: "2026-01-01", pv_minor: 0, ev_minor: null, ac_minor: null },
      { date: "2026-01-11", pv_minor: 100_000, ev_minor: 80_000, ac_minor: 90_000 },
      { date: "2026-01-21", pv_minor: 400_000, ev_minor: null, ac_minor: null },
    ],
  };
  it("draws the planned line, dots only where recorded, the budget line, and a table with every number", () => {
    const { root, done } = render(SCurve, { curve });
    expect(root.querySelectorAll("polyline.pv")).toHaveLength(1);
    expect(root.querySelectorAll("circle.ev")).toHaveLength(1);
    expect(root.querySelectorAll("rect.ac")).toHaveLength(1);
    expect(root.querySelectorAll("line.bac")).toHaveLength(1);
    expect(root.querySelector("svg")?.getAttribute("role")).toBe("img");
    const text = root.textContent ?? "";
    expect(text).toContain("only on days recorded (1 so far)");
    expect(text).toContain("not recorded");
    expect(text).toContain("4,000.00 GBP");
    expect(root.querySelectorAll("tbody tr")).toHaveLength(3);
    done();
  });
});
