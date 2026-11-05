import { describe, expect, it } from "vitest";
import {
  allowedActions,
  emptyIssueForm,
  formFromIssue,
  humanize,
  issueBody,
  issueProblem,
  issueStatusLabel,
  missingFields,
  needsResolution,
  readOnlyFields,
  requiredFields,
  statusChoices,
  statusLabel,
  statusWords,
  translated,
  typeChoices,
  typeLabel,
  usableOptions,
  workflowFor,
} from "../../src/lib/issue-view";
import {
  draftFromWorkflow,
  emptyWorkflowDraft,
  labelsText,
  parseLabels,
  workflowBody,
  workflowChange,
  workflowProblem,
} from "../../src/lib/issue-settings-view";
import type {
  Issue,
  IssueOptions,
  IssueWorkflow,
} from "../../src/lib/api/ppm-types";

const ISSUE: Issue = {
  pid: "i1",
  plan_pid: "p1",
  title: "Build server is down",
  description: null,
  kind: "blocker",
  severity: "high",
  priority: null,
  status: "in_progress",
  owner_ref: "person:ada",
  raised_by: null,
  raised_on: "2026-09-10",
  due_date: "2026-09-12",
  resolution: null,
  resolved_on: null,
  age_calendar_days: 3,
  overdue: false,
};

describe("issue form", () => {
  it("needs a title", () => {
    expect(issueProblem(emptyIssueForm())).toBe("title_required");
    expect(issueProblem({ ...emptyIssueForm(), title: "   " })).toBe(
      "title_required",
    );
    expect(issueProblem({ ...emptyIssueForm(), title: "x" })).toBeNull();
  });

  it("needs a resolution to close, not to resolve", () => {
    const f = { ...emptyIssueForm(), title: "x", status: "closed" as const };
    expect(issueProblem(f)).toBe("resolution_required");
    expect(issueProblem({ ...f, resolution: "Fixed" })).toBeNull();
    expect(issueProblem({ ...f, status: "resolved" })).toBeNull();
  });

  it("refuses a due date before the day it was raised", () => {
    const f = {
      ...emptyIssueForm(),
      title: "x",
      raisedOn: "2026-09-10",
      dueDate: "2026-09-09",
    };
    expect(issueProblem(f)).toBe("due_before_raised");
    expect(issueProblem({ ...f, dueDate: "2026-09-10" })).toBeNull();
  });

  it("sends empty boxes as null and leaves raised_on out when blank", () => {
    const body = issueBody({ ...emptyIssueForm(), title: "  New  " });
    expect(body).toEqual({
      title: "New",
      description: null,
      kind: "concern",
      severity: "med",
      status: "open",
      priority: null,
      owner_ref: null,
      due_date: null,
      resolution: null,
    });
    expect("raised_on" in body).toBe(false);
  });

  it("round-trips an issue through the form", () => {
    const body = issueBody(formFromIssue(ISSUE));
    expect(body.title).toBe("Build server is down");
    expect(body.raised_on).toBe("2026-09-10");
    expect(body.owner_ref).toBe("person:ada");
    expect(body.status).toBe("in_progress");
  });
});

