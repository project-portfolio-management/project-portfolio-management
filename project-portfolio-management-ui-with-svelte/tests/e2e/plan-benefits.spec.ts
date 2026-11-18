// NC-1 slice 11 (DP-9): a plan's benefit classification and validation, in a real browser over a stubbed service.
// The plan page links here; a financial benefit is classified (the whole classification sent, a yearly amount only when
// recurring) and validated by an amount to date, its standing and history shown; a non-financial one is given a KPI
// and says it is measured by it; a validator in the programme is refused in the catalogue's words. NC-1 slice 12 part
// (f): the value realized read live (no service text), and an observed value recorded (its rules said first, the first
// measurable one asked first, a refusal in the catalogue's words). Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const VALUE = {
  plan_pid: PID,
  transformation_roi: {
    basis_points: 2500,
    absent: null,
    realized_minor: 500000,
    investment_minor: 400000,
    measured_share_basis_points: 5000,
  },
  time_to_value: { observations: 1, p50_days: 40, p85_days: 40, absent: null },
  adoption: { measured: false, reason: "service text" },
  performance_to_business_case: [
    {
      metric: "Beds freed",
      baseline_value: 10,
      target_value: 40,
      promised_by: null,
      source: "charter",
      approved_at: "2026-10-01T09:00:00Z",
    },
  ],
  as_of: "2026-11-18T10:00:00Z",
  asserted: true,
};

/** The realized-value reading and the value points recorded (NC-1 slice 12 part f). */
async function stubValue(page: Page, opts: { refuse?: number } = {}) {
  const sent: Array<{ path: string; body: unknown }> = [];
  await page.route(
    /\/api\/plans\/[^/]+\/(value-realization|value-points)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") return route.fulfill({ json: VALUE });
      sent.push({ path, body: req.postDataJSON() });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { code: "unprocessable", detail: "service text" },
        });
      return route.fulfill({ json: { pid: "v1" } });
    },
  );
  return sent;
}

async function stubBenefits(page: Page, opts: { refuse?: number } = {}) {
  const benefits = [
    {
      pid: "b1",
      title: "Fewer readmissions",
      category: "financial",
      currency: "GBP",
      target_minor: 1000000,
      realized_minor: 250000,
      target_note: null,
      status: "planned",
      expected_on: null,
      recurrence: null,
      annualized_minor: null,
      classification: null,
      kpi_pid: null,
    },
    {
      pid: "b2",
      title: "Better patient experience",
      category: "quality",
      currency: null,
      target_minor: null,
      realized_minor: 0,
      target_note: null,
      status: "planned",
      expected_on: null,
      recurrence: null,
      annualized_minor: null,
      classification: null,
      kpi_pid: null,
    },
  ];
  const history: Array<Record<string, unknown>> = [];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/kpis/, (route) =>
    route.fulfill({
      json: {
        as_of: "2026-11-17",
        kpis: [
          {
            kpi: { pid: "k1", name: "Patient survey score" },
            status: "on_track",
            latest: null,
            trend: null,
            stale: null,
          },
        ],
        summary: {},
        note: "",
      },
    }),
  );
  await page.route(/\/api\/plans\/[^/]+\/benefits/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    if (method === "GET") {
      if (path.endsWith("/benefits"))
        return route.fulfill({ json: { benefits, totals: [] } });
      const last = history.at(-1);
      return route.fulfill({
        json: {
          benefit: "b1",
          claimed_minor: 250000,
          validated_minor: last ? last.amount_minor : null,
          standing: last ? "partly_validated" : "claimed",
          history,
        },
      });
    }
    const body = req.postDataJSON();
    sent.push({ method, path, body });
    if (opts.refuse)
      return route.fulfill({
        status: opts.refuse,
        json: { error: "x", description: "service text" },
      });
    if (path.endsWith("/classification")) {
      const b = benefits.find((x) => path.includes(`/${x.pid}/`))!;
      Object.assign(
        b,
        {
          recurrence: null,
          annualized_minor: null,
          classification: null,
          kpi_pid: null,
        },
        body,
      );
      return route.fulfill({ json: {} });
    }
    history.push({
      amount_minor: body.amount_minor,
      currency: "GBP",
      validator: "x",
      validated_on: "2026-11-17",
      evidence_attachment_pid: null,
      note: body.note ?? null,
    });
    return route.fulfill({ json: {} });
  });
  return sent;
}

test("a financial benefit is classified, sending the whole classification, and a yearly amount only when recurring", async ({
  page,
}) => {
  const sent = await stubBenefits(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page
    .getByRole("link", { name: "Benefits: classification and validation" })
    .click();
  const b1 = page.getByRole("article", { name: "Fewer readmissions" });
  await expect(
    b1.getByText("Target £10,000.00; realized (claimed) £2,500.00."),
  ).toBeVisible();
  await expect(b1.getByLabel("Yearly amount (GBP)")).toHaveCount(0);
  await b1.getByLabel("Recurs").selectOption("recurring");
  await b1.getByLabel("Yearly amount (GBP)").fill("1,200");
  await b1.getByLabel("Classification").selectOption("cash_releasing");
  await b1.getByRole("button", { name: "Save the classification" }).click();
  await expect(b1.getByRole("status").first()).toHaveText(
    "Classification saved.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/benefits/b1/classification`,
      body: {
        recurrence: "recurring",
        annualized_minor: 120000,
        classification: "cash_releasing",
      },
    },
  ]);
});

