// The scheduling-canon views (SC-11, plus KPIs, OKRs and milestones) in a real
// browser over a page.route-stubbed API: what they say, that "not computable" is
// never a pass, that leveling needs an explicit confirm, and an axe audit of
// every page. No Rust service needed.

import { test, expect, type Page, type Route } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const PLAN = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const T1 = "11111111-1111-4111-8111-111111111111";
const T2 = "22222222-2222-4222-8222-222222222222";

type Handler = (route: Route, body: unknown) => unknown;

/** Stub the API: `routes` maps "METHOD /api/path" to a JSON answer or a handler. */
async function stub(page: Page, routes: Record<string, unknown | Handler>) {
  await page.route("**/api/**", async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const key = `${req.method()} ${path}`;
    const hit = routes[key];
    if (hit === undefined)
      return route.fulfill({ status: 404, json: { error: "not_found", key } });
    const body = req.postData() ? JSON.parse(req.postData() ?? "null") : null;
    const out = typeof hit === "function" ? (hit as Handler)(route, body) : hit;
    return route.fulfill({ json: out });
  });
}

async function audit(page: Page) {
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
    .analyze();
  const bad = results.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical",
  );
  expect(
    bad.map(
      (v) =>
        `${v.id}: ${v.help} — ${v.nodes.map((n) => `${n.target.join(" ")} [${n.any[0]?.message ?? ""}]`).join("; ")}`,
    ),
  ).toEqual([]);
}

const HEALTH = {
  name: "Schedule health self-check",
  disclaimer:
    "This is a self-check in the style of the DCMA 14-Point Assessment. It is not a DCMA assessment and does not show DCMA compliance.",
  preset: "dcma-default",
  thresholds: { values: {}, source: "default" },
  plan_pid: PLAN,
  as_of: "2026-10-05",
  calendar: "default",
  baseline: null,
  tasks: 3,
  passed: 1,
  failed: 1,
  not_computable: 1,
  all_pass: false,
  note: "A check that cannot be computed never counts as a pass.",
  checks: [
    {
      id: 1,
      key: "missing_logic",
      name: "Missing logic",
      of_what: "incomplete tasks",
      count: 0,
      of: 3,
      value: 0,
      threshold: { op: "<=", limit: 5, unit: "percent" },
      pass: true,
      reason: null,
      offenders: [],
      note: null,
    },
    {
      id: 8,
      key: "high_duration",
      name: "High duration",
      of_what: "incomplete tasks that have a duration",
      count: 1,
      of: 3,
      value: 33.3,
      threshold: { op: "<=", limit: 5, unit: "percent" },
      pass: false,
      reason: null,
      offenders: [{ pid: T1, title: "Build the thing" }],
      note: null,
    },
    {
      id: 14,
      key: "bei",
      name: "Baseline execution index",
      of_what: "tasks baselined to finish by the status date",
      count: 0,
      of: 0,
      value: null,
      threshold: { op: ">=", limit: 950, unit: "milli" },
      pass: null,
      reason: "no_baseline",
      offenders: [],
      note: null,
    },
  ],
};

test("schedule health: a check that cannot be computed is not a pass, and offenders link to the task", async ({
  page,
}) => {
  await stub(page, { [`GET /api/plans/${PLAN}/schedule-health`]: HEALTH });
  await page.goto(`/en-001/plans/${PLAN}/schedule-health`);
  await expect(page.getByText("not a DCMA assessment")).toBeVisible();
  await expect(
    page.getByText("1 passed, 1 failed, 1 not computable."),
  ).toBeVisible();
  const bei = page.getByRole("row", { name: /Baseline execution index/ });
  await expect(bei).toContainText("Not computable");
  await expect(bei).toContainText(
    "Not enough data: No schedule baseline has been taken",
  );
  await expect(bei).not.toContainText(/\bPass\b/);
  await page.getByText("1 task", { exact: true }).click();
  await expect(
    page.getByRole("link", { name: "Build the thing" }),
  ).toHaveAttribute("href", new RegExp(`/critical-path#task-${T1}$`));
  await audit(page);
});

