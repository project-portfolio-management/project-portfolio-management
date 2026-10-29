// NC-10: the impact / effort matrix and Now / Next / Later over ideas and proposals, over stubbed answers.
import { test, expect } from "@playwright/test";

const IDEAS = [
  {
    pid: "i1",
    title: "Dark mode",
    pitch: null,
    tags: [],
    votes: 4,
    status: "open",
    converted_proposal_pid: null,
  },
  {
    pid: "i2",
    title: "Old idea",
    pitch: null,
    tags: [],
    votes: 0,
    status: "dismissed",
    converted_proposal_pid: null,
  },
];
const PROPOSALS = [
  {
    pid: "p1",
    title: "Data lake",
    summary: null,
    kind_target: "projects",
    sponsor_ref: null,
    strategic_rationale: null,
    requested_minor: null,
    currency: null,
    status: "draft",
    promoted_plan_pid: null,
  },
  {
    pid: "p2",
    title: "Rejected thing",
    summary: null,
    kind_target: "projects",
    sponsor_ref: null,
    strategic_rationale: null,
    requested_minor: null,
    currency: null,
    status: "rejected",
    promoted_plan_pid: null,
  },
];

test.describe("roadmap (NC-10)", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/ideas*", (r) => r.fulfill({ json: IDEAS }));
    await page.route("**/api/proposals*", (r) =>
      r.fulfill({ json: PROPOSALS }),
    );
  });

  test("rates items with the keyboard, and they move in the matrix and the horizons", async ({
    page,
  }) => {
    await page.goto("/en-001/roadmap", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: /Roadmap/ }),
    ).toBeVisible();
    // Dismissed ideas and rejected proposals are not offered.
    await expect(page.getByRole("row", { name: /Dark mode/ })).toBeVisible();
    await expect(page.getByRole("row", { name: /Data lake/ })).toBeVisible();
    await expect(page.getByText("Old idea")).toHaveCount(0);
    await expect(page.getByText("Rejected thing")).toHaveCount(0);

    const matrix = page.getByTestId("roadmap-matrix");
    await expect(matrix.getByText("Idea: Dark mode")).toHaveCount(0);
    await expect(page.getByTestId("horizon-unrated")).toContainText(
      "Idea: Dark mode",
    );

    // Choose with the keyboard only: focus the select, type the option's first letter.
    const impact = page.getByRole("combobox", { name: "Impact of Dark mode" });
    await impact.focus();
    await page.keyboard.press("h");
    await expect(impact).toHaveValue("high");
    // One rating is not enough to place it.
    await expect(matrix.getByText("Idea: Dark mode")).toHaveCount(0);
    const effort = page.getByRole("combobox", { name: "Effort of Dark mode" });
    await effort.focus();
    await page.keyboard.press("l");
    await expect(effort).toHaveValue("low");

    const cell = matrix
      .getByRole("row", { name: /High impact/ })
      .getByRole("cell")
      .first();
    await expect(cell).toContainText("Idea: Dark mode");
    await expect(page.getByTestId("horizon-now")).toContainText(
      "Idea: Dark mode",
    );
    await expect(page.getByTestId("horizon-unrated")).not.toContainText(
      "Dark mode",
    );
    await expect(page.getByTestId("horizon-unrated")).toContainText(
      "Proposal: Data lake",
    );

    // A balanced proposal is Next; a costly low-value one is Later.
    await page
      .getByRole("combobox", { name: "Impact of Data lake" })
      .selectOption("medium");
    await page
      .getByRole("combobox", { name: "Effort of Data lake" })
      .selectOption("medium");
    await expect(page.getByTestId("horizon-next")).toContainText(
      "Proposal: Data lake",
    );
    await page
      .getByRole("combobox", { name: "Effort of Data lake" })
      .selectOption("high");
    await expect(page.getByTestId("horizon-later")).toContainText(
      "Proposal: Data lake",
    );
  });

  test("remembers the ratings in this browser after a reload", async ({
    page,
  }) => {
    await page.goto("/en-001/roadmap", { waitUntil: "networkidle" });
    await page
      .getByRole("combobox", { name: "Impact of Dark mode" })
      .selectOption("high");
    await page
      .getByRole("combobox", { name: "Effort of Dark mode" })
      .selectOption("low");
    await expect(page.getByTestId("horizon-now")).toContainText("Dark mode");
    await page.reload({ waitUntil: "networkidle" });
    await expect(page.getByTestId("horizon-now")).toContainText(
      "Idea: Dark mode",
    );
    await expect(
      page.getByRole("combobox", { name: "Impact of Dark mode" }),
    ).toHaveValue("high");
  });

  test("shows a fixed sentence, not service text, when loading fails", async ({
    page,
  }) => {
    await page.route("**/api/ideas*", (r) =>
      r.fulfill({
        status: 500,
        json: { error: "internal boom", description: "secret detail" },
      }),
    );
    await page.goto("/en-001/roadmap", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
    await expect(page.getByText(/boom|secret detail/)).toHaveCount(0);
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/roadmap", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", {
        level: 2,
        name: "Matrics effaith ac ymdrech",
      }),
    ).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001"]) {
    test(`passes axe in ${locale}, with ratings set`, async ({ page }) => {
      await page.goto(`/${locale}/roadmap`, { waitUntil: "networkidle" });
      const selects = page.locator('select[aria-label*="Dark mode"]');
      await selects.first().selectOption("high");
      await selects.nth(1).selectOption("low");
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
        .analyze();
      expect(
        r.violations.map(
          (v) =>
            `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
        ),
      ).toEqual([]);
    });
  }
});
