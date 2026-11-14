// NC-1 slice 6 (LT-11): the caller's own timesheet and an approver's decision, in a real browser over a stubbed
// service. The week grid shows each plan's minutes by day in hours and minutes, with billable and non-billable apart
// and the day totals; an empty week says so and cannot be sent; the week is sent for approval; weeks are moved back
// and forward and typed, and "this week" is the week of today in the browser's time zone. An approver reads someone
// else's week and approves it or sends it back with a reason; their own week and a rejection with no reason are
// refused on the page. A 403, 409 or 422 says the catalogue's sentence, not the service's text. Welsh reads. The page
// passes axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const ME = "person:6f1c2a3b-0000-4000-8000-000000000009";
const ALICE = "person:6f1c2a3b-0000-4000-8000-000000000001";
const PLAN_A = "11111111-1111-4111-8111-111111111111";
const PLAN_B = "22222222-2222-4222-8222-222222222222";
// Wednesday 11 November 2026, in ISO week 2026-W46 (Monday 9 to Sunday 15 November) in London.
const NOW = new Date("2026-11-11T10:00:00Z");

test.use({ timezoneId: "Europe/London" });

interface Sent {
  method: string;
  path: string;
  body: unknown;
}

/** The Monday of an ISO week, as a civil date (the stub's own, so it does not lean on the page's code). */
function monday(week: string): string {
  const [y, w] = week.split("-W").map(Number) as [number, number];
  const jan4 = Date.UTC(y, 0, 4);
  const day = (new Date(jan4).getUTCDay() + 6) % 7;
  return new Date(jan4 - day * 86_400_000 + (w - 1) * 7 * 86_400_000)
    .toISOString()
    .slice(0, 10);
}

function days(week: string): string[] {
  const start = Date.parse(`${monday(week)}T00:00:00Z`);
  return Array.from({ length: 7 }, (_, i) =>
    new Date(start + i * 86_400_000).toISOString().slice(0, 10),
  );
}

type Row = { plan_pid: string; by_day: number[]; billable: boolean[] };

/** What each person recorded, by week. A day's minutes are billable or not as `billable` says. */
const RECORDED: Record<string, Row[]> = {
  [`${ME}|2026-W46`]: [
    {
      plan_pid: PLAN_A,
      by_day: [90, 0, 45, 0, 0, 0, 0],
      billable: [true, true, false, true, true, true, true],
    },
    {
      plan_pid: PLAN_B,
      by_day: [0, 30, 0, 0, 0, 0, 0],
      billable: [false, false, false, false, false, false, false],
    },
  ],
  [`${ALICE}|2026-W46`]: [
    {
      plan_pid: PLAN_A,
      by_day: [0, 0, 0, 480, 0, 0, 0],
      billable: [true, true, true, true, true, true, true],
    },
  ],
};

/** A person reference as the service compares it: on the id, whatever the prefix or case. */
function norm(person: string): string {
  return `person:${(person.split(":")[1] ?? "").toLowerCase()}`;
}

function grid(person: string, week: string) {
  const rows = (RECORDED[`${norm(person)}|${week}`] ?? []).map((r) => {
    const billable = r.by_day.reduce(
      (s, m, i) => s + (r.billable[i] ? m : 0),
      0,
    );
    const total = r.by_day.reduce((s, m) => s + m, 0);
    return {
      plan_pid: r.plan_pid,
      by_day: r.by_day,
      split: { billable, non_billable: total - billable },
      total,
    };
  });
  const day_totals = [0, 1, 2, 3, 4, 5, 6].map((i) =>
    rows.reduce((s, r) => s + r.by_day[i]!, 0),
  );
  const split = rows.reduce(
    (s, r) => ({
      billable: s.billable + r.split.billable,
      non_billable: s.non_billable + r.split.non_billable,
    }),
    { billable: 0, non_billable: 0 },
  );
  const any = rows.length > 0;
  return {
    days: days(week),
    rows,
    day_totals,
    split,
    total: any ? split.billable + split.non_billable : null,
    reason: any ? null : "no_entries",
  };
}

