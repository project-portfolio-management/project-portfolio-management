// Plural rules (WW-1.24 in part; UI spec "Numbers, dates and plurals"): a string that fills a count is a plural group
// chosen with `Intl.PluralRules` in the page's locale (`tn`), never with `n === 1`. Ported from the workforce planning
// management UI's test of the same name (WPM-T374), adapted to this flat catalogue (`<base>_one`, `<base>_other`).

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

import {
  LOCALES,
  PLURAL_CATEGORIES,
  PLURAL_KEYS,
  STRINGS_BY_LOCALE,
  pluralCategories,
  pluralCategory,
  pluralKey,
  translatePlural,
} from "../../src/lib/i18n.svelte";

const KEY = "p.common.business_day" as const;

describe("plural selection", () => {
  it("English has two forms: one and other", () => {
    expect(pluralCategory(1, "en-001")).toBe("one");
    expect(pluralCategory(-1, "en-001")).toBe("one");
    expect(pluralCategory(0, "en-001")).toBe("other");
    expect(pluralCategory(2, "en-001")).toBe("other");
    expect(pluralCategory(1.5, "en-001")).toBe("other");
    expect(translatePlural(KEY, 1, "en-001", { n: 1 })).toBe("1 business day");
    expect(translatePlural(KEY, 5, "en-001", { n: 5 })).toBe("5 business days");
  });

  it("Arabic and Welsh have six categories, each with its own rules", () => {
    expect(pluralCategories("ar-001")).toEqual([...PLURAL_CATEGORIES]);
    expect(
      [0, 1, 2, 3, 11, 100].map((n) => pluralCategory(n, "ar-001")),
    ).toEqual(["zero", "one", "two", "few", "many", "other"]);
    expect(pluralCategories("cy-001")).toEqual([...PLURAL_CATEGORIES]);
    expect([0, 1, 2, 3, 6, 4].map((n) => pluralCategory(n, "cy-001"))).toEqual([
      "zero",
      "one",
      "two",
      "few",
      "many",
      "other",
    ]);
  });

  it("French counts 0 and 1 as one; Chinese has one form for every count", () => {
    expect(pluralCategory(0, "fr-001")).toBe("one");
    expect(pluralCategory(1, "fr-001")).toBe("one");
    expect(pluralCategories("zh-001")).toEqual(["other"]);
    expect(pluralCategories("zh-cn")).toEqual(["other"]);
  });

  it("a Welsh category the catalogue lacks uses the Welsh other form, not English", () => {
    // Welsh 2 is "two"; the catalogue holds only one and other for this group (machine-written, unreviewed).
    expect(pluralKey(KEY, 2, "cy-001")).toBe(`${KEY}_other`);
    expect(translatePlural(KEY, 2, "cy-001", { n: 2 })).toBe(
      "2 diwrnod gwaith",
    );
    expect(pluralKey(KEY, 1, "cy-001")).toBe(`${KEY}_one`);
  });

  it("every locale has text for the group, so its own rules choose the form (WW-1.1)", () => {
    // Arabic 1 is "one"; 0 is "zero", which the catalogue lacks, so Arabic's other form.
    expect(pluralKey(KEY, 1, "ar-001")).toBe(`${KEY}_one`);
    expect(pluralKey(KEY, 0, "ar-001")).toBe(`${KEY}_other`);
    // Chinese has only "other", so 1 is "other" in Chinese (until WW-1.1 it fell back to English's "one").
    expect(pluralKey(KEY, 1, "zh-cn")).toBe(`${KEY}_other`);
    expect(translatePlural(KEY, 1, "de-de", { n: 1 })).toBe("1 Arbeitstag");
    expect(translatePlural(KEY, 2, "de-de", { n: 2 })).toBe("2 Arbeitstage");
  });

  it("no English string fills a count with (s)", () => {
    const english = Object.entries(STRINGS_BY_LOCALE["en-001"]).filter(
      ([, text]) => /\w\(s\)/.test(text),
    );
    expect(english).toEqual([]);
  });
});