const PROPOSAL = {
  advisory: true,
  calendar: "default",
  allow_critical: false,
  load_percent: 100,
  moves: [
    {
      pid: T1,
      title: "Build",
      person: "person-1",
      person_ref: "person:abc",
      from_start: "2026-10-05",
      to_start: "2026-10-07",
      scheduled_start: "2026-10-07",
      from_finish: "2026-10-06",
      to_finish: "2026-10-08",
      delay_days: 2,
      critical: false,
      float_before: 3,
      float_after: 1,
      because_of: "2026-10-05",
    },
    {
      pid: T2,
      title: "Test",
      person: "person-1",
      person_ref: "person:abc",
      from_start: "2026-10-09",
      to_start: "2026-10-12",
      scheduled_start: "2026-10-12",
      from_finish: "2026-10-09",
      to_finish: "2026-10-12",
      delay_days: 1,
      critical: false,
      float_before: 2,
      float_after: 1,
      because_of: "2026-10-09",
    },
  ],
  effects: [],
  over_allocated: [
    {
      person: "person-1",
      person_ref: "person:abc",
      from: "2026-10-05",
      to: "2026-10-06",
      demand: 200,
      capacity: 100,
      tasks: [{ pid: T1, title: "Build" }],
    },
  ],
  unresolved: [],
  finish_before: "2026-10-16",
  finish_after: "2026-10-16",
  finish_delay_days: 0,
  critical_moved: false,
  truncated: false,
  note: "This is advice; nothing has been changed.",
};

test("leveling: nothing is applied until the moves are ticked, confirmed and sent", async ({
  page,
}) => {
  let applied: { moves: { task_pid: string }[]; confirm: boolean } | null =
    null;
  await stub(page, {
    [`GET /api/plans/${PLAN}/leveling`]: PROPOSAL,
    [`POST /api/plans/${PLAN}/leveling/apply`]: (_r: Route, body: unknown) => {
      applied = body as typeof applied;
      return {
        requested: 1,
        ok: 1,
        refused: 0,
        results: [{ pid: T1, result: "ok", note: null }],
        finish_before: "2026-10-16",
        finish_after: "2026-10-16",
        finish_delay_days: 0,
      };
    },
  });
  await page.goto(`/en-001/plans/${PLAN}/leveling`);
  await expect(page.getByText("This is advice.")).toBeVisible();
  await expect(page.getByText("person:abc is asked for 200%")).toBeVisible();
  const apply = page.getByRole("button", { name: /^Apply \d+ move/ });
  await expect(apply).toBeDisabled();
  await page.getByLabel(/I have read the proposal/).check();
  await expect(apply).toBeEnabled();
  await page.getByRole("checkbox", { name: /Apply: Test/ }).uncheck();
  await expect(
    page.getByRole("button", { name: "Apply 1 move" }),
  ).toBeEnabled();
  await audit(page);
  await page.getByRole("button", { name: "Apply 1 move" }).click();
  await expect(page.getByText(/Applied 1 of 1 move/)).toBeVisible();
  expect(applied).not.toBeNull();
  expect(applied?.confirm).toBe(true);
  expect(applied?.moves.map((m) => m.task_pid)).toEqual([T1]);
});

