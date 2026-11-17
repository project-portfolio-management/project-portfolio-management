// NC-1 slice 11 (EX-6): a cutover's stop criteria as the service keeps them, and its rules said first.
import { describe, expect, it } from "vitest";
import {
  criteriaProblem,
  parseCriteria,
  sameCriteria,
} from "../../src/lib/cutover-view";

describe("stop criteria", () => {
  it("are one per line, trimmed, with blank lines dropped", () => {
    expect(
      parseCriteria("  Rollback rehearsed \n\n Fares checked\n  "),
    ).toEqual(["Rollback rehearsed", "Fares checked"]);
  });
  it("number 1 to 30", () => {
    expect(criteriaProblem([])).toEqual({ key: "p.cutover.problem_count" });
    expect(
      criteriaProblem(Array.from({ length: 31 }, (_, i) => `Criterion ${i}`)),
    ).toEqual({ key: "p.cutover.problem_count" });
    expect(
      criteriaProblem(Array.from({ length: 30 }, (_, i) => `Criterion ${i}`)),
    ).toBeNull();
  });
  it("are each 5 to 500 characters, counted as characters, naming the first line that is not", () => {
    expect(criteriaProblem(["Rollback rehearsed", "ok"])).toEqual({
      key: "p.cutover.problem_length",
      line: 2,
    });
    expect(criteriaProblem(["x".repeat(501)])).toEqual({
      key: "p.cutover.problem_length",
      line: 1,
    });
    expect(criteriaProblem(["😀😀😀😀😀"])).toBeNull();
    expect(criteriaProblem(["x".repeat(500)])).toBeNull();
  });
  it("compare in order", () => {
    expect(sameCriteria(["a", "b"], ["a", "b"])).toBe(true);
    expect(sameCriteria(["a", "b"], ["b", "a"])).toBe(false);
    expect(sameCriteria(["a"], ["a", "b"])).toBe(false);
  });
});