test("a financial benefit is validated by its amount to date; its standing and history show", async ({
  page,
}) => {
  const sent = await stubBenefits(page);
  await page.goto(`/en-001/plans/${PID}/benefits`);
  await settle(page);
  const b1 = page.getByRole("article", { name: "Fewer readmissions" });
  await expect(b1.getByText("Claimed, not yet validated")).toBeVisible();
  await b1.getByLabel("Amount validated to date (GBP)").fill("-1");
  await b1.getByRole("button", { name: "Record the validation" }).click();
  await expect(b1.getByRole("alert")).toHaveText(
    "Write the amount validated to date as a number, not below 0.",
  );
  await b1.getByLabel("Amount validated to date (GBP)").fill("2,000");
  await b1.getByLabel("Note (optional)").fill(" Audit Q3 ");
  await b1.getByRole("button", { name: "Record the validation" }).click();
  await expect(b1.getByText("Validation recorded.")).toBeVisible();
  await expect(b1.getByText("Partly validated")).toBeVisible();
  await expect(b1.getByText("£2,000.00 validated to date")).toBeVisible();
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/benefits/b1/validations`,
    body: { amount_minor: 200000, note: "Audit Q3" },
  });
});

test("a non-financial benefit is given a KPI and says it is measured by it", async ({
  page,
}) => {
  const sent = await stubBenefits(page);
  await page.goto(`/en-001/plans/${PID}/benefits`);
  await settle(page);
  const b2 = page.getByRole("article", { name: "Better patient experience" });
  await expect(
    b2.getByText(
      "A non-financial benefit is measured by its KPI, not validated as an amount.",
    ),
  ).toBeVisible();
  await expect(
    b2.getByRole("button", { name: "Record the validation" }),
  ).toHaveCount(0);
  await b2.getByLabel("KPI that measures it").selectOption("k1");
  await b2.getByRole("button", { name: "Save the classification" }).click();
  await expect(b2.getByRole("status").first()).toHaveText(
    "Classification saved.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/benefits/b2/classification`,
      body: { kpi_pid: "k1" },
    },
  ]);
});

test("a validator in the programme is refused in the catalogue's words", async ({
  page,
}) => {
  await stubBenefits(page, { refuse: 403 });
  await page.goto(`/en-001/plans/${PID}/benefits`);
  await settle(page);
  const b1 = page.getByRole("article", { name: "Fewer readmissions" });
  await b1.getByLabel("Amount validated to date (GBP)").fill("100");
  await b1.getByRole("button", { name: "Record the validation" }).click();
  await expect(b1.getByRole("alert")).toHaveText(
    "A validator may not be a member of the programme: ask someone outside it to validate.",
  );
});

test("the value realized is read live, each figure in the catalogue's words, never the service's", async ({
  page,
}) => {
  await stubBenefits(page);
  await stubValue(page);
  await page.goto(`/en-001/plans/${PID}/benefits`);
  await settle(page);
  const v = page.getByRole("region", { name: "Value realized" });
  await expect(v.getByTestId("value-roi")).toHaveText(
    "Return on investment: 25.0% · measured evidence: 50% of the value points",
  );
  await expect(v.getByTestId("value-ttv")).toHaveText(
    "Time to value: median 40 calendar days, 85th percentile 40 (1 observation)",
  );
  await expect(v.getByTestId("value-adoption")).toHaveText(
    "No adoption reading is recorded.",
  );
  await expect(v.getByTestId("value-targets")).toContainText(
    "Beds freed: from 10 to 40 · the charter · approved",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("an observed value: its rules said first, the first measurable one asked first, a refusal in the catalogue's words", async ({
  page,
}) => {
  await stubBenefits(page);
  const sent = await stubValue(page);
  await page.goto(`/en-001/plans/${PID}/benefits`);
  await settle(page);
  const v = page.getByRole("region", { name: "Value realized" });
  await expect(v).toContainText(
    "One value point per plan may be the first measurable value",
  );
  await v.getByLabel("Value", { exact: true }).fill("2.5");
  await v.getByRole("button", { name: "Record the value" }).click();
  await expect(v.getByRole("alert")).toHaveText(
    "Without a currency, the value must be a whole number.",
  );
  expect(sent).toEqual([]);
  await v.getByLabel("Value", { exact: true }).fill("1,250.50");
  await v.getByLabel("Currency (three letters, or empty)").fill("gbp");
  await v.getByLabel("How it was arrived at").selectOption("estimated");
  await v
    .getByLabel("Evidence (a link or reference, optional)")
    .fill(" doc:7 ");
  await v.getByRole("button", { name: "Record the value" }).click();
  await expect(v.getByRole("status")).toHaveText("Value recorded.");
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/value-points`,
      body: {
        value: 125050,
        method: "estimated",
        currency: "GBP",
        evidence_ref: "doc:7",
      },
    },
  ]);

  await v.getByLabel("Value", { exact: true }).fill("10");
  await v.getByLabel("This is the first measurable value").check();
  await v.getByRole("button", { name: "Record the value" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Record this as the plan's first measurable value? It stops the time-to-value clock and cannot be moved.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toHaveLength(1);

  await page.unroute(/\/api\/plans\/[^/]+\/(value-realization|value-points)/);
  const refused = await stubValue(page, { refuse: 422 });
  await v.getByRole("button", { name: "Record the value" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(v.getByRole("alert")).toHaveText(
    "Refused: this plan already has a first measurable value; the clock stops once.",
  );
  expect(refused).toEqual([
    {
      path: `/api/plans/${PID}/value-points`,
      body: {
        value: 1000,
        method: "estimated",
        currency: "GBP",
        is_first_measurable: true,
      },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the benefits page reads in Welsh and passes axe", async ({ page }) => {
  await stubBenefits(page);
  await stubValue(page);
  await page.goto(`/cy-001/plans/${PID}/benefits`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Buddion: dosbarthiad a dilysu",
  );
  await expect(
    page.getByRole("heading", { level: 2, name: "Gwerth a wireddwyd" }),
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
