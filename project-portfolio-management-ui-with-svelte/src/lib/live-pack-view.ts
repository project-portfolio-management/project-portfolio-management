// The director pack's programme readings, read live (`/portfolio-map`, NC-1 slice 12 part f; DP-7, DP-8, DP-9, IR-6):
// the buffer fever chart (`GET /api/buffer-fever`), earned value over the threshold (`GET
// /api/portfolio-earned-value`), validated benefits (`GET /api/portfolio-benefits`) and issues and change requests
// (`GET /api/director-pack/issues-and-changes`). Each answer has the shape of the pack's own section, so the page draws
// it with the pack's renderer (`buildPackView`); this module holds what the page asks and checks first. The service
// still decides, and reads only the plans the reader may read.

import type { AcSource } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { buildPackView, type Deps, type Section } from "./pack-view.js";

/** The issues and change requests are counted over this many calendar days, 1 to 366; 30 when none is given. */
export const WINDOW_MIN = 1;
export const WINDOW_MAX = 366;
export const WINDOW_DEFAULT = 30;

/** The actual cost earned value may read, in the service's order; `budget_lines` is its default. */
export const AC_SOURCES: readonly AcSource[] = [
  "budget_lines",
  "labour",
  "both",
];

/** Each actual-cost source's words. */
export const AC_KEY: Record<AcSource, StringKey> = {
  budget_lines: "p.live_pack.ac_budget_lines",
  labour: "p.live_pack.ac_labour",
  both: "p.live_pack.ac_both",
};

/** The problem with a window of `days` (a number input's value: `null` when empty, which asks for the default). */
export function windowProblem(days: number | null): StringKey | null {
  if (days === null) return null;
  return Number.isInteger(days) && days >= WINDOW_MIN && days <= WINDOW_MAX
    ? null
    : "p.live_pack.problem_window";
}

/** The window to ask for: `undefined` (the service's 30) when none is given or it is the default. */
export function windowParam(days: number | null): number | undefined {
  return days === null || days === WINDOW_DEFAULT ? undefined : days;
}

/** The actual-cost source to ask for: `undefined` for the service's default. */
export function acParam(ac: AcSource): AcSource | undefined {
  return ac === "budget_lines" ? undefined : ac;
}

/** Whether the issues answer says the reader can read no plan with issues or change requests (its `reason` is the
 * service's English and is not shown). */
export function issuesUnavailable(
  body: Record<string, unknown> | null,
): boolean {
  return body !== null && body.available === false;
}

/** The pack section a live answer is drawn as. */
export type LiveId = "fever" | "evm" | "benefits" | "issues" | "changes";

/**
 * The pack's own section for each live answer that has loaded, drawn by the pack's renderer from the answer alone:
 * the fever chart's `buffer_fever`, earned value's `earned_value`, benefits' `benefits`, and the two halves of the
 * issues answer as the pack's `issues` and `change_requests`.
 */
export function liveSections(
  live: {
    fever?: Record<string, unknown> | null;
    evm?: Record<string, unknown> | null;
    benefits?: Record<string, unknown> | null;
    issues?: Record<string, unknown> | null;
  },
  d: Deps,
): Partial<Record<LiveId, Section>> {
  const body: Record<string, unknown> = {};
  if (live.fever) body.buffer_fever = live.fever;
  if (live.evm) body.earned_value = live.evm;
  if (live.benefits) body.benefits = live.benefits;
  if (live.issues && !issuesUnavailable(live.issues)) {
    body.issues = live.issues.issues;
    body.change_requests = live.issues.change_requests;
  }
  const want: Array<[LiveId, boolean]> = [
    ["fever", Boolean(live.fever)],
    ["evm", Boolean(live.evm)],
    ["benefits", Boolean(live.benefits)],
    ["issues", "issues" in body],
    ["changes", "issues" in body],
  ];
  const all = buildPackView(body, d);
  const out: Partial<Record<LiveId, Section>> = {};
  for (const [id, has] of want) {
    const s = all.find((x) => x.id === id);
    if (has && s) out[id] = s;
  }
  return out;
}
