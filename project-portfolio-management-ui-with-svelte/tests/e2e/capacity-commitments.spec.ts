// FB-6 (completed) and FB-5 in a real browser: the capacity commitment form (figures in the reader's own number
// format, every wrong field named before anything is sent), the outcome of the demand sent to the workforce
// service said in plain words after a save (a figure, a withdrawal at zero, a refusal, a supply commitment never
// sent), the recent sends, the ways in (heatmap, a programme's plan page), Welsh, and axe. The service is stubbed.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const POOL = "11111111-1111-4111-8111-111111111111";
const PLAN = "22222222-2222-4222-8222-222222222222";
const OTHER_PLAN = "33333333-3333-4333-8333-333333333333";
const PEER = "3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map(
        (v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
      ),
  ).toEqual([]);
}

const pathOf = (url: string) =>
  new URL(url).pathname.replace(/^\/api\/proxy/, "");

type Answer = (body: Record<string, unknown>) => {
  status: number;
  json: unknown;
};

const send = (fte_centi: number | null, outcome: string, at: string) => ({
  outcome,
  detail: "English text from the service",
  fte_centi,
  peer_event_id: `ppm:${POOL}:${PLAN}:x:${at}`,
  at,
});

async function stub(
  page: Page,
  answer: Answer,
  puts: Record<string, unknown>[],
) {
  const commitments = [
    {
      pid: "c1",
      pool_pid: POOL,
      plan_pid: PLAN,
      organization: null,
      kind: "demand",
      month: "2026-11",
      committed_fte: 0,
      delivered_fte: null,
      note: null,
      peer_demand_send: send(0, "sent", "2026-10-30T10:00:00Z"),
    },
    {
      pid: "c2",
      pool_pid: POOL,
      plan_pid: PLAN,
      organization: null,
      kind: "demand",
      month: "2026-12",
      committed_fte: 1.5,
      delivered_fte: null,
      note: null,
      peer_demand_send: send(150, "unreachable", "2026-10-30T09:00:00Z"),
    },
    {
      pid: "c3",
      pool_pid: POOL,
      plan_pid: PLAN,
      organization: null,
      kind: "supply",
      month: "2026-11",
      committed_fte: 2,
      delivered_fte: null,
      note: null,
    },
  ];
  await page.route("**/api/**", (r) => {
    const path = pathOf(r.request().url());
    const method = r.request().method();
    if (path === "/api/skill-pools")
      return r.fulfill({
        json: {
          skill_pools: [
            {
              pid: POOL,
              name: "Nursing",
              active: true,
              roles: ["Nurse"],
              skills: [],
              peer_pool_ref: PEER,
            },
            {
              pid: "retired",
              name: "Old pool",
              active: false,
              roles: [],
              skills: [],
              peer_pool_ref: null,
            },
          ],
        },
      });
    if (path === "/api/plans")
      return r.fulfill({
        json: [
          { pid: PLAN, name: "Care programme" },
          { pid: OTHER_PLAN, name: "Estates" },
        ],
      });
    if (path === `/api/plans/${PLAN}`)
      return r.fulfill({
        json: { pid: PLAN, name: "Care programme", kind: "Program" },
      });
    if (path === "/api/capacity-commitments" && method === "GET")
      return r.fulfill({ json: { commitments } });
    if (path === "/api/capacity-commitments" && method === "PUT") {
      const body = r.request().postDataJSON() as Record<string, unknown>;
      puts.push(body);
      const a = answer(body);
      return r.fulfill({ status: a.status, json: a.json });
    }
    return r.fulfill({ status: 404, json: { error: "not_found" } });
  });
}

const saved = (
  body: Record<string, unknown>,
  extra: Record<string, unknown> = {},
) => ({
  status: 200,
  json: {
    pid: "new",
    organization: null,
    delivered_fte: null,
    note: null,
    ...body,
    ...extra,
  },
});

