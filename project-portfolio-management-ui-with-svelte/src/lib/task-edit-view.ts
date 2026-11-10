// Editing a task and its links to other tasks (NC-1 slice 3, LT-10): turning what a person typed into the request
// body, checking the rules the service would otherwise refuse, and the words for each link from this task's side.
// The rules mirror the service's `validate_task` (`src/controllers/engineering/board.rs`: a title of at most 1024
// bytes, an assignee that is a `person:` or `worker:` reference to a UUID, whole points from 0 to 100) and
// `src/task_relations.rs` (five kinds; no link to itself; no second copy of a link; `precedes` needs the other task
// due strictly after this one). The service decides; this only saves a round trip and lets the page say which field
// is wrong, in the reader's language.

import type {
  Task,
  TaskRelation,
  TaskRelationKind,
  UpdateTaskBody,
} from "./api/ppm-types.js";

/** The service's cap on a task title, in bytes (`crate::validation::MAX_TEXT_LEN`). */
export const MAX_TITLE_BYTES = 1024;
/** The service's story-point range. */
export const MIN_POINTS = 0;
export const MAX_POINTS = 100;

/** The link kinds, in the order the add form offers them. */
export const RELATION_KINDS: readonly TaskRelationKind[] = [
  "blocks",
  "precedes",
  "related",
  "duplicates",
  "copied_from",
];

/** What the edit form holds: every field a string, so an empty box is "". */
export interface TaskForm {
  title: string;
  description: string;
  assignee: string;
  points: string;
  dueDate: string;
}

/** The first thing wrong with the edit form, or `null` when it can be sent. */
export type TaskProblem =
  | "title_required"
  | "title_too_long"
  | "assignee_not_reference"
  | "points_not_whole"
  | "points_out_of_range"
  | "due_not_date"
  | null;

/** The form for editing `task`. */
export function formFromTask(task: Task): TaskForm {
  return {
    title: task.title,
    description: task.description ?? "",
    assignee: task.assignee_ref ?? "",
    points: task.points === null ? "" : String(task.points),
    dueDate: task.due_date ?? "",
  };
}

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const WHOLE = /^\d+$/;
const DATE = /^\d{4}-\d{2}-\d{2}$/;

/** Whether `value` is a `person:` or `worker:` reference to a UUID, as the service requires of an assignee. */
export function isPersonRef(value: string): boolean {
  const at = value.indexOf(":");
  if (at < 0) return false;
  const scheme = value.slice(0, at);
  return (
    (scheme === "person" || scheme === "worker") &&
    UUID.test(value.slice(at + 1))
  );
}

/** The first thing wrong with the form. */
export function taskProblem(f: TaskForm): TaskProblem {
  const title = f.title.trim();
  if (title === "") return "title_required";
  if (new TextEncoder().encode(title).length > MAX_TITLE_BYTES)
    return "title_too_long";
  const assignee = f.assignee.trim();
  if (assignee !== "" && !isPersonRef(assignee))
    return "assignee_not_reference";
  const points = f.points.trim();
  if (points !== "") {
    if (!WHOLE.test(points)) return "points_not_whole";
    const n = Number(points);
    if (n < MIN_POINTS || n > MAX_POINTS) return "points_out_of_range";
  }
  const due = f.dueDate.trim();
  if (
    due !== "" &&
    (!DATE.test(due) || Number.isNaN(Date.parse(`${due}T00:00:00Z`)))
  )
    return "due_not_date";
  return null;
}

const orNull = (s: string): string | null =>
  s.trim() === "" ? null : s.trim();

/**
 * The request body for the form. Every field is sent, because the edit replaces them all: the fields this form does
 * not show (its component, parent task and sprint) go back as `task` has them, so an edit never moves the task.
 */
