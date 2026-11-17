// NC-1 slice 11 (DP-2): a plan's delivery confidence, in a real browser over a stubbed service. The plan page links to
// it; "not assessed" is said as such, never as green; an assessment is added with every problem said first; amber/red
// warns that it opens an exception and the answer says when it is due; the history is newest first; an independent
// assessment without the right is refused in the catalogue's words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

async function stubConfidence(
  page: Page,
  opts: { refuse?: number; empty?: boolean } = {},
) {
  const history: Array<Record<string, unknown>> = opts.empty
    ? []
    : [
        {
          pid: "c1",
          plan_pid: PID,
          value: "green",
          kind: "self",
          rationale: "On track for the October release.",
          assessor: "person:1",
          assessed_on: "2026-10-01",
        },
      ];
  const sent: unknown[] = [];
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/delivery-confidence`, async (route) => {
    const req = route.request();
    if (req.method() === "GET") {
      const last = history.at(-1);
      return route.fulfill({
        json: {
          plan_pid: PID,
          shown: last
            ? {
                value: last.value,
                kind: last.kind,
                assessed_on: last.assessed_on,
                stale: true,
                marked_self: last.kind === "self",
              }
            : { value: "not_assessed" },
          history,
        },
      });
    }
    const body = req.postDataJSON();
    sent.push(body);
    if (opts.refuse)
      return route.fulfill({
        status: opts.refuse,
        json: { error: "forbidden", description: "service text" },
      });
    const row = {
      pid: `c${history.length + 1}`,
      plan_pid: PID,
      assessor: "person:1",
      assessed_on: "2026-11-17",
      ...body,
    };
    history.push(row);
    const opens = body.value === "amber_red" || body.value === "red";
    return route.fulfill({
      json: {
        assessment: row,
        exception: opens ? { pid: "e1", window_hours: 48 } : null,
      },
    });
  });
  return sent;
}

test("the plan page leads to its confidence; not assessed is said as such, never as green", async ({
  page,
}) => {
  await stubConfidence(page, { empty: true });
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Delivery confidence" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Delivery confidence" }),
  ).toBeVisible();
  const shown = page.getByRole("region", {
    name: "What the portfolio map shows",
  });
  await expect(shown.getByText("not assessed", { exact: true })).toBeVisible();
  await expect(shown.getByText(/That is not the same as green/)).toBeVisible();
  await expect(
    page.getByText("No assessment has been stated yet."),
  ).toBeVisible();
});

test("an assessment is added after every problem is said; amber/red warns and the answer says when it is due", async ({
  page,
}) => {
  const sent = await stubConfidence(page);
  await page.goto(`/en-001/plans/${PID}/confidence`);
  await settle(page);
  const form = page.getByRole("region", { name: "Add an assessment" });
  await form.getByLabel("Why (at least 10 characters)").fill("too short");
  await form.getByRole("button", { name: "Add the assessment" }).click();
  await expect(form.getByRole("alert")).toContainText("Choose a value.");
  await expect(form.getByRole("alert")).toContainText(
    "Say why, in at least 10 characters.",
  );
  expect(sent).toEqual([]);

  await form.getByLabel("Delivery confidence").selectOption("amber_red");
  await expect(form.getByRole("note")).toContainText("opens an exception");
  await form
    .getByLabel("Why (at least 10 characters)")
    .fill("  Two suppliers are a month late.  ");
  await form.getByRole("button", { name: "Add the assessment" }).click();
  await expect(form.getByRole("status")).toHaveText(
    "Added. This plan now has an open exception: it is to be completed within 48 hours.",
  );
  expect(sent).toEqual([
    {
      value: "amber_red",
      kind: "self",
      rationale: "Two suppliers are a month late.",
    },
  ]);

  const history = page.getByRole("region", { name: "History, newest first" });
  await expect(history.getByRole("listitem").first()).toContainText(
    "amber/red",
  );
  await expect(history.getByRole("listitem").first()).toContainText(
    "Two suppliers are a month late.",
  );
  await expect(history.getByRole("listitem").nth(1)).toContainText(
    "On track for the October release.",
  );
});

test("an independent assessment without the right is refused in the catalogue's words", async ({
  page,
}) => {
  const sent = await stubConfidence(page, { refuse: 403 });
  await page.goto(`/en-001/plans/${PID}/confidence`);
  await settle(page);
  const form = page.getByRole("region", { name: "Add an assessment" });
  await form.getByLabel("Delivery confidence").selectOption("amber");
  await form.getByRole("radio", { name: "An independent assessment" }).check();
  await form
    .getByLabel("Why (at least 10 characters)")
    .fill("Reviewed the plan and its data.");
  await form.getByRole("button", { name: "Add the assessment" }).click();
  await expect(form.getByRole("alert")).toHaveText(
    "An independent assessment needs the independent assessor right. You can add a self-assessment.",
  );
  await expect(form.getByRole("alert")).not.toContainText("service text");
  expect(sent).toEqual([
    {
      value: "amber",
      kind: "independent",
      rationale: "Reviewed the plan and its data.",
    },
  ]);
});

test("the confidence page reads in Welsh and passes axe", async ({ page }) => {
  await stubConfidence(page);
  await page.goto(`/cy-001/plans/${PID}/confidence`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Hyder cyflawni",
  );
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
