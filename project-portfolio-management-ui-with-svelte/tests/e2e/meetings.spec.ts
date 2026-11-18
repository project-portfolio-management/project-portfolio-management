// NC-1 slice 12 part (e) (NC-14, BR-3): a plan's meetings and the tasks made from their actions, in a real browser over
// a stubbed service. The plan page links here; the rules are said first (no attendees; held minutes cannot be
// corrected); each meeting's state is a word and a shape and its links are named as the records are now; planning and
// recording as held check every rule before sending and send what was typed, trimmed; recording and cancelling ask
// first and Cancel sends nothing; tasks are proposed from the actions with the rules said first, nothing sent when none
// is chosen or a rule fails, each verdict in the catalogue's words by the service's code, and created only from a check
// still current with every proposal ready, after a question; each refusal is the catalogue's sentence; the service's
// English is never shown. Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const PLANNED = {
  pid: "m1",
  title: "Gate 2 review",
  scheduled_on: "2026-11-24",
  status: "planned",
  held_on: null,
  minutes: null,
  agenda: [
    {
      kind: null,
      text: "Scope",
      link: { kind: "task", pid: "t1", name: "Order the servers" },
    },
    {
      kind: null,
      text: "Budget",
      link: { kind: "risk", pid: "r9", name: null },
    },
  ],
  outcomes: [],
};
const HELD = {
  pid: "m2",
  title: "Kick-off",
  scheduled_on: "2026-11-03",
  status: "held",
  held_on: "2026-11-04",
  minutes: "We agreed.\nPriya will order the racks by the tenth.",
  agenda: [],
  outcomes: [
    { index: 0, kind: "decision", text: "Go", link: null, task_pid: null },
    {
      index: 1,
      kind: "action",
      text: "Order the racks",
      link: null,
      task_pid: null,
    },
    {
      index: 2,
      kind: "action",
      text: "Book the room",
      link: null,
      task_pid: "t9",
    },
  ],
};
const CANCELLED = {
  ...PLANNED,
  pid: "m3",
  title: "Supplier call",
  status: "cancelled",
  agenda: [],
};

type Sent = { method: string; path: string; body: unknown; key: string | null };

async function stub(
  page: Page,
  opts: { refuse?: Record<string, number> } = {},
) {
  const meetings = [
    structuredClone(PLANNED),
    structuredClone(HELD),
    structuredClone(CANCELLED),
  ];
  const sent: Sent[] = [];
  await stubPlansApi(page);
  await page.route(
    /\/api\/plans\/[^/]+\/(meetings|tasks|risks|milestones|issues)(\/.*)?$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const parts = path.split("/").slice(4); // after /api/plans/{pid}
      if (req.method() === "GET") {
        if (parts[0] === "tasks")
          return route.fulfill({
            json: {
              tasks: [{ pid: "t1", title: "Order the servers" }],
              counts: {},
            },
          });
        if (parts[0] === "risks")
          return route.fulfill({
            json: [{ pid: "r1", title: "Supplier insolvency" }],
          });
        if (parts[0] === "milestones")
          return route.fulfill({ json: [{ pid: "ms1", name: "Go-live" }] });
        if (parts[0] === "issues")
          return route.fulfill({
            json: { plan_pid: PID, issues: [], total: 0, open: 0 },
          });
        if (parts.length === 1)
          return route.fulfill({
            json: meetings.map(
              ({ pid, title, scheduled_on, status, held_on }) => ({
                pid,
                title,
                scheduled_on,
                status,
                held_on,
              }),
            ),
          });
        const m = meetings.find((x) => x.pid === parts[1]);
        return m
          ? route.fulfill({ json: m })
          : route.fulfill({
              status: 404,
              json: { code: "not_found", detail: "service text" },
            });
      }
      const body = req.postDataJSON();
      sent.push({
        method: req.method(),
        path,
        body,
        key: req.headers()["idempotency-key"] ?? null,
      });
      const action = parts.length === 1 ? "plan" : parts.slice(2).join("/");
      const refuse = opts.refuse?.[action];
      if (refuse)
        return route.fulfill({
          status: refuse,
          json: {
            code:
              refuse === 409
                ? "conflict"
                : refuse === 404
                  ? "not_found"
                  : "unprocessable",
            detail: "service text",
          },
        });
      const m = meetings.find((x) => x.pid === parts[1]);
      if (action === "plan") {
        meetings.unshift({
          ...structuredClone(PLANNED),
          pid: "m9",
          title: body.title,
          scheduled_on: body.scheduled_on,
          agenda: [],
        });
        return route.fulfill({ json: { pid: "m9", status: "planned" } });
      }
      if (action === "held" && m) {
        Object.assign(m, {
          status: "held",
          minutes: body.minutes,
          held_on: body.held_on ?? m.scheduled_on,
          outcomes: (body.outcomes ?? []).map(
            (o: { kind: string; text: string }, index: number) => ({
              ...o,
              link: null,
              index,
              task_pid: null,
            }),
          ),
        });
        return route.fulfill({ json: { pid: m.pid, status: "held" } });
      }
      if (action === "cancel" && m) {
        m.status = "cancelled";
        return route.fulfill({ json: { pid: m.pid, status: "cancelled" } });
      }
      if (action === "task-proposals/preview")
        return route.fulfill({
          json: {
            meeting: parts[1],
            counts: { ready: 0, dropped: 0 },
            items: body.items.map(
              (it: { assignee_ref?: string }, index: number) =>
                it.assignee_ref === "person:nobody"
                  ? { index, status: "dropped", reason: "assignee_not_in_plan" }
                  : {
                      index,
                      status: "ready",
                      assignee: it.assignee_ref ?? null,
                    },
            ),
          },
        });
      if (action === "task-proposals/commit" && m) {
        const o = m.outcomes.find((x) => x.index === body.items[0].outcome);
        if (o) o.task_pid = "t-new";
        return route.fulfill({
          json: {
            meeting: m.pid,
            created: [{ index: 0, outcome: 1, task_pid: "t-new" }],
          },
        });
      }
      return route.fulfill({
        status: 404,
        json: { code: "not_found", detail: "service text" },
      });
    },
  );
  return sent;
}

