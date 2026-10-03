import { describe, expect, it } from "vitest";
import { flushSync, mount, unmount } from "svelte";
import LilyGanttTable from "../../src/lib/components/LilyGanttTable.svelte";
import LilyKanbanTable from "../../src/lib/components/LilyKanbanTable.svelte";

/** Mount into jsdom (the suite compiles for the client), return the HTML, clean up. */
function render(component: unknown, options: { props: Record<string, unknown> }): { body: string } {
  const target = document.createElement("div");
  document.body.append(target);
  const instance = mount(component as never, { target, props: options.props });
  flushSync();
  const body = target.innerHTML;
  void unmount(instance);
  target.remove();
  return { body };
}

describe("Lily Gantt table", () => {
  const items = [
    { pid: "a", name: "Design", start: "2026-10-05", end: "2026-10-14", critical: true },
    { pid: "b", name: "Undated", start: null, end: null },
  ];
  it("is a labelled grid whose marked cells name their task and dates", () => {
    const { body } = render(LilyGanttTable, { props: { items, label: "Schedule" } });
    expect(body).toContain('role="grid"');
    expect(body).toContain('aria-label="Schedule"');
    expect(body).toContain("Design: 2026-10-05 to 2026-10-14");
    expect(body).toContain("(ends here)");
    expect(body).toContain('aria-selected="true"');
    expect(body).toContain("critical path");
  });
  it("lists a task with no dates instead of drawing it", () => {
    const { body } = render(LilyGanttTable, { props: { items } });
    expect(body).toMatch(/No usable dates:[\s\S]*Undated/);
    expect(body).not.toContain("Undated: not scheduled");
  });
  it("says so when nothing is dated", () => {
    const { body } = render(LilyGanttTable, { props: { items: [items[1]!] } });
    expect(body).toContain("No dated tasks to show.");
  });
});

describe("Lily Kanban table", () => {
  const columns = [
    { id: "todo", label: "Todo" },
    { id: "done", label: "Done" },
  ];
  it("is a labelled grid with a labelled Move control on every card", () => {
    const { body } = render(LilyKanbanTable, {
      props: {
        cards: [
          { pid: "1", title: "Fix login", status: "todo" },
          { pid: "2", title: "Ship", status: "done" },
        ],
        columns,
        label: "Board",
        onmove: () => undefined,
      },
    });
    expect(body).toContain('role="grid"');
    expect(body).toContain("Todo (1)");
    expect(body).toContain("Move Fix login to");
    expect(body).toContain("Move Ship to");
    expect(body).toContain('aria-label="Fix login, Todo"');
  });
  it("shows an unknown-status card rather than dropping it", () => {
    const { body } = render(LilyKanbanTable, {
      props: {
        cards: [{ pid: "9", title: "Odd", status: "archived" }],
        columns,
        onmove: () => undefined,
      },
    });
    expect(body).toContain("Unknown status:");
    expect(body).toContain("Odd (archived)");
  });
});
