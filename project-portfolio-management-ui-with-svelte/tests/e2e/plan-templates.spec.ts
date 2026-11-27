// NC-1 slice 9 (LT-2): plan templates, in a real browser over a stubbed service. The list shows saved templates and the
// built-in starters; a template's page shows what it holds and starts plans from it (a start date only when it carries
// dates, a parent only when chosen); a plan's page saves it as a template, a taken name said in the catalogue's words;
// a saved template is removed after a question and a starter cannot be. Every problem is said before anything is sent.
// Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const SAVED = "66666666-6666-4666-8666-666666666666";
const STARTER = "77777777-7777-4777-8777-777777777777";
const NEW_ROOT = "88888888-8888-4888-8888-888888888888";

const node = (
  key: number,
  parent: number | null,
  name: string,
  tasks: number,
  milestones: Array<[string, number]>,
) => ({
  key,
  parent,
  plan: { name },
  start_offset_days: 0,
  target_offset_days: null,
  components: [],
  tasks: Array.from({ length: tasks }, (_, i) => ({
    key: i,
    title: `Task ${i}`,
    description: null,
    points: null,
    flow_type: null,
    due_offset_days: null,
    component: null,
    parent: null,
  })),
  milestones: milestones.map(([name, offset_days]) => ({
    name,
    kind: null,
    offset_days,
  })),
});

const TEMPLATES = {
  [SAVED]: {
    pid: SAVED,
    name: "Replatform kit",
    description: "Our usual shape.",
    version: 2,
    created_by: "person:1",
    source_plan_pid: PID,
    created_at: "2026-11-01T00:00:00Z",
    updated_at: "2026-11-01T00:00:00Z",
    snapshot: {
      base_date: "2026-01-05",
      has_offsets: true,
      nodes: [
        node(0, null, "Replatform", 3, [["Go live", 90]]),
        node(1, 0, "Data move", 1, []),
      ],
    },
  },
  [STARTER]: {
    pid: STARTER,
    name: "Small project",
    description: "A plan with a few tasks.",
    version: 1,
    created_by: "system:starter",
    source_plan_pid: null,
    created_at: "2026-11-01T00:00:00Z",
    updated_at: "2026-11-01T00:00:00Z",
    snapshot: {
      base_date: null,
      has_offsets: false,
      nodes: [node(0, null, "Small project", 2, [])],
    },
  },
};

async function stubTemplates(
  page: Page,
  opts: { saveStatus?: number; omitted?: number; reviseStatus?: number } = {},
) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  const live = new Set([SAVED, STARTER]);
  await stubPlansApi(page);
  await page.route(
    /\/api\/(plan-templates|plans\/from-template|plans\/[^/]+\/save-as-template)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      if (method !== "GET")
        sent.push({ method, path, body: req.postDataJSON() ?? null });
      if (path === "/api/plan-templates")
        return route.fulfill({
          json: [...live].map((pid) => {
            const t = TEMPLATES[pid as keyof typeof TEMPLATES];
            return {
              pid,
              name: t.name,
              description: t.description,
              starter: pid === STARTER,
              plans: t.snapshot.nodes.length,
              needs_start_date: t.snapshot.has_offsets,
            };
          }),
        });
      if (/^\/api\/plan-templates\/[^/]+\/plans$/.test(path))
        return route.fulfill({
          json: [
            {
              plan_pid: PID,
              name: "Website replatform",
              synced_version: 1,
              template_version: 2,
              changes_waiting: 3,
            },
            {
              plan_pid: NEW_ROOT,
              name: "Second rollout",
              synced_version: 2,
              template_version: 2,
              changes_waiting: 0,
            },
          ],
        });
      if (/^\/api\/plan-templates\/[^/]+\/revise$/.test(path))
        return opts.reviseStatus
          ? route.fulfill({
              status: opts.reviseStatus,
              json: { code: "unprocessable_entity", detail: "service text" },
            })
          : route.fulfill({
              json: {
                pid: SAVED,
                version: 3,
                omitted_unreadable: opts.omitted ?? 0,
              },
            });
      const one = path.match(/^\/api\/plan-templates\/([^/]+)$/);
      if (one) {
        if (!live.has(one[1]!))
          return route.fulfill({ status: 404, json: { error: "not_found" } });
        if (method === "DELETE") {
          live.delete(one[1]!);
          return route.fulfill({ json: {} });
        }
        return route.fulfill({
          json: TEMPLATES[one[1] as keyof typeof TEMPLATES],
        });
      }
      if (path.startsWith("/api/plans/from-template/"))
        return route.fulfill({
          json: {
            root_pid: NEW_ROOT,
            plans: [NEW_ROOT, "x"],
            tasks_created: 4,
            milestones_created: 1,
          },
        });
      if (opts.saveStatus)
        return route.fulfill({
          status: opts.saveStatus,
          json: { error: "x", description: "service text" },
        });
      return route.fulfill({
        json: {
          pid: SAVED,
          name: "Replatform kit",
          plans: 2,
          tasks: 4,
          milestones: 1,
          needs_start_date: true,
          omitted_unreadable: opts.omitted ?? 0,
        },
      });
    },
  );
  return sent;
}

