// Pure helpers for strategic aims and the live portfolio map (`/portfolio-map`, DP-4, NC-1 slice 11): which objectives
// are aims and which programmes there are, both read from the map (every aim is a group of it, with or without
// programmes), and the checks before anything is sent.

import type { StringKey } from "./i18n.svelte.js";
import type { PortfolioMap } from "./api/ppm-types.js";

/** The strategic aims, in the map's order: every group but the last, "no strategic aim" (`aim_pid: null`). */
export function aimsOf(
  map: PortfolioMap,
): Array<{ pid: string; title: string }> {
  return map.groups.flatMap((g) =>
    g.aim_pid ? [{ pid: g.aim_pid, title: g.aim }] : [],
  );
}

/** Every programme on the map once, with its primary aim (`null` for none), by name. */
export function programmesOf(
  map: PortfolioMap,
): Array<{ pid: string; name: string; primary: string | null }> {
  return map.groups
    .flatMap((g) =>
      g.programmes.map((p) => ({
        pid: p.plan_pid,
        name: p.name,
        primary: g.aim_pid,
      })),
    )
    .sort((a, b) => a.name.localeCompare(b.name));
}

/** The problem with a period, or `null`: a month written `YYYY-MM`, or empty for the current one. */
export function mapPeriodProblem(period: string): StringKey | null {
  return period === "" || /^\d{4}-(0[1-9]|1[0-2])$/.test(period)
    ? null
    : "p.aims.problem_period";
}

/** What choosing a primary aim sends, or the problem: both chosen, and not already that programme's primary. */
export function primaryProblem(
  programme: { primary: string | null } | undefined,
  aimPid: string,
): StringKey | null {
  if (!programme || aimPid === "") return "p.aims.problem_choose";
  if (programme.primary === aimPid) return "p.aims.problem_already_primary";
  return null;
}