const meeting = (page: Page, title: string) =>
  page.getByRole("region", { name: `Open the meeting ${title}` });

test("the plan page links here; the rules are said first; each meeting's state and links are shown, with no service text", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page
    .getByRole("link", { name: "Meetings, minutes and actions" })
    .click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Meetings" }),
  ).toBeVisible();
  await expect(page.getByTestId("meetings-rules")).toContainText(
    "A held or cancelled meeting is a record: held minutes cannot be corrected and a cancelled meeting cannot be restored. Attendees are not recorded.",
  );
  const list = page.getByRole("list", {
    name: "The plan's meetings, newest first",
  });
  await expect(
    list.getByRole("listitem").filter({ hasText: "Gate 2 review" }),
  ).toContainText("○ planned, planned for 24 Nov 2026");
  await expect(
    list.getByRole("listitem").filter({ hasText: "Kick-off" }).first(),
  ).toContainText("✓ held, held on 4 Nov 2026");
  await expect(
    list.getByRole("listitem").filter({ hasText: "Supplier call" }),
  ).toContainText("✗ cancelled");
  await expect(
    page.getByRole("link", {
      name: "Download the meeting Gate 2 review as PDF",
    }),
  ).toHaveAttribute(
    "href",
    new RegExp(`/api/proxy/api/plans/${PID}/meetings/m1/pdf$`),
  );
  await page
    .getByRole("button", { name: "Open the meeting Gate 2 review" })
    .click();
  await expect(
    meeting(page, "Gate 2 review")
      .getByTestId("meeting-agenda")
      .getByRole("listitem"),
  ).toHaveText([
    "Scope · about Task: Order the servers",
    "Budget · about a record that is no longer there",
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("planning a meeting checks every rule first, sends nothing until they hold, then sends what was typed", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/meetings`);
  await settle(page);
  const region = page.getByRole("region", { name: "Plan a meeting" });
  await expect(
    region.getByText(
      "A title of up to 200 characters and a date. The agenda holds up to 50 items of up to 500 characters each; an item may point at a task, risk, milestone or issue of this plan.",
    ),
  ).toBeVisible();
  await region.getByRole("button", { name: "Add an agenda item" }).click();
  const item = region.getByRole("group", { name: "Agenda item 1" });
  await expect(
    item.getByLabel("The record agenda item 1 concerns"),
  ).toBeEnabled();
  await item
    .getByLabel("The record agenda item 1 concerns")
    .selectOption({ label: "Supplier insolvency" });
  await region.getByRole("button", { name: "Plan the meeting" }).click();
  await expect(region.getByRole("alert").getByRole("listitem")).toHaveText([
    "Give a title of up to 200 characters.",
    "Give a date.",
    "For Agenda item 1: Write it in up to 500 characters.",
  ]);
  expect(sent).toEqual([]);
  await region
    .getByLabel("Title (up to 200 characters)")
    .fill("  Risk review ");
  await region.getByLabel("Date it is planned for").fill("2026-12-01");
  await item
    .getByLabel("What agenda item 1 is about (up to 500 characters)")
    .fill(" The supplier ");
  await region.getByRole("button", { name: "Plan the meeting" }).click();
  await expect(page.getByTestId("page-notice")).toHaveText(
    "Planned: Risk review, on 1 Dec 2026.",
  );
  expect(
    sent.map(({ method, path, body }) => ({ method, path, body })),
  ).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/meetings`,
      body: {
        title: "Risk review",
        scheduled_on: "2026-12-01",
        agenda: [{ text: "The supplier", link: { kind: "risk", pid: "r1" } }],
      },
    },
  ]);
});

