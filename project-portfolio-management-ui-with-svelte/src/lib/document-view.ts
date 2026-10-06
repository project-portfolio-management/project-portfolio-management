// Controlled documents (PL-6): the words and the one next step for each state. Pure
// and tested. A status is a word and a shape, never colour alone, and each state
// names exactly one primary action.

import type {
  ControlledDocument,
  DocumentStatus,
  DocumentVersion,
} from "#lib/api/ppm.js";

export const STATUS_TEXT: Record<DocumentStatus, string> = {
  draft: "Draft",
  in_review: "In review",
  approved: "Approved, not yet in force",
  effective: "In force",
  superseded: "Replaced",
};

export const STATUS_SYMBOL: Record<DocumentStatus, string> = {
  draft: "✎",
  in_review: "◔",
  approved: "✓",
  effective: "●",
  superseded: "○",
};

/** The one thing to do next with a version. */
export type NextAction =
  "send_for_review" | "decide" | "make_effective" | "start_next_draft" | "none";

export const ACTION_LABEL: Record<Exclude<NextAction, "none">, string> = {
  send_for_review: "Send for review",
  decide: "Approve",
  make_effective: "Bring into force today",
  start_next_draft: "Start the next draft",
};

export const STEP_TITLE: Record<Exclude<NextAction, "none">, string> = {
  send_for_review: "Review",
  decide: "Waiting for the approver",
  make_effective: "Approved",
  start_next_draft: "Change this document",
};

export const STEP_HELP: Record<Exclude<NextAction, "none">, string> = {
  send_for_review:
    "Someone else approves it: you cannot approve your own document.",
  decide: "Only the person asked can decide.",
  make_effective:
    "It comes into force today and replaces the version now in force.",
  start_next_draft: "The next draft starts as a copy of the latest text.",
};

/** The version still being worked on (draft, in review or approved), if any: at most one. */
export function inFlight(doc: ControlledDocument): DocumentVersion | null {
  return (
    doc.versions.find(
      (v) =>
        v.status === "draft" ||
        v.status === "in_review" ||
        v.status === "approved",
    ) ?? null
  );
}

/** What the document needs next: finish the version in flight, or start another. */
export function nextAction(doc: ControlledDocument): NextAction {
  const v = inFlight(doc);
  if (!v) return doc.versions.length > 0 ? "start_next_draft" : "none";
  if (v.status === "draft") return "send_for_review";
  if (v.status === "in_review") return "decide";
  return "make_effective";
}

/** "Version 3 — In force since 2026-10-06", "Version 2 — Replaced on 2026-10-06 by version 3". */
export function versionLine(
  v: DocumentVersion,
  all: readonly DocumentVersion[] = [],
): string {
  const base = `Version ${v.version} — ${STATUS_TEXT[v.status]}`;
  if (v.status === "effective" && v.effective_from)
    return `${base} since ${v.effective_from}`;
  if (v.status === "superseded" && v.superseded_on) {
    const next = all.find((x) => x.pid === v.superseded_by);
    return `${base} on ${v.superseded_on}${next ? ` by version ${next.version}` : ""}`;
  }
  return base;
}

/** One line for the list: which version is in force, or that none is. */
export function forceText(doc: ControlledDocument): string {
  if (doc.in_force_today === null) return "Nothing in force yet";
  return `Version ${doc.in_force_today} in force`;
}

/** Whether the text of a version may still change. */
export function isEditable(v: DocumentVersion): boolean {
  return v.status === "draft";
}

/** The approver the form sends: a person URN, built from a bare id or accepted as given. */
export function approverRef(input: string): string | null {
  const t = input.trim();
  if (/^(person|worker):[0-9a-f-]{36}$/i.test(t)) return t;
  if (/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(t))
    return `person:${t}`;
  return null;
}