describe("every plural group has the forms it needs", () => {
  it("finds the plural groups", () => {
    expect(PLURAL_KEYS.length).toBeGreaterThanOrEqual(54);
    expect(PLURAL_KEYS).toContain(KEY);
  });

  it("English holds exactly one and other for each group", () => {
    const en = STRINGS_BY_LOCALE["en-001"];
    const problems: string[] = [];
    for (const base of PLURAL_KEYS) {
      const have = PLURAL_CATEGORIES.filter(
        (c) => en[`${base}_${c}`] !== undefined,
      );
      if (have.join() !== "one,other")
        problems.push(`${base}: has ${have.join(",")}`);
    }
    expect(problems, problems.join("\n")).toEqual([]);
  });

  it("each locale holds only categories its language uses, and other wherever it holds any", () => {
    const problems: string[] = [];
    for (const locale of LOCALES) {
      const table = STRINGS_BY_LOCALE[locale];
      const need = pluralCategories(locale) as string[];
      for (const base of PLURAL_KEYS) {
        const have = PLURAL_CATEGORIES.filter(
          (c) => table[`${base}_${c}`] !== undefined,
        );
        if (have.length === 0) continue;
        if (!have.includes("other"))
          problems.push(`${locale} ${base}: no other`);
        // English "one" is kept where a language has none (Chinese), as nothing reads it there.
        for (const c of have)
          if (!need.includes(c) && c !== "one")
            problems.push(
              `${locale} ${base}: ${c} is not a ${locale} category`,
            );
        for (const c of have)
          if (!table[`${base}_${c}`]?.trim())
            problems.push(`${locale} ${base}_${c} is empty`);
      }
    }
    expect(problems, problems.join("\n")).toEqual([]);
  });

  it("Welsh holds every group (one and other at least; zero, two, few and many are not written yet)", () => {
    const cy = STRINGS_BY_LOCALE["cy-001"];
    const missing = PLURAL_KEYS.filter(
      (b) => cy[`${b}_one`] === undefined || cy[`${b}_other`] === undefined,
    );
    expect(missing).toEqual([]);
  });
});

// The scan: code names a plural group's `_one` or `_other` key itself only when it picks the form by hand
// (`n === 1 ? "…_one" : "…_other"`, English's rule). `tn` takes the base key.
const here = join(__dirname, "..", "..");

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) out.push(...walk(path));
    else if (/\.(svelte|ts)$/.test(path)) out.push(path);
  }
  return out;
}

/** Lines of `text` that name a plural group's form directly. */
export function handPicked(
  text: string,
  groups: ReadonlySet<string>,
): string[] {
  const found: string[] = [];
  for (const line of text.split("\n")) {
    for (const m of line.matchAll(/"([\w.]+)_(zero|one|two|few|many|other)"/g))
      if (m[1] && groups.has(m[1])) found.push(line.trim().slice(0, 120));
  }
  return found;
}

describe("no plural form chosen by hand in src/", () => {
  const groups = new Set<string>(PLURAL_KEYS);

  it("finds it when it is there (the scan works)", () => {
    expect(
      handPicked('n === 1 ? t("p.common.business_day_one", { n }) : x', groups),
    ).toHaveLength(1);
    expect(handPicked('tn("p.common.business_day", n, { n })', groups)).toEqual(
      [],
    );
    // A key that merely ends in _one, with no _other beside it, is not a plural group.
    expect(handPicked('t("p.tolerances.raise_one")', groups)).toEqual([]);
  });

  it("finds none", () => {
    const offenders: string[] = [];
    for (const file of walk(join(here, "src"))) {
      if (/i18n\.svelte\.ts$|i18n-pages(?:\.[a-z]+)?\.ts$/.test(file)) continue;
      for (const f of handPicked(readFileSync(file, "utf8"), groups))
        offenders.push(`${relative(here, file)}: ${f}`);
    }
    expect(offenders, offenders.join("\n")).toEqual([]);
  });
});
