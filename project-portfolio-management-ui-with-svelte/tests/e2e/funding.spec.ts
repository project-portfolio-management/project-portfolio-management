// NC-1 slice 11 (DP-6): funding sources, in a real browser over a stubbed service. Sources are listed, added (a short or
// taken name said first), retired and restored; a plan's budget lines each get a source or none, a retired one shown
// only on the line that has it, and a refusal puts the choice back; the money by funding source is drawn as the
// director pack draws it, for a chosen year. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";
import { BODY } from "../unit/fixtures/director-pack-body";

async function stubFunding(page: Page, opts: { lineStatus?: number } = {}) {
  const sources = [
    { pid: "s1", name: "Capital grant", active: true },
    { pid: "s2", name: "Old fund", active: false },
  ];
  const lines = [
    {
      pid: "l1",
      category: "capex",
      description: "Servers",
      currency: "GBP",
      planned_minor: 500000,
      actual_minor: 0,
      funding_source_pid: null,
    },
    {
      pid: "l2",
      category: "opex",
      description: "Licences",
      currency: "GBP",
      planned_minor: 120000,
      actual_minor: 0,
      funding_source_pid: "s2",
    },
  ];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  const asked: string[] = [];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/budget-lines`, (route) =>
    route.fulfill({ json: { lines, totals: [] } }),
  );
  await page.route(/\/api\/portfolio-money/, (route) => {
    asked.push(new URL(route.request().url()).search);
    return route.fulfill({ json: BODY.funding });
  });
  await page.route(
    /\/api\/(funding-sources|plans\/[^/]+\/budget-lines\/[^/]+\/funding-source)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET")
        return route.fulfill({ json: { funding_sources: sources } });
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (path.endsWith("/funding-source")) {
        if (opts.lineStatus)
          return route.fulfill({
            status: opts.lineStatus,
            json: { error: "x", description: "service text" },
          });
        lines.find((l) => path.includes(`/${l.pid}/`))!.funding_source_pid =
          body.funding_source_pid ?? null;
        return route.fulfill({ json: {} });
      }
      if (req.method() === "POST") {
        const row = { pid: "s3", name: body.name, active: true };
        sources.push(row);
        return route.fulfill({ json: row });
      }
      const row = sources.find((s) => path.endsWith(s.pid))!;
      row.active = body.active;
      return route.fulfill({ json: row });
    },
  );
  return { sent, asked };
}

test("sources are listed, added after their name is checked, retired and restored", async ({
  page,
}) => {
  const { sent } = await stubFunding(page);
  await page.goto("/en-001/funding");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Funding" }),
  ).toBeVisible();
  const list = page.getByRole("region", { name: "Funding sources" });
  await list.getByLabel("New funding source").fill(" capital GRANT ");
  await list.getByRole("button", { name: "Add the source" }).click();
  await expect(list.getByRole("alert")).toHaveText(
    "A funding source with that name already exists.",
  );
  await list.getByLabel("New funding source").fill("x");
  await list.getByRole("button", { name: "Add the source" }).click();
  await expect(list.getByRole("alert")).toHaveText(
    "A funding source's name is 2 to 120 characters.",
  );
  expect(sent).toEqual([]);
  await list.getByLabel("New funding source").fill("  Trust income ");
  await list.getByRole("button", { name: "Add the source" }).click();
  await expect(list.getByRole("status")).toHaveText("Funding source added.");
  await list.getByRole("button", { name: "Retire: Capital grant" }).click();
  await expect(list.getByRole("status")).toHaveText("Funding source retired.");
  await list.getByRole("button", { name: "Restore: Old fund" }).click();
  await expect(list.getByRole("status")).toHaveText("Funding source restored.");
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/funding-sources",
      body: { name: "Trust income" },
    },
    { method: "PUT", path: "/api/funding-sources/s1", body: { active: false } },
    { method: "PUT", path: "/api/funding-sources/s2", body: { active: true } },
  ]);
});

test("a plan's lines each get a source or none; a retired one shows only on the line that has it", async ({
  page,
}) => {
  const { sent } = await stubFunding(page);
  await page.goto("/en-001/funding");
  await settle(page);
  const lines = page.getByRole("region", {
    name: "Budget lines and their funding source",
  });
  await lines.getByLabel("Plan").selectOption(PID);
  const servers = lines.getByLabel("Funding source: Servers");
  const licences = lines.getByLabel("Funding source: Licences");
  await expect(servers).toHaveValue("");
  await expect(
    servers.getByRole("option", { name: "Old fund (retired)" }),
  ).toHaveCount(0);
  await expect(licences).toHaveValue("s2");
  await expect(
    licences.getByRole("option", { name: "Old fund (retired)" }),
  ).toHaveCount(1);
  await servers.selectOption("s1");
  await expect(lines.getByRole("status")).toHaveText(
    "The line's funding source is set.",
  );
  await licences.selectOption("");
  await expect(lines.getByRole("status")).toHaveText(
    "The line no longer has a funding source.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/budget-lines/l1/funding-source`,
      body: { funding_source_pid: "s1" },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/budget-lines/l2/funding-source`,
      body: {},
    },
  ]);
});

test("a refused line choice is said in the catalogue's words and put back", async ({
  page,
}) => {
  await stubFunding(page, { lineStatus: 422 });
  await page.goto("/en-001/funding");
  await settle(page);
  const lines = page.getByRole("region", {
    name: "Budget lines and their funding source",
  });
  await lines.getByLabel("Plan").selectOption(PID);
  await lines.getByLabel("Funding source: Servers").selectOption("s1");
  await expect(lines.getByRole("alert")).toHaveText(
    "That funding source is retired, so a line cannot be given it.",
  );
  await expect(lines.getByLabel("Funding source: Servers")).toHaveValue("");
});

test("the money by funding source is drawn as the pack draws it, for a chosen year", async ({
  page,
}) => {
  const { asked } = await stubFunding(page);
  await page.goto("/en-001/funding");
  await settle(page);
  const money = page.getByRole("region", { name: "Money by funding source" });
  await expect(money.getByRole("table").first()).toBeVisible();
  await money.getByLabel(/Financial year/).fill("1999");
  await money.getByRole("button", { name: "Show" }).click();
  await expect(money.getByRole("alert")).toHaveText(
    "Write the financial year as the year it starts in, from 2000 to 2100.",
  );
  await money.getByLabel(/Financial year/).fill("2026");
  await money.getByLabel("Combine capital and revenue").check();
  await money.getByRole("button", { name: "Show" }).click();
  await expect.poll(() => asked).toEqual(["", "?fy=2026&combine_classes=true"]);
});

test("the funding page reads in Welsh and passes axe", async ({ page }) => {
  await stubFunding(page);
  await page.goto("/cy-001/funding");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Cyllid");
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
