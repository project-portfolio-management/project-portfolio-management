// The milestone sign-off screen (EX-4), over stubbed API answers.
import { test, expect } from "@playwright/test";

const DATA = {
  totals: { requiring_signoff: 2, delivered: 2, met: 1, delivered_not_met: 1, met_share_of_delivered_bp: 5000 },
  milestones: [
    { milestone_pid: "m1", plan_pid: "p1", name: "Go-live", due: "2026-11-01T00:00:00Z", signoff: { state: "delivered_not_met", delivered_on: "2026-10-30", met_on: null, outstanding: ["Acme Build"] } },
    { milestone_pid: "m2", plan_pid: "p1", name: "Handover", due: "2026-12-01T00:00:00Z", signoff: { state: "met", delivered_on: "2026-11-30", met_on: "2026-12-02", outstanding: [] } },
  ],
  note: "x",
};

test.describe("milestone sign-off", () => {
  let posted = "";
  test.beforeEach(async ({ page }) => {
    posted = "";
    await page.route("**/api/milestone-signoffs", (r) => r.fulfill({ json: DATA }));
    await page.route("**/api/organizations", (r) => r.fulfill({ json: { organizations: [{ pid: "o1", name: "Acme Build", kind: "partner", active: true, audience: "org:o1" }] } }));
    await page.route("**/api/plans/p1/milestones/m1/signoffs", (r) => { posted = r.request().postData() ?? ""; return r.fulfill({ json: {} }); });
  });

  test("shows delivered and met apart, the gap, and records a sign-off", async ({ page }) => {
    await page.goto("/en-001/milestone-signoffs", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Milestone sign-off" })).toBeVisible();
    await expect(page.getByText("Needing sign-off: 2. Delivered: 2. Met: 1. Delivered but not met: 1. Met, as a share of those delivered: 50%.")).toBeVisible();
    const gap = page.getByRole("row", { name: /Go-live.*delivered, not met.*Acme Build/ });
    await expect(gap).toContainText("▲");
    await expect(page.getByRole("row", { name: /Handover.*met.*2026-12-02/ })).toContainText("●");
    await page.getByRole("button", { name: "Sign Go-live as the chosen organization" }).click();
    await expect.poll(() => posted).toBe('{"organization_pid":"o1","action":"sign"}');
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/milestone-signoffs", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Llofnodi cerrig milltir" })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/milestone-signoffs`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
