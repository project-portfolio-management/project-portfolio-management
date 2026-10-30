// LT-12: an axe-core audit of **every page route** (enumerated from `src/routes`
// when the suite runs, so a new page cannot go unaudited), over the same stubbed
// plans API the phone audit uses. Zero serious or critical WCAG 2 A/AA violations.
// Pages whose data calls are not stubbed render their own error state, which is
// audited too — an error page must be accessible as well.

import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { ROUTES, settle, stubPlansApi } from "./fixtures";

test.describe("axe audit of every route (LT-12)", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  test("the route list is not empty and includes the pages added recently", () => {
    expect(ROUTES.length).toBeGreaterThan(40);
    for (const r of [
      "/me",
      "/okrs",
      "/kpis",
      "/milestones",
      "/resources",
      "/cost-risk",
      "/display",
    ]) {
      expect(ROUTES, r).toContain(r);
    }
  });

  // Every route under the light and the dark system preference: the layout follows the OS preference
  // (ThemePicker `detectFromSystem`), so the dark theme is what a dark-mode visitor sees first.
  for (const route of ROUTES)
    for (const colorScheme of ["light", "dark"] as const) {
      test(`${route} has no serious or critical violations${colorScheme === "dark" ? " (dark theme)" : ""}`, async ({
        page,
      }) => {
        await page.emulateMedia({ colorScheme });
        await page.goto(`/en-001${route === "/" ? "" : route}`, {
          waitUntil: "networkidle",
        });
        await expect(page.locator("h1").first()).toBeVisible();
        await settle(page);
        const results = await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
          .analyze();
        const bad = results.violations.filter(
          (v) => v.impact === "serious" || v.impact === "critical",
        );
        expect(
          bad.map(
            (v) =>
              `${v.id}: ${v.help} — ${v.nodes
                .slice(0, 3)
                .map(
                  (n) =>
                    `${n.target.join(" ")} [${n.any[0]?.message ?? n.all[0]?.message ?? ""}]`,
                )
                .join("; ")}`,
          ),
        ).toEqual([]);
      });
    }
});
