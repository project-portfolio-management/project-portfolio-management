// NC-1 slice 11 (DP-5): a plan's planned gate dates and programme status, in a real browser over a stubbed service.
// Gate dates send only what changed; a plan that is not a programme says it has no status; a programme is paused with
// a reason (citing a gate review), shows what it still holds, resumes, and once stopped offers nothing more; a refusal
// names the right it needs. Welsh; axe.

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
