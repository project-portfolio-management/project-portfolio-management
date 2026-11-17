// NC-1 slice 11 (EX-10): scorecards and the de-escalation tracker, in a real browser over a stubbed service. The level
// is shown as the sponsor set it and recorded by the sponsor or an administrator (a 403 in the catalogue's words); a
// quarter's reasons are built from the detail, never from the service's sentences; criteria and evidence are added,
// their rules said first, nothing sent for no change; a scorecard is added, its KPIs chosen and measures added from
// the template with the direction rule said first. Welsh; axe on the open forms.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle, stubPlansApi } from "./fixtures";

/** The last `n` finished calendar quarters, oldest first, as the service labels them. */
function finished(n: number): string[] {
  const d = new Date();
  let year = d.getUTCFullYear();
  let q = Math.floor(d.getUTCMonth() / 3) + 1;
  const out: string[] = [];
  for (let i = 0; i < n; i++) {
    if (q === 1) {
      year -= 1;
      q = 4;
    } else q -= 1;
    out.unshift(`${year}-Q${q}`);
  }
  return out;
}
const [QA, QB] = finished(2) as [string, string];
const name = (label: string) => {
  const [y, q] = label.split("-Q");
  return `Q${q} ${y}`;
};

const kpiRows = () => [
  {
    kpi_pid: "k1",
    name: "Milestones on time",
    unit: "percent",
    direction: "higher_better",
    target_value: 80,
    quarters: [
      { quarter: QA, value: 85, observed_on: null, met: true },
      { quarter: QB, value: 70, observed_on: null, met: false },
    ],
  },
  {
    kpi_pid: "k2",
    name: "Assurance",
    unit: "count",
    direction: "higher_better",
    target_value: 3,
    quarters: [
      { quarter: QA, value: 3, observed_on: null, met: true },
      { quarter: QB, value: null, observed_on: null, met: null },
    ],
  },
];

