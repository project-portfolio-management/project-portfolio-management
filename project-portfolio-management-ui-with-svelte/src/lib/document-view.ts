// Controlled documents (PL-6): the words and the one next step for each state. Pure
// and tested. A status is a word and a shape, never colour alone, and each state
// names exactly one primary action.

import type {
  ControlledDocument,
  DocumentStatus,
  DocumentVersion,
} from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export const STATUS_TEXT: Record<DocumentStatus, string> = {
  get draft() { return t("l.document_view.draft"); },
  get in_review() { return t("l.document_view.in_review"); },
  get approved() { return t("l.document_view.approved_not_yet_in_force"); },
  get effective() { return t("p.documents.in_force"); },
  get superseded() { return t("l.document_view.replaced"); },
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
  get send_for_review() { return t("l.document_view.send_for_review"); },
  get decide() { return t("l.document_view.approve"); },
  get make_effective() { return t("l.document_view.bring_into_force_today"); },
  get start_next_draft() { return t("l.document_view.start_the_next_draft"); },
};

export const STEP_TITLE: Record<Exclude<NextAction, "none">, string> = {
  get send_for_review() { return t("p.plans.pid.governance.review"); },
  get decide() { return t("l.document_view.waiting_for_the_approver"); },
  get make_effective() { return t("l.document_view.approved"); },
  get start_next_draft() { return t("l.document_view.change_this_document"); },
};

export const STEP_HELP: Record<Exclude<NextAction, "none">, string> = {
  get send_for_review() { return t("l.document_view.someone_else_approves_it_you"); },
  get decide() { return t("l.document_view.only_the_person_asked_can"); },
  get make_effective() { return t("l.document_view.it_comes_into_force_today"); },
  get start_next_draft() { return t("l.document_view.the_next_draft_starts_as"); },
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
  const base = t("l.document_view.version", { version: v.version, v: STATUS_TEXT[v.status] });
  if (v.status === "effective" && v.effective_from)
    return t("l.document_view.since", { base, effective_from: v.effective_from });
  if (v.status === "superseded" && v.superseded_on) {
    const next = all.find((x) => x.pid === v.superseded_by);
    return t("l.document_view.on", { base, superseded_on: v.superseded_on, v: next ? t("l.document_view.by_version", { version: next.version }) : "" });
  }
  return base;
}

/** One line for the list: which version is in force, or that none is. */
export function forceText(doc: ControlledDocument): string {
  if (doc.in_force_today === null) return t("l.document_view.nothing_in_force_yet");
  return t("l.document_view.version_in_force", { in_force_today: doc.in_force_today });
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