export function taskBody(f: TaskForm, task: Task): UpdateTaskBody {
  const points = f.points.trim();
  return {
    title: f.title.trim(),
    description: orNull(f.description),
    assignee_ref: orNull(f.assignee),
    points: points === "" ? null : Number(points),
    due_date: orNull(f.dueDate),
    component_pid: task.component_pid ?? null,
    parent_task_pid: task.parent_task_pid ?? null,
    sprint_pid: task.sprint_pid ?? null,
  };
}

/** The catalogue key for a link as read from this task's side (`Blocks`, `Blocked by`, …). */
export function relationWordsKey(
  r: Pick<TaskRelation, "kind" | "direction">,
):
  | "p.task.rel_related"
  | "p.task.rel_blocks"
  | "p.task.rel_blocked_by"
  | "p.task.rel_precedes"
  | "p.task.rel_follows"
  | "p.task.rel_duplicates"
  | "p.task.rel_duplicated_by"
  | "p.task.rel_copied_from"
  | "p.task.rel_copied_to" {
  const out = r.direction === "outgoing";
  switch (r.kind) {
    case "related":
      return "p.task.rel_related";
    case "blocks":
      return out ? "p.task.rel_blocks" : "p.task.rel_blocked_by";
    case "precedes":
      return out ? "p.task.rel_precedes" : "p.task.rel_follows";
    case "duplicates":
      return out ? "p.task.rel_duplicates" : "p.task.rel_duplicated_by";
    case "copied_from":
      return out ? "p.task.rel_copied_from" : "p.task.rel_copied_to";
  }
}

/** The first thing wrong with adding a link of `kind` from `task` to `target`, or `null`. */
export type RelationProblem =
  "choose_task" | "self" | "exists" | "cycle" | "precedes_dates" | null;

/**
 * Checks the page can make before asking: a chosen task, not this one, no second copy of a link (a `related` link
 * either way is the same link), no two-task loop of `blocks` or `precedes`, and for `precedes` both due dates in
 * order when both are set. A longer loop, and a task that has gone, are the service's to find (a 422).
 */
export function relationProblem(
  kind: TaskRelationKind,
  task: Pick<Task, "pid" | "due_date">,
  target: Pick<Task, "pid" | "due_date"> | null,
  existing: readonly TaskRelation[],
): RelationProblem {
  if (!target) return "choose_task";
  if (target.pid === task.pid) return "self";
  const same = existing.filter(
    (r) => r.kind === kind && r.other_task.pid === target.pid,
  );
  if (same.some((r) => r.direction === "outgoing" || kind === "related"))
    return "exists";
  if (
    (kind === "blocks" || kind === "precedes") &&
    same.some((r) => r.direction === "incoming")
  )
    return "cycle";
  if (
    kind === "precedes" &&
    task.due_date &&
    target.due_date &&
    target.due_date <= task.due_date
  ) {
    return "precedes_dates";
  }
  return null;
}

/** A step a person takes from this screen. */
export type TaskStep = "edit" | "add_link" | "remove_link";

/**
 * The catalogue sentence for a refusal of `step`, by HTTP status: 403 that the caller may not change this plan's
 * tasks, 404 that the task or link has gone, 409 that the link already exists, 422 what the service did not accept.
 * `null` for anything else, so the page uses the general rule. The task routes take no `If-Match`, so there is no 412.
 */
export function taskRefusalKey(
  step: TaskStep,
  status: number,
):
  | "p.task.refused_forbidden"
  | "p.task.refused_gone"
  | "p.task.refused_link_gone"
  | "p.task.refused_link_exists"
  | "p.task.refused_edit_invalid"
  | "p.task.refused_link_invalid"
  | null {
  if (status === 403) return "p.task.refused_forbidden";
  if (status === 404)
    return step === "remove_link"
      ? "p.task.refused_link_gone"
      : "p.task.refused_gone";
  if (status === 409 && step === "add_link")
    return "p.task.refused_link_exists";
  if (status === 422)
    return step === "edit"
      ? "p.task.refused_edit_invalid"
      : "p.task.refused_link_invalid";
  return null;
}
