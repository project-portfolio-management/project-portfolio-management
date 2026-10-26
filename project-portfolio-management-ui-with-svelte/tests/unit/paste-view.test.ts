import { describe, expect, it } from "vitest";
import { MAX_ROWS, counts, isIsoDate, parsePasted, parseTsv, rowId, toImportCsv } from "../../src/lib/paste-view";

describe("parseTsv", () => {
  it("splits on tabs and lines, either line ending, and ignores blank lines", () => {
    expect(parseTsv("a\tb\r\n\r\nc\td\n").map((r) => r.cells)).toEqual([["a", "b"], ["c", "d"]]);
  });
  it("reads a quoted cell with a tab, a quote and a line break in it, and counts lines", () => {
    const rows = parseTsv('"x\ty ""q""\nz"\t2026-10-05\nnext');
    expect(rows[0]!.cells).toEqual(['x\ty "q"\nz', "2026-10-05"]);
    expect(rows[1]!.line).toBe(3);
  });
});

describe("isIsoDate", () => {
  it("takes real days in one form only", () => {
    expect(isIsoDate("2026-10-05")).toBe(true);
    expect(isIsoDate("2026-02-30")).toBe(false);
    expect(isIsoDate("05/10/2026")).toBe(false);
    expect(isIsoDate("2026-1-5")).toBe(false);
  });
});

describe("parsePasted", () => {
  it("skips a header line and reads title, due date and notes", () => {
    const p = parsePasted("Title\tDue\tNotes\nWrite plan\t2026-10-05\tfirst draft\nBook room");
    expect(p.header).toBe(true);
    expect(p.rows.map((r) => [r.title, r.due, r.notes, r.problems.length])).toEqual([
      ["Write plan", "2026-10-05", "first draft", 0],
      ["Book room", null, "", 0],
    ]);
  });
  it("flags a missing title, a date it does not understand and a repeat, and says which line", () => {
    const p = parsePasted("Good\t2026-10-05\n\t2026-10-06\nBad date\t5 Oct\nGood\t2026-10-05");
    const keys = p.rows.map((r) => r.problems.map((x) => x.key));
    expect(keys).toEqual([[], ["no_title"], ["date_bad"], ["same_as"]]);
    expect(p.rows[3]!.problems[0]).toEqual({ key: "same_as", line: 1 });
    expect(p.rows[2]!.due).toBeNull();
    expect(counts(p.rows)).toEqual({ good: 1, left: 3 });
  });
  it("leaves out the rows past the limit", () => {
    const text = Array.from({ length: MAX_ROWS + 2 }, (_, i) => `Task ${i}`).join("\n");
    const p = parsePasted(text);
    expect(p.rows.filter((r) => r.problems.length > 0)).toHaveLength(2);
    expect(p.rows.at(-1)!.problems[0]!.key).toBe("beyond_limit");
  });
  it("flags a long title", () => {
    expect(parsePasted("x".repeat(301)).rows[0]!.problems[0]!.key).toBe("title_long");
  });
});

describe("toImportCsv", () => {
  it("sends only the good rows, quoted where it matters, with ids made from their content", () => {
    const p = parsePasted('Plan, then "go"\t2026-10-05\tnote\nBad\tnope\n');
    const csv = toImportCsv(p.rows);
    const lines = csv.trim().split("\n");
    expect(lines[0]).toBe("Task ID,Name,Due Date,Notes");
    expect(lines).toHaveLength(2);
    expect(lines[1]).toBe(`${rowId("Plan, then \"go\"", "2026-10-05")},"Plan, then ""go""",2026-10-05,note`);
  });
  it("gives the same id to the same row and a different one to another", () => {
    expect(rowId("Write plan", "2026-10-05")).toBe(rowId("  write PLAN ", "2026-10-05"));
    expect(rowId("Write plan", "2026-10-05")).not.toBe(rowId("Write plan", "2026-10-06"));
    expect(rowId("a", null)).toMatch(/^paste-[0-9a-f]{8}$/);
  });
});
