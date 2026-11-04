// OC-1: organization-defined issue types, statuses and roles in a real browser, over a stubbed service. An administrator
// adds a type and a role, retires a type and registers a workflow; a refusal says the catalogue's sentence. A plan's
// issue form then offers the organization's type, starts where its workflow says, offers only the moves this caller may
// make, and asks for a resolution in a terminal status whatever it is called. Welsh reads. The pages pass axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

async function stubSettings(page: Page, allow = true) {
  const types = [
    { key: "bug", label: "Bug", position: 0, retired: false },
    { key: "risk", label: "Risk", position: 1, retired: false },
  ];
  const roles: Record<string, unknown>[] = [];
  const workflows: Record<string, unknown>[] = [];
  const sent: { method: string; path: string; body: unknown }[] = [];
  const answer = async (
    route: import("@playwright/test").Route,
    list: Record<string, unknown>[],
  ) => {
    const req = route.request();
    const path = new URL(req.url()).pathname;
    if (req.method() === "GET") return route.fulfill({ json: list });
    const body = req.postDataJSON?.() ?? null;
    sent.push({ method: req.method(), path, body });
    if (!allow)
      return route.fulfill({
        status: 403,
        json: { error: "forbidden", description: "service text" },
      });
    if (req.method() === "POST") {
      list.push({
        retired: false,
        position: list.length,
        pid: "w1",
        applies_to: "issue",
        ...body,
      });
      return route.fulfill({ status: 201, json: body });
    }
    if (req.method() === "PUT") {
      const key = path.split("/").pop();
      const found = list.find((x) => x.key === key);
      if (found) Object.assign(found, body);
      return route.fulfill({ json: found ?? {} });
    }
    list.length = 0;
    return route.fulfill({ json: {} });
  };
  await page.route("**/api/issue-types**", (route) => answer(route, types));
  await page.route("**/api/organization-roles**", (route) =>
    answer(route, roles),
  );
  await page.route("**/api/workflows**", (route) => answer(route, workflows));
  return sent;
}

test("an administrator adds a type and a role, retires a type and registers a workflow", async ({
  page,
}) => {
  const sent = await stubSettings(page);
  await page.goto("/en-001/issue-settings");
  await settle(page);

  const types = page.getByRole("region", { name: "Issue types" });
  await types.getByLabel("Key").fill("incident");
  await types.getByLabel("Name").fill("Incident");
  await types.getByRole("button", { name: "Add type" }).click();
  await expect(
    types.getByRole("rowheader", { name: "Incident" }),
  ).toBeVisible();

  await types.getByRole("button", { name: "Retire: Risk" }).click();
  await expect(
    types.getByRole("button", { name: "Restore: Risk" }),
  ).toBeVisible();
  expect(sent.find((s) => s.method === "PUT")?.path).toContain(
    "/api/issue-types/risk",
  );

  const roles = page.getByRole("region", { name: "Roles" });
  await expect(roles.getByText("grants nothing")).toBeVisible();
  await roles.getByLabel("Key").fill("change-manager");
  await roles.getByLabel("Name").fill("Change manager");
  await roles.getByRole("button", { name: "Add role" }).click();
  await expect(
    roles.getByRole("rowheader", { name: "Change manager" }),
  ).toBeVisible();

  const flows = page.getByRole("region", { name: "Statuses and moves" });
  await flows.getByLabel("Workflow name").fill("Incidents");
  await flows.getByLabel("Applies to").selectOption("incident");
  await flows.getByRole("button", { name: "Add status" }).click();
  await flows.getByLabel("Key").nth(2).fill("with_supplier");
  await flows.getByLabel("Name", { exact: true }).nth(2).fill("With supplier");
  await flows.getByLabel("Meaning").nth(2).selectOption("waiting");
  await flows.getByRole("button", { name: "Add move" }).click();
  await flows.getByLabel("Roles (comma-separated").fill("change-manager");
  await flows.getByRole("button", { name: "Register workflow" }).click();

  await expect(
    flows.getByRole("rowheader", { name: "Incidents" }),
  ).toBeVisible();
  const posted = sent.find((s) => s.path.endsWith("/api/workflows"))
    ?.body as Record<string, unknown>;
  expect(posted).toMatchObject({
    name: "Incidents",
    applies_to: "issue",
    issue_type: "incident",
    is_default: true,
    transitions: [{ from: "open", to: "closed", roles: ["change-manager"] }],
  });
  expect(posted.states).toContainEqual({
    key: "with_supplier",
    label: "With supplier",
    category: "waiting",
    is_initial: false,
    is_terminal: false,
  });
});

