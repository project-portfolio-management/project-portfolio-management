// PpmClient path mapping + money formatting (the client is the single
// source of PPM endpoint paths, so a wrong path fails here first).

import { describe, expect, it, vi } from "vitest";
import { ApiClient } from "#lib/api/client.js";
import { PpmClient, money } from "#lib/api/ppm.js";

/** A PpmClient whose fetch records every (url, method) pair. */
function recordingClient(): {
  ppm: PpmClient;
  calls: { url: string; method: string }[];
} {
  const calls: { url: string; method: string }[] = [];
  const fetchFn = vi.fn(
    async (input: RequestInfo | URL, init?: RequestInit) => {
      calls.push({ url: String(input), method: init?.method ?? "GET" });
      return new Response("{}", {
        status: 200,
        headers: { "content-type": "application/json" },
      });
    },
  ) as unknown as typeof fetch;
  const ppm = new PpmClient(
    new ApiClient({ baseUrl: "http://svc", fetch: fetchFn }),
  );
  return { ppm, calls };
}

describe("PpmClient my-work path (LT-3)", () => {
  it("sends the IANA zone and, when given, as_of", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.myTasks("America/Los_Angeles");
    await ppm.myTasks("Europe/London", "2026-10-02T23:30:00Z");
    expect(calls[0].url).toBe(
      "http://svc/api/me/tasks?tz=America%2FLos_Angeles",
    );
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
    await ppm.setAvailability({
      person_ref: "person:x",
      from: "2026-10-12",
      to: "2026-10-18",
      max_percent: 40,
    });
    await ppm.removeAvailability("v1");
    await ppm.scheduleBaselines("p1");
    await ppm.takeScheduleBaseline("p1", "approved");
    await ppm.scheduleVariance("p1");
    await ppm.scheduleVariance("p1", 2);
    await ppm.planRaci("p1");
    await ppm.setRaciRow("p1", "t1", [{ person_ref: "person:x", roles: "AR" }]);
    await ppm.earnedValue("p1");
    await ppm.setEarnedValueMethod("p1", "points");
    await ppm.setTaskPercent("p1", "t1", {
      method: "manual",
      percent: 60,
      note: "why",
    });
    await ppm.earnedValueForecast("p1");
    await ppm.earnedValueSeries("p1");
    await ppm.snapshotEarnedValue("p1");
    await ppm.earnedValueRollup("p1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
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
      "GET /api/plans/p1/earned-value/forecast",
      "GET /api/plans/p1/earned-value/series",
      "POST /api/plans/p1/earned-value/snapshot",
      "GET /api/plans/p1/earned-value/rollup",
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
    expect(ppm.reportCsvUrl("rep1")).toContain(
      "/api/reports/rep1/run?format=csv",
    );
  });
});

describe("PpmClient issue paths (NC-1, IR-2, IR-3)", () => {
  it("lists, raises, replaces and raises from a risk", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.planIssues("p1");
    await ppm.raiseIssue("p1", { title: "T" });
    await ppm.updateIssue("p1", "i1", { title: "T2", status: "resolved" });
    await ppm.raiseIssueFromRisk("p1", "r1");
    await ppm.raiseIssueFromTask("p1", "t1");
    await ppm.issue("p1", "i1");
    await ppm.removeIssue("p1", "i1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/issues",
      "POST /api/plans/p1/issues",
      "PUT /api/plans/p1/issues/i1",
      "POST /api/plans/p1/issues/from-risk/r1",
      "POST /api/plans/p1/issues/from-task/t1",
      "GET /api/plans/p1/issues/i1",
      "DELETE /api/plans/p1/issues/i1",
    ]);
  });
});

describe("PpmClient change-request paths (NC-1 slice 2, IR-4)", () => {
  it("lists, drafts, replaces a draft, submits, reviews, decides and withdraws", async () => {
    const { ppm, calls } = recordingClient();
    const body = { title: "T", description: "D", reason: "R" };
    await ppm.planChangeRequests("p1");
    await ppm.draftChangeRequest("p1", body);
    await ppm.updateChangeRequest("p1", "c1", body);
    await ppm.submitChangeRequest("p1", "c1", { approver_ref: "person:bob" });
    await ppm.reviewChangeRequest("p1", "c1");
    await ppm.decideChangeRequest("p1", "c1", {
      decision: "approve",
      reason: "Within tolerance",
    });
    await ppm.withdrawChangeRequest("p1", "c1");
    await ppm.changeRequest("p1", "c1");
    await ppm.implementChangeRequest("p1", "c1");
    await ppm.setChangeRequestLinks("p1", "c1", {
      issue_pids: ["i1"],
      risk_pids: [],
    });
    await ppm.changeRequestPreview("p1", "c1");
    await ppm.baselineChangeRequest("p1", "c1");
    await ppm.updateRisk("p1", "r1", { status: "mitigating", impact: 2 });
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/change-requests",
      "POST /api/plans/p1/change-requests",
      "PUT /api/plans/p1/change-requests/c1",
      "POST /api/plans/p1/change-requests/c1/submit",
      "POST /api/plans/p1/change-requests/c1/review",
      "POST /api/plans/p1/change-requests/c1/decision",
      "POST /api/plans/p1/change-requests/c1/withdraw",
      "GET /api/plans/p1/change-requests/c1",
      "POST /api/plans/p1/change-requests/c1/implement",
      "PUT /api/plans/p1/change-requests/c1/links",
      "GET /api/plans/p1/change-requests/c1/preview",
      "POST /api/plans/p1/change-requests/c1/baseline",
      "PUT /api/plans/p1/risks/r1",
    ]);
  });
});

