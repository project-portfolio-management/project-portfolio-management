// Status updates on a cadence, with their evidence (`/plans/[pid]/status` and the due list on `/dashboard`; NC-4,
// BR-2, NC-1 slice 12 part b): the service's rules (`domain/src/status_updates.rs`, `domain/src/status_evidence.rs`
// and `controllers/status_updates.rs`), said before anything is sent. The service still decides. A plan expects an
// update every 1 to 92 days; setting the cadence restarts the clock, and no route removes one. An update is a
// person's call (on track, at risk, off track) with a narrative of up to 4000 characters, and up to 10 claims, each a
// sentence of up to 500 characters citing up to 5 sources: a record of the plan or one of the published numbers. The
// service computes no health score. Updates are never changed or removed: each one posted is a new one.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type {
  StatusCall,
  StatusSource,
  StatusStanding,
  StatusUpdateBody,
} from "./api/ppm-types.js";

/** The calls an update can make, in the service's order. */
export const STATUS_CALLS: readonly StatusCall[] = [
  "on_track",
  "at_risk",
  "off_track",
];
/** The shortest and longest cadence, in days. */
export const CADENCE_MIN = 1;
export const CADENCE_MAX = 92;
/** The longest narrative, in characters (the service counts characters, not trimmed). */
export const NARRATIVE_MAX = 4000;
/** The most claims an update carries, the longest claim and the most sources a claim cites. */
export const CLAIMS_MAX = 10;
export const CLAIM_MAX = 500;
export const SOURCES_MAX = 5;
/** The numbers a claim may cite, in the service's order; each has words in the catalogue. */
export const METRIC_KEYS = [
  "overdue",
  "due_soon",
  "blocked",
  "no_due_date",
] as const;
export type MetricKey = (typeof METRIC_KEYS)[number];
/** The kinds of the plan's records this page offers as sources. The service also accepts change requests and cutover
 * decisions; this page does not offer them. */
export const RECORD_KINDS = ["task", "risk", "milestone", "issue"] as const;
export type RecordKind = (typeof RECORD_KINDS)[number];

/** A call as a word's key and a symbol: never a colour alone. */
export const CALL_MARK: Record<StatusCall, { symbol: string; key: StringKey }> =
  {
    on_track: { symbol: "✓", key: "p.status.call_on_track" },
    at_risk: { symbol: "!", key: "p.status.call_at_risk" },
    off_track: { symbol: "✗", key: "p.status.call_off_track" },
  };

const chars = (s: string) => [...s].length;

/** A whole number from a form value (a number input binds a number once typed in, text before), or `null`. */
function whole(v: string | number): number | null {
  const raw = String(v ?? "").trim();
  if (raw === "") return null;
  const n = Number(raw);
  return Number.isSafeInteger(n) ? n : null;
}

/** The body for `PUT …/status-cadence`, the problem, or `null` when it is the cadence already set. Setting the same
 * cadence again would restart the clock, so it is not sent. */
export function cadenceBody(
  current: number | null,
  value: string | number,
): { body: { cadence_days: number } } | { problem: StringKey } | null {
  const n = whole(value);
  if (n === null || n < CADENCE_MIN || n > CADENCE_MAX)
    return { problem: "p.status.problem_cadence" };
  if (current === n) return null;
  return { body: { cadence_days: n } };
}

/** One claim as the form holds it: its sentence and the sources chosen (`kind:<pid>` or `metric:<key>`). */
export interface ClaimForm {
  text: string;
  sources: string[];
}

/** What the update form holds. */
export interface UpdateForm {
  status: StatusCall | "";
  narrative: string;
  claims: ClaimForm[];
}

/** One problem with the update, and the claim it is on (1-based; 0 for the update itself). */
export interface UpdateProblem {
  key: StringKey;
  claim: number;
}

/** The body for `POST …/status-updates`, or every problem in the service's order. A claim with no text and no source
 * is left out; one with sources needs its sentence. */
export function updateBody(
  form: UpdateForm,
): { body: StatusUpdateBody } | { problems: UpdateProblem[] } {
  const problems: UpdateProblem[] = [];
  if (!(STATUS_CALLS as readonly string[]).includes(form.status))
    problems.push({ key: "p.status.problem_call", claim: 0 });
  if (form.narrative.trim() === "" || chars(form.narrative) > NARRATIVE_MAX)
    problems.push({ key: "p.status.problem_narrative", claim: 0 });
  const claims = form.claims
    .map((c, i) => ({ ...c, n: i + 1 }))
    .filter((c) => c.text.trim() !== "" || c.sources.length > 0);
  if (claims.length > CLAIMS_MAX)
    problems.push({ key: "p.status.problem_claims", claim: 0 });
  for (const c of claims) {
    if (c.text.trim() === "" || chars(c.text) > CLAIM_MAX)
      problems.push({ key: "p.status.problem_claim_text", claim: c.n });
    if (new Set(c.sources).size > SOURCES_MAX)
      problems.push({ key: "p.status.problem_sources", claim: c.n });
  }
  if (problems.length > 0) return { problems };
  const body: StatusUpdateBody = {
    status: form.status as StatusCall,
    narrative: form.narrative.trim(),
  };
  if (claims.length > 0)
    body.claims = claims.map((c) => ({
      text: c.text.trim(),
      sources: [...new Set(c.sources)],
    }));
  return { body };
}

/** Where a plan stands against its cadence, as the page says it: overdue by some days (a plural), due today, or the
 * date the next update is due. Read from the service's fields; nothing is worked out here. */
export function standingOf(s: StatusStanding): {
  is: "overdue" | "due" | "not_due";
  date: string;
  days: number;
} {
  if (s.overdue) return { is: "overdue", date: s.due_on, days: s.days_overdue };
  if (s.due) return { is: "due", date: s.due_on, days: 0 };
  return { is: "not_due", date: s.due_on, days: 0 };
}

/** Whether `key` is a number this page has words for. */
export function isMetricKey(key: string): key is MetricKey {
  return (METRIC_KEYS as readonly string[]).includes(key);
}

/** What one source of a claim says, as the page shows it: a record's kind and title (the record's own data), a
 * number's catalogue name and value (unknown is not zero), or that it is unavailable. The service's English
 * definition is never used. */
export type SourceView =
  | { as: "record"; kind: RecordKind | "other"; title: string }
  | { as: "metric"; key: MetricKey; value: number | null }
  | { as: "unavailable" };

export function sourceView(s: StatusSource): SourceView {
  if (s.state !== "ok") return { as: "unavailable" };
  if (s.kind === "metric") {
    const m = s as Extract<StatusSource, { kind: "metric" }>;
    if (!isMetricKey(m.key)) return { as: "unavailable" };
    return { as: "metric", key: m.key, value: m.known ? m.value : null };
  }
  const r = s as Extract<StatusSource, { title: string }>;
  const kind = (RECORD_KINDS as readonly string[]).includes(r.kind)
    ? (r.kind as RecordKind)
    : "other";
  return { as: "record", kind, title: r.title };
}

/** The sentence for a refusal of `action`, or `null` for the generic one. The service gives every 422 on an update
 * one code (`unprocessable`) with English text; the page checks the rest first, so a 422 after that is a source that is
 * no longer a record of this plan. */
export function statusRefusalKey(
  action: "cadence" | "update",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.status.refused_gone";
  if (err.status !== 422) return null;
  return action === "cadence"
    ? "p.status.problem_cadence"
    : "p.status.refused_update";
}
