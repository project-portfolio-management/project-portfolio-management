// NC-1 slice 11 (DP-1, DP-12): drafting, reviewing, issuing and reissuing director packs: the audiences, the checks
// before sending, which actions a pack's state allows, and the sentence for each refusal.
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ApiError } from "../../src/lib/api/client";
import {
  CUT_OFF_MAX_DAYS,
  PACK_AUDIENCES,
  dayProblem,
  draftProblems,
  lastResult,
  noteProblem,
  packActions,
  refusalKey,
  rhythmRefusalKey,
} from "../../src/lib/pack-actions";

const err = (status: number) => new ApiError(status, "x", "service text");

describe("audiences", () => {
  it("are the service's four, in its order", () => {
    const rules = readFileSync(
      join(
        process.cwd(),
        "../project-portfolio-management-service-with-rust/domain/src/director_pack.rs",
      ),
      "utf8",
    );
    const listed = rules.match(
      /pub const AUDIENCES: \[&str; \d+\] = \[([^\]]+)\]/,
    )![1]!;
    expect(PACK_AUDIENCES.map((a) => a.key)).toEqual(
      [...listed.matchAll(/"([a-z_]+)"/g)].map((m) => m[1]),
    );
  });
});

describe("checks before drafting", () => {
  const now = new Date("2026-11-17T12:00:00Z");
  it("needs a month and an audience, and says every problem at once", () => {
    expect(
      draftProblems({ period: "2026-13", audience: "", cutOff: "" }, now),
    ).toEqual([
      "p.director_pack.problem_period",
      "p.director_pack.problem_audience",
    ]);
    expect(
      draftProblems({ period: "2026-11", audience: "board", cutOff: "" }, now),
    ).toEqual([]);
  });
  it("takes a cut-off that is not in the future nor more than 400 days old", () => {
    const ok = { period: "2026-11", audience: "board" };
    expect(draftProblems({ ...ok, cutOff: "2026-11-18T09:00" }, now)).toEqual([
      "p.director_pack.problem_cut_off_future",
    ]);
    const old = new Date(
      now.getTime() - (CUT_OFF_MAX_DAYS + 1) * 86_400_000,
    ).toISOString();
    expect(draftProblems({ ...ok, cutOff: old }, now)).toEqual([
      "p.director_pack.problem_cut_off_old",
    ]);
    expect(draftProblems({ ...ok, cutOff: "nonsense" }, now)).toEqual([
      "p.director_pack.problem_cut_off_bad",
    ]);
    expect(draftProblems({ ...ok, cutOff: "2026-10-31T18:00" }, now)).toEqual(
      [],
    );
  });
  it("needs a change note of ten characters once trimmed to reissue", () => {
    expect(noteProblem("  long enough ")).toBeNull();
    expect(noteProblem("  short   ")).toBe("p.director_pack.problem_note");
  });
});

describe("what a pack's state allows", () => {
  it("a manual draft is reviewed or issued; a scheduled one waits for its review; an issued one is reissued", () => {
    expect(
      packActions({ status: "draft", origin: "manual", reviewed: false }),
    ).toEqual({
      review: true,
      issue: true,
      issueWaitsForReview: false,
      reissue: false,
    });
    expect(
      packActions({ status: "draft", origin: "scheduled", reviewed: false }),
    ).toMatchObject({
      issue: false,
      issueWaitsForReview: true,
    });
    expect(
      packActions({ status: "draft", origin: "scheduled", reviewed: true }),
    ).toMatchObject({
      review: false,
      issue: true,
    });
    expect(
      packActions({ status: "issued", origin: "manual", reviewed: true }),
    ).toEqual({
      review: false,
      issue: false,
      issueWaitsForReview: false,
      reissue: true,
    });
  });
  it("names the right a 403 needs and what a 409 means for each action", () => {
    expect(refusalKey("issue", err(403))).toBe("p.director_pack.refused_issue");
    expect(refusalKey("draft", err(403))).toBe("p.director_pack.refused_edit");
    expect(refusalKey("draft", err(409))).toBe(
      "p.director_pack.conflict_draft",
    );
    expect(refusalKey("reissue", err(409))).toBe(
      "p.director_pack.conflict_reissue",
    );
    expect(refusalKey("issue", err(422))).toBeNull();
    expect(refusalKey("issue", new Error("offline"))).toBeNull();
  });
});

describe("schedules and subscriptions (DP-12)", () => {
  it("takes a day of the month from 1 to 28 only", () => {
    expect(dayProblem("1")).toBeNull();
    expect(dayProblem("28")).toBeNull();
    expect(dayProblem("29")).toBe("p.director_pack.problem_day");
    expect(dayProblem("0")).toBe("p.director_pack.problem_day");
    expect(dayProblem("2.5")).toBe("p.director_pack.problem_day");
    expect(dayProblem("")).toBe("p.director_pack.problem_day");
  });
  it("reads the service's two last-run results and shows nothing for any other text", () => {
    expect(lastResult("drafted 2026-10")).toEqual({
      key: "p.director_pack.last_drafted",
      period: "2026-10",
    });
    expect(lastResult("skipped 2026-10: a pack for it already exists")).toEqual(
      {
        key: "p.director_pack.last_skipped",
        period: "2026-10",
      },
    );
    expect(lastResult("failed: database")).toBeNull();
    expect(lastResult(null)).toBeNull();
  });
  it("names the refusals: editor rights for a schedule, a readable audience for a subscription", () => {
    expect(rhythmRefusalKey("schedule", err(403))).toBe(
      "p.director_pack.refused_edit",
    );
    expect(rhythmRefusalKey("schedule", err(409))).toBe(
      "p.director_pack.conflict_schedule",
    );
    expect(rhythmRefusalKey("subscription", err(403))).toBe(
      "p.director_pack.refused_subscribe",
    );
    expect(rhythmRefusalKey("subscription", err(409))).toBeNull();
  });
});
