// NC-1 slice 12 part (b) (NC-4, BR-2): a plan's status updates on a cadence with their evidence, and the due list on
// the dashboard, in a real browser over a stubbed service. The plan page links here; where the plan stands is read
// from the service; the cadence's rule is said first, nothing is sent for the same cadence, and a 422 is the
// catalogue's sentence; an update's rules are said first and checked before sending, with claims citing the
// published numbers and the plan's records, and a 422 is the catalogue's sentence; each source is shown as the
// service resolves it; the service's English (labels, definitions, notes, details) is never shown. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const LATEST = {
  pid: "u2",
  plan_pid: PID,
  status: "at_risk",
  narrative: "Supplier late; recovery plan agreed.",
  posted_on: "2026-11-10",
  posted_by: "person:lead-7",
  posted_at: "2026-11-10T09:00:00Z",
  claims: [
    {
      text: "Two tasks are blocked by the supplier.",
      sources: [
        {
          source: "metric:blocked",
          state: "ok",
          kind: "metric",
          key: "blocked",
          value: 2,
          known: true,
          definition: "service text",
        },
        {
          source: "metric:overdue",
          state: "ok",
          kind: "metric",
          key: "overdue",
          value: null,
          known: false,
          definition: "service text",
        },
        {
          source: "task:t1",
          state: "ok",
          kind: "task",
          pid: "t1",
          title: "Order the servers",
        },
        { source: "risk:r9", state: "unavailable" },
      ],
    },
  ],
};
const EARLIER = {
  ...LATEST,
  pid: "u1",
  status: "on_track",
  narrative: "Started well.",
  posted_on: "2026-10-27",
  posted_by: null,
  claims: [],
};

async function stub(
  page: Page,
  opts: { cadence?: number | null; refuse?: number } = {},
) {
  let cadenceDays = opts.cadence === undefined ? 7 : opts.cadence;
  const updates = [LATEST, EARLIER];
  const standing = {
    due_on: "2026-11-17",
    due: true,
    overdue: true,
    days_overdue: 1,
  };
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(
    /\/api\/plans\/[^/]+\/(status|status-cadence|status-updates|tasks|risks|milestones|issues)$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      const end = path.split("/").at(-1);
      if (req.method() === "GET") {
        if (end === "status")
          return route.fulfill({
            json: {
              plan_pid: PID,
              as_of: "2026-11-18",
              latest: updates[0] ?? null,
              cadence_days: cadenceDays,
              standing: cadenceDays === null ? null : standing,
            },
          });
        if (end === "status-cadence")
          return cadenceDays === null
            ? route.fulfill({
                status: 404,
                json: { code: "not_found", detail: "service text" },
              })
            : route.fulfill({
                json: {
                  plan_pid: PID,
                  cadence_days: cadenceDays,
                  set_on: "2026-11-03",
                  set_by: "person:pmo-2",
                  standing,
                },
              });
        if (end === "status-updates") return route.fulfill({ json: updates });
        if (end === "tasks")
          return route.fulfill({
            json: {
              tasks: [{ pid: "t1", title: "Order the servers" }],
              counts: {},
            },
          });
        if (end === "risks")
          return route.fulfill({
            json: [{ pid: "r1", title: "Supplier insolvency" }],
          });
        if (end === "milestones")
          return route.fulfill({ json: [{ pid: "m1", name: "Go-live" }] });
        return route.fulfill({
          json: {
            plan_pid: PID,
            issues: [{ pid: "i1", title: "Data centre power" }],
          },
        });
      }
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      if (opts.refuse)
        return route.fulfill({
          status: opts.refuse,
          json: { code: "unprocessable", detail: "service text" },
        });
      if (end === "status-cadence") {
        cadenceDays = body.cadence_days;
        return route.fulfill({
          json: {
            plan_pid: PID,
            cadence_days: cadenceDays,
            set_on: "2026-11-18",
            set_by: "person:pmo-2",
            standing,
          },
        });
      }
      const posted = {
        ...LATEST,
        pid: `u${updates.length + 1}`,
        status: body.status,
        narrative: body.narrative,
        posted_on: "2026-11-18",
        claims: [],
      };
      updates.unshift(posted);
      return route.fulfill({ json: posted });
    },
  );
  await page.route("**/api/status-metrics", (route) =>
    route.fulfill({
      json: {
        due_soon_days: 7,
        metrics: ["overdue", "due_soon", "blocked", "no_due_date"].map(
          (key) => ({ key, label: "service text", definition: "service text" }),
        ),
        note: "service text",
      },
    }),
  );
  return sent;
}

