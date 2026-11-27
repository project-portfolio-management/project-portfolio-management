// The approvals page's rules (PL-1; NC-1 slice 12 part n).

import { describe, expect, it } from "vitest";
import {
  MAX_TEXT,
  decideBody,
  isApproverRef,
  refusalKey,
  requestBody,
} from "../../src/lib/approvals-view";

const U = "3f2c6b7e-0000-4000-8000-000000000001";

describe("requestBody", () => {
  it("trims, and sends no note as null", () => {
    expect(
      requestBody({
        subject_kind: "plan",
        subject_pid: ` ${U} `,
        approver_ref: ` person:${U} `,
        message: " ",
      }),
    ).toEqual({
      body: {
        subject_kind: "plan",
        subject_pid: U,
        approver_ref: `person:${U}`,
        message: null,
      },
    });
  });
  it("says every problem at once", () => {
    expect(
      requestBody({
        subject_kind: "task",
        subject_pid: "abc",
        approver_ref: "person:ana",
        message: "x".repeat(MAX_TEXT + 1),
      }),
    ).toEqual({
      problems: [
        "p.approvals.subject_bad",
        "p.approvals.approver_bad",
        "p.approvals.text_long",
      ],
    });
  });
  it("takes a person or a worker, never an organisation", () => {
    expect(isApproverRef(`worker:${U}`)).toBe(true);
    expect(isApproverRef(`organization:${U}`)).toBe(false);
  });
});

describe("decideBody and refusalKey", () => {
  it("approves with or without a comment, and a rejection says why", () => {
    expect(decideBody("approved", " ")).toEqual({
      body: { decision: "approved" },
    });
    expect(decideBody("approved", " fine ")).toEqual({
      body: { decision: "approved", comment: "fine" },
    });
    expect(decideBody("rejected", "  ")).toEqual({
      problem: "p.approvals.why_required",
    });
    expect(decideBody("rejected", "over budget")).toEqual({
      body: { decision: "rejected", comment: "over budget" },
    });
    expect(decideBody("approved", "x".repeat(MAX_TEXT + 1))).toEqual({
      problem: "p.approvals.text_long",
    });
  });
  it("says a refusal in the page's words", () => {
    expect(refusalKey(409, "decide")).toBe("p.approvals.already_settled");
    expect(refusalKey(403, "decide")).toBe("p.approvals.not_approver");
    expect(refusalKey(403, "withdraw")).toBe("p.approvals.not_requester");
    expect(refusalKey(404, "request")).toBe("p.approvals.not_found");
    expect(refusalKey(500, "request")).toBeNull();
  });
});
