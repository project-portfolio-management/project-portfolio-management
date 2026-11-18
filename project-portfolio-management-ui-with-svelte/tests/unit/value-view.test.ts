// NC-1 slice 12 part (f): a plan's realized value. The service's rules said first: a value point's value (money in
// minor units with a currency, a whole number without), its method (the service's three), what is not sent (a blank
// currency or evidence, the first-measurable flag unless set); a business-case target's metric, whole numbers and
// source; each absent figure's reason in words (an unknown one a general sentence); and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import {
  ABSENT_KEY,
  METHODS,
  METHOD_KEY,
  SOURCES,
  SOURCE_KEY,
  absentKey,
  businessCaseBody,
  businessCaseRefusalKey,
  sourceKey,
  valuePointBody,
  valuePointRefusalKey,
} from "../../src/lib/value-view";

const point = (over = {}) => ({
  value: "",
  currency: "",
  method: "measured" as const,
  first: false,
  evidence: "",
  ...over,
});

describe("the words", () => {
  it("names the service's three methods, strongest first", () => {
    expect(METHODS).toEqual(["measured", "estimated", "asserted"]);
    for (const m of METHODS) expect(PAGES_EN[METHOD_KEY[m]]).toBeTruthy();
  });
  it("says every reason a figure is absent, and a general one for a reason it does not know", () => {
    expect(Object.keys(ABSENT_KEY)).toEqual([
      "unrealized",
      "no_denominator",
      "mixed_currency",
      "no_baseline",
      "no_responses",
      "overflow",
    ]);
    expect(absentKey("mixed_currency")).toBe("p.value.absent_mixed_currency");
    expect(absentKey("something_new")).toBe("p.value.absent_other");
    expect(absentKey(null)).toBe("p.value.absent_other");
  });
  it("names where a target was promised", () => {
    expect(SOURCES).toEqual(["charter", "gate_review"]);
    for (const s of SOURCES) expect(PAGES_EN[SOURCE_KEY[s]]).toBeTruthy();
    expect(sourceKey("gate_review")).toBe("p.value.source_gate_review");
    expect(sourceKey("other")).toBe("p.value.source_other");
  });
});

describe("recording an observed value", () => {
  it("sends money in minor units of its currency, in the reader's format", () => {
    expect(
      valuePointBody(point({ value: "1.234,50", currency: "eur" }), "de-DE"),
    ).toEqual({
      body: { value: 123450, method: "measured", currency: "EUR" },
    });
  });
  it("sends a whole number without a currency, and no blank currency or evidence", () => {
    expect(
      valuePointBody(point({ value: "42", method: "estimated" }), "en-GB"),
    ).toEqual({
      body: { value: 42, method: "estimated" },
    });
  });
  it("sends the first-measurable flag only when set, and the evidence trimmed", () => {
    expect(
      valuePointBody(
        point({ value: "3", first: true, evidence: "  doc:12 " }),
        "en-GB",
      ),
    ).toEqual({
      body: {
        value: 3,
        method: "measured",
        is_first_measurable: true,
        evidence_ref: "doc:12",
      },
    });
  });
  it("says every problem at once", () => {
    expect(valuePointBody(point({ value: "2.5" }), "en-GB")).toEqual({
      problems: ["p.value.problem_whole"],
    });
    expect(
      valuePointBody(point({ value: "lots", currency: "GBP" }), "en-GB"),
    ).toEqual({
      problems: ["p.value.problem_amount"],
    });
    expect(
      valuePointBody(point({ value: "1", currency: "pounds" }), "en-GB"),
    ).toEqual({
      problems: ["p.value.problem_currency"],
    });
    expect(
      valuePointBody(point({ value: "", method: "guessed" as never }), "en-GB"),
    ).toEqual({
      problems: ["p.value.problem_whole", "p.value.problem_method"],
    });
  });
  it("reads a 422 on a first-measurable point as the one already taken", () => {
    const e = new ApiError(422, "service text");
    expect(valuePointRefusalKey(e, true)).toBe("p.value.refused_first_taken");
    expect(valuePointRefusalKey(e, false)).toBe("p.value.refused_value");
    expect(valuePointRefusalKey(new ApiError(404, "service text"), false)).toBe(
      "p.value.refused_gone",
    );
    expect(
      valuePointRefusalKey(new ApiError(403, "service text"), false),
    ).toBeNull();
  });
});

describe("recording a business-case target", () => {
  const target = (over = {}) => ({
    metric: "  Beds freed ",
    baseline: 10,
    target: 40,
    unit: "",
    source: "charter" as const,
    ...over,
  });
  it("sends the metric trimmed, two whole numbers and the source, a blank unit left out", () => {
    expect(businessCaseBody(target())).toEqual({
      body: {
        metric: "Beds freed",
        baseline_value: 10,
        target_value: 40,
        source: "charter",
      },
    });
    expect(
      businessCaseBody(target({ unit: " beds ", source: "gate_review" })),
    ).toEqual({
      body: {
        metric: "Beds freed",
        baseline_value: 10,
        target_value: 40,
        unit: "beds",
        source: "gate_review",
      },
    });
  });
  it("says every problem at once", () => {
    expect(
      businessCaseBody(
        target({ metric: " ", baseline: null, source: "board" as never }),
      ),
    ).toEqual({
      problems: [
        "p.value.problem_metric",
        "p.value.problem_target_values",
        "p.value.problem_source",
      ],
    });
    expect(businessCaseBody(target({ target: 1.5 }))).toEqual({
      problems: ["p.value.problem_target_values"],
    });
    expect(
      businessCaseBody(target({ baseline: 0, target: -3 })),
    ).toHaveProperty("body");
  });
  it("says a 404 is the plan gone and a 422 a refused target", () => {
    expect(businessCaseRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.value.refused_gone",
    );
    expect(businessCaseRefusalKey(new ApiError(422, "service text"))).toBe(
      "p.value.refused_target",
    );
    expect(businessCaseRefusalKey(new Error("x"))).toBeNull();
  });
});
