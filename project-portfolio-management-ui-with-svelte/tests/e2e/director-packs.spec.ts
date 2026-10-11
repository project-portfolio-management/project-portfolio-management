// DP-11: the director pack pages, over stubbed `/api/director-packs`. Axe runs in English, Welsh and a
// locale with no page catalogue of its own (it falls back to English), and a reading is checked to carry a
// word and a symbol as well as a colour.

import { test, expect } from "@playwright/test";
import { settle } from "./fixtures";
import { BODY } from "../unit/fixtures/director-pack-body";

const PID = "22222222-2222-4222-8222-222222222222";
const META = { pid: PID, period: "2026-10", audience: "board", version: 2, status: "issued", cut_off: "2026-10-31T00:00:00Z", change_note: "Corrected the funding line", supersedes_pid: null, issued_at: "2026-11-01T09:00:00Z", issued_by: "x" };

test.describe("director packs (DP-11)", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/director-packs", (route) => route.fulfill({ json: { packs: [META, { ...META, pid: "33333333-3333-4333-8333-333333333333", period: "2026-09", version: 1, change_note: null }] } }));
    await page.route(`**/api/director-packs/${PID}`, (route) => route.fulfill({ json: { pack: META, body: BODY } }));
  });

  test("lists the packs and opens one", async ({ page }) => {
    await page.goto("/en-001/director-packs", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { name: "Director packs" })).toBeVisible();
    await expect(page.locator("tbody tr")).toHaveCount(2);
    await page.getByRole("link", { name: /Open the pack for 2026-10, board, version 2/ }).click();
    await expect(page.getByRole("heading", { level: 1 })).toContainText("Director pack 2026-10 — board");
  });

  test("shows every section with readings as words and symbols", async ({ page }) => {
    await page.goto(`/en-001/director-packs/${PID}`, { waitUntil: "networkidle" });
    for (const h of ["Data quality", "What changed", "Portfolio map", "Exceptions", "Gate decisions", "Funding", "Buffer fever chart", "Earned value", "Benefits", "Capacity", "Milestone sign-off", "Issues", "Change requests"]) {
      await expect(page.getByRole("heading", { level: 2, name: h, exact: true })).toBeVisible();
    }
    await expect(page.getByText("Change note: Corrected the funding line")).toBeVisible();
    const care = page.getByRole("row", { name: /Care.*amber\/red \(self-assessed, stale\)/ });
    await expect(care.first()).toContainText("▲");
    await expect(page.getByRole("row", { name: /Engineering.*shortfall/ })).toContainText("▲");
    await expect(page.getByRole("row", { name: /Engineering.*no supply recorded/ })).toContainText("–");
    await expect(page.getByRole("row", { name: /Cutover.*delivered, not met.*Acme Council/ })).toContainText("▲");
    await expect(page.getByRole("row", { name: /Training.*met/ })).toContainText("●");
    await expect(page.getByText("no funding source").first()).toBeVisible();
    await expect(page.getByRole("link", { name: "Download as PDF" })).toHaveAttribute("href", new RegExp(`/api/proxy/api/director-packs/${PID}/pdf$`));
    await expect(page.locator("table caption").first()).toBeVisible();
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto(`/cy-001/director-packs/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 2, name: "Map y portffolio" })).toBeVisible();
    await expect(page.getByRole("row", { name: /Engineering.*diffyg/ })).toBeVisible();
    await expect(page.getByText("Portfolio map")).toHaveCount(0);
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/director-packs/${PID}`, { waitUntil: "networkidle" });
      await settle(page);
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id)).toEqual([]);
    });
  }
});
