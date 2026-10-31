// CC-6: the critical chain and buffers, buffer management and the drum pipeline, over stubbed API answers.
// Axe runs in English, Welsh and a locale with no page catalogue of its own. A zone or an outcome is checked
// to carry a word and a shape as well as a colour, every chart has a table that gives the same figures, and
// the pages say that nothing is changed.

import { test, expect } from "@playwright/test";
import { settle } from "./fixtures";

const PID = "44444444-4444-4444-8444-444444444444";
const T = (n: number, title: string) => ({
  pid: `00000000-0000-4000-8000-00000000000${n}`,
  title,
});

const CHAIN = {
  advisory: true,
  state: "chain",
  calendar: "default",
  critical_path: {
    tasks: [T(1, "Long path")],
    finish: "2026-10-09",
    chains: 1,
  },
  levelled_finish: "2026-10-12",
  finish_delay_days: 1,
  over_allocated_after: 0,
  chain_days: 6,
  crosses_paths: true,
  tied_chains: false,
  tasks: [
    {
      ...T(2, "Build"),
      duration_days: 4,
      start: "2026-10-05",
      finish: "2026-10-08",
      on_critical_path: false,
    },
    {
      ...T(3, "Test"),
      duration_days: 2,
      start: "2026-10-09",
      finish: "2026-10-12",
      on_critical_path: false,
    },
  ],
  links: [
    {
      from: T(2, "").pid,
      to: T(3, "").pid,
      kind: "resource",
      person: "person-1",
      person_ref: "person:ada",
    },
  ],
  note: "advice",
};

const BUFFERS = {
  advisory: true,
  state: "buffers",
  buffer_rule: "cut_and_paste",
  estimate_rule: "explicit_only",
  resource_buffer_lead_days: 2,
  chain_days_at_aggressive: 12,
  chain: [T(2, "Build"), T(3, "Test")],
  project_buffer: {
    days: 5,
    safety_removed_days: 10,
    chain_finish: "2026-10-20",
    finish_with_buffer: "2026-10-27",
  },
  feeding_buffers: [
    {
      joins: T(3, "Test"),
      tasks: [T(4, "Procure")],
      duration_days: 5,
      buffer_days: 5,
      room_before_join_days: 1,
      would_push_chain: true,
      push_days: 4,
    },
    {
      joins: T(3, "Test"),
      tasks: [T(5, "Paperwork")],
      duration_days: 3,
      buffer_days: 2,
      room_before_join_days: 4,
      would_push_chain: false,
      push_days: 0,
    },
  ],
  resource_buffers: [
    {
      task: T(2, "Build"),
      starts: "2026-10-05",
      alert_on: "2026-10-01",
      person: "person-1",
      person_ref: "person:ada",
    },
  ],
  scheduled_at_likely_duration: [T(6, "Loose end")],
  note: "advice",
};

const LINES = {
  green_amber: { at_start: 30, at_end: 70 },
  amber_red: { at_start: 60, at_end: 100 },
};
const point = (
  zone: string,
  x: number,
  y: number,
  slip: number,
  size: number,
) => ({
  chain_completed_pct: x,
  buffer_consumed_pct: y,
  plotted_x_pct: x,
  plotted_y_pct: Math.min(100, Math.max(0, y)),
  edge: y > 100 ? "above" : null,
  zone,
  slip_days: slip,
  buffer_days: size,
});
const MANAGEMENT = {
  state: "readings",
  buffer_plan: { pid: "bp", taken_on: "2026-10-01" },
  lines: LINES,
  readings: [
    {
      key: "project",
      kind: "project",
      buffer_days: 5,
      chain_done_days: 6,
      chain_total_days: 12,
      slip_days: 6,
      point: point("red", 50, 120, 6, 5),
      no_point: null,
      chain: ["Build", "Test"],
      joins: null,
    },
    {
      key: "feeding:a:b",
      kind: "feeding",
      buffer_days: 2,
      chain_done_days: 0,
      chain_total_days: 3,
      slip_days: 0,
      point: point("green", 0, 0, 0, 2),
      no_point: null,
      chain: ["Paperwork"],
      joins: "Test",
    },
    {
      key: "feeding:c:d",
      kind: "feeding",
      buffer_days: null,
      chain_done_days: 0,
      chain_total_days: 5,
      slip_days: null,
      point: null,
      no_point: "no_buffer",
      no_point_reason: "this chain could not be sized",
      chain: ["Procure"],
      joins: "Test",
    },
  ],
  alerts: [
    {
      kind: "project_buffer_red",
      buffer_key: "project",
      chain: ["Build", "Test"],
      joins: null,
      buffer_consumed_pct: 120,
    },
  ],
  red_without_open_recovery_action: ["project"],
  recovery_actions: [
    {
      pid: "ra1",
      buffer_key: "project",
      action: "Add a second tester",
      due_date: "2026-10-15",
      status: "open",
      closed_on: null,
      owner: "person-1",
      owner_ref: "person:ada",
    },
  ],
  snapshots: [
    {
      pid: "s1",
      taken_on: "2026-10-08",
      zones: { project: "amber", "feeding:a:b": "green" },
    },
  ],
};

