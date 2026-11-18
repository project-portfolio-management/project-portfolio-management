// A plan's lessons learned and what refers to a record (`/plans/[pid]/lessons`), and a proposal's lessons from similar
// plans (`/proposals`); NC-8, BR-4, NC-1 slice 12 part e. The service's rules (`controllers/lessons.rs` and the
// references route in `controllers/meetings.rs`), said before anything is sent. The service still decides.
//
// A lesson is recorded at a phase gate (G0 to G5) or at closure, as something that went well, went badly, or a
// recommendation, with a summary of up to 300 characters and an optional detail of up to 1024 bytes, and may point at
// a record of the plan. It is never changed: it can only be withdrawn (it is then no longer shown, and stays in the
// audit trail). What refers to a record lists the meetings and lessons that point at it. At intake a proposal's title
// is matched against every plan with lessons (closed or not), and each similar plan's lessons are shown with the
// match's score and confidence.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type {
  LessonBody,
  LessonCategory,
  ProposalLessonMatch,
} from "./api/ppm-types.js";
import { linkOf } from "./meeting-view.js";

/** The phase gates, in order (the service's `GATES`). */
export const GATES = [
  "g0_concept",
  "g1_feasibility",
  "g2_definition",
  "g3_delivery",
  "g4_launch",
  "g5_benefits",
] as const;
/** When a lesson is recorded: at a gate, or at closure. */
export const OCCASIONS = [...GATES, "closure"] as const;
/** What a lesson is, in the service's order. */
export const CATEGORIES: readonly LessonCategory[] = [
  "went_well",
  "went_badly",
  "recommendation",
];
/** The longest summary, in characters. */
export const SUMMARY_MAX = 300;
/** The longest detail, in bytes of UTF-8 (the service counts bytes). */
export const DETAIL_MAX_BYTES = 1024;

/** A category as a word's key and a shape: never a colour alone. */
export const CATEGORY_MARK: Record<
  LessonCategory,
  { symbol: string; key: StringKey }
> = {
  went_well: { symbol: "✓", key: "p.lessons.category_went_well" },
  went_badly: { symbol: "✗", key: "p.lessons.category_went_badly" },
  recommendation: { symbol: "→", key: "p.lessons.category_recommendation" },
};

/** Whether `occasion` is a gate. */
export const isGate = (occasion: string): boolean =>
  (GATES as readonly string[]).includes(occasion);

const bytes = (s: string) => new TextEncoder().encode(s).length;

/** What the lesson form holds. */
export interface LessonForm {
  occasion: string;
  category: LessonCategory | "";
  summary: string;
  detail: string;
  link: string;
}

/** The body for `POST …/lessons`, or every problem, in the service's order. A blank detail is not sent. */
export function lessonBody(
  form: LessonForm,
): { body: LessonBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  if (!(OCCASIONS as readonly string[]).includes(form.occasion))
    problems.push("p.lessons.problem_occasion");
  if (!(CATEGORIES as readonly string[]).includes(form.category))
    problems.push("p.lessons.problem_category");
  const summary = form.summary.trim();
  if (summary === "" || [...summary].length > SUMMARY_MAX)
    problems.push("p.lessons.problem_summary");
  const detail = form.detail.trim();
  if (bytes(detail) > DETAIL_MAX_BYTES)
    problems.push("p.lessons.problem_detail");
  if (problems.length > 0) return { problems };
  const body: LessonBody = {
    occasion: form.occasion,
    category: form.category as LessonCategory,
    summary,
  };
  if (detail !== "") body.detail = detail;
  const link = linkOf(form.link);
  if (link) body.link = link;
  return { body };
}

/** What a person does on these pages. */
export type LessonAction = "record" | "withdraw" | "references" | "proposal";

/**
 * The sentence for a refusal of `action`, or `null` for the general one. The service's codes are `not_found` and
 * `unprocessable`, one per status: a 404 on withdrawing is a lesson already withdrawn (or a plan gone), on the
 * references a record that is no longer one of the plan's (or one the caller cannot read), and on a proposal one that
 * is gone or hidden; a 422 on recording, after the page's own checks, is a linked record no longer of the plan.
 */
export function lessonRefusalKey(
  action: LessonAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404)
    return action === "withdraw"
      ? "p.lessons.refused_withdrawn"
      : action === "references"
        ? "p.lessons.refused_no_record"
        : action === "proposal"
          ? "p.lessons.refused_proposal_gone"
          : "p.lessons.refused_gone";
  if (err.status === 422 && action === "record")
    return "p.lessons.refused_link";
  return null;
}

/** A match's confidence as a word's key. */
export const CONFIDENCE_KEY: Record<
  ProposalLessonMatch["match"]["confidence"],
  StringKey
> = {
  High: "p.lessons.confidence_high",
  Medium: "p.lessons.confidence_medium",
  Low: "p.lessons.confidence_low",
};

/** A score in `[0, 1]` as a whole percentage, clamped. */
export const scorePercent = (score: number): number =>
  Math.round(
    Math.min(1, Math.max(0, Number.isFinite(score) ? score : 0)) * 100,
  );

/** The title's similarity from a match's breakdown as a whole percentage, or `null` when it was not compared. */
export function nameScore(m: ProposalLessonMatch["match"]): number | null {
  const v = m.breakdown?.name_score;
  return typeof v === "number" ? scorePercent(v) : null;
}
