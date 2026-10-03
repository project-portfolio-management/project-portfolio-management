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

/** The slice of a capacity row the model needs. */
export interface CapacityPerson {
  person_ref: string;
  allocated_percent: number;
  over_allocated: boolean;
  capacity: { source: "default" | "availability"; min_percent: number };
  peak_percent: number;
  over_capacity: OverCapacitySpan[];
  proposed: { percent: number; allocations: number; would_exceed_capacity: boolean };
}

/** What the meter draws. */
export interface LoadModel {
  /** The meter's full width in percent of time (a multiple of 25, at least 100). */
  scale: number;
  committedPct: number;
  /** Width of the proposed extension after the firm bar, in percent of the meter. */
  proposedPct: number;
  /** Where the capacity line sits, in percent of the meter. */
  capacityPct: number;
  state: "over" | "proposed_over" | "ok";
  marker: string;
  label: string;
}

const clampPct = (n: number) => Math.max(0, Math.min(100, n));

/** The meter model for one person. */
export function loadModel(p: CapacityPerson): LoadModel {
  const committed = Math.max(0, p.allocated_percent);
  const proposed = Math.max(0, p.proposed.percent);
  const capacity = Math.max(0, p.capacity.min_percent);
  const need = Math.max(100, committed, committed + proposed, capacity);
  const scale = Math.ceil(need / 25) * 25;
  const state: LoadModel["state"] = p.over_allocated ? "over" : p.proposed.would_exceed_capacity ? "proposed_over" : "ok";
  const marker = state === "over" ? "!" : state === "proposed_over" ? "◐" : "○";
  const parts: string[] = [];
  parts.push(
    p.capacity.source === "availability"
      ? `${committed}% committed against a capacity as low as ${capacity}%`
      : `${committed}% committed`,
  );
  if (p.over_capacity.length > 0) {
    const worst = p.over_capacity.reduce((a, b) => (b.demand - b.capacity > a.demand - a.capacity ? b : a));
    parts.push(`over capacity from ${worst.from}${worst.to === worst.from ? "" : ` to ${worst.to}`} (${worst.demand}% asked, ${worst.capacity}% available)`);
  } else if (p.over_allocated) {
    parts.push("over capacity");
  }
  if (p.proposed.allocations > 0) {
    parts.push(
      `${p.proposed.percent}% proposed, not counted${p.proposed.would_exceed_capacity ? "; committing it would put them over capacity" : ""}`,
    );
  }
  return {
    scale,
    committedPct: clampPct((committed / scale) * 100),
    proposedPct: clampPct(((committed + proposed) / scale) * 100) - clampPct((committed / scale) * 100),
    capacityPct: clampPct((capacity / scale) * 100),
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
export function spanSegments(from: string, to: string, spans: readonly OverCapacitySpan[]): SpanSegment[] {
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
        label: `${s.from}${s.to === s.from ? "" : ` to ${s.to}`}: ${s.demand}% asked, ${s.capacity}% available`,
      },
    ];
  });
}

/** Whether an error from the commit call is the "would be over capacity" 409 that `confirm` can override. */
export function isOverCapacityConflict(status: number | undefined, body: unknown): boolean {
  if (status !== 409) return false;
  try {
    return JSON.stringify(body ?? "").includes("over_capacity");
  } catch {
    return false;
  }
}
