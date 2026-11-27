// RE-1 and RE-3 (NC-1 slice 12 part l) in a real browser: `/plans/[pid]/roadmap` makes a Product plan's roadmap,
// changes its own fields with only what changed sent, marks it reviewed, and adds, changes, moves and removes items;
// translates the roadmap and its items field by field (RE-6); sets an item's details and organizations; gathers edits into one pending change set, kept through an ended session; a plan of another kind is told it has none; a slug another roadmap holds is said in the page's words. Welsh; axe.

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
  opts: {
    kind?: string;
    made?: boolean;
    putStatus?: number;
    publishConflict?: boolean;
    publish401?: number;
  } = {},
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
          translations: {
            cy: { title: { value: "Taliadau", needs_review: true } },
          },
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
    // A change set's lines, as the service gives them: the roadmap's fields, items' fields, then new items.
    type Set = {
      roadmap?: Record<string, { base: unknown; value: unknown }>;
      items?: {
        pid: string;
        fields: Record<string, { base: unknown; value: unknown }>;
      }[];
      new?: { title: string; placement?: string }[];
    };
    const linesOf = (b: Set) => [
      ...Object.entries(b.roadmap ?? {}).map(([field, c]) => ({
        subject: "roadmap",
        field,
        from: c.base,
        to: c.value,
      })),
      ...(b.items ?? []).flatMap((i) =>
        Object.entries(i.fields).map(([field, c]) => ({
          subject: "item",
          item: i.pid,
          field,
          from: c.base,
          to: c.value,
        })),
      ),
      ...(b.new ?? []).map((n) => ({ subject: "new_item", title: n.title })),
    ];
    if (path.endsWith("/publish/preview")) {
      return route.fulfill({
        json: {
          changes: linesOf(body as Set),
          conflicts: [],
          problems: [],
          would_publish: true,
        },
      });
    }
    if (path.endsWith("/publish")) {
      if (opts.publish401 && opts.publish401-- > 0)
        return route.fulfill({
          status: 401,
          json: { error: "unauthorized", description: "service text" },
        });
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
      const b = body as Set;
      const value = (f: Record<string, { value: unknown }>) =>
        Object.fromEntries(Object.entries(f).map(([k, c]) => [k, c.value]));
      roadmap = {
        ...roadmap,
        ...value(b.roadmap ?? {}),
        items: [
          ...items.map((i) => {
            const c = (b.items ?? []).find((x) => x.pid === i.pid);
            return c ? { ...i, ...value(c.fields) } : i;
          }),
          ...(b.new ?? []).map((n, k) =>
            item(`n${k}`, n.title, n.placement ?? null, 9),
          ),
        ],
      };
      return route.fulfill({ json: { applied: linesOf(b), roadmap } });
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
    } else if (path.includes("/translations/")) {
      // A translation is stored as given, needing no review.
      const lang = path.split("/").at(-1)!;
      const id = path.split("/").at(-3)!;
      const put = (t: unknown) => {
        const all = (t ?? {}) as Record<string, Record<string, unknown>>;
        const now = { ...(all[lang] ?? {}) };
        for (const [f, v] of Object.entries(body as object))
          now[f] = { value: v, needs_review: false };
        return { ...all, [lang]: now };
      };
      roadmap =
        id === "roadmap"
          ? { ...roadmap, translations: put(roadmap?.translations) }
          : {
              ...roadmap,
              items: items.map((i) =>
                i.pid === id ? { ...i, translations: put(i.translations) } : i,
              ),
            };
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

  const own = page.getByRole("form", { name: "The roadmap", exact: true });
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

test("a roadmap is filed under a category, a retired one shown only while it is the roadmap's (RE-7)", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.route("**/api/roadmap-categories", (r) =>
    r.fulfill({
      json: {
        categories: [
          { pid: "c1", name: "Patients", translations: {}, active: true },
          { pid: "c2", name: "Staff", translations: {}, active: false },
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const own = page.getByRole("form", { name: "The roadmap", exact: true });
  const picker = own.getByRole("combobox", { name: "Category (optional)" });
  await expect(picker.getByRole("option")).toHaveText([
    "No category",
    "Patients",
  ]);
  await picker.selectOption("c1");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved the roadmap.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap`,
    body: { category_pid: "c1" },
  });
});

test("the roadmap and an item are translated field by field; a moved source is marked; tags keep their count (RE-6)", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const tr = page.getByTestId("translations");
  await expect(tr.getByTestId("tr-summary")).toHaveText(
    "cy (needing review: 1)",
  );
  const own = tr.getByRole("form", { name: "The roadmap's own fields" });
  await expect(own.getByLabel(/^Title/)).toHaveValue("Taliadau");
  await expect(own).toContainText("its source changed since: check it");
  await own.getByRole("button", { name: "Save the translation" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await own.getByLabel(/^Vision/).fill(" Talu mewn tap ");
  await own.getByRole("button", { name: "Save the translation" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved the translation into cy.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap/translations/cy`,
    body: { vision: "Talu mewn tap" },
  });

  await tr.getByText("Item: Tap to pay").click();
  const it = tr.getByRole("form", { name: "Item: Tap to pay" });
  await it.getByLabel(/^Tags/).fill("cardiau, arian");
  await it.getByRole("button", { name: "Save the translation" }).click();
  await expect(tr.getByTestId("tr-problems")).toHaveText(
    /give as many tags as the source has/,
  );
  await it.getByLabel(/^Tags/).fill("cardiau");
  await it.getByRole("button", { name: "Save the translation" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved the translation into cy.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap/items/a/translations/cy`,
    body: { tags: ["cardiau"] },
  });

  await tr.getByLabel(/^Language code/).fill("not a code");
  await own.getByRole("button", { name: "Save the translation" }).click();
  await expect(page.getByTestId("roadmap-error")).toHaveText(
    "Give a language code such as cy or zh-cn.",
  );
});

test("an item's details are indented lines and its organizations are picked from the active ones; only what changed is sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.route("**/api/organizations", (r) =>
    r.fulfill({
      json: {
        organizations: [
          {
            pid: "o1",
            name: "Swansea Bay University Health Board",
            kind: "delivery",
            active: true,
            audience: "org:o1",
            short_name: "Swansea Bay",
            translations: {},
          },
          {
            pid: "o2",
            name: "Old Partner",
            kind: "partner",
            active: false,
            audience: "org:o2",
            translations: {},
          },
          {
            pid: "o3",
            name: "Digital Health and Care Wales",
            kind: "delivery",
            active: true,
            audience: "org:o3",
            national: true,
            translations: {},
          },
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const tap = page
    .getByTestId("placement-now")
    .getByRole("listitem")
    .filter({ hasText: "Tap to pay" })
    .first();
  await tap.getByRole("button", { name: "Change" }).click();
  const form = page.getByRole("form", { name: "Change “Tap to pay”" });
  const who = form.getByRole("group", { name: "Who it is for (optional)" });
  await expect(who.getByRole("checkbox")).toHaveCount(2);
  await expect(who.getByText("Old Partner")).toHaveCount(0);
  await form.getByLabel(/^Details/).fill("  Cards");
  await form.getByRole("button", { name: "Save the item" }).click();
  await expect(page.getByTestId("roadmap-problems")).toHaveText(
    /^The first detail line is at the top/,
  );
  await form.getByLabel(/^Details/).fill("Cards\n  Debit\n  Credit");
  await who
    .getByRole("checkbox", { name: "Swansea Bay University Health Board" })
    .check();
  await form.getByRole("button", { name: "Save the item" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Saved “Tap to pay”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/roadmap/items/a`,
    body: {
      details: [
        { text: "Cards", level: 0 },
        { text: "Debit", level: 1 },
        { text: "Credit", level: 1 },
      ],
      organization_pids: ["o1"],
    },
  });
  await expect(tap.getByTestId("details-a").getByRole("listitem")).toHaveText([
    "Cards",
    "Debit",
    "Credit",
  ]);
  await expect(tap).toContainText("For: Swansea Bay University Health Board");
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("edits gathered into one pending change set are shown, kept through an ended session, and published together (RE-2, RE-13)", async ({
  page,
}) => {
  const sent = await stub(page, { publish401: 1 });
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const pending = page.getByTestId("pending");
  await pending
    .getByRole("checkbox", { name: "Gather my edits into one change set" })
    .check();

  const own = page.getByRole("form", { name: "The roadmap", exact: true });
  await own.getByLabel(/^Address/).fill("pay-now");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-problems")).toHaveText(
    "The address is not part of a change set: save it with gathering off.",
  );
  await own.getByLabel(/^Address/).fill("payments");
  await own.getByLabel(/^Title/).fill("Payments now");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Gathered; 1 pending.",
  );
  await expect(own.getByLabel(/^Title/)).toHaveValue("Payments now");

  const tap = page
    .getByTestId("placement-now")
    .getByRole("listitem")
    .filter({ hasText: "Tap to pay" })
    .first();
  await tap.getByRole("button", { name: "Change" }).click();
  const form = page.getByRole("form", { name: "Change “Tap to pay”" });
  await form.getByLabel(/^Phase/).fill("Beta");
  await form.getByRole("button", { name: "Save the item" }).click();
  await expect(tap).toContainText("Phase: Beta");

  const add = page.getByRole("form", { name: "Add an item" });
  await add.getByLabel(/^Title/).fill("Instalments");
  await add.getByRole("button", { name: "Add the item" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Gathered; 3 pending.",
  );
  await expect(
    pending.getByTestId("pending-lines").getByRole("listitem"),
  ).toHaveText([
    "Title: from Payments to Payments now",
    "“Tap to pay”, Phase: from none to Beta",
    "A new item: “Instalments”",
  ]);
  await expect(pending.getByRole("checkbox")).toBeDisabled();
  expect(sent.filter((s) => s.method !== "GET")).toEqual([]);

  // Leaving by a link asks first; staying keeps everything.
  page.once("dialog", (d) => void d.dismiss());
  await page.getByRole("link", { name: /back/i }).first().click();
  await expect(pending.getByTestId("pending-count")).toHaveText("Pending: 3");

  await pending
    .getByRole("button", { name: "Preview the pending changes" })
    .click();
  const preview = page.getByTestId("publish-preview");
  await expect(
    preview.getByTestId("preview-changes").getByRole("listitem"),
  ).toHaveCount(3);
  await preview.getByRole("button", { name: "Publish the change set" }).click();
  await expect(
    page.getByTestId("roadmap-error").getByRole("link", { name: /sign in/i }),
  ).toBeVisible();
  await expect(pending.getByTestId("pending-count")).toHaveText("Pending: 3");
  await expect(page.getByText("service text")).toHaveCount(0);

  await preview.getByRole("button", { name: "Publish the change set" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Published the change set.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/roadmap/publish`,
    body: {
      roadmap: { title: { base: "Payments", value: "Payments now" } },
      items: [{ pid: "a", fields: { phase: { base: null, value: "Beta" } } }],
      new: [{ title: "Instalments", placement: "now" }],
    },
  });
  await expect(pending.getByTestId("pending-count")).toHaveCount(0);
  await expect(page.getByTestId("placement-now")).toContainText("Instalments");
  await expect(pending.getByRole("checkbox")).toBeEnabled();
});

test("gathered edits are discarded only after a question", async ({ page }) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/roadmap`);
  await settle(page);
  const pending = page.getByTestId("pending");
  await pending
    .getByRole("checkbox", { name: "Gather my edits into one change set" })
    .check();
  const own = page.getByRole("form", { name: "The roadmap", exact: true });
  await own.getByLabel(/^Vision/).fill("Pay by phone");
  await own.getByRole("button", { name: "Save the roadmap" }).click();
  await expect(pending.getByTestId("pending-count")).toHaveText("Pending: 1");
  const ask = page.getByRole("alertdialog");
  await pending
    .getByRole("button", { name: "Discard the pending changes" })
    .click();
  await expect(ask).toContainText("Discard all 1 pending changes?");
  await ask.getByRole("button", { name: "Cancel" }).click();
  await expect(pending.getByTestId("pending-count")).toHaveText("Pending: 1");
  await pending
    .getByRole("button", { name: "Discard the pending changes" })
    .click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("roadmap-notice")).toHaveText(
    "Discarded the pending changes.",
  );
  await expect(own.getByLabel(/^Vision/)).toHaveValue("Pay in a tap");
});
