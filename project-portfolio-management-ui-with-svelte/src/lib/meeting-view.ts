// A plan's meetings and the tasks made from their actions (`/plans/[pid]/meetings`; NC-14, BR-3, NC-1 slice 12 part
// e): the service's rules (`domain/src/meetings.rs`, `domain/src/meeting_actions.rs`, `controllers/meetings.rs` and
// `controllers/meeting_actions.rs`), said before anything is sent. The service still decides.
//
// A meeting is planned with a title (up to 200 characters), a date and an agenda of up to 50 items (each up to 500
// characters), then recorded once as held, with minutes (up to 20,000 characters) and up to 50 outcomes (a decision,
// an action or a note, each up to 500 characters), or cancelled. A held or cancelled meeting is a record: its minutes
// cannot be corrected and it cannot be restored. An agenda item or outcome may point at a record of the plan. The
// service records no attendees.
//
// From a held meeting's actions, tasks are proposed (up to 20 at a time): each quotes a passage of at least 8
// characters found in the minutes (case and spacing ignored), may name an assignee already on the plan's tasks or
// RACI, a due date from a week before the meeting to a year after, and points from 0 to 1000. A preview writes
// nothing; creating them is all or none, and an outcome gets one task only.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type {
  HeldMeetingBody,
  Meeting,
  MeetingStatus,
  OutcomeKind,
  PlanMeetingBody,
  RecordLink,
  TaskProposal,
  TaskProposalPreview,
} from "./api/ppm-types.js";

/** The longest title, in characters. */
export const TITLE_MAX = 200;
/** The most agenda items, and the most outcomes, a meeting holds. */
export const ITEMS_MAX = 50;
/** The longest agenda item or outcome, in characters. */
export const TEXT_MAX = 500;
/** The longest minutes, in characters. */
export const MINUTES_MAX = 20_000;
/** What an outcome is, in the service's order. */
export const OUTCOME_KINDS: readonly OutcomeKind[] = [
  "decision",
  "action",
  "note",
];
/** The most proposals one request carries. */
export const PROPOSALS_MAX = 20;
/** The shortest passage of the minutes that counts as evidence, in characters after collapsing spaces. */
export const EXCERPT_MIN = 8;
/** The longest task title, in characters. */
export const TASK_TITLE_MAX = 200;
/** A due date may fall this many days before the meeting, and this many after. */
export const WINDOW_BEFORE_DAYS = 7;
export const WINDOW_AFTER_DAYS = 365;
/** Points run from 0 to this. */
export const POINTS_MAX = 1000;
/** The kinds of the plan's records this page offers to link. The service also takes change requests and cutover
 * decisions; this page does not offer them (a link made elsewhere is still shown). */
export const LINK_KINDS = ["task", "risk", "milestone", "issue"] as const;
export type LinkKind = (typeof LINK_KINDS)[number];

/** A meeting's state as a word's key and a shape: never a colour alone. */
export const STATUS_MARK: Record<
  MeetingStatus,
  { symbol: string; key: StringKey }
> = {
  planned: { symbol: "○", key: "p.meetings.status_planned" },
  held: { symbol: "✓", key: "p.meetings.status_held" },
  cancelled: { symbol: "✗", key: "p.meetings.status_cancelled" },
};

/** An outcome's kind as a word's key. */
export const OUTCOME_KEY: Record<OutcomeKind, StringKey> = {
  decision: "p.meetings.outcome_decision",
  action: "p.meetings.outcome_action",
  note: "p.meetings.outcome_note",
};

const chars = (s: string) => [...s].length;

/** A chosen record (`kind:<pid>`, or `""` for none) as a link, or `undefined`. */
export function linkOf(choice: string): RecordLink | undefined {
  const at = choice.indexOf(":");
  if (at <= 0 || at === choice.length - 1) return undefined;
  return { kind: choice.slice(0, at), pid: choice.slice(at + 1) };
}

/** Whether `kind` is one this page names with its own word. */
export const isLinkKind = (kind: string): kind is LinkKind =>
  (LINK_KINDS as readonly string[]).includes(kind);

/** One problem with a form, and the line it is on (1-based; 0 for the form itself). */
export interface LineProblem {
  key: StringKey;
  line: number;
}

/** One agenda item as the form holds it. */
export interface AgendaForm {
  text: string;
  link: string;
}

