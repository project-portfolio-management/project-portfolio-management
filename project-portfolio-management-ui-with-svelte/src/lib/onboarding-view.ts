// Onboarding and offboarding: the words, symbols and form checks. Pure and tested.
// Status is a word and a shape, never colour alone.

import type {
  Closeout,
  OnboardingItem,
  OnboardingKind,
  OnboardingProgress,
} from "#lib/api/ppm.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const KIND_TEXT: Record<OnboardingKind, string> = {
  person_join: "Someone joins this plan",
  person_leave: "Someone leaves this plan",
  plan_intake: "Take this plan in",
  plan_closeout: "Close this plan out",
};

export const isPersonKind = (k: OnboardingKind) =>
  k === "person_join" || k === "person_leave";

const isDate = (s: string) => /^\d{4}-\d{2}-\d{2}$/.test(s);
const isPerson = (s: string) => /^(person|worker):.+/.test(s.trim());

/** The request to start a case, or the first thing wrong with the form. */
export function caseBody(
  kind: OnboardingKind,
  subject: FieldValue,
  date: FieldValue,
):
  | { kind: OnboardingKind; subject_ref?: string; effective_date: string }
  | { error: string } {
  if (!isDate(str(date))) return { error: "Give the date it takes effect." };
  if (isPersonKind(kind)) {
    if (!isPerson(str(subject)))
      return { error: "Give the person as person:… (or worker:…)." };
    return {
      kind,
      subject_ref: str(subject).trim(),
      effective_date: str(date),
    };
  }
  return { kind, effective_date: str(date) };
}

/** The request to record access, or the first thing wrong. */
export function accessBody(
  person: FieldValue,
  access: FieldValue,
  action: FieldValue,
):
  | { person_ref: string; access: string; action: "granted" | "revoked" }
  | { error: string } {
  if (!isPerson(str(person)))
    return { error: "Give the person as person:… (or worker:…)." };
  if (blank(access)) return { error: "Say what access, in words." };
  if (action !== "granted" && action !== "revoked")
    return { error: "Choose granted or revoked." };
  return { person_ref: str(person).trim(), access: str(access).trim(), action };
}

/** The handover form: who, and to whom (empty releases the work). */
export function handoverBody(
  from: FieldValue,
  to: FieldValue,
): { from_person: string; to_person: string | null } | { error: string } {
  if (!isPerson(str(from)))
    return { error: "Give who is handing over, as person:…" };
  if (!blank(to) && !isPerson(str(to)))
    return {
      error:
        "Give who takes over as person:… (or leave it empty to release the work).",
    };
  if (!blank(to) && str(to).trim() === str(from).trim())
    return { error: "Hand over to someone else." };
  return {
    from_person: str(from).trim(),
    to_person: blank(to) ? null : str(to).trim(),
  };
}

/** "3 of 6 done, 1 skipped, 2 overdue". */
export function progressText(p: OnboardingProgress): string {
  if (p.total === 0) return "No items";
  const parts = [`${p.done} of ${p.total} done`];
  if (p.skipped) parts.push(`${p.skipped} skipped`);
  if (p.overdue) parts.push(`${p.overdue} overdue`);
  return parts.join(", ");
}

/** A word and a shape for an item. */
export function itemState(i: Pick<OnboardingItem, "resolution" | "overdue">): {
  symbol: string;
  word: string;
} {
  if (i.resolution === "done") return { symbol: "●", word: "Done" };
  if (i.resolution === "skipped") return { symbol: "○", word: "Skipped" };
  return i.overdue
    ? { symbol: "■", word: "Overdue" }
    : { symbol: "▲", word: "Open" };
}

/** One sentence for the close-out panel. */
export function closeoutText(c: Pick<Closeout, "ready" | "findings">): string {
  if (c.ready) return "Nothing is left open. This plan is ready to close.";
  return `Not ready to close: ${c.findings.map((f) => f.text).join("; ")}.`;
}

export const HOLDING_TEXT: Record<string, string> = {
  task: "Task",
  allocation: "Booking",
  raci: "Role",
  risk: "Risk",
};

/** A sentence for what a preview would do. */
export function handoverText(
  action: "reassign" | "release",
  n: number,
  to: string | null,
): string {
  if (n === 0) return "This person holds nothing live on this plan.";
  return action === "reassign"
    ? `${n} item${n === 1 ? "" : "s"} would move to ${to}.`
    : `${n} item${n === 1 ? "" : "s"} would be released: tasks and risks unassigned, bookings and roles removed.`;
}
