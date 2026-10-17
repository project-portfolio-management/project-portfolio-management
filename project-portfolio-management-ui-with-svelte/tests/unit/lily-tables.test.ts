import { describe, expect, it } from "vitest";
import { flushSync, mount, unmount } from "svelte";
import LilyKanbanTable from "../../src/lib/components/LilyKanbanTable.svelte";

/** Mount into jsdom (the suite compiles for the client), return the HTML, clean up. */
function render(
  component: unknown,
  options: { props: Record<string, unknown> },
): { body: string } {
  const target = document.createElement("div");
  document.body.append(target);
  const instance = mount(component as never, { target, props: options.props });
  flushSync();
  const body = target.innerHTML;
  void unmount(instance);
  target.remove();
  return { body };
}

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
