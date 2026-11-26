/**
 * Pure helpers behind the capacity view (PJ-3, PJ-4). No DOM, no Svelte, no
 * clock: unit-tested.
 *
 * The meter compares **firm** bookings with the capacity in force, and draws the
 * **proposed** ones as a separate, hatched extension: a proposal is never added
 * into the firm figure, only shown next to it.
 */

import { dayNumber } from "./lily-grids";
import type { OverCapacitySpan } from "./api/ppm";
import { t } from "./i18n.svelte.js";
import { formatPercentValue } from "./locale-format.js";

/** The slice of a capacity row the model needs. */
export interface CapacityPerson {
  person_ref: string;
  allocated_percent: number;
  over_allocated: boolean | null;
  capacity: {
    source: "availability" | "partly_declared" | "undeclared";
    min_percent: number | null;
  };
  peak_percent: number;
  over_capacity: OverCapacitySpan[];
  proposed: {
    percent: number;
    allocations: number;
    would_exceed_capacity: boolean | null;
  };
}

/** What the meter draws. */
export interface LoadModel {
  /** The meter's full width in percent of time (a multiple of 25, at least 100). */
  scale: number;
  committedPct: number;
  /** Width of the proposed extension after the firm bar, in percent of the meter. */
  proposedPct: number;
  /** Where the capacity line sits, in percent of the meter; `null` when no capacity is declared in the window. */
  capacityPct: number | null;
  /** `unknown`: no day is over, and some day with work has no declared capacity (CF-7). */
  state: "over" | "proposed_over" | "unknown" | "ok";
  marker: string;
  label: string;
}

const clampPct = (n: number) => Math.max(0, Math.min(100, n));

/** The meter model for one person. */
export function loadModel(p: CapacityPerson): LoadModel {
  const committed = Math.max(0, p.allocated_percent);
  const proposed = Math.max(0, p.proposed.percent);
  const declared = p.capacity.min_percent;
  const capacity = declared === null ? null : Math.max(0, declared);
  const need = Math.max(100, committed, committed + proposed, capacity ?? 0);
  const scale = Math.ceil(need / 25) * 25;
  const state: LoadModel["state"] =
    p.over_allocated === true
      ? "over"
      : p.proposed.would_exceed_capacity === true
        ? "proposed_over"
        : p.over_allocated === null
          ? "unknown"
          : "ok";
  const marker =
    state === "over"
      ? "!"
      : state === "proposed_over"
        ? "◐"
        : state === "unknown"
          ? "?"
          : "○";
  const parts: string[] = [];
  parts.push(
    capacity !== null
      ? t("l.capacity_view.committed_against_a_capacity_as", {
          committed,
          capacity,
        })
      : t("l.capacity_view.committed", { committed }),
  );
  // An undeclared day over 100% is over whatever could be declared; its excess is measured against 100.
  const excess = (s: OverCapacitySpan) => s.demand - (s.capacity ?? 100);
  if (p.over_capacity.length > 0) {
    const worst = p.over_capacity.reduce((a, b) =>
      excess(b) > excess(a) ? b : a,
    );
    const v =
      worst.to === worst.from ? "" : t("l.capacity_view.to", { to: worst.to });
    parts.push(
      worst.capacity === null
        ? t("l.capacity_view.over_capacity_from_asked_undeclared", {
            from: worst.from,
            v,
            demand: formatPercentValue(worst.demand),
          })
        : t("l.capacity_view.over_capacity_from_asked_available", {
            from: worst.from,
            v,
            demand: worst.demand,
            capacity: worst.capacity,
          }),
    );
  } else if (p.over_allocated === true) {
    parts.push(t("l.capacity_view.over_capacity"));
  } else if (p.over_allocated === null) {
    parts.push(t("l.capacity_view.capacity_undeclared"));
  }
  if (p.proposed.allocations > 0) {
    parts.push(
      t("l.capacity_view.proposed_not_counted", {
        percent: p.proposed.percent,
        v: p.proposed.would_exceed_capacity
          ? t("l.capacity_view.committing_it_would_put_them")
          : "",
      }),
    );
  }
  return {
    scale,
    committedPct: clampPct((committed / scale) * 100),
    proposedPct:
      clampPct(((committed + proposed) / scale) * 100) -
      clampPct((committed / scale) * 100),
    capacityPct: capacity === null ? null : clampPct((capacity / scale) * 100),
    state,
    marker,
    label: parts.join("; "),
  };
}

/** One over-capacity run placed on the window's timeline. */
export interface SpanSegment {
  startPct: number;
  widthPct: number;
  label: string;
}

/** Place over-capacity runs on the `[from, to]` window as percentages of its width. */
export function spanSegments(
  from: string,
  to: string,
  spans: readonly OverCapacitySpan[],
): SpanSegment[] {
  const a = dayNumber(from);
  const b = dayNumber(to);
  if (a === null || b === null || b < a) return [];
  const days = b - a + 1;
  return spans.flatMap((s) => {
    const sa = dayNumber(s.from);
    const sb = dayNumber(s.to);
    if (sa === null || sb === null) return [];
    const start = Math.max(sa, a);
    const end = Math.min(sb, b);
    if (end < start) return [];
    return [
      {
        startPct: ((start - a) / days) * 100,
        widthPct: ((end - start + 1) / days) * 100,
        label:
          s.capacity === null
            ? t("l.capacity_view.asked_undeclared", {
                from: s.from,
                v: s.to === s.from ? "" : t("l.capacity_view.to", { to: s.to }),
                demand: formatPercentValue(s.demand),
              })
            : t("l.capacity_view.asked_available", {
                from: s.from,
                v: s.to === s.from ? "" : t("l.capacity_view.to", { to: s.to }),
                demand: s.demand,
                capacity: s.capacity,
              }),
      },
    ];
  });
}

/** Whether an error from the commit call is the "would be over capacity" 409 that `confirm` can override. */
export function isOverCapacityConflict(
  status: number | undefined,
  body: unknown,
): boolean {
  if (status !== 409) return false;
  try {
    return JSON.stringify(body ?? "").includes("over_capacity");
  } catch {
    return false;
  }
}
