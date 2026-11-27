// RE-1 and RE-3 (NC-1 slice 12 part l) in a real browser: `/plans/[pid]/roadmap` makes a Product plan's roadmap,
// changes its own fields with only what changed sent, marks it reviewed, and adds, changes, moves and removes items;
// a plan of another kind is told it has none; a slug another roadmap holds is said in the page's words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const item = (
  pid: string,
  title: string,
  placement: string | null,
  position: number,
  over: Record<string, unknown> = {},
) => ({
  pid,
  public_id: `R${pid.toUpperCase()}`,
  title,
  description: null,
  expected_outcome: null,
  phase: null,
  tags: [],
  organization_pids: [],
  details: [],
  placement,
  position,
  translations: {},
  ...over,
});

async function stub(
  page: Page,
  opts: { kind?: string; made?: boolean; putStatus?: number } = {},
) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  let roadmap: Record<string, unknown> | null =
    opts.made === false
      ? null
      : {
          pid: "rm1",
          plan_pid: PID,
          slug: "payments",
          title: "Payments",
          vision: "Pay in a tap",
          service_description: null,
          category_pid: null,
          published: false,
          reviewed_on: "2026-09-01",
          freshness: {
            state: "stale",
            stale_after_days: 42,
            stale_from: "2026-10-14",
            today: "2026-11-27",
          },
          translations: {},
          items: [
            item("a", "Tap to pay", "now", 0, { tags: ["cards"] }),
            item("b", "Wallets", "now", 1),
            item("c", "Refunds", null, 0),
          ],
        };
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}`, (r) =>
    r.fulfill({
      json: {
        kind: opts.kind ?? "Product",
        name: "Payments",
        goals: [],
        keywords: [],
        tags: [],
        identifiers: [],
        relationships: [],
        same_as: [],
        alternate_names: [],
        in_language: ["en"],
      },
    }),
  );
  await page.route(/\/api\/plans\/[^/]+\/roadmap(\/.*)?$/, async (route) => {
    const r = route.request();
    const path = new URL(r.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = r.method();
    if (method === "GET")
      return roadmap
        ? route.fulfill({ json: roadmap })
        : route.fulfill({ status: 404, json: { code: "not_found" } });
    const body = r.postDataJSON() ?? null;
    sent.push({ method, path, body });
    if (opts.putStatus && method === "PUT" && path.endsWith("/roadmap"))
      return route.fulfill({
        status: opts.putStatus,
        json: { code: "conflict", detail: "service text" },
      });
    const items = (roadmap?.items ?? []) as ReturnType<typeof item>[];
    if (path.endsWith("/publish/preview")) {
      const b = body as {
        roadmap: { published: { base: boolean; value: boolean } };
      };
      return route.fulfill({
        json: {
          changes: [
            {
              subject: "roadmap",
              field: "published",
              from: b.roadmap.published.base,
              to: b.roadmap.published.value,
            },
          ],
          conflicts: [],
          problems: [],
          would_publish: true,
        },
      });
    }
    if (path.endsWith("/publish")) {
      if (opts.publishConflict)
        return route.fulfill({
          status: 409,
          json: {
            error: "conflict",
            description: "service text",
            conflicts: [
              {
                subject: "roadmap",
                field: "title",
                base: "Payments",
                value: "Payments",
                current: "Pay",
              },
            ],
          },
        });
      const b = body as { roadmap: { published: { value: boolean } } };
      roadmap = { ...roadmap, published: b.roadmap.published.value };
      return route.fulfill({ json: { applied: [], roadmap } });
    }
    if (path.endsWith("/roadmap") && method === "PUT") {
      roadmap = {
        ...(roadmap ?? {
          pid: "rm1",
          plan_pid: PID,
          slug: "payments",
          published: false,
          reviewed_on: null,
          freshness: {
            state: "unknown",
            stale_after_days: 42,
            stale_from: null,
            today: "2026-11-27",
          },
          items: [],
          vision: null,
          service_description: null,
        }),
        ...(body as object),
      };
    } else if (path.endsWith("/reviewed")) {
      roadmap = {
        ...roadmap,
        reviewed_on: "2026-11-27",
        freshness: {
          state: "fresh",
          stale_after_days: 42,
          stale_from: "2027-01-09",
          today: "2026-11-27",
        },
      };
    } else if (path.endsWith("/roadmap/items")) {
      const b = body as { title: string; placement?: string };
      roadmap = {
        ...roadmap,
        items: [
          ...items,
          item(
            "d",
            b.title,
            b.placement ?? null,
            items.filter((i) => i.placement === (b.placement ?? null)).length,
          ),
        ],
      };
    } else if (path.endsWith("/move")) {
      const b = body as { placement: string | null; index?: number };
      const id = path.split("/").at(-2)!;
      roadmap = {
        ...roadmap,
        items: items.map((i) =>
          i.pid === id
            ? { ...i, placement: b.placement, position: b.index ?? 99 }
            : i.placement === b.placement &&
                b.index !== undefined &&
                i.position >= b.index
              ? { ...i, position: i.position - 1 }
              : i,
        ),
      };
    } else if (path.endsWith("/delete")) {
      const id = path.split("/").at(-2)!;
      roadmap = { ...roadmap, items: items.filter((i) => i.pid !== id) };
    } else if (method === "PUT") {
      const id = path.split("/").at(-1)!;
      roadmap = {
        ...roadmap,
        items: items.map((i) =>
          i.pid === id ? { ...i, ...(body as object) } : i,
        ),
      };
    }
    return route.fulfill({ json: {} });
  });
  return sent;
}

test("a roadmap's state and items by placement; marked reviewed; its own fields saved with only what changed", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  await expect(page.getByTestId("roadmap-state")).toContainText(
    "Not published",
  );
  await expect(page.getByTestId("roadmap-state")).toContainText(
    "Stale: due a review",
  );
  const now = page.getByTestId("placement-now").getByRole("listitem");
  await expect(now).toHaveCount(2);
  await expect(now.nth(0)).toContainText("Tap to pay");
  await expect(now.nth(0)).toContainText("Tags: cards");
  await expect(page.getByTestId("placement-none")).toContainText("Refunds");
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);

  await page.getByRole("button", { name: "Mark it reviewed today" }).click();
  await expect(page.getByTestId("roadmap-state")).toContainText("Fresh");
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap/reviewed`,
    body: { today: true },
  });

  const own = page.getByRole("form", { name: "The roadmap" });
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await own.getByLabel(/^Address/).fill("Pay Now");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-problems")).toContainText(
    "The address must be lower-case letters",
  );
  await own.getByLabel(/^Address/).fill("pay-now");
  await own.getByLabel(/^Vision/).fill("");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved the roadmap.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap`,
    body: { slug: "pay-now", vision: "" },
  });
});

test("items are added, changed, moved up, moved to another placement, and removed after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const add = page.getByRole("form", { name: "Add an item" });
  await add
    .getByRole("combobox", { name: "Where it goes" })
    .selectOption("later");
  await add.getByLabel(/^Title/).fill(" Instalments ");
  await add.getByLabel(/^Tags/).fill("credit, Credit");
  await add.getByRole("button", { name: "Add the item" }).click();
  await expect(page.getByTestId("roadmap-problems")).toHaveText(
    "Each tag may appear only once.",
  );
  await add.getByLabel(/^Tags/).fill("credit");
  await add.getByRole("button", { name: "Add the item" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Added “Instalments”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/items`,
    body: { title: "Instalments", tags: ["credit"], placement: "later" },
  });
  await expect(page.getByTestId("placement-later")).toContainText(
    "Instalments",
  );

  const now = page.getByTestId("placement-now").getByRole("listitem");
  await now.nth(1).getByRole("button", { name: "Up" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Moved “Wallets” to Now.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/items/b/move`,
    body: { placement: "now", index: 0 },
  });

  const refunds = page
    .getByTestId("placement-none")
    .getByRole("listitem")
    .nth(0);
  await refunds.getByRole("combobox", { name: "Move to" }).selectOption("next");
  await refunds.getByRole("button", { name: "Move", exact: true }).click();
  await expect(page.getByTestId("placement-next")).toContainText("Refunds");
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/items/c/move`,
    body: { placement: "next" },
  });

  const tap = page
    .getByTestId("placement-now")
    .getByRole("listitem")
    .filter({ hasText: "Tap to pay" });
  await tap.getByRole("button", { name: "Change" }).click();
  const form = page.getByRole("form", { name: "Change “Tap to pay”" });
  await form.getByRole("button", { name: "Save the item" }).click();
  await expect(form).toHaveCount(0);
  await tap.getByRole("button", { name: "Change" }).click();
  await form.getByLabel(/^Phase/).fill("Beta");
  await form.getByRole("button", { name: "Save the item" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved “Tap to pay”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap/items/a`,
    body: { phase: "Beta" },
  });

  const ask = page.getByRole("alertdialog");
  await tap.getByRole("button", { name: "Remove" }).click();
  await expect(ask).toContainText(
    "Its public id is never given to another item.",
  );
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Removed “Tap to pay”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/items/a/delete`,
    body: null,
  });
});

