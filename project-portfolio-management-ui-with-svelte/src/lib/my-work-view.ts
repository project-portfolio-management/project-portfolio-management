// My work (LT-3) + float for the worker (SC-10): the plain sentence shown under
// a task about its schedule. Pure, so it is unit-testable without a component.
//
// A task with no schedule data shows nothing at all — never "0 business days of float".
// Negative float is said in words, not just colour, and the text is ordinary
// text, so a screen reader reads it with the rest of the row.

import type {
  DiscussionSubjectKind,
  Grain,
  MyTaskSchedule,
} from "#lib/api/ppm.js";
import { ApiError } from "./api/client.js";
import { businessDays } from "./units.js";
import { t, type StringKey } from "./i18n.svelte.js";

export type FloatKind = "late" | "critical" | "float";

export interface FloatText {
  kind: FloatKind;
  /** The sentence, ready to render. */
  text: string;
}

const days = businessDays;

/**
 * The sentence for a task's schedule, or `null` when there is nothing to say
 * (no schedule data).
 */
export function floatText(
  schedule: MyTaskSchedule | null | undefined,
): FloatText | null {
  if (!schedule || !Number.isFinite(schedule.total_float)) return null;
  const f = schedule.total_float;
  if (f < 0) {
    return {
      kind: "late",
      text: t("l.my_work_view.this_is_already_late_for", { days: days(-f) }),
    };
  }
  if (f === 0 || schedule.critical) {
    return { kind: "critical", text: t("l.my_work_view.on_the_critical_path") };
  }
  return {
    kind: "float",
    text: t("l.my_work_view.of_float", { days: days(f) }),
  };
}

// ---- the calendar feed, what I follow and the work over time (NC-1 slice 12 part h: LT-7, RM-3, PJ-5) ----
//
// The feed (`controllers/feeds.rs`): a token is shown once (only its hash is kept), lasts 365 days, and making one
// revokes every earlier one; revoking stops them all at once. The service answers with addresses on its own origin
// (`/api/feeds/calendar.ics?token=…`); a calendar app reaches them through this site's proxy, which forwards a
// request with no session to the service without a token, and `/api/feeds/` authenticates by the token alone. The
// token is shown only as the service returns it, inside that address, and is never logged or stored by the page.
//
// What I follow (`controllers/watchers.rs`): a kind and a reference for each, newest first, at most 500; no names.
//
// The work over time (`controllers/assignments.rs`, `domain/src/work_profile.rs`): days, weeks (the default, not sent)
// or months; a window of at most ten years and 120 periods (a daily one fewer than 120 days, which is the same
// rule); from not after to. With no
// dates the window is drawn from the placed work. Every 422 is `unprocessable` with English text.

/** The address a calendar app subscribes to, through this site's proxy, or `null` when the answer is not a feed. */
export function feedAddress(base: string, url: string): string | null {
  return url.startsWith("/api/feeds/") ? `${base}${url}` : null;
}

/** The kinds a person can follow (`SUBJECT_KINDS`), each named in words. */
export const WATCH_KIND_KEY: Record<string, StringKey> = {
  plan: "p.watching.kind_plan",
  task: "p.watching.kind_task",
  risk: "p.watching.kind_risk",
  proposal: "p.watching.kind_proposal",
  idea: "p.watching.kind_idea",
  canvas_item: "p.watching.kind_canvas_item",
  budget_line: "p.watching.kind_budget_line",
  scenario: "p.watching.kind_scenario",
  document_version: "p.watching.kind_document_version",
};

/** The words for a kind the service sent; one this page does not know is "something". */
export function watchKindKey(kind: string): StringKey {
  return WATCH_KIND_KEY[kind] ?? "p.watching.kind_other";
}

/** The most the service lists (`LIMIT 500`). */
export const WATCHING_MAX = 500;

/** The start of a reference, for a thing the page has no name for. */
export function shortRef(pid: string): string {
  return pid.slice(0, 8);
}

/** Whether `kind` is one this page may send back to stop following it. */
export function isWatchKind(kind: string): kind is DiscussionSubjectKind {
  return kind in WATCH_KIND_KEY;
}

/** The work over time as asked: the period (`""` for weeks) and the two dates (`""` when left out). */
export interface WorkProfileForm {
  period: "" | Grain;
  from: string;
  to: string;
}

/** The longest window, in days, and the most periods (`MAX_DAYS`, `MAX_PERIODS`). */
export const PROFILE_MAX_DAYS = 3660;
export const PROFILE_MAX_PERIODS = 120;

const civilMs = (s: string): number | null => {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s.trim());
  if (!m) return null;
  const ms = Date.UTC(Number(m[1]), Number(m[2]) - 1, Number(m[3]));
  const back = new Date(ms);
  return back.getUTCDate() === Number(m[3]) &&
    back.getUTCMonth() === Number(m[2]) - 1
    ? ms
    : null;
};

/** How many periods of `grain` the window from `a` to `b` (UTC midnights) holds, as the service counts them. */
function periodCount(a: number, b: number, grain: Grain): number {
  const day = 86_400_000;
  if (grain === "day") return Math.round((b - a) / day) + 1;
  if (grain === "week") {
    const weekday = (new Date(a).getUTCDay() + 6) % 7;
    return Math.floor((b - (a - weekday * day)) / day / 7) + 1;
  }
  const x = new Date(a);
  const y = new Date(b);
  return (
    (y.getUTCFullYear() - x.getUTCFullYear()) * 12 +
    (y.getUTCMonth() - x.getUTCMonth()) +
    1
  );
}

/** The query for `GET /api/work-profile`, or every problem at once. Weeks, the default, are not sent. */
export function workProfileQuery(
  form: WorkProfileForm,
):
  | { query: { period?: Grain; from?: string; to?: string } }
  | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const from = form.from.trim();
  const to = form.to.trim();
  const a = from ? civilMs(from) : null;
  const b = to ? civilMs(to) : null;
  if (from && a === null) problems.push("p.workprofile.problem_from");
  if (to && b === null) problems.push("p.workprofile.problem_to");
  const grain: Grain = form.period || "week";
  if (a !== null && b !== null) {
    if (a > b) problems.push("p.workprofile.problem_order");
    else if ((b - a) / 86_400_000 > PROFILE_MAX_DAYS)
      problems.push("p.workprofile.problem_too_long");
    else if (periodCount(a, b, grain) > PROFILE_MAX_PERIODS)
      problems.push("p.workprofile.problem_periods");
  }
  if (problems.length > 0) return { problems };
  return {
    query: {
      ...(grain !== "week" ? { period: grain } : {}),
      ...(from ? { from } : {}),
      ...(to ? { to } : {}),
    },
  };
}

/** An action on `/me` that the service may refuse. */
export type MeAction =
  "issue_feed" | "revoke_feed" | "unwatch" | "work_profile";

/**
 * The catalogue sentence for a refusal, or `null` for the general rule. A 403 on the feed is the policy refusing the
 * caller; a 422 on stopping a follow is a kind the service does not take; a 422 on the work over time is a window it
 * would not draw (no work placed and no dates, or too many periods).
 */
export function meRefusalKey(action: MeAction, err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (action === "issue_feed" || action === "revoke_feed")
    return err.status === 403 ? "p.feed.refused_forbidden" : null;
  if (action === "unwatch")
    return err.status === 422 ? "p.watching.refused" : null;
  return err.status === 422 ? "p.workprofile.refused" : null;
}
