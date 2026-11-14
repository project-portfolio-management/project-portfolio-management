// RE-9: accessibility-tree snapshots of the main region in the states a person meets most, in English and
// Welsh. axe says whether the tree is valid; a snapshot says whether it is still the tree we meant: a
// lost label, heading or landmark changes it. Snapshots live in structure.spec.ts-snapshots/ and are
// reviewed like code; update them with `--update-snapshots` only when the change is intended.
import { test, expect } from "./test";
import { PID, stubPlansApi } from "./fixtures";

const LOCALES = ["en-001", "cy-001"] as const;

for (const locale of LOCALES) {
  test.describe(`structure in ${locale}`, () => {
    test.beforeEach(async ({ page }) => {
      await stubPlansApi(page);
    });

    test("the plan list", async ({ page }) => {
      await page.goto(`/${locale}/plans`, { waitUntil: "networkidle" });
      await expect(page.locator("main")).toMatchAriaSnapshot({
        name: `plans-${locale}.aria.yml`,
      });
    });

    test("a plan's page", async ({ page }) => {
      await page.goto(`/${locale}/plans/${PID}`, { waitUntil: "networkidle" });
      await expect(page.locator("main")).toMatchAriaSnapshot({
        name: `plan-${locale}.aria.yml`,
      });
    });

    test("the new plan form after a blank name is submitted", async ({
      page,
    }) => {
      await page.goto(`/${locale}/plans/new`, { waitUntil: "networkidle" });
      // Spaces pass the browser's `required` check and reach the form's own rule (a blank name).
      await page.locator("main form input[type=text]").first().fill("   ");
      await page.locator("main form button[type=submit]").click();
      await expect(page.locator("main [role=alert]")).toBeVisible();
      await expect(page.locator("main")).toMatchAriaSnapshot({
        name: `plan-form-error-${locale}.aria.yml`,
      });
    });

    test("a page that does not exist", async ({ page }) => {
      await page.goto(`/${locale}/no-such-page`, { waitUntil: "networkidle" });
      await expect(page.locator("main")).toMatchAriaSnapshot({
        name: `not-found-${locale}.aria.yml`,
      });
    });

    test("sign-in", async ({ page }) => {
      await page.goto(`/${locale}/signin`, { waitUntil: "networkidle" });
      await expect(page.locator("main")).toMatchAriaSnapshot({
        name: `signin-${locale}.aria.yml`,
      });
    });
  });
}
