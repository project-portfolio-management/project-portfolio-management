// The funding ledger screen (EX-12), over stubbed API answers.
import { test, expect } from "@playwright/test";

const A = {
  pid: "a1",
  funding_source_pid: "f1",
  funding_source: "Treasury",
  amount_minor: 1000000,
  currency: "GBP",
  allocated_on: "2026-04-01",
  reference: "FY27",
  planned_on_lines_minor: 750000,
  untraced_minor: 250000,
  lines: 1,
};
const TRACE = {
  allocation: A,
  currency: "GBP",
  lines: [
    {
      line_pid: "l1",
      plan_pid: "p1",
      plan: "Apollo",
      description: "Build",
      category: "labour",
      planned_minor: 750000,
      actual_minor: 200000,
      gate: "G2",
      released: false,
    },
  ],
  money: {
    allocated_minor: 1000000,
    planned_on_lines_minor: 750000,
    spent_minor: 200000,
    untraced_minor: 250000,
    untraced_reason: "allocated but not yet planned on any budget line",
  },
  gates: { lines_with_a_gate: 1, released: 0 },
  milestones: { met: 1, total: 2, items: [] },
  benefits: {
    validated_by_currency: [{ currency: "GBP", validated_minor: 50000 }],
    planned_by_currency: [],
    benefits: 1,
  },
  note: "x",
};

test.describe("funding ledger", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/funding-allocations", (r) =>
      r.fulfill({ json: { allocations: [A] } }),
    );
    await page.route("**/api/funding-allocations/a1/trace", (r) =>
      r.fulfill({ json: TRACE }),
    );
  });

  test("lists allocations with untraced money and traces one", async ({
    page,
  }) => {
    await page.goto("/en-001/ledger", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Funding ledger" }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", {
        name: /Treasury.*FY27.*£10,000\.00.*£7,500\.00.*£2,500\.00/,
      }),
    ).toBeVisible();
    await page
      .getByRole("button", { name: "Trace Treasury 2026-04-01" })
      .click();
    await expect(
      page.getByText(
        "Allocated £10,000.00, planned on lines £7,500.00, spent £2,000.00, untraced £2,500.00 (GBP).",
      ),
    ).toBeVisible();
    await expect(
      page.getByText("allocated but not yet planned on any budget line"),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /Apollo.*Build.*G2: held/ }),
    ).toBeVisible();
    await expect(page.getByText("Milestones met: 1 of 2.")).toBeVisible();
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/ledger", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Cyfriflyfr cyllid" }),
    ).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/ledger`, { waitUntil: "networkidle" });
      await page
        .getByRole("button")
        .first()
        .click()
        .catch(() => {});
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
        .analyze();
      expect(
        r.violations
          .filter((v) => v.impact === "serious" || v.impact === "critical")
          .map(
            (v) =>
              `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
          ),
      ).toEqual([]);
    });
  }
});
