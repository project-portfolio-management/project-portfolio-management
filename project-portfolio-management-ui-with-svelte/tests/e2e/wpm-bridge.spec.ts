// FB-6 in a real browser: the skill-pool link to the workforce service (set, refused as already linked, checked
// before sending, cleared after a confirmation) and the capacity heatmap's readings for a linked pool (live, last
// received with its age, hidden, unknown with the reason, never zero; what was entered here beside, not added), in
// English and Welsh, with axe. The service is stubbed; no workforce service is called.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const POOL = "11111111-1111-4111-8111-111111111111";
const OTHER = "22222222-2222-4222-8222-222222222222";
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

interface Put {
  pid: string;
  body: unknown;
}

async function poolsPage(
  page: Page,
  answer: (put: Put) => { status: number; json: unknown },
  puts: Put[],
  locale = "en-001",
) {
  const pools = [
    {
      pid: POOL,
      name: "Nursing",
      active: true,
      roles: ["Nurse"],
      skills: [],
      peer_pool_ref: null as string | null,
    },
    {
      pid: OTHER,
      name: "Pharmacy",
      active: true,
      roles: ["Pharmacist"],
      skills: ["dispensing"],
      peer_pool_ref: PEER as string | null,
    },
  ];
  await page.route("**/api/**", (r) => {
    const path = pathOf(r.request().url());
    if (path === "/api/skill-pools")
      return r.fulfill({ json: { skill_pools: pools } });
    const m = /^\/api\/skill-pools\/([^/]+)\/peer-link$/.exec(path);
    if (m && r.request().method() === "PUT") {
      const put = { pid: m[1] ?? "", body: r.request().postDataJSON() };
      puts.push(put);
      const a = answer(put);
      if (a.status === 200) {
        const p = pools.find((x) => x.pid === put.pid);
        if (p)
          p.peer_pool_ref = (
            put.body as { peer_pool_ref: string | null }
          ).peer_pool_ref;
      }
      return r.fulfill({ status: a.status, json: a.json });
    }
    return r.fulfill({ status: 404, json: { error: "not_found" } });
  });
  await page.goto(`/${locale}/skill-pools`, { waitUntil: "networkidle" });
}

const ok = () => ({ status: 200, json: {} });

test("a pool is linked to the workforce service with an id checked first, and the rule is on screen", async ({
  page,
}) => {
  const puts: Put[] = [];
  await poolsPage(page, ok, puts);
  await expect(page.getByTestId("link-rule")).toContainText(
    "must be set on both sides",
  );
  await expect(page.getByTestId("link-rule")).toContainText(
    "never matched by name",
  );
  await expect(page.getByRole("row", { name: /Pharmacy/ })).toContainText(PEER);
  await expect(page.getByRole("row", { name: /Nursing/ })).toContainText(
    "not linked",
  );
  await audit(page);

  await page.getByRole("button", { name: "Change link for Nursing" }).click();
  const box = page.getByLabel("Workforce service pool id");
  await box.fill("Nursing");
  await box.blur();
  await expect(
    page.getByText("That is not a workforce service pool id"),
  ).toBeVisible();
  await expect(box).toHaveAttribute("aria-invalid", "true");
  await page.getByRole("button", { name: "Save the link" }).click();
  expect(puts, "an id that is not a UUID is not sent").toEqual([]);
  await box.fill(PEER.toUpperCase());
  await page.getByRole("button", { name: "Save the link" }).click();
  expect(puts, "upper case is not sent either").toEqual([]);
  await audit(page);

  await box.fill(PEER);
  await page.getByRole("button", { name: "Save the link" }).click();
  await expect(page.getByTestId("link-done")).toContainText(
    `Nursing is now linked to the workforce service pool ${PEER}`,
  );
  expect(puts).toEqual([{ pid: POOL, body: { peer_pool_ref: PEER } }]);
});

