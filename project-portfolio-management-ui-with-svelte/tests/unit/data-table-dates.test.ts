// WW-1.16: a plain DataTable column holding an ISO civil date shows the page's locale's date, not the ISO text.
import { describe, expect, it } from "vitest";
import { flushSync, mount, unmount } from "svelte";
import DataTable from "../../src/lib/components/DataTable.svelte";

describe("DataTable dates", () => {
  it("formats a civil-date cell and leaves other text alone", () => {
    const root = document.createElement("div");
    document.body.append(root);
    const instance = mount(DataTable as never, {
      target: root,
      props: {
        label: "Signoffs",
        columns: [
          { id: "name", header: "Name", rowHeader: true },
          { id: "met_on", header: "Met" },
          { id: "code", header: "Code" },
        ],
        rows: [{ name: "Gate 2", met_on: "2026-10-05", code: "2026-10-05x" }],
      },
    });
    flushSync();
    const text = root.textContent ?? "";
    expect(text).toContain("5 Oct 2026");
    expect(text).not.toMatch(/\b2026-10-05\b(?!x)/);
    expect(text).toContain("2026-10-05x");
    void unmount(instance);
    root.remove();
  });
});
