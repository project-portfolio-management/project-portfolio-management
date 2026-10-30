// FB-4: a programme's people impact, as the service passes it through from the
// workforce service (WPM). Pure: the page renders what this returns.
//
// WPM's rules are kept: each part is known or unknown with WPM's reason; a count
// under WPM's floor is `null` with `hidden: true` and is shown as hidden, never
// as zero; there is no total, so none is made here.

/** A count of people as WPM gives it. */
export interface ImpactCount {
  count: number | null;
  hidden: boolean;
}

/** One pool's count. */
export interface ImpactPoolCount extends ImpactCount {
  pool_pid: string;
  pool: string | null;
}

/** A part counting people by pool. */
export interface ImpactPeoplePart {
  known: boolean;
  reason: string | null;
  pools: ImpactPoolCount[] | null;
  in_no_pool: ImpactCount | null;
  end_not_recorded?: ImpactCount | null;
}

/** The over-committed pool-months part. */
export interface ImpactOverPart {
  known: boolean;
  reason: string | null;
  considered: number | null;
  over: number | null;
  unknown: number | null;
  pools:
    | {
        pool_pid: string;
        pool: string | null;
        over_months: string[];
        unknown_months: string[];
      }[]
    | null;
}

/** `GET /api/plans/{pid}/people-impact`. */
export interface PeopleImpact {
  plan_pid: string;
  programme_ref: string;
  source: "wpm";
  status: "live" | "unknown";
  reason: string | null;
  as_of: string | null;
  plan_ends_on: string | null;
  plan_ends_on_reason: string | null;
  floor: number | null;
  workers_committed: ImpactPeoplePart;
  contingent_ending_before_end: ImpactPeoplePart;
  over_committed_pool_months: ImpactOverPart;
}

/** One line of a people part: a pool (or no pool, or no end recorded) and its count. */
export interface ImpactRow {
  /** The pool's name, or the pool id when WPM gave no name; `null` for the two fixed rows. */
  name: string | null;
  /** `pool`, `in_no_pool` or `end_not_recorded`. */
  kind: "pool" | "in_no_pool" | "end_not_recorded";
  /** The count, or `null` when hidden. */
  count: number | null;
  hidden: boolean;
}

/** A part as shown: its rows when known, else WPM's reason. */
export type ImpactPartView =
  { known: true; rows: ImpactRow[] } | { known: false; reason: string };

/** The rows of a people part, in WPM's order. A count WPM hides stays `null` and hidden. */
export function peopleRows(
  part: ImpactPeoplePart,
  fallback: string,
): ImpactPartView {
  if (!part.known) return { known: false, reason: part.reason ?? fallback };
  // A count that is not there is shown as hidden, never as zero.
  const cell = (c: ImpactCount) => ({
    count: c.hidden ? null : c.count,
    hidden: c.hidden || c.count === null,
  });
  const rows: ImpactRow[] = (part.pools ?? []).map((p) => ({
    name: p.pool ?? p.pool_pid,
    kind: "pool",
    ...cell(p),
  }));
  if (part.in_no_pool)
    rows.push({ name: null, kind: "in_no_pool", ...cell(part.in_no_pool) });
  if (part.end_not_recorded)
    rows.push({
      name: null,
      kind: "end_not_recorded",
      ...cell(part.end_not_recorded),
    });
  return { known: true, rows };
}
