import { describe, expect, it } from "vitest";
import { cellSentence, findingText, normalise, shortRef, toAssignments, toggleLetter, validPersonRef } from "../../src/lib/raci-view";

describe("normalise", () => {
  it("orders letters R-A-C-I, upper-cases and deduplicates", () => {
    expect(normalise("ar")).toBe("RA");
    expect(normalise("iica")).toBe("ACI");
    expect(normalise("xyz")).toBe("");
  });
});

describe("toggleLetter", () => {
  it("adds and removes a letter, keeping the order", () => {
    let t = toggleLetter({}, "p:a", "A");
    expect(t.cells).toEqual({ "p:a": "A" });
    t = toggleLetter(t.cells, "p:a", "R");
    expect(t.cells).toEqual({ "p:a": "RA" });
    t = toggleLetter(t.cells, "p:a", "A");
    expect(t.cells).toEqual({ "p:a": "R" });
  });
  it("drops a person who is left with no letter", () => {
    expect(toggleLetter({ "p:a": "C" }, "p:a", "C").cells).toEqual({});
  });
  it("gives Accountable to one person only, and says whose it was", () => {
    const t = toggleLetter({ "p:a": "RA", "p:b": "C" }, "p:b", "A");
    expect(t.cells).toEqual({ "p:a": "R", "p:b": "AC" });
    expect(t.accountableMovedFrom).toBe("p:a");
  });
  it("does not report a move when nobody had Accountable, or when it is re-toggled off", () => {
    expect(toggleLetter({ "p:a": "R" }, "p:b", "A").accountableMovedFrom).toBeNull();
    expect(toggleLetter({ "p:a": "A" }, "p:a", "A").accountableMovedFrom).toBeNull();
  });
  it("a person may hold R, A, C and I together; other people are untouched", () => {
    let c: Record<string, string> = { "p:z": "I" };
    for (const l of ["R", "A", "C", "I"] as const) c = toggleLetter(c, "p:a", l).cells;
    expect(c).toEqual({ "p:z": "I", "p:a": "RACI" });
  });
  it("never mutates its input", () => {
    const before = { "p:a": "RA" };
    toggleLetter(before, "p:a", "C");
    expect(before).toEqual({ "p:a": "RA" });
  });
});

describe("payload and wording", () => {
  it("sends only people who hold a letter, normalised", () => {
    expect(toAssignments({ "p:a": "ar", "p:b": "" })).toEqual([{ person_ref: "p:a", roles: "RA" }]);
    expect(toAssignments({})).toEqual([]);
  });
  it("names what a row is missing in words", () => {
    expect(findingText("no_accountable")).toBe("Nobody is accountable");
    expect(findingText("no_responsible")).toBe("Nobody is responsible");
    expect(findingText("many_accountable")).toMatch(/More than one/);
  });
  it("writes a sentence for each cell", () => {
    expect(cellSentence("Design", "person:3f2c6b7e-0000-4000-8000-0000000000a1", "RA")).toBe("Design: 3f2c6b7e… is Responsible and Accountable");
    expect(cellSentence("Design", "person:abc", "")).toBe("Design: abc has no role");
  });
  it("shortens a reference and validates a new column", () => {
    expect(shortRef("person:3f2c6b7e-0000-4000-8000-0000000000a1")).toBe("3f2c6b7e…");
    expect(shortRef("plain")).toBe("plain");
    expect(validPersonRef("person:3f2c6b7e-0000-4000-8000-0000000000a1")).toBe(true);
    expect(validPersonRef("worker:3f2c6b7e-0000-4000-8000-0000000000a1")).toBe(true);
    expect(validPersonRef("alex")).toBe(false);
    expect(validPersonRef("person:not-a-uuid")).toBe(false);
  });
});
