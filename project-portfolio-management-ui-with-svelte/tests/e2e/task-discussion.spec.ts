// NC-1 slice 5 (LT-4, LT-5, RM-3): following a task, reacting to it, its comments and its files, on the task page, in
// a real browser over a stubbed service. A comment is posted, edited (its earlier version shown) and deleted after a
// question; a blank or over-long one is refused on the page; a 403 and a 404 say the catalogue's sentence. A reaction
// is a toggle, added and taken back through the POST twin. Following and stopping use the POST twins; without a
// sign-in the page says to sign in. A file is sent as itself with its type and its name in the query; a web page, an
// empty file and one over 10 MiB are refused before any call; a 422 and a 503 say why in words; a file is a link to
// the service's download and is never drawn in the page; removing one asks first. Welsh reads. The page passes axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const T1 = "aaaaaaaa-0000-4000-8000-000000000001";
const ME = "0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";
const OTHER = "9f8e7d6c-5b4a-4392-8170-6f5e4d3c2b1a";

const task = {
  pid: T1,
  title: "Write the spec",
  description: null,
  status: "in_progress",
  assignee_ref: null,
  points: null,
  due_date: null,
  deadline: null,
  sprint_pid: null,
  component_pid: null,
  parent_task_pid: null,
  created_at: "2026-11-01T00:00:00Z",
  status_changed_at: "2026-11-02T00:00:00Z",
  done_at: null,
  blocked_days: null,
};

const comment = (
  pid: string,
  author: string | null,
  body: string,
  edited: string | null = null,
) => ({
  pid,
  subject_kind: "task",
  subject_pid: T1,
  author_sub: author,
  body,
  created_at: "2026-11-03T10:00:00Z",
  updated_at: "2026-11-03T10:00:00Z",
  edited_at: edited,
});

interface Sent {
  method: string;
  path: string;
  query: string;
  body: unknown;
  type: string | undefined;
  size: number;
}

interface Options {
  /** Answer the next call of `METHOD path` with this status. */
  refuse?: Record<string, number>;
  /** Answer the follow status with 401, as the service does without a sign-in. */
  signedOut?: boolean;
}

