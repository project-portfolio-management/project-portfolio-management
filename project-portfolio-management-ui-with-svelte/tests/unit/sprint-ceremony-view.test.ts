// NC-1 slice 12 part (j), FR-29: a sprint's ceremonies and its commitment, checked first.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { SprintCommitment } from "../../src/lib/api/ppm-types";
import {
  ceremonyBody,
  ceremonyKey,
  commitProblem,
  emptyCeremonyForm,
  heldCounts,
  scopeChange,
  sprintRefusalKey,
} from "../../src/lib/sprint-ceremony-view";

const PERSON = "worker:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";
const held = {
  held: [
    { kind: "planning", count: 1 },
    { kind: "daily", count: 3 },
  ],
};

describe("ceremonies", () => {
  it("counts every kind, zero for one the service leaves out", () => {
    expect(heldCounts(held)).toEqual([
      { kind: "planning", count: 1 },
      { kind: "daily", count: 3 },
      { kind: "review", count: 0 },
      { kind: "retrospective", count: 0 },
    ]);
    expect(ceremonyKey("standup")).toBe("p.ceremony.kind_other");
  });
  it("sends a blank facilitator or summary not at all", () => {
    expect(
      ceremonyBody(
        { kind: "review", facilitator: ` ${PERSON} `, summary: " Shipped " },
        held,
      ),
    ).toEqual({
      body: { kind: "review", facilitator_ref: PERSON, summary: "Shipped" },
    });
    expect(
      ceremonyBody({ kind: "daily", facilitator: "", summary: " " }, held),
    ).toEqual({ body: { kind: "daily" } });
  });
  it("refuses a second planning or review and a facilitator that is not a reference, first", () => {
    expect(
      ceremonyBody({ kind: "planning", facilitator: "Ada", summary: "" }, held),
    ).toEqual({
      problems: ["p.ceremony.problem_once", "p.ceremony.problem_facilitator"],
    });
    expect(ceremonyBody(emptyCeremonyForm(), held)).toEqual({
      problems: ["p.ceremony.problem_kind"],
    });
    // A second daily is fine.
    expect(
      "body" in
        ceremonyBody({ kind: "daily", facilitator: "", summary: "" }, held),
    ).toBe(true);
  });
});

describe("the commitment", () => {
  const c = (over: Partial<SprintCommitment> = {}): SprintCommitment => ({
    sprint_pid: "s1",
    committed: 0,
    current: 2,
    added_after_commitment: [],
    removed_after_commitment: [],
    was_committed: false,
    note: "service text",
    ...over,
  });
  it("is taken once, never for an empty sprint, and only when read", () => {
    expect(commitProblem(c())).toBeNull();
    expect(commitProblem(c({ was_committed: true, committed: 2 }))).toBe(
      "p.commitment.problem_done",
    );
    expect(commitProblem(c({ current: 0 }))).toBe("p.commitment.problem_empty");
    expect(commitProblem(null)).toBe("p.commitment.problem_unread");
  });
  it("names the tasks added and removed, the unknown by the start of the id", () => {
    expect(
      scopeChange(
        c({
          added_after_commitment: ["t3"],
          removed_after_commitment: ["0123456789abcdef"],
        }),
        (p) => (p === "t3" ? "Spike" : undefined),
      ),
    ).toEqual({ added: ["Spike"], removed: ["01234567"] });
  });
});

describe("refusals", () => {
  const err = (status: number) => new ApiError(status, "service text", {});
  it("is a sentence by action and status", () => {
    expect(sprintRefusalKey("ceremony", err(422))).toBe(
      "p.ceremony.refused_ceremony",
    );
    expect(sprintRefusalKey("commit", err(422))).toBe(
      "p.commitment.refused_commit",
    );
    expect(sprintRefusalKey("commit", err(404))).toBe(
      "p.ceremony.refused_gone",
    );
    expect(sprintRefusalKey("ceremony", err(403))).toBe(
      "p.ceremony.refused_forbidden",
    );
    expect(sprintRefusalKey("ceremony", err(500))).toBeNull();
  });
});
