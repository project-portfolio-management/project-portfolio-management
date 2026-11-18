// Hammocks (`/plans/[pid]/critical-path`, NC-1 slice 12 part i, PJ-7): a summary task whose dates are derived from the
// tasks of the same plan it links, from the earliest start to the latest finish. The service's rules
// (`controllers/hammock.rs`, `domain/src/hammock.rs`), said and checked before anything is sent; the service still
// decides.
//
// A hammock is not in the dependency network, so it never changes a task's float or the finish. A task becomes one only
// when it carries no duration, work, constraint or dependency (the start date it had is cleared); turning a hammock
// back into a task removes every link it has. A hammock links to a task or another hammock of the same plan, not to
// itself, not twice, and not in a loop of hammocks. Every one of those refusals is the same 422 `unprocessable` with
// English text (a service gap), so the page checks them all first and says a general sentence for a 422.

import { ApiError } from "./api/client.js";
import type { Cpm, Hammock, HammockSpan } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** Why a hammock has no dates, in words. */
export function unscheduledKey(span: HammockSpan): StringKey | null {
  if (span.state === "scheduled") return null;
  return span.reason === "no_links"
    ? "p.hammock.no_links"
    : "p.hammock.members_unscheduled";
}

/** Why a task cannot become a hammock, from what the critical path says of it, or `null` when it may. */
export function makeProblem(
  taskPid: string,
  cpm: Pick<Cpm, "nodes" | "edges"> | null,
): StringKey | null {
  if (!taskPid) return "p.hammock.problem_choose_task";
  if (!cpm) return null;
  const node = cpm.nodes.find((n) => n.pid === taskPid && !n.external);
  if (
    node &&
    ((node.duration_days ?? null) !== null ||
      (node.work_minutes ?? 0) > 0 ||
      node.constraint !== null)
  )
    return "p.hammock.problem_carries_work";
  if (
    cpm.edges.some(
      (e) => e.predecessor_pid === taskPid || e.successor_pid === taskPid,
    )
  )
    return "p.hammock.problem_has_dependencies";
  return null;
}

/** Whether `from` reaches `to` through hammock links (a member that is itself a hammock). */
function reaches(
  from: string,
  to: string,
  byPid: ReadonlyMap<string, Hammock>,
): boolean {
  const seen = new Set<string>();
  const stack = [from];
  while (stack.length > 0) {
    const at = stack.pop() as string;
    if (at === to) return true;
    if (seen.has(at)) continue;
    seen.add(at);
    for (const l of byPid.get(at)?.links ?? []) stack.push(l.pid);
  }
  return false;
}

/** Why a link from `hammockPid` to `memberPid` would be refused, or `null`. */
export function linkProblem(
  hammockPid: string,
  memberPid: string,
  hammocks: readonly Hammock[],
): StringKey | null {
  if (!memberPid) return "p.hammock.problem_choose_member";
  if (memberPid === hammockPid) return "p.hammock.problem_self";
  const byPid = new Map(hammocks.map((h) => [h.pid, h]));
  const h = byPid.get(hammockPid);
  if (h?.links.some((l) => l.pid === memberPid))
    return "p.hammock.problem_linked";
  if (byPid.has(memberPid) && reaches(memberPid, hammockPid, byPid))
    return "p.hammock.problem_loop";
  return null;
}

/** An action on hammocks. */
export type HammockAction = "make" | "unmake" | "link" | "unlink";

/** The catalogue sentence for a refusal, or `null` for the general rule. */
export function hammockRefusalKey(
  action: HammockAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404)
    return action === "unlink"
      ? "p.hammock.refused_link_gone"
      : "p.hammock.refused_gone";
  if (err.status === 422)
    return action === "make"
      ? "p.hammock.refused_make"
      : "p.hammock.refused_link";
  return null;
}
