import { describe, expect, it } from "vitest";
import {
  differences,
  kindOf,
  refusalText,
  reviseRefusalKey,
  syncBody,
} from "../../src/lib/template-sync-view";
import type { TemplateChange } from "../../src/lib/api/ppm";

const task = (
  title: string,
  description: string | null,
  points: number | null,
) => ({
  key: 1,
  title,
  description,
  points,
  flow_type: null,
  due_offset_days: null,
  component: null,
  parent: null,
});
const add: TemplateChange = {
  id: "add:3",
  kind: "add",
  title: "New",
  from: null,
  to: task("New", null, 1),
  locally_edited: false,
};
const change: TemplateChange = {
  id: "change:1",
  kind: "change",
  title: "Cut over",
  from: task("Cut over", null, 5),
  to: task("Cut over", "Two nights", 8),
  locally_edited: true,
};

describe("template-sync-view (NC-12)", () => {
  it("says a change's kind as a word and a symbol", () => {
    expect(kindOf(add)).toEqual({ symbol: "+", text: "added" });
    expect(kindOf({ ...add, kind: "weird" as never }).text).toBe(
      "unknown change",
    );
  });
  it("lists only what a change alters, an absent value said as none", () => {
    expect(differences(change)).toEqual([
      "Description: none → Two nights",
      "Points: 5 → 8",
    ]);
    expect(differences(add)).toEqual([]);
  });
  it("sends the ticked changes, and an overwrite only for a ticked change the plan edited", () => {
    expect(syncBody([add, change], new Set(), new Set())).toBeNull();
    expect(
      syncBody(
        [add, change],
        new Set(["add:3"]),
        new Set(["add:3", "change:1"]),
      ),
    ).toEqual({ accept: ["add:3"] });
    expect(
      syncBody(
        [add, change],
        new Set(["add:3", "change:1"]),
        new Set(["change:1"]),
      ),
    ).toEqual({
      accept: ["add:3", "change:1"],
      overwrite_local: ["change:1"],
    });
  });
  it("says why a change was refused from what the page sent", () => {
    expect(
      refusalText({ id: "change:1", reason: "x" }, [change], new Set()),
    ).toMatch(/has edited it/);
    expect(
      refusalText(
        { id: "change:1", reason: "x" },
        [change],
        new Set(["change:1"]),
      ),
    ).toMatch(/removed the task/);
    expect(refusalText({ id: "gone:9", reason: "x" }, [], new Set())).toMatch(
      /^“gone:9”/,
    );
  });
  it("has a sentence for a reshaped tree", () => {
    expect(reviseRefusalKey(422)).toBe("p.template_sync.revise_shape");
    expect(reviseRefusalKey(500)).toBe("p.template_sync.could_not_revise");
  });
});
