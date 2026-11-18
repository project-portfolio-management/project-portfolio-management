// NC-1 slice 11 (EX-7): exceptions a threshold opened, and the threshold sweep, in a real browser over a stubbed
// service. A threshold exception says so instead of a confidence value and takes a threshold decision (pause, reset,
// stop, or an override with an expiry in the future and at most 366 days out), its rules said first; the sweep lists
// what it crossed and pauses, resets and stops nothing. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const E3 = "66666666-6666-4666-8666-666666666663";
const row = {
  pid: E3,
  plan_pid: PID,
  status: "open",
  opened_at: "2026-11-16T09:00:00Z",
  opened_value: "threshold",
  worst_value: "threshold",
  worsened_count: 0,
  cause: null,
  decision_needed: null,
  owner_ref: null,
  owner_role: null,
  decide_by: null,
  completed_at: null,
  complete: false,
  missing: ["cause", "decision_needed", "owner", "decide_by"],
  breached: false,
  window_hours: 48,
  hours_open: 20,
  closed_at: null,
  decision: null,
  decided_by: null,
  threshold_kind: "p80_cap",
};

async function stub(page: Page, extra: Array<Record<string, unknown>> = []) {
  const r = structuredClone(row);
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/(exceptions|threshold-sweep)/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET")
      return route.fulfill({
        json: {
          window_hours: 48,
          exceptions: [...(r.status === "open" ? [r] : []), ...extra],
        },
      });
    const body = req.postDataJSON();
    sent.push({ method: req.method(), path, body });
    if (path.endsWith("threshold-sweep"))
      return route.fulfill({
        json: {
          results: [
            {
              plan_pid: PID,
              programme: "Website replatform",
              kind: "p80_cap",
              crossed: true,
              outcome: "already_open",
              exception_pid: E3,
            },
            {
              plan_pid: PID,
              programme: "Website replatform",
              kind: "forecast_past",
              crossed: false,
            },
          ],
          note: "service text",
        },
      });
    Object.assign(r, {
      status: "closed",
      decision: `${body.action}: ${body.reason}`,
    });
    return route.fulfill({ json: { ok: true } });
  });
  return sent;
}

const isoIn = (days: number) =>
  new Date(Date.now() + days * 86_400_000).toISOString().slice(0, 10);

test("a threshold exception says so, and an override needs a future expiry within 366 days", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await expect(page.getByTestId("threshold-line")).toHaveText(
    "Opened because the programme crossed its threshold: P80 cost above a cap.",
  );
  await expect(page.getByText("Worst value")).toHaveCount(0);
  await page
    .getByRole("button", { name: "Record the decision: Website replatform" })
    .click();
  await page
    .getByRole("combobox", { name: /^Decision/ })
    .selectOption("override");
  await page
    .getByLabel("Reason (at least 10 characters)")
    .fill("Board accepted the overrun for Q1");
  await page
    .getByRole("button", { name: "Record the decision", exact: true })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "An override needs a date until which it holds.",
  );
  await page.getByLabel(/Override until/).fill(isoIn(400));
  await page
    .getByRole("button", { name: "Record the decision", exact: true })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "An override may last at most 366 days.",
  );
  expect(sent).toEqual([]);
  const until = isoIn(90);
  await page.getByLabel(/Override until/).fill(until);
  await page
    .getByRole("button", { name: "Record the decision", exact: true })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Decision recorded: Override.",
  );
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/exceptions/${E3}/threshold-decision`,
      body: {
        action: "override",
        reason: "Board accepted the overrun for Q1",
        expiry: until,
      },
    },
  ]);
});

test("a pause sends no expiry", async ({ page }) => {
  const sent = await stub(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await page
    .getByRole("button", { name: "Record the decision: Website replatform" })
    .click();
  await page
    .getByLabel("Reason (at least 10 characters)")
    .fill("Stop new work until the cap is reset");
  await page
    .getByRole("button", { name: "Record the decision", exact: true })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Decision recorded: Pause.",
  );
  expect(sent.map((x) => x.body)).toEqual([
    { action: "pause", reason: "Stop new work until the cap is reset" },
  ]);
});

test("each exception names the threshold that opened or joined it, in the catalogue's words", async ({
  page,
}) => {
  await stub(page, [
    // A confidence exception a threshold joined while it was open.
    {
      ...row,
      pid: "66666666-6666-4666-8666-666666666664",
      opened_value: "red",
      worst_value: "red",
      threshold_kind: "forecast_past",
    },
    // One delivery confidence alone opened: no threshold named.
    {
      ...row,
      pid: "66666666-6666-4666-8666-666666666665",
      opened_value: "amber_red",
      worst_value: "amber_red",
      threshold_kind: null,
    },
    // A threshold exception from before the kind was returned keeps the plain sentence.
    {
      ...row,
      pid: "66666666-6666-4666-8666-666666666666",
      threshold_kind: undefined,
    },
  ]);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await expect(page.getByTestId("threshold-line")).toHaveText([
    "Opened because the programme crossed its threshold: P80 cost above a cap.",
    "While this was open, the programme also crossed a threshold: Forecast finish past a date.",
    "Opened because a threshold of the programme was crossed.",
  ]);
  await expect(page.getByText(/forecast_past|p80_cap/)).toHaveCount(0);
});

test("the sweep lists what it crossed in the catalogue's words", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await page.getByRole("button", { name: "Check the thresholds now" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "2 thresholds checked; 0 exceptions opened.",
  );
  await expect(
    page
      .getByRole("list", { name: "Thresholds crossed" })
      .getByRole("listitem"),
  ).toHaveText([
    "Website replatform: P80 cost above a cap — exception already open",
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(sent).toEqual([
    { method: "POST", path: "/api/threshold-sweep", body: {} },
  ]);
});

test("the threshold decision reads in Welsh and passes axe", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/cy-001/exceptions");
  await settle(page);
  await expect(
    page.getByText(
      "Agorwyd oherwydd i'r rhaglen groesi ei throthwy: Cost P80 uwchlaw cap.",
    ),
  ).toBeVisible();
  await page
    .getByRole("button", { name: /Website replatform/ })
    .last()
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