test("a workforce pool already linked here (409) and an id the service refuses (422) are said plainly", async ({
  page,
}) => {
  const puts: Put[] = [];
  let status = 409;
  await poolsPage(
    page,
    () => ({ status, json: { error: "conflict", description: "internal" } }),
    puts,
  );
  await page.getByRole("button", { name: "Change link for Nursing" }).click();
  await page.getByLabel("Workforce service pool id").fill(PEER);
  await page.getByRole("button", { name: "Save the link" }).click();
  await expect(page.getByTestId("link-failure")).toHaveText(
    "Another pool here is already linked to that workforce service pool. Remove that link first: one pool here per pool there.",
  );
  await expect(page.getByTestId("link-failure")).not.toContainText("internal");
  await audit(page);
  status = 422;
  await page.getByRole("button", { name: "Save the link" }).click();
  await expect(page.getByTestId("link-failure")).toContainText(
    "The service did not accept that id",
  );
  expect(puts).toHaveLength(2);
});

test("removing a link asks first; Cancel sends nothing, OK sends null", async ({
  page,
}) => {
  const puts: Put[] = [];
  // FB-5: the service withdraws what was sent to the old workforce service pool and lists it.
  const withdrawals = () => ({
    status: 200,
    json: {
      peer_demand_withdrawals: ["2026-11", "2026-12"].map((month) => ({
        plan_pid: POOL,
        month,
        outcome: "started",
        detail: "withdrawing the figure from the workforce service",
        fte_centi: 0,
        peer_event_id: `ppm:${OTHER}:${POOL}:${month}:1`,
      })),
    },
  });
  await poolsPage(page, withdrawals, puts);
  await page.getByRole("button", { name: "Change link for Pharmacy" }).click();
  await expect(page.getByLabel("Workforce service pool id")).toHaveValue(PEER);
  await page.getByRole("button", { name: "Remove the link" }).click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText("Remove the link between Pharmacy");
  await expect(ask).toContainText("will be withdrawn there (sent as zero)");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(puts).toEqual([]);
  await page.getByRole("button", { name: "Remove the link" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("link-done")).toContainText(
    "Pharmacy is no longer linked",
  );
  await expect(page.getByTestId("link-done")).toContainText(
    "Demand figures being withdrawn from the workforce service pool it was linked to: 2.",
  );
  expect(puts).toEqual([{ pid: OTHER, body: { peer_pool_ref: null } }]);
  await expect(page.getByRole("row", { name: /Pharmacy/ })).toContainText(
    "not linked",
  );
});

const cell = (month: string, over: Record<string, unknown>) => ({
  month,
  state: "enough",
  word: "enough",
  shape: "circle",
  colour: "green",
  demand_fte: 2,
  own_demand_fte: 2,
  partner_demand_fte: 0,
  supply_fte: 3,
  own_supply_fte: null,
  partner_supply_committed_fte: null,
  partner_supply_delivered_fte: null,
  gap_fte: 1,
  partner_delivery_short: false,
  partners: 0,
  supply_source: "here",
  ...over,
});
const unknownCell = {
  state: "supply_unknown",
  word: "supply unknown",
  shape: "question mark",
  colour: "grey",
  supply_fte: null,
  gap_fte: null,
};
const peer = (status: string, over: Record<string, unknown> = {}) => ({
  supply_source: "wpm",
  entered_here_supply_fte: 9,
  peer_supply: {
    source: "wpm",
    status,
    supply_fte: null,
    as_of: null,
    age_seconds: null,
    reason: null,
    ...over,
  },
});

const HEAT = {
  months: ["2026-11", "2026-12", "2027-01", "2027-02"],
  partner_names_shown: false,
  note: "",
  table: [],
  pools: [
    {
      pool_pid: POOL,
      name: "Nursing",
      peer_pool_ref: PEER,
      cells: [
        cell("2026-11", {
          ...peer("live", {
            supply_fte: 3,
            as_of: "2026-10-30T09:00:00Z",
            age_seconds: 5,
          }),
        }),
        cell("2026-12", {
          state: "shortfall",
          word: "shortfall",
          shape: "triangle",
          colour: "red",
          supply_fte: 1,
          gap_fte: -1,
          ...peer("last_received", {
            supply_fte: 1,
            as_of: "2026-10-30T07:00:00Z",
            age_seconds: 7200,
            reason: "WPM could not be reached",
          }),
        }),
        cell("2027-01", { ...unknownCell, ...peer("hidden") }),
        cell("2027-02", {
          ...unknownCell,
          ...peer("unknown", {
            reason: "the request carried no bearer token to forward",
          }),
        }),
      ],
    },
    {
      pool_pid: OTHER,
      name: "Pharmacy",
      peer_pool_ref: null,
      cells: ["2026-11", "2026-12", "2027-01", "2027-02"].map((m) =>
        cell(m, {}),
      ),
    },
  ],
};

async function heatPage(page: Page, asked: string[], locale = "en-001") {
  await page.route("**/api/**", (r) => {
    const u = new URL(r.request().url());
    const path = pathOf(r.request().url());
    if (path === "/api/capacity-heatmap") {
      asked.push(u.search);
      return r.fulfill({ json: HEAT });
    }
    return r.fulfill({ status: 404, json: { error: "not_found" } });
  });
  await page.goto(`/${locale}/capacity-heatmap`, { waitUntil: "networkidle" });
}

test("the heatmap shows a linked pool's supply from the workforce service: live, last received, hidden, unknown", async ({
  page,
}) => {
  const asked: string[] = [];
  await heatPage(page, asked);
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Capacity by skill pool and month",
    }),
  ).toBeVisible();
  const live = page.getByTestId(`cell-${POOL}-2026-11`);
  await expect(live).toContainText("enough");
  await expect(live).toContainText("Supply from the workforce service");
  await expect(live).toContainText("Live, taken");
  await expect(live).toContainText("Supply: 3");
  await expect(live).toContainText("Entered here: 9 (shown beside, not added)");

  const last = page.getByTestId(`cell-${POOL}-2026-12`);
  await expect(last).toContainText("▲");
  await expect(last).toContainText("shortfall");
  await expect(last).toContainText("Last received 2 hours ago");
  await expect(last).toContainText("not live now: WPM could not be reached");

  const hidden = page.getByTestId(`cell-${POOL}-2027-01`);
  await expect(hidden).toContainText("?");
  await expect(hidden).toContainText("supply unknown");
  await expect(hidden).toContainText("Supply: unknown");
  await expect(hidden).toContainText(
    "Hidden: the workforce service hides a month",
  );
  await expect(hidden).not.toContainText("Supply: 0");

  const unknown = page.getByTestId(`cell-${POOL}-2027-02`);
  await expect(unknown).toContainText("Supply: unknown");
  await expect(unknown).toContainText(
    "Unknown: the request carried no bearer token to forward",
  );
  await expect(unknown).toContainText("Entered here: 9");

  const here = page.getByTestId(`cell-${OTHER}-2026-11`);
  await expect(here).not.toContainText("workforce service");
  await expect(page.getByRole("rowheader", { name: /Nursing/ })).toContainText(
    "Linked to the workforce service",
  );
  await audit(page);

  await page.getByLabel("Number of months").selectOption("12");
  await page.getByRole("button", { name: "Show" }).click();
  await expect.poll(() => asked.at(-1)).toBe("?months=12");
});

test("both screens read in Welsh, and are reachable from the navigation", async ({
  page,
}) => {
  await heatPage(page, [], "cy-001");
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Capasiti fesul cronfa sgiliau a mis",
    }),
  ).toBeVisible();
  await expect(page.getByTestId(`cell-${POOL}-2027-02`)).toContainText(
    "Cyflenwad: anhysbys",
  );
  await expect(page.getByTestId(`cell-${POOL}-2026-11`)).toContainText(
    "Cyflenwad o'r gwasanaeth gweithlu",
  );
  await expect(
    page.locator('a[href="/cy-001/capacity-heatmap"]').first(),
  ).toBeAttached();
  await audit(page);
  await page.unrouteAll();
  await poolsPage(page, ok, [], "cy-001");
  await expect(
    page.getByRole("heading", { level: 1, name: "Cronfeydd sgiliau" }),
  ).toBeVisible();
  await expect(page.getByTestId("link-rule")).toContainText(
    "Rhaid gosod y cysylltiad ar y ddwy ochr",
  );
  await audit(page);
});
