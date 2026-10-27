// PL-6 in a real browser: the list says what is in force, a draft is edited and sent for
// review, an approved version is brought into force, a final version cannot be edited,
// each state has one primary button, and the pages pass the axe audit.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { primaryButtons, settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join("; ")}`),
  ).toEqual([]);
}

const ver = (n: number, status: string, over = {}) => ({
  pid: `v${n}`,
  version: n,
  status,
  change_summary: null,
  approval_pid: null,
  effective_from: null,
  superseded_on: null,
  superseded_by: null,
  created_by: "me",
  ...over,
});
const APPROVER = "3f2c6b7e-0000-4000-8000-0000000000b2";

test("the list shows what is in force and creates a document", async ({
  page,
}) => {
  let created: unknown = null;
  const doc = {
    pid: "d1",
    title: "Safety policy",
    plan_pid: null,
    created_by: "me",
    in_force_today: 2,
    versions: [ver(1, "superseded"), ver(2, "effective")],
  };
  await page.route("**/api/documents", (r) => {
    if (r.request().method() === "POST") {
      created = r.request().postDataJSON();
      return r.fulfill({
        json: {
          ...doc,
          pid: "d2",
          title: "Travel policy",
          in_force_today: null,
          versions: [ver(1, "draft")],
        },
      });
    }
    return r.fulfill({ json: [doc] });
  });
  await page.goto("/en-001/documents");
  await expect(page.getByRole("link", { name: "Safety policy" })).toBeVisible();
  await expect(page.getByText("Version 2 in force")).toBeVisible();
  await page.getByLabel("Title").fill("Travel policy");
  await page.getByRole("button", { name: "Create document" }).click();
  await expect(page.getByRole("link", { name: "Travel policy" })).toBeVisible();
  expect(created).toMatchObject({ title: "Travel policy" });
  expect(await primaryButtons(page)).toEqual(["Create document"]);
  await audit(page);
});

test("a draft is edited and sent for review; one primary button", async ({
  page,
}) => {
  let sent: unknown = null;
  let edited: unknown = null;
  const draft = {
    pid: "d1",
    title: "Safety policy",
    plan_pid: null,
    created_by: "me",
    in_force_today: null,
    versions: [ver(1, "draft")],
  };
  await page.route("**/api/documents/d1", (r) => r.fulfill({ json: draft }));
  await page.route("**/api/documents/d1/versions/1", (r) => {
    if (r.request().method() === "PUT") {
      edited = r.request().postDataJSON();
      return r.fulfill({ json: { ...ver(1, "draft"), body: "New text" } });
    }
    return r.fulfill({ json: { ...ver(1, "draft"), body: "Wear boots." } });
  });
  await page.route("**/api/documents/d1/versions/1/submit", (r) => {
    sent = r.request().postDataJSON();
    return r.fulfill({ json: ver(1, "in_review", { approval_pid: "a1" }) });
  });
  await page.goto("/en-001/documents/d1");
  await expect(page.getByLabel("Text", { exact: true })).toHaveValue(
    "Wear boots.",
  );
  expect(await primaryButtons(page)).toEqual(["Send for review"]);
  await page.getByLabel("Text", { exact: true }).fill("New text");
  await page.getByRole("button", { name: "Save draft" }).click();
  await expect(page.getByRole("status")).toContainText("Saved.");
  expect(edited).toMatchObject({ body: "New text" });
  await page.getByLabel("Approver's id").fill("not-an-id");
  await page.getByRole("button", { name: "Send for review" }).click();
  await expect(page.getByRole("alert")).toContainText("approver's id");
  expect(sent).toBeNull();
  await page.getByLabel("Approver's id").fill(APPROVER);
  await page.getByRole("button", { name: "Send for review" }).click();
  await expect(page.getByRole("status")).toContainText("Sent for review.");
  await expect.poll(() => sent).toEqual({ approver_ref: `person:${APPROVER}` });
  await audit(page);
});

test("an approved version comes into force; a final version is read-only; the date lookup answers", async ({
  page,
}) => {
  let made = false;
  const approved = {
    pid: "d1",
    title: "Safety policy",
    plan_pid: null,
    created_by: "me",
    in_force_today: 1,
    versions: [
      ver(1, "effective", { effective_from: "2026-10-01" }),
      ver(2, "approved", { approval_pid: "a2" }),
    ],
  };
  await page.route("**/api/documents/d1", (r) => r.fulfill({ json: approved }));
  await page.route("**/api/documents/d1/versions/2", (r) =>
    r.fulfill({
      json: { ...ver(2, "approved", { approval_pid: "a2" }), body: "v2 text" },
    }),
  );
  await page.route("**/api/documents/d1/versions/1", (r) =>
    r.fulfill({
      json: {
        ...ver(1, "effective", { effective_from: "2026-10-01" }),
        body: "v1 text",
      },
    }),
  );
  await page.route("**/api/documents/d1/versions/2/make-effective", (r) => {
    made = true;
    return r.fulfill({
      json: ver(2, "effective", { effective_from: "2026-10-06" }),
    });
  });
  await page.route("**/api/documents/d1/in-force**", (r) =>
    r.fulfill({
      json: {
        document_pid: "d1",
        on: "2026-10-02",
        in_force: ver(1, "effective"),
      },
    }),
  );
  await page.goto("/en-001/documents/d1");
  await expect(
    page.getByRole("button", { name: "Bring into force today" }),
  ).toBeVisible();
  expect(await primaryButtons(page)).toEqual(["Bring into force today"]);
  await page.getByRole("button", { name: /Version 1 — In force/ }).click();
  await expect(page.getByText("This version's text is final")).toBeVisible();
  await expect(page.getByLabel("Text", { exact: true })).toHaveCount(0);
  await page.getByLabel("Date", { exact: true }).fill("2026-10-02");
  await page.getByRole("button", { name: "Look up" }).click();
  await expect(page.getByTestId("in-force")).toContainText(
    "Version 1 was in force on 2026-10-02.",
  );
  await page.getByRole("button", { name: "Bring into force today" }).click();
  await expect(page.getByRole("status")).toContainText("In force from today.");
  expect(made).toBe(true);
  await audit(page);
});
