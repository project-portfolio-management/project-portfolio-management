// The key people screen (EX-11), over stubbed API answers.
import { test, expect } from "@playwright/test";

test.describe("key people", () => {
  let data = {
    entries: [
      { pid: "k1", subject_kind: "person", subject: "Ana Lopez", successor_ref: null, readiness: "none", skill_pool_pid: null, note: "Only person who knows the interface" },
      { pid: "k2", subject_kind: "role", subject: "Release manager", successor_ref: "Bo Chen", readiness: "ready", skill_pool_pid: null, note: null },
    ],
    contingent: ["Cy Dunn"],
  };
  let put = "";
  let removed = "";
  let contingent = "";
  test.beforeEach(async ({ page }) => {
    put = removed = contingent = "";
    await page.route("**/api/key-persons", (r) => {
      if (r.request().method() === "PUT") { put = r.request().postData() ?? ""; return r.fulfill({ json: {} }); }
      return r.fulfill({ json: data });
    });
    await page.route("**/api/key-persons/contingent", (r) => { contingent = r.request().postData() ?? ""; return r.fulfill({ json: {} }); });
    await page.route("**/api/key-persons/k1", (r) => { removed = r.request().method(); data = { ...data, entries: data.entries.slice(1) }; return r.fulfill({ json: {} }); });
  });

  test("lists single points of failure, records one, removes one, and marks contingent workers", async ({ page }) => {
    await page.goto("/en-001/key-persons", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: /Key people/ })).toBeVisible();
    await expect(page.getByRole("row", { name: /Ana Lopez.*no successor named.*not ready/ })).toContainText("✖");
    await expect(page.getByRole("row", { name: /Release manager.*Bo Chen.*ready/ })).toBeVisible();
    await page.getByLabel("Name or role").fill("Dee Park");
    await page.getByLabel("Successor readiness").selectOption("partly");
    await page.getByRole("button", { name: "Record", exact: true }).click();
    await expect.poll(() => put).toBe('{"subject_kind":"person","subject":"Dee Park","readiness":"partly"}');
    await page.getByLabel("Person reference").fill("Eve Fox");
    await page.getByRole("button", { name: "Mark", exact: true }).click();
    await expect.poll(() => contingent).toBe('{"person_ref":"Eve Fox","contingent":true}');
    await page.getByRole("button", { name: "Remove Ana Lopez" }).click();
    await expect.poll(() => removed).toBe("DELETE");
  });

  test("says so when the caller is not a resource manager", async ({ page }) => {
    await page.route("**/api/key-persons", (r) => r.fulfill({ status: 404, json: { error: "not_found" } }));
    await page.goto("/en-001/key-persons", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/key-persons", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: /Pobl allweddol/ })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/key-persons`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
