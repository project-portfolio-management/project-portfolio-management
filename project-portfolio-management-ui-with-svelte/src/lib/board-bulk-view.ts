// Many tasks at once on the board (`/plans/[pid]/board`, NC-1 slice 12 part j, RM-9): the same fields set on the
// chosen tasks (`POST /api/tasks/bulk`), the chosen tasks copied or moved to a plan (`POST /api/tasks/copy`, `/move`)
// and deleted one call each after a question (`DELETE /api/plans/{pid}/tasks/{t_pid}`). The service's rules
// (`controllers/task_bulk.rs`, `src/task_bulk.rs`), said and checked before anything is sent; the service still
// decides.
//
// At most 200 tasks a call. A bulk edit sets the assignee (a `person:` or `worker:` reference and an id), the due date
// and the start date (1970-01-05 to 2100-12-31), each left as it is, set or cleared; the status is never bulk-edited
// (a move on the board). A copy is a new task in the destination's first state, its assignee kept only when asked. A
// move clears the sprint, component and parent, and a status the destination lacks becomes its first; a task with
// sub-tasks is refused. Every answer is one result per task; a refusal's reason is the service's English (a service
// gap), so the page names the tasks refused and not why. Every whole-call refusal is one 422 `unprocessable`.

import { ApiError } from "./api/client.js";
import type {
  BulkEditBody,
  BulkResult,
  BulkTransferBody,
  Task,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { isPersonRef } from "./access-view.js";
import { isIsoDate } from "./paste-view.js";

/** The most tasks one call takes. */
export const BULK_MAX = 200;

/** The earliest and latest start date the service takes. */
export const START_MIN = "1970-01-05";
export const START_MAX = "2100-12-31";

/** What a bulk edit does to one field. */
export type FieldMode = "keep" | "set" | "clear";

/** One field of the bulk edit as typed. */
export interface FieldForm {
  mode: FieldMode;
  value: string;
}

/** The bulk edit as typed. */
export interface BulkEditForm {
  assignee: FieldForm;
  due: FieldForm;
  start: FieldForm;
}

/** Every field left as it is. */
export function emptyBulkEditForm(): BulkEditForm {
  return {
    assignee: { mode: "keep", value: "" },
    due: { mode: "keep", value: "" },
    start: { mode: "keep", value: "" },
  };
}

/** The problems with the choice of tasks itself. */
function choiceProblems(ids: readonly string[]): StringKey[] {
  if (ids.length === 0) return ["p.bulk.problem_none"];
  if (new Set(ids).size > BULK_MAX) return ["p.bulk.problem_too_many"];
  return [];
}

const fieldValue = (f: FieldForm): string | null | undefined =>
  f.mode === "keep" ? undefined : f.mode === "clear" ? null : f.value.trim();

/**
 * The body to send, every problem at once, or `null` when every chosen task already has what would be set (nothing
 * is sent). A start date is always a change: the board does not read it.
 */
export function bulkEditBody(
  ids: readonly string[],
  form: BulkEditForm,
  tasks: readonly Pick<Task, "pid" | "assignee_ref" | "due_date">[],
): { body: BulkEditBody } | { problems: StringKey[] } | null {
  const problems = choiceProblems(ids);
  const assignee = fieldValue(form.assignee);
  const due = fieldValue(form.due);
  const start = fieldValue(form.start);
  if (assignee === undefined && due === undefined && start === undefined)
    problems.push("p.bulk.problem_no_field");
  if (typeof assignee === "string" && !isPersonRef(assignee))
    problems.push("p.bulk.problem_assignee");
  if (typeof due === "string" && !isIsoDate(due))
    problems.push("p.bulk.problem_due");
  if (
    typeof start === "string" &&
    (!isIsoDate(start) || start < START_MIN || start > START_MAX)
  )
    problems.push("p.bulk.problem_start");
  if (problems.length > 0) return { problems };
  const set: BulkEditBody["set"] = {};
  if (assignee !== undefined) set.assignee_ref = assignee;
  if (due !== undefined) set.due_date = due;
  if (start !== undefined) set.start_date = start;
  const chosen = tasks.filter((x) => ids.includes(x.pid));
  const unchanged =
    start === undefined &&
    chosen.length === new Set(ids).size &&
    chosen.every(
      (x) =>
        (assignee === undefined || (x.assignee_ref ?? null) === assignee) &&
        (due === undefined || (x.due_date ?? null) === due),
    );
  if (unchanged) return null;
  return { body: { ids: [...new Set(ids)], set } };
}

/** Copy or move. */
export type TransferMode = "copy" | "move";

/**
 * The body to send or every problem at once. A move to the plan the tasks are in is refused first (the service
 * refuses each task); a copy may go to the same plan. `keep_assignee` is sent only for a copy that keeps it.
 */
export function transferBody(
  mode: TransferMode,
  ids: readonly string[],
  toPlan: string,
  currentPlan: string,
  keepAssignee: boolean,
): { body: BulkTransferBody } | { problems: StringKey[] } {
  const problems = choiceProblems(ids);
  if (!toPlan) problems.push("p.bulk.problem_plan");
  else if (mode === "move" && toPlan === currentPlan)
    problems.push("p.bulk.problem_same_plan");
  if (problems.length > 0) return { problems };
  const body: BulkTransferBody = {
    ids: [...new Set(ids)],
    to_plan_pid: toPlan,
  };
  if (mode === "copy" && keepAssignee) body.keep_assignee = true;
  return { body };
}

/** What a bulk answer comes to: how many were done, the titles of those refused, and (a move) how many were mapped. */
export interface BulkOutcome {
  done: number;
  requested: number;
  refused: string[];
  mapped: number;
}

/**
 * The outcome in the page's words. A move's note on a task done says its status was mapped (the note itself is the
 * service's English and is not shown); a copy's note names the new task and is not read. A refused task the page does
 * not know is named by the start of its id.
 */
export function bulkOutcome(
  mode: "edit" | TransferMode,
  result: BulkResult,
  titleOf: (pid: string) => string | undefined,
): BulkOutcome {
  const refused = result.results
    .filter((r) => r.result === "refused")
    .map((r) => titleOf(r.pid) ?? r.pid.slice(0, 8));
  const mapped =
    mode === "move"
      ? result.results.filter((r) => r.result === "ok" && r.note !== null)
          .length
      : 0;
  return {
    done: result.ok,
    requested: result.requested,
    refused,
    mapped,
  };
}

/** The problem with deleting the chosen tasks, or `null`. */
export function deleteProblem(ids: readonly string[]): StringKey | null {
  return ids.length === 0 ? "p.bulk.problem_none" : null;
}

/** The catalogue sentence for a refused bulk call, or `null` for the general rule. */
export function bulkRefusalKey(
  action: "edit" | TransferMode,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 422)
    return action === "edit"
      ? "p.bulk.refused_edit"
      : "p.bulk.refused_transfer";
  if (err.status === 404) return "p.bulk.refused_plan_gone";
  if (err.status === 403) return "p.bulk.refused_forbidden";
  return null;
}

/** Whether a refused delete means the task is gone already (counted as gone, not as a failure). */
export function deleteGone(err: unknown): boolean {
  return err instanceof ApiError && err.status === 404;
}
