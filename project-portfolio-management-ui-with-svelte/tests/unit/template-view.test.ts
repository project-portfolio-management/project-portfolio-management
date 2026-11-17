// NC-1 slice 9 (LT-2): the plan template pages' rules: the outline, and the checks the service makes before saving a
// template or starting plans from one.
import { describe, expect, it } from "vitest";
import {
  PLAN_NAME_MAX,
  TEMPLATE_NAME_MAX,
  instantiateBody,
  isDay,
  nameProblem,
  outline,
} from "../../src/lib/template-view";
import type { TemplateNode } from "../../src/lib/api/ppm-types";

const node = (
  key: number,
  parent: number | null,
  name: string,
  tasks = 0,
): TemplateNode => ({
  key,
  parent,
  plan: { name },
  start_offset_days: null,
  target_offset_days: null,
  components: [],
  tasks: Array.from({ length: tasks }, (_, i) => ({
    key: i,
    title: `t${i}`,
    description: null,
    points: null,
    flow_type: null,
    due_offset_days: null,
    component: null,
    parent: null,
  })),
  milestones: [],
});

describe("outline", () => {
  it("keeps the service's order and gives each plan its depth", () => {
    expect(
      outline([
        node(0, null, "Root", 2),
        node(1, 0, "Child"),
        node(2, 1, "Grandchild", 1),
        node(3, 0, "Second"),
      ]).map((r) => [r.name, r.depth, r.tasks]),
    ).toEqual([
      ["Root", 0, 2],
      ["Child", 1, 0],
      ["Grandchild", 2, 1],
      ["Second", 1, 0],
    ]);
  });
  it("puts a plan whose parent is missing at the top rather than losing it", () => {
    expect(outline([node(5, 9, "Orphan")])[0]!.depth).toBe(0);
  });
});

describe("a template's name", () => {
  it("is needed, and at most 120 characters, counted as characters", () => {
    expect(nameProblem("  ")).toBe("p.plan_templates.problem_name_blank");
    expect(nameProblem("x".repeat(TEMPLATE_NAME_MAX + 1))).toBe(
      "p.plan_templates.problem_name_long",
    );
    expect(nameProblem("😀".repeat(TEMPLATE_NAME_MAX))).toBeNull();
  });
});

describe("starting plans from a template", () => {
  const form = { name: "", startDate: "", parentPid: "" };
  it("sends nothing it does not need: no blank name, no start date for a template without dates, no parent", () => {
    expect(instantiateBody(form, false)).toEqual({ body: {} });
    expect(
      instantiateBody({ ...form, startDate: "2027-01-04" }, false),
    ).toEqual({ body: {} });
    expect(
      instantiateBody(
        { name: " Next ", startDate: "", parentPid: "p1" },
        false,
      ),
    ).toEqual({
      body: { name: "Next", parent_pid: "p1" },
    });
  });
  it("needs a real day as the start date when the template carries dates", () => {
    expect(instantiateBody(form, true)).toEqual({
      problem: "p.plan_templates.problem_start_needed",
    });
    expect(instantiateBody({ ...form, startDate: "2027-02-30" }, true)).toEqual(
      {
        problem: "p.plan_templates.problem_start_bad",
      },
    );
    expect(instantiateBody({ ...form, startDate: "2027-01-04" }, true)).toEqual(
      {
        body: { start_date: "2027-01-04" },
      },
    );
  });
  it("refuses a plan name longer than the service keeps", () => {
    expect(
      instantiateBody({ ...form, name: "x".repeat(PLAN_NAME_MAX + 1) }, false),
    ).toEqual({
      problem: "p.plan_templates.problem_plan_name_long",
    });
  });
  it("reads a day only as YYYY-MM-DD on the calendar", () => {
    expect(isDay("2028-02-29")).toBe(true);
    expect(isDay("2027-02-29")).toBe(false);
    expect(isDay("4 Jan 2027")).toBe(false);
  });
});
