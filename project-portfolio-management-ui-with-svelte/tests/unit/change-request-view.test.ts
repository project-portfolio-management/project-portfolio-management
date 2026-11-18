// NC-1 slice 2 (IR-4): the change-request form's checks and body, and the steps each status allows. The rules mirror
// the service's `src/change_requests.rs`; the service still decides.

import { describe, expect, it } from "vitest";
import type {
  ChangeRequest,
  ChangeRequestPreview,
  Issue,
} from "../../src/lib/api/ppm-types";
import {
  LINKS_MAX,
  answeredElsewhere,
  canBaseline,
  canRelink,
  forecastView,
  linksBody,
  linksProblem,
  approverProblem,
  changeRequestBody,
  changeRequestProblem,
  decisionReasonShort,
  emptyChangeRequestForm,
  formFromChangeRequest,
  identifier,
  refusalKey,
  stepsFor,
} from "../../src/lib/change-request-view";

const filled = () => ({
  ...emptyChangeRequestForm(),
  title: " Move go-live ",
  description: "Two weeks later",
  reason: "Supplier slipped",
  kind: "schedule" as const,
});

const cr = (over: Partial<ChangeRequest> = {}): ChangeRequest => ({
  pid: "c1",
  plan_pid: "p1",
  title: "Move go-live",
  description: "Two weeks later",
  reason: "Supplier slipped",
  kind: "schedule",
  impact: {
    schedule_business_days: 10,
    cost_minor: 150000,
    cost_currency: "GBP",
    summary: null,
    has_forecast_edits: false,
  },
  status: "draft",
  requested_by: "alice",
  requested_on: "2026-11-01",
  age_calendar_days: 9,
  approver_ref: null,
  decided_by: null,
  decided_on: null,
  decision_reason: null,
  ...over,
});

describe("changeRequestProblem", () => {
  it("accepts a filled form with no impact", () => {
    expect(changeRequestProblem(filled(), "en-GB")).toBeNull();
  });
  it("needs a title, a description and a reason, in that order", () => {
    expect(changeRequestProblem({ ...emptyChangeRequestForm() }, "en-GB")).toBe(
      "title_required",
    );
    expect(
      changeRequestProblem({ ...filled(), description: " " }, "en-GB"),
    ).toBe("description_required");
    expect(changeRequestProblem({ ...filled(), reason: "" }, "en-GB")).toBe(
      "reason_required",
    );
  });
  it("takes a whole number of business days within ±3650, negative allowed", () => {
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "-5" }, "en-GB"),
    ).toBeNull();
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "3650" }, "en-GB"),
    ).toBeNull();
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "3651" }, "en-GB"),
    ).toBe("days_out_of_range");
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "-3651" }, "en-GB"),
    ).toBe("days_out_of_range");
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "2.5" }, "en-GB"),
    ).toBe("days_not_whole");
    expect(
      changeRequestProblem({ ...filled(), scheduleDays: "ten" }, "en-GB"),
    ).toBe("days_not_whole");
  });
  it("needs a cost and its currency together, the currency a three-letter code", () => {
    expect(
      changeRequestProblem({ ...filled(), costAmount: "1500" }, "en-GB"),
    ).toBe("cost_needs_currency");
    expect(
      changeRequestProblem({ ...filled(), currency: "GBP" }, "en-GB"),
    ).toBe("currency_needs_cost");
    expect(
      changeRequestProblem(
        { ...filled(), costAmount: "1500", currency: "pounds" },
        "en-GB",
      ),
    ).toBe("currency_invalid");
    expect(
      changeRequestProblem(
        { ...filled(), costAmount: "lots", currency: "GBP" },
        "en-GB",
      ),
    ).toBe("cost_not_amount");
    expect(
      changeRequestProblem(
        { ...filled(), costAmount: "1,500.50", currency: "gbp" },
        "en-GB",
      ),
    ).toBeNull();
  });
});

