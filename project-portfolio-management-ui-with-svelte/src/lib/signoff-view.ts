// Which organizations must sign a milestone off (EX-4, NC-1 slice 11): the service's rules, said before anything is
// sent. `PUT …/signoff-required` replaces the whole set, so the page starts from the milestone's `signoff.required`
// (every required organization, signed or not), never from `outstanding`, which drops one once it signs.

import type { MilestoneSignoff, Organization } from "./api/ppm.js";

/** The most organizations one milestone may require. */
export const REQUIRED_MAX = 20;

/** The pids a milestone requires now, in the service's order. */
export function requiredPids(signoff: MilestoneSignoff | undefined): string[] {
  return (signoff?.required ?? []).map((r) => r.organization_pid);
}

/** Whether two sets of pids are the same, in any order. */
export function samePids(a: readonly string[], b: readonly string[]): boolean {
  const x = new Set(a);
  return x.size === new Set(b).size && b.every((p) => x.has(p));
}

/**
 * What is wrong with choosing `chosen`, or `null`. The service needs every organization to exist and be active; an
 * inactive one that is still required must be unticked by the reader, not dropped silently.
 */
export function requiredProblem(
  chosen: readonly string[],
  orgs: readonly Pick<Organization, "pid" | "name" | "active">[],
):
  | { key: "p.signoffs.problem_too_many" }
  | { key: "p.signoffs.problem_inactive"; name: string }
  | null {
  if (new Set(chosen).size > REQUIRED_MAX)
    return { key: "p.signoffs.problem_too_many" };
  const byPid = new Map(orgs.map((o) => [o.pid, o]));
  for (const p of chosen) {
    const o = byPid.get(p);
    if (o && !o.active)
      return { key: "p.signoffs.problem_inactive", name: o.name };
  }
  return null;
}
