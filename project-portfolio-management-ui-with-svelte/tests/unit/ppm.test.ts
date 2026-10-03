// PpmClient path mapping + money formatting (the client is the single
// source of PPM endpoint paths, so a wrong path fails here first).

import { describe, expect, it, vi } from "vitest";
import { ApiClient } from "$lib/api/client";
import { PpmClient, money } from "$lib/api/ppm";

/** A PpmClient whose fetch records every (url, method) pair. */
function recordingClient(): { ppm: PpmClient; calls: { url: string; method: string }[] } {
  const calls: { url: string; method: string }[] = [];
  const fetchFn = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
    calls.push({ url: String(input), method: init?.method ?? "GET" });
    return new Response("{}", {
      status: 200,
      headers: { "content-type": "application/json" },
    });
  }) as unknown as typeof fetch;
  const ppm = new PpmClient(new ApiClient({ baseUrl: "http://svc", fetch: fetchFn }));
  return { ppm, calls };
}

describe("PpmClient my-work path (LT-3)", () => {
  it("sends the IANA zone and, when given, as_of", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.myTasks("America/Los_Angeles");
    await ppm.myTasks("Europe/London", "2026-10-02T23:30:00Z");
    expect(calls[0].url).toBe("http://svc/api/me/tasks?tz=America%2FLos_Angeles");
    expect(calls[1].url).toBe(
      "http://svc/api/me/tasks?tz=Europe%2FLondon&as_of=2026-10-02T23%3A30%3A00Z",
    );
    expect(calls.every((c) => c.method === "GET")).toBe(true);
  });
});

