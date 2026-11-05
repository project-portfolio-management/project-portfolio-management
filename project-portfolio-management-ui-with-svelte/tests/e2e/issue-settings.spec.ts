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
  await types.getByLabel("Name", { exact: true }).fill("Incident");
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
  await expect(roles.getByText("can only narrow")).toBeVisible();
  await roles.getByLabel("Key").fill("change-manager");
  await roles.getByLabel("Name", { exact: true }).fill("Change manager");
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
    labels: {},
    category: "waiting",
    is_initial: false,
    is_terminal: false,
    field_rules: [],
  });
});

test("a role's permissions and translations are saved, and a bad translation is refused here", async ({
  page,
}) => {
  const sent = await stubSettings(page);
  await page.goto("/en-001/issue-settings");
  await settle(page);
  const roles = page.getByRole("region", { name: "Roles" });
  await roles.getByLabel("Key").fill("triager");
  await roles.getByLabel("Name", { exact: true }).fill("Triager");
  await roles
    .getByRole("checkbox", { name: "Raise issues", exact: true })
    .check();
  await roles.getByRole("button", { name: "Add role" }).click();
  await expect(roles.getByRole("rowheader", { name: "Triager" })).toBeVisible();
  expect(
    sent.find((s) => s.path.endsWith("/organization-roles"))?.body,
  ).toMatchObject({
    permissions: ["issue_add"],
  });

  await roles
    .getByRole("checkbox", { name: "Move issues: Triager", exact: true })
    .check();
  await roles
    .getByLabel(
      "Translations (language=name, separated by semicolons): Triager",
    )
    .fill("cy=Brysbennydd");
  await roles.getByRole("button", { name: "Save: Triager" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "Saved." }),
  ).toBeVisible();
  expect(sent.at(-1)?.body).toMatchObject({
    labels: { cy: "Brysbennydd" },
    permissions: ["issue_add", "issue_move"],
  });

  const types = page.getByRole("region", { name: "Issue types" });
  const count = sent.length;
  await types
    .getByLabel("Translations (language=name, separated by semicolons): Bug")
    .fill("nonsense");
  await types.getByRole("button", { name: "Save: Bug" }).click();
  await expect(page.getByRole("alert")).toContainText("language=name");
  expect(sent.length).toBe(count);
});

test("a workflow is changed in place, with a field rule and a translated status", async ({
  page,
}) => {
  const sent = await stubSettings(page);
  await page.route("**/api/workflows/w9", async (route) => {
    const req = route.request();
    if (req.method() === "GET")
      return route.fulfill({
        json: {
          pid: "w9",
          name: "Triage",
          applies_to: "issue",
          issue_type: null,
          plan_pid: null,
          is_default: true,
          workflow,
        },
      });
    sent.push({
      method: req.method(),
      path: "/api/workflows/w9",
      body: req.postDataJSON(),
    });
    return route.fulfill({ json: { pid: "w9" } });
  });
  await page.route("**/api/workflows", (route) =>
    route.request().method() === "GET"
      ? route.fulfill({
          json: [
            {
              pid: "w9",
              name: "Triage",
              applies_to: "issue",
              issue_type: null,
              plan_pid: null,
              is_default: true,
            },
          ],
        })
      : route.fallback(),
  );
  await page.goto("/en-001/issue-settings");
  await settle(page);
  const flows = page.getByRole("region", { name: "Statuses and moves" });
  await flows.getByRole("button", { name: "Edit: Triage" }).click();
  await expect(flows.getByText("Changing Triage")).toBeVisible();
  await expect(flows.getByLabel("Workflow name")).toHaveValue("Triage");
  await expect(flows.getByLabel("Applies to")).toBeDisabled();
  await flows
    .getByLabel(/^Translations/)
    .first()
    .fill("cy=Newydd");
  await flows.getByRole("button", { name: "Add field rule: New" }).click();
  await flows.getByLabel(/^Field/).selectOption("due_date");
  await flows.getByRole("button", { name: "Save workflow" }).click();
  await expect(flows.getByText("Changing Triage")).toHaveCount(0);
  const change = sent.find((s) => s.method === "PUT")?.body as {
    states: Record<string, unknown>[];
    transitions: unknown[];
  };
  expect(change.states[0]).toMatchObject({
    key: "new",
    labels: { cy: "Newydd" },
    is_initial: true,
    field_rules: [{ field: "due_date", rule: "required", roles: [] }],
  });
  expect(change.transitions).toEqual(workflow.transitions);
});

