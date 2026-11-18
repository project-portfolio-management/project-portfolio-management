// The intake's forecast and editing a draft (`/proposals`, NC-1 slice 12 part g, EX-13): how many proposals will be
// approved in the next periods (`GET /api/proposals/forecast`), and a draft read as stored and edited (`GET`/`PUT
// /api/proposals/{pid}`), which is also where an existing proposal is tagged as an external ask. The service's rules
// (`controllers/governance.rs`, `src/tba/forecast.rs`), said before anything is sent. The service still decides.
//
// The forecast samples the intake's own approvals per period (from the audit trail), the same throughput simulation a
// plan's board forecast runs; proposals arriving are shown beside it and do not feed it. Periods ahead are 1 to 520, a
// period 1 to 90 days and the history 1 to 260 periods; a forecast needs at least 6 periods of history. The low figure
// is the one to plan on: at least that many, with 85% confidence. Only a draft can be edited; the edit replaces every
// field, so the fields this page does not show are sent back as stored. A title is needed; a title and a summary are at
// most 1024 bytes; an amount needs its currency; a named trade-off is 10 to 2000 characters.

import { ApiError } from "./api/client.js";
import type { CreateProposalBody, Proposal } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { currencyCode, toMinor } from "./money.js";
import { tradeoffProblem } from "./tradeoff-view.js";

/** The forecast's bounds and the service's defaults (left out of the query). */
export const PERIODS = { min: 1, max: 520, fallback: 4 } as const;
export const PERIOD_DAYS = { min: 1, max: 90, fallback: 7 } as const;
export const HISTORY = { min: 1, max: 260, fallback: 12 } as const;
/** The fewest periods of history a forecast is drawn from (`MIN_THROUGHPUT_PERIODS`). */
export const HISTORY_NEEDED = 6;
/** The longest title or summary, in bytes of UTF-8 (`validation::MAX_TEXT_LEN`; the service counts bytes). */
export const TEXT_MAX_BYTES = 1024;

/** The forecast as asked: number inputs' values (`undefined` or `null` when empty, which is the default). */
export interface ForecastForm {
  periods: number | null | undefined;
  periodDays: number | null | undefined;
  historyPeriods: number | null | undefined;
}

/** The forecast's query, or every problem with it. A default is not sent. */
export function forecastQuery(form: ForecastForm):
  | {
      query: { periods?: number; periodDays?: number; historyPeriods?: number };
    }
  | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const query: {
    periods?: number;
    periodDays?: number;
    historyPeriods?: number;
  } = {};
  const check = (
    v: number | null | undefined,
    bound: { min: number; max: number; fallback: number },
    problem: StringKey,
    set: (n: number) => void,
  ) => {
    if (v === null || v === undefined) return;
    const n = Number(v);
    if (!Number.isInteger(n) || n < bound.min || n > bound.max)
      problems.push(problem);
    else if (n !== bound.fallback) set(n);
  };
  check(form.periods, PERIODS, "p.intake.problem_periods", (n) => {
    query.periods = n;
  });
  check(form.periodDays, PERIOD_DAYS, "p.intake.problem_period_days", (n) => {
    query.periodDays = n;
  });
  check(form.historyPeriods, HISTORY, "p.intake.problem_history", (n) => {
    query.historyPeriods = n;
  });
  return problems.length > 0 ? { problems } : { query };
}

/** Why the forecast has no figures, in words (the service's `reason` is English and never shown): too little history,
 * or a general sentence. */
export function forecastAbsentKey(historyPeriods: number): StringKey {
  return historyPeriods < HISTORY_NEEDED
    ? "p.intake.absent_history"
    : "p.intake.absent_other";
}

/** The sentence for a refused forecast, or `null` for the generic one: the bounds are checked first. */
export function forecastRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  return err.status === 422 ? "p.intake.refused_forecast" : null;
}

/** A draft as typed. The amount is in the reader's number format. */
export interface ProposalEditForm {
  title: string;
  summary: string;
  kindTarget: string;
  requested: string;
  currency: string;
  externalAsk: boolean;
  tradeoff: string;
}