describe("PpmClient task edit and link paths (NC-1 slice 3, LT-10)", () => {
  it("replaces a task's fields, and lists, adds and removes its links", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.updateTask("p1", "t1", {
      title: "T",
      description: null,
      assignee_ref: null,
      points: null,
      due_date: null,
      component_pid: null,
      parent_task_pid: null,
      sprint_pid: null,
    });
    await ppm.taskRelations("p1", "t1");
    await ppm.addTaskRelation("p1", "t1", {
      kind: "blocks",
      to_task_pid: "t2",
    });
    await ppm.removeTaskRelation("p1", "t1", "r1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "PUT /api/plans/p1/tasks/t1",
      "GET /api/plans/p1/tasks/t1/relations",
      "POST /api/plans/p1/tasks/t1/relations",
      "DELETE /api/plans/p1/tasks/t1/relations/r1",
    ]);
  });
});

describe("PpmClient notification paths (NC-1 slice 4, PL-5, LT-6)", () => {
  it("reads the inbox in a zone, and reads, sets and resets choices and destinations", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.myInbox("Europe/London");
    await ppm.myInbox("Europe/London", true);
    await ppm.myNotificationPrefs();
    await ppm.setNotificationPref({
      kind: "task_assigned",
      channel: "in_app",
      enabled: false,
      digest: "instant",
    });
    await ppm.resetNotificationPref({ kind: "task_assigned", channel: "chat" });
    await ppm.myNotificationSinks();
    await ppm.addNotificationSink({
      kind: "email",
      destination: "me@example.com",
    });
    await ppm.removeNotificationSink("s1");
    await ppm.sinkDeliveries("s1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/me/inbox?tz=Europe%2FLondon",
      "GET /api/me/inbox?tz=Europe%2FLondon&unread=true",
      "GET /api/me/notification-prefs",
      "PUT /api/me/notification-prefs",
      "POST /api/me/notification-prefs/reset",
      "GET /api/me/notification-sinks",
      "POST /api/me/notification-sinks",
      "POST /api/me/notification-sinks/s1/delete",
      "GET /api/me/notification-sinks/s1/deliveries",
    ]);
  });
});

describe("PpmClient discussion paths (NC-1 slice 5, LT-4, LT-5, RM-3)", () => {
  /** A client whose lists answer in two pages, the first naming the second in its Link header. */
  function pagedClient() {
    const calls: { url: string; method: string; init?: RequestInit }[] = [];
    const fetchFn = vi.fn(
      async (input: RequestInfo | URL, init?: RequestInit) => {
        const url = String(input);
        const method = init?.method ?? "GET";
        calls.push({ url, method, init });
        if (method === "GET" && /\/api\/(comments|attachments)\?/.test(url)) {
          const second = url.includes("cursor=");
          return new Response(JSON.stringify([{ pid: second ? "b" : "a" }]), {
            status: 200,
            headers: second
              ? { "content-type": "application/json" }
              : {
                  "content-type": "application/json",
                  link: `<${new URL(url).pathname}${new URL(url).search}&cursor=c2>; rel="next"`,
                },
          });
        }
        return new Response("{}", {
          status: 200,
          headers: { "content-type": "application/json" },
        });
      },
    ) as unknown as typeof fetch;
    return {
      ppm: new PpmClient(
        new ApiClient({ baseUrl: "http://svc", fetch: fetchFn }),
      ),
      calls,
    };
  }

  it("reads every page of comments and files, following the Link header", async () => {
    const { ppm, calls } = pagedClient();
    expect((await ppm.comments("task", "t1")).map((c) => c.pid)).toEqual([
      "a",
      "b",
    ]);
    expect((await ppm.attachments("task", "t1")).map((a) => a.pid)).toEqual([
      "a",
      "b",
    ]);
    expect(calls.map((c) => c.url.replace("http://svc", ""))).toEqual([
      "/api/comments?subject_kind=task&subject_pid=t1",
      "/api/comments?subject_kind=task&subject_pid=t1&cursor=c2",
      "/api/attachments?subject_kind=task&subject_pid=t1",
      "/api/attachments?subject_kind=task&subject_pid=t1&cursor=c2",
    ]);
  });

  it("uses the POST twins for every removal, as the default policy keeps DELETE for admins", async () => {
    const { ppm, calls } = pagedClient();
    const subject = { subject_kind: "task" as const, subject_pid: "t1" };
    await ppm.addComment({ ...subject, body: "Hello" });
    await ppm.editComment("c1", { body: "Hello again" });
    await ppm.deleteComment("c1");
    await ppm.commentHistory("c1");
    await ppm.reactions("task", "t1");
    await ppm.addReaction({ ...subject, kind: "heart" });
    await ppm.removeReaction({ ...subject, kind: "heart" });
    await ppm.watchStatus("task", "t1");
    await ppm.watch(subject);
    await ppm.unwatch(subject);
    await ppm.removeAttachment("a1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "POST /api/comments",
      "PUT /api/comments/c1",
      "POST /api/comments/c1/delete",
      "GET /api/comments/c1/history",
      "GET /api/reactions?subject_kind=task&subject_pid=t1",
      "POST /api/reactions",
      "POST /api/reactions/remove",
      "GET /api/watchers?subject_kind=task&subject_pid=t1",
      "POST /api/watchers",
      "POST /api/watchers/unwatch",
      "POST /api/attachments/a1/delete",
    ]);
  });

  it("uploads the file itself as the body, with its type, and the name in the query", async () => {
    const { ppm, calls } = pagedClient();
    const file = new Blob(["a,b\n1,2\n"], { type: "text/csv" });
    await ppm.uploadAttachment(
      "task",
      "t1",
      file,
      "Résumé 2026.csv",
      "text/csv",
    );
    const call = calls[0]!;
    expect(call.method).toBe("POST");
    expect(call.url).toBe(
      "http://svc/api/attachments?subject_kind=task&subject_pid=t1&filename=R%C3%A9sum%C3%A9+2026.csv",
    );
    expect(call.init?.body).toBe(file);
    expect((call.init?.headers as Record<string, string>)["content-type"]).toBe(
      "text/csv",
    );
  });

  it("downloads through the proxy, from the service's own download route", () => {
    expect(PpmClient.withFetch().attachmentDownloadUrl("a1")).toMatch(
      /\/api\/proxy\/api\/attachments\/a1\/download$/,
    );
  });
});

