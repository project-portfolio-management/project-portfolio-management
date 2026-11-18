// NC-1 slice 11 (DP-5): a plan's planned gate dates and programme status, in a real browser over a stubbed service.
// Gate dates send only what changed; a plan that is not a programme says it has no status; a programme is paused with
// a reason (citing a gate review), shows what it still holds, resumes, and once stopped offers nothing more; a refusal
// names the right it needs. The delivery model, its owner and the reporting method (EX-2, EX-14). Triage and
// thresholds (EX-7). The phase and a programme's start (EX-8): the start check said first, a refused start in the
// catalogue's words, an override, a move back with its reason, a plan with no phase. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

async function stubProgramme(
  page: Page,
  opts: {
    kind?: string;
    refuse?: number;
    phase?: string | null;
    refuseStart?: boolean;
    phaseRefuse?: number;
  } = {},
) {
  const gates = [{ gate: "g1_feasibility", planned_on: "2026-12-01" }];
  let status = "active";
  const history: Array<Record<string, unknown>> = [];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}`, async (route) => {
    const res = await route.fetch().catch(() => null);
    const base = res ? await res.json().catch(() => ({})) : {};
    return route.fulfill({
      json: {
        ...base,
        kind: opts.kind ?? "Program",
        name: "Website replatform",
        pid: PID,
      },
    });
  });
  await page.route(`**/api/plans/${PID}/gate-reviews`, (route) =>
    route.fulfill({
      json: {
        stage: "g1_feasibility",
        next_gate: "g2_definition",
        reviews: [
          {
            pid: "r1",
            gate: "g1_feasibility",
            decision: "hold",
            conditions: null,
            approver_ref: null,
            decided_at: "2026-11-10T09:00:00Z",
          },
        ],
      },
    }),
  );
  // Triage and thresholds (EX-7).
  const triages: Array<Record<string, unknown>> = [];
  const thresholds: Array<Record<string, unknown>> = [
    {
      kind: "confidence_red_packs",
      count: 3,
      date: null,
      cap_minor: null,
      currency: null,
    },
  ];
  await page.route(
    /\/api\/plans\/[^/]+\/(triage|thresholds)$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET")
        return route.fulfill({
          json: path.endsWith("triage")
            ? { triage: triages[0] ?? null, history: triages }
            : { thresholds },
        });
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { error: "x", description: "service text" },
        });
      if (path.endsWith("triage")) {
        const row = {
          pid: `t${triages.length + 1}`,
          triaged_on: "2026-11-17",
          ...body,
        };
        triages.unshift(row);
        return route.fulfill({ json: row });
      }
      const i = thresholds.findIndex((t) => t.kind === body.kind);
      if (body.remove) {
        if (i >= 0) thresholds.splice(i, 1);
        return route.fulfill({ json: { removed: body.kind } });
      }
      const row = {
        kind: body.kind,
        count: body.count ?? null,
        date: body.date ?? null,
        cap_minor: body.cap_minor ?? null,
        currency: body.currency ?? null,
      };
      if (i >= 0) thresholds[i] = row;
      else thresholds.push(row);
      return route.fulfill({ json: row });
    },
  );
  // The delivery model and reporting method (EX-2, EX-14).
  const profile: Record<string, unknown> = {
    delivery_model: null,
    delivery_owner: null,
    delivered_by_organization: null,
    reporting_method: "evm",
  };
  await page.route(/\/api\/organizations/, (route) =>
    route.fulfill({
      json: {
        organizations: [
          {
            pid: "o1",
            name: "Digital Service",
            kind: "delivery",
            active: true,
          },
          { pid: "o2", name: "Old Partner", kind: "partner", active: false },
        ],
      },
    }),
  );
  await page.route(
    /\/api\/plans\/[^/]+\/(delivery-model|reporting-method)$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") return route.fulfill({ json: profile });
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { error: "x", description: "service text" },
        });
      if (path.endsWith("reporting-method")) {
        profile.reporting_method = body.method;
        return route.fulfill({
          json: { plan_pid: PID, reporting_method: body.method },
        });
      }
      profile.delivery_model = body.delivery_model;
      profile.delivery_owner = body.delivery_owner_organization_pid
        ? {
            organization_pid: body.delivery_owner_organization_pid,
            name: "Digital Service",
          }
        : null;
      return route.fulfill({ json: profile });
    },
  );
  // The phase and the start check (FR-30, EX-8).
  const PHASES = [
    "initiating",
    "planning",
    "executing",
    "controlling",
    "closing",
  ];
  let phase: string | null = opts.phase === undefined ? "planning" : opts.phase;
  const check: Record<string, unknown> = {
    plan_pid: PID,
    wip: { limit: 3, limit_setting_unusable: false, in_delivery: 3 },
    constraint_pool: "Engineering",
    demand_by_pool: [
      {
        pool: "Engineering",
        months: [
          { month: "2026-12", demand_fte: 1.5 },
          { month: "2027-01", demand_fte: 2 },
        ],
      },
      { pool: "Design", months: [{ month: "2027-01", demand_fte: 0.25 }] },
    ],
    would_be_refused: opts.refuseStart ?? false,
    refusals: opts.refuseStart
      ? [
          { reason: "limit_reached", in_delivery: 3, limit: 3 },
          {
            reason: "constraint_pool_over",
            pool: "Engineering",
            months: ["2026-12", "2027-01"],
          },
        ]
      : [],
    queue_position: null,
    note: "service text",
  };
  await page.route(
    /\/api\/plans\/[^/]+\/(phase|phase-history|start-check)$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") {
        if (path.endsWith("start-check")) return route.fulfill({ json: check });
        return route.fulfill({
          json: {
            plan_pid: PID,
            current_phase: phase,
            next_phase: PHASES[PHASES.indexOf(phase ?? "") + 1] ?? null,
            transitions: phase
              ? [
                  {
                    pid: "pt1",
                    from_phase: null,
                    to_phase: "initiating",
                    reason: null,
                    occurred_at: "2026-10-01T09:00:00Z",
                  },
                ]
              : [],
            durations: PHASES.map((p, i) => ({
              phase: p,
              ms: i === 0 ? 86_400_000 * 3 : i === 1 ? 86_400_000 : 0,
              visits: i <= PHASES.indexOf(phase ?? "") ? 1 : 0,
              current: p === phase,
            })),
            as_of: "2026-11-17T09:00:00Z",
          },
        });
      }
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (opts.phaseRefuse)
        return route.fulfill({
          status: opts.phaseRefuse,
          json: { code: "unprocessable", detail: "service text" },
        });
      if (
        body.phase === "executing" &&
        opts.refuseStart &&
        !body.start_override_reason
      ) {
        check.queue_position = 2;
        return route.fulfill({
          status: 422,
          json: { code: "start_refused", detail: "service text" },
        });
      }
      phase = body.phase;
      check.queue_position = null;
      return route.fulfill({
        json: {
          phase,
          next_phase: PHASES[PHASES.indexOf(phase ?? "") + 1] ?? null,
        },
      });
    },
  );
  await page.route(/\/api\/exceptions/, (route) =>
    route.fulfill({ json: { exceptions: [], window_hours: 48 } }),
  );
  await page.route(
    /\/api\/plans\/[^/]+\/(planned-gates|programme-status)/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") {
        if (path.endsWith("planned-gates"))
          return route.fulfill({
            json: { plan_pid: PID, planned_gates: gates },
          });
        return route.fulfill({
          json: {
            plan_pid: PID,
            status,
            history,
            still_held:
              status === "active"
                ? null
                : {
                    allocations_flagged: 2,
                    money_by_currency: [
                      {
                        currency: "GBP",
                        planned_minor: 120000,
                        actual_minor: 45000,
                      },
                    ],
                    benefits: 1,
                  },
          },
        });
      }
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { error: "x", description: "service text" },
        });
      if (path.includes("planned-gates/")) {
        const gate = path.split("/").pop()!;
        const at = gates.find((g) => g.gate === gate);
        if (at) at.planned_on = body.planned_on;
        else gates.push({ gate, planned_on: body.planned_on });
        return route.fulfill({ json: { gate, planned_on: body.planned_on } });
      }
      status = body.status;
      history.push({
        status,
        reason: body.reason,
        decided_by: "x",
        decided_on: "2026-11-17",
        gate_review_pid: body.gate_review_pid ?? null,
        exception_pid: null,
      });
      return route.fulfill({ json: { plan_pid: PID, status, decision: "d1" } });
    },
  );
  return sent;
}

test("the plan page leads here; gate dates send only what changed", async ({
  page,
}) => {
  const sent = await stubProgramme(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Gates and programme status" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Gates and programme status" }),
  ).toBeVisible();
  const gates = page.getByRole("region", { name: "Planned gate dates" });
  await expect(gates.getByLabel("G1 feasibility")).toHaveValue("2026-12-01");
  await gates.getByRole("button", { name: "Save the gate dates" }).click();
  await expect(gates.getByRole("status")).toHaveText(
    "No gate date changed, so nothing was sent.",
  );
  await gates.getByLabel("G2 definition").fill("2027-02-01");
  await gates.getByRole("button", { name: "Save the gate dates" }).click();
  await expect(gates.getByRole("status")).toHaveText("Gate dates saved.");
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/planned-gates/g2_definition`,
      body: { planned_on: "2027-02-01" },
    },
  ]);
});