describe("PpmClient critical-path and calendar paths (SC-1, SC-2)", () => {
  it("maps the endpoints and verbs", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.planCpm("p1");
    await ppm.planCpm("p1", 3);
    await ppm.setTaskSchedule("p1", "t1", { duration_days: 3 });
    await ppm.planCalendar("p1");
    await ppm.setPlanCalendar("p1", { weekdays: "1111100", holidays: [] });
    await ppm.meCalendar();
    await ppm.setMeCalendar({ weekdays: "1110000", holidays: [] });
    await ppm.commitAllocation("p1", "a1");
    await ppm.commitAllocation("p1", "a1", true);
    await ppm.availability();
    await ppm.availability("person:x y");
    await ppm.setAvailability({ person_ref: "person:x", from: "2026-10-12", to: "2026-10-18", max_percent: 40 });
    await ppm.removeAvailability("v1");
    await ppm.scheduleBaselines("p1");
    await ppm.takeScheduleBaseline("p1", "approved");
    await ppm.scheduleVariance("p1");
    await ppm.scheduleVariance("p1", 2);
    await ppm.planRaci("p1");
    await ppm.setRaciRow("p1", "t1", [{ person_ref: "person:x", roles: "AR" }]);
    await ppm.earnedValue("p1");
    await ppm.setEarnedValueMethod("p1", "points");
    await ppm.setTaskPercent("p1", "t1", { method: "manual", percent: 60, note: "why" });
    expect(calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`)).toEqual([
      "GET /api/plans/p1/cpm",
      "GET /api/plans/p1/cpm?near=3",
      "PUT /api/plans/p1/tasks/t1/schedule",
      "GET /api/plans/p1/calendar",
      "PUT /api/plans/p1/calendar",
      "GET /api/me/calendar",
      "PUT /api/me/calendar",
      "POST /api/plans/p1/allocations/a1/commit",
      "POST /api/plans/p1/allocations/a1/commit?confirm=true",
      "GET /api/availability",
      "GET /api/availability?person_ref=person%3Ax%20y",
      "POST /api/availability",
      "POST /api/availability/v1/delete",
      "GET /api/plans/p1/schedule-baselines",
      "POST /api/plans/p1/schedule-baselines",
      "GET /api/plans/p1/schedule-variance",
      "GET /api/plans/p1/schedule-variance?baseline=2",
      "GET /api/plans/p1/raci",
      "PUT /api/plans/p1/tasks/t1/raci",
      "GET /api/plans/p1/earned-value",
      "PUT /api/plans/p1/earned-value-settings",
      "PUT /api/plans/p1/tasks/t1/percent-complete",
    ]);
  });
});

describe("PpmClient paths", () => {
  it("maps the catalogue endpoints", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.dashboard();
    await ppm.listProposals("draft");
    await ppm.proposalAction("p1", "approve");
    await ppm.promoteProposal("p1");
    await ppm.proposalDuplicates("p1");
    await ppm.convertIdea("i1", "projects");
    await ppm.governance("w1");
    await ppm.reviewGate("w1", { gate: "g0_concept", decision: "approved" });
    await ppm.escalateRisk("w2", "r1");
    await ppm.recordActual("w3", "l1", 500);
    await ppm.realizeBenefit("w4", "b1", { amount_minor: 1 });
    await ppm.schedule("w3");
    await ppm.capacity("2026-07-01", "2026-07-31");
    await ppm.evaluateScenario("s1");
    await ppm.commitScenario("s1");
    await ppm.alignment("o1");
    await ppm.linkObjective("w1", "o1", 4);
    await ppm.runReport("rep1");
    await ppm.executiveHealth();
    await ppm.executiveDecisions(10);
    await ppm.executiveBenefits();
    await ppm.financialVariance();
    await ppm.financialExposure();
    await ppm.technologyDependencyRisk();
    await ppm.technologyRadar();
    await ppm.executiveAlignment();
    await ppm.technologyDebt();
    await ppm.technologyFlow(3);
    await ppm.compareScenarios("s1", "s2");
    await ppm.boardPack();
    await ppm.boardInvestments();
    await ppm.takeSnapshot();
    await ppm.boardTrends();
    await ppm.auditorTrail({ action: "merged" });
    await ppm.auditorFindings();
    await ppm.complianceRegister();
    await ppm.complianceFindings();
    await ppm.riskHeatmap();
    await ppm.securityRegister();
    await ppm.regulatorExtract();
    await ppm.listTasks("w1");
    await ppm.createTask("w1", { title: "T" });
    await ppm.moveTask("w1", "t1", "done");
    await ppm.listSprints("w1");
    await ppm.createSprint("w1", { name: "S1" });
    await ppm.burndown("w1", "s1");
    await ppm.standup("w1");
    await ppm.engineeringBlocked();
    await ppm.engineeringMoscow();
    await ppm.engineeringDeliveryLinks();
    await ppm.milestoneCalendar("demo");
    await ppm.velocity("w1");
    await ppm.listNotes("w1", "s1");
    await ppm.createNote("w1", "s1", { category: "action", body: "B" });
    await ppm.convertNote("w1", "s1", "n1");
    await ppm.devopsMetrics(6);
    await ppm.devopsReleases();

    expect(calls.map((call) => `${call.method} ${call.url}`)).toEqual([
      "GET http://svc/api/at-a-glance",
      "GET http://svc/api/proposals?status=draft",
      "POST http://svc/api/proposals/p1/approve",
      "POST http://svc/api/proposals/p1/promote",
      "GET http://svc/api/proposals/p1/duplicates",
      "POST http://svc/api/ideas/i1/convert",
      "GET http://svc/api/plans/w1/governance",
      "POST http://svc/api/plans/w1/gate-reviews",
      "POST http://svc/api/plans/w2/risks/r1/escalate",
      "POST http://svc/api/plans/w3/budget-lines/l1/actual",
      "POST http://svc/api/plans/w4/benefits/b1/realize",
      "GET http://svc/api/plans/w3/schedule",
      "GET http://svc/api/capacity?from=2026-07-01&to=2026-07-31",
      "GET http://svc/api/scenarios/s1/evaluate",
      "POST http://svc/api/scenarios/s1/commit",
      "GET http://svc/api/objectives/o1/alignment",
      "POST http://svc/api/plans/w1/objectives",
      "GET http://svc/api/reports/rep1/run",
      "GET http://svc/api/executive/health",
      "GET http://svc/api/executive/decisions?limit=10",
      "GET http://svc/api/executive/benefits",
      "GET http://svc/api/financials/variance",
      "GET http://svc/api/financials/exposure",
      "GET http://svc/api/technology/dependency-risk",
      "GET http://svc/api/technology/radar",
      "GET http://svc/api/executive/alignment",
      "GET http://svc/api/technology/debt",
      "GET http://svc/api/technology/flow?months=3",
      "GET http://svc/api/scenarios/compare?a=s1&b=s2",
      "GET http://svc/api/board/pack",
      "GET http://svc/api/board/investments",
      "POST http://svc/api/board/snapshots",
      "GET http://svc/api/board/trends",
      "GET http://svc/api/auditor/trail?action=merged",
      "GET http://svc/api/auditor/findings",
      "GET http://svc/api/compliance/register",
      "GET http://svc/api/compliance/findings",
      "GET http://svc/api/risk/heatmap",
      "GET http://svc/api/security/register",
      "GET http://svc/api/regulator/extract",
      "GET http://svc/api/plans/w1/tasks",
      "POST http://svc/api/plans/w1/tasks",
      "PATCH http://svc/api/plans/w1/tasks/t1",
      "GET http://svc/api/plans/w1/sprints",
      "POST http://svc/api/plans/w1/sprints",
      "GET http://svc/api/plans/w1/burndown?sprint=s1",
      "GET http://svc/api/plans/w1/standup",
      "GET http://svc/api/engineering/blocked",
      "GET http://svc/api/engineering/moscow",
      "GET http://svc/api/engineering/delivery-links",
      "GET http://svc/api/engineering/milestone-calendar?kind=demo",
      "GET http://svc/api/plans/w1/velocity",
      "GET http://svc/api/plans/w1/sprints/s1/notes",
      "POST http://svc/api/plans/w1/sprints/s1/notes",
      "POST http://svc/api/plans/w1/sprints/s1/notes/n1/convert",
      "GET http://svc/api/devops/metrics?months=6",
      "GET http://svc/api/devops/releases",
    ]);
  });

  it("builds the CSV download URL against the configured base", () => {
    const { ppm } = recordingClient();
    // reportCsvUrl uses the app-wide API_BASE_URL (the BFF proxy).
    expect(ppm.reportCsvUrl("rep1")).toContain("/api/reports/rep1/run?format=csv");
  });
});

describe("money", () => {
  it("formats minor units as grouped major units", () => {
    expect(money(123_450, "GBP")).toBe("1,234.50 GBP");
    expect(money(5, "EUR")).toBe("0.05 EUR");
    expect(money(-99, "GBP")).toBe("-0.99 GBP");
    expect(money(100_00)).toBe("100.00");
  });
});
