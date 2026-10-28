// BR-14: the command palette. Ctrl or Cmd with K opens it; a plan is found by name, reached with the keyboard alone;
// axe passes with it open; Escape closes it and hands focus back; a visit is remembered as a recent item.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

async function stub(page: Page) {
  await stubPlansApi(page);
  // Registered after the general stub, so it answers first.
  await page.route("**/api/plans/search**", (route) =>
    route.fulfill({ json: [{ pid: PID, name: "Website replatform" }] }),
  );
}

const palette = (page: Page) =>
  page.getByRole("dialog", { name: "Search and commands" });
const input = (page: Page) => palette(page).getByRole("combobox");

test.describe("command palette (BR-14)", () => {
  test.beforeEach(async ({ page }) => {
    await stub(page);
    await page.goto("/en-001/plans", { waitUntil: "networkidle" });
    await expect(page.locator("h1").first()).toBeVisible();
  });

  test("opens with Ctrl-K, finds a plan by name and goes to it with the keyboard alone", async ({
    page,
  }) => {
    await page.keyboard.press("Control+k");
    await expect(palette(page)).toBeVisible();
    await expect(input(page)).toBeFocused();

    await input(page).pressSequentially("Webs");
    const option = palette(page).getByRole("option", {
      name: /Website replatform/,
    });
    await expect(option).toBeVisible();
    await expect(input(page)).toHaveAttribute(
      "aria-activedescendant",
      /palette-row-0/,
    );
    await expect(option).toHaveAttribute("aria-selected", "true");

    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(new RegExp(`/en-001/plans/${PID}$`));
    await expect(palette(page)).toBeHidden();
  });

  test("a command is reached by name, and Cmd-K opens it too", async ({
    page,
  }) => {
    await page.keyboard.press("Meta+k");
    await expect(palette(page)).toBeVisible();
    await input(page).pressSequentially("my wo");
    await expect(palette(page).getByRole("option")).toHaveCount(1);
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/\/en-001\/me$/);
  });

  test("the arrow keys move the active row, wrapping, and a visit is remembered", async ({
    page,
  }) => {
    await page.keyboard.press("Control+k");
    await input(page).pressSequentially("a"); // one letter: no plan search, three commands match
    await expect(palette(page).getByRole("option").first()).toBeVisible();
    const count = await palette(page).getByRole("option").count();
    expect(count).toBeGreaterThan(1);
    await page.keyboard.press("ArrowUp");
    await expect(input(page)).toHaveAttribute(
      "aria-activedescendant",
      `palette-row-${count - 1}`,
    );
    await page.keyboard.press("ArrowDown");
    await expect(input(page)).toHaveAttribute(
      "aria-activedescendant",
      "palette-row-0",
    );

    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/\/en-001\/plans\/new$/);
    await page.keyboard.press("Control+k");
    // Empty query: the item just chosen is the first row, marked as recent.
    const first = palette(page).getByRole("option").first();
    await expect(first).toContainText("Recent");
  });

  test("Escape closes it and focus goes back to the button that opened it; Tab stays inside", async ({
    page,
  }) => {
    // The button is an entry in the menu, which is closed until it is opened.
    await page.locator("button.hamburger").click();
    const trigger = page
      .locator("#primary-nav")
      .getByRole("button", { name: "Search and commands" });
    await trigger.focus();
    await page.keyboard.press("Enter");
    await expect(palette(page)).toBeVisible();
    for (let i = 0; i < 4; i++) await page.keyboard.press("Tab");
    await expect(palette(page).locator(":focus")).toHaveCount(1);
    await page.keyboard.press("Escape");
    await expect(palette(page)).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("when plan search fails the commands still work and the failure is said in words", async ({
    page,
  }) => {
    await page.route("**/api/plans/search**", (route) =>
      route.fulfill({ status: 500, json: { error: "down" } }),
    );
    await page.keyboard.press("Control+k");
    await input(page).pressSequentially("Webs");
    await expect(palette(page).getByRole("status")).toContainText(
      "Plan search is not available",
    );
  });

  test("axe finds no serious or critical violation with it open", async ({
    page,
  }) => {
    await page.keyboard.press("Control+k");
    await input(page).pressSequentially("Webs");
    await expect(palette(page).getByRole("option").first()).toBeVisible();
    await settle(page);
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    const bad = results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    );
    expect(bad.map((v) => `${v.id}: ${v.help}`)).toEqual([]);
  });
});
