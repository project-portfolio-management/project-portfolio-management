// NC-1 slice 9 (LT-1): strategy canvases, in a real browser over a stubbed service. The canvases list starts a
// free-standing canvas and a plan's starts one on the plan; a canvas shows every box, empty ones too; items are added,
// edited, moved, removed and turned into an idea, an objective or (on a plan's canvas) a risk, once; every problem is
// said before anything is sent; a refusal is the catalogue's sentence; removing a canvas goes back. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const FREE = "22222222-2222-4222-8222-222222222222";
const ON_PLAN = "55555555-5555-4555-8555-555555555555";
const SWOT = {
  key: "swot",
  title: "SWOT analysis",
  boxes: [
    { key: "strengths", title: "Strengths" },
    { key: "weaknesses", title: "Weaknesses" },
    { key: "opportunities", title: "Opportunities" },
    { key: "threats", title: "Threats" },
  ],
};
const TEMPLATES = [
  {
    key: "lean",
    title: "Lean Canvas",
    boxes: [{ key: "problem", title: "Problem" }],
  },
  SWOT,
];

type Item = {
  pid: string;
  canvas_pid: string;
  box_key: string;
  body: string;
  owner_ref: string | null;
  position: number;
  promoted_to_kind: string | null;
  promoted_to_pid: string | null;
  created_at: string;
  updated_at: string;
};

async function stubCanvases(
  page: Page,
  opts: { refuse?: number; load?: number } = {},
) {
  const now = "2026-11-16T10:00:00Z";
  const canvases = [
    {
      pid: FREE,
      plan_pid: null,
      template: "swot",
      title: "Market position",
      description: "Where we stand.",
      created_at: now,
      updated_at: now,
    },
    {
      pid: ON_PLAN,
      plan_pid: PID,
      template: "swot",
      title: "Replatform risks",
      description: null,
      created_at: now,
      updated_at: now,
    },
  ];
  let n = 0;
  const items: Item[] = [
    {
      pid: "a0000000-0000-4000-8000-000000000001",
      canvas_pid: FREE,
      box_key: "strengths",
      body: "Loyal customers\nin two regions",
      owner_ref: null,
      position: 0,
      promoted_to_kind: null,
      promoted_to_pid: null,
      created_at: now,
      updated_at: now,
    },
    {
      pid: "a0000000-0000-4000-8000-000000000002",
      canvas_pid: ON_PLAN,
      box_key: "threats",
      body: "Vendor ends support",
      owner_ref: null,
      position: 0,
      promoted_to_kind: null,
      promoted_to_pid: null,
      created_at: now,
      updated_at: now,
    },
  ];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/canvas/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    if (method !== "GET")
      sent.push({ method, path, body: req.postDataJSON() ?? null });
    if (path === "/api/canvas-templates")
      return route.fulfill({ json: TEMPLATES });
    if (opts.load && method === "GET")
      return route.fulfill({
        status: opts.load,
        json: { error: "x", description: "service text" },
      });
    if (opts.refuse && method !== "GET")
      return route.fulfill({
        status: opts.refuse,
        json: { error: "x", description: "service text" },
      });
    if (path === "/api/canvases" && method === "GET") {
      const plan = new URL(req.url()).searchParams.get("plan");
      return route.fulfill({
        json: canvases.filter((c) => !plan || c.plan_pid === plan),
      });
    }
    if (path === "/api/canvases" && method === "POST") {
      const b = req.postDataJSON();
      const pid = `c0000000-0000-4000-8000-00000000000${++n}`;
      canvases.push({
        pid,
        plan_pid: b.plan_pid ?? null,
        template: b.template,
        title: b.title,
        description: b.description ?? null,
        created_at: now,
        updated_at: now,
      });
      return route.fulfill({ json: { pid } });
    }
    const m = path.match(
      /^\/api\/canvases\/([^/]+)(?:\/items(?:\/([^/]+)(\/promote)?)?)?$/,
    );
    const canvas = canvases.find((c) => c.pid === m?.[1]);
    if (!m || !canvas)
      return route.fulfill({ status: 404, json: { error: "not_found" } });
    const [, cid, iid, promote] = m;
    const item = items.find((i) => i.pid === iid);
    if (!path.includes("/items")) {
      if (method === "DELETE") {
        canvases.splice(canvases.indexOf(canvas), 1);
        return route.fulfill({ json: {} });
      }
      const own = items.filter((i) => i.canvas_pid === cid);
      return route.fulfill({
        json: {
          canvas,
          boxes: (canvas.template === "swot" ? SWOT : TEMPLATES[0]!).boxes.map(
            (b) => ({
              ...b,
              count: own.filter((i) => i.box_key === b.key).length,
            }),
          ),
          items: own,
        },
      });
    }
    if (method === "POST" && !iid) {
      const b = req.postDataJSON();
      const pid = `b0000000-0000-4000-8000-00000000000${++n}`;
      items.push({
        pid,
        canvas_pid: cid!,
        box_key: b.box,
        body: b.body,
        owner_ref: null,
        position: 9,
        promoted_to_kind: null,
        promoted_to_pid: null,
        created_at: now,
        updated_at: now,
      });
      return route.fulfill({ json: { pid } });
    }
    if (!item)
      return route.fulfill({ status: 404, json: { error: "not_found" } });
    if (promote) {
      const b = req.postDataJSON();
      item.promoted_to_kind = b.target;
      item.promoted_to_pid = "d0000000-0000-4000-8000-000000000001";
      return route.fulfill({
        json: { kind: b.target, pid: item.promoted_to_pid },
      });
    }
    if (method === "PUT") {
      const b = req.postDataJSON();
      if (b.body) item.body = b.body;
      if (b.box) item.box_key = b.box;
      return route.fulfill({ json: item });
    }
    items.splice(items.indexOf(item), 1);
    return route.fulfill({ json: {} });
  });
  return sent;
}

