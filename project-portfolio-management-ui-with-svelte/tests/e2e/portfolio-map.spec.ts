// NC-1 slice 11 (DP-4): the live portfolio map and strategic aims, in a real browser over a stubbed service. The map is
// drawn as the director pack draws it (the no-aim group in the catalogue's words); a period is asked of the service; an
// objective is marked and unmarked as a strategic aim, and a refusal puts the box back; a programme's primary aim is set,
// and choosing its current one is said first. NC-1 slice 12 part (f): the pack's programme readings read now (fever
// chart, earned value, benefits, issues and change requests), drawn by the pack's renderer with no service text; a
// window checked first; the window and actual cost asked; a failed reading and no readable plan said. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle, stubPlansApi } from "./fixtures";

const prog = (plan_pid: string, name: string) => ({
  plan_pid,
  name,
  stage: null,
  phase: null,
  programme_status: "active",
  confidence: {
    value: "amber",
    kind: "self",
    marked_self: true,
    assessed_on: "2026-11-01",
    stale: false,
  },
  derived_rag: "green",
  trend: "same",
  disagreement: { with_derived: true, independent_vs_self: false },
  also_serves: [],
  reason_no_primary: null,
});

async function stubMap(
  page: Page,
  opts: { refuse?: number; liveFail?: string; noIssues?: boolean } = {},
) {
  const aims = new Set(["o1"]);
  const primary: Record<string, string | null> = { p1: "o1", p2: null };
  const sent: Array<{ path: string; body: unknown }> = [];
  const asked: string[] = [];
  await stubPlansApi(page);
  await page.route("**/api/objectives", (route) =>
    route.fulfill({
      json: [
        { pid: "o1", title: "Better care", description: null, period: null },
        { pid: "o2", title: "Lower cost", description: null, period: null },
      ],
    }),
  );
  await page.route(/\/api\/portfolio-map/, (route) => {
    const url = new URL(route.request().url());
    asked.push(url.search);
    const titles: Record<string, string> = {
      o1: "Better care",
      o2: "Lower cost",
    };
    const names: Record<string, string> = { p1: "Wards", p2: "Apps" };
    const groups = [
      ...[...aims].map((a) => ({
        aim_pid: a,
        aim: titles[a]!,
        programmes: Object.keys(primary)
          .filter((p) => primary[p] === a)
          .map((p) => prog(p, names[p]!)),
      })),
      {
        aim_pid: null,
        aim: "no strategic aim",
        programmes: Object.keys(primary)
          .filter((p) => !primary[p] || !aims.has(primary[p]!))
          .map((p) => prog(p, names[p]!)),
      },
    ];
    return route.fulfill({
      json: {
        period: url.searchParams.get("period") ?? "2026-11",
        programmes_total: 2,
        groups,
      },
    });
  });
  await page.route(
    /\/api\/(objectives\/[^/]+\/strategic-aim|plans\/[^/]+\/primary-aim)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const body = req.postDataJSON();
      sent.push({ path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { error: "x", description: "service text" },
        });
      if (path.endsWith("/strategic-aim")) {
        const o = path.split("/")[3]!;
        if (body.strategic) aims.add(o);
        else aims.delete(o);
        return route.fulfill({
          json: { objective_pid: o, strategic: body.strategic },
        });
      }
      primary[path.split("/")[3]!] = body.objective_pid;
      return route.fulfill({
        json: { plan_pid: path.split("/")[3], primary_aim: body.objective_pid },
      });
    },
  );
  const live = await stubLive(page, opts);
  return { sent, asked, live };
}

/** The pack's programme readings read now (NC-1 slice 12 part f): every answer carries the service's English where it
 * would, as "service text", which must never show. */
