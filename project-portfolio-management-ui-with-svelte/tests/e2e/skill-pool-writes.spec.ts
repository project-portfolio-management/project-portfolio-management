// NC-1 slice 11 (DP-10): adding and changing skill pools, in a real browser over a stubbed service. A pool is added
// with its roles and skills (a taken name and a role another pool holds said first); a pool's roles are changed (only
// the changed list sent, nothing for no change); a pool is retired and restored; a 409 is said in the catalogue's
// words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle, stubPlansApi } from "./fixtures";

async function stubPools(page: Page, opts: { status?: number } = {}) {
  const pools = [
    {
      pid: "p1",
      name: "Data",
      active: true,
      roles: ["Data engineer"],
      skills: ["SQL"],
      peer_pool_ref: null,
    },
    {
      pid: "p2",
      name: "Design",
      active: true,
      roles: ["Designer"],
      skills: [],
      peer_pool_ref: null,
    },
  ];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/skill-pools/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET")
      return route.fulfill({ json: { skill_pools: pools } });
    const body = req.postDataJSON();
    sent.push({ method: req.method(), path, body });
    if (opts.status)
      return route.fulfill({
        status: opts.status,
        json: { error: "conflict", description: "service text" },
      });
    if (req.method() === "POST") {
      const row = { pid: "p3", active: true, peer_pool_ref: null, ...body };
      pools.push(row);
      return route.fulfill({ json: row });
    }
    const row = pools.find((p) => path.endsWith(p.pid))!;
    Object.assign(row, body);
    return route.fulfill({ json: row });
  });
  return sent;
}

test("a pool is added after a taken name and a role another pool holds are said", async ({
  page,
}) => {
  const sent = await stubPools(page);
  await page.goto("/en-001/skill-pools");
  await settle(page);
  const add = page.getByRole("region", { name: "Add a pool" });
  await add.getByLabel("Pool name").fill(" data ");
  await add.getByRole("button", { name: "Add the pool" }).click();
  await expect(add.getByRole("alert")).toHaveText(
    "A pool with that name already exists.",
  );
  await add.getByLabel("Pool name").fill("Delivery");
  await add.getByLabel("Roles").fill("Delivery lead, data ENGINEER");
  await add.getByRole("button", { name: "Add the pool" }).click();
  await expect(add.getByRole("alert")).toHaveText(
    "Already in another pool: data ENGINEER (in Data).",
  );
  expect(sent).toEqual([]);
  await add
    .getByLabel("Roles")
    .fill("Delivery lead,\n Scrum master , delivery lead");
  await add.getByLabel("Skills").fill("Planning");
  await add.getByRole("button", { name: "Add the pool" }).click();
  await expect(add.getByRole("status")).toHaveText("Pool Delivery added.");
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/skill-pools",
      body: {
        name: "Delivery",
        roles: ["Delivery lead", "Scrum master"],
        skills: ["Planning"],
      },
    },
  ]);
});

test("a pool's roles are changed, only the changed list sent; it is retired and restored", async ({
  page,
}) => {
  const sent = await stubPools(page);
  await page.goto("/en-001/skill-pools");
  await settle(page);
  const edit = page.getByRole("region", { name: "Change a pool" });
  await edit.getByLabel("Pool").selectOption("p1");
  await expect(edit.getByLabel("Roles")).toHaveValue("Data engineer");
  await edit.getByRole("button", { name: "Save the roles and skills" }).click();
  await expect(edit.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await edit.getByLabel("Roles").fill("Data engineer, Analyst");
  await edit.getByRole("button", { name: "Save the roles and skills" }).click();
  await expect(edit.getByRole("status")).toHaveText(
    "The roles and skills of Data are saved.",
  );
  await edit.getByRole("button", { name: "Retire the pool" }).click();
  await expect(edit.getByRole("status")).toHaveText("Pool Data retired.");
  await edit.getByRole("button", { name: "Restore the pool" }).click();
  await expect(edit.getByRole("status")).toHaveText("Pool Data restored.");
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/skill-pools/p1",
      body: { roles: ["Data engineer", "Analyst"] },
    },
    { method: "PUT", path: "/api/skill-pools/p1", body: { active: false } },
    { method: "PUT", path: "/api/skill-pools/p1", body: { active: true } },
  ]);
});

test("a 409 is said in the catalogue's words", async ({ page }) => {
  await stubPools(page, { status: 409 });
  await page.goto("/en-001/skill-pools");
  await settle(page);
  const add = page.getByRole("region", { name: "Add a pool" });
  await add.getByLabel("Pool name").fill("Delivery");
  await add.getByRole("button", { name: "Add the pool" }).click();
  await expect(add.getByRole("alert")).toHaveText(
    "A pool with that name exists, or one of these roles or skills is already in another pool.",
  );
  await expect(add.getByRole("alert")).not.toContainText("service text");
});

test("the pool forms read in Welsh and pass axe", async ({ page }) => {
  await stubPools(page);
  await page.goto("/cy-001/skill-pools");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 2, name: "Ychwanegu cronfa" }),
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
