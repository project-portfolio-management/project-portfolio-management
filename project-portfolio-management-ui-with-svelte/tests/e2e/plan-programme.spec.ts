// NC-1 slice 11 (DP-5): a plan's planned gate dates and programme status, in a real browser over a stubbed service.
// Gate dates send only what changed; a plan that is not a programme says it has no status; a programme is paused with
// a reason (citing a gate review), shows what it still holds, resumes, and once stopped offers nothing more; a refusal
// names the right it needs. The delivery model, its owner and the reporting method (EX-2, EX-14). Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

async function stubProgramme(
  page: Page,
  opts: { kind?: string; refuse?: number } = {},
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

test("the page reads in Welsh and passes axe", async ({ page }) => {
  await stubProgramme(page);
  await page.goto(`/cy-001/plans/${PID}/programme`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Pyrth a statws rhaglen",
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
