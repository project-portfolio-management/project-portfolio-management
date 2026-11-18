// Links between tasks of different plans (NC-1 slice 12 part i, PJ-13, LT-10): listed, added and removed on the task
// page (`GET`/`POST /api/dependencies`, `DELETE /api/dependencies/{pid}`), and the tasks of other plans the critical
// path already carries drawn on `/plans/[pid]/critical-path`. The service's rules (`controllers/visibility/
// dependencies.rs`, `controllers/cpm.rs`), said and checked before anything is sent; the service still decides.
//
// A link joins a task this person can read to another they can read; one with an end they cannot read is opaque: it is
// counted, never named, and it is left out of the calculation. Its kind is finish-to-start (the default, not sent),
// start-to-start, finish-to-finish or start-to-finish; its lag is whole business days from -3650 to 3650 (negative is a
// lead; 0, the default, not sent). A link is not made twice in the same direction, nor in a loop (the service checks
// loops across plans), nor to a hammock. Each 422 is the same `unprocessable` with English text (a service gap).

import { ApiError } from "./api/client.js";
import type {
  Cpm,
  CpmEdge,
  CpmNode,
  DependencyBody,
  DependencyRow,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** The longest lag or lead, in business days. */
export const LAG_MAX = 3650;

/** The kinds of link, the default first. */
export const LINK_KINDS = ["fs", "ss", "ff", "sf"] as const;
export type LinkKind = (typeof LINK_KINDS)[number];

/** Each kind in words. */
export const LINK_KIND_KEY: Record<LinkKind, StringKey> = {
  fs: "p.crossplan.kind_fs",
  ss: "p.crossplan.kind_ss",
  ff: "p.crossplan.kind_ff",
  sf: "p.crossplan.kind_sf",
};

/** A full link (both ends readable). */
export type FullDependency = Exclude<DependencyRow, { restricted: true }>;

/** The links this person can read, and how many have an end they cannot. */
export function splitRows(rows: readonly DependencyRow[]): {
  links: FullDependency[];
  restricted: number;
} {
  const links: FullDependency[] = [];
  let restricted = 0;
  for (const r of rows) {
    if (r.restricted) restricted += 1;
    else links.push(r);
  }
  return { links, restricted };
}

/** The other end of a link from `taskPid`'s side, and whether this task comes first. */
export function otherEnd(
  row: FullDependency,
  taskPid: string,
): { pid: string; thisFirst: boolean } {
  return row.predecessor_pid === taskPid
    ? { pid: row.successor_pid, thisFirst: true }
    : { pid: row.predecessor_pid, thisFirst: false };
}

/** The link as asked. The lag is a number input's value (`null` or `undefined` when empty). */
export interface CrossLinkForm {
  thisFirst: boolean;
  planPid: string;
  otherPid: string;
  kind: LinkKind;
  lag: number | null | undefined;
}

export function emptyCrossLinkForm(): CrossLinkForm {
  return { thisFirst: true, planPid: "", otherPid: "", kind: "fs", lag: null };
}

/** The body to send, or every problem at once. */
export function crossLinkBody(
  form: CrossLinkForm,
  taskPid: string,
  links: readonly FullDependency[],
): { body: DependencyBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  if (!form.planPid) problems.push("p.crossplan.problem_plan");
  if (!form.otherPid) problems.push("p.crossplan.problem_task");
  const lag =
    form.lag === null || form.lag === undefined ? 0 : Number(form.lag);
  if (!Number.isSafeInteger(lag) || Math.abs(lag) > LAG_MAX)
    problems.push("p.crossplan.problem_lag");
  const predecessor = form.thisFirst ? taskPid : form.otherPid;
  const successor = form.thisFirst ? form.otherPid : taskPid;
  if (form.otherPid && form.otherPid === taskPid)
    problems.push("p.crossplan.problem_self");
  else if (
    form.otherPid &&
    links.some(
      (l) => l.predecessor_pid === predecessor && l.successor_pid === successor,
    )
  )
    problems.push("p.crossplan.problem_exists");
  if (problems.length > 0) return { problems };
  const body: DependencyBody = {
    predecessor_pid: predecessor,
    successor_pid: successor,
  };
  if (form.kind !== "fs") body.kind = form.kind;
  if (lag !== 0) body.lag_days = lag;
  return { body };
}

/** The tasks of other plans the critical path carries, by plan, in the order first met. */
export function externalPlans(
  cpm: Pick<Cpm, "nodes">,
): { planPid: string; planName: string; nodes: CpmNode[] }[] {
  const out: { planPid: string; planName: string; nodes: CpmNode[] }[] = [];
  for (const n of cpm.nodes) {
    if (!n.external) continue;
    const key = n.plan_pid ?? "";
    let group = out.find((g) => g.planPid === key);
    if (!group) {
      group = { planPid: key, planName: n.plan_name ?? "", nodes: [] };
      out.push(group);
    }
    group.nodes.push(n);
  }
  return out;
}

/** The links that cross into another plan, with both ends' names, this plan's task and the other plan's. */
export function crossPlanEdges(cpm: Pick<Cpm, "nodes" | "edges">): {
  edge: CpmEdge;
  from: CpmNode | undefined;
  to: CpmNode | undefined;
}[] {
  const byPid = new Map(cpm.nodes.map((n) => [n.pid, n]));
  return cpm.edges
    .filter((e) => e.cross_plan)
    .map((edge) => ({
      edge,
      from: byPid.get(edge.predecessor_pid),
      to: byPid.get(edge.successor_pid),
    }));
}

/** The catalogue sentence for a refusal, or `null` for the general rule. */
export function crossLinkRefusalKey(
  action: "add" | "remove" | "list",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (action === "add") {
    if (err.status === 404) return "p.crossplan.refused_unreadable";
    if (err.status === 409) return "p.crossplan.refused_exists";
    if (err.status === 422) return "p.crossplan.refused_add";
    return null;
  }
  if (action === "remove" && err.status === 404)
    return "p.crossplan.refused_gone";
  return null;
}
