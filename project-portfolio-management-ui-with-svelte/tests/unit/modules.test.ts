// RM-7: which plan workspace links show, given the enabled modules.

import { describe, expect, it } from "vitest";
import { PLAN_LINKS, visiblePlanLinks } from "../../src/lib/modules";

describe("visiblePlanLinks (RM-7)", () => {
  it("fails open: an unknown module list shows every link", () => {
    expect(visiblePlanLinks(null)).toEqual(PLAN_LINKS.map((l) => l.key));
  });

  it("shows only the links whose module is on", () => {
    expect(visiblePlanLinks(["tasks", "time"])).toEqual([
      "board",
      "flow",
      "critical-path",
      "raci",
      "schedule-health",
      "leveling",
      "schedule-risk",
      "work",
      "what-if",
      "onboarding",
      "handover",
      "wbs",
    ]);
    expect(visiblePlanLinks(["governance"])).toEqual(["governance"]);
    expect(visiblePlanLinks(["milestones", "tasks"])).toEqual([
      "board",
      "schedule",
      "flow",
      "critical-path",
      "raci",
      "calendar",
      "schedule-health",
      "leveling",
      "schedule-risk",
      "work",
      "what-if",
      "onboarding",
      "handover",
      "wbs",
    ]);
  });

  it("an explicit empty set is a real choice and hides every link", () => {
    expect(visiblePlanLinks([])).toEqual([]);
  });

  it("every link names a module the service knows", () => {
    const known = [
      "tasks",
      "time",
      "risks",
      "financials",
      "milestones",
      "governance",
    ];
    for (const l of PLAN_LINKS) expect(known).toContain(l.module);
  });
});
