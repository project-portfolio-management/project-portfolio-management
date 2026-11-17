// NC-1 slice 11 (DP-9): a plan's benefit classification and validation, in a real browser over a stubbed service.
// The plan page links here; a financial benefit is classified (the whole classification sent, a yearly amount only when
// recurring) and validated by an amount to date, its standing and history shown; a non-financial one is given a KPI
// and says it is measured by it; a validator in the programme is refused in the catalogue's words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

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

test("the benefits page reads in Welsh and passes axe", async ({ page }) => {
  await stubBenefits(page);
  await page.goto(`/cy-001/plans/${PID}/benefits`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Buddion: dosbarthiad a dilysu",
  );
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
