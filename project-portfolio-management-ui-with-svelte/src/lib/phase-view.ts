// A plan's phase and a programme's start (FR-30, EX-8, NC-1 slice 11 part m): the service's rules
// (`domain/src/phase.rs`, `domain/src/start_control.rs`, `controllers/phase.rs` and `controllers/start_control.rs`),
// said before anything is sent. The service still decides. A move forward is one step at a time; a move back needs a
// reason; a programme (kind Program) moving into executing meets the start check (the limit on programmes in active
// delivery and the constraint pool), and a refused start goes ahead only with an override reason of 10 or more
// characters. The refusal's 422 (`start_refused`) carries its reasons and the queue place as fields (the problem's
// `errors` member, NC-1 slice 12 part a), so the page words the refusal from them and never shows the English `detail`.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { PhaseToken, StartCheck, StartRefusal } from "./api/ppm.js";

/** The phases, in the service's order. */
export const PHASES: readonly PhaseToken[] = [
  "initiating",
  "planning",
  "executing",
  "controlling",
  "closing",
];
/** The shortest start override reason, in characters after trimming. */
export const OVERRIDE_MIN = 10;

const chars = (s: string) => [...s.trim()].length;

/** The place of `phase` in the order; -1 for a plan with no phase yet. */
export function phaseIndex(phase: PhaseToken | null): number {
  return phase === null ? -1 : PHASES.indexOf(phase);
}

/** The moves the service allows from `current`: one step forward (none after closing) and every earlier phase, the
 * nearest first (each needs a reason). */
export function phaseChoices(current: PhaseToken | null): {
  forward: PhaseToken | null;
  back: PhaseToken[];
} {
  const i = phaseIndex(current);
  return {
    forward: PHASES[i + 1] ?? null,
    back: i <= 0 ? [] : PHASES.slice(0, i).reverse(),
  };
}

/** Whether a move from `current` to `to` goes back. */
export function isBack(current: PhaseToken | null, to: PhaseToken): boolean {
  return phaseIndex(to) < phaseIndex(current);
}

/** Whether a move of this plan into `to` meets the start check: a programme moving into executing. */
export function meetsStartCheck(
  isProgramme: boolean,
  to: PhaseToken | "",
): boolean {
  return isProgramme && to === "executing";
}

/** What the phase form holds. */
export interface PhaseForm {
  to: PhaseToken | "";
  reason: string;
  override: string;
}

/** The body for `PUT …/phase`, or the first problem in the service's order. The override is sent only on a
 * programme's move into executing. */
export function phaseMoveBody(
  current: PhaseToken | null,
  form: PhaseForm,
  isProgramme: boolean,
):
  | {
      body: {
        phase: PhaseToken;
        reason?: string;
        start_override_reason?: string;
      };
    }
  | { problem: StringKey } {
  const to = form.to;
  if (to === "" || !PHASES.includes(to))
    return { problem: "p.phase.problem_choose" };
  if (to === current) return { problem: "p.phase.problem_same" };
  const a = phaseIndex(current);
  const b = phaseIndex(to);
  if (b > a + 1) return { problem: "p.phase.problem_skip" };
  const reason = form.reason.trim();
  if (b < a && reason === "") return { problem: "p.phase.problem_reason" };
  const override = form.override.trim();
  const starting = meetsStartCheck(isProgramme, to);
  if (starting && override !== "" && chars(override) < OVERRIDE_MIN)
    return { problem: "p.phase.problem_override" };
  const body: {
    phase: PhaseToken;
    reason?: string;
    start_override_reason?: string;
  } = { phase: to };
  if (reason) body.reason = reason;
  if (starting && override) body.start_override_reason = override;
  return { body };
}

/** Whether `err` is the service refusing a start (422 `start_refused`): the programme has joined the start queue. */
export function isStartRefused(err: unknown): boolean {
  if (!(err instanceof ApiError) || err.status !== 422) return false;
  const body = err.body as { code?: unknown } | undefined;
  return body?.code === "start_refused";
}

/** The reasons and the queue place a refused start carries (`errors: { refusals, queue_position }`), or `null` when the
 * refusal does not carry them in a shape this page reads. A reason of a kind this page does not know is left out. */
export function startRefusedFields(
  err: unknown,
): { refusals: StartRefusal[]; queuePosition: number | null } | null {
  if (!isStartRefused(err)) return null;
  const errors = ((err as ApiError).body as { errors?: unknown } | undefined)
    ?.errors as { refusals?: unknown; queue_position?: unknown } | undefined;
  if (!errors || !Array.isArray(errors.refusals)) return null;
  const refusals = (errors.refusals as Array<Record<string, unknown>>).filter(
    (r): r is StartRefusal & Record<string, unknown> =>
      (r?.reason === "limit_reached" &&
        typeof r.in_delivery === "number" &&
        typeof r.limit === "number") ||
      (r?.reason === "constraint_pool_over" &&
        typeof r.pool === "string" &&
        Array.isArray(r.months)),
  );
  const q = errors.queue_position;
  return {
    refusals,
    queuePosition: typeof q === "number" && Number.isSafeInteger(q) ? q : null,
  };
}

/** The sentence for any other refusal of a phase move, or `null` for the generic one. */
export function phaseRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (isStartRefused(err)) return null;
  if (err.status === 422) return "p.phase.refused_move";
  if (err.status === 404) return "p.phase.refused_gone";
  return null;
}

/** One refusal in the reader's words, from its structured fields (never the service's sentence). */
export function startRefusalText(
  r: StartRefusal,
  tr: (
    key: "p.phase.refusal_limit" | "p.phase.refusal_pool",
    values: Record<string, string | number>,
  ) => string,
  month: (iso: string) => string,
): string {
  return r.reason === "limit_reached"
    ? tr("p.phase.refusal_limit", {
        in_delivery: r.in_delivery,
        limit: r.limit,
      })
    : tr("p.phase.refusal_pool", {
        pool: r.pool,
        months: r.months.map(month).join(", "),
      });
}

/** The demand as a grid: every month any pool has, in order, and each pool's FTE in it (`null` for none). */
export function demandGrid(demand: StartCheck["demand_by_pool"]): {
  months: string[];
  rows: { pool: string; cells: (number | null)[] }[];
} {
  const months = [
    ...new Set(demand.flatMap((p) => p.months.map((m) => m.month))),
  ].sort();
  return {
    months,
    rows: demand.map((p) => ({
      pool: p.pool,
      cells: months.map(
        (m) => p.months.find((x) => x.month === m)?.demand_fte ?? null,
      ),
    })),
  };
}

/** Whole days in `ms`, rounded down. */
export function wholeDays(ms: number): number {
  return Math.max(0, Math.floor(ms / 86_400_000));
}
