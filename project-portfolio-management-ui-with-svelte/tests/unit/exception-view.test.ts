// NC-1 slice 11 (DP-3): the delivery exceptions register: what a completion sends (only what changed, never a
// blank), the checks before sending, and the sentence for each refusal.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { DeliveryException } from "../../src/lib/api/ppm-types";
import {
  MISSING_LABELS,
  completeBody,
  completeProblems,
  decisionProblem,
  exceptionRefusalKey,
  formOf,
} from "../../src/lib/exception-view";

const exc = (over: Partial<DeliveryException> = {}): DeliveryException => ({
  pid: "e1",
  plan_pid: "p1",
  status: "open",
  opened_at: "2026-11-16T09:00:00Z",
  opened_value: "amber_red",
  worst_value: "amber_red",
  worsened_count: 0,
  cause: "Supplier late",
  decision_needed: null,
  owner_ref: null,
  owner_role: null,
  decide_by: null,
  completed_at: null,
  complete: false,
  missing: ["decision_needed", "owner", "decide_by"],
  breached: false,
  window_hours: 48,
  hours_open: 20,
  closed_at: null,
  decision: null,
  decided_by: null,
  ...over,
});

describe("completing an exception", () => {
  it("sends only the parts that changed, trimmed, and nothing when nothing did", () => {
    const e = exc();
    expect(completeBody(formOf(e), e)).toBeNull();
    expect(
      completeBody(
        {
          ...formOf(e),
          decisionNeeded: "  Fund a second supplier? ",
          decideBy: "2026-11-20",
        },
        e,
      ),
    ).toEqual({
      decision_needed: "Fund a second supplier?",
      decide_by: "2026-11-20",
    });
  });
  it("never sends a blank, which the service would keep as it was", () => {
    const e = exc();
    expect(completeBody({ ...formOf(e), cause: "   " }, e)).toBeNull();
  });
  it("says a part of one or two characters does not count, as the service judges it", () => {
    expect(completeProblems({ ...formOf(exc()), ownerRef: "ab" })).toEqual([
      "p.exceptions.problem_part_short",
    ]);
    expect(completeProblems({ ...formOf(exc()), ownerRef: "" })).toEqual([]);
  });
  it("names every part the service may say is missing", () => {
    expect(Object.keys(MISSING_LABELS)).toEqual([
      "cause",
      "decision_needed",
      "owner",
      "decide_by",
    ]);
  });
});

describe("deciding an exception", () => {
  it("needs a decision of ten characters once trimmed", () => {
    expect(decisionProblem("  Stop it ")).toBe("p.exceptions.problem_decision");
    expect(decisionProblem("Fund the second supplier.")).toBeNull();
  });
  it("reads a 422 on a decision as one a threshold opened, and a 409 as already closed", () => {
    expect(exceptionRefusalKey("decide", new ApiError(422, "x"))).toBe(
      "p.exceptions.refused_threshold",
    );
    expect(exceptionRefusalKey("complete", new ApiError(422, "x"))).toBeNull();
    expect(exceptionRefusalKey("complete", new ApiError(409, "x"))).toBe(
      "p.exceptions.conflict_closed",
    );
  });
});
