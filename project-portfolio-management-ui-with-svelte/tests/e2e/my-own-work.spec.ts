// NC-1 slice 12 part (h): a person's own work on /me, in a real browser over a stubbed service. The day's plan (PL-4):
// read for today in the browser's zone, its slots with a clash in words and a shape, what is due, a slot left out
// counted; a slot's rules said first and checked before anything is sent; a slot sent in UTC; removed after a question
// (Cancel sends nothing); each refusal in the catalogue's words. The calendar feed (LT-7): made after a question, the
// addresses shown once through this site's proxy, stopped after a question. What I follow (RM-3): a plan named, the
// rest by kind and reference, stopped after a question. The work over time (PJ-5): checked first, defaults not sent,
// two tables, a refusal. Every stub that would carry the service's English carries "service text", which never shows.
// Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

// Wednesday 18 November 2026, 10:00 in London (GMT, so local time is UTC).
const NOW = new Date("2026-11-18T10:00:00Z");
const TASK_REF = "6f1c2a3b-0000-4000-8000-0000000000aa";

test.use({ timezoneId: "Europe/London" });

interface Sent {
  method: string;
  path: string;
  body?: unknown;
}

const task = (pid: string, title: string) => ({
  pid,
  title,
  status: "todo",
  due_date: "2026-11-18",
  points: null,
  plan_pid: PID,
  plan_name: "Website replatform",
  schedule: null,
});

