// T-8 in a real browser (NC-1 slice 12 part k): `/jobs` lists the caller's bulk jobs newest first, each state as a
// word and a symbol and the counts in one sentence; a running job is cancelled after a question; an export and an
// import are started with only what differs from the defaults sent. The service's English is never shown. Welsh
// reads. The page passes axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

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

const counts = (total: number, failed = 0, review = 0) => ({
  total,
  created: total - failed - review,
  updated: 0,
  unchanged: 0,
  queued_for_review: review,
  failed,
});
const base = {
  rows_omitted: 0,
  errors_url: null,
  review_url: null,
  download_url: null,
  expires_at: null,
  failure_reason: "service text",
  cancel_requested: false,
  cancelled: false,
  note: "service text",
  finished_at: null,
};
const JOBS = [
  {
    ...base,
    job_id: "11111111-0000-4000-8000-000000000001",
    kind: "import",
    status: "completed_with_errors",
    format: "csv",
    counts: counts(3, 1, 1),
    reconciles: true,
    errors_url:
      "/api/plans/bulk-jobs/11111111-0000-4000-8000-000000000001/errors",
    review_url:
      "/api/plans/review-queue?job=11111111-0000-4000-8000-000000000001",
    progress: 100,
    started_at: "2026-11-20T09:00:00Z",
    finished_at: "2026-11-20T09:01:00Z",
  },
  {
    ...base,
    job_id: "11111111-0000-4000-8000-000000000002",
    kind: "export",
    status: "running",
    format: "jsonl",
    counts: counts(0),
    reconciles: true,
    progress: 40,
    started_at: "2026-11-26T10:00:00Z",
  },
  {
    ...base,
    job_id: "11111111-0000-4000-8000-000000000003",
    kind: "export",
    status: "completed",
    format: "jsonl",
    counts: counts(12),
    reconciles: false,
    download_url:
      "/api/plans/export/11111111-0000-4000-8000-000000000003/download",
    progress: 100,
    started_at: "2026-11-25T10:00:00Z",
    finished_at: "2026-11-25T10:02:00Z",
  },
];

type Sent = { method: string; url: string; body: string; type: string | null };
const HELD = [
  {
    pid: "22222222-0000-4000-8000-000000000001",
    status: "pending",
    provenance: "import",
    candidate: { name: "Apollo 2", kind: "Project" },
    matches: [
      {
        pid: "33333333-0000-4000-8000-000000000001",
        name: "Apollo",
        score: 0.82,
        confidence: "Medium",
      },
    ],
    job_pid: "11111111-0000-4000-8000-000000000001",
    source_line: 2,
    resolved_by: null,
    resolved_at: null,
    resolution_pid: null,
    resolution_comment: null,
    created_at: "2026-11-20T09:00:30Z",
    updated_at: "2026-11-20T09:00:30Z",
  },
];

async function stub(
  page: Page,
  opts: { exportStatus?: number; decideStatus?: number } = {},
) {
  const sent: Sent[] = [];
  await page.route("**/api/proxy/api/plans/**", async (r) => {
    const req = r.request();
    const url = new URL(req.url());
    const path = url.pathname.replace("/api/proxy", "");
    if (req.method() !== "GET")
      sent.push({
        method: req.method(),
        url: path + url.search,
        body: req.postData() ?? "",
        type: req.headers()["content-type"] ?? null,
      });
    if (req.method() === "GET" && path === "/api/plans/bulk-jobs") {
      const kind = url.searchParams.get("kind");
      return r.fulfill({ json: JOBS.filter((j) => !kind || j.kind === kind) });
    }
    if (req.method() === "GET" && path === "/api/plans/review-queue")
      return r.fulfill({
        json: sent.some((s) => s.url.includes("/review-queue/")) ? [] : HELD,
      });
    if (req.method() === "POST" && path.includes("/review-queue/"))
      return opts.decideStatus
        ? r.fulfill({
            status: opts.decideStatus,
            json: { code: "conflict", detail: "service text" },
          })
        : r.fulfill({
            json: {
              ...HELD[0],
              status: path.endsWith("/confirm") ? "confirmed" : "rejected",
            },
          });
    if (req.method() === "POST" && path.endsWith("/cancel"))
      return r.fulfill({
        json: {
          job_id: JOBS[1].job_id,
          status: "running",
          cancel_requested: true,
          changed: true,
          note: "service text",
        },
      });
    if (req.method() === "POST" && path === "/api/plans/export")
      return opts.exportStatus
        ? r.fulfill({
            status: opts.exportStatus,
            json: { code: "forbidden", detail: "service text" },
          })
        : r.fulfill({
            status: 202,
            json: { job_id: "new-export", status: "queued" },
          });
    if (req.method() === "POST" && path === "/api/plans/import")
      return r.fulfill({
        status: 202,
        json: { job_id: "new-import", status: "queued" },
      });
    return r.fulfill({ status: 404, json: { code: "not_found" } });
  });
  return sent;
}

