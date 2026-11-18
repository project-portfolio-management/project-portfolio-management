// NC-1 slice 11 (EX-10): scorecards and the de-escalation tracker, the service's rules said first, what each action
// sends (nothing for no change), the reasons for a quarter built from the detail, and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  criterionProblem,
  deescalationRefusalKey,
  evidenceBody,
  kpiName,
  kpisBody,
  lastFinishedQuarter,
  levelBody,
  levelProblem,
  parseQuarter,
  quarterName,
  quarterReasons,
  redLineProblem,
  sameEvidence,
  scorecardProblem,
  stateOf,
  templateBody,
} from "../../src/lib/deescalation-view";
import type {
  DeescalationCriterion,
  ScorecardKpi,
  ScorecardRow,
  TemplateMeasure,
} from "../../src/lib/api/ppm-types";

const CARDS: ScorecardRow[] = [
  { pid: "s1", name: "Outcomes", for_deescalation: true, active: true },
  { pid: "s2", name: "Delivery", for_deescalation: false, active: true },
];

describe("a new scorecard", () => {
  it("has a name of 3 to 120 characters after trimming", () => {
    expect(scorecardProblem("  ab ", false, [])).toBe(
      "p.deescalation.problem_name",
    );
    expect(scorecardProblem("x".repeat(121), false, [])).toBe(
      "p.deescalation.problem_name",
    );
    expect(scorecardProblem("abc", false, [])).toBeNull();
  });
  it("is not named as another is, in any case", () => {
    expect(scorecardProblem(" outcomes ", false, CARDS)).toBe(
      "p.deescalation.problem_name_taken",
    );
  });
  it("is the one the tracker reads only when no other is", () => {
    expect(scorecardProblem("Benefits", true, CARDS)).toBe(
      "p.deescalation.problem_tracker_taken",
    );
    expect(scorecardProblem("Benefits", false, CARDS)).toBeNull();
    expect(scorecardProblem("Benefits", true, [CARDS[1]!])).toBeNull();
  });
});

describe("a scorecard's KPIs", () => {
  it("send nothing when the set is the same, in any order", () => {
    expect(kpisBody(["a", "b"], ["b", "a"])).toBeNull();
    expect(kpisBody(["a", "b"], ["b", "a", "a"])).toBeNull();
  });
  it("send the whole new set, once each", () => {
    expect(kpisBody(["a"], ["a", "c", "c"])).toEqual({
      body: { kpi_pids: ["a", "c"] },
    });
    expect(kpisBody(["a"], [])).toEqual({ body: { kpi_pids: [] } });
  });
  it("are at most 60", () => {
    const many = Array.from({ length: 61 }, (_, i) => `k${i}`);
    expect(kpisBody([], many)).toEqual({
      problem: "p.deescalation.problem_kpis_max",
    });
    expect(kpisBody([], many.slice(0, 60))).toHaveProperty("body");
  });
});

const TEMPLATE: TemplateMeasure[] = [
  {
    key: "milestones_met_on_time",
    name: "service text",
    direction: "higher_better",
    unit: "percent",
  },
  {
    key: "open_critical_issues",
    name: "service text",
    direction: "lower_better",
    unit: "count",
  },
];
const pick = (
  key: string,
  target: string | number,
  red: string | number,
  chosen = true,
) => ({ key, chosen, target, red });

describe("measures from the template", () => {
  it("keep the direction rule: red below the target when higher is better, above it when lower is better", () => {
    expect(redLineProblem("higher_better", 80, 80)).toBe(
      "p.deescalation.problem_red_higher",
    );
    expect(redLineProblem("higher_better", 80, 60)).toBeNull();
    expect(redLineProblem("lower_better", 2, 2)).toBe(
      "p.deescalation.problem_red_lower",
    );
    expect(redLineProblem("lower_better", 2, 5)).toBeNull();
  });
  it("send only the picked measures, with whole numbers typed or bound as numbers", () => {
    expect(
      templateBody(
        [
          pick("milestones_met_on_time", "90", 70),
          pick("open_critical_issues", 0, 3, false),
        ],
        TEMPLATE,
      ),
    ).toEqual({
      body: {
        measures: [
          { key: "milestones_met_on_time", target_value: 90, red_value: 70 },
        ],
      },
    });
  });
  it("need at least one pick, whole numbers, and the rule kept, naming the measure", () => {
    expect(
      templateBody([pick("milestones_met_on_time", 90, 70, false)], TEMPLATE),
    ).toEqual({ problem: "p.deescalation.problem_pick_one", key: "" });
    expect(
      templateBody([pick("open_critical_issues", "", 3)], TEMPLATE),
    ).toEqual({
      problem: "p.deescalation.problem_whole",
      key: "open_critical_issues",
    });
    expect(
      templateBody([pick("open_critical_issues", 1.5, 3)], TEMPLATE),
    ).toEqual({
      problem: "p.deescalation.problem_whole",
      key: "open_critical_issues",
    });
    expect(
      templateBody([pick("open_critical_issues", 3, 1)], TEMPLATE),
    ).toEqual({
      problem: "p.deescalation.problem_red_lower",
      key: "open_critical_issues",
    });
    expect(templateBody([pick("nope", 1, 0)], TEMPLATE)).toEqual({
      problem: "p.deescalation.problem_not_in_template",
      key: "nope",
    });
  });
});

