// NC-1 slice 4 (PL-5, LT-6, RM-11): the caller's own notifications, in a real browser over a stubbed service. The
// inbox shows each line (a digest says how many it stands for), marks a line read (every notification it stands for)
// and asks for unread only; a choice is switched off per channel, a daily digest is chosen, and a kind goes back to
// the default; a destination is added (refused on the page when the service would refuse it, and the secret leaves
// the box once saved), its deliveries are read (never the service's error text), and it is removed. A 403, 404 or
// 422 says the catalogue's sentence, not the service's text. Welsh reads. The page passes axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const PLAN = "11111111-1111-4111-8111-111111111111";
const SECRET = "https://hooks.slack.com/services/T000/B000/SECRETSECRET1234";

interface Sent {
  method: string;
  path: string;
  body: unknown;
}

/** The stub; `refuse` makes the next call of that method and path answer with that status. */
async function stub(page: Page, refuse: Partial<Record<string, number>> = {}) {
  const sent: Sent[] = [];
  const read = new Set<string>();
  let prefs = [
    {
      pid: "pr1",
      kind: "comment",
      channel: "email",
      enabled: false,
      digest: "instant",
    },
  ];
  let sinks = [
    {
      pid: "s1",
      kind: "slack",
      enabled: true,
      channel: "chat",
      destination: "https://hooks.slack.com/…1234",
    },
  ];
  const refusal = (key: string) => {
    const status = refuse[key];
    if (!status) return null;
    delete refuse[key];
    return status;
  };
  await page.route(/\/api\/proxy\/api\/(me|notifications)\//, async (route) => {
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
    if (path === "/api/me/inbox") {
      const items = [
        {
          kind: "task_assigned",
          subject_kind: "plan",
          subject_pid: PLAN,
          message: "You were assigned: Ship it",
          count: 1,
          pids: ["n1"],
          unread: !read.has("n1"),
          latest_at: "2026-11-10T09:00:00Z",
          digest: false,
        },
        {
          kind: "comment",
          subject_kind: "task",
          subject_pid: "t1",
          message: "Ana replied",
          count: 3,
          pids: ["n2", "n3", "n4"],
          unread: false,
          latest_at: "2026-11-09T17:00:00Z",
          digest: true,
        },
      ];
      const shown =
        url.searchParams.get("unread") === "true"
          ? items.filter((i) => i.unread)
          : items;
      return route.fulfill({
        json: {
          tz: url.searchParams.get("tz"),
          items: shown,
          unread_count: items.filter((i) => i.unread).length,
        },
      });
    }
    const mark = /^\/api\/notifications\/(n\d)\/read$/.exec(path);
    if (mark && method === "POST") {
      read.add(mark[1]!);
      return route.fulfill({ json: { pid: mark[1] } });
    }
    if (path === "/api/me/notification-prefs" && method === "GET")
      return route.fulfill({
        json: {
          prefs,
          known_kinds: ["comment", "task_assigned"],
          channels: ["in_app", "webhook", "chat", "email"],
          digests: ["instant", "daily"],
          default: { enabled: true, digest: "instant" },
        },
      });
    if (path === "/api/me/notification-prefs" && method === "PUT") {
      const b = body as {
        kind: string;
        channel: string;
        enabled: boolean;
        digest: string;
      };
      prefs = [
        ...prefs.filter((p) => !(p.kind === b.kind && p.channel === b.channel)),
        { pid: `pr-${b.kind}-${b.channel}`, ...b },
      ];
      return route.fulfill({ json: prefs.at(-1) });
    }
    if (path === "/api/me/notification-prefs/reset" && method === "POST") {
      const b = body as { kind: string; channel: string };
      prefs = prefs.filter(
        (p) => !(p.kind === b.kind && p.channel === b.channel),
      );
      return route.fulfill({ json: {} });
    }
    if (path === "/api/me/notification-sinks" && method === "GET")
      return route.fulfill({ json: sinks });
    if (path === "/api/me/notification-sinks" && method === "POST") {
      const b = body as { kind: string; destination: string };
      const domain = b.destination.split("@")[1] ?? "";
      const added = {
        pid: "s2",
        kind: b.kind,
        enabled: true,
        channel: b.kind === "email" ? "email" : "chat",
        destination: `***@${domain}`,
      };
      sinks = [...sinks, added];
      return route.fulfill({ json: added });
    }
    const del = /^\/api\/me\/notification-sinks\/(s\d)\/delete$/.exec(path);
    if (del && method === "POST") {
      sinks = sinks.filter((s) => s.pid !== del[1]);
      return route.fulfill({ json: {} });
    }
    if (/^\/api\/me\/notification-sinks\/s\d\/deliveries$/.test(path))
      return route.fulfill({
        json: [
          {
            notification_pid: "n1",
            status: "sent",
            attempts: 1,
            last_error: null,
            next_attempt_at: null,
            delivered_at: "2026-11-10T09:00:05Z",
          },
          {
            notification_pid: "n2",
            status: "pending",
            attempts: 2,
            last_error: `error sending request for url (${SECRET})`,
            next_attempt_at: "2026-11-10T09:30:00Z",
            delivered_at: null,
          },
        ],
      });
    return route.fulfill({
      status: 404,
      json: { error: "unhandled in stub" },
    });
  });
  return sent;
}

