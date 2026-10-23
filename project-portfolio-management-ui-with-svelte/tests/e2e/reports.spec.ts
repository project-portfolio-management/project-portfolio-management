// Saved reports (CQ-24): list, save a definition, run it into a table, and the CSV link.
import { test, expect } from "@playwright/test";

const REPORT = { pid: "r1", name: "Late projects", collection: "projects", filters: {}, fields: ["name", "status"] };
const RUN = { name: "Late projects", collection: "projects", rows: 2, data: [{ name: "Website replatform", status: "Active" }, { name: "Data centre exit", status: "On hold" }] };

test.describe("reports", () => {
  test("lists a saved report and runs it into a table", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) => r.fulfill({ json: RUN }));
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Reports" })).toBeVisible();
    await expect(page.getByText("Late projects")).toBeVisible();
    await expect(page.getByRole("link", { name: "CSV" })).toHaveAttribute("href", /\/api\/reports\/r1\/run\?format=csv$/);
    await page.getByRole("button", { name: "Run" }).click();
    await expect(page.getByText("2 rows")).toBeVisible();
    await expect(page.getByRole("row", { name: /Website replatform\s+Active/ })).toBeVisible();
    await expect(page.getByRole("row", { name: /Data centre exit\s+On hold/ })).toBeVisible();
  });

  test("saves a definition with the filter and the field list", async ({ page }) => {
    let posted: Record<string, unknown> | null = null;
    await page.route("**/api/reports", async (r) => {
      if (r.request().method() === "POST") {
        posted = r.request().postDataJSON();
        return r.fulfill({ status: 201, json: { pid: "r2" } });
      }
      return r.fulfill({ json: [] });
    });
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByPlaceholder("Report name").fill("Replatform");
    await page.getByPlaceholder("name contains…").fill("replatform");
    await page.getByPlaceholder("fields (comma separated)").fill("name, status ,");
    await page.getByRole("button", { name: "Save" }).click();
    await expect.poll(() => posted).toMatchObject({ name: "Replatform", collection: "projects", filters: { name_like: "replatform" }, fields: ["name", "status"] });
  });

  test("says so when a report cannot be run", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) => r.fulfill({ status: 500, json: { error: "boom" } }));
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Run" }).click();
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with a report run and shown", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) => r.fulfill({ json: RUN }));
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Run" }).click();
    await expect(page.getByRole("row", { name: /Website replatform/ })).toBeVisible();
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
    expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
  });
});
