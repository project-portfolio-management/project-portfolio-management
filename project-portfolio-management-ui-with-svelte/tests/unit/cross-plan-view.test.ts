// NC-1 slice 12 part (i), PJ-13 and LT-10: links between tasks of different plans.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { Cpm, DependencyRow } from "../../src/lib/api/ppm-types";
import {
  crossLinkBody,
  crossLinkRefusalKey,
  crossPlanEdges,
  emptyCrossLinkForm,
  externalPlans,
  otherEnd,
  splitRows,
  type FullDependency,
} from "../../src/lib/cross-plan-view";

const link = (a: string, b: string): FullDependency => ({
  pid: `${a}-${b}`,
  predecessor_pid: a,
  successor_pid: b,
  kind: "fs",
  lag_days: 0,
});

describe("the links touching a task", () => {
  it("keeps the readable ones and counts the restricted, never naming them", () => {
    const rows: DependencyRow[] = [
      link("me", "x"),
      { pid: "r1", restricted: true, label: "service text" },
      { pid: "r2", restricted: true, label: "service text" },
    ];
    const r = splitRows(rows);
    expect(r.links.map((l) => l.pid)).toEqual(["me-x"]);
    expect(r.restricted).toBe(2);
    expect(JSON.stringify(r.links)).not.toContain("service text");
  });
  it("names the other end from this task's side", () => {
    expect(otherEnd(link("me", "x"), "me")).toEqual({
      pid: "x",
      thisFirst: true,
    });
    expect(otherEnd(link("y", "me"), "me")).toEqual({
      pid: "y",
      thisFirst: false,
    });
  });
});

describe("a new link's body", () => {
  const form = { ...emptyCrossLinkForm(), planPid: "p2", otherPid: "x" };
  it("sends only the two ends for the defaults", () => {
    expect(crossLinkBody(form, "me", [])).toEqual({
      body: { predecessor_pid: "me", successor_pid: "x" },
    });
    expect(
      crossLinkBody({ ...form, thisFirst: false, lag: 0 }, "me", []),
    ).toEqual({
      body: { predecessor_pid: "x", successor_pid: "me" },
    });
  });
  it("sends a kind and a lag or lead only when they are not the defaults", () => {
    expect(crossLinkBody({ ...form, kind: "ss", lag: -2 }, "me", [])).toEqual({
      body: {
        predecessor_pid: "me",
        successor_pid: "x",
        kind: "ss",
        lag_days: -2,
      },
    });
  });
  it("says every problem at once", () => {
    expect(
      crossLinkBody({ ...emptyCrossLinkForm(), lag: 3651 }, "me", []),
    ).toEqual({
      problems: [
        "p.crossplan.problem_plan",
        "p.crossplan.problem_task",
        "p.crossplan.problem_lag",
      ],
    });
    expect(crossLinkBody({ ...form, lag: 1.5 }, "me", [])).toEqual({
      problems: ["p.crossplan.problem_lag"],
    });
    expect(crossLinkBody({ ...form, lag: -3650 }, "me", [])).toHaveProperty(
      "body",
    );
  });
  it("refuses itself and a link already there the same way, not the reverse", () => {
    expect(crossLinkBody({ ...form, otherPid: "me" }, "me", [])).toEqual({
      problems: ["p.crossplan.problem_self"],
    });
    expect(crossLinkBody(form, "me", [link("me", "x")])).toEqual({
      problems: ["p.crossplan.problem_exists"],
    });
    expect(crossLinkBody(form, "me", [link("x", "me")])).toHaveProperty("body");
  });
});

describe("the tasks of linked plans on the critical path", () => {
  const n = (pid: string, over: Record<string, unknown>) =>
    ({ pid, title: pid, ...over }) as unknown as Cpm["nodes"][number];
  const cpm = {
    nodes: [
      n("a", {}),
      n("x1", { external: true, plan_pid: "p2", plan_name: "Data" }),
      n("y1", { external: true, plan_pid: "p3", plan_name: "Infra" }),
      n("x2", { external: true, plan_pid: "p2", plan_name: "Data" }),
    ],
    edges: [
      {
        pid: "e1",
        predecessor_pid: "x1",
        successor_pid: "a",
        kind: "fs",
        lag_days: 0,
        cross_plan: true,
      },
      {
        pid: "e2",
        predecessor_pid: "x1",
        successor_pid: "x2",
        kind: "fs",
        lag_days: 0,
        cross_plan: false,
      },
    ],
  } as Pick<Cpm, "nodes" | "edges">;
  it("groups them by plan in the order met", () => {
    expect(
      externalPlans(cpm).map((g) => [g.planName, g.nodes.map((x) => x.pid)]),
    ).toEqual([
      ["Data", ["x1", "x2"]],
      ["Infra", ["y1"]],
    ]);
  });
  it("lists only the links that cross, with both ends", () => {
    const e = crossPlanEdges(cpm);
    expect(e.map((x) => [x.from?.pid, x.to?.pid])).toEqual([["x1", "a"]]);
  });
});

describe("a refused link", () => {
  const e = (s: number) =>
    new ApiError(s, "service text", { code: "unprocessable" });
  it("is the catalogue's sentence by action and status", () => {
    expect(crossLinkRefusalKey("add", e(404))).toBe(
      "p.crossplan.refused_unreadable",
    );
    expect(crossLinkRefusalKey("add", e(409))).toBe(
      "p.crossplan.refused_exists",
    );
    expect(crossLinkRefusalKey("add", e(422))).toBe("p.crossplan.refused_add");
    expect(crossLinkRefusalKey("remove", e(404))).toBe(
      "p.crossplan.refused_gone",
    );
    expect(crossLinkRefusalKey("remove", e(422))).toBeNull();
    expect(crossLinkRefusalKey("list", e(404))).toBeNull();
  });
});
