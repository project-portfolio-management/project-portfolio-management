// RM-8 and PL-7 (NC-1 slice 12 part m) in a real browser: `/plans/[pid]/access` makes a plan private with its
// members (the whole setting, only when it changed), grants a guest read or comment access with an end, lists the
// grants live first, and revokes one after a question; every problem is said first. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const U1 = "3f2c6b7e-0000-4000-8000-000000000001";
const U2 = "3f2c6b7e-0000-4000-8000-000000000002";
const G1 = "9a1b2c3d-0000-4000-8000-0000000000a1";

async function stub(page: Page) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  let access = { plan_pid: PID, private: false, members: [] as string[] };
  let grants = [
    {
      pid: "g-old",
      guest_sub: U2,
      access: "comment",
      expires_at: null,
      granted_by: U1,
      revoked_at: "2026-11-20T10:00:00Z",
      live: false,
    },
  ];
  await stubPlansApi(page);
  await page.route(
    /\/api\/plans\/[^/]+\/(access|guests)(\/.*)?$/,
    async (route) => {
      const r = route.request();
      const path = new URL(r.url()).pathname.replace(/^\/api\/proxy/, "");
      const method = r.method();
      if (method === "GET")
        return route.fulfill({
          json: path.endsWith("/access") ? access : grants,
        });
      const body = r.postDataJSON() ?? null;
      sent.push({ method, path, body });
      if (path.endsWith("/access")) {
        const b = body as { private: boolean; members: string[] };
        access = {
          ...access,
          ...b,
          members:
            b.private && !b.members.includes(U1)
              ? [...b.members, U1]
              : b.members,
        };
      } else if (path.endsWith("/revoke")) {
        const id = path.split("/").at(-2);
        grants = grants.map((g) =>
          g.pid === id
            ? { ...g, live: false, revoked_at: "2026-11-27T12:00:00Z" }
            : g,
        );
      } else {
        const b = body as {
          guest_sub: string;
          access: string;
          expires_at?: string;
        };
        grants = [
          ...grants,
          {
            pid: "g-new",
            guest_sub: b.guest_sub,
            access: b.access,
            expires_at: b.expires_at ?? null,
            granted_by: U1,
            revoked_at: null,
            live: true,
          },
        ];
      }
      return route.fulfill({ json: {} });
    },
  );
  return sent;
}

test("a plan is made private with its members, only when the setting changed", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/access`);
  await settle(page);
  await expect(page.getByTestId("access-state")).toHaveText(/is not private/);
  const save = page.getByRole("button", { name: "Save who can see it" });
  await save.click();
  await expect(page.getByTestId("access-notice")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await page.getByRole("checkbox", { name: /^Private/ }).check();
  await page.getByLabel(/^Members/).fill(`alice\n${U2}`);
  await save.click();
  await expect(page.getByTestId("access-problems")).toHaveText(
    /Each member is a user id/,
  );
  await page.getByLabel(/^Members/).fill(`${U2.toUpperCase()}\n\n${U2}`);
  await save.click();
  await expect(page.getByTestId("access-notice")).toHaveText(
    "The plan is private now.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: `/api/plans/${PID}/access`,
      body: { private: true, members: [U2] },
    },
  ]);
  await expect(page.getByTestId("access-state")).toHaveText(/is private/);
  await expect(page.getByLabel(/^Members/)).toHaveValue(`${U2}\n${U1}`);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("a guest is granted access with an end, listed live first, and revoked after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/access`);
  await settle(page);
  const grants = page.getByTestId("grants").getByRole("listitem");
  await expect(grants).toHaveCount(1);
  await expect(grants.first()).toContainText("revoked");
  const form = page.getByRole("form", { name: "Grant a guest access" });
  await form.getByLabel("The guest's user id").fill("bob");
  await form.getByLabel(/^Ends/).fill("2020-01-01T09:00");
  await form.getByRole("button", { name: "Grant access" }).click();
  await expect(
    page.getByTestId("access-problems").getByRole("listitem"),
  ).toHaveText([
    "The guest is a user id, such as 3f2c6b7e-0000-4000-8000-000000000001.",
    "The end must be in the future.",
  ]);
  expect(sent).toEqual([]);
  await form.getByLabel("The guest's user id").fill(G1);
  await form.getByRole("combobox", { name: "Access" }).selectOption("comment");
  await form.getByLabel(/^Ends/).fill("2099-01-01T09:00");
  await form.getByRole("button", { name: "Grant access" }).click();
  await expect(page.getByTestId("access-notice")).toHaveText(
    `Granted guest ${G1} access.`,
  );
  const posted = sent.at(-1)!;
  expect(posted.path).toBe(`/api/plans/${PID}/guests`);
  expect(posted.body).toMatchObject({ guest_sub: G1, access: "comment" });
  expect(String((posted.body as { expires_at: string }).expires_at)).toMatch(
    /^2099-01-01T/,
  );
  await expect(grants).toHaveCount(2);
  await expect(grants.first()).toContainText(G1);
  await expect(grants.first()).toContainText("May read and comment");

  const ask = page.getByRole("alertdialog");
  await grants.first().getByRole("button", { name: "Revoke" }).click();
  await expect(ask).toContainText(`Revoke the access of guest ${G1}?`);
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toHaveLength(1);
  await grants.first().getByRole("button", { name: "Revoke" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("access-notice")).toHaveText(
    `Revoked the access of guest ${G1}.`,
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/guests/g-new/revoke`,
    body: null,
  });
  await expect(
    page.getByTestId("grants").getByRole("button", { name: "Revoke" }),
  ).toHaveCount(0);
});

test("the page reads in Welsh, and the plan workspace links to it", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/access`);
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Pwy sy'n gallu gweld y cynllun hwn",
    }),
  ).toBeVisible();
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await expect(
    page.getByRole("link", { name: "Who can see it" }),
  ).toHaveAttribute("href", new RegExp(`/plans/${PID}/access$`));
});