describe("organization-defined types and statuses (OC-1)", () => {
  const workflow: IssueWorkflow = {
    states: [
      {
        key: "new",
        label: "New",
        category: "todo",
        is_initial: true,
        is_terminal: false,
      },
      {
        key: "fixed",
        label: "Fixed",
        category: "done",
        is_initial: false,
        is_terminal: false,
      },
      {
        key: "signed_off",
        label: "Signed off",
        category: "done",
        is_initial: false,
        is_terminal: true,
      },
    ],
    transitions: [
      { from: "new", to: "fixed", roles: [] },
      { from: "fixed", to: "signed_off", roles: ["change-manager"] },
    ],
  };
  const options: IssueOptions = {
    caller_roles: [],
    types: [
      { key: "bug", label: "Bug", position: 0, retired: false, workflow },
      { key: "defect", label: "Defect", position: 1, retired: false, workflow },
      { key: "old", label: "Old", position: 2, retired: true, workflow },
    ],
  };
  const keys = (states: { key: string }[]) => states.map((s) => s.key);

  it("treats a failed or malformed answer as no options, and falls back to the built-ins", () => {
    expect(usableOptions(null)).toBeNull();
    expect(usableOptions({ issues: [] })).toBeNull();
    expect(keys(workflowFor(null, "bug").states)).toEqual([
      "open",
      "in_progress",
      "resolved",
      "closed",
    ]);
    expect(typeChoices(null, "bug").map((x) => x.key)).toContain("concern");
  });

  it("offers a retired type only to the issue that already has it", () => {
    expect(typeChoices(options, "bug").map((x) => x.key)).toEqual([
      "bug",
      "defect",
    ]);
    expect(typeChoices(options, "old").map((x) => x.key)).toEqual([
      "bug",
      "defect",
      "old",
    ]);
  });

  it("starts a new issue where a constrained workflow says", () => {
    expect(keys(statusChoices(workflow, null, []))).toEqual(["new"]);
    const free = { ...workflow, transitions: [] };
    expect(keys(statusChoices(free, null, []))).toEqual([
      "new",
      "fixed",
      "signed_off",
    ]);
  });

  it("offers only the moves this caller may make", () => {
    expect(keys(statusChoices(workflow, "new", []))).toEqual(["new", "fixed"]);
    expect(keys(statusChoices(workflow, "fixed", []))).toEqual(["fixed"]);
    expect(keys(statusChoices(workflow, "fixed", ["change-manager"]))).toEqual([
      "fixed",
      "signed_off",
    ]);
  });

  it("keeps a status the workflow no longer declares, and lets the issue move on", () => {
    expect(keys(statusChoices(workflow, "triage", []))).toEqual([
      "triage",
      "new",
      "fixed",
      "signed_off",
    ]);
  });

  it("asks for a resolution in a terminal status, whatever it is called", () => {
    expect(needsResolution(workflow, "signed_off")).toBe(true);
    expect(needsResolution(workflow, "fixed")).toBe(false);
    expect(
      issueProblem(
        { ...emptyIssueForm(), title: "x", status: "signed_off" },
        true,
      ),
    ).toBe("resolution_required");
    expect(
      issueProblem(
        { ...emptyIssueForm(), title: "x", status: "closed" },
        false,
      ),
    ).toBeNull();
  });

  it("keeps an organization's own name, and reads a bare key as words", () => {
    expect(typeLabel(options.types, "defect")).toBe("Defect");
    expect(statusLabel(workflow, "signed_off")).toBe("Signed off");
    expect(statusLabel(null, "with_supplier")).toBe("With supplier");
    expect(humanize("in-review")).toBe("In review");
  });

  it("builds a workflow body and finds what is wrong with a draft", () => {
    const d = emptyWorkflowDraft();
    expect(workflowProblem(d)).toBe("name");
    d.name = "Incidents";
    d.issueType = "defect";
    d.moves = [
      {
        id: 9,
        from: "open",
        to: "closed",
        roles: " change-manager, reviewer ,",
      },
    ];
    expect(workflowProblem(d)).toBeNull();
    const body = workflowBody(d);
    expect(body.issue_type).toBe("defect");
    expect(body.is_default).toBe(true);
    expect(body.states[0]?.is_initial).toBe(true);
    expect(body.states[1]?.is_terminal).toBe(true);
    expect(body.transitions[0]?.roles).toEqual(["change-manager", "reviewer"]);
    d.states[1] = { ...d.states[1]!, key: "open" };
    expect(workflowProblem(d)).toBe("repeated_key");
  });
});