/** The stub; `refuse` makes the next call of that method and path answer with that status. */
async function stub(
  page: Page,
  refuse: Partial<Record<string, number>> = {},
  initial: Record<string, string> = { [`${ALICE}|2026-W46`]: "submitted" },
) {
  const sent: Sent[] = [];
  const state: Record<
    string,
    { status: string; decided_by: string | null; comment: string | null }
  > = {};
  for (const [k, status] of Object.entries(initial))
    state[k] = { status, decided_by: null, comment: null };
  const refusal = (key: string) => {
    const status = refuse[key];
    if (!status) return null;
    delete refuse[key];
    return status;
  };
  await page.route(/\/api\/proxy\/api\/(timesheets|plans\/)/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    const body = req.postDataJSON?.() ?? null;
    if (method !== "GET") sent.push({ method, path, body });
    const status = refusal(`${method} ${path}`);
    if (status)
      return route.fulfill({
        status,
        json: {
          error: "refused",
          description: "service text that must not be shown",
        },
      });
    if (path === `/api/plans/${PLAN_A}`)
      return route.fulfill({
        json: { pid: PLAN_A, kind: "Project", name: "Website replatform" },
      });
    if (path === "/api/timesheets" && method === "GET") {
      const week = url.searchParams.get("week")!;
      const person = url.searchParams.get("person") ?? ME;
      const s = state[`${norm(person)}|${week}`];
      return route.fulfill({
        json: {
          person,
          week,
          week_start: monday(week),
          status: s?.status ?? "open",
          decided_by: s?.decided_by ?? null,
          decision_comment: s?.comment ?? null,
          grid: grid(person, week),
          omitted_unreadable:
            norm(person) === ME && week === "2026-W46" ? 1 : 0,
        },
      });
    }
    if (path === "/api/timesheets/submit" && method === "POST") {
      const b = body as { week: string };
      const key = `${ME}|${b.week}`;
      if (!RECORDED[key])
        return route.fulfill({ status: 422, json: { error: "unprocessable" } });
      if (state[key] && state[key].status !== "rejected")
        return route.fulfill({ status: 409, json: { error: "conflict" } });
      state[key] = { status: "submitted", decided_by: null, comment: null };
      return route.fulfill({
        json: { pid: "w1", person_ref: ME, status: "submitted" },
      });
    }
    if (path === "/api/timesheets/decide" && method === "POST") {
      const b = body as {
        person: string;
        week: string;
        decision: string;
        comment?: string;
      };
      const key = `${norm(b.person)}|${b.week}`;
      if (state[key]?.status !== "submitted")
        return route.fulfill({ status: 409, json: { error: "conflict" } });
      state[key] = {
        status: b.decision,
        decided_by: ME,
        comment: b.comment ?? null,
      };
      return route.fulfill({
        json: { pid: "w2", person_ref: b.person, status: b.decision },
      });
    }
    return route.fulfill({
      status: 404,
      json: { error: "unhandled in stub" },
    });
  });
  return sent;
}

async function open(page: Page, locale = "en-001", now = NOW) {
  await page.clock.setFixedTime(now);
  await page.goto(`/${locale}/me/timesheet`);
  await expect(page.locator("h1")).toBeVisible();
}

test("my work links to my timesheet", async ({ page }) => {
  await page.route(/\/api\/proxy\/api\/me\/tasks/, (route) =>
    route.fulfill({ status: 404, json: { error: "unhandled in stub" } }),
  );
  await page.goto("/en-001/me");
  await page.getByRole("link", { name: "My timesheet" }).click();
  await expect(page).toHaveURL(/\/en-001\/me\/timesheet$/);
});