test("the plan page links here, and where the plan stands and the history are read from the service, with no service text", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page
    .getByRole("link", { name: "Status updates and their evidence" })
    .click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Status updates" }),
  ).toBeVisible();
  const now = page.getByTestId("status-now");
  await expect(now).toContainText("! At risk");
  await expect(now).toContainText("posted on 10 Nov 2026 by person:lead-7");
  await expect(now).toContainText("Supplier late; recovery plan agreed.");
  await expect(now).toContainText("An update is expected every 7 days.");
  await expect(page.getByTestId("standing")).toHaveText(
    "✗ The next update was due on 17 Nov 2026: it is 1 day overdue.",
  );
  const history = page.getByRole("list", {
    name: "Every update, newest first",
  });
  await expect(history.getByRole("listitem").first()).toContainText(
    "Two tasks are blocked by the supplier.",
  );
  await expect(
    history
      .getByRole("list", { name: "Claims in the update of 10 Nov 2026" })
      .getByRole("listitem")
      .first()
      .getByRole("listitem"),
  ).toHaveText([
    "Blocked tasks: 2 now",
    "Overdue tasks: unknown (not zero)",
    "Task: Order the servers",
    "? A source that is no longer available",
  ]);
  await expect(history).toContainText("✓ On track · posted on 27 Oct 2026");
  await expect(
    page.getByText(
      "Open tasks due today or within the next 7 days. Unknown when there are open tasks and none has a due date.",
    ),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the cadence's rule is said first, nothing is sent for the same cadence, and a new one is sent as a number", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/status`);
  await settle(page);
  const region = page.getByRole("region", {
    name: "How often an update is expected",
  });
  await expect(
    region.getByText(
      "Every 1 to 92 days. Setting it restarts the clock from today, so the same cadence is not sent again. A cadence cannot be removed once it is set.",
    ),
  ).toBeVisible();
  await expect(
    region.getByText("Set on 3 Nov 2026 by person:pmo-2."),
  ).toBeVisible();
  const days = region.getByLabel("Days between updates (1 to 92)");
  await expect(days).toBeEnabled();
  await expect(days).toHaveValue("7");
  const save = region.getByRole("button", { name: "Save the cadence" });
  await save.click();
  await expect(region.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  for (const bad of ["0", "93"]) {
    await days.fill(bad);
    await save.click();
    await expect(region.getByRole("alert")).toHaveText(
      "A cadence is a whole number of days from 1 to 92.",
    );
  }
  expect(sent).toEqual([]);
  await days.fill("14");
  await save.click();
  await expect(region.getByRole("status")).toHaveText(
    "An update is now expected every 14 days; the clock starts today.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/status-cadence`,
      body: { cadence_days: 14 },
    },
  ]);
});

test("a plan with no cadence says so, and a refused cadence is the catalogue's sentence", async ({
  page,
}) => {
  const sent = await stub(page, { cadence: null, refuse: 422 });
  await page.goto(`/en-001/plans/${PID}/status`);
  await settle(page);
  await expect(page.getByTestId("standing")).toHaveText(
    "No cadence is set, so no update is ever due or overdue.",
  );
  const region = page.getByRole("region", {
    name: "How often an update is expected",
  });
  const days = region.getByLabel("Days between updates (1 to 92)");
  await expect(days).toBeEnabled();
  await expect(days).toHaveValue("");
  await days.fill("30");
  await region.getByRole("button", { name: "Save the cadence" }).click();
  await expect(region.getByRole("alert")).toHaveText(
    "A cadence is a whole number of days from 1 to 92.",
  );
  expect(sent).toHaveLength(1);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("an update's rules are said first, its claims cite numbers and records, and it is sent as built", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/status`);
  await settle(page);
  const region = page.getByRole("region", { name: "Post an update" });
  await expect(
    region.getByText(
      "You may add up to 10 claims, each a sentence of up to 500 characters citing up to 5 sources: the numbers below, or the plan's tasks, risks, milestones and issues. Each source is checked when the update is posted.",
    ),
  ).toBeVisible();
  const post = region.getByRole("button", { name: "Post the update" });
  await expect(post).toBeEnabled();
  await post.click();
  await expect(region.getByRole("alert")).toHaveText(
    "Choose on track, at risk or off track. Say what is happening, in up to 4000 characters.",
  );
  await region.getByRole("radio", { name: "Off track" }).check();
  await region
    .getByLabel("What is happening (up to 4000 characters)")
    .fill("  Go-live will slip a month.  ");
  await region.getByRole("button", { name: "Add a claim" }).click();
  const claim = region.getByRole("group", { name: "Claim 1" });
  await claim.getByRole("checkbox", { name: "Overdue tasks" }).check();
  await post.click();
  await expect(region.getByRole("alert")).toHaveText(
    "Claim 1: A claim is a sentence of up to 500 characters.",
  );
  expect(sent).toEqual([]);
  await claim
    .getByLabel("The claim (up to 500 characters)")
    .fill(" The supplier risk is now real. ");
  await claim
    .getByLabel("A record of the plan claim 1 cites")
    .selectOption({ label: "Supplier insolvency" });
  await claim
    .getByRole("button", { name: "Add the record to claim 1" })
    .click();
  await expect(
    claim
      .getByRole("list", { name: "Records claim 1 cites" })
      .getByRole("listitem"),
  ).toContainText("Risk: Supplier insolvency");
  await post.click();
  await expect(region.getByRole("status")).toHaveText(
    "Update posted: Off track.",
  );
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/status-updates`,
      body: {
        status: "off_track",
        narrative: "Go-live will slip a month.",
        claims: [
          {
            text: "The supplier risk is now real.",
            sources: ["metric:overdue", "risk:r1"],
          },
        ],
      },
    },
  ]);
  await expect(page.getByTestId("status-now")).toContainText(
    "✗ Off track · posted on 18 Nov 2026",
  );
});

