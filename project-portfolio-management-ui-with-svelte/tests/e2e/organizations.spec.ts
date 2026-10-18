// The partner organizations screen (EX-3), over stubbed API answers.
import { test, expect } from "@playwright/test";

test.describe("organizations", () => {
  let list = [
    { pid: "o1", name: "Acme Build", kind: "delivery", active: true, audience: "org:o1" },
    { pid: "o2", name: "Old Partner", kind: "partner", active: false, audience: "org:o2" },
  ];
  let posted = "";
  let changed = "";
  test.beforeEach(async ({ page }) => {
    posted = "";
    changed = "";
    await page.route("**/api/organizations", (r) => {
      if (r.request().method() === "POST") {
        posted = r.request().postData() ?? "";
        list = [...list, { pid: "o3", name: "New Co", kind: "supplier", active: true, audience: "org:o3" }];
        return r.fulfill({ json: {} });
      }
      return r.fulfill({ json: { organizations: list } });
    });
    await page.route("**/api/organizations/*", (r) => {
      changed = `${r.request().url().split("/").pop()} ${r.request().postData()}`;
      return r.fulfill({ json: {} });
    });
  });

  test("lists organizations, adds one, retires one", async ({ page }) => {
    await page.goto("/en-001/organizations", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Partner organizations" })).toBeVisible();
    await expect(page.getByRole("row", { name: /Acme Build.*delivery.*active.*org:o1/ })).toBeVisible();
    await expect(page.getByRole("row", { name: /Old Partner.*retired/ })).toBeVisible();
    await page.getByRole("textbox", { name: "Name" }).fill("New Co");
    await page.getByRole("combobox", { name: /Kind/ }).selectOption("supplier");
    await page.getByRole("button", { name: "Add", exact: true }).click();
    await expect.poll(() => posted).toBe('{"name":"New Co","kind":"supplier"}');
    await expect(page.getByRole("row", { name: /New Co.*supplier/ })).toBeVisible();
    await page.getByRole("button", { name: "Retire Acme Build" }).click();
    await expect.poll(() => changed).toBe('o1 {"active":false}');
  });

  test("hides and shows a column from the Columns control", async ({ page }) => {
    await page.goto("/en-001/organizations", { waitUntil: "networkidle" });
    const kind = page.getByRole("columnheader", { name: /^Kind/ });
    await expect(kind).toBeVisible();
    await page.getByRole("checkbox", { name: "Show Kind" }).uncheck();
    await expect(kind).toHaveCount(0);
    await page.getByRole("checkbox", { name: "Show Kind" }).check();
    await expect(kind).toBeVisible();
  });

  test("sorts by two columns with Shift+click", async ({ page }) => {
    await page.goto("/en-001/organizations", { waitUntil: "networkidle" });
    const kind = page.getByRole("columnheader", { name: /^Kind/ });
    const name = page.getByRole("columnheader", { name: /^Name/ });
    await kind.click();
    await name.click({ modifiers: ["Shift"] });
    await expect(kind).toHaveAttribute("data-sort-order", "1");
    await expect(name).toHaveAttribute("data-sort-order", "2");
  });

  test("exports the table as CSV, TSV and Excel", async ({ page }) => {
    list = [
      { pid: "o1", name: "Acme Build", kind: "delivery", active: true, audience: "org:o1" },
      { pid: "o2", name: "=HYPERLINK(\"x\")", kind: "partner", active: false, audience: "org:o2" },
    ];
    await page.goto("/en-001/organizations", { waitUntil: "networkidle" });
    const group = page.getByRole("group", { name: "Export" });
    await expect(group.getByRole("button", { name: "CSV" })).toBeVisible();
    for (const [name, ext] of [["CSV", "csv"], ["TSV", "tsv"], ["Excel", "xlsx"]] as const) {
      const download = page.waitForEvent("download");
      await group.getByRole("button", { name }).click();
      const file = await download;
      expect(file.suggestedFilename()).toMatch(new RegExp(`\\.${ext}$`));
      if (ext !== "xlsx") {
        const text = (await import("node:fs")).readFileSync((await file.path())!, "utf8");
        expect(text).toContain("Acme Build");
        // A name that starts with "=" must not reach a spreadsheet as a formula.
        expect(text).toContain("'=HYPERLINK");
      }
    }
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/organizations", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Sefydliadau partner" })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/organizations`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