test("recording a meeting as held says the rules, asks first (Cancel sends nothing), and sends the minutes and outcomes", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/meetings`);
  await settle(page);
  await page
    .getByRole("button", { name: "Open the meeting Gate 2 review" })
    .click();
  const form = page.getByRole("form", { name: "Record Gate 2 review as held" });
  await expect(form).toContainText(
    "This is done once: the minutes cannot be corrected afterwards.",
  );
  await form.getByRole("button", { name: "Add an outcome" }).click();
  await form.getByLabel("Outcome 1 in words (up to 500 characters)").fill("Go");
  await form.getByRole("button", { name: "Record as held" }).click();
  await expect(form.getByRole("alert").getByRole("listitem")).toHaveText([
    "Write the minutes, in up to 20,000 characters.",
    "For Outcome 1: Choose a decision, an action or a note.",
  ]);
  await expect(page.getByRole("alertdialog")).toHaveCount(0);
  await form
    .getByLabel("Minutes (up to 20,000 characters)")
    .fill(" We agreed to go. ");
  await form
    .getByLabel("What outcome 1 is")
    .selectOption({ label: "Decision" });
  await form.getByRole("button", { name: "Record as held" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Record Gate 2 review as held? Its minutes and outcomes cannot be corrected afterwards.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await form.getByRole("button", { name: "Record as held" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("page-notice")).toHaveText(
    "Recorded as held: Gate 2 review.",
  );
  expect(sent.map(({ path, body }) => ({ path, body }))).toEqual([
    {
      path: `/api/plans/${PID}/meetings/m1/held`,
      body: {
        minutes: "We agreed to go.",
        outcomes: [{ kind: "decision", text: "Go" }],
      },
    },
  ]);
  const shown = meeting(page, "Gate 2 review");
  await expect(shown.getByTestId("meeting-outcomes")).toHaveText(
    "Decision: Go",
  );
  await expect(shown).toContainText(
    "This meeting is a record: its minutes and outcomes are not changed.",
  );
  await expect(shown.getByRole("form")).toHaveCount(0);
});

test("cancelling a meeting asks first; Cancel sends nothing", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/meetings`);
  await settle(page);
  await page
    .getByRole("button", { name: "Open the meeting Gate 2 review" })
    .click();
  const cancel = page.getByRole("button", {
    name: "Cancel the meeting Gate 2 review",
  });
  await cancel.click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Cancel Gate 2 review? A cancelled meeting cannot be restored.",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "Cancel", exact: true })
    .click();
  expect(sent).toEqual([]);
  await cancel.click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("page-notice")).toHaveText(
    "Cancelled: Gate 2 review.",
  );
  expect(sent.map((s) => s.path)).toEqual([
    `/api/plans/${PID}/meetings/m1/cancel`,
  ]);
  await expect(meeting(page, "Gate 2 review")).toContainText(
    "This meeting was cancelled. It is a record and cannot be restored.",
  );
});

