// NC-1 slice 11 (EX-7): triage, thresholds and threshold decisions, the service's rules said first.
import { describe, expect, it } from "vitest";
import {
  decisionProblems,
  exceptionThreshold,
  isThresholdException,
  sameThreshold,
  thresholdBody,
  triageProblem,
} from "../../src/lib/threshold-view";

const form = (
  over: Partial<{
    count: string | number;
    date: string;
    amount: string;
    currency: string;
  }>,
) => ({
  count: "",
  date: "",
  amount: "",
  currency: "",
  ...over,
});

describe("a triage", () => {
  it("is one of the four outcomes, with a reason of at least 10 characters after trimming", () => {
    expect(triageProblem("later", "A good long reason")).toBe(
      "p.thresholds.problem_outcome",
    );
    expect(triageProblem("fix", "  too short  ")).toBe(
      "p.thresholds.problem_reason",
    );
    expect(triageProblem("fix", "0123456789")).toBeNull();
  });
});

describe("a threshold", () => {
  it("of red packs is a whole number from 1 to 24, typed or bound as a number", () => {
    expect(thresholdBody("confidence_red_packs", form({ count: "0" }))).toEqual(
      { problem: "p.thresholds.problem_count" },
    );
    expect(
      thresholdBody("confidence_red_packs", form({ count: "25" })),
    ).toEqual({ problem: "p.thresholds.problem_count" });
    expect(
      thresholdBody("confidence_red_packs", form({ count: "2.5" })),
    ).toEqual({ problem: "p.thresholds.problem_count" });
    expect(thresholdBody("confidence_red_packs", form({ count: "" }))).toEqual({
      problem: "p.thresholds.problem_count",
    });
    expect(thresholdBody("confidence_red_packs", form({ count: 24 }))).toEqual({
      body: { kind: "confidence_red_packs", count: 24 },
    });
  });
  it("of a forecast date needs a date", () => {
    expect(thresholdBody("forecast_past", form({}))).toEqual({
      problem: "p.thresholds.problem_date",
    });
    expect(
      thresholdBody("forecast_past", form({ date: "2027-03-31" })),
    ).toEqual({ body: { kind: "forecast_past", date: "2027-03-31" } });
  });
  it("of a P80 cap needs an ISO 4217 currency and an amount above zero, in minor units", () => {
    expect(
      thresholdBody("p80_cap", form({ amount: "100", currency: "XXQ" })),
    ).toEqual({ problem: "p.thresholds.problem_currency" });
    expect(
      thresholdBody("p80_cap", form({ amount: "0", currency: "GBP" })),
    ).toEqual({ problem: "p.thresholds.problem_cap" });
    expect(
      thresholdBody("p80_cap", form({ amount: "1250.50", currency: "gbp" })),
    ).toEqual({
      body: { kind: "p80_cap", cap_minor: 125050, currency: "GBP" },
    });
    expect(
      thresholdBody("p80_cap", form({ amount: "1000", currency: "JPY" })),
    ).toEqual({
      body: { kind: "p80_cap", cap_minor: 1000, currency: "JPY" },
    });
  });
  it("is unchanged only when every parameter matches", () => {
    const cur = {
      kind: "p80_cap" as const,
      count: null,
      date: null,
      cap_minor: 100,
      currency: "GBP",
    };
    expect(sameThreshold(cur, { cap_minor: 100, currency: "GBP" })).toBe(true);
    expect(sameThreshold(cur, { cap_minor: 101, currency: "GBP" })).toBe(false);
    expect(sameThreshold(undefined, { count: 3 })).toBe(false);
  });
});

describe("a threshold decision", () => {
  const today = "2026-11-17";
  it("is pause, reset, stop or override, with a reason of at least 10 characters", () => {
    expect(decisionProblems("wait", "A good long reason", "", today)).toEqual([
      "p.thresholds.problem_action",
    ]);
    expect(decisionProblems("pause", "short", "", today)).toEqual([
      "p.thresholds.problem_reason",
    ]);
    expect(decisionProblems("stop", "A good long reason", "", today)).toEqual(
      [],
    );
  });
  it("needs an override's expiry in the future and at most 366 days out", () => {
    expect(
      decisionProblems("override", "A good long reason", "", today),
    ).toEqual(["p.thresholds.problem_expiry_needed"]);
    expect(
      decisionProblems("override", "A good long reason", today, today),
    ).toEqual(["p.thresholds.problem_expiry_past"]);
    expect(
      decisionProblems("override", "A good long reason", "2027-11-19", today),
    ).toEqual(["p.thresholds.problem_expiry_long"]);
    expect(
      decisionProblems("override", "A good long reason", "2027-11-18", today),
    ).toEqual([]);
  });
});

describe("an exception", () => {
  it("was opened by a threshold when its opening value is `threshold`", () => {
    expect(isThresholdException({ opened_value: "threshold" })).toBe(true);
    expect(isThresholdException({ opened_value: "red" })).toBe(false);
  });
});

describe("the threshold behind an exception (NC-1 slice 12 part a)", () => {
  it("names the kind that opened a threshold exception", () => {
    expect(
      exceptionThreshold({
        opened_value: "threshold",
        threshold_kind: "p80_cap",
      }),
    ).toEqual({ key: "p.thresholds.opened_by_kind", kind: "p80_cap" });
  });
  it("names the kind that joined a confidence exception while it was open", () => {
    expect(
      exceptionThreshold({
        opened_value: "red",
        threshold_kind: "forecast_past",
      }),
    ).toEqual({ key: "p.thresholds.joined_by_kind", kind: "forecast_past" });
  });
  it("names none for one confidence alone opened, or a kind with no words", () => {
    expect(
      exceptionThreshold({ opened_value: "red", threshold_kind: null }),
    ).toBeNull();
    expect(exceptionThreshold({ opened_value: "red" })).toBeNull();
    expect(
      exceptionThreshold({
        opened_value: "threshold",
        threshold_kind: "new_kind" as "p80_cap",
      }),
    ).toBeNull();
  });
});