async function open(page: Page, locale = "en-001") {
  await page.goto(`/${locale}/me/notifications`);
  await expect(page.locator("h1")).toBeVisible();
}

test("my work links to my notifications", async ({ page }) => {
  await page.route(/\/api\/proxy\/api\/me\/tasks/, (route) =>
    route.fulfill({ status: 404, json: { error: "unhandled in stub" } }),
  );
  await page.goto("/en-001/me");
  await page.getByRole("link", { name: "My notifications" }).click();
  await expect(page).toHaveURL(/\/en-001\/me\/notifications$/);
});

test("the inbox shows each line, marks one read, and asks for unread only", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const inbox = page.getByTestId("inbox");
  await expect(inbox.getByText("1 unread notification.")).toBeVisible();
  await expect(inbox.getByText("You were assigned: Ship it")).toBeVisible();
  await expect(inbox.getByText("A task was given to you")).toBeVisible();
  await expect(
    inbox.getByText(/daily digest of 3 notifications/),
  ).toBeVisible();
  await expect(
    inbox.getByRole("link", { name: "Open the plan" }),
  ).toHaveAttribute("href", `/en-001/plans/${PLAN}`);

  await inbox
    .getByRole("button", { name: "Mark read: You were assigned: Ship it" })
    .click();
  await expect(page.getByRole("status")).toHaveText("Marked read.");
  await expect(inbox.getByText("0 unread notifications.")).toBeVisible();
  expect(sent).toEqual([
    { method: "POST", path: "/api/notifications/n1/read", body: {} },
  ]);

  await inbox.getByLabel("Show only unread").check();
  await expect(inbox.getByText("Nothing unread.")).toBeVisible();
});

test("a choice is switched off per channel, a daily digest chosen, and a kind put back", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const assigned = page.getByRole("group", { name: "A task was given to you" });
  await assigned.getByLabel("In chat").uncheck();
  await expect(page.getByRole("status")).toHaveText("Your choice was saved.");
  await expect(assigned.getByLabel("In chat")).not.toBeChecked();
  await assigned.getByLabel("Daily digest in the inbox").check();
  await expect(assigned.getByLabel("Daily digest in the inbox")).toBeChecked();

  const comments = page.getByRole("group", { name: "Comments" });
  await expect(comments.getByLabel("By email")).not.toBeChecked();
  await comments
    .getByRole("button", { name: "Back to the default: Comments" })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Back to the default: on, as they come.",
  );
  await expect(comments.getByLabel("By email")).toBeChecked();
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/me/notification-prefs",
      body: {
        kind: "task_assigned",
        channel: "chat",
        enabled: false,
        digest: "instant",
      },
    },
    {
      method: "PUT",
      path: "/api/me/notification-prefs",
      body: {
        kind: "task_assigned",
        channel: "in_app",
        enabled: true,
        digest: "daily",
      },
    },
    {
      method: "POST",
      path: "/api/me/notification-prefs/reset",
      body: { kind: "comment", channel: "email" },
    },
  ]);
});

