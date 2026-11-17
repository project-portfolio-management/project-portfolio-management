// NC-1 slice 11 (EX-12): the funding ledger's writes, in a real browser over a stubbed service. An allocation is
// recorded (every problem first, the amount read in the reader's format and sent in minor units); a plan's lines draw
// on allocations of their own source and currency only, a line with no source is told to get one, and a refusal puts
// the choice back; a cost code is mapped to a line; a finance export is reconciled as text, its header checked first,
// and the report says what matched, what did not and what is in another currency. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const alloc = (
  pid: string,
  source: string,
  name: string,
  currency: string,
  amount: number,
) => ({
  pid,
  funding_source_pid: source,
  funding_source: name,
  amount_minor: amount,
  currency,
  allocated_on: "2026-10-01",
  reference: null,
  planned_on_lines_minor: 0,
  untraced_minor: amount,
  lines: 0,
});

async function stubLedger(page: Page, opts: { lineStatus?: number } = {}) {
  const allocations = [
    alloc("a1", "s1", "Capital grant", "GBP", 1000000),
    alloc("a2", "s1", "Capital grant", "EUR", 50000),
    alloc("a3", "s2", "Trust", "GBP", 70000),
  ];
  const lines = [
    {
      pid: "l1",
      category: "capex",
      description: "Servers",
      currency: "GBP",
      planned_minor: 500000,
      actual_minor: 120000,
      funding_source_pid: "s1",
      funding_allocation_pid: null,
    },
    {
      pid: "l2",
      category: "opex",
      description: "Licences",
      currency: "GBP",
      planned_minor: 100000,
      actual_minor: 0,
      funding_source_pid: null,
      funding_allocation_pid: null,
    },
  ];
  const codes: Array<{ cost_code: string; budget_line_pid: string }> = [];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/budget-lines`, (route) =>
    route.fulfill({ json: { lines, totals: [] } }),
  );
  await page.route("**/api/funding-sources", (route) =>
    route.fulfill({
      json: {
        funding_sources: [
          { pid: "s1", name: "Capital grant", active: true },
          { pid: "s2", name: "Trust", active: true },
        ],
      },
    }),
  );
  await page.route(
    /\/api\/(funding-allocations|cost-codes|ledger\/reconcile|plans\/[^/]+\/budget-lines\/[^/]+\/funding-allocation)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      if (method === "GET") {
        if (path.endsWith("/cost-codes"))
          return route.fulfill({ json: { cost_codes: codes } });
        return route.fulfill({ json: { allocations } });
      }
      if (path.endsWith("/reconcile")) {
        sent.push({ method, path, body: req.postData() });
        return route.fulfill({
          json: {
            rows: 3,
            report: {
              matched: [
                {
                  cost_code: "CC-1",
                  line_pid: "l1",
                  currency: "GBP",
                  ledger_minor: 125000,
                  register_minor: 120000,
                  difference_minor: 5000,
                  rows: 1,
                },
              ],
              unmatched: [
                {
                  cost_code: "ZZ-9",
                  currency: "GBP",
                  amount_minor: 3000,
                  rows: 1,
                },
              ],
              currency_mismatches: [
                {
                  cost_code: "CC-1",
                  row_currency: "EUR",
                  line_currency: "GBP",
                  amount_minor: 900,
                },
              ],
              reconciled: false,
            },
          },
        });
      }
      const body = req.postDataJSON();
      sent.push({ method, path, body });
      if (path.endsWith("/funding-allocation")) {
        if (opts.lineStatus)
          return route.fulfill({
            status: opts.lineStatus,
            json: { error: "x", description: "service text" },
          });
        lines.find((l) => path.includes(`/${l.pid}/`))!.funding_allocation_pid =
          body.funding_allocation_pid ?? null;
        return route.fulfill({ json: {} });
      }
      if (path.endsWith("/cost-codes")) {
        codes.push(body);
        return route.fulfill({ json: body });
      }
      allocations.unshift(
        alloc(
          "a9",
          body.funding_source_pid,
          "Trust",
          body.currency,
          body.amount_minor,
        ),
      );
      return route.fulfill({ json: allocations[0] });
    },
  );
  return sent;
}

test("an allocation is recorded after every problem is said, in minor units", async ({
  page,
}) => {
  const sent = await stubLedger(page);
  await page.goto("/en-001/ledger");
  await settle(page);
  const form = page.getByRole("region", { name: "Record an allocation" });
  await form.getByLabel("Amount").fill("0");
  await form.getByRole("button", { name: "Record the allocation" }).click();
  await expect(form.getByRole("alert")).toContainText(
    "Choose a funding source.",
  );
  await expect(form.getByRole("alert")).toContainText(
    "Write the amount as a number above 0.",
  );
  await expect(form.getByRole("alert")).toContainText(
    "Choose the date it was allocated.",
  );
  expect(sent).toEqual([]);
  await form.getByLabel("Funding source").selectOption("s2");
  await form.getByLabel("Amount").fill("1,234.50");
  await form.getByLabel("Allocated on").fill("2026-11-02");
  await form.getByLabel("Reference (optional)").fill("  PO-77 ");
  await form.getByRole("button", { name: "Record the allocation" }).click();
  await expect(form.getByRole("status")).toHaveText("Allocation recorded.");
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/funding-allocations",
      body: {
        funding_source_pid: "s2",
        amount_minor: 123450,
        currency: "GBP",
        allocated_on: "2026-11-02",
        reference: "PO-77",
      },
    },
  ]);
});

test("a line draws on allocations of its own source and currency only; one without a source is told", async ({
  page,
}) => {
  const sent = await stubLedger(page);
  await page.goto("/en-001/ledger");
  await settle(page);
  const lines = page.getByRole("region", {
    name: "Budget lines and the allocations they draw on",
  });
  await lines.getByLabel("Plan").selectOption(PID);
  const servers = lines.getByLabel("Allocation it draws on: Servers");
  await expect(servers.getByRole("option")).toHaveCount(2); // none, and the one GBP allocation of its source
  await expect(lines.getByText("needs a funding source first")).toBeVisible();
  await servers.selectOption("a1");
  await expect(lines.getByRole("status")).toHaveText(
    "The line now draws on that allocation.",
  );
  await servers.selectOption("");
  await expect(lines.getByRole("status")).toHaveText(
    "The line no longer draws on an allocation.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/budget-lines/l1/funding-allocation`,
      body: { funding_allocation_pid: "a1" },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/budget-lines/l1/funding-allocation`,
      body: {},
    },
  ]);
});

test("a refused draw is said in the catalogue's words and put back", async ({
  page,
}) => {
  await stubLedger(page, { lineStatus: 422 });
  await page.goto("/en-001/ledger");
  await settle(page);
  const lines = page.getByRole("region", {
    name: "Budget lines and the allocations they draw on",
  });
  await lines.getByLabel("Plan").selectOption(PID);
  await lines.getByLabel("Allocation it draws on: Servers").selectOption("a1");
  await expect(lines.getByRole("alert")).toHaveText(
    "The line cannot draw on that allocation: they must have the same funding source and currency.",
  );
  await expect(lines.getByLabel("Allocation it draws on: Servers")).toHaveValue(
    "",
  );
});

test("a cost code is mapped to a line, and an export is reconciled as text after its header is checked", async ({
  page,
}) => {
  const sent = await stubLedger(page);
  await page.goto("/en-001/ledger");
  await settle(page);
  await page
    .getByRole("region", {
      name: "Budget lines and the allocations they draw on",
    })
    .getByLabel("Plan")
    .selectOption(PID);
  const codes = page.getByRole("region", { name: "Cost codes" });
  await codes.getByLabel("Cost code").fill(" CC-1 ");
  await codes.getByRole("button", { name: "Map the cost code" }).click();
  await expect(codes.getByRole("alert")).toHaveText(
    "Choose the budget line for the code.",
  );
  await codes
    .getByLabel("Budget line (from the plan above)")
    .selectOption("l1");
  await codes.getByRole("button", { name: "Map the cost code" }).click();
  await expect(codes.getByRole("status")).toHaveText("Cost code mapped.");
  await expect(codes.getByText("CC-1", { exact: true })).toBeVisible();

  const rec = page.getByRole("region", { name: "Reconcile a finance export" });
  await rec.getByLabel("Or paste the export (CSV)").fill("code,amount\nCC-1,1");
  await rec.getByRole("button", { name: "Reconcile" }).click();
  await expect(rec.getByRole("alert")).toHaveText(
    "The export's first line must name cost_code, amount_minor and currency.",
  );
  const csv =
    "cost_code,amount_minor,currency\nCC-1,125000,GBP\nZZ-9,3000,GBP\nCC-1,900,EUR";
  await rec.getByLabel("Or paste the export (CSV)").fill(csv);
  await rec.getByRole("button", { name: "Reconcile" }).click();
  await expect(
    rec.getByText(
      "Not reconciled: something does not match. Nothing was written.",
    ),
  ).toBeVisible();
  await expect(rec.getByText("3 rows read.")).toBeVisible();
  await expect(
    rec.getByText(
      /in the export £1,250\.00, in the register £1,200\.00, difference £50\.00/,
    ),
  ).toBeVisible();
  await expect(rec.getByText(/row in EUR, line in GBP/)).toBeVisible();
  expect(sent.map((s) => [s.method, s.path, s.body])).toEqual([
    ["PUT", "/api/cost-codes", { cost_code: "CC-1", budget_line_pid: "l1" }],
    ["POST", "/api/ledger/reconcile", csv],
  ]);
});

test("the ledger's writes read in Welsh and pass axe", async ({ page }) => {
  await stubLedger(page);
  await page.goto("/cy-001/ledger");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 2, name: "Cofnodi dyraniad" }),
  ).toBeVisible();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
