// Scorecards and the de-escalation tracker (`/deescalation`, EX-10, NC-1 slice 11): the service's rules
// (`domain/src/deescalation.rs`, `domain/src/kpi.rs` and `controllers/deescalation.rs`), said before anything is sent,
// what each action sends (nothing for no change), the reasons a quarter is not met built from the detail, and the
// sentence for each refusal. The service still decides. The escalation level is recorded as the sponsor set it and is
// never computed; the run of quarters met is offered to the person who decides and moves no level.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type {
  DeescalationCriterion,
  KpiDirection,
  QuarterState,
  ScorecardKpi,
  ScorecardRow,
  TemplateMeasure,
} from "./api/ppm-types.js";

/** A scorecard's name, in characters after trimming. */
export const SCORECARD_NAME_MIN = 3;
export const SCORECARD_NAME_MAX = 120;
/** The most KPIs a scorecard holds. */
export const SCORECARD_KPIS_MAX = 60;
/** The escalation level's text, in characters after trimming. */
export const LEVEL_MIN = 1;
export const LEVEL_MAX = 60;
/** A criterion's text, in characters after trimming. */
export const CRITERION_MIN = 5;
export const CRITERION_MAX = 500;
/** The shortest evidence note, in characters after trimming. */
export const NOTE_MIN = 5;
/** The longest evidence link, in bytes (the service counts bytes). */
export const LINK_MAX = 500;

/** The template's measures this page has words for; the service's English names are never shown. */
export const TEMPLATE_KEYS = [
  "programmes_with_independent_assurance",
  "milestones_met_on_time",
  "benefits_validated",
  "red_without_decision",
  "open_critical_issues",
  "change_requests_waiting",
] as const;

const chars = (s: string) => [...s.trim()].length;

/** What is wrong with a new scorecard, or `null`: a name of 3 to 120 characters not taken in any case, and at most one
 * scorecard the tracker reads. */
export function scorecardProblem(
  name: string,
  forDeescalation: boolean,
  scorecards: readonly ScorecardRow[],
): StringKey | null {
  const n = name.trim();
  const len = chars(n);
  if (len < SCORECARD_NAME_MIN || len > SCORECARD_NAME_MAX)
    return "p.deescalation.problem_name";
  if (scorecards.some((s) => s.name.trim().toLowerCase() === n.toLowerCase()))
    return "p.deescalation.problem_name_taken";
  if (forDeescalation && scorecards.some((s) => s.for_deescalation))
    return "p.deescalation.problem_tracker_taken";
  return null;
}

/** The body for `PUT …/kpis`, `null` when the chosen set is the one the scorecard has, or the problem (over 60). */
export function kpisBody(
  current: readonly string[],
  chosen: readonly string[],
):
  | { body: { kpi_pids: string[] } }
  | { problem: "p.deescalation.problem_kpis_max" }
  | null {
  const want = [...new Set(chosen)];
  if (want.length > SCORECARD_KPIS_MAX)
    return { problem: "p.deescalation.problem_kpis_max" };
  const have = new Set(current);
  if (want.length === have.size && want.every((k) => have.has(k))) return null;
  return { body: { kpi_pids: want } };
}

/** One template row as the form holds it: a number input binds a number once typed in, text before. */
export interface TemplatePick {
  key: string;
  chosen: boolean;
  target: string | number;
  red: string | number;
}

/** A whole number from a form value, or `null`. */
function whole(v: string | number): number | null {
  const raw = String(v ?? "").trim();
  if (raw === "") return null;
  const n = Number(raw);
  return Number.isSafeInteger(n) ? n : null;
}

/** The direction rule (`kpi::threshold_problem`): the red line below the target when higher is better, above it when
 * lower is better. */
export function redLineProblem(
  direction: KpiDirection,
  target: number,
  red: number,
): StringKey | null {
  if (direction === "higher_better" && red >= target)
    return "p.deescalation.problem_red_higher";
  if (direction === "lower_better" && red <= target)
    return "p.deescalation.problem_red_lower";
  return null;
}

