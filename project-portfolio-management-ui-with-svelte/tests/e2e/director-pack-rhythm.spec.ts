// NC-1 slice 11 (DP-12): the monthly director-pack schedules and the subscriptions, in a real browser over a stubbed
// service. Schedules are listed with their next run and last result in the reader's language; one is added (a day from
// 1 to 28, said first), moved, switched off; what is due is run, which drafts and never issues; a reader without
// editor rights is told which right is needed. On the pack list a reader chooses the audiences to be told about.
// Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const S1 = "55555555-5555-4555-8555-555555555551";

async function stubRhythm(
  page: Page,
  opts: { refuse?: number; listRefuse?: number } = {},
) {
  const schedules = [
    {
      pid: S1,
      audience: "board",
      day_of_month: 3,
      next_run_date: "2026-12-03",
      enabled: true,
      last_run_on: "2026-11-03",
      last_result: "drafted 2026-10",
    },
  ];
  let subscribed = ["board"];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await page.route("**/api/organizations", (route) =>
    route.fulfill({
      json: {
        organizations: [
          {
            pid: "o1",
            name: "Northwind Trust",
            kind: "partner",
            active: true,
            audience: "org:o1",
          },
        ],
      },
    }),
  );
  await page.route("**/api/director-packs", (route) =>
    route.fulfill({ json: { packs: [] } }),
  );
  await page.route(
    /\/api\/director-pack-(schedules|subscriptions)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      if (method === "GET") {
        if (path.endsWith("subscriptions"))
          return route.fulfill({ json: { subscribed_to: subscribed } });
        if (opts.listRefuse)
          return route.fulfill({
            status: opts.listRefuse,
            json: { error: "x", description: "service text" },
          });
        return route.fulfill({ json: { schedules } });
      }
      const body = req.postDataJSON() ?? null;
      sent.push({ method, path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { error: "x", description: "service text" },
        });
      if (path.endsWith("subscriptions")) {
        subscribed = body.subscribed
          ? [...subscribed, body.audience]
          : subscribed.filter((a) => a !== body.audience);
        return route.fulfill({ json: body });
      }
      if (path.endsWith("/sweep"))
        return route.fulfill({
          json: { drafted: ["x", "y"], skipped: 1, issued: 0 },
        });
      if (method === "POST") {
        const row = {
          pid: "55555555-5555-4555-8555-555555555552",
          audience: body.audience,
          day_of_month: body.day_of_month,
          next_run_date: "2026-12-10",
          enabled: true,
          last_run_on: null,
          last_result: null,
        };
        schedules.push(row);
        return route.fulfill({ json: row });
      }
      const row = schedules.find((s) => path.endsWith(s.pid))!;
      Object.assign(row, body);
      return route.fulfill({ json: row });
    },
  );
  return sent;
}

test("schedules show their next run and last result, and one is added after its day is checked", async ({
  page,
}) => {
  const sent = await stubRhythm(page);
  await page.goto("/en-001/director-packs");
  await settle(page);
  await page.getByRole("link", { name: "Monthly schedules" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Monthly schedules" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { level: 2, name: "Board", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText(/Drafts on day 3 of each month; next on/),
  ).toBeVisible();
  await expect(page.getByText(/drafted the pack for 2026-10/)).toBeVisible();

  const add = page.getByRole("region", { name: "Add a schedule" });
  // An audience that already has a schedule is not offered again.
  await expect(
    add.getByRole("option", { name: "Board", exact: true }),
  ).toHaveCount(0);
  await add.getByLabel("Audience").selectOption("org:o1");
  await add.getByLabel("Day of the month").fill("29");
  await add.getByRole("button", { name: "Add the schedule" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Choose a day of the month from 1 to 28.",
  );
  expect(sent).toEqual([]);
  await add.getByLabel("Day of the month").fill("10");
  await add.getByRole("button", { name: "Add the schedule" }).click();
  await expect(page.getByRole("status")).toHaveText("Schedule added.");
  await expect(
    page.getByRole("heading", {
      level: 2,
      name: "Organization: Northwind Trust",
    }),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/director-pack-schedules",
      body: { audience: "org:o1", day_of_month: 10 },
    },
  ]);
});

test("a schedule is moved and switched off, and running what is due drafts without issuing", async ({
  page,
}) => {
  const sent = await stubRhythm(page);
  await page.goto("/en-001/director-packs/schedules");
  await settle(page);
  await page.getByRole("button", { name: "Move to this day: Board" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "That is already its day, so nothing was sent.",
  );
  await page.getByLabel("Day of the month").first().fill("5");
  await page.getByRole("button", { name: "Move to this day: Board" }).click();
  await expect(page.getByRole("status")).toHaveText("Saved.");
  await page.getByRole("button", { name: "Switch off: Board" }).click();
  await expect(
    page.getByText("Switched off. It would draft on day 5 of each month."),
  ).toBeVisible();
  await page.getByRole("button", { name: "Run the due schedules" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "Drafted: 2; skipped because a pack already existed: 1.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/director-pack-schedules/${S1}`,
      body: { day_of_month: 5 },
    },
    {
      method: "PUT",
      path: `/api/director-pack-schedules/${S1}`,
      body: { enabled: false },
    },
    { method: "POST", path: "/api/director-pack-schedules/sweep", body: {} },
  ]);
});

test("a reader without editor rights is told which right the schedules need", async ({
  page,
}) => {
  await stubRhythm(page, { listRefuse: 403 });
  await page.goto("/en-001/director-packs/schedules");
  await settle(page);
  await expect(page.getByRole("alert")).toHaveText(
    "Drafting, reviewing and reissuing a pack needs director pack editor access.",
  );
  await expect(
    page.getByRole("button", { name: "Run the due schedules" }),
  ).toHaveCount(0);
});

test("on the pack list a reader chooses the audiences to be told about; a refusal puts the box back", async ({
  page,
}) => {
  const sent = await stubRhythm(page);
  await page.goto("/en-001/director-packs");
  await settle(page);
  const subs = page.getByRole("group", { name: "Packs to be told about" });
  await expect(
    subs.getByRole("checkbox", { name: "Board", exact: true }),
  ).toBeChecked();
  await subs
    .getByRole("checkbox", { name: "Organization: Northwind Trust" })
    .check();
  await expect(
    page.getByRole("status").filter({ hasText: "You will be told." }),
  ).toBeVisible();
  await subs.getByRole("checkbox", { name: "Board", exact: true }).uncheck();
  await expect(
    page.getByRole("status").filter({ hasText: "You will no longer be told." }),
  ).toBeVisible();
  expect(sent.map((s) => s.body)).toEqual([
    { audience: "org:o1", subscribed: true },
    { audience: "board", subscribed: false },
  ]);

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubRhythm(page, { refuse: 403 });
  await page.reload();
  await settle(page);
  await page
    .getByRole("group", { name: "Packs to be told about" })
    .getByRole("checkbox", { name: "Executive" })
    // A click, not `.check()`: the refusal puts the box back at once, and `.check()` fails when it reads the box
    // as unchanged (a race on a fast runner) although that is the behaviour under test.
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "You can be told only about the audiences whose packs you may read.",
  );
  await expect(
    page
      .getByRole("group", { name: "Packs to be told about" })
      .getByRole("checkbox", { name: "Executive" }),
  ).not.toBeChecked();
});

test("the schedules read in Welsh and pass axe", async ({ page }) => {
  await stubRhythm(page);
  await page.goto("/cy-001/director-packs/schedules");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Amserlenni misol",
  );
  await expect(
    page.getByRole("heading", { level: 2, name: "Bwrdd" }),
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
