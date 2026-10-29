// Impact / effort matrix and Now / Next / Later over ideas and proposals (NC-10).
//
// The service holds no impact or effort for an idea or a proposal, so this module does not invent
// one: a person rates each item here, and the rating stays in that person's browser. An item with
// either rating missing is "unrated", never placed. Pure: no runes, no DOM, no storage access.

import type { Idea, Proposal } from "./api/ppm-types.js";

export const LEVELS = ["low", "medium", "high"] as const;
export type Level = (typeof LEVELS)[number];
export type Horizon = "now" | "next" | "later" | "unrated";
export const HORIZONS: readonly Horizon[] = ["now", "next", "later", "unrated"];

export interface RoadmapItem {
  /** `idea:<pid>` or `proposal:<pid>`: stable across reloads, used as the ratings key. */
  id: string;
  kind: "idea" | "proposal";
  title: string;
}

export interface Rating {
  impact?: Level;
  effort?: Level;
}
export type Ratings = Record<string, Rating>;

/** Open ideas, and proposals that are still being decided (not rejected, not already promoted). */
export function roadmapItems(
  ideas: readonly Idea[],
  proposals: readonly Proposal[],
): RoadmapItem[] {
  return [
    ...ideas
      .filter((i) => i.status === "open")
      .map((i): RoadmapItem => ({
        id: `idea:${i.pid}`,
        kind: "idea",
        title: i.title,
      })),
    ...proposals
      .filter((p) => p.status !== "rejected" && p.status !== "promoted")
      .map((p): RoadmapItem => ({
        id: `proposal:${p.pid}`,
        kind: "proposal",
        title: p.title,
      })),
  ];
}

const RANK: Record<Level, number> = { low: 1, medium: 2, high: 3 };

/**
 * The horizon a rating implies. Value minus cost: impact rank (1 to 3) less effort rank (1 to 3).
 * 1 or more is Now; 0 with at least medium impact is Next; everything else is Later.
 */
export function horizonOf(rating: Rating | undefined): Horizon {
  if (!rating?.impact || !rating.effort) return "unrated";
  const gap = RANK[rating.impact] - RANK[rating.effort];
  if (gap >= 1) return "now";
  if (gap === 0 && RANK[rating.impact] >= 2) return "next";
  return "later";
}

/** The items rated exactly this impact and effort. */
export function inCell(
  items: readonly RoadmapItem[],
  ratings: Ratings,
  impact: Level,
  effort: Level,
): RoadmapItem[] {
  return items.filter((i) => {
    const r = ratings[i.id];
    return r?.impact === impact && r.effort === effort;
  });
}

/** The items on one horizon, in the order given. */
export function onHorizon(
  items: readonly RoadmapItem[],
  ratings: Ratings,
  horizon: Horizon,
): RoadmapItem[] {
  return items.filter((i) => horizonOf(ratings[i.id]) === horizon);
}

/** Set (or clear, with `null`) one half of an item's rating, without changing the input. */
export function withRating(
  ratings: Ratings,
  id: string,
  field: "impact" | "effort",
  value: Level | null,
): Ratings {
  const next: Rating = { ...ratings[id] };
  if (value === null) delete next[field];
  else next[field] = value;
  const out = { ...ratings };
  if (next.impact === undefined && next.effort === undefined) delete out[id];
  else out[id] = next;
  return out;
}

const isLevel = (v: unknown): v is Level =>
  typeof v === "string" && (LEVELS as readonly string[]).includes(v);

/** Read saved ratings; anything malformed is dropped rather than trusted. */
export function parseRatings(raw: string | null): Ratings {
  if (!raw) return {};
  let data: unknown;
  try {
    data = JSON.parse(raw);
  } catch {
    return {};
  }
  if (typeof data !== "object" || data === null || Array.isArray(data))
    return {};
  const out: Ratings = {};
  for (const [id, v] of Object.entries(data)) {
    if (typeof v !== "object" || v === null) continue;
    const { impact, effort } = v as Record<string, unknown>;
    const r: Rating = {};
    if (isLevel(impact)) r.impact = impact;
    if (isLevel(effort)) r.effort = effort;
    if (r.impact || r.effort) out[id] = r;
  }
  return out;
}

export function serializeRatings(ratings: Ratings): string {
  return JSON.stringify(ratings);
}
