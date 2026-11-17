// Pure helpers for the plan template pages (`/plan-templates`, `/plan-templates/[pid]`, `/plans/[pid]/save-as-template`;
// LT-2, NC-1 slice 9): the template's plans as an outline, and the checks the service makes before saving a template
// or starting plans from one, so a refusal is said before anything is sent.

import type { StringKey } from "./i18n.svelte.js";
import type { InstantiateTemplateBody, TemplateNode } from "./api/ppm-types.js";

/** The service's limit on a template's name (`src/controllers/plan_templates.rs`). */
export const TEMPLATE_NAME_MAX = 120;
/** The service's limit on a plan's name (`MAX_TEXT_LEN` in its `src/validation.rs`). */
export const PLAN_NAME_MAX = 1024;

/** One plan of a template's outline: its depth under the template's top plan, and what it carries. */
export interface OutlineRow {
  key: number;
  name: string;
  depth: number;
  tasks: number;
  milestones: TemplateNode["milestones"];
}

/**
 * The template's plans in the service's order (parents before children), each with its depth. A plan whose parent is
 * not in the template is put at the top rather than lost.
 */
export function outline(nodes: readonly TemplateNode[]): OutlineRow[] {
  const depth = new Map<number, number>();
  return nodes.map((n) => {
    const d =
      n.parent !== null && depth.has(n.parent) ? depth.get(n.parent)! + 1 : 0;
    depth.set(n.key, d);
    return {
      key: n.key,
      name: n.plan.name,
      depth: d,
      tasks: n.tasks.length,
      milestones: n.milestones,
    };
  });
}

/** The problem with a template name, or `null`: blank, or longer than the service keeps. */
export function nameProblem(name: string): StringKey | null {
  const n = name.trim();
  if (n === "") return "p.plan_templates.problem_name_blank";
  if ([...n].length > TEMPLATE_NAME_MAX)
    return "p.plan_templates.problem_name_long";
  return null;
}

/** Whether `s` is a calendar day written `YYYY-MM-DD`. */
export function isDay(s: string): boolean {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(s)) return false;
  const d = new Date(`${s}T00:00:00Z`);
  return !Number.isNaN(d.getTime()) && d.toISOString().slice(0, 10) === s;
}

/**
 * The body to start plans from a template, or the problem that stops it. A blank name is left out (the service then
 * uses the template's own); a start date is required when the template carries dates, and sent only then; no parent
 * means a new top-level plan.
 */
export function instantiateBody(
  form: { name: string; startDate: string; parentPid: string },
  needsStartDate: boolean,
): { body: InstantiateTemplateBody } | { problem: StringKey } {
  const name = form.name.trim();
  if ([...name].length > PLAN_NAME_MAX)
    return { problem: "p.plan_templates.problem_plan_name_long" };
  if (needsStartDate && form.startDate === "")
    return { problem: "p.plan_templates.problem_start_needed" };
  if (needsStartDate && !isDay(form.startDate))
    return { problem: "p.plan_templates.problem_start_bad" };
  return {
    body: {
      ...(name ? { name } : {}),
      ...(needsStartDate ? { start_date: form.startDate } : {}),
      ...(form.parentPid ? { parent_pid: form.parentPid } : {}),
    },
  };
}
