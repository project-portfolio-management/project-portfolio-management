import { describe, expect, it } from "vitest";
import {
  emptyIssueForm,
  formFromIssue,
  issueBody,
  issueProblem,
} from "../../src/lib/issue-view";
import type { Issue } from "../../src/lib/api/ppm-types";

const ISSUE: Issue = {
  pid: "i1",
  plan_pid: "p1",
  title: "Build server is down",
  description: null,
  kind: "blocker",
  severity: "high",
  priority: null,
  status: "in_progress",
  owner_ref: "person:ada",
  raised_by: null,
  raised_on: "2026-09-10",
  due_date: "2026-09-12",
  resolution: null,
  resolved_on: null,
  age_calendar_days: 3,
  overdue: false,
};

describe("issue form", () => {
  it("needs a title", () => {
    expect(issueProblem(emptyIssueForm())).toBe("title_required");
    expect(issueProblem({ ...emptyIssueForm(), title: "   " })).toBe(
      "title_required",
    );
    expect(issueProblem({ ...emptyIssueForm(), title: "x" })).toBeNull();
  });

  it("needs a resolution to close, not to resolve", () => {
    const f = { ...emptyIssueForm(), title: "x", status: "closed" as const };
    expect(issueProblem(f)).toBe("resolution_required");
    expect(issueProblem({ ...f, resolution: "Fixed" })).toBeNull();
    expect(issueProblem({ ...f, status: "resolved" })).toBeNull();
  });

  it("refuses a due date before the day it was raised", () => {
    const f = {
      ...emptyIssueForm(),
      title: "x",
      raisedOn: "2026-09-10",
      dueDate: "2026-09-09",
    };
    expect(issueProblem(f)).toBe("due_before_raised");
    expect(issueProblem({ ...f, dueDate: "2026-09-10" })).toBeNull();
  });

  it("sends empty boxes as null and leaves raised_on out when blank", () => {
    const body = issueBody({ ...emptyIssueForm(), title: "  New  " });
    expect(body).toEqual({
      title: "New",
      description: null,
      kind: "concern",
      severity: "med",
      status: "open",
      owner_ref: null,
      due_date: null,
      resolution: null,
    });
    expect("raised_on" in body).toBe(false);
  });

  it("round-trips an issue through the form", () => {
    const body = issueBody(formFromIssue(ISSUE));
    expect(body.title).toBe("Build server is down");
    expect(body.raised_on).toBe("2026-09-10");
    expect(body.owner_ref).toBe("person:ada");
    expect(body.status).toBe("in_progress");
  });
});