async function stub(
  page: Page,
  refuse: Partial<Record<string, number>> = {},
): Promise<Sent[]> {
  const sent: Sent[] = [];
  let slots = [
    {
      pid: "s1",
      task: {
        pid: "t1",
        title: "Review the plan",
        status: "todo",
        plan_pid: PID,
        plan_name: "Website replatform",
      },
      starts_at: "2026-11-18T09:00:00Z",
      ends_at: "2026-11-18T10:30:00Z",
      starts_local: "service text",
      ends_local: "service text",
    },
    {
      pid: "s2",
      task: {
        pid: "t2",
        title: "Write the brief",
        status: "todo",
        plan_pid: PID,
        plan_name: "Website replatform",
      },
      starts_at: "2026-11-18T10:00:00Z",
      ends_at: "2026-11-18T11:00:00Z",
      starts_local: "service text",
      ends_local: "service text",
    },
  ];
  let watching = [
    { subject_kind: "plan", subject_pid: PID },
    { subject_kind: "task", subject_pid: TASK_REF },
    {
      subject_kind: "comment",
      subject_pid: "77777777-0000-4000-8000-000000000001",
    },
  ];
  await stubPlansApi(page);
  await page.route(
    /\/api\/proxy\/api\/(me\/|watchers|work-profile)/,
    async (route) => {
      const req = route.request();
      const url = new URL(req.url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      const status = refuse[`${method} ${path}`];
      if (method === "GET" && path !== "/api/me/tasks")
        sent.push({ method, path: `${path}${url.search}` });
      if (method !== "GET")
        sent.push({ method, path, body: req.postDataJSON?.() ?? undefined });
      if (status) {
        delete refuse[`${method} ${path}`];
        return route.fulfill({
          status,
          json: { code: "unprocessable", detail: "service text" },
        });
      }
      if (path === "/api/me/tasks")
        return route.fulfill({
          json: {
            tz: "Europe/London",
            as_of: NOW.toISOString(),
            today: "2026-11-18",
            buckets: {
              overdue: [],
              today: [
                task("t1", "Review the plan"),
                task("t2", "Write the brief"),
              ],
              this_week: [],
              later: [],
              undated: [],
            },
            counts: {
              overdue: 0,
              today: 2,
              this_week: 0,
              later: 0,
              undated: 0,
            },
            total: 2,
            omitted_unreadable: 0,
            truncated: false,
            reason: null,
          },
        });
      if (path === "/api/me/plan") {
        const day = url.searchParams.get("day");
        const today = day === "2026-11-18";
        return route.fulfill({
          json: {
            day,
            tz: url.searchParams.get("tz"),
            window: { from: "", to: "", hours: 24 },
            slots: today ? slots : [],
            clashes: today && slots.length === 2 ? [["s1", "s2"]] : [],
            due_on_day: today
              ? [{ pid: "t1", title: "Review the plan", plan_pid: PID }]
              : [],
            omitted_unreadable: today ? 1 : 0,
          },
        });
      }
      if (path === "/api/me/planned-slots")
        return route.fulfill({
          json: { pid: "s3", task_pid: "t1", starts_at: "", ends_at: "" },
        });
      if (/^\/api\/me\/planned-slots\/s\d\/delete$/.test(path)) {
        slots = slots.filter((s) => !path.includes(`/${s.pid}/`));
        return route.fulfill({ json: {} });
      }
      if (path === "/api/me/feed-token")
        return route.fulfill({
          json: {
            token: "tok123",
            calendar_url: "/api/feeds/calendar.ics?token=tok123",
            atom_url: "/api/feeds/me.atom?token=tok123",
            note: "service text",
          },
        });
      if (path === "/api/me/feed-token/revoke")
        return route.fulfill({ json: {} });
      if (path === "/api/me/watching")
        return route.fulfill({
          json: { count: watching.length, items: watching },
        });
      if (path === "/api/watchers/unwatch") {
        const b = req.postDataJSON() as { subject_pid: string };
        watching = watching.filter((w) => w.subject_pid !== b.subject_pid);
        return route.fulfill({ json: {} });
      }
      if (path === "/api/work-profile")
        return route.fulfill({
          json: {
            from: "2026-11-16",
            to: "2026-11-29",
            period: "week",
            plans: [
              {
                plan_pid: PID,
                name: "Website replatform",
                work_minutes: 600,
                unplaced_work_minutes: 60,
                buckets: [],
              },
            ],
            totals: {
              buckets: [
                { from: "2026-11-16", to: "2026-11-22", minutes: 300 },
                { from: "2026-11-23", to: "2026-11-29", minutes: 240 },
              ],
              work_minutes: 600,
              unplaced_work_minutes: 60,
            },
            note: "service text",
          },
        });
      return route.fulfill({ status: 404, json: { detail: "service text" } });
    },
  );
  return sent;
}

async function open(page: Page, locale = "en-001") {
  await page.clock.setFixedTime(NOW);
  await page.goto(`/${locale}/me`);
  await settle(page);
}

test("the day's plan: read for today in my zone, a clash in words, what is due, and a slot left out counted", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const p = page.getByTestId("planner");
  await expect(p).toContainText(
    "A slot is yours alone and never changes the task's due date; it ends after it starts and lasts at most 12 hours.",
  );
  await expect(p).toContainText("Times are in your time zone, Europe/London.");
  await expect(
    p.getByRole("heading", {
      name: /^Planned on Wednesday,? 18 November 2026$/,
    }),
  ).toBeVisible();
  const slots = p.getByTestId("slots");
  await expect(slots.getByRole("listitem")).toHaveCount(2);
  await expect(slots.getByRole("listitem").first()).toContainText(
    /18 Nov 2026, 9:00 – 10:30 am\s*Review the plan\s*Website replatform\s*⚠ ?Clashes with 1 other slot/,
  );
  await expect(
    slots.getByRole("link", { name: "Write the brief" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/tasks/t2`);
  await expect(p).toContainText(
    "1 slot on a task you can no longer read is not shown.",
  );
  await expect(
    p.getByRole("heading", { name: "Due on this day" }),
  ).toBeVisible();
  expect(sent).toContainEqual({
    method: "GET",
    path: "/api/me/plan?day=2026-11-18&tz=Europe%2FLondon",
  });
  await p.getByRole("button", { name: "Next day" }).click();
  await expect(p).toContainText("Nothing is planned on this day.");
  await expect(p).toContainText("Nothing assigned to you is due on this day.");
  expect(sent).toContainEqual({
    method: "GET",
    path: "/api/me/plan?day=2026-11-19&tz=Europe%2FLondon",
  });
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a slot: its rules checked first, sent in UTC, and removed after a question that Cancel answers with nothing", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const p = page.getByTestId("planner");
  const form = p.getByRole("form", { name: "Plan time for a task" });
  await expect(form.getByLabel("Task")).toBeEnabled();
  await form.getByRole("button", { name: "Plan this time" }).click();
  await expect(p.getByRole("alert")).toHaveText(
    "Choose a task. Give the time it starts. Give the time it ends.",
  );
  await form
    .getByLabel("Task")
    .selectOption({ label: "Review the plan, in Website replatform" });
  await form.getByLabel("Starts").fill("11:00");
  await form.getByLabel("Ends").fill("10:00");
  await form.getByRole("button", { name: "Plan this time" }).click();
  await expect(p.getByRole("alert")).toHaveText(
    "A slot must end after it starts.",
  );
  await form.getByLabel("Ends").fill("23:30");
  await form.getByRole("button", { name: "Plan this time" }).click();
  await expect(p.getByRole("alert")).toHaveText(
    "A slot lasts at most 12 hours.",
  );
  expect(sent.filter((s) => s.method === "POST")).toEqual([]);
  await form.getByLabel("Ends").fill("12:00");
  await form.getByRole("button", { name: "Plan this time" }).click();
  await expect(p.getByRole("status")).toContainText(
    /Planned Review the plan, .*11:00.*12:00/,
  );
  expect(sent.filter((s) => s.method === "POST")).toEqual([
    {
      method: "POST",
      path: "/api/me/planned-slots",
      body: {
        task_pid: "t1",
        starts_at: "2026-11-18T11:00:00Z",
        ends_at: "2026-11-18T12:00:00Z",
      },
    },
  ]);

  await p
    .getByRole("button", { name: /^Remove the slot for Write the brief/ })
    .click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Remove the slot for Write the brief, 18 Nov 2026, 10:00 – 11:00 am? The task itself is not changed.",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent.filter((s) => s.path.endsWith("/delete"))).toEqual([]);
  await p
    .getByRole("button", { name: /^Remove the slot for Write the brief/ })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(p.getByTestId("slots").getByRole("listitem")).toHaveCount(1);
  expect(sent.filter((s) => s.path.endsWith("/delete"))).toEqual([
    { method: "POST", path: "/api/me/planned-slots/s2/delete" },
  ]);
  await expect(p.getByText(/Clashes with/)).toHaveCount(0);
});

test("a refused slot and a slot already gone are the catalogue's sentences, not the service's text", async ({
  page,
}) => {
  await stub(page, {
    "POST /api/me/planned-slots": 422,
    "POST /api/me/planned-slots/s1/delete": 404,
  });
  await open(page);
  const p = page.getByTestId("planner");
  const form = p.getByRole("form", { name: "Plan time for a task" });
  await form
    .getByLabel("Task")
    .selectOption({ label: "Write the brief, in Website replatform" });
  await form.getByLabel("Starts").fill("14:00");
  await form.getByLabel("Ends").fill("15:00");
  await form.getByRole("button", { name: "Plan this time" }).click();
  await expect(p.getByRole("alert")).toHaveText(
    "The slot was not accepted: it must end after it starts and last at most 12 hours, and a person holds at most 2,000 slots.",
  );
  await p
    .getByRole("button", { name: /^Remove the slot for Review the plan/ })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(p.getByRole("alert")).toHaveText(
    "That slot is already gone. The day has been read again.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the calendar feed: made after a question, shown once through this site, and stopped after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const f = page.getByTestId("feed");
  await expect(f).toContainText(
    "Anyone who has an address can read what it shows, so keep it to yourself.",
  );
  await expect(f).toContainText(
    "Making a new one stops any earlier one at once.",
  );
  await f.getByRole("button", { name: "Make a feed address" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Make a new feed address? Any address you made before stops working at once.",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "Cancel" })
    .click();
  expect(sent.filter((s) => s.path.includes("feed-token"))).toEqual([]);
  await f.getByRole("button", { name: "Make a feed address" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(f.getByRole("status")).toHaveText(
    "Your new addresses are below. Copy them now: they are not shown again.",
  );
  await expect(f.getByLabel("Calendar address (iCalendar)")).toHaveValue(
    /^http:\/\/localhost:\d+\/api\/proxy\/api\/feeds\/calendar\.ics\?token=tok123$/,
  );
  await expect(f.getByLabel("Notifications feed address (Atom)")).toHaveValue(
    /\/api\/proxy\/api\/feeds\/me\.atom\?token=tok123$/,
  );
  await f.getByRole("button", { name: "Stop every feed address" }).click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Stop every feed address you have made? A calendar app using one stops updating at once.",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(f.getByRole("status")).toHaveText(
    "Every feed address you made has stopped working.",
  );
  await expect(f.getByLabel("Calendar address (iCalendar)")).toHaveCount(0);
  expect(sent.filter((s) => s.path.includes("feed-token"))).toEqual([
    { method: "POST", path: "/api/me/feed-token" },
    { method: "POST", path: "/api/me/feed-token/revoke" },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("what I follow: a plan named, the rest by kind and reference, and stopped after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const w = page.getByTestId("watching");
  await expect(w).toContainText("You follow 3 things.");
  await expect(
    w.getByRole("link", { name: "Plan Website replatform" }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}`);
  await expect(w).toContainText("Task, reference 6f1c2a3b");
  await expect(w).toContainText("Something else, reference 77777777");
  // A kind the page does not know cannot be sent back to stop following it.
  await expect(w.getByRole("button", { name: "Stop following" })).toHaveCount(
    2,
  );
  await w
    .getByRole("button", { name: "Stop following Task, reference 6f1c2a3b" })
    .click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Stop following Task, reference 6f1c2a3b? You will no longer be told when it changes.",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "Cancel" })
    .click();
  expect(sent.filter((s) => s.method === "POST")).toEqual([]);
  await w
    .getByRole("button", { name: "Stop following Task, reference 6f1c2a3b" })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(w.getByRole("status")).toHaveText(
    "You no longer follow Task, reference 6f1c2a3b.",
  );
  await expect(w).toContainText("You follow 2 things.");
  expect(sent.filter((s) => s.method === "POST")).toEqual([
    {
      method: "POST",
      path: "/api/watchers/unwatch",
      body: { subject_kind: "task", subject_pid: TASK_REF },
    },
  ]);
});

test("the work over time: checked first, weeks not sent, two tables, and a refused window in the catalogue's words", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const s = page.getByTestId("work-profile");
  const form = s.getByRole("form", { name: "Show the work over time" });
  await form
    .getByLabel("From (empty for the first placed day)")
    .fill("2026-12-01");
  await form
    .getByLabel("To (empty for the last placed day)")
    .fill("2026-11-01");
  await form.getByRole("button", { name: "Show the work" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "The first day must not be after the last.",
  );
  expect(sent.filter((x) => x.path.startsWith("/api/work-profile"))).toEqual(
    [],
  );
  await form.getByLabel("From (empty for the first placed day)").fill("");
  await form.getByLabel("To (empty for the last placed day)").fill("");
  await form.getByRole("button", { name: "Show the work" }).click();
  await expect(s.getByTestId("work-profile-summary")).toContainText(
    "10 hrs of work, 1 hr of it not placed.",
  );
  await expect(
    s.getByTestId("work-profile-plans").getByRole("row", {
      name: /Website replatform\s+10 hrs\s+1 hr/,
    }),
  ).toBeVisible();
  await expect(
    s.getByTestId("work-profile-periods").getByRole("row"),
  ).toHaveCount(3);
  expect(sent.filter((x) => x.path.startsWith("/api/work-profile"))).toEqual([
    { method: "GET", path: "/api/work-profile" },
  ]);
  await page.unroute(/\/api\/proxy\/api\/(me\/|watchers|work-profile)/);
  await page.route(/\/api\/proxy\/api\/work-profile/, (route) =>
    route.fulfill({
      status: 422,
      json: { code: "unprocessable", detail: "service text" },
    }),
  );
  await form.getByLabel("Periods").selectOption({ label: "Months" });
  await form.getByRole("button", { name: "Show the work" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "That window was not accepted: with no work placed yet, give both dates, and keep within the periods said above.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("my own work reads in Welsh and passes axe with the forms open and a feed address shown", async ({
  page,
}) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Amser wedi'i gynllunio" }),
  ).toBeVisible();
  await expect(page.getByTestId("planner")).toContainText(
    "Yn gwrthdaro ag 1 slot arall",
  );
  const f = page.getByTestId("feed");
  await f.getByRole("button", { name: "Gwneud cyfeiriad ffrwd" }).click();
  await page.getByRole("alertdialog").getByRole("button").last().click();
  await expect(f.getByLabel("Cyfeiriad calendr (iCalendar)")).toBeVisible();
  await expect(page.getByTestId("watching")).toContainText(
    "Rydych yn dilyn 3 peth.",
  );
  await page
    .getByTestId("work-profile")
    .getByRole("button", { name: "Dangos y gwaith" })
    .click();
  await expect(page.getByTestId("work-profile-summary")).toBeVisible();
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
