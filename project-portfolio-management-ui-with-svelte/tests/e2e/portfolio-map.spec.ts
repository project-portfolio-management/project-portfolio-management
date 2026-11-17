// NC-1 slice 11 (DP-4): the live portfolio map and strategic aims, in a real browser over a stubbed service. The map is
// drawn as the director pack draws it (the no-aim group in the catalogue's words); a period is asked of the service; an
// objective is marked and unmarked as a strategic aim, and a refusal puts the box back; a programme's primary aim is set,
// and choosing its current one is said first. Welsh; axe.

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

async function stubMap(page: Page, opts: { refuse?: number } = {}) {
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
  return { sent, asked };
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

test("the map reads in Welsh and passes axe", async ({ page }) => {
  await stubMap(page);
  await page.goto("/cy-001/portfolio-map");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Map y portffolio",
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
