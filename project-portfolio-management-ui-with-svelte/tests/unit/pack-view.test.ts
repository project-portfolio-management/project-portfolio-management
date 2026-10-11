// DP-11: the director pack page's view. Pins that every fixed word comes from the catalogue (English and
// Welsh), that every figure in the pack's JSON is in the view, that a reading is a word and a symbol as well as
// a tone, and that money is never mixed across currencies.
import { describe, expect, it } from "vitest";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import { PAGES_CY } from "../../src/lib/i18n-pages.cy";
import { buildPackView, money, type Deps, type Section } from "../../src/lib/pack-view";
import { BODY } from "./fixtures/director-pack-body";

function deps(cat: Record<string, string>, locale: string, used: Set<string>): Deps {
  const fill = (s: string, v?: Record<string, string | number>) => s.replace(/\{(\w+)\}/g, (_, k: string) => String(v?.[k] ?? `{${k}}`));
  return {
    locale,
    t: (key, values) => {
      used.add(key);
      const s = cat[key];
      if (s === undefined) throw new Error(`missing catalogue key ${key}`);
      return fill(s, values);
    },
    label: (group, value) => {
      const key = `l.enum.${group}.${String(value)}`;
      used.add(key);
      const s = cat[key];
      if (s === undefined) throw new Error(`missing catalogue key ${key}`);
      return s;
    },
  };
}
const flat = (secs: Section[]) =>
  secs.flatMap((s) => [s.title, ...s.lines, ...s.tables.flatMap((t) => [t.caption, ...t.columns, ...t.rows.flatMap((r) => r.map((c) => `${c.symbol ?? ""}${c.text}`))])]).join("\n");

describe("the director pack view", () => {
  const usedEn = new Set<string>();
  const en = buildPackView(BODY, deps(PAGES_EN as unknown as Record<string, string>, "en-GB", usedEn));
  const text = flat(en);

  it("has every section, in order, with a heading from the catalogue", () => {
    expect(en.map((s) => s.id)).toEqual(["quality", "changed", "map", "exceptions", "gates", "funding", "fever", "evm", "benefits", "capacity", "signoffs", "asks", "keypeople", "triage", "cutovers", "outcomes", "watermelon", "issues", "changes"]);
    expect(en.every((s) => s.title.length > 2)).toBe(true);
  });

  it("uses only keys the English and the Welsh catalogue both have", () => {
    const usedCy = new Set<string>();
    buildPackView(BODY, deps(PAGES_CY as unknown as Record<string, string>, "cy-GB", usedCy));
    expect([...usedCy].sort()).toEqual([...usedEn].sort());
    for (const k of usedEn) {
      expect(PAGES_CY[k as keyof typeof PAGES_CY], k).toBeTruthy();
    }
  });

  it("shows the figures the JSON holds", () => {
    for (const needle of ["17", "9", "was 4", "Care", "Safer care", "Approve the extra supplier", "Programme director", "2026-10-20", "70", "11", "40", "£100,000.00", "£25,000.00", "£75,000.00", "JP¥123,456", "£110,000.00", "40.0", "60.0", "-20.0", "£4,000.00", "£2,000.00", "2.00", "0.80", "£1,000.00", "£600.00", "£200.00", "£400.00", "2.50"]) {
      expect(text, needle).toContain(needle);
    }
  });

  it("states every reading as a word and a symbol as well as a tone", () => {
    const cells = en.flatMap((s) => s.tables.flatMap((t) => t.rows.flat())).filter((c) => c.tone);
    expect(cells.length).toBeGreaterThan(6);
    for (const c of cells) {
      expect(c.text.length, JSON.stringify(c)).toBeGreaterThan(2);
      if (c.tone !== "grey" || c.text !== "") expect(c.symbol, JSON.stringify(c)).toBeDefined();
    }
    const shapes = new Map(cells.map((c) => [c.text.split(";")[0]!, c.symbol]));
    expect(shapes.get("shortfall")).not.toBe(shapes.get("enough"));
    expect(shapes.get("no supply recorded")).toBe("–");
  });

  it("marks a self-assessed and a stale reading, and puts a programme with no aim under 'No strategic aim'", () => {
    expect(text).toContain("amber/red (self-assessed, stale)");
    expect(text).toContain("No strategic aim");
    expect(text).toContain("no funding source");
  });

  it("says what is missing instead of showing zero", () => {
    expect(text).toContain("no fever-chart point: no schedule baseline");
    expect(text).toContain("Loose: no baseline");
    expect(text).toContain("Below the earned-value threshold: budget at completion €50.00");
    expect(text).toContain("recovery: drawn at the lower edge");
    expect(text).toContain("no supply recorded");
    expect(text).toContain("no pool assigned");
    // A supply of none is a dash, not 0.00.
    const row = en.find((s) => s.id === "capacity")!.tables[0]!.rows.find((r) => r[2]!.text.startsWith("no supply"))!;
    expect(row[4]!.text).toBe("–");
    expect(row[5]!.text).toBe("–");
  });

  it("never adds currencies: each money cell names its own", () => {
    expect(money(123_456, "JPY", "en-GB")).toBe("JP¥123,456");
    expect(money(123_456, "GBP", "en-GB")).toBe("£1,234.56");
    expect(money(1234, "KWD", "en-GB")).toContain("1.234");
    expect(money(null, "GBP", "en-GB")).toBe("–");
  });

  it("reads the service's sentences for what changed, and leaves a sentence it does not know as it is", () => {
    const secs = buildPackView({ ...BODY, what_changed: ["Open issues: 4 (no earlier pack to compare with)", "Something else: 3"] }, deps(PAGES_EN as unknown as Record<string, string>, "en-GB", new Set()));
    const lines = secs.find((s) => s.id === "changed")!.lines;
    expect(lines[0]).toBe("Open issues: 4, no earlier pack to compare with");
    expect(lines[1]).toBe("Something else: 3");
  });

  it("copes with an empty body without throwing", () => {
    const secs = buildPackView({}, deps(PAGES_EN as unknown as Record<string, string>, "en-GB", new Set()));
    expect(secs).toHaveLength(19);
  });

  it("renders in Welsh with Welsh words", () => {
    const cy = flat(buildPackView(BODY, deps(PAGES_CY as unknown as Record<string, string>, "cy-GB", new Set())));
    expect(cy).toContain("Map y portffolio");
    expect(cy).toContain("diffyg");
    expect(cy).toContain("dim cyflenwad wedi'i gofnodi");
    expect(cy).not.toContain("Portfolio map");
  });
});