test("a destination is refused on the page, added, its deliveries read, and removed", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const sinks = page.getByTestId("sinks");
  await expect(sinks.getByText("https://hooks.slack.com/…1234")).toBeVisible();
  await expect(
    sinks.locator("li").getByText("Slack chat", { exact: true }),
  ).toBeVisible();

  const add = page.getByRole("button", { name: "Add the destination" });
  await page.getByLabel("Kind").selectOption("webhook");
  const address = page.getByLabel(/^Web address/);
  await address.fill("http://hooks.example.com/x");
  await add.click();
  await expect(page.getByRole("alert")).toHaveText(
    "The web address must start with https://.",
  );
  await address.fill("https://169.254.169.254/latest");
  await add.click();
  await expect(page.getByRole("alert")).toHaveText(
    "The web address must be a public host, not a local, private or internal one.",
  );
  await page.getByLabel("Kind").selectOption("telegram");
  await address.fill("https://api.telegram.org/botX/sendMessage");
  await page.getByLabel(/^Telegram chat/).fill("");
  await add.click();
  await expect(page.getByRole("alert")).toHaveText(
    "A Telegram destination needs the chat it posts to.",
  );
  expect(sent).toEqual([]);

  await page.getByLabel("Kind").selectOption("email");
  await page.getByLabel("Email address").fill("me@example.com");
  await add.click();
  await expect(page.getByRole("status")).toHaveText(
    "The destination was added: ***@example.com.",
  );
  // The secret leaves the box once the service has it.
  await expect(page.getByLabel("Email address")).toHaveValue("");
  await expect(sinks.getByText("***@example.com")).toBeVisible();

  await sinks
    .getByRole("button", {
      name: "Recent deliveries: https://hooks.slack.com/…1234",
    })
    .click();
  const log = page.getByTestId("deliveries");
  await expect(log.getByText("Sent", { exact: true })).toBeVisible();
  await expect(log.getByText("Waiting to send")).toBeVisible();
  await expect(log.getByText("2 attempts")).toBeVisible();
  await expect(page.getByText(/SECRETSECRET/)).toHaveCount(0);

  await sinks
    .getByRole("button", { name: "Remove: https://hooks.slack.com/…1234" })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "The destination was removed.",
  );
  await expect(page.getByTestId("deliveries")).toHaveCount(0);
  await expect(sinks.getByText("https://hooks.slack.com/…1234")).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/me/notification-sinks",
      body: { kind: "email", destination: "me@example.com" },
    },
    {
      method: "POST",
      path: "/api/me/notification-sinks/s1/delete",
      body: null,
    },
  ]);
});

test("refusals say why in words: 404 on a read stamp, 422 on a choice and a destination, 403, 404 on a removal", async ({
  page,
}) => {
  await stub(page, {
    "POST /api/notifications/n1/read": 404,
    "PUT /api/me/notification-prefs": 422,
    "POST /api/me/notification-sinks": 422,
  });
  await open(page);
  await page
    .getByRole("button", { name: "Mark read: You were assigned: Ship it" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That notification has gone, or is not yours; the inbox has been read again.",
  );
  await page
    .getByRole("group", { name: "Comments" })
    .getByLabel("In the inbox", { exact: true })
    .uncheck();
  await expect(page.getByRole("alert")).toHaveText(
    "The service did not accept that choice; your choices have been read again.",
  );
  // Read again, so the box shows what the service holds.
  await expect(
    page
      .getByRole("group", { name: "Comments" })
      .getByLabel("In the inbox", { exact: true }),
  ).toBeChecked();
  await page.getByLabel("Email address").fill("me@example.com");
  await page.getByRole("button", { name: "Add the destination" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "The service did not accept the destination",
  );

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, {
    "POST /api/me/notification-prefs/reset": 403,
    "POST /api/me/notification-sinks/s1/delete": 404,
  });
  await open(page);
  await page
    .getByRole("button", { name: "Back to the default: Comments" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "You may not change these notification settings.",
  );
  await page
    .getByRole("button", { name: "Remove: https://hooks.slack.com/…1234" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That destination had already been removed; your destinations have been read again.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("a sign-in identity the service cannot address says so in each section", async ({
  page,
}) => {
  await page.route(/\/api\/proxy\/api\/me\//, (route) =>
    route.fulfill({
      status: 422,
      json: {
        error: "unprocessable",
        description: "sub_not_addressable: service text",
      },
    }),
  );
  await open(page);
  await expect(
    page.getByText(
      "Your sign-in identity is not one that notifications can be addressed to, so there is nothing to show here.",
    ),
  ).toHaveCount(3);
  await expect(page.getByText(/service text/)).toHaveCount(0);
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(
    page.getByRole("heading", { name: "Fy hysbysiadau" }),
  ).toBeVisible();
  await expect(page.getByRole("heading", { name: "Mewnflwch" })).toBeVisible();
  await expect(
    page
      .getByRole("group", { name: "Rhoddwyd tasg i chi" })
      .getByLabel("Mewn sgwrs"),
  ).toBeChecked();
  await expect(
    page.getByRole("button", { name: "Ychwanegu'r gyrchfan" }),
  ).toBeVisible();
});

test("passes the accessibility audit, with the deliveries open", async ({
  page,
}) => {
  await stub(page);
  await open(page);
  await page
    .getByRole("button", {
      name: "Recent deliveries: https://hooks.slack.com/…1234",
    })
    .click();
  await expect(page.getByTestId("deliveries")).toBeVisible();
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
