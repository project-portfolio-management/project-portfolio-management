// The issue form (NC-1, IR-2): turning what a person typed into the request body, and checking the
// rules the service would otherwise refuse (a title; a closed issue says how it was resolved; a due
// date not before the day it was raised). The service decides; this only saves a round trip and lets
// the page say which field is wrong, in the reader's language.

import type {
  Issue,
  IssueBody,
  IssueOptions,
  IssueState,
  IssueType,
  IssueWorkflow,
} from "./api/ppm-types.js";
import { enumLabel } from "./enum-labels.js";

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

/** The first thing wrong. `needsResolution` is whether the status is terminal in the issue's workflow. */
export function issueProblem(
  f: IssueForm,
  needsResolution: boolean = f.status === "closed",
): IssueProblem {
  if (f.title.trim() === "") return "title_required";
  if (needsResolution && f.resolution.trim() === "")
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

// ---- organization-defined types and statuses (OC-1) ------------------------------------------------------------

/** The built-in workflow, for when the options cannot be read: the form then offers what it always did. */
const BUILT_IN_WORKFLOW: IssueWorkflow = {
  states: [
    {
      key: "open",
      label: "Open",
      category: "todo",
      is_initial: true,
      is_terminal: false,
    },
    {
      key: "in_progress",
      label: "In progress",
      category: "active",
      is_initial: false,
      is_terminal: false,
    },
    {
      key: "resolved",
      label: "Resolved",
      category: "done",
      is_initial: false,
      is_terminal: false,
    },
    {
      key: "closed",
      label: "Closed",
      category: "done",
      is_initial: false,
      is_terminal: true,
    },
  ],
  transitions: [],
};

/** The labels the service seeds. A built-in left as it was is translated; one the organization renamed is theirs. */
const SEEDED_TYPE_LABELS: Record<string, string> = {
  bug: "Bug",
  risk: "Risk",
  blocker: "Blocker",
  question: "Question",
  improvement: "Improvement",
  concern: "Concern",
};

/** A key as words: `with_supplier` reads "With supplier". For a status with no label to hand. */
export function humanize(key: string): string {
  const words = key.replace(/[_-]+/g, " ").trim();
  return words.charAt(0).toUpperCase() + words.slice(1);
}

/** Options the page can rely on, or `null`. A failed or malformed answer falls back to the built-ins. */
export function usableOptions(value: unknown): IssueOptions | null {
  const types = (value as IssueOptions | null)?.types;
  return Array.isArray(types) && types.length > 0
    ? (value as IssueOptions)
    : null;
}

/** The workflow in force for a type. */
export function workflowFor(
  options: IssueOptions | null,
  kind: string,
): IssueWorkflow {
  return (
    options?.types.find((t) => t.key === kind)?.workflow ?? BUILT_IN_WORKFLOW
  );
}

/** A type's name in the reader's language. */
export function typeLabel(
  types: readonly IssueType[] | null | undefined,
  key: string,
): string {
  const found = types?.find((t) => t.key === key);
  if (!found)
    return key in SEEDED_TYPE_LABELS
      ? enumLabel("issue_kind", key)
      : humanize(key);
  return found.label === SEEDED_TYPE_LABELS[key]
    ? enumLabel("issue_kind", key)
    : found.label;
}

/** A status's name in the reader's language. Without a workflow to hand, a key the catalogue does not know is shown as words. */
export function statusLabel(
  workflow: IssueWorkflow | null,
  key: string,
): string {
  const seeded = BUILT_IN_WORKFLOW.states.find((s) => s.key === key);
  const found = workflow?.states.find((s) => s.key === key);
  if (seeded && (!found || found.label === seeded.label))
    return enumLabel("issue_status", key);
  return found?.label ?? humanize(key);
}

/** The types a person can choose: those not retired, and the one the issue already has. */
export function typeChoices(
  options: IssueOptions | null,
  current: string,
): IssueType[] {
  const types =
    options?.types ??
    ISSUE_KINDS.map((key, position) => ({
      key,
      label: SEEDED_TYPE_LABELS[key] ?? key,
      position,
      retired: false,
    }));
  return types.filter((t) => !t.retired || t.key === current);
}

/** The status a new issue of this type starts in. */
export function initialStatus(workflow: IssueWorkflow): string {
  return (
    workflow.states.find((s) => s.is_initial)?.key ??
    workflow.states[0]?.key ??
    "open"
  );
}

/**
 * The statuses to offer. A new issue (`from` null) may start anywhere only where moves are unconstrained. An existing
 * one keeps its own status and may go where the workflow lets this caller go, so the form does not offer a move the
 * service would refuse. The service still decides.
 */
export function statusChoices(
  workflow: IssueWorkflow,
  from: string | null,
  callerRoles: readonly string[],
): IssueState[] {
  const constrained = workflow.transitions.length > 0;
  if (from === null) {
    const start = initialStatus(workflow);
    return workflow.states.filter((s) => !constrained || s.key === start);
  }
  const declared = workflow.states.some((s) => s.key === from);
  const offered = workflow.states.filter((s) => {
    if (s.key === from || !constrained || !declared) return true;
    return workflow.transitions.some(
      (t) =>
        t.from === from &&
        t.to === s.key &&
        (t.roles.length === 0 || t.roles.some((r) => callerRoles.includes(r))),
    );
  });
  if (declared) return offered;
  // A status its workflow no longer declares stays selectable until the issue moves on.
  return [
    {
      key: from,
      label: humanize(from),
      category: "todo",
      is_initial: false,
      is_terminal: false,
    },
    ...offered,
  ];
}

/** Whether the status is terminal, so the issue needs a resolution. */
export function needsResolution(
  workflow: IssueWorkflow,
  status: string,
): boolean {
  return (
    workflow.states.find((s) => s.key === status)?.is_terminal ??
    status === "closed"
  );
}