async function stub(page: Page, opts: Options = {}) {
  const refuse = { ...(opts.refuse ?? {}) };
  let comments = [
    comment("c1", OTHER, "Is the scope agreed?"),
    comment(
      "c2",
      ME,
      "Yes, as of Monday.\nSee the notes.",
      "2026-11-04T09:00:00Z",
    ),
  ];
  let counts: Record<string, number> = {
    thumbs_up: 2,
    heart: 0,
    celebrate: 0,
    eyes: 1,
    question: 0,
  };
  let mine: string[] = ["eyes"];
  let watch = { count: 2, mine: false };
  let files = [
    {
      pid: "f1",
      filename: "Plan résumé.pdf",
      content_type: "application/pdf",
      size_bytes: 1536,
      sha256: "ab".repeat(32),
      uploaded_by: OTHER,
      created_at: "2026-11-03T11:00:00Z",
    },
  ];
  const sent: Sent[] = [];
  await page.route(`**/api/plans/${PID}/**`, (route) => {
    const path = new URL(route.request().url()).pathname.replace(
      /^\/api\/proxy/,
      "",
    );
    if (path === `/api/plans/${PID}/tasks/${T1}`)
      return route.fulfill({ json: task });
    if (path === `/api/plans/${PID}/tasks`)
      return route.fulfill({
        json: { tasks: [task], counts: {}, total: 1, truncated: false },
      });
    if (path.endsWith("/relations"))
      return route.fulfill({ json: { relations: [], hidden: 0 } });
    if (path.endsWith("/links")) return route.fulfill({ json: [] });
    return route.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
  await page.route(
    /\/api\/proxy\/api\/(comments|reactions|attachments|watchers)/,
    async (route) => {
      const req = route.request();
      const url = new URL(req.url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      const method = req.method();
      const raw = req.postDataBuffer();
      let body: unknown = null;
      if (path !== "/api/attachments" && raw && raw.length > 0)
        body = JSON.parse(raw.toString("utf8"));
      if (method !== "GET")
        sent.push({
          method,
          path,
          query: url.search,
          body,
          type: req.headers()["content-type"],
          size: raw?.length ?? 0,
        });
      const status = refuse[`${method} ${path}`];
      if (status) {
        delete refuse[`${method} ${path}`];
        return route.fulfill({
          status,
          json: {
            error: "refused",
            description: "service text that must not be shown",
          },
        });
      }
      if (path === "/api/comments" && method === "GET")
        return route.fulfill({ json: comments });
      if (path === "/api/comments" && method === "POST") {
        const b = body as { body: string };
        comments = [...comments, comment("c3", ME, b.body)];
        if (!watch.mine) watch = { count: watch.count + 1, mine: true };
        return route.fulfill({ json: { pid: "c3" } });
      }
      const one = /^\/api\/comments\/(c\d)(\/delete|\/history)?$/.exec(path);
      if (one && method === "PUT") {
        const b = body as { body: string };
        comments = comments.map((c) =>
          c.pid === one[1]
            ? { ...c, body: b.body, edited_at: "2026-11-05T09:00:00Z" }
            : c,
        );
        return route.fulfill({
          json: comments.find((c) => c.pid === one[1]),
        });
      }
      if (one && one[2] === "/delete" && method === "POST") {
        comments = comments.filter((c) => c.pid !== one[1]);
        return route.fulfill({ json: {} });
      }
      if (one && one[2] === "/history")
        return route.fulfill({
          json: [
            {
              previous_body: "Yes.",
              edited_by: ME,
              created_at: "2026-11-04T09:00:00Z",
            },
          ],
        });
      if (path === "/api/reactions" && method === "GET")
        return route.fulfill({ json: { counts, mine } });
      if (path === "/api/reactions" && method === "POST") {
        const k = (body as { kind: string }).kind;
        if (!mine.includes(k)) {
          mine = [...mine, k];
          counts = { ...counts, [k]: (counts[k] ?? 0) + 1 };
        }
        return route.fulfill({ json: { created: true } });
      }
      if (path === "/api/reactions/remove" && method === "POST") {
        const k = (body as { kind: string }).kind;
        if (mine.includes(k)) {
          mine = mine.filter((m) => m !== k);
          counts = { ...counts, [k]: (counts[k] ?? 1) - 1 };
        }
        return route.fulfill({ json: {} });
      }
      if (path === "/api/watchers" && method === "GET")
        return opts.signedOut
          ? route.fulfill({ status: 401, json: { error: "unauthorized" } })
          : route.fulfill({ json: watch });
      if (path === "/api/watchers" && method === "POST") {
        watch = { count: watch.count + (watch.mine ? 0 : 1), mine: true };
        return route.fulfill({ json: { watching: true, created: true } });
      }
      if (path === "/api/watchers/unwatch" && method === "POST") {
        watch = { count: watch.count - (watch.mine ? 1 : 0), mine: false };
        return route.fulfill({ json: {} });
      }
      if (path === "/api/attachments" && method === "GET")
        return route.fulfill({ json: files });
      if (path === "/api/attachments" && method === "POST") {
        const name = url.searchParams.get("filename") ?? "file";
        const stored = {
          pid: "f2",
          filename: name,
          content_type: req.headers()["content-type"] ?? "",
          size_bytes: raw?.length ?? 0,
          sha256: "cd".repeat(32),
        };
        files = [
          ...files,
          { ...stored, uploaded_by: ME, created_at: "2026-11-05T12:00:00Z" },
        ];
        return route.fulfill({ json: stored });
      }
      const del = /^\/api\/attachments\/(f\d)\/delete$/.exec(path);
      if (del && method === "POST") {
        files = files.filter((f) => f.pid !== del[1]);
        return route.fulfill({ json: {} });
      }
      return route.fulfill({
        status: 404,
        json: { error: "unhandled in stub" },
      });
    },
  );
  return sent;
}

const open = (page: Page, locale = "en-001") =>
  page.goto(`/${locale}/plans/${PID}/tasks/${T1}`, {
    waitUntil: "networkidle",
  });

const status = (page: Page, text: string) =>
  page.getByRole("status").filter({ hasText: text });

test("shows the thread, posts a comment, and commenting follows the task", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  await expect(page.getByRole("heading", { name: "Discussion" })).toBeVisible();
  const list = page.getByTestId("comments");
  await expect(list.getByText("Is the scope agreed?")).toBeVisible();
  await expect(list.getByText(new RegExp(`^By ${OTHER},`))).toBeVisible();
  await expect(list.getByText(/· edited /)).toBeVisible();
  await expect(page.getByText("You do not follow this.")).toBeVisible();
  await expect(page.getByText("2 people follow it.")).toBeVisible();

  await page.getByLabel("Your comment").fill("Agreed on Monday.");
  await expect(page.getByText("3,983 of 4,000 characters left.")).toBeVisible();
  await page.getByRole("button", { name: "Post the comment" }).click();
  await expect(status(page, "The comment was posted.")).toBeVisible();
  await expect(list.getByText("Agreed on Monday.")).toBeVisible();
  await expect(page.getByLabel("Your comment")).toHaveValue("");
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/comments",
    body: { subject_kind: "task", subject_pid: T1, body: "Agreed on Monday." },
  });
  await expect(page.getByText(/^You follow this:/)).toBeVisible();
  await expect(page.getByText("3 people follow it.")).toBeVisible();
});

