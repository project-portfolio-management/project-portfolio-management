// NC-1 slice 12 part (j): the board, sprints, resources and the OKR and KPI reads, in a real browser over a stubbed
// service. On the board many tasks at once (the same fields set, copied, moved after a question, deleted after a
// question; RM-9), the chosen sprint's ceremonies and its commitment (FR-29) and following the board's changes as they
// happen (PL-3). On /resources a placeholder given a person (over capacity asked first) or removed, planned against
// recorded hours, a plan's skill gaps and the cost rate in force (SC-7); on /capacity effort against capacity (PJ-3);
// on /okrs a plan's goal score with an objective's key results, and a key result's check-ins (KO-2); on /kpis every
// reading, the replaced ones marked (KO-1). Every rule said and checked first; nothing sent for no change; each refusal
// in the catalogue's words; every stub that would carry the service's English carries "service text", which never
// shows. Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const P2 = "22222222-2222-4222-8222-222222222222";
const T1 = "aaaaaaaa-0000-4000-8000-000000000001";
const T2 = "aaaaaaaa-0000-4000-8000-000000000002";
const T3 = "aaaaaaaa-0000-4000-8000-000000000003";
const S1 = "cccccccc-0000-4000-8000-000000000001";
const A1 = "dddddddd-0000-4000-8000-000000000001";
const O1 = "eeeeeeee-0000-4000-8000-000000000001";
const K1 = "eeeeeeee-0000-4000-8000-0000000000b1";
const M1 = "ffffffff-0000-4000-8000-000000000001";
const PERSON = "person:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";

interface Sent {
  method: string;
  path: string;
  body?: unknown;
}

const task = (
  pid: string,
  title: string,
  over: Record<string, unknown> = {},
) => ({
  pid,
  title,
  description: null,
  status: "todo",
  assignee_ref: null,
  points: null,
  due_date: null,
  sprint_pid: S1,
  created_at: "2026-11-01T00:00:00Z",
  status_changed_at: "2026-11-01T00:00:00Z",
  done_at: null,
  blocked_days: null,
  ...over,
});

const refused = (status: number, code = "unprocessable") => ({
  status,
  json: {
    type: "about:blank",
    title: "x",
    status,
    code,
    detail: "service text",
  },
});

const RESOURCES = {
  scope: { kind: "portfolio" },
  from: "2026-11-01",
  to: "2026-11-30",
  period: "month",
  group_by: "role",
  overlapping: false,
  periods: [
    {
      from: "2026-11-01",
      to: "2026-11-30",
      days: 30,
      total_demand_fte: 1,
      total_placeholder_fte: 1,
      total_proposed_fte: 0,
      rows: [
        {
          key: "dev",
          demand_fte: 1,
          named_fte: 0,
          placeholder_fte: 1,
          proposed_fte: 0,
          supply_fte: 1,
          gap_fte: 0,
          headcount: 1,
        },
      ],
    },
  ],
  by_plan: [],
  placeholders: [
    {
      allocation_pid: A1,
      plan_pid: PID,
      role: "dev",
      percent: 50,
      start_date: "2026-11-02",
      end_date: "2026-11-27",
      booking: "committed",
    },
  ],
  people_by_role: null,
  masked: false,
  note: "service text",
};