test("a refused update is the catalogue's sentence, never the service's", async ({
  page,
}) => {
  await stub(page, { refuse: 422 });
  await page.goto(`/en-001/plans/${PID}/status`);
  await settle(page);
  const region = page.getByRole("region", { name: "Post an update" });
  await region.getByRole("radio", { name: "On track" }).check();
  await region
    .getByLabel("What is happening (up to 4000 characters)")
    .fill("All well.");
  await region.getByRole("button", { name: "Post the update" }).click();
  await expect(region.getByRole("alert")).toHaveText(
    "Refused: a source may no longer be a record of this plan. Nothing was posted; check the claims and post again.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the dashboard lists the plans whose update is due, most overdue first, each linked to its status page", async ({
  page,
}) => {
  await page.route("**/api/proxy/api/at-a-glance", (r) =>
    r.fulfill({
      json: {
        as_of: "2026-11-18T00:00:00Z",
        site_tiles: {
          plans: 2,
          proposals_open: 0,
          materialized_risks: 0,
          open_risk_exposure: 0,
          schedule_violations: 0,
          over_allocated_people: 0,
        },
        collections: [],
      },
    }),
  );
  await page.route("**/api/status-updates/due", (r) =>
    r.fulfill({
      json: [
        {
          plan_pid: PID,
          plan_name: "Apollo",
          cadence_days: 7,
          standing: {
            due_on: "2026-11-15",
            due: true,
            overdue: true,
            days_overdue: 3,
          },
          latest_status: "at_risk",
          last_update_on: "2026-11-08",
        },
        {
          plan_pid: "22222222-2222-4222-8222-222222222222",
          plan_name: null,
          cadence_days: 14,
          standing: {
            due_on: "2026-11-18",
            due: true,
            overdue: false,
            days_overdue: 0,
          },
          latest_status: null,
          last_update_on: null,
        },
      ],
    }),
  );
  await page.goto("/en-001/dashboard");
  await settle(page);
  const due = page.getByRole("list", { name: "Status updates due" });
  await expect(due.getByRole("listitem")).toHaveText([
    "Apollo: ✗ The next update was due on 15 Nov 2026: it is 3 days overdue. Last update: At risk, on 8 Nov 2026.",
    "A plan: ! An update is due today, 18 Nov 2026. No update yet.",
  ]);
  await expect(due.getByRole("link", { name: "Apollo" })).toHaveAttribute(
    "href",
    `/en-001/plans/${PID}/status`,
  );
});

test("the page reads in Welsh and passes axe with a claim open", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/status`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Diweddariadau statws" }),
  ).toBeVisible();
  await expect(page.getByTestId("standing")).toHaveText(
    /Roedd y diweddariad nesaf yn ddyledus ar .+: mae 1 diwrnod yn hwyr\./,
  );
  await page.getByRole("button", { name: "Ychwanegu honiad" }).click();
  await expect(page.getByRole("group", { name: "Honiad 1" })).toBeVisible();
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
