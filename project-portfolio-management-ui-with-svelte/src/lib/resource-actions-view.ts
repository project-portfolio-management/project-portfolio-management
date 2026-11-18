// Resources and capacity, the rest (`/resources` and `/capacity`, NC-1 slice 12 part j, SC-7, PJ-3): a placeholder
// given a person (`POST /api/plans/{pid}/allocations/{a_pid}/resolve`) or removed (`DELETE
// /api/plans/{pid}/allocations/{a_pid}`), planned against recorded hours (`GET /api/resource-plan/hours`), a plan's
// skill gaps (`GET /api/plans/{pid}/skill-gap`), the cost rate in force (`GET /api/cost-rates/lookup`) and effort
// against capacity (`GET /api/capacity/utilization`). The service's rules (`controllers/portfolio_resources.rs`,
// `controllers/visibility/allocations.rs`, `controllers/cost_rates.rs`, `controllers/effort.rs`), said and checked
// before anything is sent; the service still decides.

import { ApiError } from "./api/client.js";
import type {
  PlannedHoursQuery,
  RateLookupQuery,
  ResolvePlaceholderBody,
  SkillGapFinding,
  UtilizationFigure,
  UtilizationQuery,
  UtilizationUnavailable,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { isPersonRef } from "./access-view.js";
import { isIsoDate } from "./paste-view.js";

const codeOf = (err: ApiError): string | null => {
  const body = err.body as { code?: unknown } | undefined;
  return typeof body?.code === "string" ? body.code : null;
};

// ---- a placeholder given a person, or an allocation removed ----

/** The body to resolve a placeholder (`confirm` only after the reader agreed to go over capacity), or the problem. */
export function resolveBody(
  person: string,
  confirm: boolean,
): { body: ResolvePlaceholderBody } | { problems: StringKey[] } {
  const ref = person.trim();
  if (!isPersonRef(ref)) return { problems: ["p.placeholder.problem_person"] };
  const body: ResolvePlaceholderBody = { person_ref: ref };
  if (confirm) body.confirm = true;
  return { body };
}

/** Whether a refused resolve is the service asking to confirm going over capacity (409 `over_capacity`). */
export function isOverCapacity(err: unknown): boolean {
  return (
    err instanceof ApiError &&
    err.status === 409 &&
    codeOf(err) === "over_capacity"
  );
}

/** The catalogue sentence for a refused resolve or removal, or `null` for the general rule. */
export function placeholderRefusalKey(
  action: "resolve" | "remove",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.placeholder.refused_gone";
  if (err.status === 403) return "p.placeholder.refused_forbidden";
  if (action === "resolve") {
    if (err.status === 409) return "p.placeholder.refused_named";
    if (err.status === 422) return "p.placeholder.refused_person";
  }
  return null;
}

// ---- planned against recorded hours ----

/** The longest window the service reads, in days. */
export const HOURS_MAX_DAYS = 3660;

const dayNumber = (iso: string) => Date.parse(`${iso}T00:00:00Z`) / 86_400_000;

/** The query (a blank end not sent: the service reads today and the 27 days before) or every problem at once. */
export function hoursQuery(
  from: string,
  to: string,
): { query: PlannedHoursQuery } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const f = from.trim();
  const e = to.trim();
  if (f && !isIsoDate(f)) problems.push("p.hours.problem_from");
  if (e && !isIsoDate(e)) problems.push("p.hours.problem_to");
  if (f && e && isIsoDate(f) && isIsoDate(e)) {
    if (e < f) problems.push("p.hours.problem_order");
    else if (dayNumber(e) - dayNumber(f) > HOURS_MAX_DAYS)
      problems.push("p.hours.problem_span");
  }
  if (problems.length > 0) return { problems };
  const query: PlannedHoursQuery = {};
  if (f) query.from = f;
  if (e) query.to = e;
  return { query };
}

/** Minutes as hours, to a tenth. */
export function hoursOf(minutes: number): number {
  return Math.round(minutes / 6) / 10;
}

