// RE-3 and RE-5 (NC-1 slice 12 part l) in a real browser: `/roadmaps` lists the roadmaps due a review, every roadmap
// the reader may read with its state and counts, linking to its editor, and the product families. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const P1 = "aaaaaaaa-0000-4000-8000-0000000000e1";
const P2 = "aaaaaaaa-0000-4000-8000-0000000000e2";
const F1 = "aaaaaaaa-0000-4000-8000-0000000000f1";

async function stub(page: Page, empty = false) {
  await page.route("**/api/public-roadmaps/stale", (r) =>
    r.fulfill({
      json: {
        stale_after_days: 42,
        today: "2026-11-27",
        count: empty ? 0 : 1,
        roadmaps: empty
          ? []
          : [
              {
                plan_pid: P2,
                title: "Wallets",
                slug: "wallets",
                published: true,
                reviewed_on: "2026-09-01",
                state: "stale",
              },
            ],
      },
    }),
  );
  await page.route(/\/api\/public-roadmaps(\?.*)?$/, (r) =>
    r.fulfill({
      json: empty
        ? []
        : [
            {
              pid: "r1",
              plan_pid: P1,
              slug: "payments",
              title: "Payments",
              category_pid: null,
              published: false,
              reviewed_on: "2026-11-20",
              freshness: "fresh",
              items: { placed: 3, unplaced: 1 },
              family: { plan_pid: F1, name: "Money", colour: null },
            },
            {
              pid: "r2",
              plan_pid: P2,
              slug: "wallets",
              title: "Wallets",
              category_pid: null,
              published: true,
              reviewed_on: "2026-09-01",
              freshness: "stale",
              items: { placed: 0, unplaced: 0 },
              family: null,
            },
          ],
    }),
  );
  await page.route(/\/api\/roadmap-families(\?.*)?$/, (r) =>
    r.fulfill({
      json: empty
        ? []
        : [
            {
              plan_pid: F1,
              name: "Money",
              colour: null,
              contrast: null,
              translations: {},
              products: 2,
            },
          ],
    }),
  );
}

test("the roadmaps due a review, every roadmap with its state and counts, and the families", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/en-001/roadmaps");
  await settle(page);
  const due = page.getByTestId("due").getByRole("listitem");
  await expect(due).toHaveCount(1);
  await expect(due.nth(0)).toContainText("Stale: due a review");
  await expect(
    due.nth(0).getByRole("link", { name: "Wallets" }),
  ).toHaveAttribute("href", `/en-001/plans/${P2}/roadmap`);
  const all = page.getByTestId("all").getByRole("listitem");
  await expect(all).toHaveCount(2);
  await expect(all.nth(0)).toContainText("Not published");
  await expect(all.nth(0)).toContainText("items placed: 3; not placed: 1");
  await expect(all.nth(0)).toContainText("family: Money");
  await expect(
    all.nth(0).getByRole("link", { name: "Payments" }),
  ).toHaveAttribute("href", `/en-001/plans/${P1}/roadmap`);
  await expect(page.getByTestId("families")).toContainText(
    "Money · products: 2",
  );
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("says so when there is nothing; reads in Welsh", async ({ page }) => {
  await stub(page, true);
  await page.goto("/en-001/roadmaps");
  await settle(page);
  await expect(page.getByTestId("due-none")).toBeVisible();
  await expect(page.getByTestId("all-none")).toBeVisible();
  await expect(page.getByTestId("families-none")).toBeVisible();
  await page.goto("/cy-001/roadmaps");
  await expect(
    page.getByRole("heading", { level: 1, name: "Mapiau ffordd cynnyrch" }),
  ).toBeVisible();
});