test("the list shows saved templates and the built-in starters", async ({
  page,
}) => {
  await stubTemplates(page);
  await page.goto("/en-001/plan-templates");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Plan templates" }),
  ).toBeVisible();
  const saved = page.getByRole("region", { name: "Saved templates" });
  await expect(
    saved.getByRole("link", { name: "Replatform kit" }),
  ).toBeVisible();
  await expect(saved.getByText("2 plans · needs a start date")).toBeVisible();
  const starters = page.getByRole("region", { name: "Built-in starters" });
  await expect(
    starters.getByRole("link", { name: "Small project" }),
  ).toBeVisible();
  await expect(starters.getByText("1 plan", { exact: true })).toBeVisible();
});

test("a template with dates shows what it holds and needs a start date to start plans", async ({
  page,
}) => {
  const sent = await stubTemplates(page);
  await page.goto(`/en-001/plan-templates/${SAVED}`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Replatform kit" }),
  ).toBeVisible();
  await expect(page.getByText("A saved template, version 2.")).toBeVisible();
  await expect(page.getByText("3 tasks")).toBeVisible();
  await expect(page.getByText("Go live: day 90")).toBeVisible();
  await expect(page.getByText("1 task", { exact: true })).toBeVisible();

  await page.getByRole("button", { name: "Start the plans" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Choose a start date: this template carries dates.",
  );
  expect(sent).toEqual([]);

  await page
    .getByLabel("Name of the top plan (optional)")
    .fill("  Replatform 2027 ");
  await page.getByLabel("Start date").fill("2027-01-04");
  await page.getByLabel("Put it under").selectOption(PID);
  await page.getByRole("button", { name: "Start the plans" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Plans: 2; tasks: 4; milestones: 1.",
  );
  await expect(
    page.getByRole("link", { name: "Open the new plan" }),
  ).toHaveAttribute("href", new RegExp(`/plans/${NEW_ROOT}$`));
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/plans/from-template/${SAVED}`,
      body: {
        name: "Replatform 2027",
        start_date: "2027-01-04",
        parent_pid: PID,
      },
    },
  ]);
});

test("a starter without dates sends no start date and cannot be removed", async ({
  page,
}) => {
  const sent = await stubTemplates(page);
  await page.goto(`/en-001/plan-templates/${STARTER}`);
  await settle(page);
  await expect(
    page.getByText(/A built-in starter\. It cannot be changed or removed/),
  ).toBeVisible();
  await expect(page.getByLabel("Start date")).toHaveCount(0);
  await expect(
    page.getByText(
      "This template carries no dates, so no start date is needed.",
    ),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Remove this template" }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Start the plans" }).click();
  await expect(page.getByRole("status")).toContainText("Made.");
  expect(sent.map((s) => s.body)).toEqual([{}]);
});

test("a saved template is removed after a question, and the list no longer has it", async ({
  page,
}) => {
  const sent = await stubTemplates(page);
  await page.goto(`/en-001/plan-templates/${SAVED}`);
  await settle(page);
  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Remove this template" }).click();
  await expect(ask).toContainText("Replatform kit");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await page.getByRole("button", { name: "Remove this template" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Plan templates" }),
  ).toBeVisible();
  await expect(page.getByRole("link", { name: "Replatform kit" })).toHaveCount(
    0,
  );
  expect(sent).toEqual([
    { method: "DELETE", path: `/api/plan-templates/${SAVED}`, body: null },
  ]);
});

test("a plan's page saves it as a template: a blank name is said first, a taken name in the catalogue's words", async ({
  page,
}) => {
  const sent = await stubTemplates(page, { saveStatus: 409 });
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Save as a template" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Save as a template" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Save the template" }).click();
  await expect(page.getByRole("alert")).toHaveText("Give the template a name.");
  expect(sent).toEqual([]);
  await page.getByLabel("Template name").fill(" Replatform kit ");
  await page.getByRole("button", { name: "Save the template" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A template with this name already exists. Choose another name.",
  );
  await expect(page.getByRole("alert")).not.toContainText("service text");
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/save-as-template`,
      body: { name: "Replatform kit" },
    },
  ]);
});