/** The body for `POST …/meetings`, or every problem. An agenda line with no text and no record is left out. */
export function planBody(form: {
  title: string;
  scheduledOn: string;
  agenda: AgendaForm[];
}): { body: PlanMeetingBody } | { problems: LineProblem[] } {
  const problems: LineProblem[] = [];
  if (form.title.trim() === "" || chars(form.title.trim()) > TITLE_MAX)
    problems.push({ key: "p.meetings.problem_title", line: 0 });
  if (!/^\d{4}-\d{2}-\d{2}$/.test(form.scheduledOn))
    problems.push({ key: "p.meetings.problem_date", line: 0 });
  const lines = form.agenda
    .map((a, i) => ({ ...a, n: i + 1 }))
    .filter((a) => a.text.trim() !== "" || a.link !== "");
  if (lines.length > ITEMS_MAX)
    problems.push({ key: "p.meetings.problem_agenda_count", line: 0 });
  for (const a of lines)
    if (a.text.trim() === "" || chars(a.text.trim()) > TEXT_MAX)
      problems.push({ key: "p.meetings.problem_item_text", line: a.n });
  if (problems.length > 0) return { problems };
  const body: PlanMeetingBody = {
    title: form.title.trim(),
    scheduled_on: form.scheduledOn,
  };
  if (lines.length > 0)
    body.agenda = lines.map((a) => {
      const link = linkOf(a.link);
      return link ? { text: a.text.trim(), link } : { text: a.text.trim() };
    });
  return { body };
}

/** One outcome as the form holds it. */
export interface OutcomeForm {
  kind: OutcomeKind | "";
  text: string;
  link: string;
}

/** The body for `POST …/held`, or every problem. An outcome line with nothing in it is left out; a blank date is not
 * sent (the service takes the scheduled date). */
export function heldBody(form: {
  minutes: string;
  heldOn: string;
  outcomes: OutcomeForm[];
}): { body: HeldMeetingBody } | { problems: LineProblem[] } {
  const problems: LineProblem[] = [];
  if (form.minutes.trim() === "" || chars(form.minutes.trim()) > MINUTES_MAX)
    problems.push({ key: "p.meetings.problem_minutes", line: 0 });
  if (form.heldOn !== "" && !/^\d{4}-\d{2}-\d{2}$/.test(form.heldOn))
    problems.push({ key: "p.meetings.problem_date", line: 0 });
  const lines = form.outcomes
    .map((o, i) => ({ ...o, n: i + 1 }))
    .filter((o) => o.text.trim() !== "" || o.link !== "" || o.kind !== "");
  if (lines.length > ITEMS_MAX)
    problems.push({ key: "p.meetings.problem_outcome_count", line: 0 });
  for (const o of lines) {
    if (!(OUTCOME_KINDS as readonly string[]).includes(o.kind))
      problems.push({ key: "p.meetings.problem_outcome_kind", line: o.n });
    if (o.text.trim() === "" || chars(o.text.trim()) > TEXT_MAX)
      problems.push({ key: "p.meetings.problem_item_text", line: o.n });
  }
  if (problems.length > 0) return { problems };
  const body: HeldMeetingBody = { minutes: form.minutes.trim() };
  if (form.heldOn !== "") body.held_on = form.heldOn;
  if (lines.length > 0)
    body.outcomes = lines.map((o) => {
      const link = linkOf(o.link);
      const out = { kind: o.kind as OutcomeKind, text: o.text.trim() };
      return link ? { ...out, link } : out;
    });
  return { body };
}

/** Lower-case and collapse runs of whitespace, as the service does before looking for a passage in the minutes. */
export function squash(text: string): string {
  return text.split(/\s+/).filter(Boolean).join(" ").toLowerCase();
}

/** Whether `excerpt` is long enough to be evidence and is found in `minutes`. */
export function excerptFound(minutes: string, excerpt: string): boolean {
  const wanted = squash(excerpt);
  return chars(wanted) >= EXCERPT_MIN && squash(minutes).includes(wanted);
}

/** A date (`YYYY-MM-DD`) moved by `days`, in the calendar (no time zone). */
function shift(date: string, days: number): string {
  const d = new Date(`${date}T00:00:00Z`);
  d.setUTCDate(d.getUTCDate() + days);
  return d.toISOString().slice(0, 10);
}

/** The first and last due date allowed for a meeting held on `on`. */
export function dueWindow(on: string): { first: string; last: string } {
  return {
    first: shift(on, -WINDOW_BEFORE_DAYS),
    last: shift(on, WINDOW_AFTER_DAYS),
  };
}

/** Whether `due` falls inside the window around the meeting. */
export function dueInWindow(due: string, on: string): boolean {
  const w = dueWindow(on);
  return due >= w.first && due <= w.last;
}

/** The date the service judges due dates against: the day it was held, or else the day it was planned for. */
export const meetingDay = (
  m: Pick<Meeting, "held_on" | "scheduled_on">,
): string => m.held_on ?? m.scheduled_on;

/** The outcomes a task may still be proposed for: the actions of a held meeting with no task yet. */
export function openActions(m: Meeting): Meeting["outcomes"] {
  if (m.status !== "held" || !m.minutes) return [];
  return m.outcomes.filter((o) => o.kind === "action" && !o.task_pid);
}

/** One proposal as the form holds it: whether it is chosen, the outcome it is for, and every field as typed. A number
 * input binds a number once typed in, text before. */
export interface ProposalForm {
  chosen: boolean;
  outcome: number;
  title: string;
  excerpt: string;
  assignee: string;
  due: string;
  points: string | number;
}