test("refuses a blank comment and one over 4000 characters on the page, before any call", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  await page.getByRole("button", { name: "Post the comment" }).click();
  await expect(page.getByRole("alert")).toHaveText("Write the comment first.");
  await page.getByLabel("Your comment").fill("x".repeat(4002));
  await expect(
    page.getByText("2 characters over the limit of 4,000."),
  ).toBeVisible();
  await page.getByRole("button", { name: "Post the comment" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A comment can be at most 4000 characters.",
  );
  expect(sent).toEqual([]);
});

test("edits a comment, shows its earlier version, and deletes one after asking", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const list = page.getByTestId("comments");
  await list.getByRole("button", { name: "Earlier versions" }).click();
  await expect(list.getByText("Yes.", { exact: true })).toBeVisible();
  await expect(list.getByText(/^Replaced /)).toBeVisible();

  await list
    .getByRole("button", { name: new RegExp(`^Edit the comment by ${ME}`) })
    .click();
  await page.getByLabel("Change the comment").fill("Yes, agreed on Monday.");
  await page.getByRole("button", { name: "Save the comment" }).click();
  await expect(
    status(page, "The comment was changed; its earlier version is kept."),
  ).toBeVisible();
  await expect(list.getByText("Yes, agreed on Monday.")).toBeVisible();
  expect(sent.at(-1)).toMatchObject({
    method: "PUT",
    path: "/api/comments/c2",
    body: { body: "Yes, agreed on Monday." },
  });

  // Cancel keeps the comment; OK deletes it through the POST twin.
  const del = list.getByRole("button", {
    name: new RegExp(`^Delete the comment by ${OTHER}`),
  });
  await del.click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "Cancel" })
    .click();
  await expect(list.getByText("Is the scope agreed?")).toBeVisible();
  await del.click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Delete this comment?",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(status(page, "The comment was deleted.")).toBeVisible();
  await expect(list.getByText("Is the scope agreed?")).toHaveCount(0);
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/comments/c1/delete",
  });
});