test("a Product plan makes its roadmap from a title; a slug taken is said in the page's words", async ({
  page,
}) => {
  const sent = await stub(page, { made: false, putStatus: 409 });
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const make = page.getByRole("form", { name: "Make the roadmap" });
  await make.getByLabel(/^Title/).fill(" Payments ");
  await make.getByRole("button", { name: "Make the roadmap now" }).click();
  await expect(page.getByTestId("roadmap-error")).toHaveText(
    "Another roadmap has or had that address; choose another.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/roadmap`,
      body: { title: "Payments" },
    },
  ]);
});

test("a plan of another kind is told it has no roadmap; the page reads in Welsh", async ({
  page,
}) => {
  await stub(page, { kind: "Project", made: false });
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await expect(page.getByTestId("roadmap-not-product")).toBeVisible();
  await expect(page.getByRole("form")).toHaveCount(0);
  await page.goto(`/cy-001/plans/${PID}/roadmap`);
  await expect(
    page.getByRole("heading", { level: 1, name: "Map ffordd y cynnyrch" }),
  ).toBeVisible();
});

test("publishing goes as one change set: previewed, then published; a conflict meanwhile is said and nothing published (RE-2)", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  await page.getByRole("button", { name: "Publish it…" }).click();
  const preview = page.getByTestId("publish-preview");
  await expect(preview.getByTestId("preview-changes")).toHaveText(
    "Published on the public reader: from no to yes",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/publish/preview`,
    body: { roadmap: { published: { base: false, value: true } } },
  });
  await preview.getByRole("button", { name: "Publish the change set" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Published the change set.",
  );
  await expect(page.getByTestId("roadmap-state")).toContainText("Published");
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/publish`,
    body: { roadmap: { published: { base: false, value: true } } },
  });
  await expect(
    page.getByRole("button", { name: "Take it off the public reader…" }),
  ).toBeVisible();
});

test("a change set refused for a conflict names it in the page's words", async ({
  page,
}) => {
  await stub(page, { publishConflict: true });
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  await page.getByRole("button", { name: "Publish it…" }).click();
  await page
    .getByTestId("publish-preview")
    .getByRole("button", { name: "Publish the change set" })
    .click();
  await expect(page.getByTestId("roadmap-error")).toHaveText(
    /^Someone else changed the roadmap meanwhile/,
  );
  await expect(page.getByTestId("preview-conflicts")).toHaveText(
    "Title: now Pay, where you asked for Payments",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await expect(
    page
      .getByTestId("publish-preview")
      .getByRole("button", { name: "Publish the change set" }),
  ).toHaveCount(0);
});