/** The body for `POST …/from-template`, or the first problem and the measure it is on (`key` empty: none picked). */
export function templateBody(
  picks: readonly TemplatePick[],
  measures: readonly TemplateMeasure[],
):
  | {
      body: {
        measures: Array<{
          key: string;
          target_value: number;
          red_value: number;
        }>;
      };
    }
  | { problem: StringKey; key: string } {
  const out: Array<{ key: string; target_value: number; red_value: number }> =
    [];
  for (const p of picks) {
    if (!p.chosen) continue;
    const m = measures.find((x) => x.key === p.key);
    if (!m)
      return { problem: "p.deescalation.problem_not_in_template", key: p.key };
    const target = whole(p.target);
    const red = whole(p.red);
    if (target === null || red === null)
      return { problem: "p.deescalation.problem_whole", key: p.key };
    const why = redLineProblem(m.direction, target, red);
    if (why) return { problem: why, key: p.key };
    out.push({ key: p.key, target_value: target, red_value: red });
  }
  if (out.length === 0)
    return { problem: "p.deescalation.problem_pick_one", key: "" };
  return { body: { measures: out } };
}

/** What is wrong with an escalation level, or `null`; the note is optional. */
export function levelProblem(level: string): StringKey | null {
  const len = chars(level);
  return len < LEVEL_MIN || len > LEVEL_MAX
    ? "p.deescalation.problem_level"
    : null;
}

/** The body for `PUT /api/deescalation/level`, or `null` when it would record the level and note already recorded. */
export function levelBody(
  current: { level: string; note: string | null } | null,
  level: string,
  note: string,
): { level: string; note?: string } | null {
  const l = level.trim();
  const n = note.trim();
  if (current && current.level === l && (current.note ?? "") === n) return null;
  return n ? { level: l, note: n } : { level: l };
}

/** What is wrong with a new criterion, or `null`. */
export function criterionProblem(text: string): StringKey | null {
  const len = chars(text);
  return len < CRITERION_MIN || len > CRITERION_MAX
    ? "p.deescalation.problem_criterion"
    : null;
}

/** The quarter a `YYYY-MM-DD` date falls in, as `2026-Q3`. */
export function quarterOf(date: string): string {
  const [y, m] = date.split("-").map(Number);
  return `${y}-Q${Math.floor(((m ?? 1) - 1) / 3) + 1}`;
}

/** `2026-Q3` as `{ year, q }`, or `null` when it is not one (years 2000 to 2100, quarters 1 to 4, as the service). */
export function parseQuarter(
  label: string,
): { year: number; q: number } | null {
  const m = /^(\d{4})-Q(\d)$/.exec(label.trim());
  if (!m) return null;
  const year = Number(m[1]);
  const q = Number(m[2]);
  return year >= 2000 && year <= 2100 && q >= 1 && q <= 4 ? { year, q } : null;
}

/** A quarter's label in the reader's words (`Q3 2026`), through the catalogue's `p.deescalation.quarter`. */
export function quarterName(
  label: string,
  tr: (
    key: "p.deescalation.quarter",
    values: { year: number; q: number },
  ) => string,
): string {
  const p = parseQuarter(label);
  return p ? tr("p.deescalation.quarter", p) : label;
}

/** The last quarter finished by `today` (`YYYY-MM-DD`). */
export function lastFinishedQuarter(today: string): string {
  const now = parseQuarter(quarterOf(today))!;
  return now.q === 1 ? `${now.year - 1}-Q4` : `${now.year}-Q${now.q - 1}`;
}

