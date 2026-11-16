// NC-1 slice 8 (RM-7): a plan's tools, in a real browser over a stubbed service. The plan page links to them; a chosen
// set, the personal preset and the deployment's default are each sent as the service takes them; choosing the default
// again on a plan that follows it sends nothing; a refusal says the catalogue's sentence; the plan page then hides the
// links of a module that is off, issues following risks as the service has it. Welsh reads. The page passes axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const ALL = [
  "tasks",
  "time",
  "risks",
  "financials",
  "milestones",
  "governance",
  "roadmap",
];
const PERSONAL = ["tasks", "time"];

async function stubModules(
  page: Page,
  opts: { status?: number; load?: number; modules?: string[] } = {},
) {
  let state = {
    source: opts.modules ? "plan" : "default",
    modules: opts.modules ?? ALL,
  };
  const view = () => ({
    plan_pid: PID,
    source: state.source,
    modules: state.modules,
    available: ALL.map((key) => ({
      key,
      enabled: state.modules.includes(key),
      segments: [],
    })),
    default: ALL,
    personal_preset: PERSONAL,
  });
  const sent: unknown[] = [];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/modules`, async (route) => {
    const req = route.request();
    if (req.method() === "GET") {
      if (opts.load)
        return route.fulfill({
          status: opts.load,
          json: { error: "x", description: "service text" },
        });
      return route.fulfill({ json: view() });
    }
    const body = req.postDataJSON() as {
      modules?: string[];
      preset?: string;
      reset?: boolean;
    };
    sent.push(body);
    if (opts.status)
      return route.fulfill({
        status: opts.status,
        json: { error: "forbidden", description: "service text" },
      });
    if (body.modules) state = { source: "plan", modules: body.modules };
    else if (body.preset) state = { source: "plan", modules: PERSONAL };
    else state = { source: "default", modules: ALL };
    return route.fulfill({ json: view() });
  });
  return sent;
}

test("the plan page leads to its tools, and a chosen set is saved as the plan's own", async ({
  page,
}) => {
  const sent = await stubModules(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Tools on this plan" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Tools on this plan" }),
  ).toBeVisible();
  await expect(
    page.getByText("This plan uses the deployment's default tools."),
  ).toBeVisible();
  const tools = page.getByRole("group", { name: "Tools this plan shows" });
  await expect(
    tools.getByRole("checkbox", { name: "Risks and issues" }),
  ).toBeChecked();
  await expect(
    page.getByRole("button", { name: "Follow the deployment's default" }),
  ).toBeDisabled();

  await tools
    .getByRole("checkbox", { name: "Budgets, costs and earned value" })
    .uncheck();
  await page.getByRole("button", { name: "Save tools" }).click();
  await expect(page.getByRole("status")).toHaveText("Saved.");
  expect(sent).toEqual([{ modules: ALL.filter((k) => k !== "financials") }]);
  await expect(
    page.getByText("This plan has its own choice of tools."),
  ).toBeVisible();
  await expect(
    tools.getByRole("checkbox", { name: "Budgets, costs and earned value" }),
  ).not.toBeChecked();
});

test("the personal set and the deployment's default are sent as the service takes them", async ({
  page,
}) => {
  const sent = await stubModules(page);
  await page.goto(`/en-001/plans/${PID}/modules`);
  await settle(page);
  await expect(
    page.getByText(
      "The personal set, for one person's own work: Tasks, the board and schedule analysis, Time recording.",
    ),
  ).toBeVisible();
  await page.getByRole("button", { name: "Use the personal set" }).click();
  await expect(page.getByRole("status")).toHaveText("Saved.");
  const tools = page.getByRole("group", { name: "Tools this plan shows" });
  await expect(tools.getByRole("checkbox", { checked: true })).toHaveCount(2);

  await page
    .getByRole("button", { name: "Follow the deployment's default" })
    .click();
  await expect(
    page.getByText("This plan uses the deployment's default tools."),
  ).toBeVisible();
  expect(sent).toEqual([{ preset: "personal" }, { reset: true }]);
});

test("choosing the default again on a plan that follows it sends nothing", async ({
  page,
}) => {
  const sent = await stubModules(page);
  await page.goto(`/en-001/plans/${PID}/modules`);
  await settle(page);
  const tools = page.getByRole("group", { name: "Tools this plan shows" });
  const time = tools.getByRole("checkbox", { name: "Time recording" });
  await time.uncheck();
  await time.check();
  await page.getByRole("button", { name: "Save tools" }).click();
  await expect(page.getByRole("status")).toContainText("Nothing to save");
  expect(sent).toEqual([]);

  // No tool at all is a choice; the page says what it means before it is sent.
  for (const box of await tools.getByRole("checkbox").all())
    await box.uncheck();
  await expect(page.getByRole("note")).toContainText("no tools");
  await page.getByRole("button", { name: "Save tools" }).click();
  await expect.poll(() => sent).toEqual([{ modules: [] }]);
});

test("a refusal and a failed load are said in the catalogue's words", async ({
  page,
}) => {
  await stubModules(page, { status: 403 });
  await page.goto(`/en-001/plans/${PID}/modules`);
  await settle(page);
  await page.getByRole("button", { name: "Use the personal set" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "You are not allowed to do that.",
  );
  await expect(page.getByRole("alert")).not.toContainText("service text");

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubModules(page, { load: 500 });
  await page.reload();
  await settle(page);
  await expect(page.getByRole("alert")).not.toContainText("service text");
  await expect(page.getByRole("button", { name: "Save tools" })).toHaveCount(0);
});

test("the plan page hides a module's links when it is off, and issues follow risks", async ({
  page,
}) => {
  await stubModules(page, { modules: ["governance", "tasks"] });
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await expect(
    page.getByRole("link", { name: "Governance", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Issues", exact: true }),
  ).toHaveCount(0);
  // Every tool off: the way to turn them back on is still there.
  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubModules(page, { modules: [] });
  await page.reload();
  await settle(page);
  await expect(
    page.getByRole("link", { name: "Governance", exact: true }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("link", { name: "Tools on this plan" }),
  ).toBeVisible();
});

test("the tools page reads in Welsh and passes axe", async ({ page }) => {
  await stubModules(page);
  await page.goto(`/cy-001/plans/${PID}/modules`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Offer ar y cynllun hwn",
  );
  await expect(
    page.getByRole("checkbox", { name: "Risgiau a materion" }),
  ).toBeChecked();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