const RESOURCES = {
  scope: { kind: "portfolio" },
  from: "2026-10-05",
  to: "2026-10-11",
  period: "week",
  group_by: "role",
  overlapping: false,
  periods: [
    {
      from: "2026-10-05",
      to: "2026-10-11",
      days: 7,
      total_demand_fte: 2.5,
      total_placeholder_fte: 1,
      total_proposed_fte: 0,
      rows: [
        {
          key: "dev",
          demand_fte: 2.5,
          named_fte: 1.5,
          placeholder_fte: 1,
          proposed_fte: 0,
          supply_fte: 2,
          gap_fte: -0.5,
          headcount: 2,
        },
      ],
    },
  ],
  placeholders: [
    {
      allocation_pid: "p1",
      plan_pid: PLAN,
      role: "dev",
      percent: 100,
      start_date: "2026-10-05",
      end_date: "2026-10-11",
      booking: "committed",
    },
  ],
  people_by_role: null,
  masked: true,
  note: "FTE is percent-days divided by 100 times the days in the period.",
};

test("resources: a shortage is said in words and FTE, with a table twin", async ({
  page,
}) => {
  await stub(page, { "GET /api/resource-plan": RESOURCES });
  await page.goto("/en-001/resources");
  const cell = page.getByRole("cell", { name: /Short 0\.5 FTE/ });
  await expect(cell).toBeVisible();
  await expect(page.getByText("roles and totals only")).toBeVisible();
  await page.getByText("Every figure as a table").click();
  await expect(page.getByRole("cell", { name: "-0.5" })).toBeVisible();
  await expect(
    page.getByText("Roles asked for with no person yet"),
  ).toBeVisible();
  await audit(page);
});

const RISK = {
  plan_pid: PLAN,
  calendar: "default",
  assumptions: {
    seed: 1,
    iterations: 2000,
    max_iterations: 20000,
    distribution: "triangular",
    fallback_band_percent: 20,
    correlation: "none",
    independence:
      "Every task duration and every risk event is drawn independently: there is no correlation between them.",
    durations: "Whole working days of the plan calendar.",
    tasks_using_fallback: [T2],
    fallback:
      "A task with no estimate is simulated at its likely duration plus and minus the band.",
  },
  cpm_finish: "2026-10-12",
  percentiles: [
    { p: 10, finish: "2026-10-11" },
    { p: 50, finish: "2026-10-13" },
    { p: 80, finish: "2026-10-15" },
    { p: 90, finish: "2026-10-16" },
  ],
  cpm_finish_confidence_bp: 2500,
  earliest: "2026-10-10",
  latest: "2026-10-20",
  histogram: [
    { date: "2026-10-11", count: 300 },
    { date: "2026-10-12", count: 500 },
    { date: "2026-10-13", count: 600 },
    { date: "2026-10-15", count: 400 },
    { date: "2026-10-16", count: 200 },
  ],
  tasks: [
    {
      pid: T1,
      title: "Build",
      optimistic: 3,
      likely: 5,
      pessimistic: 9,
      source: "estimated",
      widened: false,
      criticality_bp: 9000,
      sensitivity: 0.8,
    },
    {
      pid: T2,
      title: "Test",
      optimistic: 3,
      likely: 4,
      pessimistic: 5,
      source: "fallback",
      widened: false,
      criticality_bp: 2000,
      sensitivity: null,
    },
  ],
  events: {
    used: [],
    skipped: [
      { risk_pid: "r1", title: "Vendor slips", reason: "no_probability" },
    ],
    risks_without_schedule_fields: 1,
  },
  unscheduled: [],
  note: "P50, P80 and P90 are finish dates.",
};

test("schedule risk: P50, P80 and P90 are labelled, the figures have a table twin, and the assumptions are stated", async ({
  page,
}) => {
  await stub(page, { [`GET /api/plans/${PLAN}/schedule-risk`]: RISK });
  await page.goto(`/en-001/plans/${PLAN}/schedule-risk`);
  const svg = page.getByRole("img", {
    name: /Histogram of simulated finish dates/,
  });
  await expect(svg).toBeVisible();
  for (const label of ["P50", "P80", "P90"])
    await expect(svg.getByText(label, { exact: true })).toBeVisible();
  await expect(
    page.getByText(
      "25% of the simulated outcomes finish on or before 2026-10-12.",
    ),
  ).toBeVisible();
  await expect(page.getByText(/drawn independently/)).toBeVisible();
  await expect(
    page.getByText("No estimate: using the fallback band"),
  ).toBeVisible();
  await expect(page.getByText("Vendor slips: no probability")).toBeVisible();
  await page.getByText("The same figures as a table").click();
  await expect(page.getByRole("cell", { name: "300" })).toBeVisible();
  await audit(page);
});

