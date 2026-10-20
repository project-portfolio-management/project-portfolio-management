import { t } from "./i18n.svelte.js";
// The work breakdown structure (PJ-12): a plan's tasks as a tree, with generated
// codes (`1`, `1.2`, `1.2.3`). Pure and tested.
//
// **A WBS code is for reading, never for finding.** It changes whenever a sibling is
// added before it, removed, or a task moves under another parent, so nothing may
// store it, link to it or send it to the API: every row is keyed, and links are
// built, from the task's own pid. A task whose parent is not in the list the reader
// was given (it is not theirs to see) hangs under a "Restricted item" placeholder —
// with no name and no id of what it hides.

export interface WbsInput {
  pid: string;
  title: string;
  parent_task_pid?: string | null;
  /** Position among the siblings set by a reorder; a task without one follows the positioned ones. */
  sibling_position?: number | null;
  created_at: string;
}

export interface WbsRow {
  /** The task's own pid (a placeholder's is `restricted-N`). */
  key: string;
  /** Display only: recomputed from the tree every time. */
  code: string;
  depth: number;
  title: string;
  /** Direct children. */
  children: number;
  /** A stand-in for a parent the reader may not see. */
  restricted: boolean;
}

/**
 * Sibling order: by the position a reorder set, those without one after them, then oldest
 * first, the pid as the tie-break so the order is total.
 */
const byAge = (a: WbsInput, b: WbsInput) =>
  (a.sibling_position ?? Infinity) !== (b.sibling_position ?? Infinity)
    ? (a.sibling_position ?? Infinity) < (b.sibling_position ?? Infinity)
      ? -1
      : 1
    : a.created_at < b.created_at
    ? -1
    : a.created_at > b.created_at
      ? 1
      : a.pid < b.pid
        ? -1
        : 1;

/** The tasks in tree order with their generated codes. */
export function buildWbs(tasks: readonly WbsInput[]): WbsRow[] {
  const known = new Set(tasks.map((t) => t.pid));
  const kids = new Map<string | null, WbsInput[]>();
  const hidden = new Map<string, WbsInput[]>();
  for (const t of tasks) {
    const parent = t.parent_task_pid ?? null;
    if (parent === t.pid) continue; // its own parent: shown with the cycles below
    if (parent === null) kids.set(null, [...(kids.get(null) ?? []), t]);
    else if (known.has(parent) && parent !== t.pid)
      kids.set(parent, [...(kids.get(parent) ?? []), t]);
    else hidden.set(parent, [...(hidden.get(parent) ?? []), t]);
  }
  const out: WbsRow[] = [];
  const seen = new Set<string>();
  const walk = (list: WbsInput[], prefix: string, depth: number) => {
    list.sort(byAge).forEach((t, i) => {
      if (seen.has(t.pid)) return; // a cycle in the data: never loop
      seen.add(t.pid);
      const code = prefix ? `${prefix}.${i + 1}` : String(i + 1);
      const own = kids.get(t.pid) ?? [];
      out.push({
        key: t.pid,
        code,
        depth,
        title: t.title,
        children: own.length,
        restricted: false,
      });
      walk(own, code, depth + 1);
    });
  };
  // Top-level tasks first, then each group of tasks under a parent the reader cannot see.
  const roots = [...(kids.get(null) ?? [])];
  walk(roots, "", 0);
  let n = roots.length;
  let placeholder = 0;
  for (const group of hidden.values()) {
    n += 1;
    placeholder += 1;
    const code = String(n);
    out.push({
      key: `restricted-${placeholder}`,
      code,
      depth: 0,
      title: t("l.wbs.restricted_item"),
      children: group.length,
      restricted: true,
    });
    walk(group, code, 1);
  }
  // Anything left is part of a cycle: show it as top-level rather than lose it.
  const left = tasks.filter((t) => !seen.has(t.pid));
  if (left.length) walk([...left], "", 0);
  return out;
}

/**
 * The order to send to `PUT /api/plans/{pid}/task-order` to move a task one place among its
 * siblings, or null at the edge, for a task the reader cannot see, or one in a cycle.
 * `order` names every sibling the reader has, in the new order; the service refuses it
 * (409) if the real children differ, which is how a hidden sibling is never reordered blind.
 */
export function siblingMove(
  tasks: readonly WbsInput[],
  pid: string,
  direction: "up" | "down",
): { parent_task_pid: string | null; order: string[] } | null {
  const task = tasks.find((x) => x.pid === pid);
  if (!task) return null;
  const parent = task.parent_task_pid ?? null;
  if (parent !== null && !tasks.some((x) => x.pid === parent)) return null;
  const siblings = tasks
    .filter((x) => (x.parent_task_pid ?? null) === parent && x.pid !== parent)
    .sort(byAge);
  const at = siblings.findIndex((x) => x.pid === pid);
  const to = direction === "up" ? at - 1 : at + 1;
  if (at < 0 || to < 0 || to >= siblings.length) return null;
  const order = siblings.map((x) => x.pid);
  [order[at], order[to]] = [order[to]!, order[at]!];
  return { parent_task_pid: parent, order };
}

/** Whether a string looks like a WBS code (so a caller can refuse one where an id belongs). */
export function looksLikeWbsCode(s: string): boolean {
  return /^\d+(\.\d+)*$/.test(s);
}

/** A task's address: always its pid, never its code. */
export function taskHref(planPid: string, taskPid: string): string {
  return `/plans/${planPid}/tasks/${taskPid}`;
}

/** The most rows the nested tree draws; beyond it the table alone is shown. */
export const WBS_TREE_LIMIT = 500;
