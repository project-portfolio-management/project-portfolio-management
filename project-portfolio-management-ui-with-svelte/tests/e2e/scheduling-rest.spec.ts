// NC-1 slice 12 part (i): scheduling, the rest, in a real browser over a stubbed service. On the critical path the
// tasks of linked plans drawn with a table twin and the network's finish (PJ-13); hammocks listed with their spans,
// made and unmade after a question, linked and unlinked, every rule said and checked first (PJ-7); the plan's readings
// on asking, their queries checked first and the defaults not sent (RM-5, SC-4). On a task's page its dependencies on
// tasks of other plans, the hidden ones counted, added and removed after a question (PJ-13, LT-10), and its sub-tasks
// rolled up (RM-5). On the critical chain each task's aggressive and safe estimates, nothing sent for no change,
// cleared after a question (CC-2), and the frozen buffer plan (CC-3). On the calendar the working day and time away,
// each asked first. On flow the mix of finished work. Each refusal in the catalogue's words; every stub that would
// carry the service's English carries "service text", which never shows. Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const T1 = "aaaaaaaa-0000-4000-8000-000000000001";
const T2 = "aaaaaaaa-0000-4000-8000-000000000002";
const T3 = "aaaaaaaa-0000-4000-8000-000000000003";
const H1 = "aaaaaaaa-0000-4000-8000-0000000000a1";
const P2 = "22222222-2222-4222-8222-222222222222";
const X1 = "bbbbbbbb-0000-4000-8000-000000000001";
const X2 = "bbbbbbbb-0000-4000-8000-000000000002";
const PERSON = "person:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";

interface Sent {
  method: string;
  path: string;
  body?: unknown;
}

const node = (
  pid: string,
  title: string,
  over: Record<string, unknown> = {},
) => ({
  pid,
  title,
  status: "todo",
  early_start: "2026-11-02",
  early_finish: "2026-11-06",
  late_start: "2026-11-02",
  late_finish: "2026-11-06",
  total_float: 0,
  free_float: 0,
  critical: true,
  near_critical: false,
  planned_start: "2026-11-02",
  duration_days: 5,
  pushed_days: 0,
  constraint: null,
  constraint_date: null,
  deadline: null,
  hard_conflict: false,
  calendar: "plan",
  external: false,
  ...over,
});

const CPM = {
  plan_pid: PID,
  calendar: "plan",
  calendar_conflicts: [],
  project_finish: "2026-11-06",
  network_finish: "2026-11-20",
  nodes: [
    node(T1, "Design"),
    node(X1, "Migrate data", {
      external: true,
      plan_pid: P2,
      plan_name: "Data platform",
      early_start: "2026-11-09",
      early_finish: "2026-11-20",
      status: null,
      duration_days: null,
    }),
    node(X2, "Archive", {
      external: true,
      critical: false,
      plan_pid: P2,
      plan_name: "Data platform",
      early_start: "2026-11-09",
      early_finish: "2026-11-10",
      status: null,
      duration_days: null,
    }),
  ],
  edges: [
    {
      pid: "e1",
      predecessor_pid: T1,
      successor_pid: X1,
      kind: "fs",
      lag_days: 2,
      cross_plan: true,
    },
  ],
  unscheduled: [{ pid: T2, title: "Steering", reason: "no_duration" }],
  ignored_edges: [],
  critical_chains: [[T1]],
  chains_truncated: false,
  restricted_edges: [],
  external_truncated: true,
  note: "service text",
};

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
  sprint_pid: null,
  created_at: "2026-11-01T00:00:00Z",
  status_changed_at: "2026-11-01T00:00:00Z",
  done_at: null,
  blocked_days: null,
  is_hammock: false,
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