test("saving says what was kept and what was left out, and links to the template", async ({
  page,
}) => {
  await stubTemplates(page, { omitted: 1 });
  await page.goto(`/en-001/plans/${PID}/save-as-template`);
  await settle(page);
  await page.getByLabel("Template name").fill("Replatform kit");
  await page.getByLabel("Description (optional)").fill("Our usual shape.");
  await page.getByRole("button", { name: "Save the template" }).click();
  const status = page.getByRole("status");
  await expect(status).toContainText(
    "Saved. Plans: 2; tasks: 4; milestones: 1.",
  );
  await expect(status).toContainText(
    "1 plan under this one was left out, because you cannot read it.",
  );
  await expect(
    status.getByRole("link", { name: "Open the template" }),
  ).toHaveAttribute("href", new RegExp(`/plan-templates/${SAVED}$`));
});

test("a template reads in Welsh and passes axe", async ({ page }) => {
  await stubTemplates(page);
  await page.goto(`/cy-001/plan-templates/${SAVED}`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 2 }).first()).toHaveText(
    "Beth sydd ynddo",
  );
  await expect(page.getByText("Templed wedi'i gadw, fersiwn 2.")).toBeVisible();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});

test("a template lists the plans made from it and is revised from a plan tree after a question (NC-12)", async ({
  page,
}) => {
  const sent = await stubTemplates(page, { omitted: 1 });
  await page.goto(`/en-001/plan-templates/${SAVED}`);
  await settle(page);
  const made = page.getByTestId("made-from");
  await expect(made.getByRole("listitem")).toHaveCount(2);
  await expect(made.getByRole("listitem").nth(0)).toContainText(
    "version 1 of 2 · changes waiting: 3",
  );
  await expect(
    made.getByRole("link", { name: "See the changes" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/template`);
  await expect(made.getByRole("listitem").nth(1)).toContainText("up to date");
  const revise = page.getByRole("region", { name: "Revise it from a plan" });
  await revise.getByRole("button", { name: "Revise the template" }).click();
  await expect(revise.getByRole("alert")).toHaveText(
    "Choose the plan tree first.",
  );
  await revise
    .getByLabel("Plan tree")
    .selectOption({ label: "Website replatform" });
  const ask = page.getByRole("alertdialog");
  await revise.getByRole("button", { name: "Revise the template" }).click();
  await expect(ask).toContainText(
    "Make the tasks of “Website replatform” the next version of “Replatform kit”?",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await revise.getByRole("button", { name: "Revise the template" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("revised")).toContainText("Now version 3.");
  await expect(page.getByTestId("revised")).toContainText(
    "Plans in the tree you cannot read were left out: 1.",
  );
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/plan-templates/${SAVED}/revise`,
      body: { from_plan: PID },
    },
  ]);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("a revision that would reshape the tree is refused in the catalogue's words", async ({
  page,
}) => {
  await stubTemplates(page, { reviseStatus: 422 });
  await page.goto(`/en-001/plan-templates/${SAVED}`);
  await settle(page);
  const revise = page.getByRole("region", { name: "Revise it from a plan" });
  await revise
    .getByLabel("Plan tree")
    .selectOption({ label: "Website replatform" });
  await revise.getByRole("button", { name: "Revise the template" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(revise.getByRole("alert")).toHaveText(
    "The plan tree no longer has the template's shape: a revision may change tasks and add plans at the end, not move or rename plans.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a starter offers no revision", async ({ page }) => {
  await stubTemplates(page);
  await page.goto(`/en-001/plan-templates/${STARTER}`);
  await settle(page);
  await expect(
    page.getByRole("region", { name: "Revise it from a plan" }),
  ).toHaveCount(0);
});
