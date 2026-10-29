// IR-4: the change requests page, over a stubbed `/api/change-requests`.

import { test, expect } from "@playwright/test";
import { settle } from "./fixtures";

const CR = {
  plan_pid: "11111111-1111-4111-8111-111111111111",
  description: "d",
  reason: "r",
  requested_by: "alice",
  decided_by: null,
  decided_on: null,
  decision_reason: null,
};
const LIST = {
  change_requests: [
    {
      ...CR,
      pid: "a",
      title: "Move go-live",
      kind: "schedule",
      status: "under_review",
      impact: {
        schedule_business_days: 10,
        cost_minor: 150000,
        cost_currency: "GBP",
        summary: null,
        has_forecast_edits: true,
      },
      requested_on: "2026-09-20",
      age_calendar_days: 20,
      approver_ref: "person:bob",
    },
    {
      ...CR,
      pid: "b",
      title: "Drop the report",
      kind: "scope",
      status: "draft",
      impact: {
        schedule_business_days: null,
        cost_minor: null,
        cost_currency: null,
        summary: null,
        has_forecast_edits: false,
      },
      requested_on: "2026-10-08",
      age_calendar_days: 2,
      approver_ref: null,
    },
  ],
  summary: {
    by_status: { draft: 1, under_review: 1 },
    awaiting_decision: 1,
    oldest_open: null,
  },
  as_of: "2026-10-10",
};

test.describe("change requests page (IR-4)", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/change-requests**", (route) =>
      route.fulfill({ json: LIST }),
    );
  });

  test("lists the requests with their state in words and the impact as stated", async ({
    page,
  }) => {
    await page.goto("/en-001/change-requests", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { name: "Change requests" }),
    ).toBeVisible();
    const rows = page.locator("tbody tr");
    await expect(rows).toHaveCount(2);
    await expect(rows.nth(0)).toContainText("Move go-live");
    await expect(rows.nth(0)).toContainText("under review");
    await expect(rows.nth(0)).toContainText("10 business days");
    await expect(rows.nth(0)).toContainText("£1,500.00");
    await expect(rows.nth(1)).toContainText("No impact stated");
    await expect(rows.nth(1)).toContainText("Not yet named");
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/change-requests", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { name: "Ceisiadau newid" }),
    ).toBeVisible();
    await expect(page.locator("tbody tr").nth(0)).toContainText(
      "dan adolygiad",
    );
  });

  test("a filter is sent to the service, and the page passes axe", async ({
    page,
  }) => {
    await page.goto("/en-001/change-requests", { waitUntil: "networkidle" });
    const request = page.waitForRequest(
      (r) =>
        r.url().includes("/api/change-requests") &&
        r.url().includes("status=approved"),
    );
    await page.locator(".filter select").selectOption("approved");
    expect((await request).url()).toContain("status=approved");
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    await settle(page);
    const r = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(
      r.violations
        .filter((v) => v.impact === "serious" || v.impact === "critical")
        .map((v) => v.id),
    ).toEqual([]);
  });
});