test("someone who is not an administrator is told so in the catalogue's words", async ({
  page,
}) => {
  await stubSettings(page, false);
  await page.goto("/en-001/issue-settings");
  await settle(page);
  const types = page.getByRole("region", { name: "Issue types" });
  await types.getByLabel("Key").fill("incident");
  await types.getByLabel("Name", { exact: true }).fill("Incident");
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

async function stubPlan(
  page: Page,
  callerRoles: string[],
  standing: Record<string, unknown> = {},
) {
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
          ...standing,
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

test("the form follows the caller's permissions and the status's field rules", async ({
  page,
}) => {
  const ruled = structuredClone(workflow);
  ruled.states[1]!.field_rules = [
    { field: "title", rule: "read_only", roles: [] },
    { field: "owner_ref", rule: "required", roles: [] },
  ] as never;
  const sent = await stubPlan(page, []);
  await page.route(`**/api/plans/${PID}/issues/options`, (route) =>
    route.fulfill({
      json: {
        caller_roles: [],
        caller_permissions: ["issue_edit"],
        enforced: true,
        types: [
          {
            key: "incident",
            label: "Incident",
            position: 0,
            retired: false,
            offered: true,
            workflow: ruled,
          },
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/issues`);
  await settle(page);
  // No `issue_add`: nothing to raise with.
  await expect(
    page.getByRole("heading", { name: "Raise an issue from a risk" }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: /Signal failure/ }).click();
  const form = page.locator("form.issue-form").first();
  await expect(form.getByLabel("Title")).toBeDisabled();
  await expect(form.getByLabel("Status")).toBeDisabled();
  await form.getByRole("button", { name: "Save changes" }).click();
  await expect(form.getByLabel("Owner")).toHaveAttribute("required", "");
  expect(sent).toEqual([]);
});

test("a plan's settings name who holds what, and choose its types and workflow", async ({
  page,
}) => {
  const members = [
    {
      pid: "m1",
      plan_pid: PID,
      person_ref: "3f2c6b7e-0000-4000-8000-0000000001a1",
      role_key: "triager",
      added_by: null,
      added_at: "2026-11-05T00:00:00Z",
    },
  ];
  const sent: { method: string; path: string; body: unknown }[] = [];
  await stubSettings(page);
  await page.route("**/api/organization-roles**", (route) =>
    route.fulfill({
      json: [
        {
          key: "triager",
          label: "Triager",
          description: null,
          position: 0,
          retired: false,
        },
        {
          key: "closer",
          label: "Closer",
          description: null,
          position: 1,
          retired: false,
        },
      ],
    }),
  );
  await page.route(`**/api/plans/${PID}/role-members**`, async (route) => {
    const req = route.request();
    if (req.method() === "GET")
      return route.fulfill({
        json: {
          plan_pid: PID,
          enforced: members.length > 0,
          members,
          truncated: false,
        },
      });
    sent.push({
      method: req.method(),
      path: new URL(req.url()).pathname,
      body: req.postDataJSON?.() ?? null,
    });
    if (req.method() === "DELETE") {
      members.length = 0;
      return route.fulfill({ json: {} });
    }
    return route.fulfill({ status: 201, json: {} });
  });
  await page.route(`**/api/plans/${PID}/issue-types`, async (route) => {
    const req = route.request();
    if (req.method() === "GET")
      return route.fulfill({
        json: { plan_pid: PID, limited: true, types: ["bug"] },
      });
    sent.push({
      method: req.method(),
      path: new URL(req.url()).pathname,
      body: req.postDataJSON(),
    });
    return route.fulfill({
      json: { plan_pid: PID, limited: true, types: ["bug"] },
    });
  });
  await page.goto(`/en-001/plans/${PID}/issue-settings`);
  await settle(page);
  await expect(
    page.getByText("Issue actions on this plan need a role."),
  ).toBeVisible();

  const people = page.getByRole("region", { name: "People and roles" });
  await people
    .getByLabel("Person's user id")
    .fill("3f2c6b7e-0000-4000-8000-0000000001b2");
  await people.getByLabel(/^Role/).selectOption("closer");
  await people.getByRole("button", { name: "Add person" }).click();
  await expect
    .poll(() => sent.at(-1)?.body)
    .toEqual({
      person_ref: "3f2c6b7e-0000-4000-8000-0000000001b2",
      role_key: "closer",
    });
  await people.getByRole("button", { name: /Remove: .*Triager/ }).click();
  await expect(
    page.getByText("Anyone who can write can act on issues here."),
  ).toBeVisible();

  const offered = page.getByRole("region", {
    name: "Issue types on this plan",
  });
  await expect(offered.getByRole("checkbox", { name: "Bug" })).toBeChecked();
  await offered.getByRole("checkbox", { name: "Risk" }).check();
  await offered.getByRole("button", { name: "Save types" }).click();
  await expect
    .poll(() => sent.at(-1)?.body)
    .toEqual({ types: ["bug", "risk"] });

  const flows = page.getByRole("region", { name: "Statuses and moves" });
  await flows.getByLabel("Workflow name").fill("This plan");
  await flows.getByRole("button", { name: "Register workflow" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "Saved." }),
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

test("the register shows a status's own words, and in the reader's language", async ({
  page,
}) => {
  const issue = {
    pid: "i1",
    plan_pid: PID,
    title: "Waiting on the vendor",
    description: null,
    kind: "bug",
    severity: "high",
    priority: null,
    status: "with_supplier",
    status_category: "waiting",
    status_label: "At the vendor",
    status_labels: { cy: "Gyda'r gwerthwr" },
    owner_ref: null,
    raised_by: null,
    raised_on: "2026-10-01",
    due_date: null,
    resolution: null,
    resolved_on: null,
    age_calendar_days: 1,
    overdue: false,
  };
  await page.route("**/api/issues**", (route) =>
    route.fulfill({
      json: {
        issues: [issue],
        matching: 1,
        truncated: false,
        summary: {
          by_severity: { high: 1 },
          by_status: { open: 0, with_supplier: 1 },
          overdue: 0,
          oldest_open: null,
          status_words: {
            with_supplier: {
              label: "At the vendor",
              labels: { cy: "Gyda'r gwerthwr" },
            },
          },
        },
        as_of: "2026-11-05",
        note: "",
      },
    }),
  );
  await page.route("**/api/issue-types**", (route) =>
    route.fulfill({ json: [] }),
  );
  await page.goto("/en-001/issues");
  await settle(page);
  await expect(page.getByRole("cell", { name: "At the vendor" })).toBeVisible();
  await page.goto("/cy-001/issues");
  await settle(page);
  await expect(
    page.getByRole("cell", { name: "Gyda'r gwerthwr" }),
  ).toBeVisible();
});
