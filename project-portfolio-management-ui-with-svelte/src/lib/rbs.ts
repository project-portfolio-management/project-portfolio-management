// The resource breakdown structure (PJ-12): resources grouped by role and by skill.
// Pure and tested. Built from `GET /api/resource-plan`: roles come with the people
// in them (only for a reader who may manage people — otherwise a headcount only);
// skills carry demand but **no supply and no people**, because the service does not
// know either, and the view says so rather than inventing a figure.

import type { ResourcePlan } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export interface RbsPerson {
  ref: string;
}

export interface RbsRole {
  role: string;
  /** Named people; empty when the reader may not see them. */
  people: RbsPerson[];
  headcount: number;
  /** Demand summed across the window, in FTE-periods (a sum of per-period FTE). */
  demand_fte: number;
  supply_fte: number | null;
}

export interface RbsSkill {
  skill: string;
  demand_fte: number;
}

export interface Rbs {
  roles: RbsRole[];
  skills: RbsSkill[];
  /** True when person names were withheld. */
  masked: boolean;
  /** What the skill rows cannot say. */
  skillNote: string;
}

const sum = (xs: number[]) =>
  Math.round(xs.reduce((a, b) => a + b, 0) * 100) / 100;

/** The RBS from a by-role plan and (optionally) a by-skill plan. */
export function buildRbs(
  byRole: ResourcePlan,
  bySkill: ResourcePlan | null,
): Rbs {
  const keys = new Set<string>();
  for (const p of byRole.periods) for (const r of p.rows) keys.add(r.key);
  for (const k of Object.keys(byRole.people_by_role ?? {})) keys.add(k);
  const roles: RbsRole[] = [...keys].sort().map((role) => {
    const rows = byRole.periods.flatMap((p) =>
      p.rows.filter((r) => r.key === role),
    );
    const people = (byRole.people_by_role?.[role] ?? []).map((ref) => ({
      ref,
    }));
    const headcount = Math.max(
      people.length,
      ...rows.map((r) => r.headcount),
      0,
    );
    const supply = rows.some((r) => r.supply_fte !== null)
      ? sum(rows.map((r) => r.supply_fte ?? 0))
      : null;
    return {
      role,
      people,
      headcount,
      demand_fte: sum(rows.map((r) => r.demand_fte)),
      supply_fte: supply,
    };
  });
  const skillRows = new Map<string, number>();
  for (const p of bySkill?.periods ?? [])
    for (const r of p.rows)
      skillRows.set(r.key, (skillRows.get(r.key) ?? 0) + r.demand_fte);
  const skills: RbsSkill[] = [...skillRows.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([skill, demand]) => ({ skill, demand_fte: sum([demand]) }));
  return {
    roles,
    skills,
    masked: byRole.masked,
    skillNote:
      t("l.rbs.a_booking_that_needs_several"),
  };
}

/** A role in words: "Engineer: 3 people, demand 2.5 FTE, supply 3 FTE". */
export function roleText(r: RbsRole, masked: boolean): string {
  const people = masked
    ? t(r.headcount === 1 ? "l.rbs.headcount_one" : "l.rbs.headcount_other", { headcount: r.headcount })
    : t("l.rbs.named", { length: r.people.length });
  const supply =
    r.supply_fte === null ? t("l.rbs.supply_not_known") : t("l.rbs.supply_fte", { supply_fte: r.supply_fte });
  return t("l.rbs.demand_fte", { role: r.role, people, demand_fte: r.demand_fte, supply });
}