/** The stub; `refuse["METHOD /path"]` makes the next such call answer with that status. */
async function stub(
  page: Page,
  refuse: Partial<Record<string, number>> = {},
): Promise<Sent[]> {
  const sent: Sent[] = [];
  let hammocks = [
    {
      pid: H1,
      title: "Phase one",
      span: { state: "scheduled", start: "2026-11-02", finish: "2026-11-13" },
      links: [{ pid: T1, title: "Design", is_hammock: false }],
    },
  ];
  let deps: unknown[] = [
    {
      pid: "d1",
      predecessor_pid: T1,
      successor_pid: X1,
      kind: "fs",
      lag_days: 2,
      created_at: "",
      updated_at: "",
    },
    { pid: "d2", restricted: true, label: "service text" },
  ];
  let estimates = [
    {
      pid: T1,
      title: "Design",
      likely_days: 5,
      aggressive: { days: 3, source: "recorded" },
      safe: { days: 6, source: "recorded" },
      state: "complete",
    },
    {
      pid: T3,
      title: "Build",
      likely_days: 8,
      aggressive: null,
      safe: { days: 8, source: "risk_range" },
      state: "missing",
    },
    {
      pid: T2,
      title: "Launch",
      likely_days: 0,
      aggressive: null,
      safe: null,
      state: "milestone",
    },
  ];
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
  await page.route(
    /\/api\/proxy\/api\/(plans|dependencies|working-time|non-working)/,
    async (route) => {
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
        return route.fulfill(
          refused(
            status,
            status === 409
              ? "conflict"
              : status === 404
                ? "not_found"
                : "unprocessable",
          ),
        );
      }
      const base = `/api/plans/${PID}`;
      if (path === "/api/plans")
        return route.fulfill({
          json: [
            { pid: PID, name: "Website replatform" },
            { pid: P2, name: "Data platform" },
          ],
        });
      if (path === `/api/plans/${P2}/tasks`)
        return route.fulfill({
          json: {
            tasks: [task(X1, "Migrate data"), task(X2, "Archive")],
            counts: {},
            total: 2,
          },
        });
      if (path === `${base}/cpm`) return route.fulfill({ json: CPM });
      if (path === `${base}/schedule-baselines`)
        return route.fulfill({ json: [] });
      if (path === `${base}/schedule-variance`)
        return route.fulfill({
          json: {
            baseline: null,
            reason: "no_baseline",
            note: "service text",
            variance: null,
          },
        });
      if (path === `${base}/tasks` && method === "GET")
        return route.fulfill({
          json: {
            tasks: [
              task(T1, "Design"),
              task(T2, "Steering"),
              task(T3, "Build"),
              task(H1, "Phase one", { is_hammock: true }),
            ],
            counts: {},
            total: 4,
          },
        });
      if (path === `${base}/hammocks`)
        return route.fulfill({ json: { hammocks, note: "service text" } });
      if (path === `${base}/tasks/${T2}/hammock`) {
        hammocks = [
          ...hammocks,
          {
            pid: T2,
            title: "Steering",
            span: { state: "unscheduled", reason: "no_links" },
            links: [],
          },
        ];
        return route.fulfill({
          json: { pid: T2, is_hammock: true, changed: true },
        });
      }
      if (path === `${base}/tasks/${H1}/hammock`) {
        hammocks = hammocks.filter((h) => h.pid !== H1);
        return route.fulfill({
          json: { pid: H1, is_hammock: false, changed: true },
        });
      }
      if (path.startsWith(`${base}/hammocks/`) && method === "POST")
        return route.fulfill({ json: { pid: "l2" } });
      if (path.startsWith(`${base}/hammocks/`) && method === "DELETE") {
        hammocks = hammocks.map((h) => ({ ...h, links: [] }));
        return route.fulfill({ json: {} });
      }
      if (path === "/api/dependencies" && method === "GET")
        return route.fulfill({ json: deps });
      if (path === "/api/dependencies" && method === "POST") {
        deps = [
          ...deps,
          { pid: "d3", ...(body as object), kind: "fs", lag_days: 0 },
        ];
        return route.fulfill({ json: { pid: "d3" } });
      }
      if (path.startsWith("/api/dependencies/") && method === "DELETE") {
        deps = deps.filter(
          (d) => (d as { pid: string }).pid !== path.split("/").pop(),
        );
        return route.fulfill({ json: {} });
      }
      if (path === `${base}/tasks/${T1}`)
        return route.fulfill({ json: task(T1, "Design") });
      if (path === `${base}/tasks/${T1}/relations`)
        return route.fulfill({ json: { relations: [], hidden: 0 } });
      if (path === `${base}/tasks/${T1}/links`)
        return route.fulfill({ json: [] });
      if (path === `${base}/issues`)
        return route.fulfill({ json: { issues: [] } });
      if (path === `${base}/tasks/${T1}/rollup`)
        return route.fulfill({
          json: {
            task_pid: T1,
            rollup: {
              descendants: 3,
              done: 1,
              due: { value: "2026-11-27", reason: null },
              estimate_points: { value: 8, reason: null },
              progress_bp: { value: 2500, reason: null },
              excluded_unestimated: [T3],
              spent_minutes: 150,
            },
          },
        });
      if (path === `${base}/rollup`)
        return route.fulfill({
          json: {
            as_of: "",
            root: { pid: PID, name: "Website replatform" },
            note: "service text",
            tree: {
              plans: 2,
              max_depth: 1,
              depth_limit: 32,
              truncated: false,
              truncation_note: "service text",
              revisits: 0,
              revisit_note: null,
            },
            combined: {
              tasks: 10,
              finished: 4,
              work_in_progress: 3,
              not_started: 3,
              cycle_time: { n: 4, p50_days: 2, p85_days: 4.5 },
              lead_time: { n: 4, p50_days: 5, p85_days: 9 },
              aggregate_flow_efficiency: { value: 0.4 },
            },
            by_plan: [
              {
                plan: { pid: PID, name: "Website replatform" },
                depth: 0,
                tasks: 6,
                finished: 2,
                work_in_progress: 2,
                flow_efficiency: { value: 0.5 },
              },
              {
                plan: { pid: P2, name: "Data platform" },
                depth: 1,
                tasks: 4,
                finished: 2,
                work_in_progress: 1,
                flow_efficiency: { value: null },
              },
            ],
          },
        });
      if (path === `${base}/effort`)
        return route.fulfill({
          json: {
            plan_pid: PID,
            plan: {
              minutes: 600,
              capex_minutes: 120,
              opex_minutes: 300,
              unclassified_minutes: 180,
              billable_minutes: 240,
              asserted: true,
            },
            by_assignee: [
              {
                actor_ref: PERSON,
                totals: {
                  minutes: 600,
                  capex_minutes: 0,
                  opex_minutes: 0,
                  unclassified_minutes: 600,
                  billable_minutes: 240,
                  asserted: true,
                },
              },
            ],
            by_task: [
              {
                task_pid: T1,
                totals: {
                  minutes: 600,
                  capex_minutes: 0,
                  opex_minutes: 0,
                  unclassified_minutes: 600,
                  billable_minutes: 240,
                  asserted: true,
                },
              },
            ],
            as_of: "",
          },
        });
      if (path === `${base}/forecast`)
        return route.fulfill({
          json: {
            as_of: "",
            plan: { pid: PID, name: "Website replatform" },
            note: "service text",
            throughput_history: [2, 3, 1, 4, 2, 3],
            open_items: 12,
            batch: {
              items: 12,
              trials: 10000,
              history_periods: 12,
              period_days: 7,
              p50_days: 28,
              p85_days: 35,
              p95_days: 42,
              trials_hit_ceiling: 0,
              reason: null,
              note: "service text",
            },
            horizon: {
              periods: 4,
              days: 28,
              trials: 10000,
              history_periods: 12,
              median_items: 10,
              at_least_items: 7,
              at_most_items: 14,
              reason: "service text",
              note: "service text",
            },
          },
        });
      if (path === `${base}/performance`)
        return route.fulfill({
          json: {
            plan_pid: PID,
            stakeholder: {
              nps: {
                score: 20,
                absent: null,
                responses: 10,
                promoters: 4,
                passives: 4,
                detractors: 2,
              },
            },
            schedule: {
              spi: 9500,
              unit: "basis_points",
              absent: null,
              reason: null,
            },
            financial: {
              cpi: null,
              unit: "basis_points",
              absent: "no_baseline",
              reason: "service text",
            },
            detail: "",
            as_of: "",
          },
        });
      if (path === `${base}/workflow`)
        return route.fulfill({
          json: {
            plan_pid: PID,
            applies_to: "task",
            source: "built_in_or_default",
            constrained: false,
            workflow: {
              states: [
                {
                  key: "todo",
                  label: "service text",
                  labels: {},
                  category: "todo",
                  wip_limit: null,
                  is_initial: true,
                  is_terminal: false,
                  field_rules: [],
                },
                {
                  key: "done",
                  label: "service text",
                  labels: {},
                  category: "done",
                  wip_limit: 5,
                  is_initial: false,
                  is_terminal: true,
                  field_rules: [],
                },
              ],
              transitions: [],
            },
          },
        });
      if (path === `${base}/flow-distribution`)
        return route.fulfill({
          json: {
            plan_pid: PID,
            window_days: Number(url.searchParams.get("window_days") ?? 90),
            rollup: false,
            plans_counted: 1,
            truncated: false,
            distribution: {
              total: 10,
              intent_declared: true,
              shares: [
                {
                  flow_type: "feature",
                  count: 6,
                  basis_points: 6000,
                  intended_basis_points: 5000,
                  gap_basis_points: 1000,
                },
                {
                  flow_type: "defect",
                  count: 4,
                  basis_points: 4000,
                  intended_basis_points: 3000,
                  gap_basis_points: 1000,
                },
                {
                  flow_type: "risk",
                  count: 0,
                  basis_points: 0,
                  intended_basis_points: 1000,
                  gap_basis_points: -1000,
                },
                {
                  flow_type: "debt",
                  count: 0,
                  basis_points: 0,
                  intended_basis_points: 1000,
                  gap_basis_points: -1000,
                },
                {
                  flow_type: "unclassified",
                  count: 0,
                  basis_points: 0,
                  intended_basis_points: null,
                  gap_basis_points: null,
                },
              ],
            },
            as_of: "",
            note: "service text",
          },
        });
      if (path === `${base}/ccpm-estimates`)
        return route.fulfill({
          json: {
            rule: "explicit_only",
            rule_setting_unusable: false,
            total: 3,
            limit: 1000,
            counts: { complete: 1, missing: 1, milestone: 1 },
            needing_attention: [T3],
            tasks: estimates,
            note: "service text",
          },
        });
      if (path.endsWith("/ccpm-estimate") && method === "PUT") {
        const b = body as {
          aggressive_days: number | null;
          safe_days: number | null;
        };
        estimates = estimates.map((e) =>
          path.includes(e.pid)
            ? {
                ...e,
                aggressive:
                  b.aggressive_days === null
                    ? null
                    : { days: b.aggressive_days, source: "recorded" },
                safe:
                  b.safe_days === null
                    ? null
                    : { days: b.safe_days, source: "recorded" },
              }
            : e,
        );
        return route.fulfill({ json: { task_pid: T1, ...b, likely_days: 5 } });
      }
      if (path === `${base}/critical-chain`)
        return route.fulfill({
          json: {
            advisory: true,
            state: "no_critical_chain",
            reason: "no_assignments",
            critical_path: { tasks: [], finish: null, chains: 0 },
            finish_delay_days: 0,
            note: "service text",
          },
        });
      if (path === `${base}/critical-chain-buffers`)
        return route.fulfill({
          json: {
            advisory: true,
            state: "no_buffers",
            reason: "estimates_missing",
            buffer_rule: "cut_and_paste",
            estimate_rule: "explicit_only",
            resource_buffer_lead_days: 2,
            note: "service text",
          },
        });
      if (path === `${base}/buffer-plan`)
        return route.fulfill({
          json: {
            state: "buffer_plan",
            pid: "bp2",
            taken_on: "2026-11-16",
            document: {
              version: 1,
              calendar: "plan",
              buffer_rule: "root_sum_squares",
              project: {
                key: "project",
                days: 4,
                tasks: [
                  { pid: T1, title: "Design", days: 3 },
                  { pid: T3, title: "Build", days: 5 },
                ],
                planned_finish: "2026-12-04",
              },
              feeding: [
                {
                  key: `feeding:${T3}:${T2}`,
                  joins: { pid: T3, title: "Build" },
                  tasks: [{ pid: T2, title: "Steering", days: 2 }],
                  days: null,
                  planned_finish: null,
                },
              ],
            },
            history: [
              { pid: "bp2", taken_on: "2026-11-16" },
              { pid: "bp1", taken_on: "2026-11-02" },
            ],
          },
        });
      if (path === `${base}/calendar`)
        return route.fulfill({
          json: { source: "plan", weekdays: "1111100", holidays: [] },
        });
      if (path === "/api/working-time" || path === "/api/non-working")
        return route.fulfill({ json: { pid: "w1" } });
      // The flow page's own readings: empty, so only the mix is read here.
      if (
        /\/(time-analysis|aging-wip|constraints|flow|cumulative-flow)$/.test(
          path,
        )
      )
        return route.fulfill(refused(404, "not_found"));
      return route.fulfill(refused(404, "not_found"));
    },
  );
  return sent;
}