test("my week shows each plan by day, billable and not apart, and is sent for approval", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const mine = page.getByTestId("mine");
  await expect(
    mine.getByRole("heading", { name: "Your week 2026-W46" }),
  ).toBeVisible();
  await expect(
    page.getByText(/Today is .+ in your time zone, Europe\/London/),
  ).toBeVisible();
  await expect(mine.getByTestId("mine-status")).toHaveText(
    "○ Open: not sent for approval yet",
  );
  const table = mine.getByRole("table", {
    name: "Your time in week 2026-W46, by plan and day",
  });
  // A header for the plan, seven days, the total and the two halves.
  await expect(table.locator("thead th")).toHaveCount(11);
  const a = table.getByRole("row", { name: /Website replatform/ });
  await expect(
    a.getByRole("link", { name: "Website replatform" }),
  ).toHaveAttribute("href", `/en-001/plans/${PLAN_A}`);
  await expect(a.getByRole("cell")).toHaveText([
    "1 hr 30 mins",
    "",
    "45 mins",
    "",
    "",
    "",
    "",
    "2 hrs 15 mins",
    "1 hr 30 mins",
    "45 mins",
  ]);
  await expect(
    table.getByRole("row", { name: /Plan 22222222… \(name not shown\)/ }),
  ).toBeVisible();
  await expect(
    table.getByRole("row", { name: /Each day/ }).getByRole("cell"),
  ).toHaveText([
    "1 hr 30 mins",
    "30 mins",
    "45 mins",
    "0 mins",
    "0 mins",
    "0 mins",
    "0 mins",
    "2 hrs 45 mins",
    "1 hr 30 mins",
    "1 hr 15 mins",
  ]);
  await expect(mine.getByTestId("timesheet-split")).toHaveText(
    "Billable 1 hr 30 mins, not billable 1 hr 15 mins: 2 hrs 45 mins in all.",
  );
  await expect(
    mine.getByText(
      "Entries left out because they are in plans you cannot read: 1.",
    ),
  ).toBeVisible();

  await mine
    .getByRole("button", { name: "Send this week for approval" })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Week 2026-W46 was sent for approval.",
  );
  await expect(mine.getByTestId("mine-status")).toHaveText(
    "◔ Sent for approval: waiting for a decision",
  );
  await expect(
    mine.getByText("This week is waiting for an approver's decision."),
  ).toBeVisible();
  await expect(
    mine.getByRole("button", { name: "Send this week for approval" }),
  ).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/timesheets/submit",
      body: { week: "2026-W46" },
    },
  ]);
});

