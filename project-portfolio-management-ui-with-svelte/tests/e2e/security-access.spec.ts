// NC-1 slice 7 (RG-9, RG-10): the access review and emergency (break-glass) access, in a real browser over a stubbed
// service. The review is read only when asked (each read is audited), shows the grants that stand with the ones due
// for review in words and a shape, and links to the service's own spreadsheet. Emergency access says before the form
// that every use is recorded and audited and that the person named is told; the page refuses what the service would
// (a thin reason, a time outside 1 to 240 minutes, a person to tell who is not a reference, no acknowledgement) and
// sends nothing; an opened grant's end is shown in the browser's time zone, which is named; a live grant is ended
// early. A 403, 404, 409, 422 or 429 says the catalogue's sentence, not the service's text. Welsh reads. The page
// passes axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const ME = "6f1c2a3b-0000-4000-8000-00000000000c";
const ERIN = "person:6f1c2a3b-0000-4000-8000-00000000000e";
const PLAN_A = "11111111-1111-4111-8111-111111111111";
const PLAN_B = "22222222-2222-4222-8222-222222222222";
const PLAN_SECRET = "33333333-3333-4333-8333-333333333333";
const REASON =
  "Incident 4412: the plan owner is unreachable and the release is blocked";

// Auckland is 13 hours ahead of UTC in November, so 22:45 UTC on 10 November is 11:45 on 11 November there.
test.use({ timezoneId: "Pacific/Auckland" });

interface Sent {
  method: string;
  path: string;
  body: unknown;
}

interface Grant {
  pid: string;
  plan_pid: string;
  sub: string;
  reason: string;
  notify_ref: string;
  opened_at: string;
  expires_at: string;
  revoked_at: string | null;
  revoked_by: string | null;
  live: boolean;
}

const EARLIER: Grant = {
  pid: "g-old",
  plan_pid: PLAN_B,
  sub: ME,
  reason: "Incident 4400: the owner was on leave and payroll was due",
  notify_ref: ERIN,
  opened_at: "2026-11-02T01:00:00Z",
  expires_at: "2026-11-02T02:00:00Z",
  revoked_at: null,
  revoked_by: null,
  live: false,
};

const REVIEW = {
  as_of: "2026-11-10",
  older_than_days: 365,
  standing: 2,
  review_due: 1,
  grants: [
    {
      plan_pid: PLAN_A,
      person_ref: "person:6f1c2a3b-0000-4000-8000-00000000000a",
      access: "repo write",
      granted_on: "2025-01-10",
      granted_by: "person:admin",
      note: "=SUM(1)",
      age_calendar_days: 669,
      review_due: true,
    },
    {
      plan_pid: PLAN_A,
      person_ref: "person:6f1c2a3b-0000-4000-8000-00000000000b",
      access: "wiki editor",
      granted_on: "2026-10-01",
      granted_by: null,
      note: null,
      age_calendar_days: 40,
      review_due: false,
    },
  ],
  note: "service text that must not be shown",
};