const writes = (sent: Sent[]) => sent.filter((s) => s.method !== "GET");

async function open(page: Page, path: string, locale = "en-001") {
  await page.goto(`/${locale}/plans/${PID}/${path}`, {
    waitUntil: "networkidle",
  });
  await settle(page);
}

async function answer(page: Page, ok: boolean) {
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: ok ? "OK" : "Cancel" })
    .click();
}

test("the critical path draws the tasks of linked plans, with a table and the network's finish", async ({
  page,
}) => {
  await stub(page);
  await open(page, "critical-path");
  const ext = page.getByTestId("external-tasks");
  await expect(
    ext.getByRole("heading", { name: "Tasks of linked plans" }),
  ).toBeVisible();
  await expect(ext).toContainText(
    "With the linked plans, the network finishes 20 Nov 2026.",
  );
  await expect(ext).toContainText("Not every linked plan is shown");
  await expect(ext).toContainText(
    "Design → Migrate data (Data platform) · finish to start · lag of 2 business days",
  );
  await expect(
    ext.getByRole("figure", { name: "Tasks of Data platform on a timeline" }),
  ).toContainText("◆ Migrate data");
  await expect(
    ext.getByRole("link", { name: "Data platform" }),
  ).toHaveAttribute("href", `/en-001/plans/${P2}/critical-path`);
  await expect(
    ext.getByRole("row", {
      name: /^Archive\s+Data platform\s+9 Nov 2026\s+10 Nov 2026\s+No/,
    }),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("hammocks: the span in words, every rule checked first, made and unmade after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, "critical-path");
  const h = page.getByTestId("hammocks");
  await expect(h).toContainText("It is not in the dependency network");
  await expect(h.getByRole("listitem").first()).toContainText(
    "Phase one Spans 2–13 Nov 2026.",
  );
  const make = h.getByRole("form", { name: "Make a task a hammock" });
  await make.getByRole("button", { name: "Make it a hammock" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "Choose a task to make a hammock.",
  );
  await make.getByLabel("Task").selectOption({ label: "Design" });
  await make.getByRole("button", { name: "Make it a hammock" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "This task has a duration, work or a constraint: clear them on the schedule first.",
  );
  expect(writes(sent)).toEqual([]);
  await make.getByLabel("Task").selectOption({ label: "Steering" });
  await make.getByRole("button", { name: "Make it a hammock" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Make Steering a hammock? Its planned start is cleared",
  );
  await answer(page, false);
  expect(writes(sent)).toEqual([]);
  await make.getByLabel("Task").selectOption({ label: "Steering" });
  await make.getByRole("button", { name: "Make it a hammock" }).click();
  await answer(page, true);
  await expect(h.getByRole("status")).toHaveText("Steering is now a hammock.");
  await expect(h).toContainText("No dates yet: it links to no task.");
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/tasks/${T2}/hammock`,
      body: { enabled: true },
    },
  ]);

  const link = h.getByRole("form", { name: "Link a hammock to a task" });
  await link
    .getByRole("combobox", { name: /^Hammock/ })
    .selectOption({ label: "Phase one" });
  await link
    .getByRole("combobox", { name: /^Task it spans/ })
    .selectOption({ label: "Design" });
  await link.getByRole("button", { name: "Add the link" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "This hammock already spans that task.",
  );
  await link
    .getByRole("combobox", { name: /^Task it spans/ })
    .selectOption({ label: "Phase one (a hammock)" });
  await link.getByRole("button", { name: "Add the link" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "A hammock cannot span itself.",
  );
  await link
    .getByRole("combobox", { name: /^Task it spans/ })
    .selectOption({ label: "Build" });
  await link.getByRole("button", { name: "Add the link" }).click();
  await expect(h.getByRole("status")).toHaveText("Phase one now spans Build.");
  expect(writes(sent).at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/hammocks/${H1}/links`,
    body: { member_pid: T3 },
  });

  await h
    .getByRole("button", { name: "Remove the link from Phase one to Design" })
    .click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Remove the link from Phase one to Design? The task itself is not changed.",
  );
  await answer(page, true);
  // The write follows the answer; wait for it rather than read the list at once.
  await expect
    .poll(() => writes(sent).at(-1))
    .toEqual({
      method: "DELETE",
      path: `/api/plans/${PID}/hammocks/${H1}/links/${T1}`,
    });

  await h
    .getByRole("button", { name: "Make Phase one a task again", exact: true })
    .click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Make Phase one a task again?",
  );
  await answer(page, true);
  await expect(h.getByRole("status")).toHaveText("Phase one is a task again.");
  expect(writes(sent).at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/tasks/${H1}/hammock`,
    body: { enabled: false },
  });
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused hammock change is the catalogue's sentence, not the service's text", async ({
  page,
}) => {
  await stub(page, {
    [`POST /api/plans/${PID}/hammocks/${H1}/links`]: 422,
    [`DELETE /api/plans/${PID}/hammocks/${H1}/links/${T1}`]: 404,
  });
  await open(page, "critical-path");
  const h = page.getByTestId("hammocks");
  const link = h.getByRole("form", { name: "Link a hammock to a task" });
  await link
    .getByRole("combobox", { name: /^Hammock/ })
    .selectOption({ label: "Phase one" });
  await link
    .getByRole("combobox", { name: /^Task it spans/ })
    .selectOption({ label: "Build" });
  await link.getByRole("button", { name: "Add the link" }).click();
  await expect(h.getByRole("alert")).toHaveText(
    "The service would not add the link: the task may be gone from this plan, already spanned, or part of a loop.",
  );
  await h
    .getByRole("button", { name: "Remove the link from Phase one to Design" })
    .click();
  await answer(page, true);
  await expect(h.getByRole("alert")).toHaveText("That link is already gone.");
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the plan's readings are read on asking, their queries checked first and the defaults not sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, "critical-path");
  const r = page.getByTestId("plan-readings");
  const reads = () =>
    sent.filter((s) =>
      /\/(rollup|effort|forecast|performance|workflow)/.test(s.path),
    );
  expect(reads()).toEqual([]);

  const roll = r.getByRole("form", {
    name: "Flow across the plans it contains",
  });
  await roll.getByLabel("Levels down").fill("40");
  await roll.getByRole("button", { name: "Read" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "Levels down is a whole number from 1 to 32.",
  );
  await roll.getByLabel("Levels down").fill("");
  await roll.getByRole("button", { name: "Read" }).click();
  await expect(r).toContainText(
    "2 plans. 10 tasks: 4 finished, 3 in progress, 3 not started.",
  );
  await expect(r).toContainText(
    "within 4.5 days of starting and 9.0 days of being asked for; time spent working rather than waiting: 40%.",
  );
  await expect(
    r.getByRole("row", {
      name: /^Data platform\s+1\s+4\s+2\s+1\s+not measured/,
    }),
  ).toBeVisible();

  await r.getByRole("button", { name: "Read the time recorded" }).click();
  await expect(r).toContainText("10 hrs recorded, of which 4 hrs billable.");
  await expect(r).toContainText(
    "Capital spending 2 hrs, operating spending 5 hrs, not classified 3 hrs.",
  );
  await expect(
    r.getByRole("row", { name: /^Design\s+10 hrs\s+4 hrs/ }),
  ).toBeVisible();

  const fc = r.getByRole("form", { name: "Delivery forecast" });
  await fc.getByLabel("Periods ahead (4 if empty)").fill("0");
  await fc.getByLabel("Periods of history (12 if empty)").fill("3");
  await fc.getByRole("button", { name: "Read the forecast" }).click();
  await expect(r.getByRole("alert")).toHaveText(
    "Periods ahead is a whole number from 1 to 520. Periods of history is a whole number from 6 to 260.",
  );
  await fc.getByLabel("Periods ahead (4 if empty)").fill("4");
  await fc.getByLabel("Periods of history (12 if empty)").fill("26");
  await fc.getByRole("button", { name: "Read the forecast" }).click();
  await expect(r).toContainText(
    "12 items: half the runs finish within 28 days, 85 in 100 within 35 days and 95 in 100 within 42 days.",
  );
  await expect(r).toContainText(
    "In the next 28 days, about 10 items may be done: at least 7 in 85 runs of 100, and at most 14.",
  );

  await r.getByRole("button", { name: "Read the performance" }).click();
  await expect(r).toContainText(/Schedule performance\s*95\.00%/);
  await expect(r).toContainText(
    "Not measured: the plan has no budget baseline.",
  );
  await expect(r).toContainText(
    "20, from 10 answers (4 promoting, 2 detracting)",
  );

  await r.getByRole("button", { name: "Read the task workflow" }).click();
  await expect(r).toContainText(
    "This plan uses the built-in task workflow. A task may move from any state to any other.",
  );
  await expect(r).toContainText(
    "Done · finished · where a task ends · at most 5 at once",
  );

  expect(reads().map((s) => s.path)).toEqual([
    `/api/plans/${PID}/rollup`,
    `/api/plans/${PID}/effort`,
    `/api/plans/${PID}/forecast?history_periods=26`,
    `/api/plans/${PID}/performance`,
    `/api/plans/${PID}/workflow`,
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a task's dependencies on other plans: named, the hidden counted, added and removed after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, `tasks/${T1}`);
  const x = page.getByTestId("cross-plan-links");
  await expect(x.getByTestId("cross-links").getByRole("listitem")).toHaveText([
    /This task comes before Migrate data, in Data platform · finish to start · lag of 2 business days/,
  ]);
  await expect(x).toContainText(
    "1 more dependency joins a task you cannot read.",
  );
  const form = x.getByRole("form", {
    name: "Add a dependency on a task of another plan",
  });
  await expect(form.getByLabel("Task in that plan")).toBeDisabled();
  await form.getByLabel("Lag in business days (0 if empty)").fill("4000");
  await form.getByRole("button", { name: "Add the dependency" }).click();
  await expect(x.getByRole("alert")).toHaveText(
    "Choose the other plan. Choose a task in that plan. A lag is a whole number of business days from minus 3,650 to 3,650.",
  );
  await form.getByLabel("Other plan").selectOption({ label: "Data platform" });
  await form
    .getByLabel("Task in that plan")
    .selectOption({ label: "Migrate data" });
  await form.getByLabel("Lag in business days (0 if empty)").fill("");
  await form.getByRole("button", { name: "Add the dependency" }).click();
  await expect(x.getByRole("alert")).toHaveText(
    "This dependency is already there.",
  );
  expect(writes(sent)).toEqual([]);
  await form.getByLabel("The other task comes first").check();
  await form.getByLabel("Task in that plan").selectOption({ label: "Archive" });
  await form
    .getByLabel("How the two join")
    .selectOption({ label: "start to start" });
  await form.getByRole("button", { name: "Add the dependency" }).click();
  await expect(x.getByRole("status")).toHaveText(
    "Added a dependency on Archive.",
  );
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: "/api/dependencies",
      body: { predecessor_pid: X2, successor_pid: T1, kind: "ss" },
    },
  ]);
  await x
    .getByRole("button", {
      name: "Remove the dependency on Migrate data, in Data platform",
    })
    .click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Remove the dependency: Design comes before Migrate data, in Data platform? Neither task is changed.",
  );
  await answer(page, false);
  expect(writes(sent).filter((s) => s.method === "DELETE")).toEqual([]);
  await x
    .getByRole("button", {
      name: "Remove the dependency on Migrate data, in Data platform",
    })
    .click();
  await answer(page, true);
  await expect(x.getByRole("status")).toHaveText("Removed the dependency.");
  expect(writes(sent).at(-1)).toEqual({
    method: "DELETE",
    path: "/api/dependencies/d1",
  });

  const roll = page.getByTestId("task-rollup");
  await expect(roll).toContainText("1 of 3 done");
  await expect(roll).toContainText("27 Nov 2026");
  await expect(roll).toContainText("25%");
  await expect(roll).toContainText("2 hrs 30 mins");
  await expect(roll).toContainText("1 sub-task has no estimate");
  await expect(roll.getByRole("listitem")).toHaveText(["Build"]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused dependency says why in the catalogue's words", async ({
  page,
}) => {
  await stub(page, {
    "POST /api/dependencies": 422,
    "DELETE /api/dependencies/d1": 404,
  });
  await open(page, `tasks/${T1}`);
  const x = page.getByTestId("cross-plan-links");
  const form = x.getByRole("form", {
    name: "Add a dependency on a task of another plan",
  });
  await form.getByLabel("Other plan").selectOption({ label: "Data platform" });
  await form.getByLabel("Task in that plan").selectOption({ label: "Archive" });
  await form.getByRole("button", { name: "Add the dependency" }).click();
  await expect(x.getByRole("alert")).toHaveText(
    "The service would not add the dependency: it may already be there, make a loop, or join a hammock.",
  );
  await x
    .getByRole("button", { name: /^Remove the dependency on Migrate data/ })
    .click();
  await answer(page, true);
  await expect(x.getByRole("alert")).toHaveText(
    "That dependency is already gone, or you can no longer read both tasks.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("critical-chain estimates: where each came from, nothing sent for no change, cleared after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, "critical-chain");
  const c = page.getByTestId("ccpm-estimates");
  await expect(c).toContainText(
    "Rule: only estimates given are used; nothing is derived.",
  );
  await expect(c).toContainText("1 task needs its estimates.");
  await expect(
    c.getByRole("row", {
      name: /^Build\s+8\s+none\s+8 days, from the risk range\s+▲ Missing an estimate/,
    }),
  ).toBeVisible();
  const form = c.getByRole("form", { name: "Record a task's estimates" });
  await expect(form.getByLabel("Aggressive, in business days")).toBeDisabled();
  await expect(form.getByLabel("Task").locator("option")).toHaveText([
    "Choose a task",
    "Design",
    "Build",
  ]);
  await form.getByLabel("Task").selectOption({ label: "Design" });
  await expect(form.getByLabel("Aggressive, in business days")).toHaveValue(
    "3",
  );
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await expect(c.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await form.getByLabel("Aggressive, in business days").fill("7");
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await expect(c.getByRole("alert")).toHaveText(
    "The aggressive estimate cannot be above the safe one.",
  );
  await form.getByLabel("Aggressive, in business days").fill("4");
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await expect(c.getByRole("status")).toHaveText(
    "Saved the estimates for Design.",
  );
  await form.getByLabel("Aggressive, in business days").fill("");
  await form.getByLabel("Safe, in business days").fill("");
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Clear the estimates recorded for Design?",
  );
  await answer(page, false);
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await answer(page, true);
  await expect(c.getByRole("status")).toHaveText(
    "Cleared the estimates recorded for Design.",
  );
  expect(writes(sent)).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/tasks/${T1}/ccpm-estimate`,
      body: { aggressive_days: 4, safe_days: 6 },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/tasks/${T1}/ccpm-estimate`,
      body: { aggressive_days: null, safe_days: null },
    },
  ]);

  const f = page.getByTestId("frozen-buffer-plan");
  await expect(f).toContainText("Frozen on 16 Nov 2026;");
  await expect(
    f.getByRole("row", {
      name: /^Project buffer\s+4\s+Design → Build\s+4 Dec 2026/,
    }),
  ).toBeVisible();
  await expect(
    f.getByRole("row", {
      name: /^Feeding buffer into Build\s+not sized\s+Steering/,
    }),
  ).toBeVisible();
  await expect(f).toContainText(
    "2 buffer plans frozen, newest first: 16 Nov 2026, 2 Nov 2026.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused estimate is the catalogue's sentence", async ({ page }) => {
  await stub(page, {
    [`PUT /api/plans/${PID}/tasks/${T3}/ccpm-estimate`]: 422,
  });
  await open(page, "critical-chain");
  const c = page.getByTestId("ccpm-estimates");
  const form = c.getByRole("form", { name: "Record a task's estimates" });
  await form.getByLabel("Task").selectOption({ label: "Build" });
  await form.getByLabel("Aggressive, in business days").fill("5");
  await form.getByLabel("Safe, in business days").fill("8");
  await form.getByRole("button", { name: "Save the estimates" }).click();
  await expect(c.getByRole("alert")).toHaveText(
    "The service would not take these estimates: the task may have lost its duration or become a milestone.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the working day and time away are checked first and asked first", async ({
  page,
}) => {
  const sent = await stub(page, { "POST /api/working-time": 409 });
  await open(page, "calendar");
  const w = page.getByTestId("working-time");
  const day = w.getByRole("form", { name: "Set the working day and week" });
  await day.getByLabel("Hours in a day").fill("25");
  await day.getByLabel("Days in a week").fill("8");
  await day.getByRole("button", { name: "Set the working day" }).click();
  await expect(w.getByRole("alert")).toHaveText(
    "Hours are a whole number from 0 to 24. Days in a week are a whole number from 1 to 7.",
  );
  await day.getByLabel("Hours in a day").fill("7");
  await day.getByLabel("Days in a week").fill("5");
  await day.getByRole("button", { name: "Set the working day" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Set this plan's working day to 7 hrs 30 mins, 5 days a week?",
  );
  await answer(page, false);
  expect(writes(sent)).toEqual([]);
  await day.getByRole("button", { name: "Set the working day" }).click();
  await answer(page, true);
  await expect(w.getByRole("alert")).toHaveText(
    "This plan already has a working day set, and it cannot be replaced here.",
  );
  await day.getByLabel("The organization's default").check();
  await day.getByRole("button", { name: "Set the working day" }).click();
  await answer(page, true);
  await expect(w.getByRole("status").first()).toHaveText(
    "Set a working day of 7 hrs 30 mins, 5 days a week.",
  );

  const away = w.getByRole("form", { name: "Time away from project work" });
  await away.getByRole("button", { name: "Record the time away" }).click();
  await expect(w.getByRole("alert")).toHaveText(
    "Give the person as person: or worker: and an id. Give the first day. Give the last day.",
  );
  await away.getByLabel("Person's reference").fill(PERSON);
  await away.getByLabel("First day").fill("2026-12-24");
  await away.getByLabel("Last day").fill("2026-12-21");
  await away.getByRole("button", { name: "Record the time away" }).click();
  await expect(w.getByRole("alert")).toHaveText(
    "The last day cannot come before the first.",
  );
  await away.getByLabel("First day").fill("2026-12-21");
  await away.getByLabel("Last day").fill("2026-12-24");
  await away.getByLabel("Kind").selectOption({ label: "study leave" });
  await away.getByRole("button", { name: "Record the time away" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    `Record study leave for ${PERSON}, 21–24 Dec 2026?`,
  );
  await answer(page, true);
  await expect(w).toContainText(
    `Recorded study leave for ${PERSON}, 21–24 Dec 2026.`,
  );
  expect(writes(sent)).toEqual([
    {
      method: "POST",
      path: "/api/working-time",
      body: {
        scope_ref: `plan:${PID}`,
        minutes_per_day: 450,
        working_days_per_week: 5,
      },
    },
    {
      method: "POST",
      path: "/api/working-time",
      body: { minutes_per_day: 450, working_days_per_week: 5 },
    },
    {
      method: "POST",
      path: "/api/non-working",
      body: {
        person_ref: PERSON,
        starts_on: "2026-12-21",
        ends_on: "2026-12-24",
        kind: "study_leave",
      },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the mix of finished work: a window checked first, the default not sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page, "flow");
  const m = page.getByTestId("flow-mix");
  const form = m.getByRole("form", { name: "The mix of finished work" });
  await form.getByLabel("Days looked back (90 if empty)").fill("0");
  await form.getByRole("button", { name: "Read the mix" }).click();
  await expect(m.getByRole("alert")).toHaveText(
    "The window is a whole number of days from 1 to 3650.",
  );
  await form.getByLabel("Days looked back (90 if empty)").fill("90");
  await form.getByRole("button", { name: "Read the mix" }).click();
  await expect(m).toContainText("Work finished in the last 90 days.");
  await expect(
    m.getByRole("row", { name: /^New features\s+6\s+60%\s+50%\s+10%/ }),
  ).toBeVisible();
  await form.getByLabel("Days looked back (90 if empty)").fill("30");
  await form.getByLabel("Include the plans it contains").check();
  await form.getByRole("button", { name: "Read the mix" }).click();
  await expect(m).toContainText("Work finished in the last 30 days.");
  expect(
    sent.filter((s) => s.path.includes("flow-distribution")).map((s) => s.path),
  ).toEqual([
    `/api/plans/${PID}/flow-distribution`,
    `/api/plans/${PID}/flow-distribution?window_days=30&rollup=true`,
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the new sections read in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "critical-path", "cy-001");
  await expect(
    page.getByRole("heading", {
      name: "Hamogau: tasgau crynodeb sy'n rhychwantu eraill",
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Tasgau cynlluniau cysylltiedig" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Rhagor am y cynllun hwn" }),
  ).toBeVisible();
  await open(page, "critical-chain", "cy-001");
  await expect(
    page.getByRole("heading", { name: "Amcangyfrifon ymosodol a diogel" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Y cynllun byffer wedi'i rewi" }),
  ).toBeVisible();
  await open(page, "calendar", "cy-001");
  await expect(
    page.getByRole("heading", { name: "Y diwrnod a'r wythnos waith" }),
  ).toBeVisible();
  await open(page, `tasks/${T1}`, "cy-001");
  await expect(
    page.getByRole("heading", {
      name: "Dibyniaethau, yn y cynllun hwn neu un arall",
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Is-dasgau wedi'u crynhoi" }),
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
  await open(page, "critical-path");
  const r = page.getByTestId("plan-readings");
  await r
    .getByRole("form", { name: "Flow across the plans it contains" })
    .getByRole("button", { name: "Read" })
    .click();
  await r.getByRole("button", { name: "Read the time recorded" }).click();
  await r.getByRole("button", { name: "Read the performance" }).click();
  await r.getByRole("button", { name: "Read the task workflow" }).click();
  await expect(r).toContainText("This plan uses the built-in task workflow.");
  await audit();
  await open(page, "critical-chain");
  await page
    .getByTestId("ccpm-estimates")
    .getByRole("combobox", { name: /^Task/ })
    .selectOption({ label: "Design" });
  await audit();
  await open(page, "calendar");
  await audit();
  await open(page, `tasks/${T1}`);
  await expect(page.getByTestId("task-rollup")).toContainText("1 of 3 done");
  await audit();
  await open(page, "flow");
  await page
    .getByTestId("flow-mix")
    .getByRole("button", { name: "Read the mix" })
    .click();
  await expect(page.getByTestId("flow-mix")).toContainText(
    "Work finished in the last 90 days.",
  );
  await audit();
});