test("the canvases list starts a free-standing canvas, saying every problem before anything is sent", async ({
  page,
}) => {
  const sent = await stubCanvases(page);
  await page.goto("/en-001/canvases");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Strategy canvases" }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Market position" }),
  ).toBeVisible();
  await expect(page.getByText("SWOT analysis · not on a plan")).toBeVisible();

  await page.getByLabel("Show").selectOption("lean");
  await expect(
    page.getByText("There are no canvases of this kind."),
  ).toBeVisible();

  await page.getByRole("button", { name: "Start the canvas" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "Give the canvas a title.",
  );
  expect(sent).toEqual([]);

  await page.getByLabel("Kind of canvas").selectOption("swot");
  await page.getByLabel("Title", { exact: true }).fill("  Next year  ");
  await page.getByRole("button", { name: "Start the canvas" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Next year" }),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/canvases",
      body: { template: "swot", title: "Next year" },
    },
  ]);
});

test("a plan's page leads to its canvases, and a new one there is on the plan", async ({
  page,
}) => {
  const sent = await stubCanvases(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Canvases on this plan" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Canvases on this plan" }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Replatform risks" }),
  ).toBeVisible();
  await expect(page.getByRole("link", { name: "Market position" })).toHaveCount(
    0,
  );
  await page.getByLabel("Title", { exact: true }).fill("Cutover");
  await page.getByRole("button", { name: "Start the canvas" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Cutover" }),
  ).toBeVisible();
  expect(sent[0]!.body).toEqual({
    template: "lean",
    title: "Cutover",
    plan_pid: PID,
  });
});

test("a canvas shows every box, and items are added, edited, moved and removed", async ({
  page,
}) => {
  const sent = await stubCanvases(page);
  await page.goto(`/en-001/canvases/${FREE}`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 2 })).toHaveText([
    "Strengths (1)",
    "Weaknesses (0)",
    "Opportunities (0)",
    "Threats (0)",
    "Remove the canvas",
  ]);
  await expect(page.getByText("Nothing here yet.")).toHaveCount(3);

  const weaknesses = page.getByRole("region", { name: /Weaknesses/ });
  await weaknesses.getByRole("button", { name: "Add: Weaknesses" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Write the item's text first.",
  );
  await weaknesses
    .getByLabel("New item in Weaknesses")
    .fill("  Slow releases ");
  await weaknesses.getByRole("button", { name: "Add: Weaknesses" }).click();
  await expect(page.getByRole("status")).toHaveText("Added.");
  await expect(weaknesses.getByText("Slow releases")).toBeVisible();
  await expect(weaknesses.getByLabel("New item in Weaknesses")).toHaveValue("");

  // Edit with no change sends nothing; a move sends only the box.
  await page.getByRole("button", { name: "Edit: Loyal customers" }).click();
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await page.getByRole("button", { name: "Edit: Loyal customers" }).click();
  await page.getByLabel("Box").selectOption("opportunities");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("status")).toHaveText("Saved.");
  await expect(
    page
      .getByRole("region", { name: /Opportunities/ })
      .getByText("Loyal customers"),
  ).toBeVisible();

  // Removing asks first; Cancel sends nothing.
  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Remove: Slow releases" }).click();
  await expect(ask).toContainText("Slow releases");
  await ask.getByRole("button", { name: "Cancel" }).click();
  await page.getByRole("button", { name: "Remove: Slow releases" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByRole("status")).toHaveText("Removed.");
  await expect(page.getByText("Slow releases")).toHaveCount(0);

  expect(
    sent.map((s) => [
      s.method,
      s.path.replace(/\/api\/canvases\/[^/]+/, "…"),
      s.body,
    ]),
  ).toEqual([
    ["POST", "…/items", { box: "weaknesses", body: "Slow releases" }],
    [
      "PUT",
      "…/items/a0000000-0000-4000-8000-000000000001",
      { box: "opportunities" },
    ],
    ["DELETE", expect.stringMatching(/^…\/items\/b0/), null],
  ]);
});

