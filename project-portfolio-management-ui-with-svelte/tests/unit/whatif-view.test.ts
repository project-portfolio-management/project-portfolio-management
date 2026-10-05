import { describe, expect, it } from "vitest";
import { buildEdit, costReasonText, deltaText, describeEdit, finishText } from "../../src/lib/whatif-view";

const form = (o: object) =>
  ({ kind: "move", task: "", start: "", days: "", constraint: "", constraintDate: "", reason: "", from: "", to: "", linkKind: "", lag: "", link: "", ...o }) as never;
const err = (x: unknown) => "error" in (x as object);

describe("buildEdit", () => {
  it("builds each kind", () => {
    expect(buildEdit(form({ kind: "move", task: "t", start: "2026-10-07" }))).toEqual({ op: "move", task_pid: "t", start_date: "2026-10-07" });
    expect(buildEdit(form({ kind: "duration", task: "t", days: "0" }))).toEqual({ op: "duration", task_pid: "t", duration_days: 0 });
    expect(buildEdit(form({ kind: "add_link", from: "a", to: "b" }))).toEqual({ op: "add_link", predecessor_pid: "a", successor_pid: "b", kind: "fs", lag_days: 0 });
    expect(buildEdit(form({ kind: "remove_link", link: "l" }))).toEqual({ op: "remove_link", link_pid: "l" });
    expect(buildEdit(form({ kind: "constraint", task: "t" }))).toEqual({ op: "constraint", task_pid: "t", constraint_kind: null, constraint_date: null });
  });
  it("sends a reason only with a hard constraint, and insists on it", () => {
    expect(buildEdit(form({ kind: "constraint", task: "t", constraint: "snet", constraintDate: "2026-10-09" }))).toEqual({ op: "constraint", task_pid: "t", constraint_kind: "snet", constraint_date: "2026-10-09" });
    expect(err(buildEdit(form({ kind: "constraint", task: "t", constraint: "mso", constraintDate: "2026-10-09" })))).toBe(true);
    expect(buildEdit(form({ kind: "constraint", task: "t", constraint: "mfo", constraintDate: "2026-10-09", reason: " contract " }))).toMatchObject({ reason: "contract" });
  });
  it("refuses what the server would", () => {
    for (const f of [
      form({ kind: "move", task: "", start: "2026-10-07" }),
      form({ kind: "move", task: "t", start: "tomorrow" }),
      form({ kind: "duration", task: "t", days: "-1" }),
      form({ kind: "duration", task: "t", days: "2.5" }),
      form({ kind: "add_link", from: "a", to: "a" }),
      form({ kind: "add_link", from: "a", to: "" }),
      form({ kind: "add_link", from: "a", to: "b", lag: "x" }),
      form({ kind: "constraint", task: "t", constraint: "snet", constraintDate: "" }),
      form({ kind: "remove_link", link: "" }),
    ])
      expect(err(buildEdit(f))).toBe(true);
  });
});

describe("words", () => {
  const name = (p: string) => ({ a: "Build", b: "Test" })[p] ?? p;
  it("describes edits", () => {
    expect(describeEdit({ op: "duration", task_pid: "a", duration_days: 1 }, name)).toBe("Set “Build” to 1 working day");
    expect(describeEdit({ op: "duration", task_pid: "a", duration_days: 0 }, name)).toBe("Make “Build” a milestone");
    expect(describeEdit({ op: "add_link", predecessor_pid: "a", successor_pid: "b", kind: "ss", lag_days: 2 }, name)).toBe("“Test” follows “Build” (SS, lag 2)");
    expect(describeEdit({ op: "constraint", task_pid: "a", constraint_kind: null, constraint_date: null }, name)).toBe("Clear the constraint on “Build”");
  });
  it("states the finish plainly", () => {
    expect(finishText({ finish: { before: "2026-10-09", after: "2026-10-12" } })).toBe("The finish moves from 2026-10-09 to 2026-10-12.");
    expect(finishText({ finish: { before: "2026-10-09", after: "2026-10-09" } })).toBe("The finish stays 2026-10-09.");
    expect(finishText({ finish: { before: null, after: null } })).toMatch(/no finish/);
  });
  it("signs money and explains absent cost", () => {
    expect(deltaText(120000, "GBP")).toBe("+1,200.00 GBP");
    expect(deltaText(-5, "GBP")).toBe("-0.05 GBP");
    expect(deltaText(0, "GBP")).toBe("0.00 GBP");
    expect(costReasonText({ available: false, reason: "no_finance_access" })).toMatch(/finance data/);
    expect(costReasonText({ available: true, labour: [], unrated_minutes_before: 0, unrated_minutes_after: 0 })).toBe("");
  });
});