describe("PpmClient timesheet paths (NC-1 slice 6, LT-11)", () => {
  it("reads a week, mine or someone else's, and submits and decides", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.timesheetWeek("2026-W46");
    await ppm.timesheetWeek(
      "2026-W46",
      "person:6f1c2a3b-0000-4000-8000-000000000001",
    );
    await ppm.submitTimesheet({ week: "2026-W46" });
    await ppm.decideTimesheet({
      person: "person:6f1c2a3b-0000-4000-8000-000000000001",
      week: "2026-W46",
      decision: "approved",
    });
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/timesheets?week=2026-W46",
      "GET /api/timesheets?week=2026-W46&person=person%3A6f1c2a3b-0000-4000-8000-000000000001",
      "POST /api/timesheets/submit",
      "POST /api/timesheets/decide",
    ]);
  });
});

describe("PpmClient access review and break-glass paths (NC-1 slice 7, RG-9, RG-10)", () => {
  it("reads the review and its spreadsheet, lists, opens and ends a grant", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.accessReview();
    await ppm.accessReview({
      plan: "11111111-1111-4111-8111-111111111111",
      older_than_days: 0,
    });
    await ppm.breakGlassGrants();
    await ppm.openBreakGlass({
      plan_pid: "11111111-1111-4111-8111-111111111111",
      reason: "Incident 4412: the plan owner is unreachable",
      minutes: 30,
      notify_ref: "person:6f1c2a3b-0000-4000-8000-00000000000e",
    });
    await ppm.revokeBreakGlass("g1");
    expect(
      calls.map((c) => `${c.method} ${c.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/access-review",
      "GET /api/access-review?plan=11111111-1111-4111-8111-111111111111&older_than_days=0",
      "GET /api/break-glass",
      "POST /api/break-glass",
      "POST /api/break-glass/g1/revoke",
    ]);
    expect(ppm.accessReviewCsvUrl({ older_than_days: 90 })).toMatch(
      /\/api\/access-review\?older_than_days=90&format=csv$/,
    );
    expect(ppm.accessReviewCsvUrl()).toMatch(
      /\/api\/access-review\?format=csv$/,
    );
  });
});

describe("PpmClient canvas paths (NC-1 slice 9, LT-1)", () => {
  it("lists, reads, creates and removes canvases, and adds, edits, removes and promotes items", async () => {
    const { ppm, calls } = recordingClient();
    const c = "22222222-2222-4222-8222-222222222222";
    const i = "44444444-4444-4444-8444-444444444444";
    await ppm.canvasTemplates();
    await ppm.listCanvases();
    await ppm.listCanvases("11111111-1111-4111-8111-111111111111");
    await ppm.canvas(c);
    await ppm.createCanvas({ template: "swot", title: "T" });
    await ppm.deleteCanvas(c);
    await ppm.addCanvasItem(c, { box: "threats", body: "b" });
    await ppm.updateCanvasItem(c, i, { box: "strengths" });
    await ppm.deleteCanvasItem(c, i);
    await ppm.promoteCanvasItem(c, i, { target: "idea" });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/canvas-templates",
      "GET /api/canvases",
      "GET /api/canvases?plan=11111111-1111-4111-8111-111111111111",
      `GET /api/canvases/${c}`,
      "POST /api/canvases",
      `DELETE /api/canvases/${c}`,
      `POST /api/canvases/${c}/items`,
      `PUT /api/canvases/${c}/items/${i}`,
      `DELETE /api/canvases/${c}/items/${i}`,
      `POST /api/canvases/${c}/items/${i}/promote`,
    ]);
  });
});

describe("PpmClient plan template paths (NC-1 slice 9, LT-2)", () => {
  it("lists, reads, saves, starts from and removes templates", async () => {
    const { ppm, calls } = recordingClient();
    const p = "11111111-1111-4111-8111-111111111111";
    const tpl = "66666666-6666-4666-8666-666666666666";
    await ppm.planTemplates();
    await ppm.planTemplate(tpl);
    await ppm.saveAsTemplate(p, { name: "N" });
    await ppm.instantiateTemplate(tpl, { start_date: "2027-01-04" });
    await ppm.deletePlanTemplate(tpl);
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plan-templates",
      `GET /api/plan-templates/${tpl}`,
      `POST /api/plans/${p}/save-as-template`,
      `POST /api/plans/from-template/${tpl}`,
      `DELETE /api/plan-templates/${tpl}`,
    ]);
  });
});

describe("PpmClient director pack writes (NC-1 slice 11, DP-1, DP-12)", () => {
  it("drafts, reviews, issues and reissues", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.draftDirectorPack({ period: "2026-11", audience: "board" });
    await ppm.reviewDirectorPack("p1");
    await ppm.issueDirectorPack("p1");
    await ppm.reissueDirectorPack("p1", "Figures corrected.");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "POST /api/director-packs",
      "POST /api/director-packs/p1/review",
      "POST /api/director-packs/p1/issue",
      "POST /api/director-packs/p1/reissue",
    ]);
  });
});

describe("PpmClient director pack schedules and subscriptions (NC-1 slice 11, DP-12)", () => {
  it("lists, adds, changes and runs schedules, and reads and sets subscriptions", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.packSchedules();
    await ppm.createPackSchedule({ audience: "board", day_of_month: 3 });
    await ppm.updatePackSchedule("s1", { enabled: false });
    await ppm.sweepPackSchedules();
    await ppm.packSubscriptions();
    await ppm.setPackSubscription({ audience: "board", subscribed: true });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/director-pack-schedules",
      "POST /api/director-pack-schedules",
      "PUT /api/director-pack-schedules/s1",
      "POST /api/director-pack-schedules/sweep",
      "GET /api/director-pack-subscriptions",
      "PUT /api/director-pack-subscriptions",
    ]);
  });
});

describe("PpmClient delivery confidence (NC-1 slice 11, DP-2)", () => {
  it("reads and adds an assessment", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.deliveryConfidence("p1");
    await ppm.assessDeliveryConfidence("p1", {
      value: "amber",
      kind: "self",
      rationale: "Two suppliers late.",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/delivery-confidence",
      "PUT /api/plans/p1/delivery-confidence",
    ]);
  });
});

describe("PpmClient delivery exceptions (NC-1 slice 11, DP-3)", () => {
  it("lists with filters, completes and decides", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.deliveryExceptions();
    await ppm.deliveryExceptions({ status: "open", breached: true });
    await ppm.completeException("e1", { cause: "Supplier late" });
    await ppm.decideException("e1", "Fund a second supplier.");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/exceptions",
      "GET /api/exceptions?status=open&breached=true",
      "PUT /api/exceptions/e1",
      "POST /api/exceptions/e1/decision",
    ]);
  });
});

describe("PpmClient strategic aims and the portfolio map (NC-1 slice 11, DP-4)", () => {
  it("reads the map, marks an aim and sets a primary", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.portfolioMap();
    await ppm.portfolioMap("2026-10");
    await ppm.markStrategicAim("o1", true);
    await ppm.setPrimaryAim("p1", "o1");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/portfolio-map",
      "GET /api/portfolio-map?period=2026-10",
      "PUT /api/objectives/o1/strategic-aim",
      "PUT /api/plans/p1/primary-aim",
    ]);
  });
});

describe("PpmClient planned gates and programme status (NC-1 slice 11, DP-5)", () => {
  it("reads and sets gate dates, and reads and decides a status", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.plannedGates("p1");
    await ppm.setPlannedGate("p1", "g2_definition", "2027-02-01");
    await ppm.programmeStatus("p1");
    await ppm.decideProgrammeStatus("p1", {
      status: "paused",
      reason: "Waiting for funding.",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/planned-gates",
      "PUT /api/plans/p1/planned-gates/g2_definition",
      "GET /api/plans/p1/programme-status",
      "POST /api/plans/p1/programme-status",
    ]);
    calls.length = 0;
    await ppm.projectBuffer("p1");
    await ppm.declareProjectBuffer("p1", { buffer_days: 15 });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/project-buffer",
      "PUT /api/plans/p1/project-buffer",
    ]);
  });
});

describe("PpmClient funding sources (NC-1 slice 11, DP-6)", () => {
  it("lists, adds, retires, sets a line's source and reads the money", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.fundingSources();
    await ppm.createFundingSource("Capital grant");
    await ppm.setFundingSourceActive("s1", false);
    await ppm.setLineFundingSource("p1", "l1", "s1");
    await ppm.setLineFundingSource("p1", "l1", null);
    await ppm.portfolioMoney();
    await ppm.portfolioMoney(2026, true);
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/funding-sources",
      "POST /api/funding-sources",
      "PUT /api/funding-sources/s1",
      "PUT /api/plans/p1/budget-lines/l1/funding-source",
      "PUT /api/plans/p1/budget-lines/l1/funding-source",
      "GET /api/portfolio-money",
      "GET /api/portfolio-money?fy=2026&combine_classes=true",
    ]);
  });
});

describe("PpmClient ledger writes (NC-1 slice 11, EX-12)", () => {
  it("records, draws a line, maps a code and reconciles a CSV as text", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.recordAllocation({
      funding_source_pid: "s1",
      amount_minor: 100,
      currency: "GBP",
      allocated_on: "2026-11-01",
    });
    await ppm.setLineAllocation("p1", "l1", "a1");
    await ppm.setLineAllocation("p1", "l1", null);
    await ppm.costCodes();
    await ppm.mapCostCode("CC-1", "l1");
    await ppm.reconcileLedger("cost_code,amount_minor,currency\nCC-1,100,GBP");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "POST /api/funding-allocations",
      "PUT /api/plans/p1/budget-lines/l1/funding-allocation",
      "PUT /api/plans/p1/budget-lines/l1/funding-allocation",
      "GET /api/cost-codes",
      "PUT /api/cost-codes",
      "POST /api/ledger/reconcile",
    ]);
  });
});

describe("PpmClient benefit classification and validation (NC-1 slice 11, DP-9)", () => {
  it("classifies, reads validations and validates", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.classifyBenefit("p1", "b1", { recurrence: "recurring" });
    await ppm.benefitValidations("p1", "b1");
    await ppm.validateBenefit("p1", "b1", { amount_minor: 100 });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "PUT /api/plans/p1/benefits/b1/classification",
      "GET /api/plans/p1/benefits/b1/validations",
      "POST /api/plans/p1/benefits/b1/validations",
    ]);
  });
});

describe("PpmClient status updates and their evidence (NC-1 slice 12 part b, NC-4, BR-2)", () => {
  it("reads and sets the cadence, reads and posts updates, the numbers and the due list", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.planStatus("p1");
    await ppm.statusCadence("p1");
    await ppm.setStatusCadence("p1", 7);
    await ppm.statusUpdates("p1");
    await ppm.postStatusUpdate("p1", { status: "on_track", narrative: "x" });
    await ppm.statusMetrics();
    await ppm.statusUpdatesDue();
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/status",
      "GET /api/plans/p1/status-cadence",
      "PUT /api/plans/p1/status-cadence",
      "GET /api/plans/p1/status-updates",
      "POST /api/plans/p1/status-updates",
      "GET /api/status-metrics",
      "GET /api/status-updates/due",
    ]);
  });
});

describe("PpmClient escalation records and register completeness (NC-1 slice 12 part d, CF-4, EX-1)", () => {
  it("reads and records the acknowledgement and the resolution, and reads the register", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.escalationAcknowledgement("p1", "e1");
    await ppm.acknowledgeEscalation("p1", "e1", { note: "Seen" });
    await ppm.escalationResolution("p1", "e1");
    await ppm.resolveEscalation("p1", "e1", {
      outcome: "decided",
      response: "Funded from contingency",
    });
    await ppm.registerCompleteness();
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/escalations/e1/acknowledgement",
      "PUT /api/plans/p1/escalations/e1/acknowledgement",
      "GET /api/plans/p1/escalations/e1/resolution",
      "PUT /api/plans/p1/escalations/e1/resolution",
      "GET /api/register-completeness",
    ]);
  });
});

describe("PpmClient meetings, lessons and references (NC-1 slice 12 part e, NC-14, BR-3, NC-8, BR-4)", () => {
  it("reads and writes meetings, proposed tasks, lessons and references at the service's paths", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.meetings("p1");
    await ppm.planMeeting("p1", {
      title: "Kick-off",
      scheduled_on: "2026-11-24",
    });
    await ppm.meeting("p1", "m1");
    await ppm.holdMeeting("p1", "m1", { minutes: "We met." });
    await ppm.cancelMeeting("p1", "m1");
    await ppm.previewMeetingTasks("p1", "m1", { items: [] });
    await ppm.commitMeetingTasks("p1", "m1", { items: [] }, "k1");
    await ppm.lessons("p1");
    await ppm.recordLesson("p1", {
      occasion: "closure",
      category: "went_well",
      summary: "s",
    });
    await ppm.withdrawLesson("p1", "l1");
    await ppm.references("p1", "task", "t1");
    await ppm.proposalLessons("pr1");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/meetings",
      "POST /api/plans/p1/meetings",
      "GET /api/plans/p1/meetings/m1",
      "POST /api/plans/p1/meetings/m1/held",
      "POST /api/plans/p1/meetings/m1/cancel",
      "POST /api/plans/p1/meetings/m1/task-proposals/preview",
      "POST /api/plans/p1/meetings/m1/task-proposals/commit",
      "GET /api/plans/p1/lessons",
      "POST /api/plans/p1/lessons",
      "POST /api/plans/p1/lessons/l1/withdraw",
      "GET /api/plans/p1/references?kind=task&pid=t1",
      "GET /api/proposals/pr1/lessons",
    ]);
  });
  it("creates the tasks under the key it is given, so a retry is the same request", async () => {
    const keys: Array<string | null> = [];
    const fetchFn = vi.fn(
      async (_input: RequestInfo | URL, init?: RequestInit) => {
        keys.push(new Headers(init?.headers).get("idempotency-key"));
        return new Response("{}", {
          status: 200,
          headers: { "content-type": "application/json" },
        });
      },
    ) as unknown as typeof fetch;
    const ppm = new PpmClient(
      new ApiClient({ baseUrl: "http://svc", fetch: fetchFn }),
    );
    await ppm.commitMeetingTasks("p1", "m1", { items: [] }, "key-1");
    await ppm.commitMeetingTasks("p1", "m1", { items: [] }, "key-1");
    expect(keys).toEqual(["key-1", "key-1"]);
  });
});

describe("PpmClient programme money, value and benefits, read live (NC-1 slice 12 part f, DP-7, DP-8, DP-9)", () => {
  it("reads and writes at the service's paths, the defaults left out of the query", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.bufferFever();
    await ppm.portfolioEarnedValue();
    await ppm.portfolioEarnedValue("labour");
    await ppm.portfolioBenefits();
    await ppm.issuesAndChanges();
    await ppm.issuesAndChanges(90);
    await ppm.forecastRollup("p1");
    await ppm.forecastRollup("p1", 3);
    await ppm.planForecast("p1");
    await ppm.budgetBaselines("p1");
    await ppm.approveBudgetBaseline("p1", { currency: "GBP", periods: [] });
    await ppm.releaseBudgetLine("p1", "l1");
    await ppm.recordBusinessCase("p1", {
      metric: "Beds",
      baseline_value: 1,
      target_value: 2,
    });
    await ppm.valueRealization("p1");
    await ppm.recordValuePoint("p1", { value: 1, method: "measured" });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/buffer-fever",
      "GET /api/portfolio-earned-value",
      "GET /api/portfolio-earned-value?ac=labour",
      "GET /api/portfolio-benefits",
      "GET /api/director-pack/issues-and-changes",
      "GET /api/director-pack/issues-and-changes?window_days=90",
      "GET /api/financials/forecast?plan=p1",
      "GET /api/financials/forecast?plan=p1&depth=3",
      "GET /api/plans/p1/financials/forecast",
      "GET /api/plans/p1/budget-baselines",
      "POST /api/plans/p1/budget-baselines",
      "POST /api/plans/p1/budget-lines/l1/release",
      "POST /api/plans/p1/business-case",
      "GET /api/plans/p1/value-realization",
      "POST /api/plans/p1/value-points",
    ]);
  });
});

describe("PpmClient ranking and choosing (NC-1 slice 12 part g, NC-6, NC-7, EX-13)", () => {
  it("reads and writes at the service's paths, the defaults left out of the query", async () => {
    const { ppm, calls } = recordingClient();
    const inputs = {
      business_value: 1,
      time_criticality: null,
      risk_reduction: null,
      job_size: null,
      reach: null,
      impact: null,
      confidence_percent: null,
      effort: null,
    };
    await ppm.scoringInputs("p1");
    await ppm.setScoringInputs("p1", inputs);
    await ppm.modelScore("p1", "wsjf");
    await ppm.rankByModel("rice");
    await ppm.scenarioFrontier({ currency: "GBP" });
    await ppm.scenarioFrontier({
      currency: "GBP",
      steps: 5,
      mustInclude: ["p1", "p2"],
    });
    await ppm.scenarioFrontier({ currency: "GBP", caps: [0, 1000], steps: 5 });
    await ppm.generateScenario({ name: "Spring" });
    await ppm.rollbackScenario("s1");
    await ppm.intakeForecast();
    await ppm.intakeForecast({
      periods: 8,
      periodDays: 14,
      historyPeriods: 26,
    });
    await ppm.getProposal("a1");
    await ppm.updateProposal("a1", { title: "T", kind_target: "projects" });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/scoring-inputs",
      "PUT /api/plans/p1/scoring-inputs",
      "GET /api/plans/p1/score?model=wsjf",
      "GET /api/prioritization?model=rice",
      "GET /api/scenarios/frontier?currency=GBP",
      "GET /api/scenarios/frontier?currency=GBP&steps=5&must_include=p1%2Cp2",
      "GET /api/scenarios/frontier?currency=GBP&caps=0%2C1000",
      "POST /api/scenarios/generate",
      "POST /api/scenarios/s1/rollback",
      "GET /api/proposals/forecast",
      "GET /api/proposals/forecast?periods=8&period_days=14&history_periods=26",
      "GET /api/proposals/a1",
      "PUT /api/proposals/a1",
    ]);
  });
});

describe("PpmClient a person's own work (NC-1 slice 12 part h: PL-4, LT-7, RM-3, LT-11, RM-10, PJ-5)", () => {
  it("maps the planner, the feed, following, the work profile, time entries and the time report", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.myDayPlan("2026-11-18", "Europe/London");
    await ppm.planSlot({
      task_pid: "t1",
      starts_at: "2026-11-18T09:00:00Z",
      ends_at: "2026-11-18T10:00:00Z",
    });
    await ppm.removeSlot("s1");
    await ppm.issueFeedToken();
    await ppm.revokeFeedToken();
    await ppm.myWatching();
    await ppm.portfolioWorkProfile();
    await ppm.portfolioWorkProfile({ period: "month", from: "2026-01-01" });
    await ppm.timeEntries("p1");
    await ppm.recordTime("p1", {
      actor_ref: "person:x",
      spent_on: "2026-11-18",
      minutes: 30,
    });
    await ppm.timeReport({ groupBy: "plan" });
    await ppm.timeReport({
      groupBy: "person",
      from: "2026-11-01",
      to: "2026-11-30",
      plan: "p1",
      person: "person:x",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/me/plan?day=2026-11-18&tz=Europe%2FLondon",
      "POST /api/me/planned-slots",
      "POST /api/me/planned-slots/s1/delete",
      "POST /api/me/feed-token",
      "POST /api/me/feed-token/revoke",
      "GET /api/me/watching",
      "GET /api/work-profile",
      "GET /api/work-profile?period=month&from=2026-01-01",
      "GET /api/plans/p1/time-entries",
      "POST /api/plans/p1/time-entries",
      "GET /api/time-reports?group_by=plan",
      "GET /api/time-reports?group_by=person&from=2026-11-01&to=2026-11-30&plan=p1&person=person%3Ax",
    ]);
    expect(ppm.timeReportCsvUrl({ groupBy: "month" })).toMatch(
      /\/api\/proxy\/api\/time-reports\?group_by=month&format=csv$/,
    );
  });
});

describe("PpmClient milestone sign-off required (NC-1 slice 11, EX-4)", () => {
  it("replaces the organizations a milestone needs", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.setSignoffRequired("p1", "m1", ["o1"]);
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual(["PUT /api/plans/p1/milestones/m1/signoff-required"]);
  });
});

describe("PpmClient triage and thresholds (NC-1 slice 11, EX-7)", () => {
  it("reads and sets triage and thresholds, sweeps, and records a threshold decision", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.triage("p1");
    await ppm.setTriage("p1", "fix", "A good long reason");
    await ppm.thresholds("p1");
    await ppm.setThreshold("p1", { kind: "forecast_past", date: "2027-01-01" });
    await ppm.thresholdSweep();
    await ppm.thresholdDecision("e1", {
      action: "pause",
      reason: "A good long reason",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/triage",
      "PUT /api/plans/p1/triage",
      "GET /api/plans/p1/thresholds",
      "PUT /api/plans/p1/thresholds",
      "POST /api/threshold-sweep",
      "POST /api/exceptions/e1/threshold-decision",
    ]);
  });
});

describe("PpmClient scorecards and de-escalation (NC-1 slice 11, EX-10)", () => {
  it("reads and writes scorecards, the template and the tracker", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.scorecards();
    await ppm.createScorecard({ name: "Outcomes", for_deescalation: true });
    await ppm.scorecard("s1");
    await ppm.scorecard("s1", 8);
    await ppm.setScorecardKpis("s1", ["k1"]);
    await ppm.scorecardTemplate();
    await ppm.scorecardFromTemplate("s1", [
      { key: "milestones_met_on_time", target_value: 90, red_value: 70 },
    ]);
    await ppm.deescalation();
    await ppm.setEscalationLevel({ level: "Level 3" });
    await ppm.addDeescalationCriterion("Independent review passed");
    await ppm.setCriterionEvidence("c1", {
      quarter: "2026-Q3",
      met: true,
      note: "Audit report signed",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/scorecards",
      "POST /api/scorecards",
      "GET /api/scorecards/s1",
      "GET /api/scorecards/s1?quarters=8",
      "PUT /api/scorecards/s1/kpis",
      "GET /api/scorecard-template",
      "POST /api/scorecards/s1/from-template",
      "GET /api/deescalation",
      "PUT /api/deescalation/level",
      "POST /api/deescalation/criteria",
      "PUT /api/deescalation/criteria/c1/evidence",
    ]);
  });
});

describe("PpmClient trade-off, phase and start check (NC-1 slice 11, EX-13, EX-8)", () => {
  it("names a trade-off, reads the phase and the start check, and moves the phase", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.setProposalTradeoff("p1", "Delays the CRM upgrade");
    await ppm.phaseHistory("g1");
    await ppm.startCheck("g1");
    await ppm.setPhase("g1", {
      phase: "executing",
      start_override_reason: "The board agreed",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "PUT /api/proposals/p1/tradeoff",
      "GET /api/plans/g1/phase-history",
      "GET /api/plans/g1/start-check",
      "PUT /api/plans/g1/phase",
    ]);
  });
});

describe("PpmClient cutover criteria (NC-1 slice 11, EX-6)", () => {
  it("replaces a draft's criteria", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.replaceCutoverCriteria("p1", "d1", ["Rollback rehearsed"]);
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual(["PUT /api/plans/p1/cutover-decisions/d1/criteria"]);
  });
});

describe("PpmClient delivery model and reporting method (NC-1 slice 11, EX-2, EX-14)", () => {
  it("reads and sets a programme's delivery model, and sets its reporting method", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.deliveryProfile("p1");
    await ppm.setDeliveryModel("p1", {
      delivery_model: null,
      delivery_owner_organization_pid: null,
    });
    await ppm.setReportingMethod("p1", "product");
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/delivery-model",
      "PUT /api/plans/p1/delivery-model",
      "PUT /api/plans/p1/reporting-method",
    ]);
  });
});

describe("PpmClient skill pool writes (NC-1 slice 11, DP-10)", () => {
  it("adds a pool and changes one", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.createSkillPool({
      name: "Data",
      roles: ["Data engineer"],
      skills: [],
    });
    await ppm.updateSkillPool("p1", { active: false });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual(["POST /api/skill-pools", "PUT /api/skill-pools/p1"]);
  });
});

describe("PpmClient scheduling, the rest (NC-1 slice 12 part i)", () => {
  it("reaches every route, defaults left out of the queries", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.hammocks("p1");
    await ppm.setHammock("p1", "t1", true);
    await ppm.linkHammock("p1", "h1", "t2");
    await ppm.unlinkHammock("p1", "h1", "t2");
    await ppm.dependencies("t1");
    await ppm.addDependency({ predecessor_pid: "t1", successor_pid: "x1" });
    await ppm.removeDependency("d1");
    await ppm.taskRollup("p1", "t1");
    await ppm.planFlowRollup("p1");
    await ppm.planFlowRollup("p1", 3);
    await ppm.planEffort("p1");
    await ppm.deliveryForecast("p1");
    await ppm.deliveryForecast("p1", { items: 20, period_days: 14 });
    await ppm.planPerformance("p1");
    await ppm.flowDistribution("p1");
    await ppm.flowDistribution("p1", {
      window_days: 30,
      rollup: true,
      depth: 0,
    });
    await ppm.planWorkflow("p1");
    await ppm.ccpmEstimates("p1");
    await ppm.setCcpmEstimate("p1", "t1", { aggressive_days: 2, safe_days: 4 });
    await ppm.bufferPlan("p1");
    await ppm.setWorkingTime({
      minutes_per_day: 450,
      working_days_per_week: 5,
    });
    await ppm.recordNonWorking({
      person_ref: "person:a",
      starts_on: "2026-12-21",
      ends_on: "2026-12-24",
      kind: "leave",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "GET /api/plans/p1/hammocks",
      "POST /api/plans/p1/tasks/t1/hammock",
      "POST /api/plans/p1/hammocks/h1/links",
      "DELETE /api/plans/p1/hammocks/h1/links/t2",
      "GET /api/dependencies?item=t1",
      "POST /api/dependencies",
      "DELETE /api/dependencies/d1",
      "GET /api/plans/p1/tasks/t1/rollup",
      "GET /api/plans/p1/rollup",
      "GET /api/plans/p1/rollup?depth=3",
      "GET /api/plans/p1/effort",
      "GET /api/plans/p1/forecast",
      "GET /api/plans/p1/forecast?items=20&period_days=14",
      "GET /api/plans/p1/performance",
      "GET /api/plans/p1/flow-distribution",
      "GET /api/plans/p1/flow-distribution?window_days=30&rollup=true&depth=0",
      "GET /api/plans/p1/workflow",
      "GET /api/plans/p1/ccpm-estimates?limit=1000",
      "PUT /api/plans/p1/tasks/t1/ccpm-estimate",
      "GET /api/plans/p1/buffer-plan",
      "POST /api/working-time",
      "POST /api/non-working",
    ]);
  });
});

describe("PpmClient the board, sprints, resources and the OKR and KPI reads (NC-1 slice 12 part j)", () => {
  it("reaches every route, defaults left out of the queries", async () => {
    const { ppm, calls } = recordingClient();
    await ppm.bulkEditTasks({ ids: ["t1"], set: { due_date: null } });
    await ppm.copyTasks({ ids: ["t1"], to_plan_pid: "p2" });
    await ppm.moveTasks({ ids: ["t1"], to_plan_pid: "p2" });
    await ppm.deleteTask("p1", "t1");
    await ppm.sprintCeremonies("s1");
    await ppm.holdCeremony("s1", { kind: "daily" });
    await ppm.sprintCommitment("s1");
    await ppm.commitSprint("s1");
    await ppm.utilization();
    await ppm.utilization({ by: "person", window_days: 90, plan_pid: "p1" });
    await ppm.plannedHours();
    await ppm.plannedHours({ from: "2026-11-01", to: "2026-11-28" });
    await ppm.skillGap("p1");
    await ppm.rateLookup({ person_ref: "person:a", date: "2026-11-18" });
    await ppm.resolvePlaceholder("p1", "a1", { person_ref: "person:a" });
    await ppm.removeAllocation("p1", "a1");
    await ppm.planOkr("p1");
    await ppm.objectiveKeyResults("o1");
    await ppm.keyResultCheckIns("k1");
    await ppm.kpiReadings("m1");
    await ppm.recordAdoption("p1", {
      active_users: 1,
      target_users: 2,
      window_days: 30,
      definition: "x",
    });
    expect(
      calls.map((x) => `${x.method} ${x.url.replace("http://svc", "")}`),
    ).toEqual([
      "POST /api/tasks/bulk",
      "POST /api/tasks/copy",
      "POST /api/tasks/move",
      "DELETE /api/plans/p1/tasks/t1",
      "GET /api/sprints/s1/ceremonies",
      "POST /api/sprints/s1/ceremonies",
      "GET /api/sprints/s1/commitment",
      "POST /api/sprints/s1/commit",
      "GET /api/capacity/utilization",
      "GET /api/capacity/utilization?by=person&window_days=90&plan_pid=p1",
      "GET /api/resource-plan/hours",
      "GET /api/resource-plan/hours?from=2026-11-01&to=2026-11-28",
      "GET /api/plans/p1/skill-gap",
      "GET /api/cost-rates/lookup?person_ref=person%3Aa&date=2026-11-18",
      "POST /api/plans/p1/allocations/a1/resolve",
      "DELETE /api/plans/p1/allocations/a1",
      "GET /api/plans/p1/okr",
      "GET /api/objectives/o1/key-results",
      "GET /api/key-results/k1/check-ins",
      "GET /api/kpis/m1/readings",
      "POST /api/plans/p1/adoption",
    ]);
    expect(ppm.liveUrl("p 1")).toMatch(/\/api\/plans\/p%201\/live$/);
  });
});

describe("money", () => {
  it("formats minor units as grouped major units", () => {
    expect(money(123_450, "GBP", "en-GB")).toBe("£1,234.50");
    expect(money(5, "EUR", "en-GB")).toBe("€0.05");
    expect(money(-99, "GBP", "en-GB")).toBe("-£0.99");
    expect(money(null, "GBP", "en-GB")).toBe("–");
  });
});
