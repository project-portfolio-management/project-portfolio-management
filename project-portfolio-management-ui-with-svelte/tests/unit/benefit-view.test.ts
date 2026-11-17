// NC-1 slice 11 (DP-9): a benefit's classification and validation: the whole classification sent (a field left out is
// cleared), a yearly amount only for a recurring, financial benefit, a validation in minor units, and the refusal.
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ApiError } from "../../src/lib/api/client";
import type { BenefitBoard } from "../../src/lib/api/ppm-types";
import {
  STANDING_LABELS,
  classifyBody,
  formOf,
  validationBody,
  validationRefusalKey,
} from "../../src/lib/benefit-view";

type Benefit = BenefitBoard["benefits"][number];
const ben = (over: Partial<Benefit> = {}): Benefit => ({
  pid: "b1",
  title: "Fewer readmissions",
  category: "financial",
  currency: "GBP",
  target_minor: 1000000,
  realized_minor: 250000,
  target_note: null,
  status: "planned",
  expected_on: null,
  ...over,
});

describe("classifying a benefit", () => {
  it("fills the form from what it has, the yearly amount in the reader's format", () => {
    expect(
      formOf(
        ben({
          recurrence: "recurring",
          annualized_minor: 123450,
          classification: "societal",
        }),
        "en-GB",
      ),
    ).toEqual({
      recurrence: "recurring",
      annual: "1234.50",
      classification: "societal",
      kpi: "",
    });
  });
  it("sends the whole classification, leaving out what is not stated so it is cleared", () => {
    expect(
      classifyBody(
        { recurrence: "", annual: "", classification: "", kpi: "" },
        ben(),
      ),
    ).toEqual({ body: {} });
    expect(
      classifyBody(
        {
          recurrence: "recurring",
          annual: "1,234.50",
          classification: "cash_releasing",
          kpi: "",
        },
        ben(),
        "en-GB",
      ),
    ).toEqual({
      body: {
        recurrence: "recurring",
        annualized_minor: 123450,
        classification: "cash_releasing",
      },
    });
  });
  it("takes a yearly amount only for a recurring, financial benefit, not below 0", () => {
    expect(
      classifyBody(
        { recurrence: "one_off", annual: "5", classification: "", kpi: "" },
        ben(),
      ),
    ).toEqual({
      problems: ["p.benefits.problem_annual_recurring"],
    });
    expect(
      classifyBody(
        { recurrence: "recurring", annual: "5", classification: "", kpi: "" },
        ben({ currency: null }),
      ),
    ).toEqual({
      problems: ["p.benefits.problem_annual_financial"],
    });
    expect(
      classifyBody(
        { recurrence: "recurring", annual: "-1", classification: "", kpi: "" },
        ben(),
        "en-GB",
      ),
    ).toEqual({
      problems: ["p.benefits.problem_annual_amount"],
    });
  });
  it("names the service's recurrences and classifications", () => {
    const rules = readFileSync(
      join(
        process.cwd(),
        "../project-portfolio-management-service-with-rust/domain/src/benefit_validation.rs",
      ),
      "utf8",
    );
    expect(rules).toContain(
      'pub const RECURRENCES: &[&str] = &["recurring", "one_off"];',
    );
    expect(rules).toContain(
      'pub const CLASSIFICATIONS: &[&str] = &["cash_releasing", "non_cash_releasing", "societal"];',
    );
    expect(Object.keys(STANDING_LABELS)).toEqual([
      "not_claimed",
      "claimed",
      "partly_validated",
      "validated",
    ]);
  });
});

describe("validating a benefit", () => {
  it("sends the amount validated to date in minor units, and a note only when written", () => {
    expect(validationBody("2,500", "GBP", " ", "en-GB")).toEqual({
      body: { amount_minor: 250000 },
    });
    expect(validationBody("0", "GBP", " Audit Q3 ", "en-GB")).toEqual({
      body: { amount_minor: 0, note: "Audit Q3" },
    });
    expect(validationBody("-5", "GBP", "", "en-GB")).toEqual({
      problem: "p.benefits.problem_validated_amount",
    });
  });
  it("says a member of the programme may not validate", () => {
    expect(validationRefusalKey(new ApiError(403, "x"))).toBe(
      "p.benefits.refused_member",
    );
    expect(validationRefusalKey(new ApiError(422, "x"))).toBeNull();
  });
});
