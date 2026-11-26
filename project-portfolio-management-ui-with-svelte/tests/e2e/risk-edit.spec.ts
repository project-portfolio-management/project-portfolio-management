// NC-1 slice 12 part (c): changing a risk on the plan's governance page (`PUT /api/plans/{pid}/risks/{risk_pid}`), over
// stubbed answers. The rules are said first; a score out of range, a cleared review date and an over-long mitigation are
// refused before anything is sent; nothing is sent for no change; only the fields that changed are sent, a typed score
// as a number; "materialized" is left to Escalate; a 422 and a 404 say the catalogue's sentence, never the service's
// text; the table names each status in words. Welsh; axe with the form open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const R1 = "88888888-8888-4888-8888-888888888881";

async function stub(page: Page, opts: { refuse?: number } = {}) {
  const sent: Array<{ path: string; body: unknown }> = [];
  let risk = {
    pid: R1,
    title: "Supplier fails",
    status: "open",
    probability: 3,
    impact: 4,
    exposure: 12,
    owner_ref: null,
    mitigation: "Second supplier",
    review_date: "2026-12-01",
    escalated_at: null,
  };
  await stubPlansApi(page);
  const json = (route: import("@playwright/test").Route, body: unknown) =>
    route.fulfill({ json: body });
  await page.route(`**/api/plans/${PID}/governance`, (r) =>
    json(r, {
      pid: PID,
      name: "Apollo",
      stage: null,
      next_gate: null,
      gate_reviews: 0,
      latest_review: null,
      risks: { open: 1, materialized: 0, max_exposure: 12, total_exposure: 12 },
      budget: [],
    }),
  );
  await page.route(`**/api/plans/${PID}/gate-reviews`, (r) =>
    json(r, { stage: null, next_gate: null, reviews: [] }),
  );
  await page.route(`**/api/plans/${PID}/budget-lines`, (r) =>
    json(r, { lines: [], totals: [] }),
  );
  await page.route(`**/api/plans/${PID}/benefits`, (r) =>
    json(r, { benefits: [], totals: [] }),
  );
  await page.route(`**/api/plans/${PID}/objectives`, (r) => json(r, []));
  await page.route(`**/api/plans/${PID}/allocations`, (r) => json(r, []));
  await page.route(/\/api\/objectives(\?|$)/, (r) => json(r, []));
  await page.route(`**/api/plans/${PID}/milestones`, (r) => json(r, []));
  await page.route(/\/api\/milestones\?/, (r) =>
    json(r, {
      as_of: "2026-11-18",
      due_soon_days: 14,
      at_risk_days: 5,
      summary: { by_status: {}, by_schedule_state: {} },
      note: "service text",
      milestones: [],
    }),
  );
  await page.route(/\/api\/organizations/, (r) =>
    json(r, { organizations: [] }),
  );
  await page.route(`**/api/plans/${PID}/risks`, (r) => json(r, [risk]));
  await page.route(`**/api/plans/${PID}/risks/${R1}`, async (route) => {
    const body = route.request().postDataJSON() as Record<string, unknown>;
    sent.push({ path: new URL(route.request().url()).pathname, body });
    if (opts.refuse)
      return route.fulfill({
        status: opts.refuse,
        json: {
          code: opts.refuse === 422 ? "validation" : "not_found",
          detail: "service text",
        },
      });
    risk = { ...risk, ...body } as typeof risk;
    risk.exposure = risk.probability * risk.impact;
    return json(route, risk);
  });
  return sent;
}

const openForm = async (
  page: Page,
  locale = "en-001",
  name = "Change: Supplier fails",
) => {
  await page.goto(`/${locale}/plans/${PID}/governance`);
  await settle(page);
  await page.getByRole("button", { name }).click();
};

