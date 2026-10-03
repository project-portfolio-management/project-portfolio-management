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
