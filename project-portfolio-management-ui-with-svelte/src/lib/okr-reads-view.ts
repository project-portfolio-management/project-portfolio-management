// The OKR and KPI reads and adoption (`/okrs`, `/kpis`, `/plans/[pid]/benefits`; NC-1 slice 12 part j, KO-1, KO-2):
// a plan's alignment-weighted score (`GET /api/plans/{pid}/okr`) with each objective's key results (`GET
// /api/objectives/{pid}/key-results`), a key result's check-ins (`GET /api/key-results/{pid}/check-ins`), every
// reading of a KPI (`GET /api/kpis/{pid}/readings`), and how many people use what a plan delivered (`POST
// /api/plans/{pid}/adoption`). The service's rules (`controllers/okr.rs`, `controllers/kpis.rs`,
// `controllers/value.rs`), said and checked first; the service still decides.
//
// Every score is derived on read. An objective with no measurable key result is not measured and is left out of the
// plan's score: it neither drags it down nor lifts it; a plan whose objectives are all unmeasured is not measured,
// never 0%. A key result has no progress when its start equals its target, when it is held steady with no tolerance,
// or when the sums are too large. Confidence is recorded beside a check-in and never part of a score. A KPI's later
// reading on the same day replaces an earlier one, which is kept and marked. Adoption is active users against the
// users intended over a window, with what counted as active stored beside it so later readings stay comparable; the
// target is more than zero, the active users not negative, the window at least a day. Every adoption refusal is one
// 422 `unprocessable` with English text.

import { ApiError } from "./api/client.js";
import type { AdoptionBody } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** Why a key result has no progress, in words. */
export const UNMEASURABLE_KEY: Record<string, StringKey> = {
  no_range: "p.planokr.unmeasurable_no_range",
  no_tolerance: "p.planokr.unmeasurable_no_tolerance",
  overflow: "p.planokr.unmeasurable_overflow",
};

/** Why a key result has no progress; a reason the page does not know is a general one. */
export function unmeasurableKey(reason: string | null): StringKey {
  return UNMEASURABLE_KEY[reason ?? ""] ?? "p.planokr.unmeasurable_other";
}

/** The objectives that count toward a plan's score and those left out (not measured). */
export function scoredSplit<T extends { measured: boolean }>(
  objectives: readonly T[],
): { counted: T[]; left_out: T[] } {
  return {
    counted: objectives.filter((o) => o.measured),
    left_out: objectives.filter((o) => !o.measured),
  };
}

/** The catalogue sentence for a refused plan score or key-result read, or `null`. */
export function okrReadRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  return err.status === 404 ? "p.planokr.refused_gone" : null;
}

// ---- adoption ----

/** The adoption form as typed (number inputs bind numbers once typed in). */
export interface AdoptionForm {
  active: number | null | undefined;
  target: number | null | undefined;
  windowDays: number | null | undefined;
  definition: string;
}

export function emptyAdoptionForm(): AdoptionForm {
  return { active: null, target: null, windowDays: 30, definition: "" };
}

/** The largest window the service stores, in days. */
export const ADOPTION_MAX_WINDOW = 2_147_483_647;

const whole = (v: number | null | undefined): number | null =>
  v === null || v === undefined || Number.isNaN(Number(v)) ? null : Number(v);

/** The body to send or every problem at once. */
export function adoptionBody(
  form: AdoptionForm,
): { body: AdoptionBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const active = whole(form.active);
  const target = whole(form.target);
  const days = whole(form.windowDays);
  if (active === null || !Number.isSafeInteger(active) || active < 0)
    problems.push("p.adoption.problem_active");
  if (target === null || !Number.isSafeInteger(target) || target <= 0)
    problems.push("p.adoption.problem_target");
  if (
    days === null ||
    !Number.isSafeInteger(days) ||
    days <= 0 ||
    days > ADOPTION_MAX_WINDOW
  )
    problems.push("p.adoption.problem_window");
  const definition = form.definition.trim();
  if (!definition) problems.push("p.adoption.problem_definition");
  if (problems.length > 0) return { problems };
  return {
    body: {
      active_users: active as number,
      target_users: target as number,
      window_days: days as number,
      definition,
    },
  };
}

/** The catalogue sentence for a refused adoption, or `null`. */
export function adoptionRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 422) return "p.adoption.refused";
  if (err.status === 404) return "p.adoption.refused_gone";
  if (err.status === 403) return "p.adoption.refused_forbidden";
  return null;
}