test("lists the jobs newest first, each state in words, with its links", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/en-001/jobs");
  await expect(
    page.getByRole("heading", { level: 1, name: "Imports and exports" }),
  ).toBeVisible();
  const items = page.getByTestId("jobs").getByRole("listitem");
  await expect(items).toHaveCount(3);
  await expect(items.nth(0)).toContainText("export · JSON lines");
  await expect(items.nth(0)).toContainText("▶ running · 40% done");
  await expect(items.nth(1)).toContainText("✓ completed");
  await expect(items.nth(1)).toContainText(
    "These counts do not add up to the total.",
  );
  await expect(
    items.nth(1).getByRole("link", { name: "Download the export" }),
  ).toHaveAttribute(
    "href",
    /\/api\/proxy\/api\/plans\/export\/11111111-0000-4000-8000-000000000003\/download$/,
  );
  await expect(items.nth(2)).toContainText("import · CSV");
  await expect(items.nth(2)).toContainText("◐ completed with errors");
  await expect(items.nth(2)).toContainText(
    "3 rows: 1 created, 0 updated, 0 unchanged, 1 held for review, 1 failed.",
  );
  await expect(items.nth(2)).toContainText(
    "listed under “Held for review” below",
  );
  await expect(
    items.nth(2).getByRole("link", { name: "Error report (CSV)" }),
  ).toHaveAttribute("href", /\/errors\?format=csv$/);
  // Only the running job offers a cancel; the service's English is never shown.
  await expect(
    page.getByRole("button", { name: /^Cancel the (import|export) job$/ }),
  ).toHaveCount(1);
  await expect(page.getByText("service text")).toHaveCount(0);
  await audit(page);
});

test("a running job is cancelled only after a question", async ({ page }) => {
  const sent = await stub(page);
  await page.goto("/en-001/jobs");
  const cancel = page.getByRole("button", { name: "Cancel the export job" });
  const ask = page.getByRole("alertdialog");
  await cancel.click();
  await expect(ask).toContainText("A queued job does not start");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await cancel.click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect
    .poll(() => sent.map((s) => `${s.method} ${s.url}`))
    .toEqual([
      "POST /api/plans/bulk-jobs/11111111-0000-4000-8000-000000000002/cancel",
    ]);
});

test("an export sends only what differs from the defaults, and a refusal has its own sentence", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/jobs");
  await settle(page);
  await page.getByRole("button", { name: "Start the export" }).click();
  await expect(page.getByTestId("jobs-notice")).toHaveText(
    "Started: the job is at the top of the list.",
  );
  const form = page.getByRole("form", { name: "Export plans" });
  await form.getByLabel("File format").selectOption("csv");
  await form.getByLabel("Plan kind (optional)").fill(" Project ");
  await form.getByLabel("Name contains (optional)").fill("apollo");
  await page.getByRole("button", { name: "Start the export" }).click();
  await expect
    .poll(() =>
      sent
        .filter((s) => s.url === "/api/plans/export")
        .map((s) => JSON.parse(s.body)),
    )
    .toEqual([{}, { format: "csv", kind: "Project", name_like: "apollo" }]);

  await page.unrouteAll();
  await stub(page, { exportStatus: 403 });
  await page.reload();
  await settle(page);
  await page
    .getByRole("form", { name: "Export plans" })
    .getByLabel("Full export, with people references")
    .check();
  await page.getByRole("button", { name: "Start the export" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A full export needs the destructive right.",
  );
});

