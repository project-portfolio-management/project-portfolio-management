// Onboarding and offboarding in a real browser: checklists with a word and a
// shape per item, close-out readiness, the access record (or a plain note when
// the person may not see it), and handover with a preview and a guarded commit.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, primaryButtons } from "./fixtures";

async function audit(page: Page) {
  const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"]).analyze();
  expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id)).toEqual([]);
}

const progress = (done: number, open: number, overdue = 0) => ({ total: done + open, done, skipped: 0, open, overdue, resolved_bp: 5000 });
const ITEMS = [
  { pid: "i1", position: 0, title: "Revoke access", owner_ref: null, due_date: "2026-10-01", resolution: null, note: null, overdue: true },
  { pid: "i2", position: 1, title: "Record lessons", owner_ref: "person:x", due_date: "2026-12-01", resolution: null, note: null, overdue: false },
  { pid: "i3", position: 2, title: "Archive", owner_ref: null, due_date: null, resolution: "done", note: null, overdue: false },
];
const CASE = {
  pid: "c1", kind: "plan_closeout", plan_pid: PID, title: "Closing out Apollo", subject_ref: null,
  effective_date: "2026-10-12", status: "open", closed_reason: null, progress: progress(1, 2, 1), items: ITEMS, masked: false,
};
const CLOSEOUT = {
  plan_pid: PID, ready: false,
  counts: { open_tasks: 2, open_risks: 0, active_allocations: 1, outstanding_access: 1, open_cases: 1 },
  findings: [{ code: "open_tasks", count: 2, text: "2 task(s) are not finished" }, { code: "outstanding_access", count: 1, text: "1 access grant(s) have not been revoked" }],
};

async function stub(page: Page, opts: { accessDenied?: boolean } = {}) {
  await page.route(`**/api/plans/${PID}/onboarding/cases**`, (r) =>
    r.request().method() === "POST"
      ? r.fulfill({ json: CASE })
      : r.fulfill({ json: { cases: [{ pid: "c1", kind: "plan_closeout", title: "Closing out Apollo", status: "open", subject_ref: null, effective_date: "2026-10-12", progress: progress(1, 2, 1) }], hidden_people_cases: 2, masked: true } }));
  await page.route(`**/api/plans/${PID}/closeout`, (r) => r.fulfill({ json: CLOSEOUT }));
  await page.route(`**/api/plans/${PID}/access-records`, (r) =>
    opts.accessDenied
      ? r.fulfill({ status: 403, json: { error: "forbidden" } })
      : r.request().method() === "POST"
        ? r.fulfill({ json: { pid: "a1" } })
        : r.fulfill({ json: { outstanding: [{ person_ref: "person:x", access: "repo write", granted_on: "2026-10-01" }], records: [], note: "This is a record, kept by hand." } }));
}

test("checklists show a word and a shape per item, and readiness says why not", async ({ page }) => {
  await stub(page);
  await page.route("**/api/onboarding/cases/c1", (r) => r.fulfill({ json: CASE }));
  await page.goto(`/en-001/plans/${PID}/onboarding`);
  await expect.poll(() => primaryButtons(page), { message: "with no checklist open, starting one is the primary action" }).toEqual(["Start"]);
  await expect(page.getByTestId("closeout")).toContainText("Not ready to close: 2 task(s) are not finished; 1 access grant(s) have not been revoked.");
  await expect(page.getByTestId("cases")).toContainText("1 of 3 done, 1 overdue");
  await expect(page.getByText("2 checklists about people are not shown to you.")).toBeVisible();
  await page.getByRole("button", { name: "Open Closing out Apollo" }).click();
  const c = page.getByTestId("case");
  await expect(c).toContainText("Overdue");
  await expect(c).toContainText("Done");
  await expect(c).toContainText("Open");
  await expect(page.getByTestId("outstanding")).toContainText("person:x: repo write");
  await expect.poll(() => primaryButtons(page), { message: "one primary action: finishing the open checklist" }).toEqual(["Complete checklist"]);
  await audit(page);
});