test("a plan that is not a programme says it has no status, and still plans its gates", async ({
  page,
}) => {
  await stubProgramme(page, { kind: "Project" });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  await expect(
    page.getByText(/Only a plan of kind Program has a programme status/),
  ).toBeVisible();
  await expect(
    page.getByRole("region", { name: "Programme status" }),
  ).toHaveCount(0);
  await expect(
    page.getByRole("region", { name: "Planned gate dates" }),
  ).toBeVisible();
});

test("a programme is paused with a reason citing a review, shows what it holds, resumes, and stopped is final", async ({
  page,
}) => {
  const sent = await stubProgramme(page);
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const status = page.getByRole("region", { name: "Programme status" });
  await expect(status.getByText("Status: active")).toBeVisible();
  await expect(
    status.getByRole("radio", { name: "Resume the programme" }),
  ).toHaveCount(0);

  await status.getByRole("button", { name: "Record the decision" }).click();
  await expect(status.getByRole("alert")).toHaveText(
    "Choose the change. Give the reason in at least 10 characters.",
  );
  expect(sent).toEqual([]);

  await status.getByRole("radio", { name: "Pause the programme" }).check();
  await status
    .getByLabel("Reason (at least 10 characters)")
    .fill("  Waiting for the funding decision.  ");
  await status
    .getByLabel("It follows from a gate review (optional)")
    .selectOption("r1");
  await status.getByRole("button", { name: "Record the decision" }).click();
  await expect(status.getByRole("status")).toHaveText(
    "Recorded: the programme is now paused.",
  );
  await expect(
    status.getByText("2 allocations are flagged, not deleted."),
  ).toBeVisible();
  await expect(status.getByText("1 benefit is kept on record.")).toBeVisible();
  await expect(
    status.getByText("It followed from a gate review of this programme."),
  ).toBeVisible();

  await status.getByRole("radio", { name: "Stop the programme" }).check();
  await expect(status.getByRole("note")).toHaveText(
    "Stopping is final: the programme stays stopped and keeps its record.",
  );
  await status
    .getByLabel("Reason (at least 10 characters)")
    .fill("The case for it has gone.");
  await status.getByRole("button", { name: "Record the decision" }).click();
  await expect(
    status.getByText(
      "This programme is stopped. That is final; its record is kept.",
    ),
  ).toBeVisible();
  await expect(
    status.getByRole("button", { name: "Record the decision" }),
  ).toHaveCount(0);
  expect(sent.map((s) => s.body)).toEqual([
    {
      status: "paused",
      reason: "Waiting for the funding decision.",
      gate_review_pid: "r1",
    },
    { status: "stopped", reason: "The case for it has gone." },
  ]);
});