test("milestones, KPIs and OKRs pass the axe audit", async ({ page }) => {
  await stub(page, {
    "GET /api/milestones": {
      as_of: "2026-10-05",
      due_soon_days: 14,
      at_risk_days: 5,
      summary: {
        by_status: { late: 0, due_soon: 1, upcoming: 0, achieved: 0 },
        by_schedule_state: { no_forecast: 1 },
      },
      note: "",
      milestones: [
        {
          pid: "m1",
          plan_pid: PLAN,
          name: "Beta",
          kind: "milestone",
          due: "2026-10-12",
          completion_mode: "manual",
          achieved: false,
          achieved_by: null,
          achieved_on: null,
          done_reason: null,
          status: "due_soon",
          schedule: {
            state: "no_forecast",
            forecast: null,
            slack_days: null,
            reason: "no_linked_tasks",
            partial: false,
            unforecast_tasks: [],
          },
          linked_tasks: 0,
          open_tasks: 0,
          links: [],
          slip: {
            original_due: "2026-10-12",
            reschedules: 0,
            later_moves: 0,
            net_days: 0,
          },
        },
      ],
    },
    "GET /api/kpis": {
      as_of: "2026-10-05",
      note: "",
      summary: { green: 0, amber: 0, red: 0, no_data: 1 },
      kpis: [
        {
          kpi: {
            pid: "k1",
            plan_pid: null,
            objective_pid: null,
            name: "Uptime",
            description: null,
            unit: "percent",
            direction: "higher_better",
            target_value: 95,
            red_value: 80,
            cadence: "weekly",
            owner_ref: null,
          },
          status: "no_data",
          latest: null,
          trend: null,
          stale: true,
          readings: 0,
        },
      ],
    },
    "GET /api/okrs": {
      period: null,
      as_of: "2026-10-05T00:00:00Z",
      note: "",
      objectives: [
        {
          objective: {
            pid: "o1",
            title: "Grow",
            description: null,
            period: "2026-Q4",
          },
          score_basis_points: 4000,
          measured: true,
          single_currency: true,
          key_results: [
            {
              key_result: {
                pid: "kr1",
                objective_pid: "o1",
                title: "Users",
                metric: "number",
                direction: "increase",
                start_value: 0,
                target_value: 100,
                current_value: 40,
                tolerance: null,
                unit: "users",
                currency: null,
                owner_ref: null,
                due_date: null,
              },
              progress_basis_points: 4000,
              unmeasurable: null,
              latest_check_in: {
                observed_at: "2026-10-04T00:00:00Z",
                value: 40,
                confidence: 70,
                note: null,
              },
            },
          ],
        },
      ],
    },
  });
  for (const path of ["/en-001/milestones", "/en-001/kpis", "/en-001/okrs"]) {
    await page.goto(path);
    await expect(page.locator("h1")).toBeVisible();
    await audit(page);
  }
  await page.goto("/en-001/kpis");
  await expect(page.getByText("No data yet").first()).toBeVisible();
});

const COST_ASSUMPTIONS = {
  seed: 1,
  iterations: 5000,
  max_iterations: 50000,
  distribution: "triangular",
  fallback_band_percent: 20,
  correlation: "none",
  independence:
    "Every line cost and every risk event is drawn independently: there is no correlation between them.",
  floor: "What is already spent is never given back.",
  currencies:
    "Each currency is simulated and reported on its own; currencies are never added or converted.",
  budget: "The budget is the sum of the lines' planned amounts.",
  fallback:
    "A line with no low or high estimate is simulated at its planned amount plus and minus the band.",
};