test("weeks move back and forward, a typed week is read, and an empty week cannot be sent", async ({
  page,
}) => {
  await stub(page);
  await open(page);
  const mine = page.getByTestId("mine");
  await expect(
    page.getByRole("button", { name: "This week", exact: true }),
  ).toBeDisabled();
  await page.getByRole("button", { name: "Previous week" }).click();
  await expect(
    mine.getByRole("heading", { name: "Your week 2026-W45" }),
  ).toBeVisible();
  await expect(mine.getByTestId("timesheet-empty")).toHaveText(
    "No time is recorded in this week, so there is no total.",
  );
  await expect(
    mine.getByText("A week with no time recorded cannot be sent for approval."),
  ).toBeVisible();
  await expect(
    mine.getByRole("button", { name: /Send this week/ }),
  ).toHaveCount(0);
  await page.getByRole("button", { name: "Next week" }).click();
  await page.getByRole("button", { name: "Next week" }).click();
  await expect(
    mine.getByRole("heading", { name: "Your week 2026-W47" }),
  ).toBeVisible();

  await page.getByLabel("Week, such as 2026-W46").fill("2026-W54");
  await page.getByRole("button", { name: "Show the week" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "That is not a week: write the year, ‘-W’ and the week's number, such as 2026-W46.",
  );
  await page.getByLabel("Week, such as 2026-W46").fill(" 2026-w53 ");
  await page.getByRole("button", { name: "Show the week" }).click();
  await expect(
    mine.getByRole("heading", { name: "Your week 2026-W53" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "This week", exact: true }).click();
  await expect(
    mine.getByRole("heading", { name: "Your week 2026-W46" }),
  ).toBeVisible();
});

test.describe("in Auckland", () => {
  test.use({ timezoneId: "Pacific/Auckland" });
  test("this week is the week of today in the browser's time zone", async ({
    page,
  }) => {
    await stub(page);
    // Sunday 15 November at 23:30 UTC is already Monday 16 November in Auckland: week 47, not 46.
    await open(page, "en-001", new Date("2026-11-15T23:30:00Z"));
    await expect(
      page
        .getByTestId("mine")
        .getByRole("heading", { name: "Your week 2026-W47" }),
    ).toBeVisible();
    await expect(
      page.getByText(/in your time zone, Pacific\/Auckland/),
    ).toBeVisible();
  });
});

test("a refused submit says the catalogue's sentence", async ({ page }) => {
  await stub(page, {
    "POST /api/timesheets/submit": 409,
  });
  await open(page);
  const mine = page.getByTestId("mine");
  await mine
    .getByRole("button", { name: "Send this week for approval" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "This week has already been sent for approval or approved; it has been read again.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { "POST /api/timesheets/submit": 422 });
  await open(page);
  await mine
    .getByRole("button", { name: "Send this week for approval" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "The week was not sent: it has no time recorded in it.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { "POST /api/timesheets/submit": 403 });
  await open(page);
  await mine
    .getByRole("button", { name: "Send this week for approval" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Only the person a week belongs to may send it for approval.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("an approver sends a week back, which needs a reason", async ({
  page,
}) => {
  const sent = await stub(page, {}, { [`${ALICE}|2026-W46`]: "submitted" });
  await open(page);
  const other = page.getByTestId("other");
  await other
    .getByLabel("Person: ‘person:’ or ‘worker:’ and their id")
    .fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await expect(
    other.getByRole("heading", { name: `Week 2026-W46 of ${ALICE}` }),
  ).toBeVisible();
  await expect(other.getByTestId("other-status")).toHaveText(
    "◔ Sent for approval: waiting for a decision",
  );
  await expect(
    other
      .getByRole("table", {
        name: `Time of ${ALICE} in week 2026-W46, by plan and day`,
      })
      .getByRole("row", { name: /Website replatform/ })
      .getByRole("cell")
      .nth(3),
  ).toHaveText("8 hrs");

  await other.getByLabel("Send back", { exact: true }).check();
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Say why the week is sent back.",
  );
  await other.getByLabel("Why it is sent back (needed)").fill(" fix Thursday ");
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("status")).toHaveText(
    `Week 2026-W46 of ${ALICE} was sent back.`,
  );
  await expect(other.getByTestId("other-status")).toHaveText(
    "✗ Sent back: change it and send it again — with the note: fix Thursday",
  );
  await expect(
    other.getByText(
      "This week was sent back; it can be decided again once the person sends it again.",
    ),
  ).toBeVisible();
  await expect(
    other.getByRole("button", { name: "Record the decision" }),
  ).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/timesheets/decide",
      body: {
        person: ALICE,
        week: "2026-W46",
        decision: "rejected",
        comment: "fix Thursday",
      },
    },
  ]);
});

test("my own sent-back week shows the approver's reason and offers sending again", async ({
  page,
}) => {
  await page.route(/\/api\/proxy\/api\/timesheets\?/, (route) =>
    route.fulfill({
      json: {
        person: ME,
        week: "2026-W46",
        week_start: "2026-11-09",
        status: "rejected",
        decided_by: ALICE,
        decision_comment: "fix Monday",
        grid: grid(ME, "2026-W46"),
        omitted_unreadable: 0,
      },
    }),
  );
  await open(page);
  const mine = page.getByTestId("mine");
  await expect(mine.getByTestId("mine-status")).toHaveText(
    "✗ Sent back: change it and send it again — the approver said: fix Monday",
  );
  await expect(
    mine.getByRole("button", { name: "Send this week for approval again" }),
  ).toBeVisible();
});

test("an approver approves, which is final; their own week and refusals are said in words", async ({
  page,
}) => {
  const sent = await stub(
    page,
    {},
    {
      [`${ALICE}|2026-W46`]: "submitted",
      [`${ME}|2026-W46`]: "submitted",
    },
  );
  await open(page);
  const other = page.getByTestId("other");
  const person = other.getByLabel(
    "Person: ‘person:’ or ‘worker:’ and their id",
  );

  await person.fill("alice");
  await other.getByRole("button", { name: "Show their week" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Give the person as ‘person:’ or ‘worker:’ followed by their id, with nothing else.",
  );

  // Their own week, named with the other prefix and in capitals, is still theirs.
  await person.fill("worker:6F1C2A3B-0000-4000-8000-000000000009");
  await other.getByRole("button", { name: "Show their week" }).click();
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "That is your own week: nobody decides their own timesheet.",
  );

  await person.fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await expect(
    other.getByText(
      "An approval is final: the week is then locked, and nobody can send it back.",
    ),
  ).toBeVisible();
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("status")).toHaveText(
    `Week 2026-W46 of ${ALICE} is approved.`,
  );
  await expect(other.getByTestId("other-status")).toHaveText(
    "✓ Approved: final, and locked against new time",
  );
  await expect(
    other.getByText(
      "An approved week is final: no more time can be recorded in it.",
    ),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/timesheets/decide",
      body: { person: ALICE, week: "2026-W46", decision: "approved" },
    },
  ]);
});

test("refusals to read or decide another's week say the catalogue's sentence", async ({
  page,
}) => {
  await stub(page, {
    "POST /api/timesheets/decide": 403,
  });
  await open(page);
  const other = page.getByTestId("other");
  await other
    .getByLabel("Person: ‘person:’ or ‘worker:’ and their id")
    .fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "You may not decide this week: it needs the timesheet approver right, and nobody decides their own week.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { "POST /api/timesheets/decide": 409 });
  await open(page);
  await other
    .getByLabel("Person: ‘person:’ or ‘worker:’ and their id")
    .fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await other.getByRole("button", { name: "Record the decision" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "This week is not waiting for a decision any more; it has been read again.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page);
  await page.route(/\/api\/proxy\/api\/timesheets\?.*person=/, (route) =>
    route.fulfill({
      status: 403,
      json: {
        error: "forbidden",
        description: "service text that must not be shown",
      },
    }),
  );
  await open(page);
  await other
    .getByLabel("Person: ‘person:’ or ‘worker:’ and their id")
    .fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await expect(other.getByRole("alert")).toHaveText(
    "Only the person themselves or a timesheet approver may read that week.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("a sign-in identity with no person reference says so", async ({
  page,
}) => {
  await page.route(/\/api\/proxy\/api\/timesheets/, (route) =>
    route.fulfill({
      status: 422,
      json: {
        error: "unprocessable",
        description: "sub_not_addressable: service text",
      },
    }),
  );
  await open(page);
  await expect(page.getByTestId("mine").getByRole("alert")).toHaveText(
    "Your sign-in identity has no person reference, so there is no timesheet to show for it.",
  );
  await expect(page.getByText(/service text/)).toHaveCount(0);
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Fy nhaflen amser" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Eich wythnos 2026-W46" }),
  ).toBeVisible();
  await expect(page.getByTestId("mine-status")).toHaveText(
    "○ Ar agor: heb ei hanfon i'w chymeradwyo eto",
  );
  await expect(
    page
      .getByRole("row", { name: /Website replatform/ })
      .getByRole("cell")
      .nth(7),
  ).toHaveText("2 awr 15 munud");
  await expect(
    page.getByRole("button", { name: "Anfon yr wythnos hon i'w chymeradwyo" }),
  ).toBeVisible();
});

test("passes the accessibility audit, with someone else's week open", async ({
  page,
}) => {
  await stub(page);
  await open(page);
  const other = page.getByTestId("other");
  await other
    .getByLabel("Person: ‘person:’ or ‘worker:’ and their id")
    .fill(ALICE);
  await other.getByRole("button", { name: "Show their week" }).click();
  await expect(
    other.getByRole("button", { name: "Record the decision" }),
  ).toBeVisible();
  await expect(page.getByTestId("mine").getByRole("table")).toBeVisible();
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});
