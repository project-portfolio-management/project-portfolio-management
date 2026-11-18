// The prioritization queue: a keyboard grid whose columns can be moved (Alt+Arrow on a header).
import { test, expect } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const QUEUE = {
  plans: [
    {
      pid: "p1",
      name: "Alpha",
      kind: "project",
      stage: null,
      score: 72.5,
      band: "high",
      coverage: 0.8,
      missing_evidence: [],
    },
    {
      pid: "p2",
      name: "Beta",
      kind: "project",
      stage: null,
      score: null,
      band: "unscored",
      coverage: 0,
      missing_evidence: ["roi"],
    },
  ],
  scored_of: 1,
  as_of: "2026-10-18",
};

test.describe("prioritization queue", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/prioritization*", (r) =>
      r.fulfill({ json: QUEUE }),
    );
  });

  test("lists the ranked plans and moves a column with Alt+Arrow", async ({
    page,
  }) => {
    await page.goto("/en-001/prioritization", { waitUntil: "networkidle" });
    await expect(page.getByRole("row", { name: /Alpha.*72\.5/ })).toBeVisible();
    const headers = () => page.getByRole("columnheader").allInnerTexts();
    const before = await headers();
    expect(before[0]).toMatch(/Plan/);
    await page.getByRole("columnheader").first().focus();
    await page.keyboard.press("Alt+ArrowRight");
    await expect.poll(async () => (await headers())[1]).toMatch(/Plan/);
    await expect.poll(async () => (await headers())[0]).toBe(before[1]);
  });

  test("still opens the explanation from its button", async ({ page }) => {
    await page.route("**/api/plans/p1/smart-score", (r) =>
      r.fulfill({
        json: {
          pid: "p1",
          name: "Alpha",
          as_of: "2026-10-18",
          smart_score: {
            score: 72.5,
            band: "high",
            coverage: 0.8,
            missing_evidence: [],
            components: [
              { name: "roi", weight: 0.5, raw: 0.9, contribution: 45 },
            ],
          },
        },
      }),
    );
    await page.goto("/en-001/prioritization", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Explain" }).first().click();
    await expect(
      page.getByRole("heading", { level: 2, name: "Alpha" }),
    ).toBeVisible();
  });
});

// NC-1 slice 12 part (g) (NC-6): the queue ranked by WSJF or RICE, and a plan's estimates. The band is offered only with
// the Smart Score; a WSJF row says what its score waits for in the catalogue's words; the estimates are read before
// they are edited (the inputs disabled until then), every problem said first, nothing sent for what is stored, all
// eight inputs sent with an emptied box as null; each model's components named and explained by the catalogue; a
// refusal in its words. Every stub that would carry the service's English carries "service text". Welsh; axe.
const STORED = {
  business_value: 8,
  time_criticality: 5,
  risk_reduction: 3,
  job_size: 4,
  reach: null,
  impact: 2,
  confidence_percent: 80,
  effort: 4,
};

async function stubModels(
  page: import("./test").Page,
  opts: { putStatus?: number } = {},
) {
  const sent: Array<{ method: string; path: string; body?: unknown }> = [];
  let inputs = { ...STORED };
  await page.route("**/api/prioritization*", (r) => {
    const url = new URL(r.request().url());
    const model = url.searchParams.get("model");
    sent.push({ method: "GET", path: `/api/prioritization${url.search}` });
    if (!model) return r.fulfill({ json: QUEUE });
    return r.fulfill({
      json: {
        model,
        plans: [
          {
            pid: "p1",
            name: "Alpha",
            kind: "project",
            stage: null,
            score: 4,
            missing_inputs: [],
          },
          {
            pid: "p2",
            name: "Beta",
            kind: "project",
            stage: null,
            score: null,
            missing_inputs: [
              { input: "job_size", reason: "service text" },
              { input: "business_value", reason: "service text" },
            ],
          },
        ],
        scored_of: 2,
        as_of: "2026-11-18",
      },
    });
  });
  await page.route(/\/api\/plans\/p1\/(scoring-inputs|score)/, async (r) => {
    const req = r.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "PUT") {
      const body = req.postDataJSON();
      sent.push({ method: "PUT", path, body });
      if (opts.putStatus)
        return r.fulfill({
          status: opts.putStatus,
          json: { code: "unprocessable", detail: "service text" },
        });
      inputs = body;
      return r.fulfill({
        json: {
          plan_pid: "p1",
          set: true,
          inputs,
          set_by: "service text",
          updated_at: "2026-11-18T10:00:00Z",
        },
      });
    }
    if (path.endsWith("/scoring-inputs"))
      return r.fulfill({
        json: {
          plan_pid: "p1",
          set: true,
          inputs,
          set_by: "service text",
          updated_at: "2026-11-17T09:00:00Z",
        },
      });
    const model = url.searchParams.get("model");
    const wsjf = {
      model: "wsjf",
      score: 4,
      formula: "service text",
      components: [
        { name: "business_value", value: 8, explanation: "service text" },
        { name: "time_criticality", value: 5, explanation: "service text" },
        { name: "risk_reduction", value: 3, explanation: "service text" },
        { name: "cost_of_delay", value: 16, explanation: "service text" },
        { name: "job_size", value: 4, explanation: "service text" },
      ],
      missing: [],
    };
    const rice = {
      model: "rice",
      score: inputs.reach === null ? null : 800,
      formula: "service text",
      components: [
        { name: "reach", value: inputs.reach, explanation: "service text" },
        { name: "impact", value: 2, explanation: "service text" },
        { name: "confidence", value: 0.8, explanation: "service text" },
        { name: "effort", value: 4, explanation: "service text" },
      ],
      missing:
        inputs.reach === null
          ? [{ input: "reach", reason: "service text" }]
          : [],
    };
    return r.fulfill({
      json: {
        pid: "p1",
        name: "Alpha",
        inputs_set: true,
        score: model === "wsjf" ? wsjf : rice,
      },
    });
  });
  return sent;
}

