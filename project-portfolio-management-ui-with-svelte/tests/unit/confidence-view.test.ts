// NC-1 slice 11 (DP-2): a plan's delivery confidence: the scale held to the service's, every value with a symbol, the
// checks before an assessment is sent, and the history newest first.
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ApiError } from "../../src/lib/api/client";
import {
  CONFIDENCE_SCALE,
  type ConfidenceAssessment,
} from "../../src/lib/api/ppm-types";
import {
  CONFIDENCE_MARK,
  assessRefusalKey,
  assessmentProblems,
  newestFirst,
  opensException,
} from "../../src/lib/confidence-view";

describe("the scale", () => {
  it("is the service's, best first, and every value has a symbol, not only a colour", () => {
    const rules = readFileSync(
      join(
        process.cwd(),
        "../project-portfolio-management-service-with-rust/src/delivery_confidence.rs",
      ),
      "utf8",
    );
    const listed = rules.match(
      /pub const SCALE: \[&str; \d+\] = \[([^\]]+)\]/,
    )![1]!;
    expect([...CONFIDENCE_SCALE]).toEqual(
      [...listed.matchAll(/"([a-z_]+)"/g)].map((m) => m[1]),
    );
    for (const v of [...CONFIDENCE_SCALE, "not_assessed" as const])
      expect(CONFIDENCE_MARK[v].symbol).not.toBe("");
  });
  it("opens an exception at amber/red and red only", () => {
    expect(CONFIDENCE_SCALE.filter(opensException)).toEqual([
      "amber_red",
      "red",
    ]);
  });
});

describe("checks before an assessment is sent", () => {
  const ok = {
    value: "amber",
    kind: "self",
    rationale: "Two suppliers late.",
    assessedOn: "",
  };
  it("says every problem at once", () => {
    expect(
      assessmentProblems(
        {
          value: "",
          kind: "x",
          rationale: " short ",
          assessedOn: "2026-11-18",
        },
        "2026-11-17",
      ),
    ).toEqual([
      "p.confidence.problem_value",
      "p.confidence.problem_kind",
      "p.confidence.problem_rationale",
      "p.confidence.problem_future",
    ]);
    expect(assessmentProblems(ok, "2026-11-17")).toEqual([]);
    expect(
      assessmentProblems({ ...ok, assessedOn: "2026-11-17" }, "2026-11-17"),
    ).toEqual([]);
  });
  it("names the right an independent assessment needs, and nothing else", () => {
    expect(assessRefusalKey(new ApiError(403, "x"))).toBe(
      "p.confidence.refused_independent",
    );
    expect(assessRefusalKey(new ApiError(422, "x"))).toBeNull();
  });
});

describe("history", () => {
  it("is shown newest first without changing what the service sent", () => {
    const a = (pid: string) => ({ pid }) as ConfidenceAssessment;
    const sent = [a("1"), a("2"), a("3")];
    expect(newestFirst(sent).map((x) => x.pid)).toEqual(["3", "2", "1"]);
    expect(sent.map((x) => x.pid)).toEqual(["1", "2", "3"]);
  });
});
