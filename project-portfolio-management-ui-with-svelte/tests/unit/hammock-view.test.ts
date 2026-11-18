// NC-1 slice 12 part (i), PJ-7: hammocks' rules, checked before anything is sent.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { Cpm, Hammock } from "../../src/lib/api/ppm-types";
import {
  hammockRefusalKey,
  linkProblem,
  makeProblem,
  unscheduledKey,
} from "../../src/lib/hammock-view";

const node = (pid: string, over: Record<string, unknown> = {}) =>
  ({
    pid,
    duration_days: null,
    work_minutes: null,
    constraint: null,
    external: false,
    ...over,
  }) as unknown as Cpm["nodes"][number];
const edge = (a: string, b: string) =>
  ({
    pid: `${a}${b}`,
    predecessor_pid: a,
    successor_pid: b,
    kind: "fs",
    lag_days: 0,
  }) as Cpm["edges"][number];
const hammock = (pid: string, links: string[]): Hammock => ({
  pid,
  title: pid,
  span: { state: "unscheduled", reason: "no_links" },
  links: links.map((l) => ({ pid: l, title: l, is_hammock: false })),
});

describe("making a task a hammock", () => {
  const cpm = {
    nodes: [
      node("free"),
      node("timed", { duration_days: 3 }),
      node("worked", { work_minutes: 60 }),
      node("pinned", { constraint: "snet" }),
      node("linked"),
      node("other", { external: true, duration_days: 2 }),
    ],
    edges: [edge("linked", "free2")],
  };
  it("needs a task", () => {
    expect(makeProblem("", cpm)).toBe("p.hammock.problem_choose_task");
  });
  it("refuses a task that carries a duration, work or a constraint", () => {
    expect(makeProblem("timed", cpm)).toBe("p.hammock.problem_carries_work");
    expect(makeProblem("worked", cpm)).toBe("p.hammock.problem_carries_work");
    expect(makeProblem("pinned", cpm)).toBe("p.hammock.problem_carries_work");
  });
  it("refuses a task with a dependency either way", () => {
    expect(makeProblem("linked", cpm)).toBe(
      "p.hammock.problem_has_dependencies",
    );
    expect(makeProblem("free2", cpm)).toBe(
      "p.hammock.problem_has_dependencies",
    );
  });
  it("allows a bare task, one not in the network, and leaves the rest to the service without a schedule", () => {
    expect(makeProblem("free", cpm)).toBeNull();
    expect(makeProblem("unscheduled-task", cpm)).toBeNull();
    expect(makeProblem("timed", null)).toBeNull();
  });
});

describe("linking a hammock", () => {
  const hs = [
    hammock("h1", ["t1", "h2"]),
    hammock("h2", ["h3"]),
    hammock("h3", []),
  ];
  it("needs a member, not itself, and not one already spanned", () => {
    expect(linkProblem("h1", "", hs)).toBe("p.hammock.problem_choose_member");
    expect(linkProblem("h1", "h1", hs)).toBe("p.hammock.problem_self");
    expect(linkProblem("h1", "t1", hs)).toBe("p.hammock.problem_linked");
  });
  it("refuses a loop of hammocks, however deep", () => {
    expect(linkProblem("h3", "h1", hs)).toBe("p.hammock.problem_loop");
    expect(linkProblem("h2", "h1", hs)).toBe("p.hammock.problem_loop");
  });
  it("allows a nested hammock that makes no loop, and a plain task", () => {
    expect(linkProblem("h1", "h3", hs)).toBeNull();
    expect(linkProblem("h3", "t9", hs)).toBeNull();
  });
});

describe("a hammock's span and its refusals", () => {
  it("says why a span has no dates", () => {
    expect(
      unscheduledKey({
        state: "scheduled",
        start: "2026-11-02",
        finish: "2026-11-06",
      }),
    ).toBeNull();
    expect(unscheduledKey({ state: "unscheduled", reason: "no_links" })).toBe(
      "p.hammock.no_links",
    );
    expect(
      unscheduledKey({ state: "unscheduled", reason: "members_unscheduled" }),
    ).toBe("p.hammock.members_unscheduled");
  });
  it("names each refusal by action and status, never the service's text", () => {
    const e = (s: number) =>
      new ApiError(s, "service text", {
        code: "unprocessable",
        detail: "service text",
      });
    expect(hammockRefusalKey("make", e(422))).toBe("p.hammock.refused_make");
    expect(hammockRefusalKey("link", e(422))).toBe("p.hammock.refused_link");
    expect(hammockRefusalKey("link", e(404))).toBe("p.hammock.refused_gone");
    expect(hammockRefusalKey("unlink", e(404))).toBe(
      "p.hammock.refused_link_gone",
    );
    expect(hammockRefusalKey("unmake", e(500))).toBeNull();
    expect(hammockRefusalKey("make", new Error("x"))).toBeNull();
  });
});