test("a demand commitment is checked, sent in the reader's number format, and its send said in plain words", async ({
  page,
}) => {
  const puts: Record<string, unknown>[] = [];
  await stub(
    page,
    (b) =>
      saved(b, {
        peer_demand_send: {
          outcome: "started",
          detail: "x",
          fte_centi: 150,
          peer_event_id: "e",
        },
      }),
    puts,
  );
  await page.goto(`/en-001/capacity-commitments?plan=${PLAN}`, {
    waitUntil: "networkidle",
  });
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Capacity commitments",
  );
  await expect(page.getByLabel("Programme", { exact: true })).toHaveValue(PLAN);
  await expect(
    page.getByLabel("Skill pool", { exact: true }),
  ).not.toContainText("Old pool");
  await audit(page);

  // Nothing is sent while a field is wrong.
  await page.getByLabel("Committed (full-time equivalent)").fill("lots");
  await page.getByRole("button", { name: "Save the commitment" }).click();
  await expect(page.getByText("Choose a skill pool.")).toBeVisible();
  await expect(page.getByText("A number from 0 to 1000").first()).toBeVisible();
  await expect(
    page.getByLabel("Committed (full-time equivalent)"),
  ).toHaveAttribute("aria-invalid", "true");
  expect(puts).toEqual([]);
  await audit(page);

  await page.getByLabel("Skill pool", { exact: true }).selectOption(POOL);
  await page.getByLabel("Month", { exact: true }).fill("2026-12");
  await page.getByLabel("Committed (full-time equivalent)").fill("1.5");
  await page.getByLabel("Partner organization (optional)").fill("Acme");
  await page.getByRole("button", { name: "Save the commitment" }).click();
  await expect(page.getByTestId("send-outcome")).toContainText(
    "Sending 1.5 full-time equivalent (the programme's total for this pool and month) to the workforce service",
  );
  expect(puts).toEqual([
    {
      pool_pid: POOL,
      plan_pid: PLAN,
      kind: "demand",
      month: "2026-12",
      committed_fte: 1.5,
      organization: "Acme",
    },
  ]);

  // The recent sends, newest first, in plain words; the service's English detail is not shown.
  const table = page.locator('section[aria-labelledby="sends-h"]');
  await expect(table.getByRole("row").nth(1)).toContainText(
    "a withdrawal (zero)",
  );
  await expect(table).toContainText(
    "The workforce service withdrew its figure for this programme, pool and month.",
  );
  await expect(table).toContainText("could not be reached");
  await expect(page.locator("body")).not.toContainText(
    "English text from the service",
  );
  await audit(page);
});

test("a zero is the withdrawal, a refusal is said plainly, and a supply commitment is never sent", async ({
  page,
}) => {
  const puts: Record<string, unknown>[] = [];
  let next: Answer = (b) =>
    saved(b, {
      peer_demand_send: {
        outcome: "started",
        detail: "x",
        fte_centi: 0,
        peer_event_id: "e",
      },
    });
  await stub(page, (b) => next(b), puts);
  await page.goto(`/en-001/capacity-commitments`, { waitUntil: "networkidle" });
  await page.getByLabel("Skill pool", { exact: true }).selectOption(POOL);
  await page.getByLabel("Programme", { exact: true }).selectOption(PLAN);
  await page.getByLabel("Committed (full-time equivalent)").fill("0");
  await page.getByRole("button", { name: "Save the commitment" }).click();
  await expect(page.getByTestId("send-outcome")).toContainText(
    "Withdrawing this programme's figure for this pool and month from the workforce service",
  );
  expect(puts.at(-1)?.committed_fte).toBe(0);

  next = (b) => saved(b);
  await page.getByLabel("Supply: capacity promised to the programme").check();
  await page.getByLabel("Committed (full-time equivalent)").fill("2");
  await page.getByRole("button", { name: "Save the commitment" }).click();
  await expect(page.getByTestId("commit-done")).toContainText("Saved");
  await expect(page.getByTestId("send-outcome")).toHaveCount(0);
  expect(puts.at(-1)?.kind).toBe("supply");

  next = () => ({
    status: 422,
    json: {
      error: "unprocessable_entity",
      description: "a commitment is to a programme",
    },
  });
  await page.getByLabel("Programme", { exact: true }).selectOption(OTHER_PLAN);
  await page.getByRole("button", { name: "Save the commitment" }).click();
  await expect(page.getByTestId("commit-failure")).toContainText(
    "must be a programme",
  );
  await expect(page.getByTestId("commit-failure")).not.toContainText(
    "a commitment is to a programme",
  );
  await audit(page);
});

test("the form is reached from the heatmap and a programme's plan page, and reads in Welsh", async ({
  page,
}) => {
  const puts: Record<string, unknown>[] = [];
  await stub(
    page,
    (b) =>
      saved(b, {
        peer_demand_send: {
          outcome: "sent",
          detail: "x",
          fte_centi: 0,
          peer_event_id: "e",
        },
      }),
    puts,
  );
  await page.goto(`/en-001/plans/${PLAN}`, { waitUntil: "networkidle" });
  await expect(
    page.locator(`a[href="/en-001/capacity-commitments?plan=${PLAN}"]`),
  ).toBeVisible();
  await page.goto(`/en-001/capacity-heatmap`, { waitUntil: "networkidle" });
  await page.getByRole("link", { name: "Set a capacity commitment" }).click();
  await expect(page).toHaveURL(/\/en-001\/capacity-commitments$/);

  await page.goto(`/cy-001/capacity-commitments`, { waitUntil: "networkidle" });
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Ymrwymiadau capasiti",
  );
  await expect(
    page.locator('a[href="/cy-001/capacity-heatmap"]').first(),
  ).toBeAttached();
  await page.locator("#c-pool").selectOption(POOL);
  await page.locator("#c-plan").selectOption(PLAN);
  await page.locator("#c-committed").fill("0");
  await page.locator("form button.primary").click();
  await expect(page.getByTestId("send-outcome")).toContainText(
    "gwasanaeth gweithlu",
  );
  await expect(page.getByTestId("send-outcome")).not.toContainText("withdrew");
  await audit(page);
});
