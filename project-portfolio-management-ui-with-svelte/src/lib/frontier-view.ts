// Choosing a portfolio (`/scenarios`, NC-1 slice 12 part g, NC-7): the efficient frontier (`GET
// /api/scenarios/frontier`), a generated draft (`POST /api/scenarios/generate`) and rolling a committed scenario back
// (`POST /api/scenarios/{pid}/rollback`). The service's rules (`controllers/strategy.rs`, `domain/src/strategy.rs`),
// said before anything is sent. The service still decides.
//
// The generator scores every plan the reader can read by its Smart Score and prices it by its budget lines in the
// chosen currency, then takes plans greedily by score per unit of cost within the cap; plans that must be included
// are taken first, whatever they cost. A plan with no Smart Score is left out, never scored zero; one whose budget
// lines are all in another currency cannot be priced and is left out under a cap; a plan with no budget lines costs
// nothing. The frontier re-runs it at each cap (given, up to 50, or 2 to 50 spread evenly from nothing to the cost of
// every priceable plan) and saves nothing; each point is what a single generated draft at that cap would hold. The
// greedy choice is not optimal, so a point that buys no more value than a cheaper cap is marked, not hidden. A
// rollback makes a committed scenario a draft again; the commitment's record is kept in the audit trail.

import { ApiError } from "./api/client.js";
import type {
  FrontierPoint,
  GenerateScenarioBody,
  ScenarioChoice,
  ScenarioExcludeReason,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { currencyCode, toMinor } from "./money.js";

/** How many caps the frontier spreads: 2 to 50, 10 when none is given (not sent). */
export const STEPS_MIN = 2;
export const STEPS_MAX = 50;
export const STEPS_DEFAULT = 10;
/** The most caps, and the most plans that must be included, one request may name. */
export const LIST_MAX = 50;
/** The longest scenario name, in bytes of UTF-8 (`validation::MAX_TEXT_LEN`). */
export const NAME_MAX_BYTES = 1024;

/** Why the generator left a plan out, in words. */
export const REASON_KEY: Record<ScenarioExcludeReason, StringKey> = {
  no_score: "p.frontier.reason_no_score",
  foreign_currency: "p.frontier.reason_foreign_currency",
  over_cap: "p.frontier.reason_over_cap",
  must_include_conflict: "p.frontier.reason_must_include_conflict",
};

/** The words for a reason the service sent; one this page does not know is a general sentence. */
export function reasonKey(reason: string | null | undefined): StringKey {
  return (
    (reason && (REASON_KEY as Record<string, StringKey>)[reason]) ||
    "p.frontier.reason_other"
  );
}

/** The frontier's request as typed: a currency, and either a number of evenly spread caps (a number input's value)
 * or caps named one per line in the reader's number format. */
export interface FrontierForm {
  currency: string;
  how: "steps" | "caps";
  steps: number | null | undefined;
  caps: string;
  mustInclude: string[];
}

/** The frontier's query, or every problem with it. Ten steps is the service's default and is not sent. */
export function frontierQuery(
  form: FrontierForm,
  locale?: string,
):
  | {
      query: {
        currency: string;
        caps?: number[];
        steps?: number;
        mustInclude?: string[];
      };
    }
  | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const currency = currencyCode(form.currency);
  if (currency === null) problems.push("p.frontier.problem_currency");
  let caps: number[] | undefined;
  let steps: number | undefined;
  if (form.how === "caps") {
    const lines = form.caps
      .split(/\r?\n/)
      .map((l) => l.trim())
      .filter(Boolean);
    if (lines.length === 0) problems.push("p.frontier.problem_caps_none");
    else if (lines.length > LIST_MAX)
      problems.push("p.frontier.problem_caps_many");
    else if (currency !== null) {
      const minor = lines.map((l) => toMinor(l, currency, locale));
      if (minor.some((m) => m === null || m < 0 || !Number.isSafeInteger(m)))
        problems.push("p.frontier.problem_caps_amount");
      else caps = minor as number[];
    }
  } else if (form.steps !== null && form.steps !== undefined) {
    const n = Number(form.steps);
    if (!Number.isInteger(n) || n < STEPS_MIN || n > STEPS_MAX)
      problems.push("p.frontier.problem_steps");
    else if (n !== STEPS_DEFAULT) steps = n;
  }
  if (form.mustInclude.length > LIST_MAX)
    problems.push("p.frontier.problem_must_include");
  if (problems.length > 0) return { problems };
  return {
    query: {
      currency: currency!,
      ...(caps ? { caps } : {}),
      ...(steps !== undefined ? { steps } : {}),
      ...(form.mustInclude.length > 0
        ? { mustInclude: [...form.mustInclude] }
        : {}),
    },
  };
}

/** A generated draft as typed: a name (needed here, so the list never carries the service's English default name),
 * an optional cap in the reader's number format and its currency, and the plans it must include. */
