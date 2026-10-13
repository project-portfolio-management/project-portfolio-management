// EX-8: the start queue screen, over stubbed API answers.
import { test, expect } from "@playwright/test";

const QUEUE = {
  queue: [
    { position: 1, plan_pid: "p1", name: "Apollo", queued_on: "2026-10-01T09:00:00Z", refusals: [{ reason: "limit_reached", in_delivery: 3, limit: 3 }] },
    { position: 2, plan_pid: "p2", name: "Gemini", queued_on: "2026-10-02T09:00:00Z", refusals: [{ reason: "constraint_pool_over", pool: "Engineering", months: ["2026-11", "2026-12"] }] },
  ],
  note: "In the order the programmes asked to start.",
};

test.describe("start queue (EX-8)", () => {
  let withdrawn = "";
  test.beforeEach(async ({ page }) => {
    withdrawn = "";
    await page.route("**/api/start-queue", (r) => r.fulfill({ json: withdrawn ? { queue: QUEUE.queue.slice(1), note: "" } : QUEUE }));
    await page.route("**/api/plans/*/start-queue/withdraw", (r) => {
      withdrawn = r.request().url();
      return r.fulfill({ json: { withdrawn: "p1" } });
    });
  });

  test("lists the queue in order with the reasons, and withdraws a programme", async ({ page }) => {
    await page.goto("/en-001/start-queue", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Start queue" })).toBeVisible();
    await expect(page.getByRole("row", { name: /1.*Apollo.*All 3 places are taken: 3 programmes are in delivery/ })).toBeVisible();
    await expect(page.getByRole("row", { name: /2.*Gemini.*The constraint pool Engineering is short of supply in: 2026-11, 2026-12/ })).toBeVisible();
    await page.getByRole("button", { name: "Withdraw Apollo from the queue" }).click();
    await expect.poll(() => withdrawn).toContain("/api/plans/p1/start-queue/withdraw");
    await expect(page.getByRole("row", { name: /Apollo/ })).toHaveCount(0);
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/start-queue", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Ciw dechrau" })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/start-queue`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
