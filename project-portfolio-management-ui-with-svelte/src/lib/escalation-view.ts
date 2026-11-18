// Escalations: the words, shapes and checks the escalation pages use. Pure and tested.
//
// A state is never conveyed by colour or shape alone: every state has a word and a shape, and "overdue" is said in
// words as well as shown.

import type {
  AcknowledgeEscalationBody,
  Escalation,
  EscalationAcknowledgement,
  EscalationResolution,
  EscalationStatus,
  ResolveEscalationBody,
} from "#lib/api/ppm.js";
import { ApiError } from "./api/client.js";
import { t, type StringKey } from "./i18n.svelte.js";
import { formatDateTime, formatInteger } from "./locale-format.js";

export const REASONS = [
  "tolerance",
  "risk",
  "issue",
  "dependency",
  "decision",
  "resource",
  "other",
] as const;
export const OUTCOMES = ["decided", "guidance_given", "rejected"] as const;
export const DIMENSIONS = [
  "time",
  "schedule",
  "cost",
  "risk",
  "resources",
] as const;
/** The shortest written response or reason (the service's rule). */
export const NOTE_MIN = 10;

export const STATUS_SYMBOL: Record<EscalationStatus, string> = {
  open: "◆",
  acknowledged: "◇",
  resolved: "✓",
  withdrawn: "✕",
};

export const STATUS_TEXT: Record<EscalationStatus, string> = {
  get open() {
    return t("l.escalation.open");
  },
  get acknowledged() {
    return t("l.escalation.acknowledged");
  },
  get resolved() {
    return t("l.escalation.resolved");
  },
  get withdrawn() {
    return t("l.escalation.withdrawn");
  },
};

/** Worst-first order for lists: live and overdue before the rest. */
export const STATUS_ORDER: readonly EscalationStatus[] = [
  "open",
  "acknowledged",
  "resolved",
  "withdrawn",
];

/** Whether an escalation still awaits an answer. */
export const isLive = (e: Pick<Escalation, "status">): boolean =>
  e.status === "open" || e.status === "acknowledged";

export function reasonLabel(r: string): string {
  const key = `l.escalation.reason_${r}`;
  const labels: Record<string, string> = {
    tolerance: t("l.escalation.reason_tolerance"),
    risk: t("l.escalation.reason_risk"),
    issue: t("l.escalation.reason_issue"),
    dependency: t("l.escalation.reason_dependency"),
    decision: t("l.escalation.reason_decision"),
    resource: t("l.escalation.reason_resource"),
    other: t("l.escalation.reason_other"),
  };
  return labels[r] ?? key;
}

export function outcomeLabel(o: string): string {
  const labels: Record<string, string> = {
    decided: t("l.escalation.outcome_decided"),
    guidance_given: t("l.escalation.outcome_guidance_given"),
    rejected: t("l.escalation.outcome_rejected"),
  };
  return labels[o] ?? o;
}

export function eventLabel(e: string): string {
  const labels: Record<string, string> = {
    raised: t("l.escalation.event_raised"),
    acknowledged: t("l.escalation.event_acknowledged"),
    level_raised: t("l.escalation.event_level_raised"),
    resolved: t("l.escalation.event_resolved"),
    withdrawn: t("l.escalation.event_withdrawn"),
  };
  return labels[e] ?? e;
}

/** What is wrong with a draft, in words; empty when it can be sent. */
export function raiseProblems(d: {
  title: string;
  reason_kind: string;
  tolerance_dimension: string;
  level: string;
  owner_ref: string;
}): string[] {
  const out: string[] = [];
  if (d.title.trim() === "" || [...d.title].length > 200)
    out.push(t("l.escalation.title_problem"));
  if (d.reason_kind === "tolerance" && d.tolerance_dimension === "")
    out.push(t("l.escalation.dimension_problem"));
  if (
    d.level.trim() !== "" &&
    !(
      Number.isInteger(Number(d.level)) &&
      Number(d.level) >= 1 &&
      Number(d.level) <= 9
    )
  )
    out.push(t("l.escalation.level_problem"));
  const o = d.owner_ref.trim();
  if (o !== "" && !/^(person|worker):\S+/.test(o))
    out.push(t("l.escalation.owner_problem"));
  return out;
}

/** Whether a note or response is long enough to count. */
export const noteOk = (s: string): boolean => [...s.trim()].length >= NOTE_MIN;

/** Who the service records as the raiser of an escalation raised by the `tolerance_escalations` task. */
export const AUTOMATIC_RAISER = "system:tolerance-escalations";

/** "Raised automatically" for one the scheduled task raised, else empty. */
export const automaticText = (e: Pick<Escalation, "raised_by">): string =>
  e.raised_by === AUTOMATIC_RAISER ? t("l.escalation.automatic") : "";

