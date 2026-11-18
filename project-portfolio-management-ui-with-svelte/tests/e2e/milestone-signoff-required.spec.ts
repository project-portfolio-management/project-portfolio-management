// NC-1 slice 11 (EX-4): choosing which organizations must sign a milestone off, on the plan's governance page, over
// stubbed answers. The set shown is the milestone's whole `signoff.required` (signed or not), because saving replaces
// it: an organization that has signed stays in the set; an inactive one still required must be unticked on purpose;
// nothing is sent for no change; clearing the set is said. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const M1 = "77777777-7777-4777-8777-777777777771";
const M2 = "77777777-7777-4777-8777-777777777772";
const ACME = "o-acme";
const OTHER = "o-other";
const OLD = "o-old";

async function stub(page: Page, opts: { oldRequired?: boolean } = {}) {
  const sent: Array<{ path: string; body: unknown }> = [];
  const required: Record<
    string,
    Array<{ organization_pid: string; name: string; signed_on: string | null }>
  > = {
    [M1]: [
      { organization_pid: ACME, name: "Acme Council", signed_on: "2026-11-10" },
      ...(opts.oldRequired
        ? [{ organization_pid: OLD, name: "Old Partner", signed_on: null }]
        : []),
    ],
    [M2]: [],
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
      risks: { open: 0, materialized: 0, max_exposure: 0, total_exposure: 0 },
      budget: [],
    }),
  );
  await page.route(`**/api/plans/${PID}/gate-reviews`, (r) =>
    json(r, { stage: null, next_gate: null, reviews: [] }),
  );
  await page.route(`**/api/plans/${PID}/risks`, (r) => json(r, []));
  await page.route(`**/api/plans/${PID}/budget-lines`, (r) =>
    json(r, { lines: [], totals: [] }),
  );
  await page.route(`**/api/plans/${PID}/benefits`, (r) =>
    json(r, { benefits: [], totals: [] }),
  );
  await page.route(`**/api/plans/${PID}/objectives`, (r) => json(r, []));
  await page.route(`**/api/plans/${PID}/allocations`, (r) => json(r, []));
  await page.route(/\/api\/objectives(\?|$)/, (r) => json(r, []));
  await page.route(`**/api/plans/${PID}/milestones`, (r) =>
    json(r, [
      {
        pid: M1,
        name: "Go live",
        due: "2026-12-01",
        kind: "milestone",
        done: false,
        overdue: false,
      },
      {
        pid: M2,
        name: "Handover",
        due: "2027-01-15",
        kind: "milestone",
        done: false,
        overdue: false,
      },
    ]),
  );
  await page.route(/\/api\/milestones\?/, (r) =>
    json(r, {
      as_of: "2026-11-18",
      due_soon_days: 14,
      at_risk_days: 5,
      summary: { by_status: {}, by_schedule_state: {} },
      note: "service text",
      milestones: [M1, M2].map((pid) => ({
        pid,
        plan_pid: PID,
        name: pid === M1 ? "Go live" : "Handover",
        ...(required[pid]!.length > 0
          ? {
              signoff: {
                state: "not_delivered",
                delivered_on: null,
                met_on: null,
                outstanding: required[pid]!.filter((x) => !x.signed_on).map(
                  (x) => x.name,
                ),
                required: required[pid],
              },
            }
          : {}),
      })),
    }),
  );
  await page.route(/\/api\/organizations/, (r) =>
    json(r, {
      organizations: [
        { pid: ACME, name: "Acme Council", kind: "partner", active: true },
        {
          pid: OTHER,
          name: "Other Health Trust",
          kind: "partner",
          active: true,
        },
        { pid: OLD, name: "Old Partner", kind: "partner", active: false },
      ],
    }),
  );
  await page.route(/\/milestones\/[^/]+\/signoff-required$/, async (route) => {
    const path = new URL(route.request().url()).pathname.replace(
      /^\/api\/proxy/,
      "",
    );
    const body = route.request().postDataJSON();
    sent.push({ path, body });
    const m = path.includes(M1) ? M1 : M2;
    const names: Record<string, string> = {
      [ACME]: "Acme Council",
      [OTHER]: "Other Health Trust",
    };
    required[m] = body.organization_pids.map((p: string) => ({
      organization_pid: p,
      name: names[p],
      signed_on: p === ACME ? "2026-11-10" : null,
    }));
    return json(route, {
      milestone_pid: m,
      organization_pids: body.organization_pids,
    });
  });
  return sent;
}

test("a signed organization stays in the set when another is added, and an unchanged set sends nothing", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/governance`);
  await settle(page);
  await page.getByRole("heading", { level: 2, name: "Milestones" }).click();
  const sec = page.getByRole("region", { name: "Partner sign-off" });
  await expect(
    sec.getByText("Acme Council signed on 10 Nov 2026"),
  ).toBeVisible();
  await sec
    .getByRole("button", { name: "Change who must sign Go live" })
    .click();
  await expect(sec.getByLabel("Acme Council")).toBeChecked();
  await expect(sec.getByLabel("Old Partner (no longer active)")).toHaveCount(0);
  await sec.getByRole("button", { name: "Save who must sign" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await sec
    .getByRole("button", { name: "Change who must sign Go live" })
    .click();
  await sec.getByLabel("Other Health Trust").check();
  await sec.getByRole("button", { name: "Save who must sign" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "Who must sign Go live is saved.",
  );
  await expect(
    sec.getByText(
      "Acme Council signed on 10 Nov 2026; Other Health Trust has not signed yet",
    ),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/milestones/${M1}/signoff-required`,
      body: { organization_pids: [ACME, OTHER] },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("an inactive organization still required must be unticked on purpose, and clearing the set is said", async ({
  page,
}) => {
  const sent = await stub(page, { oldRequired: true });
  await page.goto(`/en-001/plans/${PID}/governance`);
  await settle(page);
  await page.getByRole("heading", { level: 2, name: "Milestones" }).click();
  const sec = page.getByRole("region", { name: "Partner sign-off" });
  await sec
    .getByRole("button", { name: "Change who must sign Go live" })
    .click();
  await expect(sec.getByLabel("Old Partner (no longer active)")).toBeChecked();
  await sec.getByLabel("Other Health Trust").check();
  await sec.getByRole("button", { name: "Save who must sign" }).click();
  await expect(sec.getByRole("alert")).toHaveText(
    "Old Partner is no longer active, so it cannot be required: untick it to save.",
  );
  expect(sent).toEqual([]);
  await sec.getByLabel("Old Partner (no longer active)").uncheck();
  await sec.getByLabel("Acme Council").uncheck();
  await sec.getByLabel("Other Health Trust").uncheck();
  await sec.getByRole("button", { name: "Save who must sign" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "Go live no longer needs a partner sign-off.",
  );
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/milestones/${M1}/signoff-required`,
      body: { organization_pids: [] },
    },
  ]);
});

test("the sign-off editor reads in Welsh and passes axe", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/governance`);
  await settle(page);
  await page.getByRole("heading", { level: 2, name: "Cerrig milltir" }).click();
  await page
    .getByRole("button", { name: "Newid pwy sy'n gorfod llofnodi Go live" })
    .click();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