test("the rules are said first; nothing is sent for no change, or for a score out of range", async ({
  page,
}) => {
  const sent = await stub(page);
  await openForm(page);
  await expect(
    page
      .getByRole("table", { name: "Risk" })
      .getByText("Open", { exact: true }),
  ).toBeVisible();
  const form = page.getByRole("region", {
    name: "Change the risk “Supplier fails”",
  });
  await expect(form).toContainText(
    "Likelihood and impact are whole numbers from 1 to 5; the exposure is their product. The mitigation is at most 1024 bytes (fewer characters in some scripts). A review date can be changed but not removed. Only what you change is sent.",
  );
  await expect(form).toContainText(
    "To record that a risk has happened, use Escalate in the table",
  );
  // "Materialized" is Escalate's to record.
  await expect(
    form.getByLabel("Status of the risk").locator("option"),
  ).toHaveText(["Open", "Mitigating", "Closed"]);
  await expect(form.getByLabel("Likelihood (1 to 5)")).toHaveValue("3");
  const save = form.getByRole("button", { name: "Save the risk" });
  await save.click();
  await expect(form.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await form.getByLabel("Impact (1 to 5)").fill("6");
  await form.getByLabel("Review date").fill("");
  await save.click();
  await expect(form.getByRole("alert").getByRole("listitem")).toHaveText([
    "Likelihood and impact must each be a whole number from 1 to 5.",
    "A review date cannot be removed once set; choose another date.",
  ]);
  expect(sent).toHaveLength(0);
});

test("only the fields that changed are sent, a typed score as a number", async ({
  page,
}) => {
  const sent = await stub(page);
  await openForm(page);
  const form = page.getByRole("region", {
    name: "Change the risk “Supplier fails”",
  });
  await form.getByLabel("Status of the risk").selectOption("mitigating");
  await form.getByLabel("Impact (1 to 5)").fill("2");
  await form.getByLabel("Mitigation").fill("  Escrow the designs  ");
  await form.getByRole("button", { name: "Save the risk" }).click();
  await expect(form.getByRole("status")).toHaveText(
    "The risk “Supplier fails” was saved.",
  );
  expect(sent).toEqual([
    {
      path: `/api/proxy/api/plans/${PID}/risks/${R1}`,
      body: {
        status: "mitigating",
        impact: 2,
        mitigation: "Escrow the designs",
      },
    },
  ]);
  await expect(page.getByRole("table", { name: "Risk" })).toContainText(
    "3×2 = 6",
  );
  await expect(
    page
      .getByRole("table", { name: "Risk" })
      .getByText("Mitigating", { exact: true }),
  ).toBeVisible();
});

for (const [code, words] of [
  [
    422,
    "Refused: the service did not accept the status, scores or mitigation. Nothing was changed.",
  ],
  [404, "That risk is no longer on this plan; the list was read again."],
] as const) {
  test(`a ${code} says the catalogue's sentence, never the service's text`, async ({
    page,
  }) => {
    await stub(page, { refuse: code });
    await openForm(page);
    const form = page.getByRole("region", {
      name: "Change the risk “Supplier fails”",
    });
    await form.getByLabel("Likelihood (1 to 5)").fill("5");
    await form.getByRole("button", { name: "Save the risk" }).click();
    await expect(form.getByRole("alert")).toHaveText(words);
    await expect(page.getByText("service text")).toHaveCount(0);
  });
}

test("shows the score history, and sends the reason only with a changed score (OF-3)", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.route(`**/api/plans/${PID}/risks/${R1}/scores`, (r) =>
    r.fulfill({
      json: {
        risk_pid: R1,
        note: "service text",
        scores: [
          {
            probability: 2,
            impact: 4,
            score: 8,
            recorded_at: "2026-11-01T09:00:00Z",
            actor: null,
            reason: null,
            backfilled: true,
          },
          {
            probability: 3,
            impact: 4,
            score: 12,
            recorded_at: "2026-11-20T09:00:00Z",
            actor: null,
            reason: "Supplier slipped twice",
            backfilled: false,
          },
        ],
      },
    }),
  );
  await openForm(page);
  const form = page.getByRole("region", {
    name: "Change the risk “Supplier fails”",
  });
  await expect(
    form.getByRole("heading", { name: "Score history" }),
  ).toBeVisible();
  await expect(form.getByRole("listitem")).toHaveText([
    /1 Nov 2026: likelihood 2 × impact 4 = 8\. First known score/,
    /20 Nov 2026: likelihood 3 × impact 4 = 12\.\s+Reason: Supplier slipped twice/,
  ]);
  // A reason with no score change is not sent.
  await form.getByLabel("Why the score changed (optional)").fill("Just a note");
  await form.getByLabel("Review date").fill("2026-12-15");
  await form.getByRole("button", { name: "Save the risk" }).click();
  await expect.poll(() => sent.length).toBe(1);
  expect(sent[0].body).toEqual({ review_date: "2026-12-15" });
  await expect(form.getByRole("status")).toHaveText(
    "The risk “Supplier fails” was saved.",
  );
  // With a changed score it is.
  await form
    .getByLabel("Why the score changed (optional)")
    .fill("Vendor slipped");
  await form.getByLabel("Impact (1 to 5)").fill("5");
  await form.getByRole("button", { name: "Save the risk" }).click();
  await expect.poll(() => sent.length).toBe(2);
  expect(sent[1].body).toEqual({ impact: 5, reason: "Vendor slipped" });
});

test("reads in Welsh, and passes axe with the form open", async ({ page }) => {
  await stub(page);
  await openForm(page, "cy-001", "Newid: Supplier fails");
  await expect(
    page.getByRole("heading", { name: "Newid y risg “Supplier fails”" }),
  ).toBeVisible();
  await expect(page.getByRole("button", { name: "Cadw'r risg" })).toBeVisible();
  await page.goto(`/en-001/plans/${PID}/governance`);
  await settle(page);
  await page.getByRole("button", { name: "Change: Supplier fails" }).click();
  await expect(
    page.getByRole("heading", { name: "Change the risk “Supplier fails”" }),
  ).toBeVisible();
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});
