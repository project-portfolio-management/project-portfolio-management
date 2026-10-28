// PL-2 in a real browser: define an approval process, start a run, follow it and finish the step that is
// yours, with the service stubbed. The service decides who may do what; here a refusal is shown as a message.

import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

test("a process is defined, started, followed and its to-do step completed", async ({
  page,
}) => {
  const processes = [
    { pid: "p1", name: "Release sign-off", current_version: 1 },
  ];
  let runs: unknown[] = [];
  let todoDone = false;
  const sent: Record<string, unknown> = {};
  const detail = () => ({
    pid: "r1",
    process_pid: "p1",
    process_name: "Release sign-off",
    version: 1,
    subject_kind: "plan",
    subject_pid: PID,
    status: todoDone ? "completed" : "running",
    current_step: todoDone ? 2 : 1,
    started_by: "u",
    finished_at: null,
    steps: [
      {
        index: 0,
        kind: "approve",
        person_ref: "person:a",
        title: "Sign off",
        state: "completed",
      },
      {
        index: 1,
        kind: "todo",
        person_ref: "person:b",
        title: "Tag the release",
        state: todoDone ? "completed" : "waiting",
      },
    ],
    log: [
      {
        step: 0,
        event: "step_completed",
        actor: "person:a",
        note: null,
        at: "2026-10-20T10:00:00Z",
      },
    ],
  });
  await page.route("**/api/processes", async (r) => {
    if (r.request().method() === "POST") {
      sent.created = r.request().postDataJSON();
      processes.push({
        pid: "p2",
        name: (sent.created as { name: string }).name,
        current_version: 1,
      });
      return r.fulfill({ json: processes[1] });
    }
    return r.fulfill({ json: processes });
  });
  await page.route("**/api/processes/p1/start", (r) => {
    sent.started = r.request().postDataJSON();
    runs = [detail()];
    return r.fulfill({ json: detail() });
  });
  await page.route("**/api/process-executions", (r) =>
    r.fulfill({ json: runs.length ? [detail()] : [] }),
  );
  await page.route("**/api/process-executions/r1", (r) =>
    r.fulfill({ json: detail() }),
  );
  await page.route("**/api/process-executions/r1/steps/1/complete", (r) => {
    todoDone = true;
    return r.fulfill({ json: detail() });
  });

  await page.goto("/en-001/approval-processes");
  await expect(
    page.getByRole("heading", { name: "Approval processes" }),
  ).toBeVisible();
  await expect(
    page.getByText("Release sign-off", { exact: true }),
  ).toBeVisible();
  await expect(page.getByText("You have not started any runs.")).toBeVisible();

  // A draft with a bad step is refused here, in words, before anything is sent.
  await page.getByLabel("Name").fill("Hotfix");
  await page.getByRole("button", { name: "Define the process" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "Step 1: give it a title",
  );
  expect(sent.created).toBeUndefined();
  await page.getByLabel("Person (person:… or worker:…)").fill("person:a");
  await page.getByLabel("What it says").fill("Approve the hotfix");
  await page.getByRole("button", { name: "Define the process" }).click();
  await expect(page.getByRole("status")).toContainText("Defined Hotfix.");
  expect(sent.created).toEqual({
    name: "Hotfix",
    steps: [
      { kind: "approve", person_ref: "person:a", title: "Approve the hotfix" },
    ],
  });

  // Start a run of the first process on a plan.
  await page
    .getByRole("button", { name: "Start a run of Release sign-off" })
    .click();
  await page.getByLabel("Its identifier").fill(PID);
  await page.getByRole("button", { name: "Start the run" }).click();
  expect(sent.started).toEqual({ subject_kind: "plan", subject_pid: PID });
  await expect(page.getByText("Release sign-off on plan")).toBeVisible();

  // Follow it and finish the to-do that is waiting.
  await page.getByRole("button", { name: "Show Release sign-off" }).click();
  const region = page.getByRole("region", { name: "Show Release sign-off" });
  await expect(region).toContainText("Tag the release (person:b) — waiting");
  await expect(region).toContainText("step_completed");
  await settle(page);
  const axe = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    axe.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
  await region.getByRole("button", { name: "Mark step 2 done" }).click();
  await expect(
    page.getByText("completed", { exact: true }).first(),
  ).toBeVisible();
  await expect(
    region.getByRole("button", { name: "Cancel the run" }),
  ).toHaveCount(0);
});

test("a refusal from the service is shown as a message", async ({ page }) => {
  await page.route("**/api/processes", (r) =>
    r.fulfill({
      json: [{ pid: "p1", name: "Release sign-off", current_version: 1 }],
    }),
  );
  await page.route("**/api/process-executions", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/processes/p1/start", (r) =>
    r.fulfill({
      status: 404,
      json: { error: { code: "not_found", message: "no such plan" } },
    }),
  );
  await page.goto("/en-001/approval-processes");
  await page
    .getByRole("button", { name: "Start a run of Release sign-off" })
    .click();
  await page
    .getByLabel("Its identifier")
    .fill("00000000-0000-4000-8000-000000000000");
  await page.getByRole("button", { name: "Start the run" }).click();
  await expect(page.getByRole("status")).toContainText("That did not work");
});
