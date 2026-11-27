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
      "components",
      "flow",
      "critical-path",
      "raci",
      "schedule-health",
      "leveling",
      "critical-chain",
      "buffers",
      "deployment-moves",
      "schedule-risk",
      "work",
      "what-if",
      "onboarding",
      "handover",
      "wbs",
      "import",
      "template",
      "export",
    ]);
    expect(visiblePlanLinks(["governance"])).toEqual([
      "governance",
      "goals",
      "confidence",
      "programme",
      "status",
      "meetings",
      "lessons",
      "tolerances",
      "escalations",
      "change-requests",
      "ways-of-working",
      "governance-framework",
      "links",
      "cutover",
    ]);
    // Issues are the risks module's in the service, so they follow risks, not governance.
    expect(visiblePlanLinks(["risks"])).toEqual(["issues"]);
    expect(visiblePlanLinks(["milestones", "tasks"])).toEqual([
      "board",
      "components",
      "schedule",
      "flow",
      "critical-path",
      "raci",
      "calendar",
      "schedule-health",
      "leveling",
      "critical-chain",
      "buffers",
      "deployment-moves",
      "schedule-risk",
      "work",
      "what-if",
      "onboarding",
      "handover",
      "wbs",
      "import",
      "template",
      "export",
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
