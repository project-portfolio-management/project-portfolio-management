// NC-1 slice 3 (LT-10): editing a task and linking it to other tasks, in a real browser over a stubbed service. A
// board card links to the task's page; the edit sends every field (the sprint, component and parent go back as they
// were, and no status); bad points are refused on the page before any call; a 422 or a 403 from the service says the
// catalogue's sentence, not the service's text. The task's links read from its own side ("Blocked by"), a hidden one is
// counted; a link is added and removed; a second copy is refused on the page; a 409, a 422 and a 404 say so in words.
// Welsh reads. The page passes axe. NC-1 slice 12 part (c) (IR-3): a blocked task raises an issue, once while it is
// open; the rule is said first; a task not blocked offers nothing; a 409 and a 422 say the catalogue's sentence.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const T1 = "aaaaaaaa-0000-4000-8000-000000000001";
const ASSIGNEE = "person:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";

const task = (
  pid: string,
  title: string,
  over: Record<string, unknown> = {},
) => ({
  pid,
  title,
  description: null,
  status: "in_progress",
  assignee_ref: null,
  points: null,
  due_date: null,
  deadline: null,
  sprint_pid: null,
  component_pid: null,
  parent_task_pid: null,
  created_at: "2026-11-01T00:00:00Z",
  status_changed_at: "2026-11-02T00:00:00Z",
  done_at: null,
  blocked_days: null,
  ...over,
});

interface Sent {
  method: string;
  path: string;
  body: unknown;
}

