// NC-1 slice 11 (DP-6): funding sources: the name checks, a line's choices (active, plus its own retired one), the
// financial year, each refusal's sentence, and the money drawn by the director pack's renderer.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { FundingSource } from "../../src/lib/api/ppm-types";
import {
  fundingRefusalKey,
  fyProblem,
  lineChoices,
  sourceNameProblem,
} from "../../src/lib/funding-view";

const src = (pid: string, name: string, active = true): FundingSource => ({
  pid,
  name,
  active,
});
const SOURCES = [
  src("s1", "Capital grant"),
  src("s2", "Old fund", false),
  src("s3", "Trust income"),
];

describe("funding sources", () => {
  it("need a name of 2 to 120 characters, not already on the list in any case", () => {
    expect(sourceNameProblem(" a ", SOURCES)).toBe("p.funding.problem_name");
    expect(sourceNameProblem("x".repeat(121), SOURCES)).toBe(
      "p.funding.problem_name",
    );
    expect(sourceNameProblem("  capital GRANT ", SOURCES)).toBe(
      "p.funding.problem_name_taken",
    );
    expect(sourceNameProblem("Lottery bid", SOURCES)).toBeNull();
  });
  it("offer a line every active source, and its own retired one so it shows what it has", () => {
    expect(lineChoices(SOURCES, null).map((s) => s.pid)).toEqual(["s1", "s3"]);
    expect(lineChoices(SOURCES, "s2").map((s) => s.pid)).toEqual([
      "s1",
      "s2",
      "s3",
    ]);
  });
  it("take a financial year from 2000 to 2100, or empty for this one", () => {
    expect(fyProblem("")).toBeNull();
    expect(fyProblem("2026")).toBeNull();
    expect(fyProblem("1999")).toBe("p.funding.problem_fy");
    expect(fyProblem("26")).toBe("p.funding.problem_fy");
  });
  it("say a taken name and a retired source in the catalogue's words", () => {
    expect(fundingRefusalKey(new ApiError(409, "x"))).toBe(
      "p.funding.problem_name_taken",
    );
    expect(fundingRefusalKey(new ApiError(422, "x"))).toBe(
      "p.funding.refused_inactive",
    );
    expect(fundingRefusalKey(new ApiError(500, "x"))).toBeNull();
  });
});
