// NC-1 slice 2 (IR-4): the change-request form's checks and body, and the steps each status allows. The rules mirror
// the service's `src/change_requests.rs`; the service still decides.

import { describe, expect, it } from "vitest";
import type { ChangeRequest } from "../../src/lib/api/ppm-types";
import {
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
    expect(stepsFor("approved")).toEqual(["withdraw"]);
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
