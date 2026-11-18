// NC-1 slice 12 part (f): a plan's money read live. The service's rules said first: the roll-up's depth (1 to 32); a
// baseline's currency, periods (1 to 256, an end not before its start, a planned amount not below zero) and reason
// (needed after the first, at most 1024 bytes counted as bytes); one the same as the latest not sent; a gated line's
// release only once its gate has passed (the service's gate order); and each refusal's sentence, the incomplete
// register read from its machine code.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { BudgetBaseline } from "../../src/lib/api/ppm-types";
import {
  DEPTH_MAX,
  ETC_SOURCE_KEY,
  GATES,
  PERIODS_MAX,
  REASON_MAX_BYTES,
  baselineBody,
  baselineFormOf,
  baselineRefusalKey,
  depthProblem,
  gateReached,
  releaseRefusalKey,
  releaseState,
  rollupRefusalKey,
} from "../../src/lib/forecast-view";
import { PAGES_EN } from "../../src/lib/i18n-pages";

const V1: BudgetBaseline = {
  pid: "b1",
  version: 1,
  currency: "GBP",
  approved_at: "2026-10-01T09:00:00Z",
  reason: null,
  periods: [
    {
      period_start: "2026-11-01",
      period_end: "2026-11-30",
      planned_minor: 150000,
    },
  ],
};
const form = (over: Partial<Parameters<typeof baselineBody>[0]> = {}) => ({
  currency: "gbp",
  periods: [{ start: "2026-11-01", end: "2026-11-30", amount: "1,500.00" }],
  reason: "",
  ...over,
});

describe("the forecast's words", () => {
  it("names both sources of an estimate to complete", () => {
    expect(Object.keys(ETC_SOURCE_KEY)).toEqual(["tpc", "baseline_remaining"]);
    for (const k of Object.values(ETC_SOURCE_KEY))
      expect(PAGES_EN[k]).toBeTruthy();
  });
  it("walks 1 to 32 levels, empty for all", () => {
    expect(DEPTH_MAX).toBe(32);
    expect(depthProblem(null)).toBeNull();
    expect(depthProblem(1)).toBeNull();
    expect(depthProblem(32)).toBeNull();
    for (const bad of [0, 33, 2.5])
      expect(depthProblem(bad)).toBe("p.forecast.problem_depth");
  });
  it("says a 404 on the roll-up is a plan gone or hidden", () => {
    expect(rollupRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.forecast.refused_plan_gone",
    );
    expect(rollupRefusalKey(new ApiError(500, "service text"))).toBeNull();
    expect(rollupRefusalKey(new Error("x"))).toBeNull();
  });
});

