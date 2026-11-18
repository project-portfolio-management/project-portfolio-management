// NC-1 slice 12 part (e) (NC-8, BR-4): a plan's lessons learned, what refers to a record, and a proposal's lessons
// from similar plans. The service's rules said first: what recording sends (a blank detail not sent, a record only when
// chosen), every problem in the service's order, the detail counted in bytes as the service counts, each refusal's
// sentence, and a match's score and confidence as the page says them.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  CATEGORY_MARK,
  DETAIL_MAX_BYTES,
  OCCASIONS,
  SUMMARY_MAX,
  isGate,
  lessonBody,
  lessonRefusalKey,
  nameScore,
  scorePercent,
} from "../../src/lib/lesson-view";

const RISK = "22222222-2222-4222-8222-222222222222";
const form = {
  occasion: "g2_definition",
  category: "went_badly" as const,
  summary: "  Suppliers need a second source. ",
  detail: "",
  link: "",
};

describe("recording a lesson", () => {
  it("is at a gate or at closure", () => {
    expect(OCCASIONS).toEqual([
      "g0_concept",
      "g1_feasibility",
      "g2_definition",
      "g3_delivery",
      "g4_launch",
      "g5_benefits",
      "closure",
    ]);
    expect(isGate("g5_benefits")).toBe(true);
    expect(isGate("closure")).toBe(false);
  });
  it("sends the summary trimmed, no detail when blank, and the record only when chosen", () => {
    expect(lessonBody(form)).toEqual({
      body: {
        occasion: "g2_definition",
        category: "went_badly",
        summary: "Suppliers need a second source.",
      },
    });
    expect(
      lessonBody({ ...form, detail: " More. ", link: `risk:${RISK}` }),
    ).toEqual({
      body: {
        occasion: "g2_definition",
        category: "went_badly",
        summary: "Suppliers need a second source.",
        detail: "More.",
        link: { kind: "risk", pid: RISK },
      },
    });
  });
  it("names every problem in the service's order", () => {
    expect(
      lessonBody({
        occasion: "",
        category: "",
        summary: " ",
        detail: "",
        link: "",
      }),
    ).toEqual({
      problems: [
        "p.lessons.problem_occasion",
        "p.lessons.problem_category",
        "p.lessons.problem_summary",
      ],
    });
    expect(
      lessonBody({ ...form, summary: "x".repeat(SUMMARY_MAX + 1) }),
    ).toEqual({
      problems: ["p.lessons.problem_summary"],
    });
    expect(
      lessonBody({ ...form, summary: "é".repeat(SUMMARY_MAX) }),
    ).toHaveProperty("body");
  });
  it("counts the detail in bytes, as the service does", () => {
    expect(
      lessonBody({ ...form, detail: "x".repeat(DETAIL_MAX_BYTES) }),
    ).toHaveProperty("body");
    expect(
      lessonBody({ ...form, detail: "é".repeat(DETAIL_MAX_BYTES / 2 + 1) }),
    ).toEqual({
      problems: ["p.lessons.problem_detail"],
    });
  });
  it("says each category with a word and a shape", () => {
    expect(Object.values(CATEGORY_MARK).map((m) => m.symbol)).toEqual([
      "✓",
      "✗",
      "→",
    ]);
  });
});

describe("a refusal", () => {
  const e = (s: number) => new ApiError(s, "service text");
  it("has a sentence per action and status", () => {
    expect(lessonRefusalKey("withdraw", e(404))).toBe(
      "p.lessons.refused_withdrawn",
    );
    expect(lessonRefusalKey("references", e(404))).toBe(
      "p.lessons.refused_no_record",
    );
    expect(lessonRefusalKey("proposal", e(404))).toBe(
      "p.lessons.refused_proposal_gone",
    );
    expect(lessonRefusalKey("record", e(404))).toBe("p.lessons.refused_gone");
    expect(lessonRefusalKey("record", e(422))).toBe("p.lessons.refused_link");
  });
  it("leaves anything else to the general rule", () => {
    expect(lessonRefusalKey("withdraw", e(422))).toBeNull();
    expect(lessonRefusalKey("record", e(500))).toBeNull();
    expect(lessonRefusalKey("record", new Error("x"))).toBeNull();
  });
});

describe("a match with a similar plan", () => {
  it("is said as a whole percentage, clamped", () => {
    expect(scorePercent(0.874)).toBe(87);
    expect(scorePercent(1.2)).toBe(100);
    expect(scorePercent(Number.NaN)).toBe(0);
  });
  it("says the titles' likeness when the matcher compared them, and nothing when it did not", () => {
    expect(
      nameScore({
        score: 0.9,
        confidence: "Medium",
        breakdown: { name_score: 0.912 },
      }),
    ).toBe(91);
    expect(
      nameScore({
        score: 0.9,
        confidence: "Medium",
        breakdown: { name_score: null },
      }),
    ).toBeNull();
    expect(
      nameScore({ score: 0.9, confidence: "Medium", breakdown: {} }),
    ).toBeNull();
  });
});
