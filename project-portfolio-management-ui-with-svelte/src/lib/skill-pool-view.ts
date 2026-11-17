// Pure helpers for adding and changing skill pools (`/skill-pools`, DP-10, NC-1 slice 11): a list of roles or skills
// as the service keeps it, the checks the service makes before anything is sent (a name of 2 to 120 characters, not
// taken; a role or skill in at most one pool), and what saving a change sends.

import type { StringKey } from "./i18n.svelte.js";
import type { SkillPool } from "./api/ppm-types.js";

export const POOL_NAME_MIN = 2;
export const POOL_NAME_MAX = 120;

/** A typed list as the service keeps it: split on commas and lines, trimmed, blanks dropped, repeats (in any case) once. */
export function parseList(text: string): string[] {
  const seen = new Set<string>();
  const out: string[] = [];
  for (const v of text.split(/[,\n]/)) {
    const t = v.trim();
    if (t && !seen.has(t.toLowerCase())) {
      seen.add(t.toLowerCase());
      out.push(t);
    }
  }
  return out;
}

/** The roles and skills that another pool already holds (compared in any case), with that pool's name. */
export function clashes(
  roles: readonly string[],
  skills: readonly string[],
  pools: readonly SkillPool[],
  self: string | null,
): Array<{ value: string; pool: string }> {
  const out: Array<{ value: string; pool: string }> = [];
  for (const p of pools) {
    if (p.pid === self) continue;
    const theirs = (list: readonly string[]) =>
      new Set(list.map((v) => v.toLowerCase()));
    const r = theirs(p.roles);
    const s = theirs(p.skills);
    for (const v of roles)
      if (r.has(v.toLowerCase())) out.push({ value: v, pool: p.name });
    for (const v of skills)
      if (s.has(v.toLowerCase())) out.push({ value: v, pool: p.name });
  }
  return out;
}

/** The problem with a new pool's name, or `null`: 2 to 120 characters, and no other pool of that name (any case). */
export function poolNameProblem(
  name: string,
  pools: readonly SkillPool[],
): StringKey | null {
  const n = name.trim();
  const len = [...n].length;
  if (len < POOL_NAME_MIN || len > POOL_NAME_MAX)
    return "p.skill_pools.problem_name";
  if (pools.some((p) => p.name.toLowerCase() === n.toLowerCase()))
    return "p.skill_pools.problem_name_taken";
  return null;
}

/** What saving a pool's roles and skills sends: each list only when it changed, so an unchanged one is kept. */
export function membersBody(
  pool: SkillPool,
  roles: readonly string[],
  skills: readonly string[],
): { roles?: string[]; skills?: string[] } | null {
  const same = (a: readonly string[], b: readonly string[]) =>
    a.length === b.length &&
    a.every((v, i) => v.toLowerCase() === b[i]!.toLowerCase());
  const out: { roles?: string[]; skills?: string[] } = {};
  if (!same(roles, pool.roles)) out.roles = [...roles];
  if (!same(skills, pool.skills)) out.skills = [...skills];
  return Object.keys(out).length > 0 ? out : null;
}