/** The stub; `refuse["METHOD /path"]` makes the next such call answer with that status (and `codes` its code). */
async function stub(
  page: Page,
  refuse: Partial<Record<string, number>> = {},
  codes: Partial<Record<string, string>> = {},
): Promise<Sent[]> {
  const sent: Sent[] = [];
  let tasks = [
    task(T1, "Design", { assignee_ref: PERSON, due_date: "2026-12-01" }),
    task(T2, "Build"),
    task(T3, "Test", { sprint_pid: null }),
  ];
  let committed = false;
  let held = 0;
  await page.route(/\/api\/proxy\/api\//, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    const body = req.postDataJSON?.() ?? undefined;
    const key = `${method} ${path}`;
    if (method !== "GET") sent.push({ method, path, body });
    else sent.push({ method, path: `${path}${url.search}` });
    const status = refuse[key];
    if (status) {
      delete refuse[key];
      return route.fulfill(refused(status, codes[key]));
    }
    const base = `/api/plans/${PID}`;
    if (path === "/api/plans")
      return route.fulfill({
        json: [
          { pid: PID, name: "Website replatform" },
          { pid: P2, name: "Data platform" },
        ],
      });
    if (path === `${base}/tasks` && method === "GET")
      return route.fulfill({
        json: { tasks, counts: {}, total: tasks.length },
      });
    if (path === `${base}/sprints` && method === "GET")
      return route.fulfill({
        json: [
          {
            pid: S1,
            name: "Sprint 7",
            starts_on: "2026-11-16",
            ends_on: "2026-11-27",
          },
        ],
      });
    if (path === `${base}/burndown`)
      return route.fulfill({
        json: {
          as_of: "2026-11-18T00:00:00Z",
          sprint: {
            pid: S1,
            name: "Sprint 7",
            starts_on: "2026-11-16",
            ends_on: "2026-11-27",
          },
          total_tasks: 2,
          derivation: "service text",
          points: [{ date: "2026-11-16", remaining: 2 }],
        },
      });
    if (path === `${base}/sprints/${S1}/notes`)
      return route.fulfill({ json: [] });
    if (path === `${base}/standup`)
      return route.fulfill({
        json: { tasks_created: [], tasks_moved: [], blocked_now: [] },
      });
    if (path === `${base}/velocity`)
      return route.fulfill({
        json: {
          sprints: [
            {
              sprint: { pid: S1, name: "Sprint 7" },
              tasks_done: 1,
              points_done: 3,
              unpointed_done: 0,
            },
          ],
          note: "service text",
        },
      });
    if (
      path === "/api/tasks/bulk" ||
      path === "/api/tasks/copy" ||
      path === "/api/tasks/move"
    ) {
      const ids = (body as { ids: string[] }).ids;
      const results = ids.map((pid, i) =>
        i === ids.length - 1 && ids.length > 1
          ? { pid, result: "refused", note: "service text" }
          : {
              pid,
              result: "ok",
              note: path.endsWith("move") ? "service text" : null,
            },
      );
      const ok = results.filter((r) => r.result === "ok").length;
      return route.fulfill({
        json: { requested: ids.length, ok, refused: ids.length - ok, results },
      });
    }
    if (path.startsWith(`${base}/tasks/`) && method === "DELETE") {
      tasks = tasks.filter((x) => x.pid !== path.split("/").pop());
      return route.fulfill({ json: {} });
    }
    if (path === `${base}/live`)
      return route.fulfill({
        status: 200,
        headers: { "content-type": "text/event-stream" },
        body: `retry: 600000\nid: 7\nevent: task_moved\ndata: {"plan_pid":"${PID}","kind":"task_moved","data":{}}\n\n`,
      });
    if (path === `/api/sprints/${S1}/ceremonies` && method === "GET")
      return route.fulfill({
        json: {
          ceremonies: [
            {
              pid: "c1",
              sprint_pid: S1,
              kind: "planning",
              held_at: "2026-11-16T09:00:00Z",
              facilitator_ref: PERSON,
              summary: "Scope agreed",
            },
            ...(held > 0
              ? [
                  {
                    pid: "c2",
                    sprint_pid: S1,
                    kind: "daily",
                    held_at: "2026-11-18T09:00:00Z",
                    facilitator_ref: null,
                    summary: null,
                  },
                ]
              : []),
          ],
          held: [
            { kind: "planning", count: 1 },
            { kind: "daily", count: held },
            { kind: "review", count: 0 },
            { kind: "retrospective", count: 0 },
          ],
          retrospective_notes: [],
        },
      });
    if (path === `/api/sprints/${S1}/ceremonies`) {
      held += 1;
      return route.fulfill({ json: { pid: "c2" } });
    }
    if (path === `/api/sprints/${S1}/commitment`)
      return route.fulfill({
        json: committed
          ? {
              sprint_pid: S1,
              committed: 2,
              current: 2,
              added_after_commitment: [T3],
              removed_after_commitment: [T2],
              was_committed: true,
              note: "service text",
            }
          : {
              sprint_pid: S1,
              committed: 0,
              current: 2,
              added_after_commitment: [],
              removed_after_commitment: [],
              was_committed: false,
              note: "service text",
            },
      });
    if (path === `/api/sprints/${S1}/commit`) {
      committed = true;
      return route.fulfill({ json: { committed: 2 } });
    }
    if (path === "/api/resource-plan")
      return route.fulfill({ json: RESOURCES });
    if (path === `${base}/allocations/${A1}/resolve`)
      return route.fulfill({
        json: { pid: A1, person_ref: PERSON, over_capacity: true },
      });
    if (path === `${base}/allocations/${A1}`)
      return route.fulfill({ json: {} });
    if (path === "/api/resource-plan/hours")
      return route.fulfill({
        json: {
          from: "2026-10-22",
          to: "2026-11-18",
          plans: [
            {
              plan_pid: PID,
              planned_minutes: 4800,
              placeholder_planned_minutes: 960,
              actual_minutes: 4500,
              actual_reason: null,
              variance_minutes: -300,
            },
            {
              plan_pid: P2,
              planned_minutes: 600,
              placeholder_planned_minutes: 0,
              actual_minutes: null,
              actual_reason: "no_actuals",
              variance_minutes: null,
            },
          ],
          unplanned_actuals: [{ plan_pid: P2, minutes: 120, people_count: 1 }],
          total_planned_minutes: 5400,
          total_actual_minutes: 4500,
          total_actual_reason: null,
          truncated: false,
          masked: true,
          note: "service text",
        },
      });
    if (path === `${base}/skill-gap`)
      return route.fulfill({
        json: {
          plan_pid: PID,
          findings: [
            {
              allocation_pid: A1,
              person_ref: PERSON,
              tag: "rust",
              status: "covered",
            },
            {
              allocation_pid: A1,
              person_ref: PERSON,
              tag: "kafka",
              status: "missing",
            },
            {
              allocation_pid: "a2",
              person_ref: "person:x",
              tag: "sql",
              status: "unknown",
              reason: "service text",
            },
          ],
        },
      });
    if (path === "/api/cost-rates/lookup")
      return route.fulfill({
        json: {
          person_ref: PERSON,
          date: "2026-11-18",
          table: "standard",
          rate: {
            per_hour_minor: 6500,
            currency: "GBP",
            effective_from: "2026-04-01",
            table_pid: "r1",
            source: "role",
          },
          reason: null,
        },
      });
    if (path === "/api/capacity")
      return route.fulfill({
        json: { from: "2026-11-01", to: "2026-11-30", people: [] },
      });
    if (path === "/api/capacity/utilization") {
      const person = url.searchParams.get("by") === "person";
      const fig = (actor: string, over: Record<string, unknown> = {}) => ({
        actor_ref: actor,
        basis_points: 10500,
        unavailable: null,
        effort_minutes: 12600,
        available_minutes: 12000,
        declared_minutes: 13440,
        non_working_minutes: 1440,
        asserted: true,
        at_or_over_capacity: true,
        ...over,
      });
      return route.fulfill({
        json: person
          ? {
              by: "person",
              window_days: Number(url.searchParams.get("window_days") ?? 28),
              utilization: [
                fig(PERSON),
                fig("person:y", {
                  basis_points: null,
                  unavailable: "below_floor",
                  at_or_over_capacity: false,
                }),
              ],
              suppression: { floor_minutes: 2400, min_group_people: 5 },
              note: "service text",
            }
          : {
              by: "team",
              window_days: 28,
              utilization: fig("team", {
                at_or_over_capacity: false,
                basis_points: 8000,
              }),
              people_counted: 6,
              suppression: { floor_minutes: 2400, min_group_people: 5 },
            },
      });
    }
    if (path === "/api/okrs")
      return route.fulfill({
        json: {
          period: null,
          as_of: "2026-11-18T10:00:00Z",
          note: "service text",
          objectives: [
            {
              objective: {
                pid: O1,
                title: "Faster releases",
                description: null,
                period: "2026-Q4",
              },
              score_basis_points: 5000,
              measured: true,
              single_currency: true,
              key_results: [
                {
                  key_result: {
                    pid: K1,
                    objective_pid: O1,
                    title: "Lead time under 5 days",
                    metric: "number",
                    direction: "decrease",
                    start_value: 10,
                    target_value: 5,
                    current_value: 7,
                    tolerance: null,
                    unit: "days",
                    currency: null,
                    owner_ref: null,
                    due_date: null,
                  },
                  progress_basis_points: 6000,
                  unmeasurable: null,
                  latest_check_in: null,
                },
              ],
            },
          ],
        },
      });
    if (path === `${base}/okr`)
      return route.fulfill({
        json: {
          plan_pid: PID,
          score_basis_points: 5000,
          measured: true,
          as_of: "2026-11-18T10:00:00Z",
          objectives: [
            {
              objective_pid: O1,
              weight: 3,
              score_basis_points: 5000,
              measured: true,
              key_results: 1,
              single_currency: true,
            },
            {
              objective_pid: "99999999-0000-4000-8000-000000000009",
              weight: 1,
              score_basis_points: null,
              measured: false,
              key_results: 1,
              single_currency: true,
            },
          ],
        },
      });
    if (path === `/api/objectives/${O1}/key-results`)
      return route.fulfill({
        json: [
          {
            key_result: { pid: K1, title: "Lead time under 5 days" },
            progress_basis_points: 6000,
            unmeasurable: null,
          },
        ],
      });
    if (
      path ===
      "/api/objectives/99999999-0000-4000-8000-000000000009/key-results"
    )
      return route.fulfill({
        json: [
          {
            key_result: { pid: "k9", title: "Hold uptime" },
            progress_basis_points: null,
            unmeasurable: "no_tolerance",
          },
        ],
      });
    if (path === `/api/key-results/${K1}/check-ins`)
      return route.fulfill({
        json: [
          {
            pid: "ci1",
            key_result_pid: K1,
            observed_at: "2026-11-10T09:00:00Z",
            value: 7,
            confidence: 70,
            note: "Two teams on the new pipeline",
            actor: "person:z",
          },
        ],
      });
    if (path === "/api/kpis")
      return route.fulfill({
        json: {
          as_of: "2026-11-18T10:00:00Z",
          note: "service text",
          summary: { green: 1, amber: 0, red: 0, no_data: 0 },
          kpis: [
            {
              kpi: {
                pid: M1,
                plan_pid: null,
                objective_pid: null,
                name: "Deploys a week",
                description: null,
                unit: null,
                direction: "higher_better",
                target_value: 10,
                red_value: 2,
                cadence: null,
                owner_ref: null,
              },
              status: "green",
              latest: { value: 12, observed_on: "2026-11-17", note: null },
              trend: null,
              stale: null,
              readings: 2,
            },
          ],
        },
      });
    if (path === `/api/kpis/${M1}/readings`)
      return route.fulfill({
        json: {
          readings: [
            {
              pid: "r2",
              observed_on: "2026-11-17",
              value: 12,
              note: null,
              actor: null,
              superseded: false,
            },
            {
              pid: "r1",
              observed_on: "2026-11-17",
              value: 9,
              note: "typo",
              actor: null,
              superseded: true,
            },
          ],
        },
      });
    return route.fulfill(refused(404, "not_found"));
  });
  return sent;
}