test("an item becomes an idea once; a risk only on a plan's canvas, with both scores", async ({
  page,
}) => {
  const sent = await stubCanvases(page);
  await page.goto(`/en-001/canvases/${FREE}`);
  await settle(page);
  await page
    .getByRole("button", { name: "Turn into…: Loyal customers" })
    .click();
  await expect(
    page.getByRole("radio", { name: "a risk on this plan" }),
  ).toHaveCount(0);
  await expect(
    page.getByText(
      "This canvas is on no plan, so an item cannot become a risk.",
    ),
  ).toBeVisible();
  await page.getByRole("radio", { name: "an idea" }).check();
  await page.getByRole("button", { name: "Turn it into this" }).click();
  await expect(page.getByRole("status")).toHaveText("Turned into an idea.");
  await expect(
    page.getByRole("link", { name: "Became an idea" }),
  ).toHaveAttribute("href", /\/ideas$/);
  await expect(
    page.getByRole("button", { name: "Turn into…: Loyal customers" }),
  ).toHaveCount(0);

  await page.goto(`/en-001/canvases/${ON_PLAN}`);
  await settle(page);
  await expect(
    page.getByRole("link", { name: "← Back to the canvases" }),
  ).toHaveAttribute("href", new RegExp(`/plans/${PID}/canvases$`));
  await page
    .getByRole("button", { name: "Turn into…: Vendor ends support" })
    .click();
  await page.getByRole("radio", { name: "a risk on this plan" }).check();
  await page.getByLabel("Probability (1 to 5)").fill("4");
  await page.getByRole("button", { name: "Turn it into this" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A risk needs a probability and an impact, each a whole number from 1 to 5.",
  );
  // Out of range: the page's sentence, not the browser's own bubble (the form has novalidate).
  await page.getByLabel("Probability (1 to 5)").fill("6");
  await page.getByLabel("Impact (1 to 5)").fill("5");
  await page.getByRole("button", { name: "Turn it into this" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A risk needs a probability and an impact, each a whole number from 1 to 5.",
  );
  await page.getByLabel("Probability (1 to 5)").fill("4");
  await page.getByLabel("Impact (1 to 5)").fill("5");
  await page.getByRole("button", { name: "Turn it into this" }).click();
  await expect(
    page.getByRole("link", { name: "Became a risk on this plan" }),
  ).toHaveAttribute("href", new RegExp(`/plans/${PID}/governance$`));
  expect(sent.map((s) => s.body)).toEqual([
    { target: "idea" },
    { target: "risk", probability: 4, impact: 5 },
  ]);
});

test("a refusal and a failed load are said in the catalogue's words; removing a canvas goes back", async ({
  page,
}) => {
  await stubCanvases(page, { refuse: 403 });
  await page.goto(`/en-001/canvases/${FREE}`);
  await settle(page);
  const strengths = page.getByRole("region", { name: /Strengths/ });
  await strengths.getByLabel("New item in Strengths").fill("x");
  await strengths.getByRole("button", { name: "Add: Strengths" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "You are not allowed to do that.",
  );
  await expect(page.getByRole("alert")).not.toContainText("service text");

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubCanvases(page);
  await page.reload();
  await settle(page);
  await page.getByRole("button", { name: "Remove this canvas" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Strategy canvases" }),
  ).toBeVisible();
  await expect(page.getByRole("link", { name: "Market position" })).toHaveCount(
    0,
  );

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubCanvases(page, { load: 500 });
  await page.goto(`/en-001/canvases/${FREE}`);
  await settle(page);
  await expect(page.getByRole("alert")).toHaveText(
    "This canvas could not be loaded.",
  );
});

test("a canvas reads in Welsh and passes axe", async ({ page }) => {
  await stubCanvases(page);
  await page.goto(`/cy-001/canvases/${FREE}`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 2 }).first()).toHaveText(
    "Cryfderau (1)",
  );
  await expect(
    page.getByText("Dadansoddiad SWOT · nid ar gynllun"),
  ).toBeVisible();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