describe("the escalation level", () => {
  it("is 1 to 60 characters after trimming", () => {
    expect(levelProblem("   ")).toBe("p.deescalation.problem_level");
    expect(levelProblem("x".repeat(61))).toBe("p.deescalation.problem_level");
    expect(levelProblem(" 3 ")).toBeNull();
  });
  it("sends nothing when the level and note are those recorded, and leaves out an empty note", () => {
    const now = { level: "Level 3", note: "Board asked" };
    expect(levelBody(now, " Level 3 ", "Board asked ")).toBeNull();
    expect(levelBody(now, "Level 2", "")).toEqual({ level: "Level 2" });
    expect(levelBody(now, "Level 3", "Again")).toEqual({
      level: "Level 3",
      note: "Again",
    });
    expect(levelBody(null, "Level 3", "")).toEqual({ level: "Level 3" });
  });
});

describe("a criterion", () => {
  it("is 5 to 500 characters after trimming", () => {
    expect(criterionProblem(" four ")).toBe("p.deescalation.problem_criterion");
    expect(criterionProblem("x".repeat(501))).toBe(
      "p.deescalation.problem_criterion",
    );
    expect(criterionProblem("Five!")).toBeNull();
  });
});

describe("quarters", () => {
  it("are YYYY-Qn, years 2000 to 2100, as the service reads them", () => {
    expect(parseQuarter("2026-Q3")).toEqual({ year: 2026, q: 3 });
    expect(parseQuarter("2026-Q5")).toBeNull();
    expect(parseQuarter("1999-Q1")).toBeNull();
    expect(parseQuarter("2026Q1")).toBeNull();
  });
  it("finish at the end of the calendar quarter", () => {
    expect(lastFinishedQuarter("2026-10-11")).toBe("2026-Q3");
    expect(lastFinishedQuarter("2026-01-01")).toBe("2025-Q4");
    expect(lastFinishedQuarter("2026-03-31")).toBe("2025-Q4");
  });
  it("are named through the catalogue", () => {
    expect(quarterName("2026-Q3", (_k, v) => `Q${v.q} of ${v.year}`)).toBe(
      "Q3 of 2026",
    );
  });
});

describe("evidence", () => {
  const today = "2026-11-17";
  const form = (over: Partial<Parameters<typeof evidenceBody>[0]>) => ({
    quarter: "2026-Q3",
    met: true as boolean | null,
    note: "Audit report signed",
    link: "",
    ...over,
  });
  it("is for a finished quarter only", () => {
    expect(evidenceBody(form({ quarter: "2026-Q4" }), today)).toEqual({
      problem: "p.deescalation.problem_quarter_unfinished",
    });
    expect(evidenceBody(form({ quarter: "" }), today)).toEqual({
      problem: "p.deescalation.problem_quarter",
    });
  });
  it("says whether it was met, with a note of at least 5 characters", () => {
    expect(evidenceBody(form({ met: null }), today)).toEqual({
      problem: "p.deescalation.problem_met",
    });
    expect(evidenceBody(form({ note: " abcd " }), today)).toEqual({
      problem: "p.deescalation.problem_note",
    });
  });
  it("takes a link only as an http or https address with no spaces, up to 500 bytes", () => {
    for (const link of [
      "ftp://x.example",
      "https://x.example/a b",
      `https://x.example/${"é".repeat(250)}`,
    ])
      expect(evidenceBody(form({ link }), today)).toEqual({
        problem: "p.deescalation.problem_link",
      });
    expect(
      evidenceBody(form({ link: " https://x.example/r ", met: false }), today),
    ).toEqual({
      body: {
        quarter: "2026-Q3",
        met: false,
        note: "Audit report signed",
        link: "https://x.example/r",
      },
    });
    expect(evidenceBody(form({}), today)).toEqual({
      body: { quarter: "2026-Q3", met: true, note: "Audit report signed" },
    });
  });
  it("sends nothing when the latest record already says the same", () => {
    const c: DeescalationCriterion = {
      pid: "c1",
      criterion: "Independent review passed",
      quarters: [
        {
          quarter: "2026-Q3",
          met: true,
          note: "Audit report signed",
          link: null,
        },
        { quarter: "2026-Q2", met: null, note: null, link: null },
      ],
    };
    expect(
      sameEvidence(c, {
        quarter: "2026-Q3",
        met: true,
        note: "Audit report signed",
      }),
    ).toBe(true);
    expect(
      sameEvidence(c, {
        quarter: "2026-Q3",
        met: false,
        note: "Audit report signed",
      }),
    ).toBe(false);
    expect(
      sameEvidence(c, {
        quarter: "2026-Q2",
        met: true,
        note: "Audit report signed",
      }),
    ).toBe(false);
  });
});

