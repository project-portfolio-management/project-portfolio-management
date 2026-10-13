// The header's picker bar: link, search, theme, language, text size and share. Each button opens its
// dropdown under the button, inside the window and not jammed at the far left, without moving the page.
import { test, expect, type Page } from "@playwright/test";

const PICKERS = [
  { name: "link", button: ".link-picker-button", popup: ".link-picker-list" },
  { name: "search", button: ".search-picker-button", popup: ".search-picker-panel" },
  { name: "theme", button: ".theme-picker-button", popup: ".theme-picker-list" },
  { name: "locale", button: ".locale-picker-button", popup: ".locale-picker-list" },
  { name: "text size", button: ".text-size-picker-button", popup: ".text-size-picker-list" },
  { name: "share", button: ".share-picker-button", popup: ".share-picker-list" },
];

async function geometry(page: Page, sel: string) {
  return page.locator(sel).first().boundingBox();
}

for (const viewport of [{ width: 1280, height: 800 }, { width: 390, height: 800 }]) {
  test.describe(`picker bar at ${viewport.width}px`, () => {
    test.use({ viewport });
    for (const p of PICKERS) {
      test(`${p.name} picker: dropdown under its button, inside the window, not at the far left, page does not move`, async ({ page }) => {
        await page.goto("/en-001/plans", { waitUntil: "networkidle" });
        const main = page.locator("main").first();
        const before = await main.boundingBox();
        const button = page.locator(p.button).first();
        await expect(button).toBeVisible();
        await button.click();
        const popup = page.locator(p.popup).first();
        await expect(popup).toBeVisible();
        const b = (await geometry(page, p.button))!;
        const d = (await geometry(page, p.popup))!;
        expect(d.y, "opens below the button").toBeGreaterThanOrEqual(b.y + b.height - 2);
        expect(d.y, "close under the button, not thrown to the bottom of the window").toBeLessThanOrEqual(b.y + b.height + 24);
        expect(d.y + Math.min(d.height, 40), "visible in the window").toBeLessThanOrEqual(viewport.height);
        // Under the button: the popup spans some of the button's width (it may flip to the button's end edge).
        expect(d.x <= b.x + b.width && d.x + d.width >= b.x, "sits under its own button").toBe(true);
        expect(d.x, "inside the window on the left").toBeGreaterThanOrEqual(0);
        expect(d.x + d.width, "inside the window on the right").toBeLessThanOrEqual(viewport.width + 1);
        if (viewport.width > 600) expect(d.x, "not at the far left").toBeGreaterThan(viewport.width / 4);
        const after = await main.boundingBox();
        expect(after?.y, "the page content does not shift").toBeCloseTo(before!.y, 0);
      });
    }
  });
}

test("the link picker lists the pages, marks the current one and navigates in the app", async ({ page }) => {
  await page.goto("/en-001/start-queue", { waitUntil: "networkidle" });
  await page.locator(".link-picker-button").click();
  const list = page.locator(".link-picker-list");
  await expect(list.getByRole("link", { name: "Start queue" })).toHaveAttribute("aria-current", "page");
  await list.getByRole("link", { name: "Organizations" }).click();
  await expect(page).toHaveURL(/\/en-001\/organizations$/);
});

test("the search picker sends the terms to /search and shows the matching plans", async ({ page }) => {
  await page.route("**/api/plans/search?*", (r) => r.fulfill({ json: [{ pid: "p1", name: "Signalling renewal" }] }));
  await page.goto("/en-001/plans", { waitUntil: "networkidle" });
  await page.locator(".search-picker-button").click();
  await page.locator(".search-picker-input").fill("signal ling");
  await page.keyboard.press("Enter");
  await expect(page).toHaveURL(/\/en-001\/search\?signal(%20|\+)ling$/);
  await expect(page.getByRole("heading", { level: 1, name: "Search plans" })).toBeVisible();
  await expect(page.getByText("1 plans match “signal ling”.")).toBeVisible();
  await expect(page.getByRole("link", { name: "Signalling renewal" })).toHaveAttribute("href", "/en-001/plans/p1");
});

test("the pickers read in Welsh", async ({ page }) => {
  await page.goto("/cy-001/plans", { waitUntil: "networkidle" });
  await expect(page.getByRole("button", { name: "Tudalennau" })).toBeVisible();
  await expect(page.getByRole("button", { name: "Chwilio cynlluniau" })).toBeVisible();
});
