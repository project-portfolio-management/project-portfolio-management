// WW-1.3: one money formatter and parser. No currency is built in, the minor unit is the currency's own
// (JPY 0, EUR 2, KWD 3, CLF 4), and an amount is read in the reader's own number format.

import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import {
  currencyCode,
  exponentOf,
  minorToInput,
  money,
  parseAmount,
  signedMoney,
  toMinor,
} from "../../src/lib/money";

describe("exponentOf", () => {
  it("is the currency's own minor unit", () => {
    expect(exponentOf("JPY", "en-GB")).toBe(0);
    expect(exponentOf("EUR", "en-GB")).toBe(2);
    expect(exponentOf("KWD", "en-GB")).toBe(3);
    expect(exponentOf("CLF", "en-GB")).toBe(4);
  });
});

describe("money", () => {
  it("scales by the currency's exponent", () => {
    expect(money(123_456, "JPY", "en-GB")).toBe("JP¥123,456");
    expect(money(1_234_567, "KWD", "en-GB")).toContain("1,234.567");
    expect(money(123_450, "EUR", "en-GB")).toBe("€1,234.50");
    expect(money(null, "EUR", "en-GB")).toBe("–");
  });
  it("uses the reader's own separators", () => {
    expect(money(123_450, "EUR", "de-DE")).toMatch(/^1\.234,50\s€$/u);
    expect(money(123_450, "EUR", "fr-FR")).toMatch(/^1\s234,50\s€$/u);
  });
  it("shows the sign of a change", () => {
    expect(signedMoney(1000, "JPY", "en-GB")).toBe("+JP¥1,000");
    expect(signedMoney(-5, "EUR", "en-GB")).toBe("-€0.05");
    expect(signedMoney(0, "EUR", "en-GB")).toBe("€0.00");
  });
});

describe("parseAmount and toMinor", () => {
  it("reads a decimal comma where the locale uses one", () => {
    expect(parseAmount("1,5", "de-DE")).toBe(1.5);
    expect(parseAmount("1 234,50", "fr-FR")).toBe(1234.5);
    expect(parseAmount("1.234,50", "de-DE")).toBe(1234.5);
    expect(toMinor("1 234,50", "EUR", "fr-FR")).toBe(123_450);
  });
  it("reads a decimal point where the locale uses one", () => {
    expect(parseAmount("1,234.50", "en-GB")).toBe(1234.5);
    expect(parseAmount("1234.5", "en-GB")).toBe(1234.5);
  });
  it("never turns text into a number", () => {
    expect(parseAmount("", "en-GB")).toBeNull();
    expect(parseAmount("abc", "en-GB")).toBeNull();
    expect(parseAmount("1.2.3", "en-GB")).toBeNull();
    expect(parseAmount("12 EUR", "en-GB")).toBeNull();
  });
  it("scales by the exponent without float error", () => {
    expect(toMinor("1500", "JPY", "en-GB")).toBe(1500);
    expect(toMinor("1.234", "KWD", "en-GB")).toBe(1234);
    expect(toMinor("1.005", "EUR", "en-GB")).toBe(101);
    expect(toMinor("0,5", "EUR", "de-DE")).toBe(50);
    expect(toMinor("1.2345", "CLF", "en-GB")).toBe(12345);
  });
});

describe("minorToInput", () => {
  it("is what the same locale reads back", () => {
    for (const [minor, cur, loc] of [
      [150_050, "EUR", "de-DE"],
      [1500, "JPY", "en-GB"],
      [1_234_567, "KWD", "fr-FR"],
    ] as const) {
      const text = minorToInput(minor, cur, loc);
      expect(toMinor(text, cur, loc)).toBe(minor);
    }
    expect(minorToInput(150_050, "EUR", "de-DE")).toBe("1500,50");
    expect(minorToInput(1500, "JPY", "en-GB")).toBe("1500");
  });
});

describe("currencyCode", () => {
  it("is three letters, upper-cased, and nothing is assumed", () => {
    expect(currencyCode("jpy")).toBe("JPY");
    expect(currencyCode(" EUR ")).toBe("EUR");
    expect(currencyCode("")).toBeNull();
    expect(currencyCode("EU")).toBeNull();
    expect(currencyCode("E1R")).toBeNull();
  });
});

describe("no built-in currency (spec 02-scope 2.0)", () => {
  const read = (p: string) =>
    readFileSync(join(__dirname, "../../src/routes", p), "utf8");
  it.each([
    ["plans/[pid]/costs/+page.svelte", /let currency = \$state\(""\)/],
    ["plans/[pid]/governance/+page.svelte", /let lineCurrency = \$state\(""\)/],
    ["cost-rates/+page.svelte", /let currency = \$state\(""\)/],
    ["scenarios/+page.svelte", /let currency = \$state\(""\)/],
    ["proposals/+page.svelte", /let currency = \$state\(""\)/],
  ])("%s starts with no currency", (file, empty) => {
    const source = read(file);
    expect(source).toMatch(empty);
    expect(source).not.toMatch(/\$state\("GBP"\)/);
  });
  it("no page divides or multiplies money by a fixed 100", () => {
    for (const file of [
      "ledger/+page.svelte",
      "plans/[pid]/deployment-moves/+page.svelte",
      "plans/[pid]/governance/+page.svelte",
      "plans/[pid]/joint-risk/+page.svelte",
      "scenarios/+page.svelte",
      "proposals/+page.svelte",
      "change-requests/+page.svelte",
    ]) {
      expect(read(file), file).not.toMatch(
        /(minor|Cost|cap|requested|Number\([^)]*\)) ?[*/] ?100\b/,
      );
    }
  });
});
