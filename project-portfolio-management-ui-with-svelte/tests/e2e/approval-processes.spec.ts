// PL-2 in a real browser: define an approval process, start a run, follow it and finish the step that is
// yours, with the service stubbed. The service decides who may do what; here a refusal is shown as a message.

import { test, expect } from "./test";
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
  // The request goes out after the click returns; wait for it rather than race it.
  await expect
    .poll(() => sent.started)
    .toEqual({
      subject_kind: "plan",
      subject_pid: PID,
    });
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

test("a process is changed as a new version, nothing is sent unchanged, its versions read, and it is retired after a question", async ({
  page,
}) => {
  const sent: Array<{ method: string; body: unknown }> = [];
  let retired = false;
  let version = 1;
  let detail = {
    pid: "p1",
    name: "Release sign-off",
    current_version: 1,
    created_by: "u",
    steps: [{ kind: "approve", person_ref: "person:a", title: "Sign off" }],
    versions: [
      {
        version: 1,
        steps: [{ kind: "approve", person_ref: "person:a", title: "Sign off" }],
        created_by: "u",
        created_at: "2026-11-01T09:00:00Z",
      },
    ],
  };
  await page.route("**/api/processes", (r) =>
    r.fulfill({
      json: retired
        ? []
        : [{ pid: "p1", name: detail.name, current_version: version }],
    }),
  );
  await page.route("**/api/process-executions", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/processes/p1", (r) => {
    const method = r.request().method();
    if (method === "GET") return r.fulfill({ json: detail });
    sent.push({ method, body: r.request().postDataJSON() ?? null });
    if (method === "DELETE") {
      retired = true;
      return r.fulfill({ json: {} });
    }
    const b = r.request().postDataJSON() as {
      name?: string;
      steps: typeof detail.steps;
    };
    version += 1;
    detail = {
      ...detail,
      name: b.name ?? detail.name,
      current_version: version,
      steps: b.steps,
      versions: [
        ...detail.versions,
        {
          version,
          steps: b.steps,
          created_by: "u",
          created_at: "2026-11-02T09:00:00Z",
        },
      ],
    };
    return r.fulfill({ json: detail });
  });
  await page.goto("/en-001/approval-processes");
  await settle(page);
  await page.getByRole("button", { name: "Change “Release sign-off”" }).click();
  const edit = page.getByRole("form", {
    name: "Change “Release sign-off” (now version 1)",
  });
  await edit.getByRole("button", { name: "Save as a new version" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "Nothing changed, so no version was made.",
  );
  expect(sent).toEqual([]);
  await page.getByRole("button", { name: "Change “Release sign-off”" }).click();
  await edit.getByRole("button", { name: "Add a step" }).click();
  await edit.getByRole("button", { name: "Save as a new version" }).click();
  await expect(edit.getByRole("alert")).toBeVisible();
  expect(sent).toEqual([]);
  await edit
    .getByRole("group", { name: "Step 2" })
    .getByLabel(/^Person/)
    .fill("person:b");
  await edit
    .getByRole("group", { name: "Step 2" })
    .getByLabel("What it says")
    .fill("Tag the release");
  await edit.getByRole("button", { name: "Save as a new version" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Saved “Release sign-off” as version 2",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      body: {
        steps: [
          { kind: "approve", person_ref: "person:a", title: "Sign off" },
          { kind: "approve", person_ref: "person:b", title: "Tag the release" },
        ],
      },
    },
  ]);
  await page.getByRole("button", { name: "Change “Release sign-off”" }).click();
  const versions = page
    .getByTestId("process-versions")
    .getByRole("listitem")
    .filter({ has: page.locator("summary") });
  await expect(versions).toHaveCount(2);
  await expect(versions.nth(0)).toContainText("current");
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);

  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Retire “Release sign-off”" }).click();
  await expect(ask).toContainText("runs already going go on");
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByRole("status")).toHaveText(
    "Retired “Release sign-off”.",
  );
  expect(sent.at(-1)).toEqual({ method: "DELETE", body: null });
});

test("a change refused for want of the right is said in the page's words", async ({
  page,
}) => {
  await page.route("**/api/processes", (r) =>
    r.fulfill({
      json: [{ pid: "p1", name: "Release sign-off", current_version: 1 }],
    }),
  );
  await page.route("**/api/process-executions", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/processes/p1", (r) =>
    r.request().method() === "GET"
      ? r.fulfill({
          json: {
            pid: "p1",
            name: "Release sign-off",
            current_version: 1,
            created_by: "someone-else",
            steps: [
              { kind: "approve", person_ref: "person:a", title: "Sign off" },
            ],
            versions: [],
          },
        })
      : r.fulfill({
          status: 403,
          json: { code: "forbidden", detail: "service text" },
        }),
  );
  await page.goto("/en-001/approval-processes");
  await settle(page);
  await page.getByRole("button", { name: "Retire “Release sign-off”" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Only the person who defined this process, or an administrator, may change or retire it.",
  );
});
