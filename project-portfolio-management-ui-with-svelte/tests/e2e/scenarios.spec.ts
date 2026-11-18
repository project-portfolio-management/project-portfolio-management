// NC-1 slice 12 part (g) (NC-7): choosing a portfolio on /scenarios, in a real browser over a stubbed service. The
// efficient frontier's rules said first and checked before anything is asked (the ten steps the service assumes not
// sent), drawn as a chart with a table twin, each point's choice plan by plan with why a plan was left out, and a point
// saved as a draft; a generated draft's name and cap checked first, its body and its choice; a committed scenario
// rolled back after a question (Cancel sends nothing), and its 409 in the catalogue's words. Every stub that would
// carry the service's English carries "service text", which never shows. Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const UNKNOWN = "99999999-9999-4999-8999-999999999999";
const RATIONALE_AT_1000 = [
  { pid: PID, included: true, score: 62.5, cost_minor: 80000 },
  {
    pid: "22222222-2222-4222-8222-222222222222",
    included: false,
    reason: "no_score",
    cost_minor: 0,
  },
  { pid: UNKNOWN, included: false, reason: "must_include_conflict" },
];

async function stub(
  page: Page,
  opts: { rollbackStatus?: number; generateStatus?: number } = {},
) {
  const scenarios = [
    {
      pid: "s1",
      name: "Roomy",
      description: null,
      status: "draft",
      budget_cap_minor: 100000,
      currency: "GBP",
      committed_at: null,
    },
    {
      pid: "s2",
      name: "Chosen",
      description: null,
      status: "committed",
      budget_cap_minor: 50000,
      currency: "GBP",
      committed_at: "2026-11-01T09:00:00Z",
    },
  ];
  const sent: Array<{ method: string; path: string; body?: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/scenarios/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET") {
      if (path === "/api/scenarios") return route.fulfill({ json: scenarios });
      if (path === "/api/scenarios/frontier") {
        sent.push({ method: "GET", path: `${path}${url.search}` });
        return route.fulfill({
          json: {
            currency: "GBP",
            value: "service text",
            points: [
              {
                budget_cap_minor: 0,
                value: 0,
                cost_minor: 0,
                included: 0,
                dominated: false,
                rationale: [],
              },
              {
                budget_cap_minor: 100000,
                value: 62.5,
                cost_minor: 80000,
                included: 1,
                dominated: false,
                rationale: RATIONALE_AT_1000,
              },
              {
                budget_cap_minor: 200000,
                value: 62.5,
                cost_minor: 80000,
                included: 1,
                dominated: true,
                rationale: RATIONALE_AT_1000,
              },
            ],
          },
        });
      }
      return route.fulfill({ json: {} });
    }
    const body = req.postDataJSON();
    sent.push({ method: req.method(), path, body });
    if (path === "/api/scenarios/generate") {
      if (opts.generateStatus)
        return route.fulfill({
          status: opts.generateStatus,
          json: { code: "unprocessable", detail: "service text" },
        });
      scenarios.push({
        pid: `g${scenarios.length}`,
        name: body.name,
        description: null,
        status: "draft",
        budget_cap_minor: body.budget_cap_minor ?? null,
        currency: body.currency ?? null,
        committed_at: null,
      });
      return route.fulfill({
        json: {
          pid: "g1",
          name: "service text",
          status: "draft",
          rationale: RATIONALE_AT_1000,
        },
      });
    }
    if (path.endsWith("/rollback")) {
      if (opts.rollbackStatus) {
        scenarios[1]!.status = "draft";
        return route.fulfill({
          status: opts.rollbackStatus,
          json: { code: "conflict", detail: "service text" },
        });
      }
      scenarios[1]!.status = "draft";
      return route.fulfill({ json: scenarios[1] });
    }
    return route.fulfill({ json: {} });
  });
  return sent;
}