/** The catalogue sentence for refused hours, or `null`. */
export function hoursRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  return err.status === 422 ? "p.hours.refused" : null;
}

// ---- a plan's skill gaps ----

/** A finding's standing in words. */
export const SKILL_STATUS_KEY: Record<SkillGapFinding["status"], StringKey> = {
  covered: "p.skillgap.status_covered",
  missing: "p.skillgap.status_missing",
  unknown: "p.skillgap.status_unknown",
};

/** A finding's shape, so the standing is never colour alone. */
export const SKILL_STATUS_SYMBOL: Record<SkillGapFinding["status"], string> = {
  covered: "✓",
  missing: "▲",
  unknown: "?",
};

/** How many findings of each standing. */
export function skillCounts(
  findings: readonly SkillGapFinding[],
): Record<SkillGapFinding["status"], number> {
  const out = { covered: 0, missing: 0, unknown: 0 };
  for (const f of findings) out[f.status] += 1;
  return out;
}

// ---- the cost rate in force ----

/** The query (`standard` not sent as the table) or every problem at once. */
export function rateQuery(
  person: string,
  date: string,
  table: string,
): { query: RateLookupQuery } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const ref = person.trim();
  const day = date.trim();
  if (!ref) problems.push("p.rate.problem_person");
  if (!isIsoDate(day)) problems.push("p.rate.problem_date");
  if (problems.length > 0) return { problems };
  const query: RateLookupQuery = { person_ref: ref, date: day };
  const name = table.trim();
  if (name && name !== "standard") query.table = name;
  return { query };
}

/** The catalogue sentence for a refused lookup, or `null`: rates are answered 404 to anyone but finance. */
export function rateRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.rate.refused_hidden";
  if (err.status === 400 || err.status === 422) return "p.rate.refused_query";
  return null;
}

// ---- effort against capacity ----

/** The window the service reads by default, in days, and the longest it takes. */
export const UTIL_DEFAULT_DAYS = 28;
export const UTIL_MAX_DAYS = 366;

/**
 * The query (the team and 28 days not sent) or the problem. A window outside 1 to 366 is refused first: the service
 * would quietly read 28 days instead (a service gap).
 */
export function utilizationQuery(
  by: "team" | "person",
  windowDays: number | null | undefined,
  planPid: string,
): { query: UtilizationQuery } | { problems: StringKey[] } {
  const query: UtilizationQuery = {};
  if (windowDays !== null && windowDays !== undefined) {
    const d = Number(windowDays);
    if (!Number.isSafeInteger(d) || d < 1 || d > UTIL_MAX_DAYS)
      return { problems: ["p.util.problem_window"] };
    if (d !== UTIL_DEFAULT_DAYS) query.window_days = d;
  }
  if (by === "person") query.by = "person";
  if (planPid) query.plan_pid = planPid;
  return { query };
}

/** Why a figure is absent, in words. */
export const UNAVAILABLE_KEY: Record<UtilizationUnavailable, StringKey> = {
  no_declared_capacity: "p.util.unavailable_no_capacity",
  all_non_working: "p.util.unavailable_all_away",
  below_floor: "p.util.unavailable_below_floor",
  small_group: "p.util.unavailable_small_group",
};

/** Why a figure is absent; a reason the page does not know is a general one. */
export function unavailableKey(reason: string | null): StringKey {
  return (
    (UNAVAILABLE_KEY as Record<string, StringKey>)[reason ?? ""] ??
    "p.util.unavailable_other"
  );
}

/** The figures as a list, whether the service answered one (the team) or many (by person). */
export function figuresOf(
  u: UtilizationFigure | UtilizationFigure[],
): UtilizationFigure[] {
  return Array.isArray(u) ? u : [u];
}

/** The catalogue sentence for a refused reading, or `null`. */
export function utilizationRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  return err.status === 422 ? "p.util.refused" : null;
}
