// NC-9 in a real browser: the portfolio balance page draws one bubble for each portfolio, states the
// imbalances in words, shows the same figures as a table, never merges currencies, and passes the axe
// audit. The service is stubbed.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
}

const ref = (pid: string, name: string) => ({ pid, name, kind: "Portfolio" });
const line = (currency: string, planned_minor: number) => ({
  currency,
  planned_minor,
  actual_minor: 0,
  remaining_minor: planned_minor,
  overrun: false,
  line_count: 1,
});
const benefit = (currency: string, target_minor: number) => ({
  currency,
  target_minor,
  realized_minor: 0,
  realization_ratio: null,
});

const VARIANCE = {
  as_of: "2026-10-29",
  note: "",
  by_collection: [],
  by_category: [],
  by_portfolio: [
    {
      portfolio: ref("a", "Digital"),
      variance: [line("GBP", 700_000), line("EUR", 5_000)],
    },
    { portfolio: ref("b", "Operations"), variance: [line("GBP", 200_000)] },
    { portfolio: ref("c", "Estates"), variance: [line("GBP", 100_000)] },
  ],
};

const BENEFITS = {
  as_of: "2026-10-29",
  note: "",
  portfolios: [
    {
      portfolio: ref("a", "Digital"),
      benefits: 1,
      non_financial: 0,
      statuses: {},
      financial: [benefit("GBP", 100_000)],
    },
    {
      portfolio: ref("b", "Operations"),
      benefits: 1,
      non_financial: 0,
      statuses: {},
      financial: [benefit("GBP", 800_000)],
    },
    {
      portfolio: ref("c", "Estates"),
      benefits: 1,
      non_financial: 0,
      statuses: {},
      financial: [benefit("GBP", 100_000)],
    },
  ],
};

const HEAT = {
  as_of: "2026-10-29",
  open_risks: 4,
  estate_open_exposure: 40,
  cells: {},
  top_risks: [],
  posture: [
    {
      portfolio: ref("a", "Digital"),
      open_exposure: 30,
      escalated: 0,
      materialized: 0,
    },
    {
      portfolio: ref("b", "Operations"),
      open_exposure: 10,
      escalated: 0,
      materialized: 0,
    },
  ],
  concentration: [],
  overdue_reviews: [],
};

async function stub(
  page: Page,
  over: { variance?: unknown; benefits?: unknown; heat?: unknown } = {},
) {
  await page.route("**/api/financials/variance", (r) =>
    r.fulfill({ json: over.variance ?? VARIANCE }),
  );
  await page.route("**/api/executive/benefits", (r) =>
    r.fulfill({ json: over.benefits ?? BENEFITS }),
  );
  await page.route("**/api/risk/heatmap", (r) =>
    r.fulfill({ json: over.heat ?? HEAT }),
  );
}

test.describe("portfolio balance", () => {
  test("draws a bubble for each portfolio, names the imbalances, and passes axe", async ({
    page,
  }) => {
    await stub(page);
    await page.goto("/en-001/balance", { waitUntil: "networkidle" });
    await page.getByTestId("balance-currency").selectOption("GBP");
    await expect(
      page.getByRole("heading", { level: 1, name: "Portfolio balance" }),
    ).toBeVisible();

    const chart = page.getByTestId("balance-chart");
    await expect(chart.getByRole("img")).toBeVisible();
    await expect(chart.getByTestId("balance-bubble")).toHaveCount(3);
    // Digital is flagged and drawn so (a mark and an outline, not colour alone); the others are not.
    await expect(
      chart.locator('[data-testid="balance-bubble"][data-flagged="yes"]'),
    ).toHaveCount(1);
    await expect(
      chart.locator('[data-testid="balance-bubble"][data-flagged="yes"]'),
    ).toContainText("Digital");

    const flags = page.getByTestId("balance-flags");
    await expect(flags).toContainText("Digital holds 70% of the planned cost.");
    await expect(flags).toContainText(
      "Digital holds 75% of the open risk exposure.",
    );
    await expect(flags).toContainText(
      "Digital has a share of planned cost well above its 10% share of target value.",
    );
    await expect(flags).not.toContainText("Operations");
    await audit(page);
  });

  test("shows the same figures as a table, in one currency, and switches currency", async ({
    page,
  }) => {
    await stub(page);
    await page.goto("/en-001/balance", { waitUntil: "networkidle" });
    // The currencies are listed in order and the first (EUR) is shown; choose pounds.
    await page.getByTestId("balance-currency").selectOption("GBP");
    const table = page.getByTestId("balance-table");
    await expect(
      table.getByRole("row", { name: /Digital.*70%.*10%.*75%/ }),
    ).toBeVisible();
    await expect(
      table.getByRole("row", { name: /Operations.*20%.*80%.*25%/ }),
    ).toBeVisible();
    await expect(table).not.toContainText("EUR");

    await page.getByTestId("balance-currency").selectOption("EUR");
    // Only Digital has euro cost: it holds all of it, and no portfolio has euro value.
    await expect(page.getByTestId("balance-flags")).toContainText(
      "Digital has planned cost but no target value recorded in EUR.",
    );
    await expect(page.getByTestId("balance-flags")).not.toContainText(
      "well above",
    );
    await expect(table.getByRole("row", { name: /Operations/ })).toBeVisible();
    await audit(page);
  });

  test("says when nothing is flagged, and when nothing is recorded", async ({
    page,
  }) => {
    await stub(page, {
      variance: { ...VARIANCE, by_portfolio: [] },
      benefits: { ...BENEFITS, portfolios: [] },
      heat: { ...HEAT, posture: [] },
    });
    await page.goto("/en-001/balance", { waitUntil: "networkidle" });
    await expect(page.getByTestId("balance-empty")).toBeVisible();
    await audit(page);
  });

  test("shows the catalogue's sentence, not the service's text, when a load fails", async ({
    page,
  }) => {
    await stub(page);
    await page.route("**/api/risk/heatmap", (r) =>
      r.fulfill({
        status: 500,
        json: { error: "internal detail: pg timeout" },
      }),
    );
    await page.goto("/en-001/balance", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toContainText(
      "Could not load the portfolio balance",
    );
    await expect(page.getByRole("alert")).not.toContainText("pg timeout");
  });

  test("renders in Welsh", async ({ page }) => {
    await stub(page);
    await page.goto("/cy-001/balance", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Cydbwysedd y portffolio" }),
    ).toBeVisible();
  });
});