test("a decision without the decider right is refused in the catalogue's words", async ({
  page,
}) => {
  await stubProgramme(page, { refuse: 403 });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const status = page.getByRole("region", { name: "Programme status" });
  await status.getByRole("radio", { name: "Pause the programme" }).check();
  await status
    .getByLabel("Reason (at least 10 characters)")
    .fill("Waiting for the funding decision.");
  await status.getByRole("button", { name: "Record the decision" }).click();
  await expect(status.getByRole("alert")).toHaveText(
    "Pausing, resuming or stopping a programme needs the programme decider right.",
  );
});

test("a programme's buffer is declared by days or by the rule, and its reading drawn as the pack draws it", async ({
  page,
}) => {
  await stubProgramme(page);
  const history: Array<Record<string, unknown>> = [];
  const sent: unknown[] = [];
  let refuseRule = true;
  await page.route(`**/api/plans/${PID}/project-buffer`, async (route) => {
    const req = route.request();
    if (req.method() === "GET")
      return route.fulfill({
        json: {
          plan_pid: PID,
          history,
          now: {
            plan_pid: PID,
            plan: "Website replatform",
            point: history.length
              ? {
                  chain_completed_pct: 40,
                  buffer_consumed_pct: 25,
                  zone: "green",
                  edge: null,
                  slip_days: 2,
                  buffer_days: 8,
                }
              : null,
            no_fever_chart_point: history.length
              ? null
              : { code: "no_buffer", reason: "service text" },
          },
          lines: {},
        },
      });
    const body = req.postDataJSON();
    sent.push(body);
    if (body.rule && refuseRule) {
      refuseRule = false;
      return route.fulfill({
        status: 422,
        json: { error: "x", description: "service text" },
      });
    }
    history.push({
      buffer_days: body.buffer_days ?? 8,
      source: body.rule ? "rule" : "declared",
      rule: body.rule ?? null,
      note: body.note ?? null,
      declared_by: "x",
      declared_on: "2026-11-17",
      baseline_version: 1,
    });
    return route.fulfill({
      json: {
        plan_pid: PID,
        buffer_days: body.buffer_days ?? 8,
        source: body.rule ? "rule" : "declared",
      },
    });
  });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const buffer = page.getByRole("region", { name: "Project buffer" });
  await buffer.getByLabel("Business days (1 to 2000)").fill("2001");
  await buffer.getByRole("button", { name: "Declare the buffer" }).click();
  await expect(buffer.getByRole("alert")).toHaveText(
    "Write the buffer as a whole number of business days from 1 to 2000.",
  );
  await buffer
    .getByRole("radio", {
      name: "Half the baselined critical work, rounded up",
    })
    .check();
  await buffer.getByRole("button", { name: "Declare the buffer" }).click();
  await expect(buffer.getByRole("alert")).toHaveText(
    /The rule needs a schedule baseline/,
  );
  await buffer.getByRole("button", { name: "Declare the buffer" }).click();
  await expect(buffer.getByRole("status")).toHaveText(
    "Declared: a buffer of 8 business days.",
  );
  await expect(buffer.getByRole("table").first()).toBeVisible();
  await expect(buffer.getByText("8 business days, by the rule")).toBeVisible();
  expect(sent).toEqual([
    { rule: "half_of_critical_duration" },
    { rule: "half_of_critical_duration" },
  ]);
});

