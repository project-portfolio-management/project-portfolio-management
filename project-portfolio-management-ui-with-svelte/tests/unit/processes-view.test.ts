import { describe, expect, it } from "vitest";
import {
  MAX_STEPS,
  blankStep,
  definitionProblems,
  isPersonRef,
  isRunning,
  kindLabel,
  runStatusLabel,
  stepStateLabel,
} from "../../src/lib/processes-view";

const ok = {
  kind: "approve" as const,
  person_ref: "person:abc",
  title: "Sign off",
};

describe("approval process drafts", () => {
  it("accepts person: and worker: references and nothing else", () => {
    expect(isPersonRef("person:abc")).toBe(true);
    expect(isPersonRef("worker:1")).toBe(true);
    for (const bad of ["", "person:", "person: ", "abc", "team:abc", ":abc"])
      expect(isPersonRef(bad)).toBe(false);
  });

  it("passes a good draft and names every problem in a bad one", () => {
    expect(definitionProblems("Release", [ok])).toEqual([]);
    const problems = definitionProblems(" ", [
      ok,
      { ...ok, title: "" },
      { ...ok, person_ref: "nobody" },
    ]);
    expect(problems).toHaveLength(3);
    expect(definitionProblems("x", [])).toHaveLength(1);
    expect(
      definitionProblems(
        "x",
        Array.from({ length: MAX_STEPS + 1 }, () => ok),
      ),
    ).toHaveLength(1);
    expect(definitionProblems("n".repeat(121), [ok])).toHaveLength(1);
    expect(
      definitionProblems("x", [{ ...ok, title: "t".repeat(201) }]),
    ).toHaveLength(1);
  });

  it("starts a new step as an approval with nothing filled in", () => {
    expect(blankStep()).toEqual({ kind: "approve", person_ref: "", title: "" });
  });
});

describe("approval process words", () => {
  it("labels kinds, step states and run statuses, and passes an unknown value through", () => {
    expect(kindLabel("todo")).toBe("To do");
    expect(kindLabel("odd")).toBe("odd");
    expect(stepStateLabel("waiting")).toBe("waiting");
    expect(stepStateLabel("odd")).toBe("odd");
    expect(runStatusLabel("cancelled")).toBe("cancelled");
  });

  it("only a running run can be cancelled or checked again", () => {
    expect(isRunning("running")).toBe(true);
    for (const s of ["completed", "rejected", "cancelled", "failed"])
      expect(isRunning(s)).toBe(false);
  });
});
