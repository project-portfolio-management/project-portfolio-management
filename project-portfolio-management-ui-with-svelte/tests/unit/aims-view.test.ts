// NC-1 slice 11 (DP-4): strategic aims and the portfolio map: the aims and programmes read from the map, the checks
// before sending, and the map drawn by the director pack's own renderer.
import { describe, expect, it } from "vitest";
import type { PortfolioMap } from "../../src/lib/api/ppm-types";
import {
  aimsOf,
  mapPeriodProblem,
  primaryProblem,
  programmesOf,
} from "../../src/lib/aims-view";
import { buildPackView } from "../../src/lib/pack-view";

const MAP: PortfolioMap = {
  period: "2026-11",
  programmes_total: 3,
  groups: [
    {
      aim_pid: "a1",
      aim: "Better care",
      programmes: [{ plan_pid: "p2", name: "Wards" }],
    },
    { aim_pid: "a2", aim: "Lower cost", programmes: [] },
    {
      aim_pid: null,
      aim: "no strategic aim",
      programmes: [
        { plan_pid: "p1", name: "Apps" },
        { plan_pid: "p3", name: "Beds" },
      ],
    },
  ],
};

describe("aims and programmes from the map", () => {
  it("lists every aim, an empty one too, and not the no-aim group", () => {
    expect(aimsOf(MAP)).toEqual([
      { pid: "a1", title: "Better care" },
      { pid: "a2", title: "Lower cost" },
    ]);
  });
  it("lists each programme once, by name, with its primary aim or none", () => {
    expect(programmesOf(MAP)).toEqual([
      { pid: "p1", name: "Apps", primary: null },
      { pid: "p3", name: "Beds", primary: null },
      { pid: "p2", name: "Wards", primary: "a1" },
    ]);
  });
});

describe("checks before sending", () => {
  it("takes a month or nothing for the period", () => {
    expect(mapPeriodProblem("")).toBeNull();
    expect(mapPeriodProblem("2026-11")).toBeNull();
    expect(mapPeriodProblem("2026-13")).toBe("p.aims.problem_period");
  });
  it("needs a programme and an aim that is not already its primary", () => {
    expect(primaryProblem(undefined, "a1")).toBe("p.aims.problem_choose");
    expect(primaryProblem({ primary: null }, "")).toBe("p.aims.problem_choose");
    expect(primaryProblem({ primary: "a1" }, "a1")).toBe(
      "p.aims.problem_already_primary",
    );
    expect(primaryProblem({ primary: "a1" }, "a2")).toBeNull();
  });
});

describe("the map drawn as the pack draws it", () => {
  it("has a map section with a row per programme and the no-aim group in the catalogue's words", () => {
    const sections = buildPackView(
      { portfolio_map: MAP },
      { t: (k) => k, label: (_g, v) => v, locale: "en-001" },
    );
    const map = sections.find((s) => s.id === "map")!;
    const rows = map.tables.flatMap((tb) => tb.rows);
    expect(rows).toHaveLength(3);
    expect(JSON.stringify(rows)).toContain("p.director_pack.no_aim");
    expect(JSON.stringify(rows)).not.toContain("no strategic aim");
  });
});
