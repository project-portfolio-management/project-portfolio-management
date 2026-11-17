// NC-1 slice 11 (DP-10): adding and changing skill pools: a list kept as the service keeps it, a role or skill in at
// most one pool (said first), the name checks, and only the lists that changed sent.
import { describe, expect, it } from "vitest";
import type { SkillPool } from "../../src/lib/api/ppm-types";
import {
  clashes,
  membersBody,
  parseList,
  poolNameProblem,
} from "../../src/lib/skill-pool-view";

const pool = (
  pid: string,
  name: string,
  roles: string[],
  skills: string[] = [],
): SkillPool => ({
  pid,
  name,
  active: true,
  roles,
  skills,
});
const POOLS = [
  pool("p1", "Data", ["Data engineer"], ["SQL"]),
  pool("p2", "Design", ["Designer"]),
];

describe("a list of roles or skills", () => {
  it("splits on commas and lines, trims, drops blanks and keeps a repeat once in any case", () => {
    expect(parseList(" Analyst, ,data engineer\nAnalyst\n analyst ")).toEqual([
      "Analyst",
      "data engineer",
    ]);
  });
});

describe("a role or skill in at most one pool", () => {
  it("names each one another pool holds, in any case, and not the pool's own", () => {
    expect(clashes(["data ENGINEER", "Tester"], ["sql"], POOLS, null)).toEqual([
      { value: "data ENGINEER", pool: "Data" },
      { value: "sql", pool: "Data" },
    ]);
    expect(clashes(["Data engineer"], ["SQL"], POOLS, "p1")).toEqual([]);
  });
});

describe("a pool's name and members", () => {
  it("needs a name of 2 to 120 characters that no other pool has", () => {
    expect(poolNameProblem(" x ", POOLS)).toBe("p.skill_pools.problem_name");
    expect(poolNameProblem(" DATA ", POOLS)).toBe(
      "p.skill_pools.problem_name_taken",
    );
    expect(poolNameProblem("Delivery", POOLS)).toBeNull();
  });
  it("sends only the lists that changed, and nothing when neither did", () => {
    const p = POOLS[0]!;
    expect(membersBody(p, ["data engineer"], ["SQL"])).toBeNull();
    expect(membersBody(p, ["Data engineer", "Analyst"], ["SQL"])).toEqual({
      roles: ["Data engineer", "Analyst"],
    });
    expect(membersBody(p, ["Data engineer"], [])).toEqual({ skills: [] });
  });
});
