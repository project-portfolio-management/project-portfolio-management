// HD-13: every page renders fully in Welsh. Each real page route (enumerated from `src/routes`)
// is loaded in English and in Welsh over the same stubbed API; any line of visible text that is
// identical in both and is not data, a name or a symbol is untranslated.

import { test, expect } from "@playwright/test";
import { ROUTES, stubPlansApi } from "./fixtures";

/** Lines that are the same in both languages by design: product, data and fixture names, units. */
const SAME = new Set([
  "PPM", "Website replatform", "WEB-42", "Kanban", "Gantt", "CSV", "GBP", "PERT", "RACI", "Main X",
  "Project", "Active", "platform", "Cymraeg", "English",
  // Catalogue words that are the same in Welsh, and the stub API's own error text (test data):
  "band", "capex", "opex", "unhandled in stub", "404: unhandled in stub",
]);

async function lines(page: import("@playwright/test").Page, path: string): Promise<string[]> {
  await page.goto(path, { waitUntil: "networkidle" });
  await expect(page.locator("h1").first()).toBeVisible();
  const text = await page.locator("body").innerText();
  return text
    .split("\n")
    .map((l) => l.replace(/[⁦-⁩‎‏]/g, "").trim())
    .filter((l) => /[A-Za-z]{4,}/.test(l));
}

test.describe("every page renders in Welsh (HD-13)", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  for (const route of ROUTES) {
    test(`${route} is fully Welsh`, async ({ page }) => {
      const suffix = route === "/" ? "" : route;
      const en = new Set(await lines(page, `/en-001${suffix}`));
      const cy = await lines(page, `/cy-001${suffix}`);
      await expect(page.locator("html")).toHaveAttribute("lang", "cy-001");
      const untranslated = cy.filter((l) => en.has(l) && !SAME.has(l));
      expect(untranslated).toEqual([]);
    });
  }
});