const COST_RISK = {
  scope: { kind: "plan", plan_pid: PLAN, plans: 1 },
  assumptions: COST_ASSUMPTIONS,
  currencies: [
    {
      currency: "GBP",
      budget_minor: 1_500_000,
      spent_minor: 200_000,
      mean_minor: 1_620_000,
      percentiles: [
        { p: 10, minor: 1_450_000 },
        { p: 50, minor: 1_600_000 },
        { p: 80, minor: 1_720_000 },
        { p: 90, minor: 1_800_000 },
      ],
      budget_confidence_bp: 3500,
      min_minor: 1_300_000,
      max_minor: 2_000_000,
      histogram: [1_300_000, 1_435_000, 1_570_000, 1_705_000, 1_840_000].map(
        (from, i) => ({
          from_minor: from,
          to_minor: from + 135_000,
          count: [100, 600, 1500, 500, 100][i] ?? 0,
        }),
      ),
      lines: [
        {
          pid: "l1",
          plan: PLAN,
          description: "Platform build",
          category: "capex",
          low: 800_000,
          likely: 1_000_000,
          high: 1_400_000,
          actual: 200_000,
          source: "estimated",
          widened: false,
          sensitivity: 0.9,
        },
        {
          pid: "l2",
          plan: PLAN,
          description: "Training",
          category: "opex",
          low: 400_000,
          likely: 500_000,
          high: 600_000,
          actual: 0,
          source: "fallback",
          widened: false,
          sensitivity: 0.3,
        },
      ],
      events: [
        {
          risk: "r1",
          plan: PLAN,
          title: "Vendor price rise",
          probability_bp: 3000,
          impact_minor: 150_000,
          hit_bp: 2950,
        },
      ],
      plans: [
        {
          plan: PLAN,
          name: "Apollo",
          budget_minor: 1_500_000,
          mean_minor: 1_620_000,
        },
      ],
    },
  ],
  events: {
    used: [
      {
        risk_pid: "r1",
        plan_pid: PLAN,
        title: "Vendor price rise",
        probability_percent: 30,
        cost_impact_minor: 150_000,
        currency: "GBP",
        also_a_schedule_event: false,
      },
    ],
    skipped: [
      { risk_pid: "r2", title: "Unquantified", reason: "no_probability" },
    ],
    risks_without_cost_fields: 2,
  },
  note: "Percentiles are the cost that share of iterations came in at or under.",
};

test("cost risk: budget confidence, labelled P50/P80/P90 and budget, a table twin, and the fallback marked", async ({
  page,
}) => {
  await stub(page, { [`GET /api/plans/${PLAN}/cost-risk`]: COST_RISK });
  await page.goto(`/en-001/plans/${PLAN}/cost-risk`);
  await expect(
    page.getByText(
      /35% of the simulated outcomes cost no more than the budget/,
    ),
  ).toBeVisible();
  const svg = page.getByRole("img", {
    name: /Histogram of simulated GBP cost/,
  });
  for (const label of ["P50", "P80", "P90", "budget"])
    await expect(svg.getByText(label, { exact: true })).toBeVisible();
  await expect(
    page.getByText("No estimate: using the fallback band"),
  ).toBeVisible();
  await expect(
    page.getByText(/stated 30%, struck in 29\.5% of runs/),
  ).toBeVisible();
  await expect(page.getByText("Unquantified: no probability")).toBeVisible();
  await expect(page.getByText(/drawn independently/)).toBeVisible();
  await page.getByText("The same figures as a table").click();
  await expect(page.getByRole("cell", { name: "1500", exact: true })).toBeVisible();
  await audit(page);
});

