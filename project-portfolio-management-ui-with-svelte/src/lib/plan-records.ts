// The plan's records a meeting's agenda or outcome, or a lesson, may point at (NC-1 slice 12 part e): its tasks,
// risks, milestones and issues, each best effort (a kind that cannot be read is simply not offered), and the words
// for a link. Shared by `/plans/[pid]/meetings` and `/plans/[pid]/lessons`.

import type { PpmClient } from "./api/ppm.js";
import { tDynamic } from "./i18n.svelte.js";
import { isLinkKind, type LinkKind } from "./meeting-view.js";

/** The plan's records by kind, each its `pid` and its own title. */
export type PlanRecords = Partial<
  Record<LinkKind, Array<{ pid: string; title: string }>>
>;

/** Read the plan's tasks, risks, milestones and issues; a kind whose read fails is left out. */
export async function loadPlanRecords(
  ppm: PpmClient,
  pid: string,
): Promise<PlanRecords> {
  const out: PlanRecords = {};
  await Promise.all([
    ppm
      .listTasks(pid)
      .then(
        (r) =>
          (out.task = r.tasks.map((x) => ({ pid: x.pid, title: x.title }))),
      )
      .catch(() => {}),
    ppm
      .listRisks(pid)
      .then((r) => (out.risk = r.map((x) => ({ pid: x.pid, title: x.title }))))
      .catch(() => {}),
    ppm
      .milestones(pid)
      .then(
        (r) => (out.milestone = r.map((x) => ({ pid: x.pid, title: x.name }))),
      )
      .catch(() => {}),
    ppm
      .planIssues(pid)
      .then(
        (r) =>
          (out.issue = r.issues.map((x) => ({ pid: x.pid, title: x.title }))),
      )
      .catch(() => {}),
  ]);
  return out;
}

/** "Task: Order the servers": a record's kind in words and its own title; a kind this page does not name is "Record". */
export function recordWords(kind: string, title: string): string {
  return tDynamic(`p.status.kind_${isLinkKind(kind) ? kind : "other"}`, {
    title,
  });
}

/** The title of the record `kind:pid` among `records`, or `null` when it is not listed. */
export function recordTitle(
  records: PlanRecords,
  kind: string,
  pid: string,
): string | null {
  if (!isLinkKind(kind)) return null;
  return records[kind]?.find((r) => r.pid === pid)?.title ?? null;
}
