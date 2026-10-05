import { describe, it, expect } from "vitest";
import { repairAria, repairRowIndexes } from "../../src/lib/aria-repair";

describe("repairing invalid ARIA from embedded widgets", () => {
  it("removes aria-rowindex=0 and leaves valid indexes alone", () => {
    const root = document.createElement("div");
    root.innerHTML =
      '<div aria-rowindex="0"></div><div aria-rowindex="1"></div><div aria-rowindex="2"><span aria-rowindex="0"></span></div>';
    expect(repairRowIndexes(root)).toBe(2);
    expect(root.querySelectorAll('[aria-rowindex="0"]').length).toBe(0);
    expect(root.querySelectorAll("[aria-rowindex]").length).toBe(2);
    expect(repairRowIndexes(root)).toBe(0);
  });

  it("keeps repairing when the widget re-renders", async () => {
    const root = document.createElement("div");
    const handle = repairAria(root);
    const row = document.createElement("div");
    row.setAttribute("aria-rowindex", "0");
    root.append(row);
    await Promise.resolve();
    await new Promise((r) => setTimeout(r, 0));
    expect(row.hasAttribute("aria-rowindex")).toBe(false);
    row.setAttribute("aria-rowindex", "0");
    await new Promise((r) => setTimeout(r, 0));
    expect(row.hasAttribute("aria-rowindex")).toBe(false);
    handle.destroy();
    row.setAttribute("aria-rowindex", "0");
    await new Promise((r) => setTimeout(r, 0));
    expect(row.getAttribute("aria-rowindex")).toBe("0");
  });
});