describe("the reasons for a quarter", () => {
  const kpi = (name: string, met: boolean | null): ScorecardKpi => ({
    kpi_pid: name,
    name,
    unit: "percent",
    direction: "higher_better",
    target_value: 80,
    quarters: [{ quarter: "2026-Q3", value: null, observed_on: null, met }],
  });
  it("name the KPIs and criteria not met and those with nothing recorded, from the detail", () => {
    const r = quarterReasons(
      "2026-Q3",
      [kpi("On time", false), kpi("Assured", null), kpi("Validated", true)],
      [
        {
          criterion: "Review passed",
          quarters: [{ quarter: "2026-Q3", met: null, note: null, link: null }],
        },
        { criterion: "No red", quarters: [] },
      ],
    );
    expect(r).toEqual({
      notMet: ["On time"],
      missing: ["Assured", "Review passed", "No red"],
      nothing: false,
    });
  });
  it("name a KPI the way the page is told to (a template KPI by its key)", () => {
    const r = quarterReasons(
      "2026-Q3",
      [
        {
          ...kpi("Milestones met", false),
          template_key: "milestones_met_on_time",
        },
      ],
      [],
      (k) => kpiName(k, (key) => `catalogue:${key}`),
    );
    expect(r.notMet).toEqual(["catalogue:milestones_met_on_time"]);
  });
  it("say there is nothing to measure when there is no KPI and no criterion", () => {
    expect(quarterReasons("2026-Q3", [], [])).toEqual({
      notMet: [],
      missing: [],
      nothing: true,
    });
  });
  it("read a reading's met as a state: null is nothing recorded", () => {
    expect([stateOf(true), stateOf(false), stateOf(null)]).toEqual([
      "met",
      "not_met",
      "missing",
    ]);
  });
});

describe("a refusal", () => {
  const e = (status: number) => new ApiError(status, "service text");
  it("is the catalogue's sentence for each action", () => {
    expect(deescalationRefusalKey("level", e(403))).toBe(
      "p.deescalation.refused_level_forbidden",
    );
    expect(deescalationRefusalKey("level", e(422))).toBe(
      "p.deescalation.problem_level",
    );
    // The two 409s on a new scorecard have a code each (NC-1 slice 12 part a); one with no code is the generic one.
    const coded = (code: string) =>
      new ApiError(409, "service text", { code, detail: "service text" });
    expect(
      deescalationRefusalKey("create", coded("scorecard_name_taken")),
    ).toBe("p.deescalation.problem_name_taken");
    expect(
      deescalationRefusalKey("create", coded("tracker_scorecard_taken")),
    ).toBe("p.deescalation.refused_tracker_taken");
    expect(deescalationRefusalKey("create", e(409))).toBeNull();
    expect(deescalationRefusalKey("create", coded("conflict"))).toBeNull();
    expect(deescalationRefusalKey("kpis", e(422))).toBe(
      "p.deescalation.refused_kpis",
    );
    expect(deescalationRefusalKey("template", e(422))).toBe(
      "p.deescalation.refused_template",
    );
    expect(deescalationRefusalKey("criterion", e(422))).toBe(
      "p.deescalation.problem_criterion",
    );
    expect(deescalationRefusalKey("evidence", e(422))).toBe(
      "p.deescalation.refused_evidence",
    );
    expect(deescalationRefusalKey("evidence", e(404))).toBe(
      "p.deescalation.refused_gone",
    );
  });
  it("falls back to the generic sentence otherwise", () => {
    expect(deescalationRefusalKey("criterion", e(500))).toBeNull();
    expect(deescalationRefusalKey("kpis", new Error("x"))).toBeNull();
  });
});

describe("a KPI's name (NC-1 slice 12 part a)", () => {
  const measure = (key: string) => `catalogue:${key}`;
  it("is the catalogue's words for a KPI made from the template", () => {
    expect(
      kpiName(
        {
          name: "Open issues at critical severity",
          template_key: "open_critical_issues",
        },
        measure,
      ),
    ).toBe("catalogue:open_critical_issues");
  });
  it("is the stored name for any other KPI, or a key the catalogue does not know", () => {
    expect(kpiName({ name: "Cycle time" }, measure)).toBe("Cycle time");
    expect(kpiName({ name: "Cycle time", template_key: null }, measure)).toBe(
      "Cycle time",
    );
    expect(
      kpiName(
        { name: "Something new", template_key: "a_new_measure" },
        measure,
      ),
    ).toBe("Something new");
  });
});