describe("roles, field rules, plan types and translated words", () => {
  const workflow: IssueWorkflow = {
    states: [
      {
        key: "new",
        label: "New",
        labels: { cy: "Newydd" },
        category: "todo",
        is_initial: true,
        is_terminal: false,
        field_rules: [
          { field: "owner_ref", rule: "required", roles: [] },
          { field: "due_date", rule: "required", roles: ["lead"] },
          { field: "title", rule: "read_only", roles: [] },
          { field: "priority", rule: "read_only", roles: ["lead"] },
        ],
      },
      {
        key: "done",
        label: "Done",
        category: "done",
        is_initial: false,
        is_terminal: true,
      },
    ],
    transitions: [],
  };

  it("finds an organization's word by language, then falls back", () => {
    const labels = { cy: "Risg", "en-gb": "Hazard" };
    expect(translated(labels, "cy-001")).toBe("Risg");
    expect(translated(labels, "en-gb-oxendict")).toBe("Hazard");
    expect(translated(labels, "fr-001")).toBeNull();
    expect(translated(undefined, "cy")).toBeNull();
  });

  it("uses the service's words on the register, then the key as words", () => {
    expect(statusWords("with_supplier", { label: "At supplier" })).toBe(
      "At supplier",
    );
    expect(statusWords("with_supplier", null)).toBe("With supplier");
    expect(
      issueStatusLabel({
        ...ISSUE,
        status: "with_supplier",
        status_label: "At supplier",
      }),
    ).toBe("At supplier");
    expect(issueStatusLabel({ ...ISSUE, status: "with_supplier" })).toBe(
      "With supplier",
    );
  });

  it("reads which fields bind this caller", () => {
    expect(requiredFields(workflow, "new", [])).toEqual(["owner_ref"]);
    expect(requiredFields(workflow, "new", ["lead"])).toEqual([
      "owner_ref",
      "due_date",
    ]);
    expect(readOnlyFields(workflow, "new", ["lead"], false)).toEqual([
      "title",
      "priority",
    ]);
    expect(readOnlyFields(workflow, "new", [], true)).toEqual([]);
    expect(readOnlyFields(workflow, null, [], false)).toEqual([]);
    const form = { ...emptyIssueForm(), ownerRef: "person:a" };
    expect(missingFields(form, ["owner_ref", "due_date", "title"])).toEqual([
      "due_date",
    ]);
  });

  it("hides what the caller's roles do not allow, and nothing without options", () => {
    expect(allowedActions(null)).toEqual({ add: true, edit: true, move: true });
    const options: IssueOptions = {
      types: [],
      caller_roles: [],
      caller_permissions: ["issue_move"],
    };
    expect(allowedActions(options)).toEqual({
      add: false,
      edit: false,
      move: true,
    });
  });

  it("offers only what the plan offers, and keeps the issue's own type", () => {
    const options: IssueOptions = {
      types: [
        {
          key: "bug",
          label: "Bug",
          position: 0,
          retired: false,
          offered: true,
          workflow,
        },
        {
          key: "risk",
          label: "Risk",
          position: 1,
          retired: false,
          offered: false,
          workflow,
        },
      ],
      caller_roles: [],
    };
    expect(typeChoices(options, "bug").map((t) => t.key)).toEqual(["bug"]);
    expect(typeChoices(options, "risk").map((t) => t.key)).toEqual([
      "bug",
      "risk",
    ]);
  });

  it("reads and writes translations as a person types them", () => {
    expect(parseLabels("cy=Risg; ZH-CN=风险;")).toEqual({
      cy: "Risg",
      "zh-cn": "风险",
    });
    expect(parseLabels("")).toEqual({});
    for (const bad of ["Risg", "=Risg", "cy=", "not a code=Risg"]) {
      expect(parseLabels(bad)).toBeNull();
    }
    expect(labelsText({ cy: "Risg", fr: "Risque" })).toBe("cy=Risg; fr=Risque");
  });

  it("round-trips a workflow through the form to change it", () => {
    const draft = draftFromWorkflow({
      name: "Triage",
      issue_type: "bug",
      workflow: {
        ...workflow,
        transitions: [{ from: "new", to: "done", roles: ["lead", "qa"] }],
      },
    });
    expect(workflowProblem(draft)).toBeNull();
    const change = workflowChange(draft);
    expect(change.states[0]?.labels).toEqual({ cy: "Newydd" });
    expect(change.states[0]?.is_initial).toBe(true);
    expect(change.states[0]?.field_rules).toEqual(
      workflow.states[0]?.field_rules,
    );
    expect(change.transitions[0]?.roles).toEqual(["lead", "qa"]);
  });

  it("refuses a bad translation before sending, and scopes a plan's workflow", () => {
    const draft = { ...emptyWorkflowDraft(), name: "Mine" };
    const body = workflowBody(draft, "plan-1");
    expect(body.plan_pid).toBe("plan-1");
    expect(body.is_default).toBe(false);
    expect(workflowBody(draft).is_default).toBe(true);
    draft.states[0]!.labels = "nonsense";
    expect(workflowProblem(draft)).toBe("bad_translations");
  });
});
