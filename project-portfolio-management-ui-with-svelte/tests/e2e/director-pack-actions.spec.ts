// NC-1 slice 11 (DP-1, DP-12): drafting, reviewing, issuing and reissuing director packs, in a real browser over a
// stubbed service. The list drafts a pack for a month and an audience (an organization too), saying every problem
// before anything is sent; a manual draft is issued; a scheduled draft waits for its review; an issued pack is reissued
// with a change note of at least ten characters; a refusal names the right it needs. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";
import { BODY } from "../unit/fixtures/director-pack-body";

const DRAFT = "44444444-4444-4444-8444-444444444441";
const SCHEDULED = "44444444-4444-4444-8444-444444444442";
const ISSUED = "44444444-4444-4444-8444-444444444443";
const NEXT = "44444444-4444-4444-8444-444444444444";

const meta = (pid: string, over: Record<string, unknown> = {}) => ({
  pid,
  period: "2026-10",
  audience: "board",
  version: 1,
  status: "draft",
  cut_off: "2026-10-31T00:00:00Z",
  change_note: null,
  supersedes_pid: null,
  issued_at: null,
  issued_by: null,
  origin: "manual",
  reviewed: false,
  stale: false,
  covers_whole_period: true,
  ...over,
});

async function stubPacks(page: Page, opts: { refuse?: number } = {}) {
  const packs: Record<string, ReturnType<typeof meta>> = {
    [DRAFT]: meta(DRAFT),
    [SCHEDULED]: meta(SCHEDULED, {
      origin: "scheduled",
      audience: "executive",
    }),
    [ISSUED]: meta(ISSUED, {
      status: "issued",
      issued_at: "2026-11-01T09:00:00Z",
      audience: "partner",
    }),
  };
  const sent: Array<{ path: string; body: unknown }> = [];
  await page.route("**/api/organizations", (route) =>
    route.fulfill({
      json: {
        organizations: [
          {
            pid: "o1",
            name: "Northwind Trust",
            kind: "partner",
            active: true,
            audience: "org:o1",
          },
          {
            pid: "o2",
            name: "Gone Ltd",
            kind: "partner",
            active: false,
            audience: "org:o2",
          },
        ],
      },
    }),
  );
  await page.route(/\/api\/director-packs/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET") {
      if (path === "/api/director-packs")
        return route.fulfill({ json: { packs: Object.values(packs) } });
      const p = packs[path.split("/").pop()!];
      return p
        ? route.fulfill({ json: { pack: p, body: BODY } })
        : route.fulfill({ status: 404, json: {} });
    }
    sent.push({ path, body: req.postDataJSON() ?? null });
    if (opts.refuse)
      return route.fulfill({
        status: opts.refuse,
        json: { error: "x", description: "service text" },
      });
    if (path === "/api/director-packs") {
      const b = req.postDataJSON();
      packs[NEXT] = meta(NEXT, { period: b.period, audience: b.audience });
      return route.fulfill({ status: 201, json: packs[NEXT] });
    }
    const [, , , pid, action] = path.split("/");
    const p = packs[pid!]!;
    if (action === "review") p.reviewed = true;
    if (action === "issue")
      Object.assign(p, { status: "issued", issued_at: "2026-11-17T10:00:00Z" });
    if (action === "reissue") {
      packs[NEXT] = meta(NEXT, {
        version: 2,
        audience: p.audience,
        change_note: req.postDataJSON().change_note,
      });
      return route.fulfill({ status: 201, json: packs[NEXT] });
    }
    return route.fulfill({ json: p });
  });
  return sent;
}

test("the list drafts a pack, saying every problem before anything is sent", async ({
  page,
}) => {
  const sent = await stubPacks(page);
  await page.goto("/en-001/director-packs");
  await settle(page);
  const form = page.getByRole("region", { name: "Draft a pack" });
  await form.getByRole("button", { name: "Draft the pack" }).click();
  await expect(form.getByRole("alert")).toContainText(
    "Choose the period as a month.",
  );
  await expect(form.getByRole("alert")).toContainText("Choose an audience.");
  expect(sent).toEqual([]);

  // Organizations are audiences too; an inactive one is not offered.
  await expect(
    form.getByRole("option", { name: "Organization: Northwind Trust" }),
  ).toHaveCount(1);
  await expect(
    form.getByRole("option", { name: "Organization: Gone Ltd" }),
  ).toHaveCount(0);

  await form.getByLabel("Period (month)").fill("2026-11");
  await form.getByLabel("Audience").selectOption("org:o1");
  await form.getByRole("button", { name: "Draft the pack" }).click();
  await expect(page).toHaveURL(new RegExp(`/director-packs/${NEXT}$`));
  expect(sent).toEqual([
    {
      path: "/api/director-packs",
      body: { period: "2026-11", audience: "org:o1" },
    },
  ]);
});

