// IR-2: the issues register page, over a stubbed `/api/issues`. Rows render in the order the
// service sends (most serious first), codes appear as words in the reader's language, overdue is
// said in words, and the filters go to the service as query parameters.

import { test, expect } from "@playwright/test";
import { settle } from "./fixtures";

const ISSUE = {
  plan_pid: "11111111-1111-4111-8111-111111111111",
  description: null,
  priority: null,
  raised_by: null,
  resolution: null,
  resolved_on: null,
};
const REGISTER = {
  issues: [
    { ...ISSUE, pid: "a", title: "Build server is down", kind: "blocker", severity: "critical", status: "open", owner_ref: "person:ada", raised_on: "2026-09-10", due_date: "2026-09-12", age_calendar_days: 30, overdue: true },
    { ...ISSUE, pid: "b", title: "Typo in the report", kind: "bug", severity: "low", status: "in_progress", owner_ref: null, raised_on: "2026-10-01", due_date: null, age_calendar_days: 9, overdue: false },
  ],
  matching: 2,
  truncated: false,
  summary: {
    by_severity: { low: 1, med: 0, high: 0, critical: 1 },
    by_status: { open: 1, in_progress: 1, resolved: 0, closed: 0 },
    overdue: 1,
    oldest_open: null,
  },
  as_of: "2026-10-10",
  note: "",
};

test.describe("issues register (IR-2)", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/issues**", (route) => route.fulfill({ json: REGISTER }));
  });

  test("lists the issues in words, most serious first, and says what is overdue", async ({ page }) => {
    await page.goto("/en-001/issues", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { name: "Issues register" })).toBeVisible();
    const rows = page.locator("tbody tr");
    await expect(rows).toHaveCount(2);
    await expect(rows.nth(0)).toContainText("Build server is down");
    await expect(rows.nth(0)).toContainText("critical");
    await expect(rows.nth(0)).toContainText("Overdue");
    await expect(rows.nth(1)).toContainText("in progress");
    await expect(rows.nth(1)).toContainText("No owner");
  });

  test("the same page reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/issues", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { name: "Cofrestr materion" })).toBeVisible();
    await expect(page.locator("tbody tr").nth(0)).toContainText("critigol");
    await expect(page.locator("tbody tr").nth(1)).toContainText("ar y gweill");
  });

  test("a filter is sent to the service", async ({ page }) => {
    await page.goto("/en-001/issues", { waitUntil: "networkidle" });
    const request = page.waitForRequest((r) => r.url().includes("/api/issues") && r.url().includes("severity=critical"));
    await page.locator(".filter select").nth(1).selectOption("critical");
    expect((await request).url()).toContain("severity=critical");
  });

  test("passes the accessibility audit", async ({ page }) => {
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    await page.goto("/en-001/issues", { waitUntil: "networkidle" });
    await expect(page.locator("h1").first()).toBeVisible();
    await settle(page);
    const r = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id)).toEqual([]);
  });
});
