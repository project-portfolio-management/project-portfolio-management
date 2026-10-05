// The CEO dashboard on an iPad (9th gen): 1080 × 810 CSS pixels (2160 × 1620 at
// 2×), landscape and portrait. The page must show every tile with NO scrolling
// of the page, in both orientations, with and without long content, and a
// failing tile must not blank the others.

import { test, expect, type Page } from "@playwright/test";

const LONG = "A very long name that goes on and on ".repeat(6);

async function stub(page: Page, opts: { long?: boolean; healthFails?: boolean } = {}) {
  const name = (n: string) => (opts.long ? `${n} ${LONG}` : n);
  await page.route("**/api/executive/health**", (r) =>
    opts.healthFails
      ? r.fulfill({ status: 500, json: { error: "boom" } })
      : r.fulfill({
          json: {
            as_of: "2026-10-05",
            derivation: "d",
            portfolios: [
              {
                portfolio: { pid: "p1", name: name("Alpha") },
                status: "red",
                rag: { red: 2, amber: 1, green: 4 },
                members: 7,
                overdue_milestones: 3,
                escalated_risks: 2,
                open_risk_exposure: 5,
                overrun_currencies: ["GBP"],
                days_since_last_update: 1,
              },
            ],
          },
        }),
  );
  await page.route("**/api/milestones**", (r) =>
    r.fulfill({
      json: {
        as_of: "2026-10-05",
        due_soon_days: 14,
        at_risk_days: 7,
        summary: { by_status: {}, by_schedule_state: {} },
        note: "",
        milestones: Array.from({ length: 9 }, (_, i) => ({
          pid: `m${i}`,
          name: name(`Milestone ${i}`),
          due: `2026-11-0${i + 1}`,
          status: i === 0 ? "late" : "upcoming",
          achieved: false,
        })),
      },
    }),
  );
  await page.route("**/api/kpis**", (r) =>
    r.fulfill({
      json: {
        as_of: "2026-10-05",
        note: "",
        summary: { green: 0, amber: 2, red: 4, no_data: 0 },
        kpis: Array.from({ length: 8 }, (_, i) => ({
          kpi: { pid: `k${i}`, name: name(`KPI ${i}`) },
          status: i % 2 ? "amber" : "red",
        })),
      },
    }),
  );
  await page.route("**/api/executive/decisions**", (r) =>
    r.fulfill({
      json: {
        as_of: "2026-10-05",
        total: 6,
        decisions: Array.from({ length: 6 }, (_, i) => ({
          at: `2026-10-0${i + 1}T00:00:00Z`,
          kind: "gate_review",
          decision: name(`Decision ${i}`),
          subject: { pid: "p", name: name("Apollo") },
        })),
      },
    }),
  );
  await page.route("**/api/announcements**", (r) =>
    r.fulfill({
      json: {
        as_of: "2026-10-05T12:00:00Z",
        total: 3,
        truncated: false,
        announcements: Array.from({ length: 3 }, (_, i) => ({
          pid: `a${i}`,
          title: name(`Announcement ${i}`),
          body: LONG.repeat(10),
          severity: i === 0 ? "urgent" : "info",
          pinned: i === 0,
          publish_at: "2026-10-05T10:00:00Z",
          expires_at: null,
          author_ref: null,
          state: "live",
        })),
      },
    }),
  );
}

const SIZES = [
  { name: "landscape", width: 1080, height: 810 },
  { name: "portrait", width: 810, height: 1080 },
];

for (const size of SIZES) {
  for (const long of [false, true]) {
    test(`the dashboard fits an iPad in ${size.name} without scrolling${long ? " (long content)" : ""}`, async ({ page }) => {
      await page.setViewportSize({ width: size.width, height: size.height });
      await stub(page, { long });
      await page.goto("/en-001/ceo");
      await expect(page.getByTestId("tile-news").getByText("Announcement 0")).toBeVisible();
      await expect(page.getByTestId("tile-milestones").getByText("Milestone 0")).toBeVisible();
      await expect(page.getByTestId("tile-decisions").getByText("Decision 5")).toBeVisible();
      expect(await page.locator("[data-testid^=tile-]").count()).toBe(6);
      for (const id of ["health", "risk", "milestones", "kpis", "decisions", "news"]) {
        await expect(page.getByTestId(`tile-${id}`)).toBeVisible();
      }
      const m = await page.evaluate(() => ({
        sh: document.documentElement.scrollHeight,
        ih: window.innerHeight,
        sw: document.documentElement.scrollWidth,
        iw: window.innerWidth,
        tiles: [...document.querySelectorAll("[data-testid^=tile-]")].map((t) => {
          const r = t.getBoundingClientRect();
          return { bottom: r.bottom, right: r.right, over: t.scrollHeight > t.clientHeight + 1 };
        }),
      }));
      expect(m.sh, "page height").toBeLessThanOrEqual(m.ih);
      expect(m.sw, "page width").toBeLessThanOrEqual(m.iw);
      for (const t of m.tiles) {
        expect(t.bottom).toBeLessThanOrEqual(m.ih);
        expect(t.right).toBeLessThanOrEqual(m.iw);
      }
      // Each tile ends in a link to its full page, one tap away.
      await expect(page.getByRole("link", { name: "All announcements" })).toBeVisible();
      const box = await page.getByRole("link", { name: "All key measures" }).boundingBox();
      expect(box!.height).toBeGreaterThanOrEqual(44);
    });
  }
}

test("a tile that fails says so and the others still show", async ({ page }) => {
  await page.setViewportSize({ width: 1080, height: 810 });
  await stub(page, { healthFails: true });
  await page.goto("/en-001/ceo");
  await expect(page.getByTestId("tile-health").getByRole("alert")).toBeVisible();
  await expect(page.getByTestId("tile-kpis").getByText("KPI 0")).toBeVisible();
});

test("a phone gets a stacked page that may scroll but never sideways", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 800 });
  await stub(page);
  await page.goto("/en-001/ceo");
  await expect(page.getByTestId("tile-news")).toBeVisible();
  const sw = await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth);
  expect(sw).toBe(true);
});