test("resolving an item sends exactly that change", async ({ page }) => {
  await stub(page);
  let sent: unknown = null;
  await page.route("**/api/onboarding/cases/c1", (r) => r.fulfill({ json: CASE }));
  await page.route("**/api/onboarding/cases/c1/items/i1", (r) => { sent = r.request().postDataJSON(); return r.fulfill({ json: CASE }); });
  await page.goto(`/en-001/plans/${PID}/onboarding`);
  await page.getByRole("button", { name: "Open Closing out Apollo" }).click();
  await page.getByRole("button", { name: "Done" }).first().click();
  await expect.poll(() => sent).toEqual({ resolution: "done" });
  await page.getByLabel("Why skip Revoke access").fill("not needed");
  await page.getByRole("button", { name: "Skip" }).first().click();
  await expect.poll(() => sent).toEqual({ resolution: "skipped", note: "not needed" });
});

test("starting a checklist is checked in the form first", async ({ page }) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/onboarding`);
  await page.getByRole("button", { name: "Start", exact: true }).click();
  await expect(page.getByRole("status")).toContainText("Give the date it takes effect.");
  await page.getByLabel("What").selectOption("person_leave");
  await page.getByLabel("Takes effect").fill("2026-10-12");
  await page.getByRole("button", { name: "Start", exact: true }).click();
  await expect(page.getByRole("status")).toContainText("Give the person as person:…");
});

test("without the people permission the access record is a plain note", async ({ page }) => {
  await stub(page, { accessDenied: true });
  await page.goto(`/en-001/plans/${PID}/onboarding`);
  await expect(page.getByTestId("access-denied")).toBeVisible();
  await expect(page.getByTestId("closeout")).toBeVisible(); // readiness names no one, so it still shows
  await audit(page);
});

const HOLD = {
  plan_pid: PID, from_person: "person:a", to_person: "person:b", handover_etag: 'W/"hv1"', action: "reassign",
  holdings: [{ kind: "task", pid: "t1", label: "Write the spec" }, { kind: "risk", pid: "r1", label: "Key person" }],
  counts: { task: 1, risk: 1 }, note: "",
};

test("handover previews, then commits with the preview's etag; a changed holding is refused", async ({ page }) => {
  let headers: Record<string, string> = {};
  let status = 200;
  await page.route(`**/api/plans/${PID}/handover/preview`, (r) => r.fulfill({ json: HOLD }));
  await page.route(`**/api/plans/${PID}/handover/commit`, (r) => {
    headers = r.request().headers();
    return status === 200 ? r.fulfill({ json: { committed: true, audit_entries: 2 } }) : r.fulfill({ status, json: { error: "stale_preview" } });
  });
  await page.goto(`/en-001/plans/${PID}/handover`);
  await page.getByRole("textbox", { name: "From" }).fill("person:a");
  await page.getByRole("textbox", { name: "To" }).fill("person:b");
  await page.getByRole("button", { name: "Preview" }).click();
  await expect.poll(() => primaryButtons(page), { message: "before the preview, previewing is" }).toEqual(["Preview"]);
  const pv = page.getByTestId("preview");
  await expect(pv).toContainText("2 items would move to person:b.");
  await expect(pv).toContainText("Write the spec");
  await expect.poll(() => primaryButtons(page), { message: "after the preview, handing over is the primary action" }).toEqual(["Hand over"]);
  await audit(page);
  await page.getByRole("button", { name: "Hand over" }).click();
  await expect(page.getByRole("status")).toContainText("Done: 2 items handed over and recorded.");
  expect(headers["if-match"]).toBe('W/"hv1"');
  // Again, but what they hold changed in the meantime.
  status = 409;
  await page.getByRole("button", { name: "Preview" }).click();
  await page.getByRole("button", { name: "Hand over" }).click();
  await expect(page.getByRole("status")).toContainText("changed since the preview, so nothing was changed");
  await expect(page.getByTestId("preview")).toHaveCount(0);
});

test("editing the people in the handover form drops the preview", async ({ page }) => {
  await page.route(`**/api/plans/${PID}/handover/preview`, (r) => r.fulfill({ json: HOLD }));
  await page.goto(`/en-001/plans/${PID}/handover`);
  await page.getByRole("textbox", { name: "From" }).fill("person:a");
  await page.getByRole("button", { name: "Preview" }).click();
  await expect(page.getByTestId("preview")).toBeVisible();
  await page.getByRole("textbox", { name: "To" }).fill("person:c");
  await expect(page.getByTestId("preview")).toHaveCount(0);
});
