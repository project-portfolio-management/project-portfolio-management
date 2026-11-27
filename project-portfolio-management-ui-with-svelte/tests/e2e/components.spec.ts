// PL-11 (NC-1 slice 12 part n) in a real browser: `/plans/[pid]/components` lists every component with its open, done
// and total tasks and the unassigned ones; a component is added, renamed or given a lead with every limit said first;
// one with tasks is not offered for removal, one without is removed after a question. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const API = "aaaaaaaa-0000-4000-8000-000000000001";
const UI = "aaaaaaaa-0000-4000-8000-000000000002";
const comp = (pid: string, name: string, lead: string | null) => ({
  pid,
  plan_pid: PID,
  name,
  lead_ref: lead,
  created_at: "2026-11-01T00:00:00Z",
  updated_at: "2026-11-01T00:00:00Z",
});

async function stub(page: Page, opts: { saveStatus?: number } = {}) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  const list = [
    comp(API, "API", "person:3f2c6b7e-0000-4000-8000-000000000001"),
    comp(UI, "UI", null),
  ];
  await stubPlansApi(page);
  await page.route(/\/api\/plans\/[^/]+\/components(\/.*)?$/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    if (method !== "GET")
      sent.push({ method, path, body: req.postDataJSON() ?? null });
    if (path.endsWith("/summary"))
      return route.fulfill({
        json: {
          buckets: [
            ...list.map((c) => ({
              component_pid: c.pid,
              name: c.name,
              open: c.pid === API ? 2 : 0,
              done: c.pid === API ? 1 : 0,
              total: c.pid === API ? 3 : 0,
            })),
            {
              component_pid: null,
              name: "unassigned",
              open: 1,
              done: 0,
              total: 1,
            },
          ],
          total: 4,
        },
      });
    if (method === "GET") return route.fulfill({ json: list });
    if (opts.saveStatus)
      return route.fulfill({
        status: opts.saveStatus,
        json: { code: "conflict", detail: "service text" },
      });
    if (method === "POST") {
      const b = req.postDataJSON() as { name: string; lead_ref: string | null };
      const c = comp(
        "aaaaaaaa-0000-4000-8000-000000000003",
        b.name,
        b.lead_ref,
      );
      list.push(c);
      return route.fulfill({ json: c });
    }
    if (method === "DELETE") {
      list.splice(
        list.findIndex((c) => path.endsWith(c.pid)),
        1,
      );
      return route.fulfill({ json: {} });
    }
    const b = req.postDataJSON() as { name: string; lead_ref: string | null };
    const i = list.findIndex((c) => path.endsWith(c.pid));
    list[i] = { ...list[i]!, name: b.name, lead_ref: b.lead_ref };
    return route.fulfill({ json: list[i] });
  });
  return sent;
}

test("every component and the unassigned tasks are counted; removal only for an empty one, after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/components`);
  await settle(page);
  const items = page.getByTestId("components").getByRole("listitem");
  await expect(items).toHaveCount(3);
  await expect(items.nth(0)).toContainText("API");
  await expect(items.nth(0)).toContainText("2 open, 1 done, 3 in all");
  await expect(items.nth(0)).toContainText(
    "Lead: person:3f2c6b7e-0000-4000-8000-000000000001",
  );
  await expect(items.nth(0)).toContainText("Tasks still belong to it: 3.");
  await expect(
    items.nth(0).getByRole("button", { name: "Remove" }),
  ).toHaveCount(0);
  await expect(items.nth(1)).toContainText("no lead");
  await expect(items.nth(2)).toContainText("Unassigned");
  await expect(items.nth(2)).toContainText("1 open, 0 done, 1 in all");
  const ask = page.getByRole("alertdialog");
  await items.nth(1).getByRole("button", { name: "Remove" }).click();
  await expect(ask).toContainText("Remove the component “UI”?");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await items.nth(1).getByRole("button", { name: "Remove" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("components-notice")).toHaveText(
    "Removed “UI”.",
  );
  expect(sent).toEqual([
    {
      method: "DELETE",
      path: `/api/plans/${PID}/components/${UI}`,
      body: null,
    },
  ]);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("adding and changing check every limit first; nothing is sent for no change", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/components`);
  await settle(page);
  const add = page.getByRole("form", { name: "Add a component" });
  await add.getByRole("button", { name: "Add the component" }).click();
  await expect(page.getByRole("alert")).toHaveText("A name is required.");
  await add.getByLabel("Name (up to 120 characters)").fill("api");
  await add.getByRole("button", { name: "Add the component" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A component with this name already exists in the plan.",
  );
  await add.getByLabel("Name (up to 120 characters)").fill(" Data ");
  await add.getByLabel(/Lead \(optional/).fill("Ana");
  await add.getByRole("button", { name: "Add the component" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "The lead must be a person: or worker: reference.",
  );
  expect(sent).toEqual([]);
  await add.getByLabel(/Lead \(optional/).fill("worker:7");
  await add.getByRole("button", { name: "Add the component" }).click();
  await expect(page.getByTestId("components-notice")).toHaveText(
    "Added “Data”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/components`,
    body: { name: "Data", lead_ref: "worker:7" },
  });

  const first = page.getByTestId("components").getByRole("listitem").nth(0);
  await first.getByRole("button", { name: "Change" }).click();
  await first.getByRole("button", { name: "Save" }).click();
  expect(sent).toHaveLength(1);
  await first.getByRole("button", { name: "Change" }).click();
  await first.getByLabel("Name (up to 120 characters)").fill("Gateway");
  await first.getByLabel(/Lead \(optional/).fill("");
  await first.getByRole("button", { name: "Save" }).click();
  await expect(page.getByTestId("components-notice")).toHaveText(
    "Saved “Gateway”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/components/${API}`,
    body: { name: "Gateway", lead_ref: null },
  });
});

test("a name taken meanwhile is refused in the catalogue's words; the plan page links here", async ({
  page,
}) => {
  await stub(page, { saveStatus: 409 });
  await page.goto(`/en-001/plans/${PID}/components`);
  await settle(page);
  const add = page.getByRole("form", { name: "Add a component" });
  await add.getByLabel("Name (up to 120 characters)").fill("Data");
  await add.getByRole("button", { name: "Add the component" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A component with this name already exists in the plan.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await page.goto(`/en-001/plans/${PID}`);
  await expect(
    page.getByRole("link", { name: "Components" }).first(),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/components`);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/components`);
  await expect(
    page.getByRole("heading", { level: 1, name: "Cydrannau" }),
  ).toBeVisible();
});