test("someone who is not an administrator is told so in the catalogue's words", async ({
  page,
}) => {
  await stubSettings(page, false);
  await page.goto("/en-001/issue-settings");
  await settle(page);
  const types = page.getByRole("region", { name: "Issue types" });
  await types.getByLabel("Key").fill("incident");
  await types.getByLabel("Name").fill("Incident");
  await types.getByRole("button", { name: "Add type" }).click();
  const alert = page.getByRole("alert");
  await expect(alert).toHaveText("You are not allowed to do that.");
});

test("the settings page reads in Welsh and passes axe", async ({ page }) => {
  await stubSettings(page);
  await page.goto("/cy-001/issue-settings");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toContainText(
    "Mathau, statwsau a rolau materion",
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

const workflow = {
  states: [
    {
      key: "new",
      label: "New",
      category: "todo",
      is_initial: true,
      is_terminal: false,
    },
    {
      key: "fixed",
      label: "Fixed",
      category: "done",
      is_initial: false,
      is_terminal: false,
    },
    {
      key: "signed_off",
      label: "Signed off",
      category: "done",
      is_initial: false,
      is_terminal: true,
    },
  ],
  transitions: [
    { from: "new", to: "fixed", roles: [] },
    { from: "fixed", to: "signed_off", roles: ["change-manager"] },
  ],
};

async function stubPlan(page: Page, callerRoles: string[]) {
  const issue = {
    pid: "i1",
    plan_pid: PID,
    title: "Signal failure",
    description: null,
    kind: "incident",
    severity: "high",
    priority: null,
    status: "fixed",
    status_category: "done",
    owner_ref: null,
    raised_by: null,
    raised_on: "2026-10-01",
    due_date: null,
    resolution: null,
    resolved_on: "2026-10-02",
    age_calendar_days: 1,
    overdue: false,
  };
  const sent: unknown[] = [];
  await page.route(`**/api/plans/${PID}/issues**`, async (route) => {
    const req = route.request();
    if (new URL(req.url()).pathname.endsWith("/options")) {
      return route.fulfill({
        json: {
          caller_roles: callerRoles,
          types: [
            { key: "bug", label: "Bug", position: 0, retired: false, workflow },
            {
              key: "incident",
              label: "Incident",
              position: 1,
              retired: false,
              workflow,
            },
          ],
        },
      });
    }
    if (req.method() === "GET") {
      return route.fulfill({
        json: {
          plan_pid: PID,
          issues: [issue],
          total: 1,
          open: 0,
          overdue: 0,
          as_of: "2026-11-04",
        },
      });
    }
    sent.push(req.postDataJSON?.());
    return route.fulfill({ json: issue });
  });
  await page.route(`**/api/plans/${PID}/risks**`, (route) =>
    route.fulfill({ json: [] }),
  );
  return sent;
}

test("the issue form uses the organization's types and statuses, and only the caller's moves", async ({
  page,
}) => {
  await stubPlan(page, []);
  await page.goto(`/en-001/plans/${PID}/issues`);
  await settle(page);
  await expect(page.getByRole("cell", { name: "Incident" })).toBeVisible();
  await expect(page.getByRole("cell", { name: "Fixed" })).toBeVisible();

  // A new issue starts where the workflow says: nothing else is offered.
  const status = page.locator("form.issue-form").first().getByLabel("Status");
  await expect(status.locator("option")).toHaveText(["New"]);
  await expect(
    page
      .locator("form.issue-form")
      .first()
      .getByLabel("Kind")
      .locator("option"),
  ).toContainText(["bug", "Incident"]);

  // Without the role, the sign-off is not offered.
  await page.getByRole("button", { name: /Signal failure/ }).click();
  await expect(status.locator("option")).toHaveText(["Fixed"]);
});

test("with the role, the sign-off is offered and asks for a resolution", async ({
  page,
}) => {
  const sent = await stubPlan(page, ["change-manager"]);
  await page.goto(`/en-001/plans/${PID}/issues`);
  await settle(page);
  await page.getByRole("button", { name: /Signal failure/ }).click();
  const form = page.locator("form.issue-form").first();
  const status = form.getByLabel("Status");
  await expect(status.locator("option")).toHaveText(["Fixed", "Signed off"]);
  await status.selectOption("signed_off");
  await form.getByRole("button", { name: "Save changes" }).click();
  await expect(page.getByRole("alert")).toBeVisible();
  expect(sent).toEqual([]);
});