test("the queue ranks by WSJF: the band only with the Smart Score, and what a score waits for in the catalogue's words", async ({
  page,
}) => {
  const sent = await stubModels(page);
  await page.goto("/en-001/prioritization");
  await settle(page);
  await expect(
    page.getByText(
      "A plan without every input a model needs is unscored and ranked last, never scored zero.",
      { exact: false },
    ),
  ).toBeVisible();
  await expect(page.getByRole("combobox", { name: /^band/ })).toBeVisible();
  await page
    .getByLabel("Rank by")
    .selectOption({ label: "WSJF (weighted shortest job first)" });
  await expect(page.getByRole("combobox", { name: /^band/ })).toHaveCount(0);
  await page.getByRole("button", { name: "Apply" }).click();
  await expect(
    page.getByText(
      "2 of 2 plans, ranked by WSJF (weighted shortest job first)",
    ),
  ).toBeVisible();
  await expect(
    page.getByRole("row", { name: /Beta.*Job size, Business value/ }),
  ).toBeVisible();
  expect(sent.map((s) => s.path)).toEqual([
    "/api/prioritization",
    "/api/prioritization?model=wsjf",
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a plan's estimates: read first, every problem said first, nothing sent for what is stored, all eight sent", async ({
  page,
}) => {
  const sent = await stubModels(page);
  await page.goto("/en-001/prioritization");
  await settle(page);
  await page
    .getByRole("button", { name: "Open the estimates of Alpha" })
    .click();
  const s = page.getByRole("region", { name: "Estimates for Alpha" });
  await expect(s).toContainText(
    "Saving replaces every estimate: an empty box clears it.",
  );
  await expect(s.getByLabel("Business value")).toHaveValue("8");
  await expect(s.getByRole("combobox", { name: /^Impact/ })).toHaveValue("2");
  await expect(
    s.getByRole("region", { name: "WSJF (weighted shortest job first)" }),
  ).toContainText("Score: 4.00");
  await expect(s).toContainText(
    "Cost of delay 16 · the sum of the three above: what a delay costs",
  );
  const rice = s.getByRole("region", {
    name: "RICE (reach, impact, confidence, effort)",
  });
  await expect(rice).toContainText("No score: it waits for these inputs.");
  await expect(rice).toContainText(
    "Reach (people or events in a period): it says how many people the work touches in a period",
  );
  await expect(rice).toContainText("80%");
  await expect(rice).toContainText("2 (high impact)");
  await s.getByRole("button", { name: "Save the estimates" }).click();
  await expect(s.getByRole("status")).toHaveText(
    "The estimates are as stored, so nothing was sent.",
  );
  await s.getByLabel("Business value").fill("21");
  await s.getByLabel("Effort (person-months)").fill("0");
  await s.getByRole("button", { name: "Save the estimates" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "Business value must be a whole number from 1 to 20. Effort (person-months) must be above zero.",
  );
  expect(sent.filter((x) => x.method === "PUT")).toEqual([]);
  await s.getByLabel("Business value").fill("9");
  await s.getByLabel("Effort (person-months)").fill("");
  await s.getByLabel("Reach (people or events in a period)").fill("2000");
  await s.getByRole("button", { name: "Save the estimates" }).click();
  await expect(s.getByRole("status")).toHaveText("Estimates for Alpha saved.");
  expect(sent.filter((x) => x.method === "PUT")).toEqual([
    {
      method: "PUT",
      path: "/api/plans/p1/scoring-inputs",
      body: { ...STORED, business_value: 9, reach: 2000, effort: null },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("refused estimates are the catalogue's sentence", async ({ page }) => {
  await stubModels(page, { putStatus: 422 });
  await page.goto("/en-001/prioritization");
  await settle(page);
  await page
    .getByRole("button", { name: "Open the estimates of Alpha" })
    .click();
  const s = page.getByRole("region", { name: "Estimates for Alpha" });
  await s.getByLabel("Job size").fill("5");
  await s.getByRole("button", { name: "Save the estimates" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "The estimates were refused: an input is out of its range.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the estimates read in Welsh and pass axe with the form open", async ({
  page,
}) => {
  await stubModels(page);
  await page.goto("/cy-001/prioritization");
  await settle(page);
  await page.getByRole("button", { name: "Agor amcangyfrifon Alpha" }).click();
  const s = page.getByRole("region", { name: "Amcangyfrifon ar gyfer Alpha" });
  await expect(s.getByLabel("Gwerth busnes")).toHaveValue("8");
  // Focus moves into the form, as a person filling it would: the grid's cell is left, so its button is no tab stop.
  await s.getByLabel("Gwerth busnes").focus();
  await expect(s).toContainText("Dim sgôr: mae'n aros am y mewnbynnau hyn.");
  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
