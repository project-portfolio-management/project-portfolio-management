import { describe, expect, it } from "vitest";
import {
  MODULE_LABELS,
  modulesBody,
  sameModules,
  toggled,
} from "../../src/lib/module-settings";
import { PLAN_LINKS } from "../../src/lib/modules";

const ORDER = [
  "tasks",
  "time",
  "risks",
  "financials",
  "milestones",
  "governance",
  "roadmap",
];

describe("the plan's tools page (RM-7)", () => {
  it("turns a module on or off in the service's order, without repeats", () => {
    expect(toggled(["time", "tasks"], "risks", true, ORDER)).toEqual([
      "tasks",
      "time",
      "risks",
    ]);
    expect(toggled(["tasks", "time"], "tasks", true, ORDER)).toEqual([
      "tasks",
      "time",
    ]);
    expect(toggled(["tasks", "time"], "tasks", false, ORDER)).toEqual(["time"]);
    // A key the page does not know is kept, after the known ones, rather than dropped.
    expect(toggled(["later", "tasks"], "time", true, ORDER)).toEqual([
      "tasks",
      "time",
      "later",
    ]);
  });

  it("compares module sets in any order", () => {
    expect(sameModules(["tasks", "time"], ["time", "tasks"])).toBe(true);
    expect(sameModules(["tasks"], ["tasks", "time"])).toBe(false);
    expect(sameModules([], [])).toBe(true);
  });

  it("sends nothing when the default is chosen again on a plan that follows it", () => {
    expect(
      modulesBody(["time", "tasks"], {
        source: "default",
        modules: ["tasks", "time"],
      }),
    ).toBeNull();
    expect(
      modulesBody(["tasks"], { source: "default", modules: ["tasks", "time"] }),
    ).toEqual({ modules: ["tasks"] });
    // A plan with its own set saves it, even when it equals the default.
    expect(
      modulesBody(["tasks", "time"], {
        source: "plan",
        modules: ["tasks", "time"],
      }),
    ).toEqual({ modules: ["tasks", "time"] });
    // No module at all is a choice the service accepts.
    expect(modulesBody([], { source: "plan", modules: ["tasks"] })).toEqual({
      modules: [],
    });
  });

  it("names every module the service has", () => {
    expect(Object.keys(MODULE_LABELS)).toEqual(ORDER);
  });

  it("files each plan link under the module whose service routes it reads", () => {
    // The service puts `/api/plans/{pid}/issues` in the risks module (src/modules.rs), so the issues page goes when
    // risks is off, not when governance is.
    const issues = PLAN_LINKS.find((l) => l.key === "issues");
    expect(issues?.module).toBe("risks");
    const known = new Set(ORDER);
    for (const l of PLAN_LINKS) expect(known.has(l.module), l.key).toBe(true);
  });
});
