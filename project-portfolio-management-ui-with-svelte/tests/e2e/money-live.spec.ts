// NC-1 slice 12 part (f): a plan's money read live, in a real browser over a stubbed service. On /plans/[pid]/costs:
// the estimate at completion in the catalogue's words; the budget baselines, a new version's rules said first, one the
// same as the latest not sent, a question before approving, and a refusal; the tranches held until a gate, each
// released after a question, the incomplete register read from its code; a business-case target recorded after a
// question. On /financials: the forecast rolled up from a chosen plan, its depth checked first and a 404 said. Every
// stub that would carry the service's English carries "service text", which never shows. Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const V1 = {
  pid: "b1",
  version: 1,
  currency: "GBP",
  approved_at: "2026-10-01T09:00:00Z",
  reason: null,
  periods: [
    {
      period_start: "2026-11-01",
      period_end: "2026-11-30",
      planned_minor: 150000,
    },
  ],
};

async function stubCosts(
  page: Page,
  opts: {
    refuse?: { path: RegExp; status: number; code?: string };
    baselines?: unknown[];
  } = {},
) {
  const sent: Array<{ path: string; body: unknown }> = [];
  const baselines = [...(opts.baselines ?? [V1])];
  const lines = [
    {
      pid: "l1",
      category: "capex",
      description: "Build phase",
      currency: "GBP",
      planned_minor: 200000,
      actual_minor: 0,
      gate: "g2_definition",
      released_at: null,
    },
    {
      pid: "l2",
      category: "capex",
      description: "Launch phase",
      currency: "GBP",
      planned_minor: 100000,
      actual_minor: 0,
      gate: "g4_launch",
      released_at: null,
    },
    {
      pid: "l3",
      category: "opex",
      description: "Support",
      currency: "GBP",
      planned_minor: 5000,
      actual_minor: 0,
      gate: null,
      released_at: null,
    },
  ];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/cpm**`, (r) =>
    r.fulfill({ json: { nodes: [] } }),
  );
  await page.route(`**/api/plans/${PID}/cost-plan**`, (r) =>
    r.fulfill({
      json: {
        plan_pid: PID,
        currencies: [],
        unrated: [],
        unassigned: [],
        unplaced: [],
        note: "service text",
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/labour-costs`, (r) =>
    r.fulfill({
      json: {
        currencies: [],
        unpriced: { entries: 0, minutes: 0, reason: null },
        note: "service text",
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/financials/forecast`, (r) =>
    r.fulfill({
      json: {
        plan_pid: PID,
        forecast: {
          currency: "GBP",
          actual_minor: 50000,
          excluded_other_currency_minor: 700,
          estimate_to_complete_minor: 150000,
          etc_source: "baseline_remaining",
          estimate_at_completion_minor: 200000,
          baseline_version: baselines.length,
          absent: null,
        },
        as_of: "2026-11-18T10:00:00Z",
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/governance`, (r) =>
    r.fulfill({
      json: {
        pid: PID,
        name: "Wards",
        stage: "g2_definition",
        next_gate: "g3_delivery",
        gate_reviews: 2,
        latest_review: null,
        risks: {
          open: 0,
          materialized: 0,
          max_exposure: null,
          total_exposure: 0,
        },
        budget: [],
      },
    }),
  );
  await page.route(
    /\/api\/plans\/[^/]+\/(budget-baselines|budget-lines|business-case)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") {
        if (path.endsWith("/budget-baselines"))
          return route.fulfill({ json: [...baselines].reverse() });
        return route.fulfill({ json: { lines, totals: [] } });
      }
      const body = req.postDataJSON();
      sent.push({ path, body });
      if (opts.refuse && opts.refuse.path.test(path))
        return route.fulfill({
          status: opts.refuse.status,
          json: {
            code: opts.refuse.code ?? "unprocessable",
            detail: "service text",
          },
        });
      if (path.endsWith("/budget-baselines")) {
        baselines.push({
          ...body,
          pid: `b${baselines.length + 1}`,
          version: baselines.length + 1,
          approved_at: "2026-11-18T10:00:00Z",
          reason: body.reason ?? null,
        });
        return route.fulfill({
          json: { pid: "bx", version: baselines.length },
        });
      }
      if (path.endsWith("/release")) {
        const l = lines.find((x) => path.includes(`/${x.pid}/`))!;
        l.released_at = "2026-11-18T10:00:00Z";
        return route.fulfill({ json: l });
      }
      return route.fulfill({ json: { pid: "t1" } });
    },
  );
  return sent;
}