test("the frontier: its rules said first, checked before anything is asked, drawn with its table twin, and a point's choice", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/scenarios");
  await settle(page);
  const c = page.getByRole("region", { name: "Choosing a portfolio" });
  await expect(c).toContainText(
    "A plan with no Smart Score is left out; a plan with no budget lines costs nothing.",
  );
  await expect(
    c.getByRole("checkbox", { name: "Website replatform" }),
  ).toBeVisible();
  const f = c.getByRole("region", { name: "Efficient frontier" });
  await expect(f).toContainText("Spread 2 to 50 caps evenly");
  await f.getByLabel("Currency of the costs (three letters)").fill("12");
  await f.getByLabel("How many caps (empty for 10)").fill("1");
  await f.getByRole("button", { name: "Draw the frontier" }).click();
  await expect(f.getByRole("alert")).toHaveText(
    "Give the currency as three letters. The number of caps must be a whole number from 2 to 50.",
  );
  expect(sent).toEqual([]);
  await f.getByLabel("Currency of the costs (three letters)").fill("gbp");
  await f.getByLabel("How many caps (empty for 10)").fill("10");
  await f.getByRole("button", { name: "Draw the frontier" }).click();
  await expect(page.getByTestId("frontier-chart")).toContainText(
    "3 budget caps. The most value, 62.5, costs £800.00. 1 cap buys no more value than a cheaper one.",
  );
  expect(sent).toEqual([
    { method: "GET", path: "/api/scenarios/frontier?currency=GBP" },
  ]);
  const table = page.getByTestId("frontier-table");
  await expect(
    table.getByRole("row", {
      name: /£2,000\.00.*No: a cheaper cap reaches as much/,
    }),
  ).toBeVisible();
  await f.getByRole("button", { name: "Show the choice at £1,000.00" }).click();
  const choice = c.getByRole("region", {
    name: "What a cap of £1,000.00 chooses",
  });
  await expect(choice).toContainText("1 plan chosen of 3.");
  await expect(
    choice.getByRole("row", {
      name: /Website replatform.*✓ chosen.*62\.5.*£800\.00/,
    }),
  ).toBeVisible();
  await expect(choice).toContainText(
    "it has no Smart Score, which is not a score of zero",
  );
  await expect(choice).toContainText(
    "A plan you cannot read, or that no longer exists",
  );
  await expect(choice).toContainText(
    "it must be included, but it is not a plan you can read",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("named caps and the plans to include are sent; a point is saved as a draft in the page's own words", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/scenarios");
  await settle(page);
  const c = page.getByRole("region", { name: "Choosing a portfolio" });
  await c.getByRole("checkbox", { name: "Website replatform" }).check();
  const f = c.getByRole("region", { name: "Efficient frontier" });
  await f.getByLabel("Currency of the costs (three letters)").fill("GBP");
  await f.getByRole("radio", { name: "Caps I name" }).check();
  await f
    .getByLabel("Budget caps, one per line (up to 50)")
    .fill("0\n1000\n2,000");
  await f.getByRole("button", { name: "Draw the frontier" }).click();
  await expect(page.getByTestId("frontier-table")).toBeVisible();
  await f
    .getByRole("button", { name: "Save the choice at £1,000.00 as a draft" })
    .click();
  await expect(f.getByRole("status")).toHaveText(
    "Draft Frontier at £1,000.00 saved; it is in the list above.",
  );
  expect(sent).toEqual([
    {
      method: "GET",
      path: `/api/scenarios/frontier?currency=GBP&caps=0%2C100000%2C200000&must_include=${PID}`,
    },
    {
      method: "POST",
      path: "/api/scenarios/generate",
      body: {
        name: "Frontier at £1,000.00",
        budget_cap_minor: 100000,
        currency: "GBP",
        must_include: [PID],
      },
    },
  ]);
  await expect(
    page.getByRole("row", { name: /Frontier at £1,000\.00/ }).first(),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a generated draft: its name and cap checked first, the body, its choice; a refusal in the catalogue's words", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/scenarios");
  await settle(page);
  const g = page.getByRole("region", { name: "Generate a draft" });
  await expect(g).toContainText("A name is needed, at most 1024 bytes.");
  await g.getByLabel("Budget cap (optional)").fill("500");
  await g.getByRole("button", { name: "Generate the draft" }).click();
  await expect(g.getByRole("alert")).toHaveText(
    "Give the draft a name. A cap needs its currency.",
  );
  expect(sent).toEqual([]);
  await g.getByLabel("Name of the draft").fill(" Spring ");
  await g.getByLabel("Currency (needed with a cap)").fill("gbp");
  await g.getByRole("button", { name: "Generate the draft" }).click();
  await expect(g.getByRole("status")).toHaveText(
    "Draft Spring saved; it is in the list above.",
  );
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/scenarios/generate",
      body: { name: "Spring", budget_cap_minor: 50000, currency: "GBP" },
    },
  ]);
  await expect(
    page.getByRole("region", { name: "What Spring chose" }),
  ).toContainText("1 plan chosen of 3.");
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused draft is the catalogue's sentence", async ({ page }) => {
  await stub(page, { generateStatus: 422 });
  await page.goto("/en-001/scenarios");
  await settle(page);
  const g = page.getByRole("region", { name: "Generate a draft" });
  await g.getByLabel("Name of the draft").fill("Spring");
  await g.getByRole("button", { name: "Generate the draft" }).click();
  await expect(g.getByRole("alert")).toHaveText("The draft was refused.");
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a committed scenario is rolled back after a question; Cancel sends nothing; a draft offers no rollback", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/scenarios");
  await settle(page);
  await expect(
    page.getByText(
      "A committed scenario can be rolled back to a draft, after a question",
    ),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Roll back Roomy" }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Roll back Chosen" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Roll Chosen back to a draft? It is no longer the committed portfolio; the commitment stays in the audit trail.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await page.getByRole("button", { name: "Roll back Chosen" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByText("Chosen is a draft again.")).toBeVisible();
  expect(sent).toEqual([
    { method: "POST", path: "/api/scenarios/s2/rollback", body: {} },
  ]);
  await expect(
    page.getByRole("button", { name: "Roll back Chosen" }),
  ).toHaveCount(0);
});

test("a rollback refused as no longer committed is the catalogue's sentence, and the list is read again", async ({
  page,
}) => {
  await stub(page, { rollbackStatus: 409 });
  await page.goto("/en-001/scenarios");
  await settle(page);
  await page.getByRole("button", { name: "Roll back Chosen" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Chosen is no longer committed, so there is nothing to roll back. The list has been read again.",
  );
  await expect(
    page.getByRole("button", { name: "Roll back Chosen" }),
  ).toHaveCount(0);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("choosing a portfolio reads in Welsh and passes axe with the frontier drawn and a choice open", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/cy-001/scenarios");
  await settle(page);
  const c = page.getByRole("region", { name: "Dewis portffolio" });
  const f = c.getByRole("region", { name: "Ffin effeithlon" });
  await f.getByLabel("Arian cyfred y costau (tair llythyren)").fill("GBP");
  await f.getByRole("button", { name: "Llunio'r ffin" }).click();
  await expect(page.getByTestId("frontier-chart")).toContainText(
    "Mae'r gwerth mwyaf",
  );
  await f.getByRole("button", { name: "Dangos y dewis ar £1,000.00" }).click();
  await expect(
    c.getByRole("region", { name: "Beth mae terfyn o £1,000.00 yn ei ddewis" }),
  ).toBeVisible();
  await expect(c).toContainText(
    "nid oes ganddo Sgôr Smart, nad yw'n sgôr o sero",
  );
  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
