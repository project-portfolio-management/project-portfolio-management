// LT-12: one primary action per screen. A screen offers many things to do, but the
// one most people came for is styled as the primary button, and only one is: when
// everything shouts, nothing does. Checked on every route as it first renders.

import { test, expect } from "@playwright/test";
import { ROUTES, stubPlansApi } from "./fixtures";

test.describe("at most one primary action per screen (LT-12)", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  for (const route of ROUTES) {
    test(`${route} shows at most one primary button`, async ({ page }) => {
      await page.goto(`/en-001${route === "/" ? "" : route}`, { waitUntil: "networkidle" });
      await expect(page.locator("h1").first()).toBeVisible();
      const names = await page
        .locator("button.primary:visible, a.primary:visible, a.button.primary:visible")
        .evaluateAll((els) => els.map((e) => (e.textContent ?? "").trim()));
      expect(names, "primary buttons on screen").toHaveLength(Math.min(names.length, 1));
    });
  }
});
