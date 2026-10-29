import { describe, it, expect } from "vitest";
import {
  formFrom,
  lineEstimateProblems,
  planRiskCalls,
  rangeMinor,
  rangeText,
  taskEstimateProblems,
  taskRangeText,
  toMinorStrict,
  type RiskInputsCurrent,
  type RiskInputsForm,
} from "../../src/lib/simulation-inputs";

const none: RiskInputsCurrent = {
  task_pid: null,
  schedule_impact_days: null,
  probability_percent: null,
  cost_impact_minor: null,
  cost_currency: null,
};
const empty: RiskInputsForm = {
  probability: "",
  taskPid: "",
  impactDays: "",
  cost: "",
  currency: "",
};
const form = (o: Partial<RiskInputsForm>): RiskInputsForm => ({
  ...empty,
  ...o,
});

describe("planRiskCalls", () => {
  it("does nothing for an empty form on an unconfigured risk", () => {
    expect(planRiskCalls(empty, none)).toEqual({ schedule: null, cost: null });
  });

  it("a schedule-only risk calls only the schedule endpoint, which sets the shared probability", () => {
    const r = planRiskCalls(
      form({ probability: "30", taskPid: "t1", impactDays: "5" }),
      none,
    );
    expect(r).toEqual({
      schedule: { task_pid: "t1", impact_days: 5, probability_percent: 30 },
      cost: null,
    });
  });

  it("a cost-only risk never touches the schedule endpoint (so it is not named as a half-set schedule risk)", () => {
    const r = planRiskCalls(
      form({ probability: "50", cost: "1500.50", currency: "gbp" }),
      none,
    );
    expect(r).toEqual({
      schedule: null,
      cost: {
        cost_impact_minor: 150_050,
        cost_currency: "GBP",
        probability_percent: 50,
      },
    });
  });

  it("a risk with both effects makes both calls with one probability", () => {
    const r = planRiskCalls(
      form({
        probability: "100",
        taskPid: "t1",
        impactDays: "4",
        cost: "40",
        currency: "GBP",
      }),
      none,
    );
    expect(r).toMatchObject({
      schedule: { probability_percent: 100, impact_days: 4 },
      cost: { cost_impact_minor: 4000, probability_percent: 100 },
    });
  });

  it("clearing the schedule part clears it, then the cost call restores the shared probability", () => {
    const current = {
      ...none,
      task_pid: "t1",
      schedule_impact_days: 5,
      probability_percent: 30,
    };
    const r = planRiskCalls(
      form({ probability: "30", cost: "10", currency: "GBP" }),
      current,
    );
    expect(r).toEqual({
      schedule: {
        task_pid: null,
        impact_days: null,
        probability_percent: null,
      },
      cost: {
        cost_impact_minor: 1000,
        cost_currency: "GBP",
        probability_percent: 30,
      },
    });
  });

  it("clearing the probability goes through the schedule endpoint, the only one that can", () => {
    const current = { ...none, probability_percent: 40 };
    const r = planRiskCalls(empty, current);
    expect(r).toEqual({
      schedule: {
        task_pid: null,
        impact_days: null,
        probability_percent: null,
      },
      cost: null,
    });
  });

  it("clearing a stored cost sends nulls", () => {
    const current = {
      ...none,
      cost_impact_minor: 5000,
      cost_currency: "GBP",
      probability_percent: 20,
    };
    const r = planRiskCalls(form({ probability: "20" }), current);
    expect(r).toMatchObject({
      cost: {
        cost_impact_minor: null,
        cost_currency: null,
        probability_percent: 20,
      },
    });
  });

  it("refuses what the service would refuse, in plain words", () => {
    const err = (f: Partial<RiskInputsForm>) =>
      (planRiskCalls(form(f), none) as { error: string }).error;
    expect(err({ probability: "101" })).toContain("0 to 100");
    expect(err({ probability: "2.5" })).toContain("whole number");
    expect(err({ taskPid: "t1", probability: "10" })).toContain("together");
    expect(err({ impactDays: "3", probability: "10" })).toContain("together");
    expect(
      err({ taskPid: "t1", impactDays: "0", probability: "10" }),
    ).toContain("1 to 365");
    expect(
      err({ taskPid: "t1", impactDays: "366", probability: "10" }),
    ).toContain("1 to 365");
    expect(err({ cost: "100", probability: "10" })).toContain("together");
    expect(err({ cost: "0", currency: "GBP", probability: "10" })).toContain(
      "above zero",
    );
    expect(err({ cost: "-5", currency: "GBP", probability: "10" })).toContain(
      "above zero",
    );
    expect(err({ cost: "5", currency: "POUNDS", probability: "10" })).toContain(
      "3-letter",
    );
    expect(err({ taskPid: "t1", impactDays: "3" })).toContain(
      "needs a probability",
    );
    expect(err({ cost: "5", currency: "GBP" })).toContain(
      "needs a probability",
    );
  });

  it("pre-fills the form from what is stored", () => {
    expect(
      formFrom({
        task_pid: "t1",
        schedule_impact_days: 5,
        probability_percent: 30,
        cost_impact_minor: 150_050,
        cost_currency: "GBP",
      }),
    ).toEqual({
      probability: "30",
      taskPid: "t1",
      impactDays: "5",
      cost: "1500.50",
      currency: "GBP",
    });
    expect(formFrom(none)).toEqual(empty);
  });
});