/** The form filled from the stored proposal; `toInput` writes an amount for a text input. */
export function proposalEditFormOf(
  p: Proposal,
  toInput: (minor: number, currency: string) => string,
): ProposalEditForm {
  return {
    title: p.title,
    summary: p.summary ?? "",
    kindTarget: p.kind_target,
    requested:
      p.requested_minor !== null && p.currency
        ? toInput(p.requested_minor, p.currency)
        : "",
    currency: p.currency ?? "",
    externalAsk: p.external_ask,
    tradeoff: p.tradeoff ?? "",
  };
}

const bytes = (s: string) => new TextEncoder().encode(s).length;

/** The body the stored proposal would send: the comparison for "nothing changed". */
function storedBody(p: Proposal): CreateProposalBody {
  const summary = (p.summary ?? "").trim();
  const tradeoff = (p.tradeoff ?? "").trim();
  return {
    title: p.title.trim(),
    kind_target: p.kind_target,
    ...(summary ? { summary } : {}),
    ...(p.sponsor_ref ? { sponsor_ref: p.sponsor_ref } : {}),
    ...(p.strategic_rationale
      ? { strategic_rationale: p.strategic_rationale }
      : {}),
    ...(p.requested_minor !== null && p.currency
      ? { requested_minor: p.requested_minor, currency: p.currency }
      : {}),
    external_ask: p.external_ask,
    ...(tradeoff ? { tradeoff } : {}),
  };
}

const canonical = (b: CreateProposalBody) =>
  JSON.stringify(
    Object.entries(b).sort(([x], [y]) => (x < y ? -1 : x > y ? 1 : 0)),
  );

/** The body for `PUT /api/proposals/{pid}`, every problem at once, or `null` when nothing changed (nothing is sent).
 * The sponsor and strategic rationale, which this page does not show, are sent back as stored; a blank summary or
 * trade-off is left out (cleared); an amount is sent with its currency. A trade-off is offered only on an external
 * ask; untagging one keeps what it named. */
export function proposalEditBody(
  p: Proposal,
  form: ProposalEditForm,
  locale?: string,
): { body: CreateProposalBody } | { problems: StringKey[] } | null {
  if (p.status !== "draft") return { problems: ["p.intake.problem_not_draft"] };
  const problems: StringKey[] = [];
  const title = form.title.trim();
  if (!title) problems.push("p.proposals.problem_title");
  else if (bytes(title) > TEXT_MAX_BYTES)
    problems.push("p.intake.problem_title_long");
  const summary = form.summary.trim();
  if (bytes(summary) > TEXT_MAX_BYTES)
    problems.push("p.intake.problem_summary_long");
  const tradeoff = form.externalAsk
    ? form.tradeoff.trim()
    : (p.tradeoff ?? "").trim();
  if (form.externalAsk && tradeoff) {
    const why = tradeoffProblem(tradeoff);
    if (why) problems.push(why);
  }
  let money: { requested_minor: number; currency: string } | null = null;
  if (form.requested.trim()) {
    const code = currencyCode(form.currency);
    const m = code === null ? null : toMinor(form.requested, code, locale);
    if (code === null || m === null || m < 0 || !Number.isSafeInteger(m))
      problems.push("p.money.amount_or_currency");
    else money = { requested_minor: m, currency: code };
  }
  if (problems.length > 0) return { problems };
  const body: CreateProposalBody = {
    title,
    kind_target: form.kindTarget,
    ...(summary ? { summary } : {}),
    ...(p.sponsor_ref ? { sponsor_ref: p.sponsor_ref } : {}),
    ...(p.strategic_rationale
      ? { strategic_rationale: p.strategic_rationale }
      : {}),
    ...(money ?? {}),
    external_ask: form.externalAsk,
    ...(tradeoff ? { tradeoff } : {}),
  };
  return canonical(body) === canonical(storedBody(p)) ? null : { body };
}

/** The sentence for a refused edit, or `null` for the generic one. The two 422s have a code each: `unprocessable`
 * (the proposal is no longer a draft) and `validation` (a field out of its bounds, checked first). */
export function proposalEditRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.proposals.refused_gone";
  if (err.status !== 422) return null;
  const code = (err.body as { code?: unknown } | undefined)?.code;
  if (code === "unprocessable") return "p.intake.refused_not_draft";
  return "p.intake.refused_edit";
}