/** "Level 2: Sponsor" or just "Level 2". */
export function levelText(
  e: Pick<Escalation, "level" | "escalated_to">,
): string {
  return e.escalated_to
    ? t("l.escalation.level_to", { level: e.level, to: e.escalated_to })
    : t("l.escalation.level", { level: e.level });
}

/** "Open for 12 days" / "Resolved after 3 days", with the late flag in words. */
export function ageText(
  e: Pick<Escalation, "status" | "age_days" | "overdue">,
): string {
  const base = isLive(e)
    ? t("l.escalation.open_for", { n: e.age_days })
    : t("l.escalation.took", { n: e.age_days });
  return e.overdue ? `${base} — ${t("l.escalation.overdue")}` : base;
}

// ---- the acknowledgement and the resolution as records (NC-1 slice 12 part d; CF-4) ------------------------------

/** The longest written response to a resolution (the service's rule). */
export const RESPONSE_MAX = 10_000;

/**
 * Which records are worth reading for an escalation in `status`: an acknowledgement exists only once it has been
 * acknowledged at its current level (an open one has none, as taking it up a level clears it), and a resolution only
 * once it is resolved. Reading the others would only ever answer 404.
 */
export function recordsToRead(status: EscalationStatus): {
  acknowledgement: boolean;
  resolution: boolean;
} {
  return {
    acknowledgement: status !== "open",
    resolution: status === "resolved",
  };
}

/** The body for `PUT …/acknowledgement`: the note trimmed, left out when blank. */
export function acknowledgeBody(note: string): AcknowledgeEscalationBody {
  const n = note.trim();
  return n ? { note: n } : {};
}

/** What is wrong with a resolution before it is sent, in words; empty when it can be sent. */
export function resolutionProblems(
  outcome: string,
  response: string,
): string[] {
  const out: string[] = [];
  if (!(OUTCOMES as readonly string[]).includes(outcome))
    out.push(t("p.escalations.problem_outcome"));
  const n = [...response.trim()].length;
  if (n < NOTE_MIN || n > RESPONSE_MAX)
    out.push(
      t("p.escalations.problem_response", {
        min: formatInteger(NOTE_MIN),
        max: formatInteger(RESPONSE_MAX),
      }),
    );
  return out;
}

/** The body for `PUT …/resolution`, the response trimmed. */
export function resolveBody(
  outcome: string,
  response: string,
): ResolveEscalationBody {
  return { outcome, response: response.trim() };
}

/** What a person does with an escalation's records from this page. */
export type EscalationRecordAction = "acknowledge" | "resolve";

/**
 * The catalogue sentence for a refusal of `action`, by HTTP status: 403 names the right it needs, 409 that it has moved
 * on, 422 that the service did not accept what was written, 404 that it is gone. `null` for anything else, so the page
 * uses the general rule. The service's codes here are `forbidden`, `conflict`, `unprocessable` and `not_found`, one per
 * status, so the status says which; where one status covers two refusals the sentence covers both.
 */
export function recordRefusalKey(
  action: EscalationRecordAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  switch (err.status) {
    case 403:
      return action === "acknowledge"
        ? "p.escalations.refused_ack_right"
        : "p.escalations.refused_resolve_right";
    case 409:
      return action === "acknowledge"
        ? "p.escalations.refused_ack_state"
        : "p.escalations.refused_resolve_state";
    case 422:
      return action === "resolve"
        ? "p.escalations.refused_resolve_invalid"
        : null;
    case 404:
      return "p.escalations.refused_gone";
    default:
      return null;
  }
}

/** Whether a failed read of a record means "there is none yet" (404) rather than a failure to report. */
export const isNoRecord = (err: unknown): boolean =>
  err instanceof ApiError && err.status === 404;

/** Who, as a reference, or the catalogue's "not recorded". */
const who = (ref: string | null | undefined): string =>
  ref?.trim() ? ref : t("p.escalations.by_unrecorded");

/** "Acknowledged at level 2 by person:ana, 18 Nov 2026, 09:15." */
export function acknowledgementText(a: EscalationAcknowledgement): string {
  return t("p.escalations.ack_record", {
    level: a.level,
    by: who(a.acknowledged_by),
    at: formatDateTime(a.acknowledged_at),
  });
}

/** "Resolved 18 Nov 2026, 09:15 by person:ana: Decided." */
export function resolutionText(r: EscalationResolution): string {
  return t("p.escalations.resolution_record", {
    at: formatDateTime(r.resolved_at ?? r.resolved_on),
    by: who(r.resolved_by),
    outcome: r.outcome
      ? outcomeLabel(r.outcome)
      : t("p.escalations.outcome_unrecorded"),
  });
}
