// FR-54 in a real browser: raise an escalation on a plan, work it (acknowledge, take it up a level with a reason,
// resolve), see the register and the link from a red tolerance. The service is stubbed; the pages pass axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
}

const esc = (over: Record<string, unknown>) => ({
  pid: "e1",
  plan_pid: PID,
  title: "Cost is beyond tolerance",
  description: "The forecast is 15% over.",
  reason_kind: "tolerance",
  tolerance_dimension: "cost",
  level: 1,
  escalated_to: "Programme manager",
  owner_ref: null,
  status: "open",
  raised_by: "u",
  raised_on: "2026-10-10",
  respond_by: null,
  outcome: null,
  response: null,
  resolved_on: null,
  age_days: 10,
  overdue: false,
  ...over,
});

test("an escalation is raised, taken up a level and resolved", async ({
  page,
}) => {
  let current = esc({});
  let live = false;
  const sent: Record<string, unknown> = {};
  const events = () => [
    {
      event: "raised",
      level: 1,
      actor: "u",
      note: null,
      at: "2026-10-10T09:00:00Z",
    },
    ...(current.level === 2
      ? [
          {
            event: "level_raised",
            level: 2,
            actor: "u",
            note: "No answer in a week",
            at: "2026-10-17T09:00:00Z",
          },
        ]
      : []),
  ];
  await page.route(`**/api/plans/${PID}/escalations`, async (r) => {
    if (r.request().method() === "POST") {
      sent.raised = r.request().postDataJSON();
      live = true;
      return r.fulfill({ json: current });
    }
    return r.fulfill({
      json: { plan_pid: PID, escalations: live ? [current] : [] },
    });
  });
  await page.route(`**/api/plans/${PID}/escalations/e1`, (r) =>
    r.fulfill({ json: { ...current, events: events() } }),
  );
  await page.route(`**/api/plans/${PID}/escalations/e1/raise-level`, (r) => {
    sent.up = r.request().postDataJSON();
    current = esc({ level: 2, escalated_to: "Sponsor" });
    return r.fulfill({ json: current });
  });
  const resolution = {
    escalation_pid: "e1",
    plan_pid: PID,
    outcome: "decided",
    response: "Funded from contingency",
    resolved_on: "2026-10-20",
    resolved_by: "person:ana",
    resolved_at: "2026-10-20",
  };
  await page.route(`**/api/plans/${PID}/escalations/e1/resolution`, (r) => {
    if (r.request().method() === "GET")
      return current.status === "resolved"
        ? r.fulfill({ json: resolution })
        : r.fulfill({
            status: 404,
            json: { code: "not_found", detail: "service text" },
          });
    sent.resolved = r.request().postDataJSON();
    current = esc({
      level: 2,
      escalated_to: "Sponsor",
      status: "resolved",
      outcome: "decided",
      response: "Funded from contingency",
      resolved_on: "2026-10-20",
      age_days: 10,
    });
    return r.fulfill({ json: resolution });
  });
  await page.route(`**/api/plans/${PID}/escalations/e1/acknowledgement`, (r) =>
    r.fulfill({
      status: 404,
      json: { code: "not_found", detail: "service text" },
    }),
  );

  await page.goto(
    `/en-001/plans/${PID}/escalations?reason=tolerance&dimension=cost`,
  );
  await expect(page.getByText("No escalations yet.")).toBeVisible();
  // The link from a red tolerance pre-fills a tolerance escalation about cost.
  await expect(page.getByLabel("Which tolerance")).toHaveValue("cost");

  // A draft with no title is refused here, in words, before anything is sent.
  await page.getByRole("button", { name: "Raise it" }).click();
  await expect(page.getByRole("alert")).toContainText("Give it a title");
  expect(sent.raised).toBeUndefined();
  await page.getByLabel("Title").fill("Cost is beyond tolerance");
  await page.getByRole("button", { name: "Raise it" }).click();
  expect(sent.raised).toEqual({
    title: "Cost is beyond tolerance",
    reason_kind: "tolerance",
    tolerance_dimension: "cost",
  });
  await expect(page.getByRole("status")).toContainText(
    "Raised Cost is beyond tolerance.",
  );
  await expect(page.getByText("open", { exact: false }).first()).toBeVisible();

  await page
    .getByRole("button", { name: "Show Cost is beyond tolerance" })
    .click();
  const region = page.getByRole("region", {
    name: "Show Cost is beyond tolerance",
  });
  await expect(region).toContainText("The forecast is 15% over.");
  await expect(region).toContainText("Raised, Level 1");
  await audit(page);

  // Taking it up needs a reason of ten characters.
  await region.getByRole("button", { name: "Take it up a level" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Write at least 10 characters.",
  );
  expect(sent.up).toBeUndefined();
  await region.getByLabel("Why it is going up").fill("No answer in a week");
  await region.getByRole("button", { name: "Take it up a level" }).click();
  expect(sent.up).toEqual({ note: "No answer in a week" });
  await expect(page.getByText("Level 2: Sponsor")).toBeVisible();
  await expect(region).toContainText("Taken up a level");

  await region.getByLabel("Response").fill("Funded from contingency");
  await region.getByRole("button", { name: "Resolve", exact: true }).click();
  // A resolution is a record: the page asks before recording it.
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("status")).toContainText(
    "Resolved; the record is shown below.",
  );
  expect(sent.resolved).toEqual({
    outcome: "decided",
    response: "Funded from contingency",
  });
  await expect(
    region.getByRole("button", { name: "Resolve", exact: true }),
  ).toHaveCount(0);
  const rec = region.getByTestId("esc-resolution");
  await expect(rec).toContainText(
    "Resolved 20 Oct 2026 by person:ana: Decided.",
  );
  await expect(rec).toContainText("Funded from contingency");
  await expect(rec).toContainText(
    "A resolution is a record: it is not changed or overwritten.",
  );
  await expect(region.getByTestId("esc-ack")).toHaveText(
    "Not acknowledged at its current level.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

// ---- NC-1 slice 12 part (d), CF-4: the acknowledgement and the resolution as records ----------------------------

const ACK = {
  escalation_pid: "e1",
  plan_pid: PID,
  level: 1,
  acknowledged_by: "person:ana",
  acknowledged_at: "2026-10-12",
  note: "Seen, on it",
};

/** One escalation in `status`; the records answer as the escalation stands; `refuse` makes a write refuse. */
async function stubRecords(
  page: Page,
  opts: {
    status?: string;
    refuse?: { status: number; code: string };
    ackRead?: number;
  } = {},
) {
  let current = esc({ status: opts.status ?? "open" });
  const sent: { method: string; url: string; body: unknown }[] = [];
  let refuse = opts.refuse;
  await page.route(`**/api/plans/${PID}/escalations`, (r) =>
    r.fulfill({ json: { plan_pid: PID, escalations: [current] } }),
  );
  await page.route(`**/api/plans/${PID}/escalations/e1`, (r) =>
    r.fulfill({
      json: {
        ...current,
        events: [
          {
            event: "raised",
            level: 1,
            actor: "u",
            note: null,
            at: "2026-10-10T09:00:00Z",
          },
        ],
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/escalations/e1/*`, (r) => {
    const req = r.request();
    const what = new URL(req.url()).pathname.split("/").pop();
    if (req.method() === "GET") {
      if (what === "acknowledgement" && opts.ackRead)
        return r.fulfill({
          status: opts.ackRead,
          json: { code: "internal", detail: "service text" },
        });
      if (what === "acknowledgement" && current.status !== "open")
        return r.fulfill({ json: ACK });
      return r.fulfill({
        status: 404,
        json: { code: "not_found", detail: "service text" },
      });
    }
    sent.push({
      method: req.method(),
      url: new URL(req.url()).pathname.replace(
        `/api/proxy/api/plans/${PID}/escalations/e1/`,
        "",
      ),
      body: req.postDataJSON(),
    });
    if (refuse)
      return r.fulfill({
        status: refuse.status,
        json: { code: refuse.code, detail: "service text" },
      });
    if (what === "acknowledgement") {
      current = esc({ status: "acknowledged" });
      return r.fulfill({ json: ACK });
    }
    return r.fulfill({ status: 500, json: {} });
  });
  return {
    sent,
    setRefuse: (x?: { status: number; code: string }) => {
      refuse = x;
    },
  };
}

async function openIt(page: Page, locale = "en-001", show = "Show") {
  await page.goto(`/${locale}/plans/${PID}/escalations`, {
    waitUntil: "networkidle",
  });
  await page
    .getByRole("button", { name: `${show} Cost is beyond tolerance` })
    .click();
  return page.getByRole("region", {
    name: `${show} Cost is beyond tolerance`,
  });
}

test("acknowledging records who and when, with a note, and the record is shown", async ({
  page,
}) => {
  const { sent } = await stubRecords(page);
  const region = await openIt(page);
  // Said first: the rights, the level rule and that a resolution is never overwritten.
  await expect(region.getByTestId("esc-rules")).toHaveText(
    "Acknowledging needs the right to acknowledge escalations, and resolving the right to resolve them; an administrator holds both, and this page cannot tell beforehand whether you do. An acknowledgement counts at the current level: taking it up a level clears it. A resolution is a record and is never overwritten.",
  );
  await expect(region.getByTestId("esc-ack")).toHaveText(
    "Not acknowledged at its current level.",
  );
  await audit(page);
  await region.getByLabel("Note (optional)").fill("  Seen, on it ");
  await region.getByRole("button", { name: "Acknowledge" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Acknowledged; the record is shown below.",
  );
  expect(sent).toEqual([
    { method: "PUT", url: "acknowledgement", body: { note: "Seen, on it" } },
  ]);
  await expect(region.getByTestId("esc-ack")).toContainText(
    "Acknowledged at level 1 by person:ana, 12 Oct 2026.",
  );
  await expect(region.getByTestId("esc-ack")).toContainText(
    "Note: Seen, on it",
  );
  // Once acknowledged it is not offered again, so a repeat is never sent.
  await expect(region.getByRole("button", { name: "Acknowledge" })).toHaveCount(
    0,
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a blank note is not sent", async ({ page }) => {
  const { sent } = await stubRecords(page);
  const region = await openIt(page);
  await region.getByLabel("Note (optional)").fill("   ");
  await region.getByRole("button", { name: "Acknowledge" }).click();
  await expect(page.getByRole("status")).toContainText("Acknowledged");
  expect(sent).toEqual([{ method: "PUT", url: "acknowledgement", body: {} }]);
});

test("the resolution's rules are said first; nothing is sent for a short response or a Cancel", async ({
  page,
}) => {
  const { sent } = await stubRecords(page, { status: "acknowledged" });
  const region = await openIt(page);
  await expect(region.getByTestId("esc-ack")).toContainText(
    "Acknowledged at level 1 by person:ana",
  );
  const form = region.getByRole("form", { name: "Resolve this escalation" });
  await expect(form).toContainText(
    "The response is 10 to 10,000 characters. You are asked before it is recorded, and it cannot be changed afterwards.",
  );
  await form.getByLabel("Response").fill("Too short");
  await form.getByRole("button", { name: "Resolve" }).click();
  await expect(form.getByRole("alert")).toHaveText(
    "Write a response of 10 to 10,000 characters.",
  );
  await expect(page.getByRole("alertdialog")).toHaveCount(0);
  await form.getByLabel("Outcome").selectOption("rejected");
  await form.getByLabel("Response").fill("Out of the programme's scope");
  await form.getByRole("button", { name: "Resolve" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Record this resolution of “Cost is beyond tolerance”? It cannot be changed or overwritten afterwards.",
  );
  await audit(page);
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
});

test("each refused record says the catalogue's sentence, never the service's", async ({
  page,
}) => {
  const { sent, setRefuse } = await stubRecords(page, {
    refuse: { status: 403, code: "forbidden" },
  });
  const region = await openIt(page);
  const refusal = page.getByTestId("esc-refusal");
  await region.getByRole("button", { name: "Acknowledge" }).click();
  await expect(refusal).toHaveText(
    "Refused: acknowledging an escalation needs the right to acknowledge escalations (an administrator holds it). Nothing was recorded.",
  );
  setRefuse({ status: 409, code: "conflict" });
  await region.getByRole("button", { name: "Acknowledge" }).click();
  await expect(refusal).toHaveText(
    "Refused: this escalation was resolved or withdrawn meanwhile, so it cannot be acknowledged. It was read again.",
  );

  const form = region.getByRole("form", { name: "Resolve this escalation" });
  const resolve = async () => {
    await form.getByLabel("Response").fill("Funded from contingency");
    await form.getByRole("button", { name: "Resolve" }).click();
    await page
      .getByRole("alertdialog")
      .getByRole("button", { name: "OK" })
      .click();
  };
  for (const [status, code, text] of [
    [
      403,
      "forbidden",
      "Refused: resolving an escalation needs the right to resolve escalations (an administrator holds it). Nothing was recorded.",
    ],
    [
      409,
      "conflict",
      "Refused: this escalation was withdrawn, or is already resolved with a different outcome or response; a resolution is never overwritten. It was read again.",
    ],
    [
      422,
      "unprocessable",
      "Refused: the outcome or the response was not accepted. Check the rules above and try again.",
    ],
    [
      404,
      "not_found",
      "This escalation is gone, or you can no longer see it. The list was read again.",
    ],
  ] as const) {
    setRefuse({ status, code });
    await resolve();
    await expect(refusal).toHaveText(text);
  }
  expect(sent.map((s) => `${s.method} ${s.url}`)).toEqual([
    "PUT acknowledgement",
    "PUT acknowledgement",
    "PUT resolution",
    "PUT resolution",
    "PUT resolution",
    "PUT resolution",
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a record that cannot be read says so, and the history still shows", async ({
  page,
}) => {
  await stubRecords(page, { status: "acknowledged", ackRead: 500 });
  const region = await openIt(page);
  await expect(region.getByTestId("esc-records-failed")).toHaveText(
    "The acknowledgement or the resolution could not be read; the history below still shows what happened.",
  );
  await expect(region).toContainText("Raised, Level 1");
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the records read in Welsh", async ({ page }) => {
  await stubRecords(page, { status: "acknowledged" });
  const region = await openIt(page, "cy-001", "Dangos");
  await expect(
    region.getByRole("heading", { name: "Cydnabyddiaeth a datrysiad" }),
  ).toBeVisible();
  await expect(region.getByTestId("esc-ack")).toContainText(
    "Cydnabuwyd ar lefel 1 gan person:ana",
  );
  await expect(region.getByTestId("esc-ack")).toContainText(
    "Nodyn: Seen, on it",
  );
  await expect(
    region.getByRole("form", { name: "Datrys yr uwchgyfeiriad hwn" }),
  ).toBeVisible();
});

test("the register shows state as words and shapes and flags the overdue", async ({
  page,
}) => {
  const rows = [
    esc({
      pid: "a",
      title: "Vendor slips",
      plan_name: "Apollo",
      overdue: true,
      age_days: 21,
    }),
    esc({
      pid: "b",
      title: "Scope change",
      plan_name: "Gemini",
      status: "acknowledged",
      level: 2,
      escalated_to: "Sponsor",
      raised_by: "system:tolerance-escalations",
    }),
  ];
  await page.route("**/api/escalations**", (r) => {
    const url = new URL(r.request().url());
    const onlyOverdue = url.searchParams.get("overdue") === "true";
    const list = rows.filter((x) => !onlyOverdue || x.overdue);
    return r.fulfill({
      json: {
        escalations: list,
        counts: { open: 1, acknowledged: 1 },
        overdue: 1,
      },
    });
  });
  await page.goto("/en-001/escalations");
  const body = page.getByRole("row");
  await expect(body.nth(1)).toContainText("Vendor slips");
  await expect(body.nth(1)).toContainText("overdue");
  await expect(body.nth(2)).toContainText("acknowledged");
  await expect(body.nth(2)).toContainText("Raised automatically");
  await expect(body.nth(1)).not.toContainText("Raised automatically");
  await expect(
    page.getByRole("link", { name: "Vendor slips" }),
  ).toHaveAttribute("href", /\/plans\/.*\/escalations$/);
  await audit(page);
  await page.getByLabel("Show").selectOption("overdue");
  await expect(page.getByRole("link", { name: "Scope change" })).toHaveCount(0);
});

test("a red tolerance with no escalation offers to raise one", async ({
  page,
}) => {
  const dimension = (name: string, over: Record<string, unknown>) => ({
    dimension: name,
    unit: "u",
    basis: `How ${name} is measured.`,
    status: "not_set",
    limit: null,
    source: "none",
    ...over,
  });
  await page.route(`**/api/plans/${PID}/tolerances`, (r) =>
    r.fulfill({
      json: {
        plan_pid: PID,
        plan_name: "Apollo",
        overall: "red",
        escalation_needed: true,
        as_of: "2026-10-20T10:00:00Z",
        how_to_read: "x",
        dimensions: [
          dimension("time", {
            status: "red",
            limit: 5,
            measured: 9,
            usage_percent: 180,
            source: "own",
            amber_from_percent: 75,
            open_escalations: 0,
            escalation_needed: true,
          }),
          dimension("cost", {
            status: "red",
            limit: 500,
            measured: 900,
            usage_percent: 180,
            source: "own",
            amber_from_percent: 75,
            open_escalations: 2,
            escalation_needed: false,
          }),
          dimension("risk", {}),
          dimension("resources", {}),
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/tolerances`);
  const time = page.getByRole("region", { name: /Time/ });
  await expect(time).toContainText("nobody has escalated it");
  await expect(
    time.getByRole("link", { name: "Raise an escalation" }),
  ).toHaveAttribute("href", /escalations\?reason=tolerance&dimension=time$/);
  const cost = page.getByRole("region", { name: /Cost/ });
  await expect(cost).toContainText("2 open escalations");
  await expect(cost).not.toContainText("nobody has escalated it");
  await audit(page);
});
