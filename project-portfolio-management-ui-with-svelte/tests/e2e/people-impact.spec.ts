// FB-4 in a real browser: a programme's plan page asks the workforce service for its people impact only when
// the reader presses the button, shows WPM's answer as it came (a hidden count as hidden, never zero; an unknown
// part with WPM's reason; no total), shows the reason and no figure when WPM was not asked or did not answer,
// and passes axe. The service is stubbed; no workforce service is called.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const PID = "55555555-5555-4555-8555-555555555555";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
}

const unknownPart = (reason: string) => ({
  known: false,
  reason,
  pools: null,
  in_no_pool: null,
});

const LIVE = {
  plan_pid: PID,
  programme_ref: `thing:${PID}`,
  source: "wpm",
  status: "live",
  reason: null,
  as_of: "2026-10-30",
  plan_ends_on: "2027-06-30",
  plan_ends_on_source: "target_date",
  plan_ends_on_reason: null,
  floor: 10,
  workers_committed: {
    known: true,
    reason: null,
    pools: [
      { pool_pid: "a", pool: "Nursing", count: 12, hidden: false },
      { pool_pid: "b", pool: "Pharmacy", count: null, hidden: true },
    ],
    in_no_pool: { count: 0, hidden: false },
  },
  contingent_ending_before_end: {
    ...unknownPart("WPM finds no funded post"),
    end_not_recorded: null,
  },
  over_committed_pool_months: {
    known: true,
    reason: null,
    considered: 6,
    over: 2,
    unknown: 1,
    pools: [],
  },
};

const UNKNOWN = {
  ...LIVE,
  status: "unknown",
  reason: "WPM refused this caller (403)",
  as_of: null,
  floor: null,
  workers_committed: unknownPart("WPM refused this caller (403)"),
  contingent_ending_before_end: {
    ...unknownPart("WPM refused this caller (403)"),
    end_not_recorded: null,
  },
  over_committed_pool_months: {
    known: false,
    reason: "WPM refused this caller (403)",
    considered: null,
    over: null,
    unknown: null,
    pools: null,
  },
};

async function planPage(
  page: Page,
  kind: string,
  impact: unknown,
  seen: string[],
  locale = "en-001",
) {
  await page.route("**/api/**", (r) => {
    const path = new URL(r.request().url()).pathname.replace(
      /^\/api\/proxy/,
      "",
    );
    if (path === `/api/plans/${PID}`)
      return r.fulfill({
        json: {
          pid: PID,
          kind,
          name: "Care closer to home",
          in_language: ["en"],
        },
      });
    if (path === `/api/plans/${PID}/people-impact`) {
      seen.push(path);
      return r.fulfill({ json: impact });
    }
    return r.fulfill({ status: 404, json: { error: "not_found" } });
  });
  await page.goto(`/${locale}/plans/${PID}`, { waitUntil: "networkidle" });
}

test("a programme's people impact is asked on request and shown as WPM gave it", async ({
  page,
}) => {
  const seen: string[] = [];
  await planPage(page, "Program", LIVE, seen);
  const box = page.getByTestId("people-impact");
  await expect(box.getByRole("heading", { level: 2 })).toHaveText(
    /People impact/,
  );
  expect(seen, "nothing is asked until the reader presses the button").toEqual(
    [],
  );
  await box.getByRole("button", { name: "Ask the workforce service" }).click();
  await expect(page.getByTestId("people-impact-source")).toContainText(
    "2026-10-30",
  );
  await expect(box).toContainText("End date sent: 2027-06-30");
  await expect(
    box.getByRole("listitem").filter({ hasText: "Nursing" }),
  ).toContainText("12");
  await expect(
    box.getByRole("listitem").filter({ hasText: "Pharmacy" }),
  ).toContainText("hidden (fewer than 10)");
  await expect(
    box.getByRole("listitem").filter({ hasText: "In no pool" }),
  ).toContainText("0");
  await expect(box).toContainText("Not known: WPM finds no funded post");
  await expect(box).toContainText(
    "2 of 6 pool-months are over; 1 cannot be said.",
  );
  await expect(box).toContainText("No total is given");
  await audit(page);
});

test("when WPM refuses, the reason is shown and no figure", async ({
  page,
}) => {
  await planPage(page, "Program", UNKNOWN, []);
  const box = page.getByTestId("people-impact");
  await box.getByRole("button", { name: "Ask the workforce service" }).click();
  await expect(page.getByTestId("people-impact-unknown")).toContainText(
    "WPM refused this caller (403)",
  );
  await expect(box.getByRole("listitem")).toHaveCount(0);
  await expect(box.getByRole("heading", { level: 3 })).toHaveCount(0);
  await audit(page);
});

test("a plan that is not a programme has no people-impact box; Welsh labels show", async ({
  page,
}) => {
  await planPage(page, "Project", LIVE, []);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Care closer to home",
  );
  await expect(page.getByTestId("people-impact")).toHaveCount(0);
  await page.unrouteAll();
  await planPage(page, "Program", LIVE, [], "cy-001");
  await expect(
    page.getByTestId("people-impact").getByRole("button"),
  ).toHaveText("Gofyn i'r gwasanaeth gweithlu");
});