describe("changeRequestBody", () => {
  it("sends every field, trimmed, with an empty impact as null rather than zero", () => {
    expect(changeRequestBody(filled(), null, "en-GB")).toEqual({
      title: "Move go-live",
      description: "Two weeks later",
      reason: "Supplier slipped",
      kind: "schedule",
      impact_schedule_days: null,
      impact_cost_minor: null,
      cost_currency: null,
      impact_summary: null,
      edits: null,
    });
  });
  it("puts the cost in the currency's own minor unit", () => {
    const f = {
      ...filled(),
      scheduleDays: "-3",
      costAmount: "1,500.50",
      currency: "gbp",
    };
    expect(changeRequestBody(f, null, "en-GB")).toMatchObject({
      impact_schedule_days: -3,
      impact_cost_minor: 150050,
      cost_currency: "GBP",
    });
    expect(
      changeRequestBody(
        { ...f, costAmount: "1500", currency: "JPY" },
        null,
        "en-GB",
      ),
    ).toMatchObject({ impact_cost_minor: 1500, cost_currency: "JPY" });
  });
  it("sends back the what-if edits already on the request, so an edit keeps them", () => {
    const edits = [{ op: "shift", task: "t1", days: 2 }];
    expect(changeRequestBody(filled(), edits, "en-GB").edits).toEqual(edits);
    expect(changeRequestBody(filled(), [], "en-GB").edits).toBeNull();
  });
  it("round-trips a change request through the form", () => {
    const f = formFromChangeRequest(cr(), "en-GB");
    expect(f).toMatchObject({
      scheduleDays: "10",
      costAmount: "1500.00",
      currency: "GBP",
    });
    expect(changeRequestBody(f, null, "en-GB")).toMatchObject({
      impact_schedule_days: 10,
      impact_cost_minor: 150000,
      cost_currency: "GBP",
    });
    const none = formFromChangeRequest(
      cr({
        impact: {
          schedule_business_days: null,
          cost_minor: null,
          cost_currency: null,
          summary: "Small",
          has_forecast_edits: false,
        },
      }),
      "en-GB",
    );
    expect(none).toMatchObject({
      scheduleDays: "",
      costAmount: "",
      currency: "",
      summary: "Small",
    });
  });
});

describe("stepsFor (the service's state machine)", () => {
  it("offers each status only the steps the service allows from it", () => {
    expect(stepsFor("draft")).toEqual(["edit", "submit", "withdraw"]);
    expect(stepsFor("submitted")).toEqual(["review", "decide", "withdraw"]);
    expect(stepsFor("under_review")).toEqual(["decide", "withdraw"]);
    expect(stepsFor("approved")).toEqual(["implement", "withdraw"]);
    for (const s of ["rejected", "implemented", "withdrawn"] as const)
      expect(stepsFor(s), s).toEqual([]);
  });
});

describe("the approver and the decision", () => {
  it("names the approver as a person: or worker: reference who is not the requester", () => {
    expect(approverProblem("person:bob", "alice")).toBeNull();
    expect(approverProblem(" worker:w9 ", "alice")).toBeNull();
    expect(approverProblem("bob", "alice")).toBe("approver_not_reference");
    expect(approverProblem("person:", "alice")).toBe("approver_not_reference");
    expect(approverProblem("person:Alice", "alice")).toBe(
      "approver_is_requester",
    );
    expect(approverProblem("person:alice", "person:alice")).toBe(
      "approver_is_requester",
    );
    expect(identifier("worker: w9 ")).toBe("w9");
  });
  it("needs a decision reason of at least 10 characters, counted after trimming", () => {
    expect(decisionReasonShort("too short")).toBe(true);
    expect(decisionReasonShort("  123456789  ")).toBe(true);
    expect(decisionReasonShort("1234567890")).toBe(false);
  });
});