test("a refused change says who may make it, and a comment already gone says so", async ({
  page,
}) => {
  await stub(page, {
    refuse: {
      "PUT /api/comments/c1": 403,
      "POST /api/comments/c2/delete": 404,
    },
  });
  await open(page);
  const list = page.getByTestId("comments");
  await list
    .getByRole("button", { name: new RegExp(`^Edit the comment by ${OTHER}`) })
    .click();
  await page.getByLabel("Change the comment").fill("Changed by me");
  await page.getByRole("button", { name: "Save the comment" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Only the person who wrote a comment may change or delete it.",
  );
  await page.getByRole("button", { name: "Cancel" }).click();
  await list
    .getByRole("button", { name: new RegExp(`^Delete the comment by ${ME}`) })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That comment has gone; the comments have been read again.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);
});

test("a reaction is a toggle, with its count and its name in words", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const group = page.getByRole("group", { name: "Reactions" });
  const heart = group.getByRole("button", { name: "Heart: 0 people" });
  await expect(heart).toHaveAttribute("aria-pressed", "false");
  await expect(
    group.getByRole("button", { name: "Looking at it: 1 person" }),
  ).toHaveAttribute("aria-pressed", "true");
  await heart.click();
  const pressed = group.getByRole("button", { name: "Heart: 1 person" });
  await expect(pressed).toHaveAttribute("aria-pressed", "true");
  expect(sent.at(-1)).toEqual(
    expect.objectContaining({
      method: "POST",
      path: "/api/reactions",
      body: { subject_kind: "task", subject_pid: T1, kind: "heart" },
    }),
  );
  await pressed.click();
  await expect(
    group.getByRole("button", { name: "Heart: 0 people" }),
  ).toHaveAttribute("aria-pressed", "false");
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/reactions/remove",
    body: { kind: "heart" },
  });
});

test("follows and stops following through the POST twins; without a sign-in it says to sign in", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  await page.getByRole("button", { name: "Follow", exact: true }).click();
  await expect(status(page, "You now follow this.")).toBeVisible();
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/watchers",
    body: { subject_kind: "task", subject_pid: T1 },
  });
  await page.getByRole("button", { name: "Stop following" }).click();
  await expect(status(page, "You no longer follow this.")).toBeVisible();
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/watchers/unwatch",
  });

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { signedOut: true });
  await open(page);
  await expect(
    page.getByText(
      "Sign in to follow this and be told of new comments and changes.",
    ),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Follow", exact: true }),
  ).toHaveCount(0);
  await expect(page.getByRole("alert")).toHaveCount(0);
});

test("lists files as download links, never drawn in the page, and attaches one as itself", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const files = page.getByTestId("attachments");
  const link = files.getByRole("link", { name: "Plan résumé.pdf" });
  await expect(link).toHaveAttribute(
    "href",
    /\/api\/proxy\/api\/attachments\/f1\/download$/,
  );
  await expect(link).not.toHaveAttribute("target", /.+/);
  await expect(
    files.getByText(new RegExp(`^PDF document, 1.5 KB, added by ${OTHER},`)),
  ).toBeVisible();
  await expect(
    page.locator("main").locator("img, iframe, object, embed, video"),
  ).toHaveCount(0);

  await page.getByLabel("File to attach").setInputFiles({
    name: "notes 2026.md",
    mimeType: "",
    buffer: Buffer.from("# Notes\n"),
  });
  await page.getByRole("button", { name: "Attach the file" }).click();
  await expect(
    status(page, "The file notes 2026.md was attached."),
  ).toBeVisible();
  await expect(
    files.getByRole("link", { name: "notes 2026.md" }),
  ).toBeVisible();
  const up = sent.at(-1)!;
  expect(up.method).toBe("POST");
  expect(up.path).toBe("/api/attachments");
  expect(new URLSearchParams(up.query).get("subject_kind")).toBe("task");
  expect(new URLSearchParams(up.query).get("subject_pid")).toBe(T1);
  expect(new URLSearchParams(up.query).get("filename")).toBe("notes 2026.md");
  expect(up.type).toBe("text/markdown");
  expect(up.size).toBe(8);
});