async function stub(
  page: Page,
  opts: { levelStatus?: number; evidenceStatus?: number } = {},
) {
  const byQuarter = [
    { quarter: QA, state: { state: "met" } },
    { quarter: QB, state: { state: "missing", because: ["service text"] } },
  ];
  const tracker = {
    level: {
      level: "Level 3",
      set_on: "2026-10-01",
      note: "Board asked for a monthly review",
    },
    level_note: "service text",
    scorecard: { pid: "s1", name: "Outcomes" },
    required_quarters: 2,
    required_quarters_setting_unusable: false,
    consecutive_quarters_met: 0,
    rule_satisfied: false,
    by_quarter: byQuarter,
    criteria_count: 1,
    kpis_you_may_not_read: 1,
    kpis: kpiRows(),
    criteria: [
      {
        pid: "c1",
        criterion: "Independent review passed",
        quarters: [
          {
            quarter: QA,
            met: true,
            link: "https://evidence.example/a",
            note: "Audit signed",
          },
          { quarter: QB, met: null, link: null, note: null },
        ],
      },
    ],
    level_history: [
      { level: "Level 3", set_on: "2026-10-01" },
      { level: "Level 4", set_on: "2026-07-01" },
    ],
  };
  const scorecards = [
    { pid: "s1", name: "Outcomes", for_deescalation: true, active: true },
  ];
  const card = {
    scorecard: { pid: "s1", name: "Outcomes", for_deescalation: true },
    quarters: [QA, QB],
    kpis: kpiRows(),
    kpis_you_may_not_read: 0,
    by_quarter: byQuarter,
    note: "service text",
  };
  const kpi = (pid: string, name: string) => ({
    kpi: {
      pid,
      plan_pid: null,
      objective_pid: null,
      name,
      description: null,
      unit: null,
      direction: "higher_better",
      target_value: 1,
      red_value: 0,
      cadence: null,
      owner_ref: null,
    },
    status: "no_data",
    latest: null,
    trend: null,
    stale: null,
    readings: 0,
  });
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(
    /\/api\/(scorecards|scorecard-template|deescalation|kpis)/,
    async (route) => {
      const req = route.request();
      const url = new URL(req.url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      if (req.method() === "GET") {
        if (path === "/api/deescalation")
          return route.fulfill({ json: tracker });
        if (path === "/api/scorecards")
          return route.fulfill({ json: { scorecards } });
        if (path === "/api/scorecards/s1") return route.fulfill({ json: card });
        if (path === "/api/scorecard-template")
          return route.fulfill({
            json: {
              measures: [
                {
                  key: "milestones_met_on_time",
                  name: "service text",
                  direction: "higher_better",
                  unit: "percent",
                },
                {
                  key: "open_critical_issues",
                  name: "service text",
                  direction: "lower_better",
                  unit: "count",
                },
              ],
              note: "service text",
            },
          });
        if (path === "/api/kpis")
          return route.fulfill({
            json: {
              as_of: "2026-11-17",
              kpis: [
                kpi("k1", "Milestones on time"),
                kpi("k2", "Assurance"),
                kpi("k3", "Benefits validated"),
              ],
              summary: { green: 0, amber: 0, red: 0, no_data: 3 },
              note: "service text",
            },
          });
        return route.fulfill({ status: 404, json: { error: "unhandled" } });
      }
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      const refuse = (status: number) =>
        route.fulfill({
          status,
          json: { error: "refused", description: "service text" },
        });
      if (path === "/api/deescalation/level") {
        if (opts.levelStatus) return refuse(opts.levelStatus);
        tracker.level = { level: body.level, set_on: "2026-11-17", note: null };
        return route.fulfill({
          json: { level: body.level, set_on: "2026-11-17" },
        });
      }
      if (path.endsWith("/evidence") && opts.evidenceStatus)
        return refuse(opts.evidenceStatus);
      if (path === "/api/scorecards") {
        scorecards.push({
          pid: "s2",
          active: true,
          for_deescalation: false,
          ...body,
        });
        return route.fulfill({ json: { pid: "s2", ...body } });
      }
      return route.fulfill({ json: {} });
    },
  );
  return sent;
}

test("the tracker shows the level, the run and each quarter's reasons from the detail, and no service text", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/en-001/deescalation");
  await settle(page);
  const tracker = page.getByRole("region", {
    name: "The de-escalation tracker",
    exact: true,
  });
  await expect(tracker.getByTestId("level-now")).toHaveText(
    /^Level Level 3, set on .+\.$/,
  );
  await expect(
    tracker.getByText("The sponsor's note: Board asked for a monthly review"),
  ).toBeVisible();
  await expect(
    tracker.getByRole("list", { name: "Earlier levels" }).getByRole("listitem"),
  ).toHaveCount(2);
  await expect(tracker.getByTestId("run")).toHaveText(
    "0 finished quarters met in a row, of the 2 the rule asks for. The rule is not satisfied yet.",
  );
  await expect(
    tracker.getByText(
      "1 key measure on this scorecard belongs to a plan you may not read: it is not shown and not counted here.",
    ),
  ).toBeVisible();
  const quarters = tracker.getByRole("table", {
    name: "Each finished quarter",
  });
  const rowA = quarters.getByRole("row", { name: new RegExp(name(QA)) });
  await expect(rowA).toContainText("✓ Met");
  await expect(rowA).toContainText(
    "Every key measure and every criterion was met.",
  );
  const rowB = quarters.getByRole("row", { name: new RegExp(name(QB)) });
  await expect(rowB).toContainText("? Nothing recorded");
  await expect(rowB).toContainText(
    "Not met: Milestones on time. Nothing recorded for: Assurance; Independent review passed.",
  );
  const kpis = tracker.getByRole("table", {
    name: "The tracker's scorecard, by quarter",
  });
  await expect(
    kpis.getByRole("row", { name: /Milestones on time/ }),
  ).toContainText("✗ Not met (70)");
  await expect(
    kpis.getByRole("row", { name: /Milestones on time/ }),
  ).toContainText("at least 80 per cent");
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the level's rule is said first, nothing is sent for no change, and a 403 is the catalogue's", async ({
  page,
}) => {
  const sent = await stub(page, { levelStatus: 403 });
  await page.goto("/en-001/deescalation");
  await settle(page);
  const form = page.getByRole("region", {
    name: "Record the escalation level",
  });
  await expect(
    form.getByText(
      /^Only the sponsor \(someone with the role sponsor\) or an administrator/,
    ),
  ).toBeVisible();
  const save = form.getByRole("button", { name: "Record the level" });
  await expect(save).toBeEnabled();
  await save.click();
  await expect(form.getByRole("alert")).toHaveText(
    "A level is 1 to 60 characters.",
  );
  await form.getByLabel("Level", { exact: true }).fill(" Level 3 ");
  await form
    .getByLabel("Note (optional)")
    .fill("Board asked for a monthly review");
  await save.click();
  await expect(form.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  expect(sent).toEqual([]);
  await form.getByLabel("Level", { exact: true }).fill("Level 2");
  await form.getByLabel("Note (optional)").fill("");
  await save.click();
  await expect(form.getByRole("alert")).toHaveText(
    "Refused: only the sponsor or an administrator can record the escalation level.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/deescalation/level",
      body: { level: "Level 2" },
    },
  ]);
});

