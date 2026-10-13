// The email deliveries screen (administrators), over stubbed API answers.
import { test, expect } from "@playwright/test";

const DATA = {
  deliveries: [
    { pid: "d1", kind: "sink", subject_ref: "s1", recipient: "ana@example.test", subject: "Weekly pack", status: "sent", attempts: 1, last_error: null, sender: "smtp", next_attempt_at: null, sent_at: "2026-10-12T09:00:00Z" },
    { pid: "d2", kind: "sink", subject_ref: "s1", recipient: "bo@example.test", subject: "Weekly pack", status: "retry", attempts: 2, last_error: "connection refused", sender: "smtp", next_attempt_at: "2026-10-12T09:30:00Z", sent_at: null },
  ],
  counts: { retry: 1, sent: 1, failed: 0 },
};

test.describe("email deliveries", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/email-deliveries**", (r) => r.fulfill({ json: DATA }));
  });

  test("lists the deliveries with status as a word and a symbol", async ({ page }) => {
    await page.goto("/en-001/email-deliveries", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Email deliveries" })).toBeVisible();
    await expect(page.getByText("Sent: 1. Waiting to retry: 1. Failed for good: 0.")).toBeVisible();
    await expect(page.getByRole("row", { name: /ana@example.test.*sent/ })).toContainText("●");
    await expect(page.getByRole("row", { name: /bo@example.test.*will retry.*connection refused/ })).toBeVisible();
  });

  test("says so when the caller is not an administrator", async ({ page }) => {
    await page.route("**/api/email-deliveries**", (r) => r.fulfill({ status: 404, json: { error: "not_found" } }));
    await page.goto("/en-001/email-deliveries", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/email-deliveries", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Negeseuon e-bost a ddanfonwyd" })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/email-deliveries`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