test("a manual draft is reviewed and issued; issuing freezes it", async ({
  page,
}) => {
  const sent = await stubPacks(page);
  await page.goto(`/en-001/director-packs/${DRAFT}`);
  await settle(page);
  const actions = page.getByRole("region", {
    name: "Review, issue or reissue",
  });
  await expect(page.getByText("Not reviewed yet.")).toBeVisible();
  await actions.getByRole("button", { name: "Mark as reviewed" }).click();
  await expect(actions.getByRole("status")).toHaveText("Marked as reviewed.");
  await expect(
    page.getByText("Reviewed by the portfolio office."),
  ).toBeVisible();
  await actions.getByRole("button", { name: "Issue the pack" }).click();
  await expect(actions.getByRole("status")).toHaveText("Issued.");
  await expect(
    actions.getByRole("button", { name: "Issue the pack" }),
  ).toHaveCount(0);
  await expect(actions.getByLabel("Change note")).toBeVisible();
  expect(sent.map((s) => s.path)).toEqual([
    `/api/director-packs/${DRAFT}/review`,
    `/api/director-packs/${DRAFT}/issue`,
  ]);
});

test("a scheduled draft cannot be issued until it is reviewed", async ({
  page,
}) => {
  await stubPacks(page);
  await page.goto(`/en-001/director-packs/${SCHEDULED}`);
  await settle(page);
  const actions = page.getByRole("region", {
    name: "Review, issue or reissue",
  });
  await expect(page.getByText(/made by the monthly schedule/)).toBeVisible();
  await expect(
    actions.getByRole("button", { name: "Issue the pack" }),
  ).toBeDisabled();
  await expect(
    actions.getByText(
      /the portfolio office reviews it before it can be issued/,
    ),
  ).toBeVisible();
  await actions.getByRole("button", { name: "Mark as reviewed" }).click();
  await expect(
    actions.getByRole("button", { name: "Issue the pack" }),
  ).toBeEnabled();
});

test("an issued pack is reissued with a change note of at least ten characters", async ({
  page,
}) => {
  const sent = await stubPacks(page);
  await page.goto(`/en-001/director-packs/${ISSUED}`);
  await settle(page);
  const actions = page.getByRole("region", {
    name: "Review, issue or reissue",
  });
  await expect(
    actions.getByRole("button", { name: "Issue the pack" }),
  ).toHaveCount(0);
  await actions.getByLabel("Change note").fill("  Too few ");
  await actions
    .getByRole("button", { name: "Reissue with a change note" })
    .click();
  await expect(actions.getByRole("alert")).toHaveText(
    "Say why there is a new version, in at least 10 characters.",
  );
  expect(sent).toEqual([]);
  await actions
    .getByLabel("Change note")
    .fill("  Funding line corrected after the audit.  ");
  await actions
    .getByRole("button", { name: "Reissue with a change note" })
    .click();
  await expect(page).toHaveURL(new RegExp(`/director-packs/${NEXT}$`));
  await expect(
    page.getByText("Change note: Funding line corrected after the audit."),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      path: `/api/director-packs/${ISSUED}/reissue`,
      body: { change_note: "Funding line corrected after the audit." },
    },
  ]);
});

test("a refusal names the right the action needs, not the service's text", async ({
  page,
}) => {
  await stubPacks(page, { refuse: 403 });
  await page.goto(`/en-001/director-packs/${DRAFT}`);
  await settle(page);
  const actions = page.getByRole("region", {
    name: "Review, issue or reissue",
  });
  await actions.getByRole("button", { name: "Issue the pack" }).click();
  await expect(actions.getByRole("alert")).toHaveText(
    "Issuing a pack needs director pack issuer access.",
  );
  await actions.getByRole("button", { name: "Mark as reviewed" }).click();
  await expect(actions.getByRole("alert")).toHaveText(
    "Drafting, reviewing and reissuing a pack needs director pack editor access.",
  );
  await expect(actions.getByRole("alert")).not.toContainText("service text");

  await page.unrouteAll({ behavior: "ignoreErrors" });
  await stubPacks(page, { refuse: 409 });
  await page.goto("/en-001/director-packs");
  await settle(page);
  const form = page.getByRole("region", { name: "Draft a pack" });
  await form.getByLabel("Period (month)").fill("2026-10");
  await form.getByLabel("Audience").selectOption("board");
  await form.getByRole("button", { name: "Draft the pack" }).click();
  await expect(form.getByRole("alert")).toHaveText(
    /This period already has a pack for that audience/,
  );
});

test("the pack actions read in Welsh and pass axe", async ({ page }) => {
  await stubPacks(page);
  await page.goto(`/cy-001/director-packs/${DRAFT}`);
  await settle(page);
  await expect(
    page.getByRole("heading", { name: "Adolygu, cyhoeddi neu ailgyhoeddi" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Cyhoeddi'r pecyn" }),
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