export interface GenerateForm {
  name: string;
  cap: string;
  currency: string;
  mustInclude: string[];
}

const bytes = (s: string) => new TextEncoder().encode(s).length;

/** The body to generate a draft, or every problem with it. A currency without a cap is sent, so each plan is priced;
 * a cap needs its currency. */
export function generateBody(
  form: GenerateForm,
  locale?: string,
): { body: GenerateScenarioBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const name = form.name.trim();
  if (!name) problems.push("p.frontier.problem_name");
  else if (bytes(name) > NAME_MAX_BYTES)
    problems.push("p.frontier.problem_name_long");
  const typedCurrency = form.currency.trim();
  const currency = typedCurrency ? currencyCode(typedCurrency) : null;
  if (typedCurrency && currency === null)
    problems.push("p.frontier.problem_currency");
  let cap: number | undefined;
  if (form.cap.trim()) {
    if (!typedCurrency) problems.push("p.frontier.problem_cap_currency");
    else if (currency !== null) {
      const m = toMinor(form.cap, currency, locale);
      if (m === null || m < 0 || !Number.isSafeInteger(m))
        problems.push("p.frontier.problem_cap_amount");
      else cap = m;
    }
  }
  if (form.mustInclude.length > LIST_MAX)
    problems.push("p.frontier.problem_must_include");
  if (problems.length > 0) return { problems };
  return {
    body: {
      name,
      ...(cap !== undefined ? { budget_cap_minor: cap } : {}),
      ...(currency ? { currency } : {}),
      ...(form.mustInclude.length > 0
        ? { must_include: [...form.mustInclude] }
        : {}),
    },
  };
}

/** The body to save one frontier point as a draft: the same cap, currency and plans that must be included. */
export function pointBody(
  name: string,
  point: Pick<FrontierPoint, "budget_cap_minor">,
  currency: string,
  mustInclude: readonly string[],
): GenerateScenarioBody {
  return {
    name,
    budget_cap_minor: point.budget_cap_minor,
    currency,
    ...(mustInclude.length > 0 ? { must_include: [...mustInclude] } : {}),
  };
}

/** A choice's rows, included first and then by name; a plan the reader has no name for is said to be unknown. */
export function choiceRows(
  rationale: readonly ScenarioChoice[],
  names: ReadonlyMap<string, string>,
): (ScenarioChoice & { name: string | null })[] {
  return rationale
    .map((r) => ({ ...r, name: names.get(r.pid) ?? null }))
    .sort(
      (a, b) =>
        Number(b.included) - Number(a.included) ||
        (a.name ?? "￿").localeCompare(b.name ?? "￿"),
    );
}

/** The chart's box, in SVG units. */
export const FRONTIER_BOX = {
  width: 360,
  height: 220,
  left: 48,
  right: 12,
  top: 12,
  bottom: 36,
} as const;

/** Each point placed on the chart: cost across, value up, both from zero to the largest. */
export function frontierPlot(
  points: readonly FrontierPoint[],
  box = FRONTIER_BOX,
): { x: number; y: number; point: FrontierPoint }[] {
  const maxCost = Math.max(1, ...points.map((p) => p.cost_minor));
  const maxValue = Math.max(1, ...points.map((p) => p.value));
  const w = box.width - box.left - box.right;
  const h = box.height - box.top - box.bottom;
  return points.map((point) => ({
    x: box.left + (Math.max(0, point.cost_minor) / maxCost) * w,
    y: box.top + h - (Math.max(0, point.value) / maxValue) * h,
    point,
  }));
}

/** The line through the points that buy more value, cheapest first. */
export function frontierLine(
  placed: readonly { x: number; y: number; point: FrontierPoint }[],
): string {
  return placed
    .filter((p) => !p.point.dominated)
    .sort((a, b) => a.point.cost_minor - b.point.cost_minor)
    .map((p) => `${Math.round(p.x * 10) / 10},${Math.round(p.y * 10) / 10}`)
    .join(" ");
}

/** The point with the most value (the cheapest of equals), or `null` for no points. */
export function bestPoint(
  points: readonly FrontierPoint[],
): FrontierPoint | null {
  let best: FrontierPoint | null = null;
  for (const p of points)
    if (
      best === null ||
      p.value > best.value ||
      (p.value === best.value && p.cost_minor < best.cost_minor)
    )
      best = p;
  return best;
}

/** The sentence for a refused frontier or draft, or `null` for the generic one: every rule is checked first, so a
 * 422 says only that the request was refused (the service's text is never shown). */
export function frontierRefusalKey(
  action: "frontier" | "generate",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status !== 422) return null;
  return action === "frontier"
    ? "p.frontier.refused_frontier"
    : "p.frontier.refused_generate";
}

/** The sentence for a refused rollback, or `null` for the generic one: a 409 is a scenario no longer committed. */
export function rollbackRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.frontier.refused_rollback_gone";
  if (err.status === 409) return "p.frontier.refused_rollback_status";
  return null;
}