describe("values bound to number inputs", () => {
  it("accepts numbers, not only strings (a number input binds a number, or undefined when empty)", () => {
    const r = planRiskCalls(
      {
        probability: 40,
        taskPid: "t1",
        impactDays: 5,
        cost: 12.5,
        currency: "gbp",
      },
      none,
    );
    expect(r).toEqual({
      schedule: { task_pid: "t1", impact_days: 5, probability_percent: 40 },
      cost: {
        cost_impact_minor: 1250,
        cost_currency: "GBP",
        probability_percent: 40,
      },
    });
    expect(
      planRiskCalls(
        {
          probability: undefined,
          taskPid: "",
          impactDays: undefined,
          cost: undefined,
          currency: "",
        },
        none,
      ),
    ).toEqual({
      schedule: null,
      cost: null,
    });
    expect(lineEstimateProblems(80, 160, 10_000, "GBP", "en-GB")).toEqual([]);
    expect(lineEstimateProblems(undefined, 90, 10_000, "GBP", "en-GB")).toEqual(
      ["The high amount cannot be below the planned amount."],
    );
    expect(rangeMinor(80.5, "GBP", "en-GB")).toBe(8050);
    expect(rangeMinor(undefined, "GBP", "en-GB")).toBeNull();
    expect(taskEstimateProblems(3, 9, 5)).toEqual([]);
    expect(taskEstimateProblems(6, undefined, 5)[0]).toContain(
      "above the likely duration",
    );
  });
});

describe("estimate ranges", () => {
  it("keeps a line's range around its planned amount", () => {
    expect(lineEstimateProblems("80", "160", 10_000, "GBP", "en-GB")).toEqual(
      [],
    );
    expect(lineEstimateProblems("120", "", 10_000, "GBP", "en-GB")).toEqual([
      "The low amount cannot be above the planned amount.",
    ]);
    expect(lineEstimateProblems("", "90", 10_000, "GBP", "en-GB")).toEqual([
      "The high amount cannot be below the planned amount.",
    ]);
    expect(lineEstimateProblems("abc", "", 10_000, "GBP", "en-GB")).toEqual([
      "Amounts must be numbers, zero or more.",
    ]);
    expect(lineEstimateProblems("-1", "", 10_000, "GBP", "en-GB")[0]).toContain(
      "numbers",
    );
    expect(lineEstimateProblems("", "", 10_000, "GBP", "en-GB")).toEqual([]);
    expect(rangeMinor("", "GBP", "en-GB")).toBeNull();
    expect(rangeMinor("80.5", "GBP", "en-GB")).toBe(8050);
    expect(toMinorStrict("0", "GBP", "en-GB")).toBeNull();
    // JPY has no minor unit, KWD has three, and a decimal-comma locale reads "1,5"
    expect(rangeMinor("1500", "JPY", "en-GB")).toBe(1500);
    expect(rangeMinor("1.234", "KWD", "en-GB")).toBe(1234);
    expect(rangeMinor("1 234,5", "EUR", "fr-FR")).toBe(123_450);
    expect(rangeMinor("1,5", "EUR", "de-DE")).toBe(150);
    expect(toMinorStrict("1,5", "KWD", "de-DE")).toBe(1500);
    expect(lineEstimateProblems("0,5", "", 10_000, "EUR", "de-DE")).toEqual([]);
  });

  it("keeps a task's range around its likely duration", () => {
    expect(taskEstimateProblems("3", "9", 5)).toEqual([]);
    expect(taskEstimateProblems("6", "", 5)[0]).toContain(
      "above the likely duration (5)",
    );
    expect(taskEstimateProblems("", "4", 5)[0]).toContain(
      "below the likely duration (5)",
    );
    expect(taskEstimateProblems("1", "", null)[0]).toContain("duration first");
    expect(taskEstimateProblems("1.5", "", 5)[0]).toContain("whole numbers");
    expect(taskEstimateProblems("", "", null)).toEqual([]);
  });

  it("says plainly when there is no range", () => {
    const f = (m: number) => `${m / 100}`;
    expect(rangeText(null, null, f)).toContain("fallback band");
    expect(rangeText(8000, 16000, f)).toBe("80 to 160");
    expect(rangeText(null, 16000, f)).toBe("planned to 160");
    expect(taskRangeText(null, 5, null)).toContain("fallback band");
    expect(taskRangeText(3, 5, 9)).toBe(
      "3 / 5 / 9 business days (optimistic / likely / pessimistic)",
    );
    expect(taskRangeText(3, 5, null)).toBe(
      "3 / 5 / 5 business days (optimistic / likely / pessimistic)",
    );
  });
});