test("criteria and evidence: the rules said first, nothing sent for no change, a 422 the catalogue's", async ({
  page,
}) => {
  const sent = await stub(page, { evidenceStatus: 422 });
  await page.goto("/en-001/deescalation");
  await settle(page);
  const region = page.getByRole("region", { name: "Criteria for coming down" });
  await expect(
    region
      .getByRole("list", { name: "Evidence for: Independent review passed" })
      .getByRole("listitem")
      .first(),
  ).toContainText(`${name(QA)}: ✓ Met — Audit signed`);

  await region.getByLabel("Criterion (5 to 500 characters)").fill("Four");
  await region.getByRole("button", { name: "Add the criterion" }).click();
  await expect(
    region.getByText("A criterion is 5 to 500 characters."),
  ).toBeVisible();
  await region
    .getByLabel("Criterion (5 to 500 characters)")
    .fill("  No red packs for two quarters ");
  await region.getByRole("button", { name: "Add the criterion" }).click();
  await expect(region.getByText("Criterion added.")).toBeVisible();

  const criterion = region.getByRole("combobox", { name: /^Criterion/ });
  await expect(criterion).toBeEnabled();
  await expect(region.getByLabel("Finished quarter")).toHaveValue(QB);
  await region.getByLabel("Finished quarter").selectOption(QA);
  const record = region.getByRole("button", { name: "Record the evidence" });
  await record.click();
  await expect(
    region.getByText("Say whether the criterion was met."),
  ).toBeVisible();
  await region.getByRole("radio", { name: "Met", exact: true }).check();
  await region.getByLabel("Note (at least 5 characters)").fill("Audit signed");
  await region
    .getByLabel("Link to the evidence (optional)")
    .fill("evidence.example/a");
  await record.click();
  await expect(
    region.getByText(
      "A link is an http or https address with no spaces, up to 500 characters.",
    ),
  ).toBeVisible();
  await region
    .getByLabel("Link to the evidence (optional)")
    .fill("https://evidence.example/a");
  await record.click();
  await expect(
    region.getByText("Nothing changed, so nothing was sent."),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/deescalation/criteria",
      body: { criterion: "No red packs for two quarters" },
    },
  ]);

  await region.getByLabel("Finished quarter").selectOption(QB);
  await region.getByRole("radio", { name: "Not met" }).check();
  await region
    .getByLabel("Note (at least 5 characters)")
    .fill("Review moved to January");
  await region.getByLabel("Link to the evidence (optional)").fill("");
  await record.click();
  await expect(
    region.getByText(
      "Refused: the quarter must have finished, the note must be at least five characters, and a link must be an http or https address with no spaces.",
    ),
  ).toBeVisible();
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: "/api/deescalation/criteria/c1/evidence",
    body: { quarter: QB, met: false, note: "Review moved to January" },
  });
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a scorecard is added, its KPIs chosen, and measures added from the template with the direction rule said first", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/deescalation");
  await settle(page);
  const add = page.getByRole("region", { name: "Scorecards", exact: true });
  await expect(
    add.getByRole("table", { name: "Scorecards", exact: true }),
  ).toContainText("Yes, the one the tracker reads");
  await add.getByLabel("Name", { exact: true }).fill(" OUTCOMES ");
  await add.getByRole("button", { name: "Add the scorecard" }).click();
  await expect(add.getByRole("alert")).toHaveText(
    "A scorecard already has that name.",
  );
  await add.getByLabel("Name", { exact: true }).fill("Delivery health");
  await add.getByLabel("The one the tracker reads").check();
  await add.getByRole("button", { name: "Add the scorecard" }).click();
  await expect(add.getByRole("alert")).toHaveText(
    "The scorecard Outcomes is already the one the tracker reads.",
  );
  expect(sent).toEqual([]);
  await add.getByLabel("The one the tracker reads").uncheck();
  await add.getByRole("button", { name: "Add the scorecard" }).click();
  await expect(add.getByRole("status")).toHaveText(
    "Scorecard Delivery health added.",
  );

  const card = page.getByRole("region", { name: "A scorecard by quarter" });
  await card.getByRole("combobox", { name: /^Scorecard/ }).selectOption("s1");
  await expect(
    card
      .getByRole("table", { name: "Key measures of Outcomes, by quarter" })
      .getByRole("row", { name: /Assurance/ }),
  ).toContainText("? Nothing recorded");
  await expect(
    card
      .getByRole("list", { name: "Each quarter of the scorecard" })
      .getByRole("listitem")
      .last(),
  ).toHaveText(
    `? ${name(QB)}: Nothing recorded. Not met: Milestones on time. Nothing recorded for: Assurance.`,
  );
  const third = card.getByRole("checkbox", { name: "Benefits validated" });
  await expect(third).toBeEnabled();
  await expect(card.getByRole("checkbox", { name: "Assurance" })).toBeChecked();
  const saveKpis = card.getByRole("button", { name: "Save the key measures" });
  await saveKpis.click();
  await expect(
    card.getByText("Nothing changed, so nothing was sent."),
  ).toBeVisible();
  await third.check();
  await saveKpis.click();
  await expect(
    card.getByText("The scorecard now holds 3 key measures."),
  ).toBeVisible();

  await expect(
    card.getByText(
      "Where higher is better, the red line must be below the target; where lower is better, above it. Targets and red lines are whole numbers.",
    ),
  ).toBeVisible();
  const m = card.getByRole("group", {
    name: "Milestones met on or before their date",
  });
  await m.getByRole("checkbox").check();
  await m.getByLabel("Target", { exact: true }).fill("80");
  await m.getByLabel("Red line").fill("90");
  const addMeasures = card.getByRole("button", { name: "Add the measures" });
  await addMeasures.click();
  await expect(
    card.getByText(
      "Milestones met on or before their date: higher is better, so the red line must be below the target.",
    ),
  ).toBeVisible();
  await m.getByLabel("Red line").fill("60");
  await addMeasures.click();
  await expect(
    card.getByText("1 measure added to the scorecard."),
  ).toBeVisible();
  await expect(
    card.getByRole("group", { name: "Open issues at critical severity" }),
  ).toContainText("Lower is better · measured as a count");
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(sent.slice(1)).toEqual([
    {
      method: "PUT",
      path: "/api/scorecards/s1/kpis",
      body: { kpi_pids: ["k1", "k2", "k3"] },
    },
    {
      method: "POST",
      path: "/api/scorecards/s1/from-template",
      body: {
        measures: [
          { key: "milestones_met_on_time", target_value: 80, red_value: 60 },
        ],
      },
    },
  ]);
  expect(sent[0]).toEqual({
    method: "POST",
    path: "/api/scorecards",
    body: { name: "Delivery health" },
  });
});

test("the page reads in Welsh and passes axe with its forms open", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/cy-001/deescalation");
  await settle(page);
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Cardiau sgorio a dad-ddwysáu",
    }),
  ).toBeVisible();
  await expect(
    page.getByText(
      "Heb eu cyflawni: Milestones on time. Dim wedi'i gofnodi ar gyfer: Assurance; Independent review passed.",
    ),
  ).toBeVisible();
  await page
    .getByRole("combobox", { name: /^Cerdyn sgorio/ })
    .selectOption("s1");
  await expect(
    page.getByRole("checkbox", { name: "Benefits validated" }),
  ).toBeEnabled();
  await expect(
    page.getByRole("group", {
      name: "Materion agored o ddifrifoldeb critigol",
    }),
  ).toBeVisible();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
