// NC-1 slice 11 part (m) (FR-30, EX-8): a plan's phase and a programme's start, the service's rules said first.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  demandGrid,
  isBack,
  isStartRefused,
  meetsStartCheck,
  phaseChoices,
  phaseMoveBody,
  phaseRefusalKey,
  startRefusalText,
  wholeDays,
} from "../../src/lib/phase-view";

const form = (
  over: Partial<{ to: string; reason: string; override: string }>,
) =>
  ({ to: "", reason: "", override: "", ...over }) as Parameters<
    typeof phaseMoveBody
  >[1];

describe("the phase choices", () => {
  it("offer one step forward and every earlier phase, the nearest first", () => {
    expect(phaseChoices(null)).toEqual({ forward: "initiating", back: [] });
    expect(phaseChoices("initiating")).toEqual({
      forward: "planning",
      back: [],
    });
    expect(phaseChoices("executing")).toEqual({
      forward: "controlling",
      back: ["planning", "initiating"],
    });
    expect(phaseChoices("closing")).toEqual({
      forward: null,
      back: ["controlling", "executing", "planning", "initiating"],
    });
  });
  it("know a move back", () => {
    expect(isBack("executing", "planning")).toBe(true);
    expect(isBack("planning", "executing")).toBe(false);
    expect(isBack(null, "initiating")).toBe(false);
  });
  it("send a programme's move into executing through the start check, and nothing else", () => {
    expect(meetsStartCheck(true, "executing")).toBe(true);
    expect(meetsStartCheck(false, "executing")).toBe(false);
    expect(meetsStartCheck(true, "controlling")).toBe(false);
    expect(meetsStartCheck(true, "")).toBe(false);
  });
});

describe("a phase move", () => {
  it("needs a phase chosen, and is nothing for the phase the plan is in", () => {
    expect(phaseMoveBody("planning", form({}), false)).toEqual({
      problem: "p.phase.problem_choose",
    });
    expect(phaseMoveBody("planning", form({ to: "planning" }), false)).toEqual({
      problem: "p.phase.problem_same",
    });
  });
  it("goes forward one step at a time, counted from before the first phase", () => {
    expect(phaseMoveBody(null, form({ to: "planning" }), false)).toEqual({
      problem: "p.phase.problem_skip",
    });
    expect(
      phaseMoveBody("initiating", form({ to: "executing" }), false),
    ).toEqual({ problem: "p.phase.problem_skip" });
    expect(phaseMoveBody(null, form({ to: "initiating" }), false)).toEqual({
      body: { phase: "initiating" },
    });
  });
  it("goes back only with a reason, which is trimmed; a forward reason is sent when given", () => {
    expect(
      phaseMoveBody("executing", form({ to: "planning", reason: "  " }), false),
    ).toEqual({ problem: "p.phase.problem_reason" });
    expect(
      phaseMoveBody(
        "executing",
        form({ to: "planning", reason: " Scope changed " }),
        false,
      ),
    ).toEqual({ body: { phase: "planning", reason: "Scope changed" } });
    expect(
      phaseMoveBody(
        "planning",
        form({ to: "executing", reason: "Ready" }),
        false,
      ),
    ).toEqual({ body: { phase: "executing", reason: "Ready" } });
  });
  it("sends an override only on a programme's start, of 10 or more characters after trimming", () => {
    expect(
      phaseMoveBody(
        "planning",
        form({ to: "executing", override: "  too short " }),
        true,
      ),
    ).toEqual({ problem: "p.phase.problem_override" });
    expect(
      phaseMoveBody(
        "planning",
        form({ to: "executing", override: " Board said " }),
        true,
      ),
    ).toEqual({
      body: { phase: "executing", start_override_reason: "Board said" },
    });
    expect(
      phaseMoveBody(
        "planning",
        form({ to: "executing", override: "Too short" }),
        true,
      ),
    ).toEqual({ problem: "p.phase.problem_override" });
    expect(
      phaseMoveBody(
        "planning",
        form({ to: "executing", override: "Board said yes" }),
        false,
      ),
    ).toEqual({ body: { phase: "executing" } });
    expect(
      phaseMoveBody(
        "executing",
        form({ to: "controlling", override: "Board said yes" }),
        true,
      ),
    ).toEqual({ body: { phase: "controlling" } });
  });
});

describe("a refusal", () => {
  const refused = new ApiError(422, "x", {
    code: "start_refused",
    detail: "service text",
  });
  it("of a start is known by its code", () => {
    expect(isStartRefused(refused)).toBe(true);
    expect(
      isStartRefused(new ApiError(422, "x", { code: "unprocessable" })),
    ).toBe(false);
    expect(
      isStartRefused(new ApiError(409, "x", { code: "start_refused" })),
    ).toBe(false);
    expect(isStartRefused(new Error("x"))).toBe(false);
  });
  it("of any other move has the catalogue's sentence", () => {
    expect(phaseRefusalKey(refused)).toBeNull();
    expect(
      phaseRefusalKey(new ApiError(422, "x", { code: "unprocessable" })),
    ).toBe("p.phase.refused_move");
    expect(phaseRefusalKey(new ApiError(404, "x"))).toBe(
      "p.phase.refused_gone",
    );
    expect(phaseRefusalKey(new ApiError(403, "x"))).toBeNull();
    expect(phaseRefusalKey(new Error("x"))).toBeNull();
  });
  it("is said from its structured fields", () => {
    const tr = (k: string, v: Record<string, string | number>) =>
      `${k} ${JSON.stringify(v)}`;
    expect(
      startRefusalText(
        { reason: "limit_reached", in_delivery: 3, limit: 3 },
        tr,
        (m) => m,
      ),
    ).toBe('p.phase.refusal_limit {"in_delivery":3,"limit":3}');
    expect(
      startRefusalText(
        {
          reason: "constraint_pool_over",
          pool: "Engineering",
          months: ["2026-11", "2026-12"],
        },
        tr,
        (m) => `<${m}>`,
      ),
    ).toBe(
      'p.phase.refusal_pool {"pool":"Engineering","months":"<2026-11>, <2026-12>"}',
    );
  });
});

describe("the demand", () => {
  it("is a grid of every month any pool has, in order, with a gap where a pool has none", () => {
    expect(
      demandGrid([
        {
          pool: "Engineering",
          months: [
            { month: "2026-12", demand_fte: 1.5 },
            { month: "2026-11", demand_fte: 2 },
          ],
        },
        { pool: "Design", months: [{ month: "2027-01", demand_fte: 0.25 }] },
      ]),
    ).toEqual({
      months: ["2026-11", "2026-12", "2027-01"],
      rows: [
        { pool: "Engineering", cells: [2, 1.5, null] },
        { pool: "Design", cells: [null, null, 0.25] },
      ],
    });
    expect(demandGrid([])).toEqual({ months: [], rows: [] });
  });
  it("and a duration is whole days, rounded down", () => {
    expect(wholeDays(0)).toBe(0);
    expect(wholeDays(86_400_000 * 2 - 1)).toBe(1);
    expect(wholeDays(86_400_000 * 3)).toBe(3);
  });
});
