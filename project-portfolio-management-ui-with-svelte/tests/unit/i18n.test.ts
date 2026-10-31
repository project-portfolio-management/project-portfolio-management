import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, it, expect, vi } from "vitest";

// The i18n store seeds from localStorage behind `$app/environment`'s
// `browser`; stub it to the server value so the module is deterministic.
vi.mock("$app/environment", () => ({ browser: false }));

import {
  LOCALES,
  LOCALE_LABELS,
  STRING_KEYS,
  STRINGS_BY_LOCALE,
  DEFAULT_LOCALE,
  translate,
  isRtl,
  i18n,
  toMessageFormat,
  type Locale,
} from "../../src/lib/i18n.svelte";
import { PAGE_KEYS } from "../../src/lib/i18n-pages";

describe("i18n catalog", () => {
  it("supports exactly the required locales, sorted by code", () => {
    expect([...LOCALES]).toEqual([
      "ar-001",
      "cy-001",
      "de-001",
      "de-de",
      "en-001",
      "es-001",
      "fr-001",
      "hi-001",
      "zh-001",
      "zh-cn",
    ]);
    expect(LOCALE_LABELS).toEqual({
      "ar-001": "العربية",
      "cy-001": "Cymraeg",
      "de-001": "Deutsch",
      "de-de": "Deutsch - Deutschland",
      "en-001": "English",
      "es-001": "Español",
      "fr-001": "Français",
      "hi-001": "हिन्दी",
      "zh-001": "中文",
      "zh-cn": "中文 - 中国",
    });
  });

  it("labels never use parentheses, and -001 locales are language-only", () => {
    for (const [code, label] of Object.entries(LOCALE_LABELS)) {
      expect(label).not.toMatch(/[()]/);
      if (code.endsWith("-001")) expect(label).not.toContain(" - ");
    }
  });

  it("normalizes legacy and regional codes to the supported locale of that language", () => {
    i18n.set("en_US");
    expect(i18n.locale).toBe("en-001");
    i18n.set("en-US");
    expect(i18n.locale).toBe("en-001");
    i18n.set("es-MX");
    expect(i18n.locale).toBe("es-001");
    i18n.set("zh-cn");
    expect(i18n.locale).toBe("zh-cn");
    i18n.set("ZH-CN");
    expect(i18n.locale).toBe("zh-cn");
    i18n.set("de-DE");
    expect(i18n.locale).toBe("de-de");
    i18n.set("ja");
    expect(i18n.locale).toBe("en-001");
  });

  it("has a human-readable label for every locale", () => {
    for (const locale of LOCALES) {
      expect(LOCALE_LABELS[locale]).toBeTruthy();
    }
  });

  // The catalogue has two tiers (HD-13). The long-standing keys (the chrome, the first screens,
  // the merge flow, ...) are translated in EVERY locale, and that guarantee is frozen: the list in
  // fixtures/catalogue-baseline-keys.json may only grow. The page and component keys (`p.*`, `c.*`,
  // `i18n-pages.ts`) are complete in English and Welsh; the other locales show English for them
  // until they are translated. Welsh completeness is pinned below and in catalogue-coverage.test.ts.
  it("every locale covers every long-standing key (frozen baseline), with no extra keys", () => {
    const baseline = JSON.parse(
      readFileSync(
        resolve(
          process.cwd(),
          "tests/unit/fixtures/catalogue-baseline-keys.json",
        ),
        "utf8",
      ),
    ) as string[];
    expect(baseline.length).toBeGreaterThan(300);
    for (const locale of LOCALES) {
      const table = STRINGS_BY_LOCALE[locale];
      for (const key of baseline) {
        expect(table[key], `${locale} missing ${key}`).toBeTruthy();
      }
      // Nothing beyond the English source of truth.
      for (const key of Object.keys(table)) {
        expect(
          STRING_KEYS as readonly string[],
          `${locale} has an extra key ${key}`,
        ).toContain(key);
      }
    }
  });

  it("Welsh (cy-001) covers every key English has", () => {
    const cy = STRINGS_BY_LOCALE["cy-001"];
    const missing = STRING_KEYS.filter((k) => !cy[k]);
    expect(
      missing,
      `cy-001 is missing ${missing.length} keys, e.g. ${missing.slice(0, 5)}`,
    ).toEqual([]);
  });

  it("every locale has its own text for every page key (WW-1.1); none falls back to English", () => {
    for (const locale of LOCALES) {
      const table = STRINGS_BY_LOCALE[locale] as Record<string, string>;
      expect(
        [...PAGE_KEYS].filter((k) => table[k] === undefined),
        locale,
      ).toEqual([]);
    }
    expect(translate("p.cost_rates.a_person" as never, "de-de")).not.toBe(
      translate("p.cost_rates.a_person" as never, "en-001"),
    );
  });

  it("default locale is English", () => {
    expect(DEFAULT_LOCALE).toBe("en-001");
  });

  it("spot-checks the PPM keys in non-Latin locales", () => {
    expect(translate("ppm.dashboard.title", "zh-cn")).toBe("组合仪表盘");
    expect(translate("ppm.gov.title", "ar-001")).toBe("الحوكمة");
    expect(translate("ppm.nav.proposals", "hi-001")).toBe("प्रस्ताव");
  });

  // The merge page's keys: present in English, genuinely translated
  // elsewhere (not English copies), and carrying both placeholders in
  // the confirmation prompt so the substitution cannot silently break.
  it("covers the merge keys in every locale", () => {
    const mergeKeys = STRING_KEYS.filter(
      (key) => key === "nav.merge" || key.startsWith("merge."),
    );
    expect(mergeKeys.length).toBeGreaterThanOrEqual(24);
    for (const locale of LOCALES) {
      for (const key of mergeKeys) {
        expect(
          translate(key, locale, { dup: "x", main: "y" }),
          `${locale} missing ${key}`,
        ).toBeTruthy();
      }
      // Both values must arrive (MF2 may wrap them in bidi isolates).
      const confirm = translate("merge.confirm", locale, {
        dup: "DUP-ID",
        main: "MAIN-ID",
      });
      expect(confirm, `${locale} merge.confirm dup`).toContain("DUP-ID");
      expect(confirm, `${locale} merge.confirm main`).toContain("MAIN-ID");
      expect(confirm, `${locale} unfilled placeholder`).not.toMatch(/[{}$]/);
    }
    expect(translate("merge.title", "fr-001")).toBe("Fusionner des plans");
    expect(translate("merge.title", "zh-cn")).toBe("合并计划");
    expect(translate("nav.merge", "ar-001")).toBe("دمج");
  });

  it("spot-checks a non-Latin locale (Chinese)", () => {
    expect(translate("nav.cases", "zh-cn")).toBe("案件");
    expect(translate("form.save", "zh-cn")).toBe("保存");
  });

  it("spot-checks a right-to-left locale (Arabic)", () => {
    expect(translate("nav.cases", "ar-001")).toBe("القضايا");
  });

  it("falls back to English then to the key", () => {
    // A locale not present falls back to English.
    expect(translate("form.save", "xx" as unknown as Locale)).toBe(
      translate("form.save", "en-001"),
    );
  });

  it("isRtl is true for ar-001 only", () => {
    expect(isRtl("ar-001")).toBe(true);
    // Legacy / region subtags are tolerated.
    expect(isRtl("ar")).toBe(true);
    expect(isRtl("ar-EG")).toBe(true);
    for (const locale of LOCALES) {
      if (locale !== "ar-001") expect(isRtl(locale)).toBe(false);
    }
  });

  it("has the splash copy in every locale (six benefits and six features)", () => {
    for (const locale of LOCALES) {
      for (const area of ["benefits", "features", "trust"]) {
        for (let n = 1; n <= 6; n++) {
          for (const part of ["title", "body"]) {
            const key = `splash.${area}.${n}.${part}` as never;
            expect(translate(key, locale), `${locale} ${key}`).not.toBe(key);
          }
        }
      }
    }
  });
});