const writes = (sent: Sent[]) => sent.filter((s) => s.method !== "GET");

async function open(page: Page, path: string, locale = "en-001") {
  await page.goto(`/${locale}${path}`, { waitUntil: "networkidle" });
  await settle(page);
}

async function answer(page: Page, ok: boolean) {
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: ok ? "OK" : "Cancel" })
    .click();
}

async function openBulk(page: Page) {
  const b = page.getByTestId("board-bulk");
  await b.getByText("Change many tasks at once").click();
  await expect(
    b.getByRole("combobox", { name: /^Plan/ }).locator("option"),
  ).toHaveCount(3);
  return b;
}

test("many tasks at once: the fields checked first, nothing sent for no change, the refused named", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, `/plans/${PID}/board`);
  const b = await openBulk(page);
  await expect(b).toContainText("At most 200 at a time");
  const edit = b.getByRole("form", { name: "Set fields on the chosen tasks" });
  await edit.getByRole("button", { name: "Set on the chosen tasks" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "Choose at least one task. Choose at least one field to set or clear.",
  );
  await b.getByRole("checkbox", { name: "Design" }).check();
  await expect(b.getByRole("group")).toContainText("1 task chosen");
  await edit.getByRole("combobox", { name: /^Assignee/ }).selectOption("set");
  await edit.getByRole("textbox", { name: "New Assignee" }).fill(PERSON);
  await edit.getByRole("button", { name: "Set on the chosen tasks" }).click();
  await expect(b.getByRole("status")).toHaveText(
    "The chosen tasks already have these values, so nothing was sent.",
  );
  await b.getByRole("checkbox", { name: "Build" }).check();
  await edit.getByRole("textbox", { name: "New Assignee" }).fill("Ada");
  await edit.getByRole("combobox", { name: /^Start date/ }).selectOption("set");
  await edit.getByLabel("New Start date").fill("1969-12-31");
  await edit.getByRole("button", { name: "Set on the chosen tasks" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "Give the assignee as person: or worker: and an id. Give the start date as a real date from 1970-01-05 to 2100-12-31.",
  );
  await edit.getByRole("textbox", { name: "New Assignee" }).fill(PERSON);
  await edit
    .getByRole("combobox", { name: /^Start date/ })
    .selectOption("keep");
  await edit.getByRole("combobox", { name: /^Due date/ }).selectOption("clear");
  await edit.getByRole("button", { name: "Set on the chosen tasks" }).click();
  await expect(b.getByRole("status")).toHaveText("Done for 1 of 2 tasks.");
  await expect(b.getByRole("alert")).toHaveText(
    "Refused, for a reason the service gives only in English: Build.",
  );
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: "/api/tasks/bulk",
      body: { ids: [T1, T2], set: { assignee_ref: PERSON, due_date: null } },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("copy, move after a question, and delete after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, `/plans/${PID}/board`);
  const b = await openBulk(page);
  const move = b.getByRole("form", { name: "Copy or move the chosen tasks" });
  await b.getByRole("checkbox", { name: "Design" }).check();
  await move
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Website replatform (this plan)" });
  await move.getByRole("button", { name: "Move to the plan" }).click();
  await expect(b.getByRole("alert")).toHaveText(
    "The tasks are in this plan already: choose another to move them to.",
  );
  await move
    .getByRole("checkbox", { name: "Keep the assignee when copying" })
    .check();
  await move.getByRole("button", { name: "Copy to the plan" }).click();
  await expect(b.getByRole("status")).toHaveText("Done for 1 of 1 task.");
  await b.getByRole("checkbox", { name: "Design" }).check();
  await move
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Data platform" });
  await move.getByRole("button", { name: "Move to the plan" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Move 1 task to Data platform? Its sprint, component and parent task are cleared",
  );
  await answer(page, false);
  await move.getByRole("button", { name: "Move to the plan" }).click();
  await answer(page, true);
  await expect(b.getByRole("status")).toHaveText(
    "Done for 1 of 1 task. 1 task had a status that plan does not have, so it starts in that plan's first state.",
  );
  await b.getByRole("checkbox", { name: "Test" }).check();
  await b.getByRole("button", { name: "Delete the chosen tasks" }).click();
  await expect(page.getByRole("alertdialog")).toContainText("Delete 1 task?");
  await answer(page, false);
  await b.getByRole("button", { name: "Delete the chosen tasks" }).click();
  await answer(page, true);
  await expect(b.getByRole("status")).toHaveText("Deleted 1 task.");
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: "/api/tasks/copy",
      body: { ids: [T1], to_plan_pid: PID, keep_assignee: true },
    },
    {
      method: "POST",
      path: "/api/tasks/move",
      body: { ids: [T1], to_plan_pid: P2 },
    },
    {
      method: "DELETE",
      path: `/api/plans/${PID}/tasks/${T3}`,
      body: undefined,
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused bulk change is the catalogue's sentence", async ({ page }) => {
  await stub(page, { "POST /api/tasks/move": 422 });
  await open(page, `/plans/${PID}/board`);
  const b = await openBulk(page);
  const move = b.getByRole("form", { name: "Copy or move the chosen tasks" });
  await b.getByRole("checkbox", { name: "Build" }).check();
  await move
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Data platform" });
  await move.getByRole("button", { name: "Move to the plan" }).click();
  await answer(page, true);
  await expect(b.getByRole("alert")).toHaveText(
    "That plan is not one you can see, or the service did not take the list.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a sprint's ceremonies and its commitment: checked first, committed once after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, `/plans/${PID}/board`);
  const c = page.getByTestId("sprint-ceremonies");
  await expect(
    c.getByRole("heading", { name: "Ceremonies of Sprint 7" }),
  ).toBeVisible();
  await expect(c.getByRole("list", { name: "Ceremonies held" })).toContainText(
    "Planning: held 1 time",
  );
  await expect(c.getByRole("list", { name: "Ceremonies held" })).toContainText(
    "Retrospective: not held",
  );
  await expect(c).toContainText("Scope agreed");
  const form = c.getByRole("form", { name: "Record a ceremony" });
  await form
    .getByRole("combobox", { name: "Ceremony" })
    .selectOption("planning");
  await form.getByLabel(/^Facilitator/).fill("Ada");
  await form.getByRole("button", { name: "Record the ceremony" }).click();
  await expect(c.getByRole("alert")).toHaveText(
    "This sprint has held its Planning already: a second one is a re-plan, which is a new sprint. Give the facilitator as person: or worker: and an id, or leave it blank.",
  );
  await form.getByRole("combobox", { name: "Ceremony" }).selectOption("daily");
  await form.getByLabel(/^Facilitator/).fill("");
  await form.getByRole("button", { name: "Record the ceremony" }).click();
  await expect(c.getByRole("status")).toHaveText("Recorded: Daily stand-up.");
  await expect(c.getByRole("list", { name: "Ceremonies held" })).toContainText(
    "Daily stand-up: held 1 time",
  );

  await expect(c.getByTestId("commitment")).toHaveText(
    "Not committed yet; 2 tasks are in the sprint now.",
  );
  await c.getByRole("button", { name: "Commit the sprint's tasks" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Commit the 2 tasks in Sprint 7 now? It is taken once and cannot be changed",
  );
  await answer(page, false);
  await c.getByRole("button", { name: "Commit the sprint's tasks" }).click();
  await answer(page, true);
  await expect(c.getByRole("status")).toHaveText("Committed 2 tasks.");
  await expect(c.getByTestId("commitment")).toHaveText(
    "2 committed; 2 in the sprint now.",
  );
  await expect(c).toContainText("1 task added since: Test.");
  await expect(c).toContainText("1 task removed since: Build.");
  await expect(
    c.getByRole("button", { name: "Commit the sprint's tasks" }),
  ).toHaveCount(0);
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: `/api/sprints/${S1}/ceremonies`,
      body: { kind: "daily" },
    },
    { method: "POST", path: `/api/sprints/${S1}/commit`, body: {} },
  ]);
  // The burndown's and velocity's notes are the catalogue's, not the service's.
  await expect(page.getByText("no ideal line is drawn")).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused ceremony and commitment are the catalogue's sentences", async ({
  page,
}) => {
  await stub(page, {
    [`POST /api/sprints/${S1}/ceremonies`]: 422,
    [`POST /api/sprints/${S1}/commit`]: 422,
  });
  await open(page, `/plans/${PID}/board`);
  const c = page.getByTestId("sprint-ceremonies");
  const form = c.getByRole("form", { name: "Record a ceremony" });
  await form.getByRole("combobox", { name: "Ceremony" }).selectOption("review");
  await form.getByRole("button", { name: "Record the ceremony" }).click();
  await expect(c.getByRole("alert")).toHaveText(
    "The service refused it: this sprint may have held that ceremony already.",
  );
  await c.getByRole("button", { name: "Commit the sprint's tasks" }).click();
  await answer(page, true);
  await expect(c.getByRole("alert")).toHaveText(
    "The service refused it: this sprint is committed already.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("following the board: asked for, an event reads the board again, and stopped", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, `/plans/${PID}/board`);
  const live = page.getByTestId("board-live");
  await expect(live.getByRole("status")).toHaveText("Not following.");
  expect(sent.filter((s) => s.path.endsWith("/live"))).toHaveLength(0);
  const reads = () =>
    sent.filter((s) => s.path === `/api/plans/${PID}/tasks`).length;
  const before = reads();
  await live
    .getByRole("button", { name: "Follow changes as they happen" })
    .click();
  await expect(live.getByRole("status")).toContainText("Last: a task moved at");
  await expect.poll(reads).toBeGreaterThan(before);
  await live.getByRole("button", { name: "Stop following" }).click();
  await expect(live.getByRole("status")).toContainText("Not following.");
  await expect(
    live.getByRole("button", { name: "Follow changes as they happen" }),
  ).toBeVisible();
});

test("a placeholder: the person checked first, over capacity asked first, removed after a question", async ({
  page,
}) => {
  const sent = await stub(
    page,
    { [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: 409 },
    {
      [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: "over_capacity",
    },
  );
  await open(page, "/resources");
  const ph = page.getByTestId("placeholder-actions");
  const form = ph.getByRole("form", { name: "Give a placeholder to a person" });
  await expect(form.getByLabel(/^Person/)).toBeDisabled();
  await form.getByRole("combobox", { name: /^Placeholder/ }).selectOption(A1);
  await expect(
    form.getByRole("combobox", { name: /^Placeholder/ }),
  ).toContainText("dev on Website replatform");
  await form.getByLabel(/^Person/).fill("Ada");
  await form.getByRole("button", { name: "Give it to the person" }).click();
  await expect(ph.getByRole("alert")).toHaveText(
    "Give the person as person: or worker: and an id.",
  );
  await form.getByLabel(/^Person/).fill(PERSON);
  await form.getByRole("button", { name: "Give it to the person" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    `Giving this to ${PERSON} would put them over capacity on some days. Do it anyway?`,
  );
  await answer(page, false);
  await expect(ph.getByRole("status")).toHaveText(
    "Not given: nothing was changed.",
  );
  await page.unroute(/\/api\/proxy\/api\//);
  const again = await stub(
    page,
    { [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: 409 },
    {
      [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: "over_capacity",
    },
  );
  await form.getByRole("button", { name: "Give it to the person" }).click();
  await answer(page, true);
  await expect(ph.getByRole("status")).toHaveText(`Given to ${PERSON}.`);
  await form.getByRole("combobox", { name: /^Placeholder/ }).selectOption(A1);
  await form.getByRole("button", { name: "Remove the placeholder" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Remove this placeholder: dev on Website replatform",
  );
  await answer(page, false);
  await form.getByRole("button", { name: "Remove the placeholder" }).click();
  await answer(page, true);
  await expect(ph.getByRole("status")).toHaveText("Removed the placeholder.");
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/allocations/${A1}/resolve`,
      body: { person_ref: PERSON },
    },
  ]);
  expect(writes(again)).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/allocations/${A1}/resolve`,
      body: { person_ref: PERSON },
    },
    {
      method: "POST",
      path: `/api/plans/${PID}/allocations/${A1}/resolve`,
      body: { person_ref: PERSON, confirm: true },
    },
    {
      method: "DELETE",
      path: `/api/plans/${PID}/allocations/${A1}`,
      body: undefined,
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused placeholder is the catalogue's sentence", async ({ page }) => {
  await stub(
    page,
    { [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: 403 },
    {
      [`POST /api/plans/${PID}/allocations/${A1}/resolve`]: "forbidden",
    },
  );
  await open(page, "/resources");
  const ph = page.getByTestId("placeholder-actions");
  const form = ph.getByRole("form", { name: "Give a placeholder to a person" });
  await form.getByRole("combobox", { name: /^Placeholder/ }).selectOption(A1);
  await form.getByLabel(/^Person/).fill(PERSON);
  await form.getByRole("button", { name: "Give it to the person" }).click();
  await expect(ph.getByRole("alert")).toHaveText(
    "Only an administrator or a resource manager can do this.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("hours, skill gaps and the rate in force: checked first, the defaults not sent", async ({
  page,
}) => {
  const sent = await stub(page, {}, {});
  await open(page, "/resources");
  const h = page.getByTestId("planned-hours");
  const hf = h.getByRole("form", { name: "Planned against recorded hours" });
  await hf.getByLabel("From").fill("2026-11-20");
  await hf.getByLabel("To").fill("2026-11-01");
  await hf.getByRole("button", { name: "Read the hours" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "The last day cannot be before the first.",
  );
  await hf.getByLabel("From").fill("");
  await hf.getByLabel("To").fill("");
  await hf.getByRole("button", { name: "Read the hours" }).click();
  await expect(h).toContainText("Planned 90 h; recorded 75 h.");
  await expect(h).toContainText(
    "People are not shown: you may not manage them.",
  );
  await expect(
    h.getByRole("row", {
      name: /^Website replatform\s+80 h\s+16 h\s+75 h\s+-5 h/,
    }),
  ).toBeVisible();
  await expect(
    h.getByRole("row", {
      name: /^Data platform\s+10 h\s+0 h\s+nothing recorded/,
    }),
  ).toBeVisible();
  await expect(h).toContainText("Data platform: 2 h recorded (1 person)");

  const g = page.getByTestId("skill-gap");
  await g
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Website replatform" });
  await g.getByRole("button", { name: "Read the skill gaps" }).click();
  await expect(g).toContainText("1 skill held; 1 skill missing; 1 not known.");
  await expect(g.getByRole("row", { name: /kafka\s+▲ missing/ })).toBeVisible();
  await expect(
    g.getByRole("row", { name: /sql\s+\? not known/ }),
  ).toBeVisible();

  const r = page.getByTestId("rate-lookup");
  const rf = r.getByRole("form", { name: "The cost rate in force" });
  await rf.getByRole("button", { name: "Read the rate" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "Give the person's reference. Give the day as a real date.",
  );
  await rf.getByLabel("Person", { exact: true }).fill(PERSON);
  await rf.getByLabel("On", { exact: true }).fill("2026-11-18");
  await rf.getByRole("button", { name: "Read the rate" }).click();
  await expect(r.getByTestId("rate-answer")).toHaveText(
    "£65.00 an hour on 18 Nov 2026, from their role's table, in force since 1 Apr 2026.",
  );
  expect(
    sent
      .filter(
        (s) => s.method === "GET" && /hours|skill-gap|lookup/.test(s.path),
      )
      .map((s) => s.path),
  ).toEqual([
    "/api/resource-plan/hours",
    `/api/plans/${PID}/skill-gap`,
    `/api/cost-rates/lookup?person_ref=${encodeURIComponent(PERSON)}&date=2026-11-18`,
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a rate kept from the reader says so in the catalogue's words", async ({
  page,
}) => {
  await stub(
    page,
    { "GET /api/cost-rates/lookup": 404 },
    { "GET /api/cost-rates/lookup": "not_found" },
  );
  await open(page, "/resources");
  const r = page.getByTestId("rate-lookup");
  const rf = r.getByRole("form", { name: "The cost rate in force" });
  await rf.getByLabel("Person", { exact: true }).fill(PERSON);
  await rf.getByLabel("On", { exact: true }).fill("2026-11-18");
  await rf.getByRole("button", { name: "Read the rate" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "No rate you can see: rates are shown to finance only.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("effort against capacity: a window checked first, the team and 28 days not sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, "/capacity");
  const u = page.getByTestId("utilization");
  await u.getByText("Effort against capacity").click();
  const f = u.getByRole("form", { name: "Effort against capacity" });
  await f.getByLabel("Days back from today").fill("400");
  await f.getByRole("button", { name: "Read the use of capacity" }).click();
  await expect(u.getByRole("alert")).toHaveText(
    "Give the window as a whole number of days from 1 to 366.",
  );
  await f.getByLabel("Days back from today").fill("28");
  await f.getByRole("button", { name: "Read the use of capacity" }).click();
  await expect(u.getByTestId("utilization-team")).toHaveText(
    "80% · 210 h of 200 h available, 24 h away",
  );
  await expect(u).toContainText("6 people counted.");
  await f.getByRole("combobox", { name: "Show" }).selectOption("person");
  await f.getByLabel("Days back from today").fill("90");
  await f.getByRole("button", { name: "Read the use of capacity" }).click();
  await expect(
    u.getByRole("row", {
      name: new RegExp(`^${PERSON}\\s+▲ 105% at or over capacity`),
    }),
  ).toBeVisible();
  await expect(
    u.getByRole("row", { name: /^person:y\s+too little to report/ }),
  ).toBeVisible();
  expect(
    sent
      .filter((s) => s.path.startsWith("/api/capacity/utilization"))
      .map((s) => s.path),
  ).toEqual([
    "/api/capacity/utilization",
    "/api/capacity/utilization?by=person&window_days=90",
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a plan's goal score, an objective's key results, a key result's check-ins and a KPI's readings", async ({
  page,
}) => {
  await stub(page);
  await open(page, "/okrs");
  const s = page.getByTestId("plan-okr");
  await s.getByRole("button", { name: "Read the score" }).click();
  await expect(s.getByRole("alert")).toHaveText("Choose a plan.");
  await s
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Website replatform" });
  await s.getByRole("button", { name: "Read the score" }).click();
  await expect(s.getByTestId("plan-okr-score")).toContainText("Score 50%");
  await expect(s).toContainText(
    "1 objective is not measured and is left out of the score.",
  );
  await expect(s).toContainText(
    "Faster releases: 50% · weight 3 · 1 key result",
  );
  await expect(s).toContainText(
    "An objective (99999999): not measured · weight 1",
  );
  await s.getByText("Key results of Faster releases").click();
  await expect(s).toContainText("Lead time under 5 days: 60%");
  await s.getByText("Key results of An objective (99999999)").click();
  await expect(s).toContainText(
    "Hold uptime: no progress: it is held steady with no tolerance",
  );

  const ci = page.getByTestId("check-ins").first();
  await ci.getByText("Check-ins for Lead time under 5 days").click();
  await expect(ci).toContainText("10 Nov 2026: value 7");
  await expect(ci).toContainText("Two teams on the new pipeline");
  await expect(page.getByText("service text")).toHaveCount(0);

  await open(page, "/kpis");
  await page.getByText(/^History/).click();
  await expect(page.getByText("Every reading, newest first.")).toBeVisible();
  await expect(
    page.getByRole("row", {
      name: /^17 Nov 2026\s+9\s+replaced by a later reading that day\s+typo/,
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("row", { name: /^17 Nov 2026\s+12\s+counted/ }),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the new sections read in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, `/plans/${PID}/board`, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Newidiadau wrth iddynt ddigwydd" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Seremonïau Sprint 7" }),
  ).toBeVisible();
  await expect(
    page.getByText("Newid llawer o dasgau ar unwaith"),
  ).toBeVisible();
  await open(page, "/resources", "cy-001");
  await expect(
    page.getByRole("heading", {
      name: "Oriau a gynlluniwyd a'r rhai a gofnodwyd",
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Y gyfradd gost mewn grym" }),
  ).toBeVisible();
  await open(page, "/okrs", "cy-001");
  await expect(
    page.getByRole("heading", { name: "Sgôr nodau cynllun" }),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("passes axe with the new sections open and read", async ({ page }) => {
  await stub(page);
  const audit = async () => {
    await settle(page);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toEqual([]);
  };
  await open(page, `/plans/${PID}/board`);
  const b = await openBulk(page);
  await b.getByRole("checkbox", { name: "Design" }).check();
  await page
    .getByTestId("board-live")
    .getByRole("button", { name: "Follow changes as they happen" })
    .click();
  await expect(
    page.getByTestId("board-live").getByRole("status"),
  ).toContainText("Last:");
  await audit();
  await open(page, "/resources");
  await page
    .getByTestId("planned-hours")
    .getByRole("button", { name: "Read the hours" })
    .click();
  await expect(page.getByTestId("planned-hours")).toContainText("Planned 90 h");
  await page
    .getByTestId("skill-gap")
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Website replatform" });
  await page
    .getByTestId("skill-gap")
    .getByRole("button", { name: "Read the skill gaps" })
    .click();
  await expect(page.getByTestId("skill-gap")).toContainText("1 not known.");
  await audit();
  await open(page, "/capacity");
  await page
    .getByTestId("utilization")
    .getByText("Effort against capacity")
    .click();
  await page
    .getByTestId("utilization")
    .getByRole("button", { name: "Read the use of capacity" })
    .click();
  await expect(page.getByTestId("utilization-team")).toBeVisible();
  await audit();
  await open(page, "/okrs");
  await page
    .getByTestId("plan-okr")
    .getByRole("combobox", { name: /^Plan/ })
    .selectOption({ label: "Website replatform" });
  await page
    .getByTestId("plan-okr")
    .getByRole("button", { name: "Read the score" })
    .click();
  await expect(page.getByTestId("plan-okr-score")).toBeVisible();
  await audit();
});
