// The issue form (NC-1, IR-2): turning what a person typed into the request body, and checking the
// rules the service would otherwise refuse (a title; a closed issue says how it was resolved; a due
// date not before the day it was raised). The service decides; this only saves a round trip and lets
// the page say which field is wrong, in the reader's language.

import type { Issue, IssueBody } from "./api/ppm-types.js";

export const ISSUE_KINDS = [
  "bug",
  "risk",
  "blocker",
  "question",
  "improvement",
  "concern",
] as const;
export const ISSUE_SEVERITIES = ["low", "med", "high", "critical"] as const;
export const ISSUE_STATUSES = [
  "open",
  "in_progress",
  "resolved",
  "closed",
] as const;

/** What the form holds: every field a string, so an empty box is "". */
export interface IssueForm {
  title: string;
  description: string;
  kind: Issue["kind"];
  severity: Issue["severity"];
  status: Issue["status"];
  ownerRef: string;
  raisedOn: string;
  dueDate: string;
  resolution: string;
}

/** The first thing wrong with the form, or `null` when it can be sent. */
export type IssueProblem =
  "title_required" | "resolution_required" | "due_before_raised" | null;

export function emptyIssueForm(): IssueForm {
  return {
    title: "",
    description: "",
    kind: "concern",
    severity: "med",
    status: "open",
    ownerRef: "",
    raisedOn: "",
    dueDate: "",
    resolution: "",
  };
}

/** The form for editing `issue`. */
export function formFromIssue(issue: Issue): IssueForm {
  return {
    title: issue.title,
    description: issue.description ?? "",
    kind: issue.kind,
    severity: issue.severity,
    status: issue.status,
    ownerRef: issue.owner_ref ?? "",
    raisedOn: issue.raised_on,
    dueDate: issue.due_date ?? "",
    resolution: issue.resolution ?? "",
  };
}

export function issueProblem(f: IssueForm): IssueProblem {
  if (f.title.trim() === "") return "title_required";
  if (f.status === "closed" && f.resolution.trim() === "")
    return "resolution_required";
  // ISO dates compare as text.
  if (f.dueDate !== "" && f.raisedOn !== "" && f.dueDate < f.raisedOn)
    return "due_before_raised";
  return null;
}

const orNull = (s: string): string | null =>
  s.trim() === "" ? null : s.trim();

/** The request body for the form. An empty `raisedOn` is left out so the service uses today. */
export function issueBody(f: IssueForm): IssueBody {
  const body: IssueBody = {
    title: f.title.trim(),
    description: orNull(f.description),
    kind: f.kind,
    severity: f.severity,
    status: f.status,
    owner_ref: orNull(f.ownerRef),
    due_date: orNull(f.dueDate),
    resolution: orNull(f.resolution),
  };
  if (f.raisedOn !== "") body.raised_on = f.raisedOn;
  return body;
}
