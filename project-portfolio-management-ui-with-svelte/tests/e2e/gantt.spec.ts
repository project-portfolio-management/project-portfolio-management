// The schedule Gantt (/<locale>/gantt) on Lily's GanttChart, over stubbed API answers.
import { test, expect, type Page } from "@playwright/test";

const SCHEDULE = {
  portfolio_pid: "p1",
  items: [
    { pid: "a", kind: "project", name: "Design", stage: null, start: "2026-10-05", end: "2026-10-09", on_critical_path: true },
    { pid: "b", kind: "project", name: "Build", stage: null, start: "2026-10-12", end: "2026-10-30", on_critical_path: true },
    { pid: "c", kind: "project", name: "Launch", stage: null, start: "2026-11-02", end: "2026-11-02", on_critical_path: false },
    { pid: "d", kind: "project", name: "Training", stage: null, start: null, end: null, on_critical_path: false },
  ],
  edges: [
    { pid: "e1", predecessor_pid: "a", successor_pid: "b", lag_days: 0 },
    { pid: "e2", predecessor_pid: "b", successor_pid: "c", lag_days: 0 },
  ],
  violations: [], critical_path: ["a", "b"], undated: ["d"],
};

async function stub(page: Page) {
  await page.route("**/api/plans/p1/schedule", (r) => r.fulfill({ json: SCHEDULE }));
  await page.route(/\/api\/plans(\?.*)?$/, (r) => r.fulfill({ json: [{ pid: "p1", name: "North Line", kind: "portfolio" }] }));
}

test.describe("schedule gantt", () => {
  test("draws bars as marked cells, says the critical path in words, lists the undated task with no bar", async ({ page }) => {
    await stub(page);
    await page.goto("/en-001/gantt", { waitUntil: "networkidle" });
    const chart = page.getByTestId("ppm-gantt");
    await expect(chart.getByRole("rowheader", { name: "Design (critical path)" })).toBeVisible();
    await expect(chart.getByRole("rowheader", { name: "Launch" })).toBeVisible();
    await expect(chart.locator("[data-unscheduled]")).toHaveCount(1);
    await expect(chart.locator("[data-unscheduled]")).toContainText("Training");
    await expect(chart.locator("[data-milestone]").first()).toBeVisible();
    await expect(chart.getByText("Depends on: Design (critical path)")).toBeAttached();
  });

  test("zooms the time scale and exports the schedule", async ({ page }) => {
    await stub(page);
    await page.goto("/en-001/gantt", { waitUntil: "networkidle" });
    const chart = page.getByTestId("ppm-gantt");
    const columns = () => chart.locator("thead th").count();
    await expect(chart.getByRole("button", { name: "Finer time scale" })).toBeVisible();
    const weeks = await columns();
    expect(weeks).toBeGreaterThan(1);
    await chart.getByRole("button", { name: "Finer time scale" }).click();
    await expect.poll(columns).toBeGreaterThan(weeks);
    await chart.getByRole("button", { name: "Coarser time scale" }).click();
    await chart.getByRole("button", { name: "Coarser time scale" }).click();
    await expect.poll(columns).toBeLessThan(weeks);
    const download = page.waitForEvent("download");
    await chart.getByRole("button", { name: "CSV" }).click();
    const file = await download;
    expect(file.suggestedFilename()).toBe("schedule.csv");
    const tsv = page.waitForEvent("download");
    await chart.getByRole("button", { name: "TSV" }).click();
    expect((await tsv).suggestedFilename()).toBe("schedule.tsv");
  });

  test("searches the tasks and draws dependency arrows", async ({ page }) => {
    await stub(page);
    await page.goto("/en-001/gantt", { waitUntil: "networkidle" });
    const chart = page.getByTestId("ppm-gantt");
    await expect(chart.locator("svg[aria-hidden='true']").first()).toBeAttached();
    await chart.getByRole("searchbox", { name: "Search" }).fill("launch");
    await expect(chart.getByRole("rowheader", { name: "Launch" })).toBeVisible();
    await expect(chart.getByRole("rowheader", { name: "Design (critical path)" })).toHaveCount(0);
  });

  for (const locale of ["en-001", "cy-001"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await stub(page);
      await page.goto(`/${locale}/gantt`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