test("refuses a web page, an empty file and one over 10 MB on the page, before any call", async ({
  page,
}) => {
  const sent = await stub(page);
  await open(page);
  const input = page.getByLabel("File to attach");
  const attach = page.getByRole("button", { name: "Attach the file" });
  await attach.click();
  await expect(page.getByRole("alert")).toHaveText("Choose a file first.");
  await input.setInputFiles({
    name: "page.html",
    mimeType: "text/html",
    buffer: Buffer.from("<script>alert(1)</script>"),
  });
  await attach.click();
  await expect(page.getByRole("alert")).toHaveText(
    "That kind of file cannot be attached. The list below says which kinds can.",
  );
  await input.setInputFiles({
    name: "empty.txt",
    mimeType: "text/plain",
    buffer: Buffer.alloc(0),
  });
  await attach.click();
  await expect(page.getByRole("alert")).toHaveText("That file is empty.");
  await input.setInputFiles({
    name: "big.pdf",
    mimeType: "application/pdf",
    buffer: Buffer.alloc(10 * 1024 * 1024 + 1),
  });
  await attach.click();
  await expect(page.getByRole("alert")).toHaveText(
    "That file is over 10 MB, the limit for one file.",
  );
  expect(sent).toEqual([]);
});

test("a refused file says why in words, and removing one asks first and keeps the uploader rule", async ({
  page,
}) => {
  await stub(page, {
    refuse: {
      "POST /api/attachments": 422,
      "POST /api/attachments/f1/delete": 403,
    },
  });
  await open(page);
  const input = page.getByLabel("File to attach");
  const attach = page.getByRole("button", { name: "Attach the file" });
  await input.setInputFiles({
    name: "fake.png",
    mimeType: "image/png",
    buffer: Buffer.from("not a png"),
  });
  await attach.click();
  await expect(page.getByRole("alert")).toContainText(
    "The service did not accept the file",
  );
  await page
    .getByRole("button", { name: "Remove the file Plan résumé.pdf" })
    .click();
  await expect(page.getByRole("alertdialog")).toContainText(
    "Remove the file Plan résumé.pdf?",
  );
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Only the person who attached a file, or an administrator, may remove it.",
  );
  await expect(
    page.getByText("service text that must not be shown"),
  ).toHaveCount(0);

  await page.unrouteAll({ behavior: "wait" });
  await stub(page, { refuse: { "POST /api/attachments": 503 } });
  await open(page);
  await page.getByLabel("File to attach").setInputFiles({
    name: "a.txt",
    mimeType: "text/plain",
    buffer: Buffer.from("hello"),
  });
  await page.getByRole("button", { name: "Attach the file" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "The file was not attached: the malware scanner did not answer. Try again later.",
  );
});

test("removes a file after asking, through the POST twin", async ({ page }) => {
  const sent = await stub(page);
  await open(page);
  await page
    .getByRole("button", { name: "Remove the file Plan résumé.pdf" })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(
    status(page, "The file Plan résumé.pdf was removed."),
  ).toBeVisible();
  await expect(page.getByText("No files attached.")).toBeVisible();
  expect(sent.at(-1)).toMatchObject({
    method: "POST",
    path: "/api/attachments/f1/delete",
  });
});

test("the discussion reads in Welsh", async ({ page }) => {
  await stub(page);
  await open(page, "cy-001");
  await expect(page.getByRole("heading", { name: "Trafodaeth" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Sylwadau" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Ffeiliau" })).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Postio'r sylw" }),
  ).toBeVisible();
  await expect(
    page
      .getByTestId("attachments")
      .getByText(/^Dogfen PDF, 1,5 cilobeit|^Dogfen PDF, 1.5 cilobeit/),
  ).toBeVisible();
});

test("passes the accessibility audit, with comments and files listed", async ({
  page,
}) => {
  await stub(page);
  await open(page);
  await expect(page.getByTestId("comments")).toBeVisible();
  await expect(page.getByTestId("attachments")).toBeVisible();
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