test("portfolio cost risk lists each plan's mean beside its budget and passes axe", async ({
  page,
}) => {
  await stub(page, {
    "GET /api/cost-risk": {
      ...COST_RISK,
      scope: { kind: "portfolio", plans: 3 },
    },
  });
  await page.goto("/en-001/cost-risk");
  await expect(page.getByText("All 3 plans you can read")).toBeVisible();
  await expect(page.getByRole("heading", { name: "By plan" })).toBeVisible();
  await expect(page.getByRole("rowheader", { name: "Apollo" })).toBeVisible();
  await audit(page);
});

const JOINT = {
  plan_pid: PLAN,
  currency: "GBP",
  other_currencies_not_simulated: ["USD"],
  calendar: "default",
  assumptions: {
    ...COST_ASSUMPTIONS,
    iterations: 2000,
    delay_cost: {
      per_day_minor: 50_000,
      currency: "GBP",
      rule: "Each working day the finish is later than the critical-path finish adds this much.",
    },
    shared_events: ["r1"],
    shared_events_rule:
      "A risk with both a schedule impact and a cost impact is one event with one probability: it strikes once and drives both.",
  },
  schedule: {
    cpm_finish: "2026-10-12",
    percentiles: [
      { p: 10, finish: "2026-10-11" },
      { p: 50, finish: "2026-10-13" },
      { p: 80, finish: "2026-10-15" },
      { p: 90, finish: "2026-10-16" },
    ],
    earliest: "2026-10-10",
    latest: "2026-10-20",
    tasks: [],
  },
  cost: {
    budget_minor: 1_500_000,
    spent_minor: 0,
    mean_minor: 1_640_000,
    mean_delay_cost_minor: 90_000,
    percentiles: [
      { p: 10, minor: 1_450_000 },
      { p: 50, minor: 1_620_000 },
      { p: 80, minor: 1_750_000 },
      { p: 90, minor: 1_830_000 },
    ],
    lines: [
      {
        pid: "l1",
        plan: PLAN,
        description: "Platform build",
        low: 800_000,
        likely: 1_000_000,
        high: 1_400_000,
        actual: 0,
        source: "estimated",
        widened: false,
        sensitivity: 0.9,
      },
    ],
  },
  confidence: { budget_bp: 4000, finish_bp: 3000, joint_bp: 1800 },
  quadrants: { both: 360, budget_only: 440, time_only: 240, neither: 960 },
  points: [
    { days_late: -1, cost_minor: 1_400_000 },
    { days_late: 0, cost_minor: 1_500_000 },
    { days_late: 5, cost_minor: 1_900_000 },
  ],
  events: {
    schedule: { used: [], skipped: [], risks_without_schedule_fields: 0 },
    cost: {
      used: [],
      skipped: [
        { risk_pid: "r2", title: "Unquantified", reason: "no_probability" },
      ],
      risks_without_cost_fields: 0,
    },
  },
  note: "Joint confidence is never above the smaller of the two on their own.",
};

test("joint risk: the joint confidence sits beside both marginals, with a quadrant table and the shared-event rule", async ({
  page,
}) => {
  await stub(page, { [`GET /api/plans/${PLAN}/joint-risk`]: JOINT });
  await page.goto(`/en-001/plans/${PLAN}/joint-risk`);
  await expect(
    page.getByText(
      "Both within budget and on time: 18% (within budget alone 40%; on time alone 30%).",
    ),
  ).toBeVisible();
  await expect(
    page.getByRole("row", { name: /Within budget but late/ }),
  ).toContainText("440");
  await expect(
    page.getByRole("img", { name: /Scatter of simulated total cost/ }),
  ).toBeVisible();
  await expect(
    page.getByText(/1 risk affect both the schedule and the cost/),
  ).toBeVisible();
  await expect(
    page.getByText(/Budget lines in USD are not part of this run/),
  ).toBeVisible();
  await expect(
    page.getByText(
      /Each working day late adds 500\.00 GBP; finishing early earns nothing back\./,
    ),
  ).toBeVisible();
  await expect(
    page.getByText("Cost — Unquantified: no probability"),
  ).toBeVisible();
  await audit(page);
});
