import { describe, expect, it } from "vitest";
import type { ToleranceDimension } from "../../src/lib/api/ppm";
import {
  DIMENSIONS,
  STATUS_ORDER,
  STATUS_SYMBOL,
  STATUS_TEXT,
  amountText,
  dimensionLabel,
  fromApiLimit,
  limitFieldLabel,
  sourceText,
  toApiLimit,
  usageText,
} from "../../src/lib/tolerance-view";

const dim = (over: Partial<ToleranceDimension>): ToleranceDimension => ({
  dimension: "time",
  unit: "calendar_days",
  basis: "b",
  status: "green",
  limit: 10,
  source: "own",
  ...over,
});

describe("tolerance words and shapes", () => {
  it("gives every status a word and its own shape, and ranks unknown above green", () => {
    for (const s of STATUS_ORDER) {
      expect(STATUS_TEXT[s]).toBeTruthy();
      expect(STATUS_SYMBOL[s]).toBeTruthy();
    }
    expect(new Set(STATUS_ORDER.map((s) => STATUS_SYMBOL[s])).size).toBe(
      STATUS_ORDER.length,
    );
    expect(STATUS_ORDER.indexOf("unknown")).toBeLessThan(
      STATUS_ORDER.indexOf("green"),
    );
    expect(STATUS_TEXT.unknown).not.toBe(STATUS_TEXT.green);
  });

  it("writes each figure in its own unit", () => {
    expect(amountText("time", 1)).toBe("1 day");
    expect(amountText("time", 5)).toBe("5 days");
    expect(amountText("schedule", 3)).toBe("3 days");
    expect(amountText("cost", 1250)).toBe("12.5%");
    expect(amountText("risk", 20)).toBe("score 20");
    expect(amountText("resources", 30)).toBe("30 points over 100%");
    expect(amountText("time", null)).toBe("");
    expect(dimensionLabel("cost")).toBe("Cost");
    expect(dimensionLabel("other")).toBe("other");
    expect(DIMENSIONS).toHaveLength(5);
  });

  it("converts cost between the percent a person types and the basis points the service stores", () => {
    expect(toApiLimit("cost", 12.5)).toBe(1250);
    expect(fromApiLimit("cost", 1250)).toBe(12.5);
    expect(toApiLimit("time", 7.4)).toBe(7);
    expect(fromApiLimit("risk", 15)).toBe(15);
    for (const d of DIMENSIONS) expect(limitFieldLabel(d)).toMatch(/^Limit/);
  });

  it("says why there is nothing to compare, rather than showing a number", () => {
    expect(usageText(dim({ status: "amber", usage_percent: 80 }))).toBe(
      "80% of the limit used",
    );
    expect(usageText(dim({ status: "unknown", reason: "no baseline" }))).toBe(
      "no baseline",
    );
    expect(usageText(dim({ status: "unknown", reason: null }))).toBe(
      "Could not be measured.",
    );
    expect(
      usageText(dim({ status: "not_set", limit: null, source: "none" })),
    ).toBe("No limit is set here or above.");
    expect(
      usageText(dim({ status: "red", limit: 0, usage_percent: null })),
    ).toMatch(/^Zero tolerance/);
    expect(sourceText(dim({ source: "own" }))).toBe("Set on this plan");
    expect(sourceText(dim({ source: "inherited" }))).toBe(
      "Inherited from a plan above",
    );
    expect(sourceText(dim({ source: "none" }))).toBe("");
  });
});