/** The stub; `refuse` makes the next call of that method answer with that status. */
async function stub(page: Page, refuse: Partial<Record<string, number>> = {}) {
  let me = task(T1, "Write the spec", {
    description: "The first draft",
    assignee_ref: ASSIGNEE,
    points: 3,
    due_date: "2026-11-20",
    sprint_pid: "s1",
    component_pid: "c1",
    parent_task_pid: "t0",
  });
  const others = [
    task("t2", "Build it", { status: "todo", due_date: "2026-11-30" }),
    task("t3", "Review it", { status: "todo", due_date: "2026-11-10" }),
  ];
  let relations = [
    {
      pid: "r1",
      kind: "blocks",
      direction: "incoming",
      other_task: {
        pid: "t3",
        title: "Review it",
        plan_pid: PID,
        status: "todo",
        due_date: "2026-11-10",
      },
    },
  ];
  const sent: Sent[] = [];
  const refusal = (method: string) => {
    const status = refuse[method];
    if (!status) return null;
    delete refuse[method];
    return status;
  };
  // The task's discussion (NC-1 slice 5) has its own spec, task-discussion.spec.ts; here it is empty.
  await page.route(
    /\/api\/(comments|reactions|attachments|watchers)(\?.*)?$/,
    (route) => {
      const path = new URL(route.request().url()).pathname;
      if (path.endsWith("/reactions"))
        return route.fulfill({ json: { counts: {}, mine: [] } });
      if (path.endsWith("/watchers"))
        return route.fulfill({ json: { count: 0, mine: false } });
      return route.fulfill({ json: [] });
    },
  );
  await page.route(`**/api/plans/${PID}/**`, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    const body = req.postDataJSON?.() ?? null;
    const base = `/api/plans/${PID}`;
    if (method !== "GET") sent.push({ method, path, body });
    const status = method === "GET" ? null : refusal(method);
    if (status)
      return route.fulfill({
        status,
        json: {
          error: "refused",
          description: "service text that must not be shown",
        },
      });
    if (path === `${base}/tasks` && method === "GET")
      return route.fulfill({
        json: {
          tasks: [me, ...others],
          counts: {},
          total: 3,
          truncated: false,
        },
      });
    if (path === `${base}/tasks/${T1}` && method === "GET")
      return route.fulfill({ json: me });
    if (path === `${base}/tasks/${T1}` && method === "PUT") {
      me = { ...me, ...(body as object) };
      return route.fulfill({ json: me });
    }
    if (path === `${base}/tasks/${T1}/relations` && method === "GET")
      return route.fulfill({ json: { relations, hidden: 1 } });
    if (path === `${base}/tasks/${T1}/relations` && method === "POST") {
      const b = body as { kind: string; to_task_pid: string };
      const other = others.find((o) => o.pid === b.to_task_pid)!;
      relations = [
        ...relations,
        {
          pid: "r2",
          kind: b.kind,
          direction: "outgoing",
          other_task: {
            pid: other.pid,
            title: other.title,
            plan_pid: PID,
            status: other.status,
            due_date: other.due_date,
          },
        },
      ];
      return route.fulfill({
        json: {
          pid: "r2",
          from_task_pid: T1,
          to_task_pid: other.pid,
          kind: b.kind,
        },
      });
    }
    const del = /\/relations\/(r\d)$/.exec(path);
    if (del && method === "DELETE") {
      relations = relations.filter((r) => r.pid !== del[1]);
      return route.fulfill({ json: {} });
    }
    if (path === `${base}/tasks/${T1}/links` && method === "GET")
      return route.fulfill({ json: [] });
    return route.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
  return sent;
}

const open = (page: Page, locale = "en-001") =>
  page.goto(`/${locale}/plans/${PID}/tasks/${T1}`, {
    waitUntil: "networkidle",
  });

test("a board card links to its task's page", async ({ page }) => {
  await page.route(/\/api\/plans\/[^/]+\/tasks(\?.*)?$/, (r) =>
    r.fulfill({
      json: { tasks: [task(T1, "Write the spec")], truncated: false, total: 1 },
    }),
  );
  await page.route(`**/api/plans/${PID}/sprints`, (r) =>
    r.fulfill({ json: [] }),
  );
  await page.route(`**/api/plans/${PID}/standup`, (r) =>
    r.fulfill({
      json: { tasks_created: [], tasks_moved: [], blocked_now: [] },
    }),
  );
  await page.route(`**/api/plans/${PID}/velocity`, (r) =>
    r.fulfill({ json: { sprints: [], note: "" } }),
  );
  await page.goto(`/en-001/plans/${PID}/board`, { waitUntil: "networkidle" });
  await expect(
    page
      .getByTestId("task-board")
      .getByRole("link", { name: "Write the spec" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/tasks/${T1}`);
});

test("edits a task, sending every field back and no status", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  await expect(
    page.getByRole("heading", { name: "Edit the task" }),
  ).toBeVisible();
  await page.getByLabel("Title", { exact: true }).fill("Write the full spec");
  await page.getByLabel(/^Story points/).fill("5");
  await page.getByLabel(/^Due date/).fill("2026-11-25");
  await page.getByRole("button", { name: "Save the task" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "The task was saved." }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { level: 1, name: "Write the full spec" }),
  ).toBeVisible();
  const put = sent.find((s) => s.method === "PUT")!;
  expect(put.path).toBe(`/api/plans/${PID}/tasks/${T1}`);
  expect(put.body).toEqual({
    title: "Write the full spec",
    description: "The first draft",
    assignee_ref: ASSIGNEE,
    points: 5,
    due_date: "2026-11-25",
    component_pid: "c1",
    parent_task_pid: "t0",
    sprint_pid: "s1",
  });
});

test("refuses bad points and a bad assignee on the page, before any call", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  await page.getByLabel(/^Story points/).fill("200");
  await page.getByRole("button", { name: "Save the task" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Story points must be from 0 to 100.",
  );
  await page.getByLabel(/^Story points/).fill("3");
  await page.getByLabel(/^Assigned to \(/).fill("alice");
  await page.getByRole("button", { name: "Save the task" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Name the assignee as a person: or worker: reference, or leave it empty.",
  );
  expect(sent).toEqual([]);
});

test("a refused edit says the catalogue's sentence, not the service's text", async ({
  page,
}) => {
  await stub(page, { PUT: 422 });
  await open(page);
  await page.getByLabel(/^Due date/).fill("2026-12-30");
  await page.getByRole("button", { name: "Save the task" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "A new due date must also keep every ‘Comes before’ link in order.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("an edit the caller may not make says so", async ({ page }) => {
  await stub(page, { PUT: 403 });
  await open(page);
  await page.getByRole("button", { name: "Save the task" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "You may not change the tasks of this plan.",
  );
});

test("shows the links from this task's side, adds one and removes one", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const list = page.getByTestId("task-relations");
  await expect(list.getByText("Blocked by")).toBeVisible();
  await expect(list.getByRole("link", { name: "Review it" })).toHaveAttribute(
    "href",
    `/en-001/plans/${PID}/tasks/t3`,
  );
  await expect(
    page.getByText("1 more link goes to a task in a plan you cannot see."),
  ).toBeVisible();

  await page.getByLabel(/^This task/).selectOption("precedes");
  await page.getByLabel(/^Other task/).selectOption({ label: "Build it" });
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "The link was added." }),
  ).toBeVisible();
  await expect(list.getByText("Comes before")).toBeVisible();
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/tasks/${T1}/relations`,
    body: { kind: "precedes", to_task_pid: "t2" },
  });

  await page
    .getByRole("button", { name: "Remove the link: Blocked by Review it" })
    .click();
  await expect(
    page.getByRole("status").filter({ hasText: "The link was removed." }),
  ).toBeVisible();
  await expect(list.getByText("Blocked by")).toHaveCount(0);
  expect(sent.at(-1)).toMatchObject({
    method: "DELETE",
    path: `/api/plans/${PID}/tasks/${T1}/relations/r1`,
  });
});

test("refuses on the page a link the task has, a loop, and dates out of order", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  // Review it already blocks this task: this task blocking it would be a loop.
  await page.getByLabel(/^This task/).selectOption("blocks");
  await page.getByLabel(/^Other task/).selectOption({ label: "Review it" });
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "The other task already has that link to this one; the two would wait on each other.",
  );
  // Review it is due before this task, so this task cannot come before it.
  await page.getByLabel(/^This task/).selectOption("precedes");
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "‘Comes before’ needs the other task to be due after this one",
  );
  await page.getByLabel(/^Other task/).selectOption("");
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(page.getByRole("alert")).toHaveText("Choose the other task.");
  expect(sent).toEqual([]);
});