async function stubLive(
  page: Page,
  opts: { liveFail?: string; noIssues?: boolean } = {},
) {
  const asked: string[] = [];
  const answers: Record<string, unknown> = {
    "/api/buffer-fever": {
      lines: {
        green_amber: { at_start: 30, at_end: 70 },
        amber_red: { at_start: 60, at_end: 100 },
      },
      programmes: [
        {
          plan: "Theatres",
          point: {
            chain_completed_pct: 40,
            buffer_consumed_pct: 75,
            zone: "red",
            edge: null,
            slip_days: 6,
            buffer_days: 8,
          },
          previous: null,
        },
        {
          plan: "Apps",
          point: null,
          no_fever_chart_point: { code: "no_baseline", reason: "service text" },
        },
      ],
      note: "service text",
    },
    "/api/portfolio-earned-value": {
      programmes: [
        {
          name: "Theatres",
          state: "evm",
          currencies: [
            {
              currency: "GBP",
              state: "included",
              totals: { bac_minor: 100000, ev_minor: 40000, ac_minor: 50000 },
              spi: { bp: 9000 },
              cpi: { bp: 8000 },
            },
          ],
        },
      ],
      note: "service text",
    },
    "/api/portfolio-benefits": {
      totals: [
        {
          currency: "GBP",
          recurrence: "one_off",
          classification: "cash_releasing",
          planned_minor: 500000,
          claimed_minor: 100000,
          validated_minor: 50000,
          unvalidated_claim_minor: 50000,
        },
      ],
      note: "service text",
    },
    "/api/director-pack/issues-and-changes": opts.noIssues
      ? {
          available: false,
          reason: "service text",
          window_days: 30,
          issues: {},
          change_requests: {},
        }
      : {
          available: true,
          reason: null,
          window_days: 30,
          as_of: "2026-11-18",
          issues: {
            total: 5,
            open: 4,
            open_by_severity: { high: 3, low: 1 },
            overdue: 2,
            raised_in_window: 5,
            resolved_in_window: 1,
            oldest_open: {
              plan_pid: "p1",
              title: "Supplier late",
              age_calendar_days: 40,
            },
            statement: "service text",
          },
          change_requests: {
            total: 3,
            by_status: { submitted: 2 },
            awaiting_decision: 2,
            approved_not_implemented: 1,
            decided_in_window: 3,
            oldest_open: null,
            statement: "service text",
          },
        },
  };
  await page.route(
    /\/api\/(buffer-fever|portfolio-earned-value|portfolio-benefits|director-pack\/issues-and-changes)/,
    (route) => {
      const url = new URL(route.request().url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      asked.push(path + url.search);
      if (opts.liveFail === path)
        return route.fulfill({
          status: 500,
          json: { code: "internal", detail: "service text" },
        });
      return route.fulfill({ json: answers[path] });
    },
  );
  return asked;
}

test("the map is drawn as the pack draws it, and a period is asked of the service", async ({
  page,
}) => {
  const { asked } = await stubMap(page);
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Portfolio map" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { level: 2, name: "Portfolio map, 2026-11" }),
  ).toBeVisible();
  await expect(page.getByRole("rowheader", { name: "Wards" })).toBeVisible();
  await expect(
    page.getByRole("cell", { name: "No strategic aim" }),
  ).toBeVisible();
  await expect(page.getByText("no strategic aim", { exact: true })).toHaveCount(
    0,
  );
  await page.getByLabel("Month (empty for this month)").fill("2026-10");
  await page.getByRole("button", { name: "Show" }).click();
  await expect(
    page.getByRole("heading", { level: 2, name: "Portfolio map, 2026-10" }),
  ).toBeVisible();
  expect(asked).toEqual(["", "?period=2026-10"]);
});

test("an objective is marked and unmarked as a strategic aim", async ({
  page,
}) => {
  const { sent } = await stubMap(page);
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  const aims = page.getByRole("group", {
    name: "Objectives that are strategic aims",
  });
  await expect(
    aims.getByRole("checkbox", { name: "Better care" }),
  ).toBeChecked();
  await aims.getByRole("checkbox", { name: "Lower cost" }).check();
  await expect(page.getByRole("status")).toHaveText(
    "Marked as a strategic aim.",
  );
  await aims.getByRole("checkbox", { name: "Better care" }).uncheck();
  await expect(page.getByRole("status")).toHaveText(
    "No longer a strategic aim.",
  );
  await expect(
    aims.getByRole("checkbox", { name: "Better care" }),
  ).not.toBeChecked();
  expect(sent).toEqual([
    { path: "/api/objectives/o2/strategic-aim", body: { strategic: true } },
    { path: "/api/objectives/o1/strategic-aim", body: { strategic: false } },
  ]);
});

