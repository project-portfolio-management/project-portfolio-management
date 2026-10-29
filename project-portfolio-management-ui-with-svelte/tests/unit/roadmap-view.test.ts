import { describe, expect, it } from "vitest";
import {
  horizonOf,
  inCell,
  onHorizon,
  parseRatings,
  roadmapItems,
  serializeRatings,
  withRating,
  type Level,
  type Ratings,
} from "../../src/lib/roadmap-view";
import type { Idea, Proposal } from "../../src/lib/api/ppm-types";

const idea = (pid: string, status: Idea["status"] = "open"): Idea => ({
  pid,
  title: `Idea ${pid}`,
  pitch: null,
  tags: [],
  votes: 0,
  status,
  converted_proposal_pid: null,
});
const proposal = (pid: string, status: Proposal["status"]): Proposal => ({
  pid,
  title: `Proposal ${pid}`,
  summary: null,
  kind_target: "projects",
  sponsor_ref: null,
  strategic_rationale: null,
  requested_minor: null,
  currency: null,
  status,
  promoted_plan_pid: null,
});

describe("roadmapItems", () => {
  it("keeps open ideas and proposals still being decided", () => {
    const items = roadmapItems(
      [idea("a"), idea("b", "dismissed"), idea("c", "converted")],
      [
        proposal("d", "draft"),
        proposal("e", "rejected"),
        proposal("f", "promoted"),
        proposal("g", "approved"),
      ],
    );
    expect(items.map((i) => i.id)).toEqual([
      "idea:a",
      "proposal:d",
      "proposal:g",
    ]);
  });
});

describe("horizonOf", () => {
  const h = (impact?: Level, effort?: Level) => horizonOf({ impact, effort });
  it("is unrated until both are set", () => {
    expect(horizonOf(undefined)).toBe("unrated");
    expect(h("high")).toBe("unrated");
    expect(h(undefined, "low")).toBe("unrated");
  });
  it("puts value above cost in Now", () => {
    expect(h("high", "low")).toBe("now");
    expect(h("high", "medium")).toBe("now");
    expect(h("medium", "low")).toBe("now");
  });
  it("puts balanced, worthwhile work in Next", () => {
    expect(h("high", "high")).toBe("next");
    expect(h("medium", "medium")).toBe("next");
  });
  it("puts the rest in Later", () => {
    expect(h("low", "low")).toBe("later");
    expect(h("medium", "high")).toBe("later");
    expect(h("low", "high")).toBe("later");
  });
});

describe("placement", () => {
  const items = roadmapItems([idea("a"), idea("b")], [proposal("c", "draft")]);
  const ratings = withRating(
    withRating({}, "idea:a", "impact", "high"),
    "idea:a",
    "effort",
    "low",
  );
  it("finds an item in its cell and its horizon only when rated", () => {
    expect(inCell(items, ratings, "high", "low").map((i) => i.id)).toEqual([
      "idea:a",
    ]);
    expect(inCell(items, ratings, "high", "high")).toEqual([]);
    expect(onHorizon(items, ratings, "now").map((i) => i.id)).toEqual([
      "idea:a",
    ]);
    expect(onHorizon(items, ratings, "unrated").map((i) => i.id)).toEqual([
      "idea:b",
      "proposal:c",
    ]);
  });
});

describe("withRating", () => {
  it("does not change its input and drops an emptied rating", () => {
    const a = withRating({}, "x", "impact", "low");
    expect(a).toEqual({ x: { impact: "low" } });
    const b = withRating(a, "x", "impact", null);
    expect(b).toEqual({});
    expect(a).toEqual({ x: { impact: "low" } });
  });
});

describe("saved ratings", () => {
  it("round-trips", () => {
    const r: Ratings = { "idea:a": { impact: "high", effort: "low" } };
    expect(parseRatings(serializeRatings(r))).toEqual(r);
  });
  it("drops what it cannot trust", () => {
    expect(parseRatings(null)).toEqual({});
    expect(parseRatings("not json")).toEqual({});
    expect(parseRatings("[1]")).toEqual({});
    expect(
      parseRatings(
        JSON.stringify({
          a: { impact: "huge", effort: "low" },
          b: 3,
          c: { impact: "x" },
        }),
      ),
    ).toEqual({ a: { effort: "low" } });
  });
});
