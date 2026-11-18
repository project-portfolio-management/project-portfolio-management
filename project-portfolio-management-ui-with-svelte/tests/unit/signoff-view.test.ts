// NC-1 slice 11 (EX-4): which organizations must sign a milestone off, the set kept whole and the rules said first.
import { describe, expect, it } from "vitest";
import {
  REQUIRED_MAX,
  requiredPids,
  requiredProblem,
  samePids,
} from "../../src/lib/signoff-view";

const signoff = (pids: string[]) => ({
  state: "not_delivered" as const,
  delivered_on: null,
  met_on: null,
  outstanding: [],
  required: pids.map((p, i) => ({
    organization_pid: p,
    name: p,
    signed_on: i === 0 ? "2026-11-10" : null,
  })),
});

describe("the required set", () => {
  it("comes from `required`, signed or not, never from `outstanding`", () => {
    expect(requiredPids(signoff(["a", "b"]))).toEqual(["a", "b"]);
    expect(requiredPids(undefined)).toEqual([]);
  });
  it("is unchanged in any order", () => {
    expect(samePids(["a", "b"], ["b", "a"])).toBe(true);
    expect(samePids(["a"], ["a", "b"])).toBe(false);
    expect(samePids([], [])).toBe(true);
  });
  it("holds at most 20 and refuses an inactive organization by name", () => {
    const orgs = [
      { pid: "a", name: "Acme", active: true },
      { pid: "z", name: "Old", active: false },
    ];
    expect(
      requiredProblem(
        Array.from({ length: REQUIRED_MAX + 1 }, (_, i) => `o${i}`),
        orgs,
      ),
    ).toEqual({ key: "p.signoffs.problem_too_many" });
    expect(requiredProblem(["a", "z"], orgs)).toEqual({
      key: "p.signoffs.problem_inactive",
      name: "Old",
    });
    expect(requiredProblem(["a"], orgs)).toBeNull();
    expect(requiredProblem([], orgs)).toBeNull();
  });
});