test("a programme's delivery model and owner are saved together, and its reporting method changed", async ({
  page,
}) => {
  const sent = await stubProgramme(page);
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const sec = page.getByRole("region", {
    name: "Delivery model and reporting",
  });
  await sec.getByRole("button", { name: "Save the delivery model" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  // Only an active organization is offered.
  await expect(
    sec.getByLabel("Owning organization").locator("option"),
  ).toHaveText(["No owning organization", "Digital Service"]);
  await sec.getByLabel("Delivery model").selectOption("central_product");
  await sec.getByLabel("Owning organization").selectOption("o1");
  await sec.getByRole("button", { name: "Save the delivery model" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "The delivery model is saved.",
  );
  await sec.getByLabel("Burn-up and flow (a product programme)").check();
  await sec.getByRole("button", { name: "Save the reporting method" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "The programme now reports by burn-up and flow.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/delivery-model`,
      body: {
        delivery_model: "central_product",
        delivery_owner_organization_pid: "o1",
      },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/reporting-method`,
      body: { method: "product" },
    },
  ]);
});

test("a refused delivery model is said in the catalogue's words", async ({
  page,
}) => {
  await stubProgramme(page, { refuse: 422 });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const sec = page.getByRole("region", {
    name: "Delivery model and reporting",
  });
  await sec.getByLabel("Delivery model").selectOption("central_product");
  await sec.getByRole("button", { name: "Save the delivery model" }).click();
  await expect(sec.getByRole("alert")).toBeVisible();
  await expect(sec.getByRole("alert")).not.toContainText("service text");
});

test("a triage is recorded with its reason said first, and thresholds are set, left unchanged and removed", async ({
  page,
}) => {
  const sent = await stubProgramme(page);
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const tri = page.getByRole("region", { name: "Triage", exact: true });
  await expect(tri.getByText("No triage recorded yet.")).toBeVisible();
  await tri.getByLabel("Outcome").selectOption("pause");
  await tri.getByLabel("Reason (at least 10 characters)").fill("too short");
  await tri.getByRole("button", { name: "Record the triage" }).click();
  await expect(tri.getByRole("alert")).toHaveText(
    "The reason must be at least 10 characters.",
  );
  await tri
    .getByLabel("Reason (at least 10 characters)")
    .fill("Supplier cannot staff the second wave");
  await tri.getByRole("button", { name: "Record the triage" }).click();
  await expect(tri.getByRole("status")).toHaveText("Triage recorded.");
  await expect(
    tri.getByText(
      "Latest triage, 17 Nov 2026: Pause. Supplier cannot staff the second wave",
    ),
  ).toBeVisible();

  const th = page.getByRole("region", { name: "Thresholds" });
  const red = th.getByRole("form", {
    name: "Confidence red for several packs",
  });
  await expect(
    red.getByText("Crossed when confidence is red in 3 issued packs in a row."),
  ).toBeVisible();
  await red.getByRole("button", { name: "Save the threshold" }).click();
  await expect(th.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await red.getByLabel("Issued packs at red in a row (1 to 24)").fill("30");
  await red.getByRole("button", { name: "Save the threshold" }).click();
  await expect(th.getByRole("alert")).toHaveText(
    "The number of packs must be a whole number from 1 to 24.",
  );
  const cap = th.getByRole("form", { name: "P80 cost above a cap" });
  await cap.getByLabel("Cap amount").fill("250000");
  await cap.getByLabel("Currency (three letters)").fill("gbx");
  await cap.getByRole("button", { name: "Save the threshold" }).click();
  await expect(th.getByRole("alert")).toHaveText(
    "The currency must be a three-letter ISO 4217 code.",
  );
  await cap.getByLabel("Currency (three letters)").fill("gbp");
  await cap.getByRole("button", { name: "Save the threshold" }).click();
  await expect(th.getByRole("status")).toHaveText(
    "Threshold saved: P80 cost above a cap.",
  );
  await red.getByRole("button", { name: "Remove the threshold" }).click();
  await expect(th.getByRole("status")).toHaveText(
    "Threshold removed: Confidence red for several packs.",
  );
  expect(sent.filter((x) => /triage|thresholds/.test(x.path))).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/triage`,
      body: {
        outcome: "pause",
        reason: "Supplier cannot staff the second wave",
      },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/thresholds`,
      body: { kind: "p80_cap", cap_minor: 25000000, currency: "GBP" },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/thresholds`,
      body: { kind: "confidence_red_packs", remove: true },
    },
  ]);
});

test("a programme's start check is said first, a refused start joins the queue in the catalogue's words, and an override starts it", async ({
  page,
}) => {
  const sent = await stubProgramme(page, { refuseStart: true });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const region = page.getByRole("region", { name: "Phase and start" });
  await expect(region.getByTestId("phase-now")).toHaveText(
    "The plan is in the Planning phase.",
  );
  await expect(
    region
      .getByRole("list", { name: "Time in each phase" })
      .getByRole("listitem"),
  ).toHaveText([
    "Initiating: 3 days · entered 1 time",
    "Planning: 1 day · entered 1 time · now",
  ]);
  const checkList = region.getByTestId("start-check");
  await expect(checkList).toContainText(
    "3 of 3 places for programmes in active delivery are taken.",
  );
  await expect(checkList).toContainText("The constraint pool is Engineering.");
  await expect(checkList).toContainText("A start now would be refused:");
  await expect(checkList).toContainText(
    "All 3 places for programmes in active delivery are taken (3 in delivery).",
  );
  await expect(checkList).toContainText(
    "The constraint pool Engineering is short of supply in December 2026, January 2027.",
  );
  const demand = region.getByRole("table", {
    name: "This programme's firm demand",
  });
  await expect(demand.getByRole("row", { name: /Engineering/ })).toContainText(
    "1.50",
  );
  await expect(demand.getByRole("row", { name: /Design/ })).toContainText("—");

  // Nothing is sent before a phase is chosen.
  const move = region.getByRole("button", { name: "Move the phase" });
  await move.click();
  await expect(region.getByRole("alert")).toHaveText(
    "Choose the phase to move to.",
  );
  expect(sent).toEqual([]);

  await region.getByRole("radio", { name: "Forward to Executing" }).check();
  await expect(
    region.getByText(
      "This start would be refused. Sent without an override reason, the programme joins the start queue (or keeps its place there).",
    ),
  ).toBeVisible();
  const override = region.getByLabel(/^Override reason, to start anyway/);
  await override.fill(" Too short ");
  await move.click();
  await expect(region.getByRole("alert")).toHaveText(
    "An override reason is 10 or more characters.",
  );
  expect(sent).toEqual([]);

  await override.fill("");
  await move.click();
  await expect(region.getByRole("alert")).toHaveText(
    "The start was refused, so the programme is number 2 in the start queue. To start it anyway, give an override reason.",
  );
  await expect(
    region.getByRole("list", { name: "Why the start was refused" }),
  ).toContainText("All 3 places for programmes in active delivery are taken");
  await expect(checkList).toContainText(
    "This programme is number 2 in the start queue.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);

  await override.fill("  The board agreed to run it in parallel.  ");
  await move.click();
  await expect(region.getByRole("status")).toHaveText(
    "The programme started with an override: it is now in the Executing phase, and the override is recorded.",
  );
  await expect(region.getByTestId("phase-now")).toHaveText(
    "The plan is in the Executing phase.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/phase`,
      body: { phase: "executing" },
    },
    {
      method: "PUT",
      path: `/api/plans/${PID}/phase`,
      body: {
        phase: "executing",
        start_override_reason: "The board agreed to run it in parallel.",
      },
    },
  ]);
});

test("a programme with room starts with nothing more, and its check says so", async ({
  page,
}) => {
  const sent = await stubProgramme(page);
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const region = page.getByRole("region", { name: "Phase and start" });
  await expect(region.getByTestId("start-check")).toContainText(
    "A start now would go ahead.",
  );
  await region.getByRole("radio", { name: "Forward to Executing" }).check();
  await expect(
    region.getByText(/^Moving into Executing is this programme's start/),
  ).toBeVisible();
  await expect(region.getByLabel(/^Override reason/)).toHaveCount(0);
  await region.getByRole("button", { name: "Move the phase" }).click();
  await expect(region.getByRole("status")).toHaveText(
    "The plan is now in the Executing phase.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/phase`,
      body: { phase: "executing" },
    },
  ]);
});