test("an import sends the file as it is, its format read from the name", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/jobs");
  await settle(page);
  await page.getByRole("button", { name: "Start the import" }).click();
  await expect(page.getByRole("alert")).toHaveText("Choose a file first.");
  expect(sent).toEqual([]);
  const form = page.getByRole("form", { name: "Import plans" });
  await form.getByLabel("File", { exact: true }).setInputFiles({
    name: "plans.csv",
    mimeType: "text/csv",
    buffer: Buffer.from("name,kind\nApollo,Project\n"),
  });
  await expect(form.getByLabel("File format")).toHaveValue("csv");
  await form.getByLabel("is created anyway").check();
  await page.getByRole("button", { name: "Start the import" }).click();
  await expect(page.getByTestId("jobs-notice")).toHaveText(
    "Started: the job is at the top of the list.",
  );
  const imp = sent.find((s) => s.url.startsWith("/api/plans/import"));
  expect(imp?.url).toBe("/api/plans/import?format=csv&on_keyless=create");
  expect(imp?.body).toBe("name,kind\nApollo,Project\n");
  expect(imp?.type).toBe("text/plain");
});

test("the list filters by kind, and none says so", async ({ page }) => {
  await stub(page);
  await page.goto("/en-001/jobs");
  await page.getByRole("combobox", { name: /^Kind/ }).selectOption("import");
  await expect(page.getByTestId("jobs").getByRole("listitem")).toHaveCount(1);
  await page.unrouteAll();
  await page.route("**/api/proxy/api/plans/bulk-jobs*", (r) =>
    r.fulfill({ json: [] }),
  );
  await page.reload();
  await expect(page.getByTestId("jobs-none")).toHaveText(
    "There is no job you can see.",
  );
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/jobs");
  await expect(
    page.getByRole("heading", { level: 1, name: "Mewnforion ac allforion" }),
  ).toBeVisible();
  await expect(
    page.getByTestId("jobs").getByRole("listitem").nth(0),
  ).toContainText("yn rhedeg");
});

test("a held row is decided after a question: created as new, or discarded", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/jobs");
  const held = page.getByTestId("held");
  await expect(held).toContainText("Apollo 2 · Project");
  await expect(held).toContainText("Line 2 of the file");
  await expect(held.getByRole("link", { name: "Apollo" })).toHaveAttribute(
    "href",
    "/en-001/plans/33333333-0000-4000-8000-000000000001",
  );
  await expect(held).toContainText("82% alike, medium confidence");
  const ask = page.getByRole("alertdialog");
  await held
    .getByRole("button", { name: "It is a duplicate: discard it" })
    .click();
  await expect(ask).toContainText("Nothing was created from it.");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await held.getByRole("button", { name: "It is new: create it" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("jobs-notice")).toHaveText(
    "Created “Apollo 2”.",
  );
  expect(sent.map((s) => `${s.method} ${s.url}`)).toEqual([
    "POST /api/plans/review-queue/22222222-0000-4000-8000-000000000001/confirm",
  ]);
  await expect(page.getByTestId("held-none")).toHaveText(
    "Nothing is held for review.",
  );
  await audit(page);
});

test("a row decided elsewhere says so; a job shows only its held rows", async ({
  page,
}) => {
  await stub(page, { decideStatus: 409 });
  await page.goto("/en-001/jobs");
  await page.getByRole("button", { name: "Show its held rows" }).click();
  await expect(page.getByTestId("held")).toContainText(
    "Showing the rows held by one import.",
  );
  await page
    .getByTestId("held")
    .getByRole("button", { name: "It is a duplicate: discard it" })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("held").getByRole("alert")).toHaveText(
    "That row has already been decided.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});
