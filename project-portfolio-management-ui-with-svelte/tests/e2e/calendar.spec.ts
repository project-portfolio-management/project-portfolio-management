// The working calendar (/<locale>/me/calendar) on Lily's CalendarView, over stubbed API answers.
import { test, expect, type Page } from "@playwright/test";

async function stub(page: Page, saved: { body: string }) {
  await page.route("**/api/me/calendar", (r) => {
    if (r.request().method() === "PUT") {
      saved.body = r.request().postData() ?? "";
      return r.fulfill({
        json: { source: "person", ...JSON.parse(saved.body) },
      });
    }
    return r.fulfill({
      json: { source: "person", weekdays: "1111100", holidays: [] },
    });
  });
  await page.route("**/api/availability*", (r) => r.fulfill({ json: [] }));
}

test.describe("working calendar", () => {
  test("browses by month, four weeks and week, and marks a holiday", async ({
    page,
  }) => {
    const saved = { body: "" };
    await stub(page, saved);
    await page.goto("/en-001/me/calendar", { waitUntil: "networkidle" });
    const cal = page.getByTestId("calendar-month");
    await expect(cal.locator("td[data-date]")).toHaveCount(42);
    await cal.getByRole("button", { name: "Next Month" }).click();
    await cal
      .getByRole("button", { name: "Four weeks", exact: false })
      .or(cal.getByRole("button", { name: "4 weeks" }))
      .click();
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
      expect(
        results.violations.map((v) => `${v.id}: ${v.nodes.length}`),
      ).toEqual([]);
    });
  }
});

// The delivery calendar (/<locale>/calendar): milestones on CalendarView. Due dates are relative to the test run's
// month, because the calendar opens on today's month.
const DUE = (() => {
  const d = new Date();
  return new Date(Date.UTC(d.getFullYear(), d.getMonth(), 24))
    .toISOString()
    .slice(0, 10);
})();
const MILESTONES = {
  as_of: "2026-10-17T00:00:00Z",
  kinds: ["demo", "release"],
  milestones: [
    {
      pid: "m1",
      name: "Sprint demo",
      kind: "demo",
      due: DUE,
      done: false,
      item: { pid: "w1", name: "Platform rebuild", kind: "Project" },
    },
    {
      pid: "m2",
      name: "Go live",
      kind: "release",
      due: DUE,
      done: true,
      item: { pid: "w1", name: "Platform rebuild", kind: "Project" },
    },
  ],
};
async function stubMilestones(page: Page) {
  await page.route("**/api/engineering/milestone-calendar*", (r) =>
    r.fulfill({ json: MILESTONES }),
  );
}

test.describe("delivery calendar", () => {
  test("shows milestones in the month, marks a done one in words, and switches to the agenda with typed fields", async ({
    page,
  }) => {
    await stubMilestones(page);
    await page.goto("/en-001/calendar", { waitUntil: "networkidle" });
    const cal = page.getByTestId("milestone-calendar");
    await expect(cal.getByText("demo: Sprint demo")).toBeVisible();
    await expect(cal.getByText("release: Go live ✓ done")).toBeVisible();
    await cal.getByRole("button", { name: "Agenda" }).click();
    await expect(
      cal.getByText("Work item: Platform rebuild").first(),
    ).toBeVisible();
    await expect(cal.getByText("Status: done")).toBeVisible();
  });

  test("searches the milestones and offers TSV", async ({ page }) => {
    await stubMilestones(page);
    await page.goto("/en-001/calendar", { waitUntil: "networkidle" });
    const cal = page.getByTestId("milestone-calendar");
    await expect(cal.getByRole("button", { name: "TSV" })).toBeVisible();
    await expect(cal.getByRole("searchbox", { name: "Search" })).toBeVisible();
  });

  test("exports the milestones as CSV", async ({ page }) => {
    await stubMilestones(page);
    await page.goto("/en-001/calendar", { waitUntil: "networkidle" });
    const download = page.waitForEvent("download");
    await page
      .getByTestId("milestone-calendar")
      .getByRole("button", { name: "CSV" })
      .click();
    expect((await download).suggestedFilename()).toBe("milestones.csv");
  });

  for (const locale of ["en-001", "cy-001"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await stubMilestones(page);
      await page.goto(`/${locale}/calendar`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
        .analyze();
      expect(
        r.violations
          .filter((v) => v.impact === "serious" || v.impact === "critical")
          .map(
            (v) =>
              `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
          ),
      ).toEqual([]);
    });
  }
});

// WW-1.4 (UI): the week starts on the locale's day, Monday in the world English locale, and on the day a person
// chooses on the Display page.
test.describe("first day of the week", () => {
  test("is the locale's by default and the person's choice once made", async ({
    page,
  }) => {
    await stub(page, { body: "" });
    await page.goto("/en-001/me/calendar", { waitUntil: "networkidle" });
    const cal = page.getByTestId("calendar-month");
    await expect(cal.locator("thead th").first()).toContainText("Mon");
    await page.goto("/en-001/display", { waitUntil: "networkidle" });
    await page.getByLabel("Weeks start on").selectOption("sun");
    await page.goto("/en-001/me/calendar", { waitUntil: "networkidle" });
    await expect(
      page.getByTestId("calendar-month").locator("thead th").first(),
    ).toContainText("Sun");
  });
});
