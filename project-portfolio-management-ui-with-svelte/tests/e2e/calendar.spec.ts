// The working calendar (/<locale>/me/calendar) on Lily's CalendarView, over stubbed API answers.
import { test, expect, type Page } from "@playwright/test";

async function stub(page: Page, saved: { body: string }) {
  await page.route("**/api/me/calendar", (r) => {
    if (r.request().method() === "PUT") {
      saved.body = r.request().postData() ?? "";
      return r.fulfill({ json: { source: "person", ...JSON.parse(saved.body) } });
    }
    return r.fulfill({ json: { source: "person", weekdays: "1111100", holidays: [] } });
  });
  await page.route("**/api/availability*", (r) => r.fulfill({ json: [] }));
}

test.describe("working calendar", () => {
  test("browses by month, four weeks and week, and marks a holiday", async ({ page }) => {
    const saved = { body: "" };
    await stub(page, saved);
    await page.goto("/en-001/me/calendar", { waitUntil: "networkidle" });
    const cal = page.getByTestId("calendar-month");
    await expect(cal.locator("td[data-date]")).toHaveCount(42);
    await cal.getByRole("button", { name: "Next Month" }).click();
    await cal.getByRole("button", { name: "Four weeks", exact: false }).or(cal.getByRole("button", { name: "4 weeks" })).click();
    await expect(cal.locator("td[data-date]")).toHaveCount(28);
    await cal.getByRole("button", { name: "Week", exact: true }).click();
    await expect(cal.locator("td[data-date]")).toHaveCount(7);
    await cal.getByRole("button", { name: "Month", exact: true }).click();
    const monday = cal.locator("td[data-date]:not([aria-disabled])").first();
    const date = await monday.getAttribute("data-date");
    await monday.click();
    await expect(monday).toContainText("✕");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    await expect.poll(() => saved.body).toContain(date!);
  });

  for (const locale of ["en-001", "cy"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await stub(page, { body: "" });
      await page.goto(`/${locale}/me/calendar`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const results = await new AxeBuilder({ page }).analyze();
      expect(results.violations.map((v) => `${v.id}: ${v.nodes.length}`)).toEqual([]);
    });
  }
});
