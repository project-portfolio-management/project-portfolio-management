// Onboarding and offboarding: the words, symbols and form checks. Pure and tested.
// Status is a word and a shape, never colour alone.

import type {
  Closeout,
  OnboardingItem,
  OnboardingKind,
  OnboardingProgress,
} from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const KIND_TEXT: Record<OnboardingKind, string> = {
  get person_join() {
    return t("l.onboarding_view.someone_joins_this_plan");
  },
  get person_leave() {
    return t("l.onboarding_view.someone_leaves_this_plan");
  },
  get plan_intake() {
    return t("l.onboarding_view.take_this_plan_in");
  },
  get plan_closeout() {
    return t("l.onboarding_view.close_this_plan_out");
  },
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
  if (!isDate(str(date)))
    return { error: t("l.onboarding_view.give_the_date_it_takes") };
  if (isPersonKind(kind)) {
    if (!isPerson(str(subject)))
      return { error: t("l.onboarding_view.give_the_person_as_person") };
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
    return { error: t("l.onboarding_view.give_the_person_as_person") };
  if (blank(access))
    return { error: t("l.onboarding_view.say_what_access_in_words") };
  if (action !== "granted" && action !== "revoked")
    return { error: t("l.onboarding_view.choose_granted_or_revoked") };
  return { person_ref: str(person).trim(), access: str(access).trim(), action };
}

/** The handover form: who, and to whom (empty releases the work). */
export function handoverBody(
  from: FieldValue,
  to: FieldValue,
): { from_person: string; to_person: string | null } | { error: string } {
  if (!isPerson(str(from)))
    return { error: t("l.onboarding_view.give_who_is_handing_over") };
  if (!blank(to) && !isPerson(str(to)))
    return {
      error: t("l.onboarding_view.give_who_takes_over_as"),
    };
  if (!blank(to) && str(to).trim() === str(from).trim())
    return { error: t("l.onboarding_view.hand_over_to_someone_else") };
  return {
    from_person: str(from).trim(),
    to_person: blank(to) ? null : str(to).trim(),
  };
}

/** "3 of 6 done, 1 skipped, 2 overdue". */
export function progressText(p: OnboardingProgress): string {
  if (p.total === 0) return t("l.onboarding_view.no_items");
  const parts = [
    t("l.onboarding_view.of_done", { done: p.done, total: p.total }),
  ];
  if (p.skipped)
    parts.push(t("l.onboarding_view.skipped", { skipped: p.skipped }));
  if (p.overdue)
    parts.push(t("l.onboarding_view.overdue", { overdue: p.overdue }));
  return parts.join(", ");
}

/** A word and a shape for an item. */
export function itemState(i: Pick<OnboardingItem, "resolution" | "overdue">): {
  symbol: string;
  word: string;
} {
  if (i.resolution === "done")
    return { symbol: "●", word: t("p.plans.pid.onboarding.done") };
  if (i.resolution === "skipped")
    return { symbol: "○", word: t("l.onboarding_view.skipped_2") };
  return i.overdue
    ? { symbol: "■", word: t("l.onboarding_view.overdue_2") }
    : { symbol: "▲", word: t("c.MilestoneCard.open") };
}

/** One sentence for the close-out panel. */
export function closeoutText(c: Pick<Closeout, "ready" | "findings">): string {
  if (c.ready) return t("l.onboarding_view.nothing_is_left_open_this");
  return t("l.onboarding_view.not_ready_to_close", {
    join: c.findings.map((f) => f.text).join("; "),
  });
}

export const HOLDING_TEXT: Record<string, string> = {
  get task() {
    return t("p.plans.pid.work.task");
  },
  get allocation() {
    return t("l.onboarding_view.booking");
  },
  get raci() {
    return t("p.resources.role");
  },
  get risk() {
    return t("p.technology.risk");
  },
};

/** A sentence for what a preview would do. */
export function handoverText(
  action: "reassign" | "release",
  n: number,
  to: string | null,
): string {
  if (n === 0) return t("l.onboarding_view.this_person_holds_nothing_live");
  const one = n === 1;
  return action === "reassign"
    ? t(
        one
          ? "l.onboarding_view.would_move_one"
          : "l.onboarding_view.would_move_other",
        { n, to },
      )
    : t(
        one
          ? "l.onboarding_view.would_release_one"
          : "l.onboarding_view.would_release_other",
        { n },
      );
}
