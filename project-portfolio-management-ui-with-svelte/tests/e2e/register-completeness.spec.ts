// EX-1 in a real browser (NC-1 slice 12 part d): `/register-completeness` lists each programme's register
// completeness, the incomplete first, every missing field named in the catalogue's words, and says whether "no
// register, no money" is on. The service's `note` (English) is never shown. Linked from `/director-packs` and
// `/exceptions`. Welsh reads. The page passes axe.

import { test, expect, type Page } from "./test";
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

const FIELDS = [
  "sro",
  "partners",
  "stage",
  "baseline",
  "target_date",
  "budget",
  "funding_source",
  "confidence",
  "top_risks",
  "dependencies",
];

const ANSWER = (enforced: boolean) => ({
  enforced,
  fields: FIELDS,
  note: "service text",
  programmes: [
    {
      plan_pid: "aaaaaaaa-0000-4000-8000-000000000001",
      name: "Apollo",
      complete: true,
      score_pct: 100,
      present: 9,
      applicable: 9,
      missing: [],
      not_applicable: ["partners"],
    },
    {
      plan_pid: "aaaaaaaa-0000-4000-8000-000000000002",
      name: "Gemini",
      complete: false,
      score_pct: 77.78,
      present: 7,
      applicable: 9,
      missing: ["sro", "top_risks"],
      not_applicable: ["funding_source"],
    },
    {
      plan_pid: "aaaaaaaa-0000-4000-8000-000000000003",
      name: "Mercury",
      complete: false,
      score_pct: 40,
      present: 4,
      applicable: 10,
      missing: [
        "baseline",
        "target_date",
        "budget",
        "confidence",
        "dependencies",
        "a_new_field",
      ],
      not_applicable: [],
    },
  ],
});

async function stub(page: Page, enforced = false, status = 200) {
  await page.route("**/api/register-completeness", (r) =>
    status === 200
      ? r.fulfill({ json: ANSWER(enforced) })
      : r.fulfill({
          status,
          json: { code: "internal", detail: "service text" },
        }),
  );
}

test("each programme's completeness, the incomplete first, every missing field named", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/en-001/register-completeness");
  await expect(
    page.getByRole("heading", { level: 1, name: "Register completeness" }),
  ).toBeVisible();
  // Said first: what is counted, and the rules for partners and funding.
  await expect(
    page.getByText(
      "The ten: a senior responsible owner (the programme's lead stands for one)",
      {
        exact: false,
      },
    ),
  ).toBeVisible();
  await expect(page.getByTestId("rc-switch")).toHaveText(
    "“No register, no money” is off: releasing a budget line or booking a firm allocation on an incomplete programme goes ahead, and the warning is kept in the audit trail.",
  );
  await expect(page.getByTestId("rc-count")).toHaveText(
    "1 of 3 programmes are complete.",
  );
  const rows = page.getByRole("row");
  await expect(rows.nth(1)).toContainText("Mercury");
  await expect(rows.nth(1)).toContainText("▲ incomplete");
  await expect(rows.nth(1)).toContainText("40%");
  await expect(rows.nth(1)).toContainText("4 of 10");
  await expect(rows.nth(1)).toContainText(
    "schedule baseline, target date, budget, delivery confidence, dependencies, another field",
  );
  await expect(rows.nth(2)).toContainText("Gemini");
  await expect(rows.nth(2)).toContainText(
    "senior responsible owner, top risks",
  );
  await expect(rows.nth(2)).toContainText("funding source");
  await expect(rows.nth(3)).toContainText("Apollo");
  await expect(rows.nth(3)).toContainText("✓ complete");
  await expect(rows.nth(3)).toContainText("none");
  await expect(page.getByRole("link", { name: "Gemini" })).toHaveAttribute(
    "href",
    "/en-001/plans/aaaaaaaa-0000-4000-8000-000000000002",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await expect(page.getByText("a_new_field")).toHaveCount(0);
  await audit(page);
});

test("says when no register, no money is on", async ({ page }) => {
  await stub(page, true);
  await page.goto("/en-001/register-completeness");
  await expect(page.getByTestId("rc-switch")).toHaveText(
    "“No register, no money” is on: releasing a budget line or booking a firm allocation on an incomplete programme is refused, naming what is missing.",
  );
});

test("a failed read says the catalogue's sentence; none says so", async ({
  page,
}) => {
  await stub(page, false, 500);
  await page.goto("/en-001/register-completeness");
  await expect(page.getByRole("alert")).toHaveText(
    "Could not load the register completeness.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await page.unrouteAll();
  await page.route("**/api/register-completeness", (r) =>
    r.fulfill({
      json: {
        enforced: false,
        fields: FIELDS,
        programmes: [],
        note: "service text",
      },
    }),
  );
  await page.reload();
  await expect(
    page.getByText("There is no programme you can read."),
  ).toBeVisible();
  await expect(page.getByTestId("rc-count")).toHaveCount(0);
});

test("the director packs and the exceptions link to it", async ({ page }) => {
  for (const from of ["/en-001/director-packs", "/en-001/exceptions"]) {
    await page.goto(from);
    await expect(
      page.getByRole("link", { name: "Register completeness" }),
    ).toHaveAttribute("href", "/en-001/register-completeness");
  }
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/register-completeness");
  await expect(
    page.getByRole("heading", { level: 1, name: "Cyflawnrwydd y gofrestr" }),
  ).toBeVisible();
  await expect(page.getByTestId("rc-count")).toHaveText(
    "Mae 1 o 3 rhaglen yn gyflawn.",
  );
  await expect(page.getByRole("row").nth(2)).toContainText(
    "uwch berchennog cyfrifol",
  );
});