/** The stub; `refuse` makes the next call of that method and path answer with that status. */
async function stub(
  page: Page,
  refuse: Partial<Record<string, number>> = {},
  initial: Grant[] = [EARLIER],
) {
  const sent: Sent[] = [];
  const reads: string[] = [];
  const grants = initial.map((g) => ({ ...g }));
  const refusal = (key: string) => {
    const status = refuse[key];
    if (!status) return null;
    delete refuse[key];
    return status;
  };
  await page.route(
    /\/api\/proxy\/api\/(access-review|break-glass|plans\/)/,
    async (route) => {
      const req = route.request();
      const url = new URL(req.url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      const body = req.postDataJSON?.() ?? null;
      if (method !== "GET") sent.push({ method, path, body });
      else reads.push(`${path}${url.search}`);
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
      if (path === `/api/plans/${PLAN_SECRET}`)
        return route.fulfill({
          json: { pid: PLAN_SECRET, kind: "Project", name: "Payroll cutover" },
        });
      if (path === "/api/access-review" && method === "GET")
        return route.fulfill({ json: REVIEW });
      if (path === "/api/break-glass" && method === "GET")
        return route.fulfill({
          json: { grants, live: grants.filter((g) => g.live).length },
        });
      if (path === "/api/break-glass" && method === "POST") {
        const b = body as { plan_pid: string; minutes: number };
        const g: Grant = {
          pid: "g-new",
          plan_pid: b.plan_pid,
          sub: ME,
          reason: REASON,
          notify_ref: ERIN,
          opened_at: "2026-11-10T22:00:00Z",
          expires_at: "2026-11-10T22:45:00Z",
          revoked_at: null,
          revoked_by: null,
          live: true,
        };
        grants.unshift(g);
        return route.fulfill({
          json: {
            pid: g.pid,
            plan_pid: g.plan_pid,
            expires_at: g.expires_at,
            notified: true,
          },
        });
      }
      const revoke = /^\/api\/break-glass\/([^/]+)\/revoke$/.exec(path);
      if (revoke && method === "POST") {
        const g = grants.find((x) => x.pid === revoke[1]);
        if (!g) return route.fulfill({ status: 404, json: { error: "nf" } });
        g.revoked_at = "2026-11-10T22:10:00Z";
        g.revoked_by = ME;
        g.live = false;
        return route.fulfill({ json: g });
      }
      return route.fulfill({
        status: 404,
        json: { error: "unhandled in stub" },
      });
    },
  );
  return { sent, reads };
}

async function open(page: Page, locale = "en-001") {
  await page.goto(`/${locale}/security/access`);
  await expect(page.locator("h1")).toBeVisible();
}

async function fill(page: Page, over: Partial<Record<string, string>> = {}) {
  const bg = page.getByTestId("break-glass");
  await bg.getByLabel("Private plan's id").fill(over.plan ?? PLAN_SECRET);
  await bg
    .getByLabel("Why you need access (needed, at least 20 characters)")
    .fill(over.reason ?? REASON);
  await bg
    .getByLabel("For how long, in minutes (1 to 240)")
    .fill(over.minutes ?? "45");
  await bg
    .getByLabel("Person to tell: ‘person:’ or ‘worker:’ and their id (not you)")
    .fill(over.notify ?? ERIN);
}

const ACK =
  "I understand that this is recorded and audited, and that the person I name will be told.";

test("the security page links to the access review and emergency access", async ({
  page,
}) => {
  await page.route(/\/api\/proxy\/api\//, (route) =>
    route.fulfill({ status: 404, json: { error: "unhandled in stub" } }),
  );
  await page.goto("/en-001/security");
  await page
    .getByRole("link", { name: "Access review and emergency access" })
    .click();
  await expect(page).toHaveURL(/\/en-001\/security\/access$/);
});

test("the access review is read only when asked, shows what is due, and links to the spreadsheet", async ({
  page,
}) => {
  const { reads } = await stub(page);
  await open(page);
  const review = page.getByTestId("review");
  await expect(
    review.getByText(/each reading is recorded in the audit trail/),
  ).toBeVisible();
  // Opening the page reads the grants list, never the audited review.
  await expect.poll(() => reads.includes("/api/break-glass")).toBe(true);
  expect(reads.filter((r) => r.startsWith("/api/access-review"))).toEqual([]);

  // A filter the service would refuse is refused on the page.
  await review
    .getByLabel("Plan id (leave blank for every plan you can read)")
    .fill("Apollo");
  await review.getByRole("button", { name: "Show the review" }).click();
  await expect(review.getByRole("alert")).toHaveText(
    "That is not a plan id: paste the plan's id, or leave the box blank for every plan.",
  );
  await review
    .getByLabel("Plan id (leave blank for every plan you can read)")
    .fill(PLAN_A);
  await review
    .getByLabel("Due for review after this many calendar days")
    .fill("-1");
  await review.getByRole("button", { name: "Show the review" }).click();
  await expect(review.getByRole("alert")).toHaveText(
    "Write the number of calendar days as a whole number, 0 or more.",
  );
  expect(reads.filter((r) => r.startsWith("/api/access-review"))).toEqual([]);

  await review
    .getByLabel("Due for review after this many calendar days")
    .fill("365");
  await review.getByRole("button", { name: "Show the review" }).click();
  await expect(review.getByTestId("review-totals")).toHaveText(
    "As of 10 Nov 2026. Grants that stand: 2. Due for review, at 365 calendar days old or more: 1.",
  );
  expect(reads.filter((r) => r.startsWith("/api/access-review"))).toEqual([
    `/api/access-review?plan=${PLAN_A}&older_than_days=365`,
  ]);
  await expect(
    review.getByRole("link", {
      name: "Download the review as a spreadsheet, with a column for your keep or revoke decisions",
    }),
  ).toHaveAttribute(
    "href",
    new RegExp(
      `/api/proxy/api/access-review\\?plan=${PLAN_A}&older_than_days=365&format=csv$`,
    ),
  );
  const table = review.getByRole("table", {
    name: "Access that stands today, oldest first",
  });
  const old = table.getByRole("row", { name: /00000000000a/ });
  await expect(old).toContainText("repo write");
  await expect(old).toContainText("Website replatform");
  await expect(old).toContainText("10 Jan 2025");
  await expect(old).toContainText("⚑ Due for review");
  await expect(old).toContainText("=SUM(1)");
  await expect(table.getByRole("row", { name: /00000000000b/ })).toContainText(
    "Not yet due",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("the access review is refused in words to someone who is not an administrator", async ({
  page,
}) => {
  await stub(page, { "GET /api/access-review": 403 });
  await open(page);
  const review = page.getByTestId("review");
  await review.getByRole("button", { name: "Show the review" }).click();
  await expect(review.getByRole("alert")).toHaveText(
    "The access review is for administrators only.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("emergency access says it is audited before the form, and the page refuses what the service would", async ({
  page,
}) => {
  const { sent } = await stub(page);
  await open(page);
  const bg = page.getByTestId("break-glass");
  const note = bg.getByRole("note");
  await expect(note).toContainText(
    "Every use of emergency access is recorded and audited.",
  );
  await expect(note).toContainText(
    "The person you name is told at once. If they cannot be told, the plan is not opened.",
  );
  await expect(note).toContainText(
    "It lets you see the plan. What you may change is still decided by your own rights.",
  );
  // The note comes before the form.
  const noteBox = await note.boundingBox();
  const formBox = await bg.getByLabel("Private plan's id").boundingBox();
  expect(noteBox!.y).toBeLessThan(formBox!.y);
  // Not the page's primary action.
  await expect(
    bg.getByRole("button", { name: "Open the plan for emergency access" }),
  ).not.toHaveClass(/primary/);

  // Everything wrong at once, and nothing sent.
  await fill(page, {
    plan: "Payroll",
    reason: "because",
    minutes: "241",
    notify: "erin",
  });
  await bg
    .getByRole("button", { name: "Open the plan for emergency access" })
    .click();
  await expect(bg.getByTestId("bg-problems").getByRole("listitem")).toHaveText([
    "Give the private plan's id.",
    "Say why you need access, in at least 20 characters.",
    "Give how long in whole minutes, from 1 to 240.",
    "Name the person to tell as ‘person:’ or ‘worker:’ followed by their id, with nothing else.",
    "Tick the box to say you understand that this is recorded and audited and that the person is told.",
  ]);
  // Twenty spaces are not a reason, and the count says so.
  await fill(page, { reason: " ".repeat(30) });
  await expect(
    bg.getByText("0 characters; at least 20 and at most 1,000."),
  ).toBeVisible();
  await bg.getByLabel(ACK).check();
  await bg
    .getByRole("button", { name: "Open the plan for emergency access" })
    .click();
  await expect(bg.getByTestId("bg-problems").getByRole("listitem")).toHaveText([
    "Say why you need access, in at least 20 characters.",
  ]);
  expect(sent).toEqual([]);
});

test("emergency access opens a plan until a time in the reader's zone, and is ended early", async ({
  page,
}) => {
  const { sent } = await stub(page);
  await open(page);
  const bg = page.getByTestId("break-glass");
  await expect(
    bg.getByText(/Times are in your time zone, Pacific\/Auckland\./),
  ).toBeVisible();
  const table = bg.getByRole("table", {
    name: "Emergency access grants, newest first",
  });
  // An ended grant: its plan is unnamed (it can no longer be read), it ran out, and it cannot be ended.
  const old = table.getByRole("row", { name: /22222222… \(name not shown\)/ });
  await expect(old).toContainText("○ Ran out");
  await expect(old).toContainText("2 Nov 2026, 3:00 pm");
  await expect(old.getByRole("button")).toHaveCount(0);

  await fill(page);
  await bg.getByLabel(ACK).check();
  await bg
    .getByRole("button", { name: "Open the plan for emergency access" })
    .click();
  await expect(bg.getByTestId("bg-opened")).toContainText(
    `The plan is open to you until 11 Nov 2026, 11:45 am (your time zone, Pacific/Auckland). ${ERIN} has been told.`,
  );
  await expect(
    bg.getByRole("link", { name: "Go to the plan" }),
  ).toHaveAttribute("href", `/en-001/plans/${PLAN_SECRET}`);
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/break-glass",
      body: {
        plan_pid: PLAN_SECRET,
        reason: REASON,
        minutes: 45,
        notify_ref: ERIN,
      },
    },
  ]);
  // The reason and the acknowledgement are cleared, so a second use asks again.
  await expect(bg.getByLabel(ACK)).not.toBeChecked();

  const live = table.getByRole("row", { name: /Payroll cutover/ });
  await expect(live).toContainText("● In force");
  await expect(live).toContainText("11 Nov 2026, 11:45 am");
  await expect(live).toContainText(REASON);
  await live
    .getByRole("button", {
      name: "End now: emergency access to Payroll cutover",
    })
    .click();
  await expect(
    bg.getByText("Emergency access to Payroll cutover has ended."),
  ).toBeVisible();
  await expect(live).toContainText("■ Ended early");
  await expect(live).toContainText(`11 Nov 2026, 11:10 am, ended by ${ME}`);
  expect(sent[1]).toEqual({
    method: "POST",
    path: "/api/break-glass/g-new/revoke",
    body: null,
  });
  await expect(bg.getByTestId("bg-opened")).toHaveText("");
});

test("the service's refusals of emergency access are catalogue sentences", async ({
  page,
}) => {
  const cases: [number, string][] = [
    [
      403,
      "You may not use emergency access: it needs the break-glass eligibility or administrator access.",
    ],
    [404, "There is no plan with that id."],
    [
      409,
      "You already hold emergency access to this plan; the list below has been read again.",
    ],
    [
      422,
      "The plan was not opened. Either it is not private, so there is nothing to open, or the person to tell is you, or that person cannot be told. No access was given.",
    ],
    [
      429,
      "You have opened the most emergency access allowed in a day (five, counted over the last 24 hours).",
    ],
  ];
  for (const [status, words] of cases) {
    await page.unrouteAll({ behavior: "wait" });
    await stub(page, { "POST /api/break-glass": status });
    await open(page);
    const bg = page.getByTestId("break-glass");
    await fill(page);
    await bg.getByLabel(ACK).check();
    await bg
      .getByRole("button", { name: "Open the plan for emergency access" })
      .click();
    await expect(bg.getByRole("alert"), String(status)).toHaveText(words);
    await expect(bg.getByTestId("bg-opened")).toHaveText("");
    await expect(
      page.getByText("service text that must not be shown"),
    ).toHaveCount(0);
  }

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { "POST /api/break-glass/g-live/revoke": 404 }, [
    { ...EARLIER, pid: "g-live", plan_pid: PLAN_SECRET, live: true },
  ]);
  await open(page);
  await page
    .getByRole("button", {
      name: "End now: emergency access to Payroll cutover",
    })
    .click();
  await expect(page.getByTestId("break-glass").getByRole("alert")).toHaveText(
    "That grant was not found, or it is not yours to end.",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { "GET /api/break-glass": 401 });
  await open(page);
  await expect(page.getByTestId("break-glass").getByRole("alert")).toHaveText(
    "Sign in first.",
  );
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Adolygiad mynediad a mynediad brys",
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", {
      name: "Mynediad brys at gynllun preifat (torri'r gwydr)",
    }),
  ).toBeVisible();
  await expect(page.getByRole("note")).toContainText(
    "Caiff pob defnydd o fynediad brys ei gofnodi a'i archwilio.",
  );
  await expect(
    page.getByRole("button", { name: "Agor y cynllun ar gyfer mynediad brys" }),
  ).toBeVisible();
  await expect(page.getByRole("row", { name: /22222222/ })).toContainText(
    "Daeth yr amser i ben",
  );
});

test("passes the accessibility audit, with the review shown and a live grant", async ({
  page,
}) => {
  await stub(page, {}, [
    { ...EARLIER, pid: "g-live", plan_pid: PLAN_SECRET, live: true },
    EARLIER,
  ]);
  await open(page);
  await page
    .getByTestId("review")
    .getByRole("button", { name: "Show the review" })
    .click();
  await expect(page.getByTestId("review-totals")).toBeVisible();
  await expect(
    page.getByRole("button", {
      name: "End now: emergency access to Payroll cutover",
    }),
  ).toBeVisible();
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