describe("refusalKey", () => {
  it("says who may take a step on a 403, and the state or the input on a 409 or 422", () => {
    expect(refusalKey("submit", 403)).toBe(
      "p.plan_change_requests.refused_requester",
    );
    expect(refusalKey("edit", 403)).toBe(
      "p.plan_change_requests.refused_requester",
    );
    expect(refusalKey("decide", 403)).toBe(
      "p.plan_change_requests.refused_approver",
    );
    expect(refusalKey("review", 403)).toBe(
      "p.plan_change_requests.refused_approver",
    );
    expect(refusalKey("withdraw", 403)).toBe(
      "p.plan_change_requests.refused_withdraw",
    );
    expect(refusalKey("decide", 409)).toBe(
      "p.plan_change_requests.refused_state",
    );
    expect(refusalKey("draft", 422)).toBe(
      "p.plan_change_requests.refused_invalid",
    );
    expect(refusalKey("draft", 403)).toBeNull();
    expect(refusalKey("submit", 500)).toBeNull();
  });
});

// NC-1 slice 12 part (c): the links, the forecast and the baseline (IR-4, IR-5).
describe("refusals of the actions on one request", () => {
  it("says each by route and status; a 404 is a request that is gone", () => {
    expect(refusalKey("implement", 409)).toBe(
      "p.plan_change_requests.refused_state",
    );
    expect(refusalKey("links", 409)).toBe(
      "p.plan_change_requests.refused_links_conflict",
    );
    expect(refusalKey("links", 422)).toBe(
      "p.plan_change_requests.refused_links_invalid",
    );
    expect(refusalKey("links", 403)).toBeNull();
    expect(refusalKey("preview", 422)).toBe(
      "p.plan_change_requests.refused_preview",
    );
    expect(refusalKey("preview", 409)).toBeNull();
    expect(refusalKey("baseline", 409)).toBe(
      "p.plan_change_requests.refused_baseline",
    );
    expect(refusalKey("baseline", 422)).toBeNull();
    expect(refusalKey("read", 409)).toBeNull();
    // A 404 on drafting is the plan: the general rule.
    expect(refusalKey("draft", 404)).toBeNull();
    for (const a of [
      "read",
      "links",
      "preview",
      "baseline",
      "implement",
      "submit",
    ] as const)
      expect(refusalKey(a, 404), a).toBe("p.plan_change_requests.refused_gone");
  });
});

describe("what a request can still do", () => {
  const cr = (
    status: ChangeRequest["status"],
    baseline_pid: string | null = null,
  ) => ({ status, baseline_pid }) as ChangeRequest;
  it("is relinked only while open", () => {
    for (const s of ["draft", "submitted", "under_review", "approved"] as const)
      expect(canRelink(s), s).toBe(true);
    for (const s of ["rejected", "implemented", "withdrawn"] as const)
      expect(canRelink(s), s).toBe(false);
  });
  it("is the reason for one baseline, once approved or implemented", () => {
    expect(canBaseline(cr("approved"))).toBe(true);
    expect(canBaseline(cr("implemented"))).toBe(true);
    expect(canBaseline(cr("approved", "b1"))).toBe(false);
    for (const s of [
      "draft",
      "submitted",
      "under_review",
      "rejected",
      "withdrawn",
    ] as const)
      expect(canBaseline(cr(s)), s).toBe(false);
  });
});

