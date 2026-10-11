// Pure helpers for the critical chain, buffer and drum pages (CC-6): what a zone or an outcome looks like
// (a word and a shape as well as a colour), where a fever-chart point and its zone bands are drawn, and
// where a bar sits on a timeline. No DOM, so each rule is unit-tested.

import type { FeverLines } from "#lib/api/ppm.js";

export type Zone = "green" | "amber" | "red";
export type Tone = "green" | "amber" | "red" | "grey";

export interface Look {
  /** The catalogue key of the word. */
  word: string;
  /** A shape; the colour is never the only signal. */
  symbol: string;
  tone: Tone;
}

/** A buffer zone: green is a circle, amber a diamond, red a triangle. */
export function zoneLook(zone: Zone | null | undefined): Look {
  switch (zone) {
    case "green":
      return {
        word: "p.critical_chain.zone_green",
        symbol: "●",
        tone: "green",
      };
    case "amber":
      return {
        word: "p.critical_chain.zone_amber",
        symbol: "◆",
        tone: "amber",
      };
    case "red":
      return { word: "p.critical_chain.zone_red", symbol: "▲", tone: "red" };
    default:
      return { word: "p.critical_chain.zone_none", symbol: "–", tone: "grey" };
  }
}

/** What the pipeline proposes for a project. */
export function outcomeLook(outcome: string): Look {
  switch (outcome) {
    case "as_planned":
      return {
        word: "p.critical_chain.outcome_as_planned",
        symbol: "●",
        tone: "green",
      };
    case "staggered":
      return {
        word: "p.critical_chain.outcome_staggered",
        symbol: "◆",
        tone: "amber",
      };
    case "exceeds_drum":
      return {
        word: "p.critical_chain.outcome_exceeds_drum",
        symbol: "▲",
        tone: "red",
      };
    default:
      return {
        word: "p.critical_chain.outcome_no_slot",
        symbol: "▲",
        tone: "red",
      };
  }
}

/** The chart's drawing box: a square plot inside a margin. */
export interface Box {
  width: number;
  height: number;
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export const FEVER_BOX: Box = {
  width: 260,
  height: 200,
  left: 38,
  top: 10,
  right: 10,
  bottom: 34,
};

const clamp = (v: number, lo: number, hi: number): number =>
  Math.min(hi, Math.max(lo, v));

/** A data point (x, y as percent 0 to 100) as a pixel pair; y grows upwards in the data. */
export function toPixel(box: Box, x: number, y: number): [number, number] {
  const w = box.width - box.left - box.right;
  const h = box.height - box.top - box.bottom;
  return [
    round(box.left + (clamp(x, 0, 100) / 100) * w),
    round(box.top + h - (clamp(y, 0, 100) / 100) * h),
  ];
}

function round(n: number): number {
  return Math.round(n * 100) / 100;
}

const asPoints = (box: Box, pts: [number, number][]): string =>
  pts.map(([x, y]) => toPixel(box, x, y).join(",")).join(" ");

/** The three zone bands as SVG polygon points, from the configured lines (heights clamped to the chart). */
export function bands(box: Box, lines: FeverLines): Record<Zone, string> {
  const ga: [number, number] = [
    clamp(lines.green_amber.at_start, 0, 100),
    clamp(lines.green_amber.at_end, 0, 100),
  ];
  const ar: [number, number] = [
    clamp(lines.amber_red.at_start, 0, 100),
    clamp(lines.amber_red.at_end, 0, 100),
  ];
  return {
    green: asPoints(box, [
      [0, 0],
      [100, 0],
      [100, ga[1]],
      [0, ga[0]],
    ]),
    amber: asPoints(box, [
      [0, ga[0]],
      [100, ga[1]],
      [100, ar[1]],
      [0, ar[0]],
    ]),
    red: asPoints(box, [
      [0, ar[0]],
      [100, ar[1]],
      [100, 100],
      [0, 100],
    ]),
  };
}

/** A shape for the point, drawn at (x, y): circle, diamond or triangle matching the zone symbol. */
export function markerPath(zone: Zone, x: number, y: number, r = 6): string {
  if (zone === "green")
    return `M ${x - r} ${y} a ${r} ${r} 0 1 0 ${2 * r} 0 a ${r} ${r} 0 1 0 ${-2 * r} 0`;
  if (zone === "amber")
    return `M ${x} ${y - r} L ${x + r} ${y} L ${x} ${y + r} L ${x - r} ${y} Z`;
  return `M ${x} ${y - r} L ${x + r} ${y + r} L ${x - r} ${y + r} Z`;
}

/** Whole days from one ISO date to another (negative when `to` is earlier). */
export function daysBetween(from: string, to: string): number {
  const utc = (d: string): number => {
    const [y = 1970, m = 1, day = 1] = d.slice(0, 10).split("-").map(Number);
    return Date.UTC(y, m - 1, day);
  };
  return Math.round((utc(to) - utc(from)) / 86_400_000);
}

export interface Span {
  start: string;
  end: string;
}

export interface Bar {
  /** Percent from the left edge. */
  left: number;
  /** Percent of the width; never so thin it vanishes. */
  width: number;
}

/** Place spans on a shared timeline from the earliest start to the latest end. */
export function layoutBars(spans: Span[], minWidth = 1.5): Bar[] {
  const head = spans[0];
  if (!head) return [];
  const first = spans.reduce((m, s) => (s.start < m ? s.start : m), head.start);
  const last = spans.reduce((m, s) => (s.end > m ? s.end : m), head.end);
  const total = Math.max(1, daysBetween(first, last) + 1);
  return spans.map((s) => {
    const left = (daysBetween(first, s.start) / total) * 100;
    const width = Math.max(
      minWidth,
      ((daysBetween(s.start, s.end) + 1) / total) * 100,
    );
    return {
      left: round(Math.min(left, 100 - minWidth)),
      width: round(Math.min(width, 100 - Math.min(left, 100 - minWidth))),
    };
  });
}

/** A feeding or project buffer key, shown as a short label: `project`, else the key's tail. */
export function bufferKeyLabel(
  key: string,
  names: Record<string, string>,
  project: string,
): string {
  return key === "project" ? project : (names[key] ?? key);
}