test("a plan's phase moves back only with a reason, said first, has no start check, and a refusal is the catalogue's", async ({
  page,
}) => {
  const sent = await stubProgramme(page, {
    kind: "Project",
    phase: "executing",
    phaseRefuse: 422,
  });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const region = page.getByRole("region", { name: "Phase", exact: true });
  await expect(region.getByTestId("phase-now")).toHaveText(
    "The plan is in the Executing phase.",
  );
  await expect(region.getByTestId("start-check")).toHaveCount(0);
  await expect(region.getByRole("radio")).toHaveCount(3);
  await region.getByRole("radio", { name: "Back to Planning" }).check();
  await expect(region.getByRole("note")).toHaveText(
    "A move back needs a reason.",
  );
  const move = region.getByRole("button", { name: "Move the phase" });
  await move.click();
  await expect(region.getByRole("alert")).toHaveText(
    "A move back needs a reason.",
  );
  expect(sent).toEqual([]);
  await region
    .getByLabel(/^Reason \(needed to move back/)
    .fill("  The scope changed after the review.  ");
  await move.click();
  await expect(region.getByRole("alert")).toHaveText(
    "Refused: a move forward is one phase at a time, a move back needs a reason, and the plan must not already be in that phase.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/phase`,
      body: {
        phase: "planning",
        reason: "The scope changed after the review.",
      },
    },
  ]);
});