/** A starting proposal for an action: its title is the action's text (cut to the longest title). */
export function proposalFor(o: Meeting["outcomes"][number]): ProposalForm {
  return {
    chosen: false,
    outcome: o.index,
    title: [...o.text.trim()].slice(0, TASK_TITLE_MAX).join(""),
    excerpt: "",
    assignee: "",
    due: "",
    points: "",
  };
}

/** A whole number from a form value, `null` for blank, `NaN` when it is not one. */
function whole(v: string | number): number | null {
  const raw = String(v ?? "").trim();
  if (raw === "") return null;
  const n = Number(raw);
  return Number.isSafeInteger(n) ? n : Number.NaN;
}

/** One problem with a proposal, and the outcome it is for (its position among the outcomes, 1-based). */
export interface ProposalProblem {
  key: StringKey;
  outcome: number;
}

/** The body for the preview and the commit, every problem, or `null` when no proposal is chosen (nothing is sent). */
export function proposalsBody(
  m: Pick<Meeting, "minutes" | "held_on" | "scheduled_on">,
  rows: ProposalForm[],
):
  { body: { items: TaskProposal[] } } | { problems: ProposalProblem[] } | null {
  const chosen = rows.filter((r) => r.chosen);
  if (chosen.length === 0) return null;
  const problems: ProposalProblem[] = [];
  if (chosen.length > PROPOSALS_MAX)
    problems.push({ key: "p.meetings.problem_proposals_count", outcome: 0 });
  const on = meetingDay(m);
  for (const r of chosen) {
    const n = r.outcome + 1;
    if (r.title.trim() === "" || chars(r.title.trim()) > TASK_TITLE_MAX)
      problems.push({ key: "p.meetings.problem_task_title", outcome: n });
    if (!excerptFound(m.minutes ?? "", r.excerpt))
      problems.push({ key: "p.meetings.problem_excerpt", outcome: n });
    if (r.due !== "" && !dueInWindow(r.due, on))
      problems.push({ key: "p.meetings.problem_due", outcome: n });
    const p = whole(r.points);
    if (p !== null && (Number.isNaN(p) || p < 0 || p > POINTS_MAX))
      problems.push({ key: "p.meetings.problem_points", outcome: n });
  }
  if (problems.length > 0) return { problems };
  return {
    body: {
      items: chosen.map((r) => {
        const item: TaskProposal = {
          title: r.title.trim(),
          excerpt: r.excerpt.trim(),
          outcome: r.outcome,
        };
        if (r.assignee.trim() !== "") item.assignee_ref = r.assignee.trim();
        if (r.due !== "") item.due_on = r.due;
        const p = whole(r.points);
        if (p !== null) item.points = p;
        return item;
      }),
    },
  };
}

/** The reasons the service gives for a dropped proposal, each with its sentence. */
const DROPPED: Record<string, StringKey> = {
  title_invalid: "p.meetings.dropped_title_invalid",
  excerpt_not_in_minutes: "p.meetings.dropped_excerpt_not_in_minutes",
  outcome_not_found: "p.meetings.dropped_outcome_not_found",
  outcome_not_an_action: "p.meetings.dropped_outcome_not_an_action",
  outcome_already_has_a_task: "p.meetings.dropped_outcome_already_has_a_task",
  due_outside_window: "p.meetings.dropped_due_outside_window",
  points_invalid: "p.meetings.dropped_points_invalid",
  assignee_not_in_plan: "p.meetings.dropped_assignee_not_in_plan",
  outcome_proposed_twice: "p.meetings.dropped_outcome_proposed_twice",
};

/** The sentence for a dropped proposal's reason code; one the page does not know is a general sentence. */
export const droppedKey = (reason: string): StringKey =>
  DROPPED[reason] ?? "p.meetings.dropped_other";

/** Whether every proposal of a preview is ready, so the tasks can be created. */
export const allReady = (p: TaskProposalPreview): boolean =>
  p.items.length > 0 && p.items.every((i) => i.status === "ready");

/** What a person does on this page. */
export type MeetingAction = "plan" | "held" | "cancel" | "preview" | "commit";

/**
 * The sentence for a refusal of `action`, or `null` for the general one. The service's codes are `not_found`,
 * `conflict` and `unprocessable`, one per status, so the status says which: a 404 is a meeting (or plan) gone; a 409
 * on recording or cancelling is a meeting already held or cancelled, and on proposing a meeting not held with minutes
 * or, when creating, an outcome that has a task meanwhile (one code for both); a 422 after the page's own checks is a
 * linked record no longer of the plan, or a proposal no longer ready.
 */
export function meetingRefusalKey(
  action: MeetingAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.meetings.refused_gone";
  if (err.status === 409)
    return action === "held" || action === "cancel"
      ? "p.meetings.refused_settled"
      : action === "commit"
        ? "p.meetings.refused_commit_state"
        : action === "preview"
          ? "p.meetings.refused_not_held"
          : null;
  if (err.status === 422)
    return action === "plan" || action === "held"
      ? "p.meetings.refused_link"
      : action === "commit"
        ? "p.meetings.refused_commit_invalid"
        : "p.meetings.refused_proposals";
  return null;
}
