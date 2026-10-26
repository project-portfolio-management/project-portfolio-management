// BR-17: a person hides a section of the top bar on the Display page; it is gone after a reload, everything else stays,
// nothing is stored for a section never hidden, and the hidden page still works at its address. axe passes on the page.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle, stubPlansApi } from "./fixtures";

// The bar sits behind the menu button until it is opened; opening it again is harmless.
const bar = (page: Page) => page.locator("#primary-nav");
async function openMenu(page: Page) {
  const button = page.locator("button.hamburger");
  if ((await button.getAttribute("aria-expanded")) !== "true") await button.click();
}
const stored = (page: Page) =>
  page.evaluate(() => JSON.parse(localStorage.getItem("mxi.display") ?? "{}") as { hidden?: string[] });

test.beforeEach(async ({ page }) => {
  await stubPlansApi(page);
});

test("a hidden section is gone after a reload, the rest stay, and showing all again stores nothing", async ({ page }) => {
  await page.goto("/en-001/display");
  await openMenu(page);
  await expect(bar(page).getByRole("link", { name: "Goals (OKRs)" })).toBeVisible();
  expect(await stored(page)).toEqual({});

  await page.getByRole("checkbox", { name: "Goals (OKRs)" }).uncheck();
  await expect(bar(page).getByRole("link", { name: "Goals (OKRs)" })).toBeHidden();
  await expect(page.getByText("1 section hidden")).toBeVisible();
  expect((await stored(page)).hidden).toEqual(["/okrs"]);

  await page.reload();
  await openMenu(page);
  await expect(bar(page).getByRole("link", { name: "Goals (OKRs)" })).toBeHidden();
  await expect(page.getByRole("checkbox", { name: "Goals (OKRs)" })).not.toBeChecked();
  // Only that one: another section, and the way home, are still there.
  await expect(bar(page).getByRole("link", { name: "Key measures (KPIs)" })).toBeVisible();
  await expect(bar(page).locator("a").first()).toBeVisible(); // the way home

  // Hidden is not removed: the page still works at its address.
  await page.goto("/en-001/okrs");
  await expect(page.locator("h1").first()).toBeVisible();
  await openMenu(page);
  await expect(bar(page).getByRole("link", { name: "Goals (OKRs)" })).toBeHidden();

  await page.goto("/en-001/display");
  await page.getByRole("button", { name: "Show all sections" }).click();
  await openMenu(page);
  await expect(bar(page).getByRole("link", { name: "Goals (OKRs)" })).toBeVisible();
  await expect(page.getByText("No section is hidden.")).toBeVisible();
  expect(await stored(page)).toEqual({});
});

test("axe finds no serious or critical violation on the page with its sections", async ({ page }) => {
  await page.goto("/en-001/display");
  await page.getByRole("checkbox", { name: "Goals (OKRs)" }).uncheck();
  await expect(page.getByText("1 section hidden")).toBeVisible();
  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  const bad = results.violations.filter((v) => v.impact === "serious" || v.impact === "critical");
  expect(bad.map((v) => `${v.id}: ${v.help}`)).toEqual([]);
});