test("tasks from the actions: rules first, nothing sent for none chosen or a rule failing, each verdict in words, created once after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/meetings`);
  await settle(page);
  await page.getByRole("button", { name: "Open the meeting Kick-off" }).click();
  const shown = meeting(page, "Kick-off");
  await expect(
    shown.getByRole("link", { name: "the task made from outcome 3" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/tasks/t9`);
  const tasks = shown.getByRole("region", { name: "Tasks from the actions" });
  await expect(tasks.getByTestId("tasks-rules")).toContainText(
    "a due date is optional, from 28 Oct 2026 to 4 Nov 2027",
  );
  // Only the action with no task yet is offered.
  await expect(tasks.getByRole("group")).toHaveCount(1);
  const check = tasks.getByRole("button", { name: "Check the proposals" });
  const create = tasks.getByRole("button", { name: "Create the tasks" });
  await check.click();
  await expect(tasks.getByRole("status")).toHaveText(
    "No proposal is chosen, so nothing was sent.",
  );
  await tasks.getByLabel("Propose a task for action 2").check();
  await expect(
    tasks.getByLabel("Task title for action 2 (up to 200 characters)"),
  ).toHaveValue("Order the racks");
  await tasks
    .getByLabel("Passage of the minutes for action 2 (at least 8 characters)")
    .fill("Priya will order the servers");
  await tasks
    .getByLabel("Points for action 2 (optional, 0 to 1000)")
    .fill("1001");
  await check.click();
  await expect(tasks.getByRole("alert").getByRole("listitem")).toHaveText([
    "For Outcome 2: Quote a passage of at least 8 characters that is in the minutes.",
    "For Outcome 2: Points are a whole number from 0 to 1000.",
  ]);
  expect(sent).toEqual([]);
  await tasks
    .getByLabel("Passage of the minutes for action 2 (at least 8 characters)")
    .fill("priya  will ORDER the racks");
  await tasks.getByLabel("Points for action 2 (optional, 0 to 1000)").fill("3");
  await tasks
    .getByLabel("Assignee for action 2 (optional reference)")
    .fill("person:nobody");
  await check.click();
  await expect(tasks.getByTestId("verdict-2")).toHaveText(
    "Not ready: the assignee is not one of this plan's people (on its tasks or its responsibility chart).",
  );
  await expect(create).toBeDisabled();
  await tasks
    .getByLabel("Assignee for action 2 (optional reference)")
    .fill("person:ana");
  await expect(
    tasks.getByText(
      "The proposals changed since they were checked: check them again.",
    ),
  ).toBeVisible();
  await expect(create).toBeDisabled();
  await check.click();
  await expect(tasks.getByTestId("verdict-2")).toHaveText(
    "Ready, for person:ana.",
  );
  await create.click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Create 1 task from the actions of Kick-off? An action gets one task only.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent.map((s) => s.path.split("/").at(-1))).toEqual([
    "preview",
    "preview",
  ]);
  await create.click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(
    shown.getByRole("link", { name: "the task made from outcome 2" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/tasks/t-new`);
  const commit = sent.at(-1)!;
  expect(commit.path).toBe(
    `/api/plans/${PID}/meetings/m2/task-proposals/commit`,
  );
  expect(commit.body).toEqual({
    items: [
      {
        title: "Order the racks",
        excerpt: "priya  will ORDER the racks",
        outcome: 1,
        assignee_ref: "person:ana",
        points: 3,
      },
    ],
  });
  expect(commit.key).toBeTruthy();
  // No action is left without a task, so nothing more is offered.
  await expect(
    shown.getByRole("region", { name: "Tasks from the actions" }),
  ).toHaveCount(0);
});

test("each refusal is the catalogue's sentence, never the service's", async ({
  page,
}) => {
  await stub(page, {
    refuse: { plan: 422, held: 409, "task-proposals/commit": 409 },
  });
  await page.goto(`/en-001/plans/${PID}/meetings`);
  await settle(page);
  const plan = page.getByRole("region", { name: "Plan a meeting" });
  await plan.getByLabel("Title (up to 200 characters)").fill("Risk review");
  await plan.getByLabel("Date it is planned for").fill("2026-12-01");
  await plan.getByRole("button", { name: "Plan the meeting" }).click();
  await expect(page.getByTestId("meetings-refusal")).toHaveText(
    "Refused: a record an item points at is no longer a record of this plan. Nothing was saved; choose the records again.",
  );
  await page
    .getByRole("button", { name: "Open the meeting Gate 2 review" })
    .click();
  const form = page.getByRole("form", { name: "Record Gate 2 review as held" });
  await form.getByLabel("Minutes (up to 20,000 characters)").fill("We met.");
  await form.getByRole("button", { name: "Record as held" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("meetings-refusal")).toHaveText(
    "Refused: this meeting has already been held or cancelled, so it is a record and is not changed. It was read again.",
  );
  await page.getByRole("button", { name: "Open the meeting Kick-off" }).click();
  const tasks = meeting(page, "Kick-off").getByRole("region", {
    name: "Tasks from the actions",
  });
  await tasks.getByLabel("Propose a task for action 2").check();
  await tasks
    .getByLabel("Passage of the minutes for action 2 (at least 8 characters)")
    .fill("Priya will order the racks");
  await tasks.getByRole("button", { name: "Check the proposals" }).click();
  await expect(tasks.getByTestId("verdict-2")).toHaveText(
    "Ready, with no assignee.",
  );
  await tasks.getByRole("button", { name: "Create the tasks" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(tasks.getByRole("alert")).toHaveText(
    "Refused: the meeting is not held, or one of these actions has a task meanwhile. Nothing was created; the meeting was read again, so check again.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the page reads in Welsh and passes axe with the forms open", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/meetings`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Cyfarfodydd" }),
  ).toBeVisible();
  const axe = async () => {
    await settle(page);
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(
      results.violations.filter(
        (v) => v.impact === "serious" || v.impact === "critical",
      ),
    ).toEqual([]);
  };
  await page
    .getByRole("button", { name: "Agor y cyfarfod Gate 2 review" })
    .click();
  await page.getByRole("button", { name: "Ychwanegu canlyniad" }).click();
  await axe();
  await page.getByRole("button", { name: "Agor y cyfarfod Kick-off" }).click();
  await page.getByLabel("Cynnig tasg ar gyfer gweithred 2").check();
  await axe();
});
