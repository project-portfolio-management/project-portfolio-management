// NC-1 slice 12 part (g), NC-6: ranking by WSJF and RICE. The service's rules said first: the three models, the Smart
// Score first; each input's bounds (WSJF 1 to 20 whole, reach a whole number not below zero, impact one of five values,
// confidence 0 to 100 whole, effort above zero), every problem at once; all eight inputs sent, an empty box as null;
// nothing sent for what is stored; each component and missing input named and explained in the catalogue's words (an
// unknown one a general word); and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import {
  FORMULA_KEY,
  IMPACTS,
  IMPACT_KEY,
  INPUTS,
  INPUT_KEY,
  MODELS,
  MODEL_KEY,
  componentKeys,
  componentShape,
  inputKey,
  missingKey,
  scoringBody,
  scoringFormOf,
  scoringRefusalKey,
} from "../../src/lib/scoring-view";
import type { ScoringInputs } from "../../src/lib/api/ppm-types";

const stored: ScoringInputs = {
  business_value: 8,
  time_criticality: 5,
  risk_reduction: 3,
  job_size: 4,
  reach: 2000,
  impact: 2,
  confidence_percent: 80,
  effort: 4,
};

describe("the words", () => {
  it("offers the Smart Score first, then WSJF and RICE, each named and the two formulas said", () => {
    expect(MODELS).toEqual(["smart", "wsjf", "rice"]);
    for (const m of MODELS) expect(PAGES_EN[MODEL_KEY[m]]).toBeTruthy();
    expect(PAGES_EN[FORMULA_KEY.wsjf]).toContain("÷ job size");
    expect(PAGES_EN[FORMULA_KEY.rice]).toContain("reach × impact");
  });
  it("names every input and the five impact values", () => {
    expect(INPUTS).toHaveLength(8);
    for (const i of INPUTS) expect(PAGES_EN[INPUT_KEY[i]]).toBeTruthy();
    expect(IMPACTS).toEqual([0.25, 0.5, 1, 2, 3]);
    for (const v of IMPACTS) expect(PAGES_EN[IMPACT_KEY[v]]).toBeTruthy();
  });
  it("names and explains each component the service returns, and an unknown one in general words", () => {
    for (const name of [
      "business_value",
      "time_criticality",
      "risk_reduction",
      "cost_of_delay",
      "job_size",
      "reach",
      "impact",
      "confidence",
      "effort",
    ]) {
      const k = componentKeys(name);
      expect(k.label).not.toBe("p.scoring.input_other");
      expect(PAGES_EN[k.means]).toBeTruthy();
    }
    expect(componentKeys("weight")).toEqual({
      label: "p.scoring.input_other",
      means: "p.scoring.means_other",
    });
  });
  it("says why a score waits for each input, never the service's reason", () => {
    expect(missingKey("job_size")).toBe("p.scoring.needs_divisor");
    expect(missingKey("effort")).toBe("p.scoring.needs_divisor");
    expect(missingKey("business_value")).toBe("p.scoring.needs_delay_part");
    expect(missingKey("confidence_percent")).toBe("p.scoring.needs_confidence");
    expect(missingKey("weather")).toBe("p.scoring.needs_other");
    expect(inputKey("reach")).toBe("p.scoring.input_reach");
    expect(inputKey("weather")).toBe("p.scoring.input_other");
  });
  it("shows confidence as a percentage, impact with its word, and an absent value as absent", () => {
    expect(componentShape("confidence", 0.8)).toEqual({
      kind: "percent",
      fraction: 0.8,
    });
    expect(componentShape("impact", 2)).toEqual({
      kind: "impact",
      value: 2,
      word: "p.scoring.impact_high",
    });
    expect(componentShape("reach", null)).toEqual({ kind: "absent" });
    expect(componentShape("job_size", 0)).toEqual({ kind: "number", value: 0 });
  });
});

describe("the body", () => {
  it("fills the form from what is stored, and every input empty when none is", () => {
    expect(scoringFormOf(stored)).toEqual(stored);
    expect(Object.values(scoringFormOf(null)).every((v) => v === null)).toBe(
      true,
    );
  });
  it("sends nothing for what is stored, or for an empty form when nothing is", () => {
    expect(scoringBody({ ...stored }, stored)).toBeNull();
    expect(scoringBody(scoringFormOf(null), null)).toBeNull();
  });
  it("sends all eight inputs, an emptied box as null to clear it", () => {
    const form = { ...scoringFormOf(stored), reach: undefined, job_size: 6 };
    expect(scoringBody(form, stored)).toEqual({
      body: { ...stored, reach: null, job_size: 6 },
    });
  });
  it("keeps a stated zero, which is not a missing input", () => {
    const r = scoringBody(
      { ...scoringFormOf(null), reach: 0, confidence_percent: 0 },
      null,
    );
    expect(r).toEqual({
      body: { ...scoringFormOf(null), reach: 0, confidence_percent: 0 },
    });
  });
  it("names every problem at once, by input", () => {
    const r = scoringBody(
      {
        business_value: 0,
        time_criticality: 21,
        risk_reduction: 2.5,
        job_size: 20,
        reach: -1,
        impact: 1.5,
        confidence_percent: 101,
        effort: 0,
      },
      null,
    );
    expect(r).toEqual({
      problems: [
        { input: "business_value", key: "p.scoring.problem_wsjf" },
        { input: "time_criticality", key: "p.scoring.problem_wsjf" },
        { input: "risk_reduction", key: "p.scoring.problem_wsjf" },
        { input: "reach", key: "p.scoring.problem_reach" },
        { input: "impact", key: "p.scoring.problem_impact" },
        { input: "confidence_percent", key: "p.scoring.problem_confidence" },
        { input: "effort", key: "p.scoring.problem_effort" },
      ],
    });
  });
  it("accepts the edges of each range", () => {
    const r = scoringBody(
      {
        business_value: 1,
        time_criticality: 20,
        risk_reduction: 1,
        job_size: 20,
        reach: 2147483647,
        impact: 0.25,
        confidence_percent: 100,
        effort: 0.1,
      },
      null,
    );
    expect(r && "body" in r).toBe(true);
    expect(
      scoringBody({ ...scoringFormOf(null), reach: 2147483648 }, null),
    ).toEqual({
      problems: [{ input: "reach", key: "p.scoring.problem_reach" }],
    });
  });
});

describe("a refusal", () => {
  it("is the catalogue's sentence, never the service's text", () => {
    expect(scoringRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.scoring.refused_gone",
    );
    expect(scoringRefusalKey(new ApiError(422, "service text"))).toBe(
      "p.scoring.refused_inputs",
    );
    expect(scoringRefusalKey(new ApiError(500, "service text"))).toBeNull();
    expect(scoringRefusalKey(new Error("offline"))).toBeNull();
  });
});