test("a plan with no phase yet offers only Initiating", async ({ page }) => {
  const sent = await stubProgramme(page, { kind: "Project", phase: null });
  await page.goto(`/en-001/plans/${PID}/programme`);
  await settle(page);
  const region = page.getByRole("region", { name: "Phase", exact: true });
  await expect(region.getByTestId("phase-now")).toHaveText(
    "No phase is recorded yet: the first move is into Initiating.",
  );
  await expect(region.getByRole("radio")).toHaveCount(1);
  await region.getByRole("radio", { name: "Forward to Initiating" }).check();
  await region.getByRole("button", { name: "Move the phase" }).click();
  await expect(region.getByRole("status")).toHaveText(
    "The plan is now in the Initiating phase.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/phase`,
      body: { phase: "initiating" },
    },
  ]);
});

test("the page reads in Welsh and passes axe", async ({ page }) => {
  await stubProgramme(page, { refuseStart: true });
  await page.goto(`/cy-001/plans/${PID}/programme`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Pyrth a statws rhaglen",
  );
  // The phase form open on a refused start, so its override field is audited too.
  const phase = page.getByRole("region", { name: "Cyfnod a chychwyn" });
  await phase.getByRole("radio", { name: "Ymlaen i Cyflawni" }).check();
  await expect(phase.getByLabel(/^Rheswm diystyru/)).toBeVisible();
  await expect(phase.getByTestId("phase-now")).toHaveText(
    "Mae'r cynllun yn y cyfnod Cynllunio.",
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