/** What is wrong with evidence for a quarter, in the service's order, or the body to send. */
export function evidenceBody(
  form: { quarter: string; met: boolean | null; note: string; link: string },
  today: string,
):
  | { body: { quarter: string; met: boolean; note: string; link?: string } }
  | { problem: StringKey } {
  const q = parseQuarter(form.quarter);
  if (!q) return { problem: "p.deescalation.problem_quarter" };
  const last = parseQuarter(lastFinishedQuarter(today))!;
  if (q.year * 4 + q.q > last.year * 4 + last.q)
    return { problem: "p.deescalation.problem_quarter_unfinished" };
  if (form.met === null) return { problem: "p.deescalation.problem_met" };
  if (chars(form.note) < NOTE_MIN)
    return { problem: "p.deescalation.problem_note" };
  const link = form.link.trim();
  if (
    link &&
    (new TextEncoder().encode(link).length > LINK_MAX ||
      !/^https?:\/\//.test(link) ||
      /\s/.test(link))
  )
    return { problem: "p.deescalation.problem_link" };
  const body: { quarter: string; met: boolean; note: string; link?: string } = {
    quarter: `${q.year}-Q${q.q}`,
    met: form.met,
    note: form.note.trim(),
  };
  if (link) body.link = link;
  return { body };
}

/** Whether evidence would record what the latest record for that criterion and quarter already says. */
export function sameEvidence(
  criterion: DeescalationCriterion | undefined,
  body: { quarter: string; met: boolean; note: string; link?: string },
): boolean {
  const e = criterion?.quarters.find((x) => x.quarter === body.quarter);
  if (!e || e.met === null) return false;
  return (
    e.met === body.met &&
    (e.note ?? "") === body.note &&
    (e.link ?? "") === (body.link ?? "")
  );
}

/** A state as a word's key and a symbol: never a colour alone. */
export const STATE_MARK: Record<
  QuarterState["state"],
  { symbol: string; key: StringKey }
> = {
  met: { symbol: "✓", key: "p.deescalation.state_met" },
  not_met: { symbol: "✗", key: "p.deescalation.state_not_met" },
  missing: { symbol: "?", key: "p.deescalation.state_missing" },
};

/** One reading or one piece of evidence as a state: `null` is nothing recorded. */
export function stateOf(
  met: boolean | null | undefined,
): QuarterState["state"] {
  if (met === true) return "met";
  if (met === false) return "not_met";
  return "missing";
}

/** Why a quarter is not met, from the detail and never from the service's sentences: the KPIs and criteria not met and
 * those with nothing recorded, by name. `nothing` is true when there is nothing to measure. */
export function quarterReasons(
  quarter: string,
  kpis: readonly ScorecardKpi[],
  criteria: readonly Pick<DeescalationCriterion, "criterion" | "quarters">[],
): { notMet: string[]; missing: string[]; nothing: boolean } {
  const notMet: string[] = [];
  const missing: string[] = [];
  const inputs: Array<{ name: string; met: boolean | null }> = [
    ...kpis.map((k) => ({
      name: k.name,
      met: k.quarters.find((q) => q.quarter === quarter)?.met ?? null,
    })),
    ...criteria.map((c) => ({
      name: c.criterion,
      met: c.quarters.find((q) => q.quarter === quarter)?.met ?? null,
    })),
  ];
  for (const i of inputs) {
    if (i.met === null) missing.push(i.name);
    else if (!i.met) notMet.push(i.name);
  }
  return { notMet, missing, nothing: inputs.length === 0 };
}

/** The sentence for a refusal of `action`, or `null` for the generic one. */
export function deescalationRefusalKey(
  action: "create" | "kpis" | "template" | "level" | "criterion" | "evidence",
  err: unknown,
  forDeescalation = false,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  const s = err.status;
  if (action === "create") {
    if (s === 409)
      return forDeescalation
        ? "p.deescalation.refused_create_conflict_tracker"
        : "p.deescalation.problem_name_taken";
    if (s === 422) return "p.deescalation.problem_name";
  }
  if (action === "kpis" && s === 422) return "p.deescalation.refused_kpis";
  if (action === "template" && s === 422)
    return "p.deescalation.refused_template";
  if (action === "level") {
    if (s === 403) return "p.deescalation.refused_level_forbidden";
    if (s === 422) return "p.deescalation.problem_level";
  }
  if (action === "criterion" && s === 422)
    return "p.deescalation.problem_criterion";
  if (action === "evidence" && s === 422)
    return "p.deescalation.refused_evidence";
  if (s === 404) return "p.deescalation.refused_gone";
  return null;
}