test("a programme's primary aim is set; its current one is said first; a refusal puts the box back", async ({
  page,
}) => {
  const { sent } = await stubMap(page);
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  const form = page.getByRole("region", { name: "A programme's primary aim" });
  await form.getByLabel("Programme").selectOption({ label: "Wards" });
  await form.getByLabel("Primary aim").selectOption({ label: "Better care" });
  await form.getByRole("button", { name: "Set the primary aim" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "That is already this programme's primary aim.",
  );
  expect(sent).toEqual([]);
  await form.getByLabel("Programme").selectOption({ label: "Apps" });
  await form.getByRole("button", { name: "Set the primary aim" }).click();
  await expect(page.getByRole("status")).toHaveText("Primary aim set.");
  expect(sent).toEqual([
    { path: "/api/plans/p2/primary-aim", body: { objective_pid: "o1" } },
  ]);

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubMap(page, { refuse: 403 });
  await page.reload();
  await settle(page);
  const aims = page.getByRole("group", {
    name: "Objectives that are strategic aims",
  });
  await aims.getByRole("checkbox", { name: "Lower cost" }).check();
  await expect(page.getByRole("alert")).toHaveText(
    "You are not allowed to do that.",
  );
  await expect(
    aims.getByRole("checkbox", { name: "Lower cost" }),
  ).not.toBeChecked();
});

test("the pack's programme readings are read now and drawn as the pack draws them, never in the service's words", async ({
  page,
}) => {
  const { live } = await stubMap(page);
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  const r = page.getByRole("region", { name: "Programme readings, now" });
  const fever = r.getByTestId("live-fever");
  await expect(
    fever.getByRole("rowheader", { name: "Theatres" }),
  ).toBeVisible();
  await expect(fever).toContainText("red");
  await expect(fever).toContainText(
    "Apps: no fever-chart point: no schedule baseline",
  );
  await expect(r.getByTestId("live-evm")).toContainText("£1,000.00");
  await expect(r.getByTestId("live-benefits")).toContainText("£5,000.00");
  await expect(r.getByTestId("live-issues")).toContainText(
    "Open by severity: high 3, low 1",
  );
  await expect(r.getByTestId("live-issues")).toContainText(
    "Oldest open issue: Supplier late (calendar days open: 40)",
  );
  await expect(r.getByTestId("live-changes")).toContainText(
    "Approved but not yet implemented: 1",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  expect([...live].sort()).toEqual([
    "/api/buffer-fever",
    "/api/director-pack/issues-and-changes",
    "/api/portfolio-benefits",
    "/api/portfolio-earned-value",
  ]);
});

test("a window of days is checked first; the window and actual cost chosen are asked of the service", async ({
  page,
}) => {
  const { live } = await stubMap(page);
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  const r = page.getByRole("region", { name: "Programme readings, now" });
  await expect(r.getByTestId("live-issues")).toBeVisible();
  const before = live.length;
  const days = r.getByLabel(
    "Issues and change requests over the last days (1 to 366; empty for 30)",
  );
  await days.fill("0");
  await r.getByRole("button", { name: "Read again" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "The number of days must be a whole number from 1 to 366, or empty for 30.",
  );
  expect(live.length).toBe(before);
  await days.fill("90");
  await r
    .getByLabel("Actual cost for earned value")
    .selectOption({ label: "Recorded labour cost" });
  await r.getByRole("button", { name: "Read again" }).click();
  await expect.poll(() => live.length).toBe(before + 4);
  expect(live.slice(before).sort()).toEqual([
    "/api/buffer-fever",
    "/api/director-pack/issues-and-changes?window_days=90",
    "/api/portfolio-benefits",
    "/api/portfolio-earned-value?ac=labour",
  ]);
});

test("a reading that fails says so in the catalogue's words; no readable plan with issues is said", async ({
  page,
}) => {
  await stubMap(page, { liveFail: "/api/buffer-fever", noIssues: true });
  await page.goto("/en-001/portfolio-map");
  await settle(page);
  const r = page.getByRole("region", { name: "Programme readings, now" });
  await expect(r.getByRole("alert")).toHaveText(
    "Could not read the buffer fever chart.",
  );
  await expect(r.getByTestId("live-fever")).toHaveCount(0);
  await expect(r.getByTestId("live-issues-none")).toHaveText(
    "You can read no plan with issues or change requests.",
  );
  await expect(r.getByTestId("live-evm")).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the map reads in Welsh and passes axe", async ({ page }) => {
  await stubMap(page);
  await page.goto("/cy-001/portfolio-map");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Map y portffolio",
  );
  await expect(
    page.getByRole("heading", {
      level: 2,
      name: "Darlleniadau'r rhaglenni, nawr",
    }),
  ).toBeVisible();
  await expect(page.getByTestId("live-issues")).toBeVisible();
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