describe("approving a baseline", () => {
  it("sends the first with no reason, in minor units, the currency upper-cased", () => {
    expect(baselineBody(form(), null, "en-GB")).toEqual({
      body: {
        currency: "GBP",
        periods: [
          {
            period_start: "2026-11-01",
            period_end: "2026-11-30",
            planned_minor: 150000,
          },
        ],
      },
    });
  });
  it("reads the amount in the reader's own number format and the currency's exponent", () => {
    const r = baselineBody(
      form({
        currency: "JPY",
        periods: [{ start: "2026-11-01", end: "2026-11-30", amount: "1.500" }],
      }),
      null,
      "de-DE",
    );
    expect(r).toEqual({
      body: {
        currency: "JPY",
        periods: [
          {
            period_start: "2026-11-01",
            period_end: "2026-11-30",
            planned_minor: 1500,
          },
        ],
      },
    });
  });
  it("leaves out a period with nothing typed", () => {
    const r = baselineBody(
      form({
        periods: [...form().periods, { start: "", end: " ", amount: "" }],
      }),
      null,
      "en-GB",
    );
    expect("body" in r && r.body.periods).toHaveLength(1);
  });
  it("says every problem at once", () => {
    expect(
      baselineBody(
        {
          currency: "pounds",
          periods: [
            { start: "2026-12-01", end: "2026-11-01", amount: "1" },
            { start: "", end: "2026-11-01", amount: "1" },
          ],
          reason: "",
        },
        V1,
        "en-GB",
      ),
    ).toEqual({
      problems: [
        "p.forecast.problem_currency",
        "p.forecast.problem_order",
        "p.forecast.problem_dates",
        "p.forecast.problem_reason",
      ],
    });
    expect(baselineBody(form({ periods: [] }), null)).toEqual({
      problems: ["p.forecast.problem_no_periods"],
    });
    expect(
      baselineBody(
        form({
          periods: [{ start: "2026-11-01", end: "2026-11-30", amount: "-5" }],
        }),
        null,
        "en-GB",
      ),
    ).toEqual({ problems: ["p.forecast.problem_amount"] });
  });
  it("holds at most 256 periods", () => {
    expect(PERIODS_MAX).toBe(256);
    const many = Array.from({ length: 257 }, () => ({
      start: "2026-11-01",
      end: "2026-11-30",
      amount: "1",
    }));
    expect(baselineBody(form({ periods: many }), null, "en-GB")).toEqual({
      problems: ["p.forecast.problem_too_many"],
    });
  });
  it("needs a reason after the first, trimmed, of at most 1024 bytes counted as bytes", () => {
    const changed = form({
      periods: [{ start: "2026-11-01", end: "2026-12-31", amount: "1500" }],
    });
    expect(baselineBody(changed, V1, "en-GB")).toEqual({
      problems: ["p.forecast.problem_reason"],
    });
    expect(
      baselineBody({ ...changed, reason: "  Scope added.  " }, V1, "en-GB"),
    ).toMatchObject({
      body: { reason: "Scope added." },
    });
    expect(REASON_MAX_BYTES).toBe(1024);
    expect(
      baselineBody({ ...changed, reason: "é".repeat(513) }, V1, "en-GB"),
    ).toEqual({
      problems: ["p.forecast.problem_reason_long"],
    });
    expect(
      baselineBody({ ...changed, reason: "é".repeat(512) }, V1, "en-GB"),
    ).toHaveProperty("body");
  });
  it("sends nothing for a version the same as the latest", () => {
    expect(baselineBody(form({ reason: "Again." }), V1, "en-GB")).toEqual({
      problems: ["p.forecast.problem_same"],
    });
  });
  it("starts a re-baseline from the latest version, in the reader's format", () => {
    expect(baselineFormOf(V1, (m) => String(m / 100))).toEqual({
      currency: "GBP",
      periods: [{ start: "2026-11-01", end: "2026-11-30", amount: "1500" }],
      reason: "",
    });
    expect(baselineFormOf(null, () => "")).toEqual({
      currency: "",
      periods: [{ start: "", end: "", amount: "" }],
      reason: "",
    });
  });
  it("says a 404 is the plan gone and a 422 after the checks is a version approved meanwhile", () => {
    expect(baselineRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.forecast.refused_gone",
    );
    expect(baselineRefusalKey(new ApiError(422, "service text"))).toBe(
      "p.forecast.refused_baseline",
    );
    expect(baselineRefusalKey(new ApiError(500, "service text"))).toBeNull();
  });
});

describe("releasing a tranche", () => {
  it("follows the service's gate order: a stage at or past the gate has reached it", () => {
    expect(GATES).toEqual([
      "g0_concept",
      "g1_feasibility",
      "g2_definition",
      "g3_delivery",
      "g4_launch",
      "g5_benefits",
    ]);
    expect(gateReached(null, "g0_concept")).toBe(false);
    expect(gateReached("g0_concept", "g0_concept")).toBe(true);
    expect(gateReached("g2_definition", "g1_feasibility")).toBe(true);
    expect(gateReached("g0_concept", "g1_feasibility")).toBe(false);
    expect(gateReached("nonsense", "g0_concept")).toBe(false);
    expect(gateReached("g5_benefits", "nonsense")).toBe(false);
  });
  it("is released, held or ready; a line with no gate is no tranche", () => {
    const line = {
      pid: "l1",
      description: "Build",
      currency: "GBP",
      planned_minor: 1,
    };
    expect(releaseState(line, "g3_delivery")).toBeNull();
    expect(
      releaseState(
        { ...line, gate: "g2_definition", released_at: "2026-11-01T00:00:00Z" },
        "g3_delivery",
      ),
    ).toBe("released");
    expect(
      releaseState(
        { ...line, gate: "g2_definition", released_at: null },
        "g1_feasibility",
      ),
    ).toBe("held");
    expect(
      releaseState(
        { ...line, gate: "g2_definition", released_at: null },
        "g2_definition",
      ),
    ).toBe("ready");
  });
  it("reads the incomplete register from its code; another 422 is a line no longer held", () => {
    const register = new ApiError(422, "service text", {
      code: "register_incomplete",
      detail: "service text",
    });
    expect(releaseRefusalKey(register)).toBe("p.forecast.refused_register");
    expect(
      releaseRefusalKey(
        new ApiError(422, "service text", { code: "unprocessable" }),
      ),
    ).toBe("p.forecast.refused_release");
    expect(releaseRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.forecast.refused_line_gone",
    );
    expect(releaseRefusalKey(new ApiError(403, "service text"))).toBeNull();
  });
});