const PIPELINE = {
  state: "pipeline",
  advisory: true,
  capacity_buffer_days: 7,
  from: "2026-10-01",
  months: 6,
  capacity_fte: 1,
  drum: {
    skill_pool_pid: "pool1",
    name: "Engineering",
    source: "most_overloaded",
    why: "largest shortfall",
  },
  staggered: 1,
  projects: [
    {
      plan_pid: "p1",
      name: "Apollo",
      outcome: "as_planned",
      first_drum_use: "2026-10-01",
      last_drum_use: "2026-10-31",
      peak_fte: 1,
      delay_days: 0,
      proposed_first_drum_use: "2026-10-01",
      explanation: "Apollo keeps its dates.",
    },
    {
      plan_pid: "p2",
      name: "Gemini",
      outcome: "staggered",
      first_drum_use: "2026-10-01",
      last_drum_use: "2026-10-31",
      peak_fte: 1,
      delay_days: 38,
      proposed_first_drum_use: "2026-11-08",
      explanation:
        "Gemini needs the drum while Apollo uses it. Proposed: start 38 days later. This is a proposal; no date has been changed.",
    },
  ],
  note: "proposal",
};

test.describe("critical chain, buffers and the drum (CC-6)", () => {
  test.beforeEach(async ({ page }) => {
    await page.route(`**/api/plans/${PID}/critical-chain`, (r) =>
      r.fulfill({ json: CHAIN }),
    );
    await page.route(`**/api/plans/${PID}/critical-chain-buffers`, (r) =>
      r.fulfill({ json: BUFFERS }),
    );
    await page.route(`**/api/plans/${PID}/buffer-management`, (r) =>
      r.fulfill({ json: MANAGEMENT }),
    );
    await page.route("**/api/portfolio-pipeline", (r) =>
      r.fulfill({ json: PIPELINE }),
    );
    await page.route("**/api/skill-pools", (r) =>
      r.fulfill({
        json: {
          skill_pools: [
            {
              pid: "pool1",
              name: "Engineering",
              active: true,
              roles: ["Developer"],
              skills: [],
            },
          ],
        },
      }),
    );
  });

  test("shows the chain beside the critical path, with a table for the bars", async ({
    page,
  }) => {
    await page.goto(`/en-001/plans/${PID}/critical-chain`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", {
        level: 1,
        name: "Critical chain and buffers",
      }),
    ).toBeVisible();
    await expect(page.getByText("Nothing has been changed")).toBeVisible();
    await expect(
      page.getByText("The chain crosses between paths: a person is shared."),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /Test.*a person: person:ada/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", {
        level: 2,
        name: "The critical path beside it",
      }),
    ).toBeVisible();
    await expect(
      page.getByText("The critical path finishes on 2026-10-09."),
    ).toBeVisible();
  });

  test("shows buffers as bars with a table, and a feeding buffer that would push the chain in words and a shape", async ({
    page,
  }) => {
    await page.goto(`/en-001/plans/${PID}/critical-chain`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByText(/Project buffer: 5 business days/),
    ).toBeVisible();
    const push = page.getByRole("row", {
      name: /Procure.*yes, by 4 business days/,
    });
    await expect(push).toContainText("▲");
    await expect(
      page.getByRole("row", { name: /Paperwork.*no$/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /Build.*person:ada.*1 Oct 2026/ }),
    ).toBeVisible();
    await expect(page.getByText("Loose end")).toBeVisible();
  });

  test("buffer management shows a fever chart per buffer, a table twin and the red chain by name", async ({
    page,
  }) => {
    await page.goto(`/en-001/plans/${PID}/buffers`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { level: 1, name: "Buffer management" }),
    ).toBeVisible();
    await expect(page.getByTestId("fever-chart")).toHaveCount(3);
    await expect(
      page.getByRole("alert").filter({
        hasText:
          "The project buffer is in the red: 120% used. Chain: Build → Test.",
      }),
    ).toBeVisible();
    await expect(
      page.getByText(
        "No open recovery action is recorded for: Project buffer.",
      ),
    ).toBeVisible();
    const red = page.getByRole("row", { name: /Project buffer.*red/ });
    await expect(red).toContainText("▲");
    await expect(
      page.getByRole("row", { name: /Feeding buffer: Paperwork.*green/ }),
    ).toContainText("●");
    await expect(
      page.getByRole("row", {
        name: /Feeding buffer: Procure.*this chain could not be sized/,
      }),
    ).toContainText("–");
    await expect(
      page.getByText(
        "The true figure is outside the chart, so the point is drawn on the edge.",
      ),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /8 Oct 2026.*amber/ }),
    ).toContainText("◆");
  });

  test("records a recovery action and closes one", async ({ page }) => {
    let posted: unknown = null;
    let closed = "";
    await page.route(`**/api/plans/${PID}/buffer-recovery-actions`, (r) => {
      posted = r.request().postDataJSON();
      return r.fulfill({ json: { pid: "ra2" } });
    });
    await page.route(
      `**/api/plans/${PID}/buffer-recovery-actions/ra1/status`,
      (r) => {
        closed = JSON.stringify(r.request().postDataJSON());
        return r.fulfill({ json: { pid: "ra1" } });
      },
    );
    await page.goto(`/en-001/plans/${PID}/buffers`, {
      waitUntil: "networkidle",
    });
    await page
      .getByLabel("What will be done")
      .fill("Borrow a tester from Gemini");
    await page.getByLabel("Owner (a person reference)").fill("person:grace");
    await page.getByRole("button", { name: "Record the action" }).click();
    await expect
      .poll(() => posted)
      .toMatchObject({
        buffer_key: "project",
        action: "Borrow a tester from Gemini",
        owner_ref: "person:grace",
      });
    await page
      .getByRole("button", { name: "Mark done: Add a second tester" })
      .click();
    await expect.poll(() => closed).toBe('{"status":"done"}');
  });

  test("the pipeline proposes staggered starts and says nothing is moved", async ({
    page,
  }) => {
    await page.goto("/en-001/pipeline", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Drum and pipeline" }),
    ).toBeVisible();
    await expect(
      page.getByText(
        "The drum is Engineering: no pool is named, and this one has the largest shortfall.",
      ),
    ).toBeVisible();
    const gemini = page.getByRole("row", {
      name: /Gemini.*start later.*8 Nov 2026.*38/,
    });
    await expect(gemini).toContainText("◆");
    await expect(gemini).toContainText("no date has been changed");
    await expect(
      page.getByRole("row", { name: /Apollo.*keeps its dates/ }),
    ).toContainText("●");
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto(`/cy-001/plans/${PID}/buffers`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { level: 1, name: "Rheoli byfferau" }),
    ).toBeVisible();
    await expect(page.getByText("Fever charts")).toHaveCount(0);
    await expect(
      page.getByRole("heading", { level: 2, name: "Siartiau twymyn" }),
    ).toBeVisible();
  });

  for (const route of [
    `plans/${PID}/critical-chain`,
    `plans/${PID}/buffers`,
    "pipeline",
  ]) {
    for (const locale of ["en-001", "cy-001", "zh-cn"]) {
      test(`${route} passes axe in ${locale}`, async ({ page }) => {
        await page.goto(`/${locale}/${route}`, { waitUntil: "networkidle" });
        await settle(page);
        const { default: AxeBuilder } = await import("@axe-core/playwright");
        const r = await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
          .analyze();
        expect(
          r.violations
            .filter((v) => v.impact === "serious" || v.impact === "critical")
            .map(
              (v) =>
                `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
            ),
        ).toEqual([]);
      });
    }
  }
});
