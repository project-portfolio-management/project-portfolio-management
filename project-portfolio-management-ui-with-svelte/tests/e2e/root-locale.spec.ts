// The root route sends each browser to the locale it reads best: `/` plus the browser's
// language (`navigator.language`, `Accept-Language`) becomes `/<locale>`. A region the app
// has no page for resolves to the language's own locale (`cy-GB` -> `/cy-001`); a language
// it does not have gets English; the reader's remembered choice wins over the browser.

import { test, expect } from "@playwright/test";
import { stubPlansApi } from "./fixtures";

const CASES: [string, string][] = [
  ["cy-GB", "/cy-001"],
  ["de-DE", "/de-de"],
  ["de-AT", "/de-001"],
  ["zh-CN", "/zh-cn"],
  ["es-MX", "/es-001"],
  ["ar-EG", "/ar-001"],
  ["ja-JP", "/en-001"],
];

for (const [language, expected] of CASES) {
  test.describe(`browser language ${language}`, () => {
    test.use({ locale: language });
    test(`"/" goes to ${expected}`, async ({ page }) => {
      await stubPlansApi(page);
      expect(await page.evaluate(() => navigator.language)).toBeTruthy();
      await page.goto("/");
      await expect(page).toHaveURL(new RegExp(`${expected}/?$`));
      expect(new URL(page.url()).pathname.replace(/\/$/, "")).toBe(expected);
      await expect(page.locator("html")).toHaveAttribute(
        "lang",
        expected.slice(1),
      );
    });
  });
}

test.describe("a remembered choice", () => {
  test.use({ locale: "cy-GB" });
  test("wins over the browser's language", async ({ page }) => {
    await stubPlansApi(page);
    await page.goto("/fr-001");
    await page.goto("/");
    await expect(page).toHaveURL(/\/fr-001\/?$/);
    expect(new URL(page.url()).pathname.replace(/\/$/, "")).toBe("/fr-001");
  });
});

test.describe("a path that already names a locale", () => {
  test.use({ locale: "cy-GB" });
  test("is left where it is", async ({ page }) => {
    await stubPlansApi(page);
    await page.goto("/de-de");
    expect(new URL(page.url()).pathname.replace(/\/$/, "")).toBe("/de-de");
  });
});