test("a refused link says why in words: 409, 422, and a link already gone", async ({
  page,
}) => {
  await stub(page, { POST: 409 });
  await open(page);
  await page.getByLabel(/^This task/).selectOption("related");
  await page.getByLabel(/^Other task/).selectOption({ label: "Build it" });
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "That link already exists; the links have been read again.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { POST: 422, DELETE: 404 });
  await open(page);
  await page.getByLabel(/^This task/).selectOption("blocks");
  await page.getByLabel(/^Other task/).selectOption({ label: "Build it" });
  await page.getByRole("button", { name: "Link the tasks" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "The service did not accept the link",
  );
  await page
    .getByRole("button", { name: "Remove the link: Blocked by Review it" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That link had already been removed; the links have been read again.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Golygu'r dasg" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Cysylltiadau â thasgau eraill" }),
  ).toBeVisible();
  await expect(
    page.getByTestId("task-relations").getByText("Wedi'i rhwystro gan"),
  ).toBeVisible();
  await expect(page.getByRole("button", { name: "Cadw'r dasg" })).toBeVisible();
});

test("passes the accessibility audit, with links listed", async ({ page }) => {
  await stub(page);
  await open(page);
  await expect(page.getByTestId("task-relations")).toBeVisible();
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

/** The task blocked, and the plan's issues as the service would list them; `raised` once the issue exists. */
async function blockedWithIssues(page: Page) {
  const state = { raised: false };
  await page.route(`**/api/plans/${PID}/tasks/${T1}`, (route) =>
    route.request().method() === "GET"
      ? route.fulfill({
          json: task(T1, "Write the spec", {
            status: "blocked",
            assignee_ref: ASSIGNEE,
          }),
        })
      : route.fallback(),
  );
  await page.route(`**/api/plans/${PID}/issues`, (route) =>
    route.fulfill({
      json: {
        plan_pid: PID,
        issues: state.raised
          ? [
              {
                pid: "i1",
                plan_pid: PID,
                title: "Supplier has not delivered",
                kind: "blocker",
                severity: "med",
                status: "open",
                status_category: "todo",
                source_task_pid: T1,
              },
            ]
          : [],
        total: 0,
        open: 0,
        overdue: 0,
        as_of: "2026-11-18",
      },
    }),
  );
  return state;
}

test("a blocked task raises an issue once, with the rule said first", async ({
  page,
}) => {
  const sent = await stub(page);
  const state = await blockedWithIssues(page);
  await page.route(`**/api/plans/${PID}/issues/from-task/${T1}`, (route) => {
    sent.push({
      method: "POST",
      path: new URL(route.request().url()).pathname,
      body: null,
    });
    state.raised = true;
    return route.fulfill({
      json: { pid: "i1", title: "Supplier has not delivered" },
    });
  });
  await open(page);
  const region = page.getByRole("region", { name: "An issue from this task" });
  await expect(region).toContainText(
    "Only a blocked task can be raised as an issue, and only once while that issue is open.",
  );
  await region
    .getByRole("button", { name: "Raise an issue from this task" })
    .click();
  await expect(region.getByRole("status")).toContainText(
    "The issue was raised.",
  );
  await expect(
    region
      .getByRole("status")
      .getByRole("link", { name: "See the plan's issues" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/issues`);
  await expect(region.getByTestId("task-issue")).toContainText(
    "An open issue was raised from this task: “Supplier has not delivered”.",
  );
  await expect(
    region.getByRole("button", { name: "Raise an issue from this task" }),
  ).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/proxy/api/plans/${PID}/issues/from-task/${T1}`,
      body: null,
    },
  ]);
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("a task that is not blocked says so and offers nothing", async ({
  page,
}) => {
  await stub(page);
  await open(page);
  const region = page.getByRole("region", { name: "An issue from this task" });
  await expect(region.getByTestId("task-issue")).toHaveText(
    "This task is not blocked, so it cannot be raised as an issue. Mark it blocked on the board first.",
  );
  await expect(region.getByRole("button")).toHaveCount(0);
});

for (const [code, words] of [
  [
    409,
    "Refused: an open issue has already been raised from this task. The issues were read again.",
  ],
  [
    422,
    "Refused: only a blocked task can be raised as an issue, and this one is no longer blocked. The task was read again.",
  ],
] as const) {
  test(`a ${code} on raising an issue says the catalogue's sentence`, async ({
    page,
  }) => {
    await stub(page);
    await blockedWithIssues(page);
    await page.route(`**/api/plans/${PID}/issues/from-task/${T1}`, (route) =>
      route.fulfill({
        status: code,
        json: {
          code: code === 409 ? "conflict" : "unprocessable",
          detail: "service text",
        },
      }),
    );
    await open(page);
    await page
      .getByRole("button", { name: "Raise an issue from this task" })
      .click();
    await expect(page.getByRole("alert")).toHaveText(words);
    await expect(page.getByText("service text")).toHaveCount(0);
  });
}

test("raising an issue from a task reads in Welsh", async ({ page }) => {
  await stub(page);
  await blockedWithIssues(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Mater o'r dasg hon" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Codi mater o'r dasg hon" }),
  ).toBeVisible();
});