describe("Sveltia I18n catalog", () => {
  // Every catalog string is plain text; MessageFormat 2 must give it back
  // unchanged, whatever braces, backslashes or leading dots it contains.
  it("returns every string of every locale unchanged (placeholders aside)", () => {
    for (const locale of LOCALES) {
      for (const [key, source] of Object.entries(STRINGS_BY_LOCALE[locale])) {
        if (/\{[A-Za-z_]\w*\}/.test(source)) continue;
        expect(translate(key as never, locale), `${locale} ${key}`).toBe(
          source,
        );
      }
    }
  });

  it("escapes MF2 syntax in plain text and converts placeholders", () => {
    expect(toMessageFormat("a {b} c")).toBe("{{a {$b} c}}");
    expect(toMessageFormat("{ } \\ .x")).toBe("{{\\{ \\} \\\\ .x}}");
  });

  it("falls back to the key when no locale has it", () => {
    expect(translate("no.such.key" as never, "cy-001")).toBe("no.such.key");
  });
});

describe("copied locales", () => {
  it("de-001 and zh-001 start as copies of de-de and zh-cn", () => {
    expect(STRINGS_BY_LOCALE["de-001"]).toEqual(STRINGS_BY_LOCALE["de-de"]);
    expect(STRINGS_BY_LOCALE["zh-001"]).toEqual(STRINGS_BY_LOCALE["zh-cn"]);
    expect(translate("nav.merge", "de-001")).toBe(
      translate("nav.merge", "de-de"),
    );
  });
});