test("the forecast at completion is read live in the catalogue's words", async ({
  page,
}) => {
  await stubCosts(page);
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const f = page.getByRole("region", { name: "Forecast at completion" });
  await expect(f).toContainText("Actual cost so far£500.00");
  await expect(f).toContainText(
    "£1,500.00 · from what the baseline still plans",
  );
  await expect(f).toContainText("Estimate at completion£2,000.00");
  await expect(f).toContainText("Uses budget baseline version 1.");
  await expect(f).toContainText(
    "Actual cost recorded in a currency other than GBP is not added in.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a new baseline: its rules said first, the same as the latest not sent, a question, then the body", async ({
  page,
}) => {
  const sent = await stubCosts(page);
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const b = page.getByRole("region", { name: "Budget baselines" });
  await expect(b).toContainText("Version 1 · GBP · approved");
  await expect(
    b.getByRole("heading", { level: 3, name: "Approve version 2" }),
  ).toBeVisible();
  await expect(b.getByLabel("Currency (three letters)")).toHaveValue("GBP");
  await expect(b.getByLabel("Period 1 planned amount")).toHaveValue("1500.00");
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "A re-baseline needs a reason.",
  );
  await b
    .getByLabel("Why the budget is re-baselined (needed)")
    .fill("Scope added.");
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "This is the same as the latest version; nothing was sent.",
  );
  await b.getByRole("button", { name: "Add a period" }).click();
  await b.getByLabel("Period 2 starts").fill("2026-12-31");
  await b.getByLabel("Period 2 ends").fill("2026-12-01");
  await b.getByLabel("Period 2 planned amount").fill("-1");
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "A period cannot end before it starts. Each planned amount must be a number, not below zero.",
  );
  expect(sent).toEqual([]);
  await b.getByLabel("Period 2 starts").fill("2026-12-01");
  await b.getByLabel("Period 2 ends").fill("2026-12-31");
  await b.getByLabel("Period 2 planned amount").fill("750");
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Approve budget baseline version 2? Once approved it is frozen and cannot be changed or removed.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(b.getByRole("status")).toHaveText("Version 2 approved.");
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/budget-baselines`,
      body: {
        currency: "GBP",
        periods: [
          {
            period_start: "2026-11-01",
            period_end: "2026-11-30",
            planned_minor: 150000,
          },
          {
            period_start: "2026-12-01",
            period_end: "2026-12-31",
            planned_minor: 75000,
          },
        ],
        reason: "Scope added.",
      },
    },
  ]);
  await expect(
    b.getByRole("heading", { level: 3, name: "Approve version 3" }),
  ).toBeVisible();
  await expect(b).toContainText("Scope added.");
});

test("the first baseline needs no reason; a refusal is the catalogue's sentence", async ({
  page,
}) => {
  const sent = await stubCosts(page, {
    baselines: [],
    refuse: { path: /budget-baselines/, status: 422 },
  });
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const b = page.getByRole("region", { name: "Budget baselines" });
  await expect(b).toContainText("No budget baseline has been approved.");
  await b.getByLabel("Currency (three letters)").fill("eur");
  await b.getByLabel("Period 1 starts").fill("2027-01-01");
  await b.getByLabel("Period 1 ends").fill("2027-03-31");
  await b.getByLabel("Period 1 planned amount").fill("10000");
  await b.getByRole("button", { name: "Approve the baseline" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(b.getByRole("alert")).toHaveText(
    "The baseline was refused: another version may have been approved meanwhile, which makes a reason needed. The list has been read again.",
  );
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/budget-baselines`,
      body: {
        currency: "EUR",
        periods: [
          {
            period_start: "2027-01-01",
            period_end: "2027-03-31",
            planned_minor: 1000000,
          },
        ],
      },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a tranche whose gate has passed is released after a question; one still held offers no release", async ({
  page,
}) => {
  const sent = await stubCosts(page);
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const tr = page.getByRole("region", { name: "Funding held until a gate" });
  await expect(tr).toContainText("The plan's stage: G2 definition.");
  await expect(tr).not.toContainText("Support");
  await expect(
    tr.getByText("still held: the gate has not passed"),
  ).toBeVisible();
  await expect(
    tr.getByRole("button", { name: "Release Launch phase" }),
  ).toHaveCount(0);
  await tr.getByRole("button", { name: "Release Build phase" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Release Build phase, held until G2 definition? A release cannot be undone.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await tr.getByRole("button", { name: "Release Build phase" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(tr.getByRole("status")).toHaveText("Build phase released.");
  await expect(tr).toContainText("released");
  await expect(
    tr.getByRole("button", { name: "Release Build phase" }),
  ).toHaveCount(0);
  expect(sent).toEqual([
    { path: `/api/plans/${PID}/budget-lines/l1/release`, body: {} },
  ]);
});

test("a release refused for an incomplete register is said from its code", async ({
  page,
}) => {
  await stubCosts(page, {
    refuse: { path: /\/release$/, status: 422, code: "register_incomplete" },
  });
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const tr = page.getByRole("region", { name: "Funding held until a gate" });
  await tr.getByRole("button", { name: "Release Build phase" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(tr.getByRole("alert")).toHaveText(
    "Refused: the programme's register is incomplete. Complete it on the register completeness page first.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a business-case target: its rules said first, a question, then the body", async ({
  page,
}) => {
  const sent = await stubCosts(page);
  await page.goto(`/en-001/plans/${PID}/costs`);
  await settle(page);
  const c = page.getByRole("region", { name: "Business case targets" });
  await c.getByLabel("Value at approval").fill("1.5");
  await c.getByRole("button", { name: "Record the target" }).click();
  await expect(c.getByRole("alert")).toHaveText(
    "Name the measure. The value at approval and the value promised must be whole numbers.",
  );
  await c.getByLabel("Measure").fill(" Beds freed ");
  await c.getByLabel("Value at approval").fill("10");
  await c.getByLabel("Value promised").fill("40");
  await c.getByLabel("Unit (optional)").fill("beds");
  await c.getByLabel("Promised in").selectOption({ label: "a gate review" });
  await c.getByRole("button", { name: "Record the target" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Record the target Beds freed? It cannot be changed or removed here.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await c.getByRole("button", { name: "Record the target" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(c.getByRole("status")).toHaveText("Target Beds freed recorded.");
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/business-case`,
      body: {
        metric: "Beds freed",
        baseline_value: 10,
        target_value: 40,
        unit: "beds",
        source: "gate_review",
      },
    },
  ]);
});

test("the costs page reads in Welsh and passes axe with its forms open", async ({
  page,
}) => {
  await stubCosts(page);
  await page.goto(`/cy-001/plans/${PID}/costs`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 2, name: "Rhagolwg wrth gwblhau" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { level: 2, name: "Llinellau sylfaen cyllideb" }),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});

async function stubRollup(page: Page, opts: { status?: number } = {}) {
  const asked: string[] = [];
  await stubPlansApi(page);
  await page.route(/\/api\/financials\/(variance|exposure)/, (r) =>
    r.fulfill({
      status: 404,
      json: { code: "not_found", detail: "service text" },
    }),
  );
  await page.route(/\/api\/financials\/forecast/, (route) => {
    const url = new URL(route.request().url());
    asked.push(url.search);
    if (opts.status)
      return route.fulfill({
        status: opts.status,
        json: { code: "not_found", detail: "service text" },
      });
    return route.fulfill({
      json: {
        as_of: "2026-11-18T10:00:00Z",
        root: { pid: PID, name: "Wards" },
        note: "service text",
        tree: { plans: 3, depth_limit: 32, truncated: false, revisits: 1 },
        rollup: {
          rows: [
            {
              currency: "EUR",
              actual_minor: 1000,
              estimate_to_complete_minor: 2000,
              estimate_at_completion_minor: 3000,
              plans: 1,
            },
            {
              currency: "GBP",
              actual_minor: 50000,
              estimate_to_complete_minor: 150000,
              estimate_at_completion_minor: 200000,
              plans: 2,
            },
          ],
          no_currency_signal: 1,
        },
      },
    });
  });
  return asked;
}

test("the forecast is rolled up from a chosen plan, one row per currency; its depth is checked first", async ({
  page,
}) => {
  const asked = await stubRollup(page);
  await page.goto("/en-001/financials");
  await settle(page);
  const r = page.getByTestId("fin-rollup");
  await r.getByRole("button", { name: "Show the forecast" }).click();
  await expect(r.getByRole("alert")).toHaveText("Choose a plan.");
  await r.getByLabel("Plan").selectOption(PID);
  await r.getByLabel("Levels deep (1 to 32; empty for all)").fill("40");
  await r.getByRole("button", { name: "Show the forecast" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "Levels deep must be a whole number from 1 to 32, or empty.",
  );
  expect(asked).toEqual([]);
  await r.getByLabel("Levels deep (1 to 32; empty for all)").fill("3");
  await r.getByRole("button", { name: "Show the forecast" }).click();
  await expect(r).toContainText("From Wards: 3 plans read,");
  await expect(r).toContainText("1 plan was reached twice and counted once.");
  await expect(r).toContainText(
    "1 plan has no currency to forecast in and is not added.",
  );
  const table = r.getByTestId("fin-rollup-table");
  await expect(table).toContainText("€30.00");
  await expect(table).toContainText("£2,000.00");
  expect(asked).toEqual([`?plan=${PID}&depth=3`]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a roll-up from a plan gone is said; the page reads in Welsh and passes axe", async ({
  page,
}) => {
  await stubRollup(page, { status: 404 });
  await page.goto("/en-001/financials");
  await settle(page);
  const r = page.getByTestId("fin-rollup");
  await r.getByLabel("Plan").selectOption(PID);
  await r.getByRole("button", { name: "Show the forecast" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "That plan no longer exists, or you cannot read it.",
  );
  await page.goto("/cy-001/financials");
  await settle(page);
  await expect(
    page.getByRole("heading", {
      level: 2,
      name: "Rhagolwg wrth gwblhau, wedi'i grynhoi",
    }),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
