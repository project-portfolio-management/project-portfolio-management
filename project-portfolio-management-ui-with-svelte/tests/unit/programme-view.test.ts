// NC-1 slice 11 (DP-5): planned gates and programme status: the changes a status allows (stopped is final), what
// saving gate dates sends, the checks before a decision, and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ApiError } from "../../src/lib/api/client";
import { PLAN_GATES } from "../../src/lib/api/ppm-types";
import {
  changedGates,
  decisionProblems,
  programmeRefusalKey,
  statusChoices,
} from "../../src/lib/programme-view";

describe("gates", () => {
  it("are the service's, in order", () => {
    const rules = readFileSync(
      join(
        process.cwd(),
        "../project-portfolio-management-service-with-rust/src/governance.rs",
      ),
      "utf8",
    );
    const listed = rules.match(
      /pub const GATES: &\[&str\] = &\[([^\]]+)\]/,
    )![1]!;
    expect([...PLAN_GATES]).toEqual(
      [...listed.matchAll(/"([a-z0-9_]+)"/g)].map((m) => m[1]),
    );
  });
  it("send only the dates that changed and are set; an emptied one cannot be cleared", () => {
    const stored = [{ gate: "g1_feasibility", planned_on: "2026-12-01" }];
    expect(
      changedGates(
        {
          g0_concept: "",
          g1_feasibility: "2026-12-01",
          g2_definition: "2027-02-01",
        },
        stored,
      ),
    ).toEqual([["g2_definition", "2027-02-01"]]);
    expect(changedGates({ g1_feasibility: "" }, stored)).toEqual([]);
    expect(changedGates({ g1_feasibility: "2026-12-15" }, stored)).toEqual([
      ["g1_feasibility", "2026-12-15"],
    ]);
  });
});

describe("programme status", () => {
  it("never offers the status it has, and nothing once stopped", () => {
    expect(statusChoices("active")).toEqual(["paused", "stopped"]);
    expect(statusChoices("paused")).toEqual(["active", "stopped"]);
    expect(statusChoices("stopped")).toEqual([]);
  });
  it("needs a change and a reason of ten characters", () => {
    expect(decisionProblems({ status: "", reason: " short " })).toEqual([
      "p.programme.problem_status",
      "p.programme.problem_reason",
    ]);
    expect(
      decisionProblems({ status: "paused", reason: "Waiting for funding." }),
    ).toEqual([]);
  });
  it("names the right a 403 needs and what a 409 and a 422 mean here", () => {
    expect(programmeRefusalKey(new ApiError(403, "x"))).toBe(
      "p.programme.refused_decider",
    );
    expect(programmeRefusalKey(new ApiError(409, "x"))).toBe(
      "p.programme.conflict_status",
    );
    expect(programmeRefusalKey(new ApiError(422, "x"))).toBe(
      "p.programme.refused_not_programme",
    );
    expect(programmeRefusalKey(new ApiError(500, "x"))).toBeNull();
  });
});