describe("the issues and risks a request answers", () => {
  const issue = (pid: string, change_request_pid: string | null = null) =>
    ({ pid, title: `Issue ${pid}`, change_request_pid }) as Issue;
  const issues = [issue("i1"), issue("i2", "c1"), issue("i3", "c9")];

  it("knows an issue answered by another request", () => {
    expect(answeredElsewhere(issues[0]!, "c1")).toBe(false);
    expect(answeredElsewhere(issues[1]!, "c1")).toBe(false);
    expect(answeredElsewhere(issues[2]!, "c1")).toBe(true);
  });

  it("refuses an issue answered elsewhere, naming it, and more than the limit", () => {
    expect(linksProblem(["i1", "i2"], ["r1"], issues, "c1")).toBeNull();
    expect(linksProblem(["i3"], [], issues, "c1")).toEqual({
      key: "p.plan_change_requests.problem_links_elsewhere",
      title: "Issue i3",
    });
    const many = Array.from({ length: LINKS_MAX + 1 }, (_, n) => `r${n}`);
    expect(linksProblem([], many, issues, "c1")).toEqual({
      key: "p.plan_change_requests.problem_links_too_many",
    });
    expect(linksProblem([], many.slice(1), issues, "c1")).toBeNull();
  });

  it("sends the whole sets, once each, and nothing for the sets it has", () => {
    const current = { issue_pids: ["i2"], risk_pids: ["r1", "r2"] };
    expect(linksBody(current, ["i2"], ["r2", "r1"])).toBeNull();
    expect(linksBody({ issue_pids: null, risk_pids: null }, [], [])).toBeNull();
    expect(linksBody(current, ["i2", "i1", "i1"], ["r1"])).toEqual({
      issue_pids: ["i2", "i1"],
      risk_pids: ["r1"],
    });
    expect(linksBody(current, [], [])).toEqual({
      issue_pids: [],
      risk_pids: [],
    });
  });
});

describe("forecastView", () => {
  const preview = (over: Record<string, unknown>): ChangeRequestPreview =>
    ({
      change_request_pid: "c1",
      label: "service text",
      note: "service text",
      stated_impact: {
        schedule_business_days: 10,
        cost_minor: null,
        cost_currency: null,
      },
      forecast: {
        finish: { before: "2026-12-01", after: "2026-12-15" },
        tasks: [
          { pid: "t1", title: "Order", before: null, after: null },
          { pid: "t2", title: "", before: null, after: null },
        ],
        moved: [
          { pid: "t1", moved: "later", start_days: 2, finish_days: 2 },
          { pid: "t2", moved: "earlier", start_days: -5, finish_days: -5 },
          { pid: "t3", moved: "none", start_days: 1, finish_days: 0 },
        ],
        cost_impact: { available: false, reason: "no_finance_access" },
        ...over,
      },
    }) as unknown as ChangeRequestPreview;

  it("lists the tasks whose finish moves, furthest first, with their own titles", () => {
    const v = forecastView(preview({}));
    expect(v.finishBefore).toBe("2026-12-01");
    expect(v.finishAfter).toBe("2026-12-15");
    expect(v.moved).toEqual([
      { pid: "t2", title: null, finishDays: -5 },
      { pid: "t1", title: "Order", finishDays: 2 },
    ]);
    expect(v.labour).toEqual({ shown: false, why: "no_finance_access" });
  });

  it("gives the labour cost's change per currency, or why there is none", () => {
    const shown = forecastView(
      preview({
        cost_impact: {
          available: true,
          labour: [
            {
              currency: "GBP",
              before_minor: 100,
              after_minor: 250,
              delta_minor: 150,
            },
          ],
          unrated_minutes_before: 0,
          unrated_minutes_after: 0,
        },
      }),
    );
    expect(shown.labour).toEqual({
      shown: true,
      deltas: [{ currency: "GBP", deltaMinor: 150 }],
    });
    expect(
      forecastView(
        preview({
          cost_impact: { available: false, reason: "no_costed_work" },
        }),
      ).labour,
    ).toEqual({
      shown: false,
      why: "no_costed_work",
    });
    expect(
      forecastView(
        preview({ cost_impact: { available: false, reason: "service text" } }),
      ).labour,
    ).toEqual({
      shown: false,
      why: "other",
    });
    expect(
      forecastView(preview({ finish: { before: null, after: null } }))
        .finishAfter,
    ).toBeNull();
  });
});
