// The prioritization queue: a keyboard grid whose columns can be moved (Alt+Arrow on a header).
import { test, expect } from "@playwright/test";

const QUEUE = {
  plans: [
    {
      pid: "p1",
      name: "Alpha",
      kind: "project",
      stage: null,
      score: 72.5,
      band: "high",
      coverage: 0.8,
      missing_evidence: [],
    },
    {
      pid: "p2",
      name: "Beta",
      kind: "project",
      stage: null,
      score: null,
      band: "unscored",
      coverage: 0,
      missing_evidence: ["roi"],
    },
  ],
  scored_of: 1,
  as_of: "2026-10-18",
};

test.describe("prioritization queue", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/prioritization*", (r) =>
      r.fulfill({ json: QUEUE }),
    );
  });

  test("lists the ranked plans and moves a column with Alt+Arrow", async ({
    page,
  }) => {
    await page.goto("/en-001/prioritization", { waitUntil: "networkidle" });
    await expect(page.getByRole("row", { name: /Alpha.*72\.5/ })).toBeVisible();
    const headers = () => page.getByRole("columnheader").allInnerTexts();
    const before = await headers();
    expect(before[0]).toMatch(/Plan/);
    await page.getByRole("columnheader").first().focus();
    await page.keyboard.press("Alt+ArrowRight");
    await expect.poll(async () => (await headers())[1]).toMatch(/Plan/);
    await expect.poll(async () => (await headers())[0]).toBe(before[1]);
  });

  test("still opens the explanation from its button", async ({ page }) => {
    await page.route("**/api/plans/p1/smart-score", (r) =>
      r.fulfill({
        json: {
          pid: "p1",
          name: "Alpha",
          as_of: "2026-10-18",
          smart_score: {
            score: 72.5,
            band: "high",
            coverage: 0.8,
            missing_evidence: [],
            components: [
              { name: "roi", weight: 0.5, raw: 0.9, contribution: 45 },
            ],
          },
        },
      }),
    );
    await page.goto("/en-001/prioritization", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Explain" }).first().click();
    await expect(
      page.getByRole("heading", { level: 2, name: "Alpha" }),
    ).toBeVisible();
  });
});
