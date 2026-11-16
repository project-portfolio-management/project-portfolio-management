// Pure helpers for the plan's tools page (`/plans/[pid]/modules`, RM-7, NC-1 slice 8): the choice being made, and what
// saving it would send. Kept out of the component so the rules are unit-tested.

import type { StringKey } from "./i18n.svelte.js";

/** The modules the service knows, in its display order, and each one's catalogue key. */
export const MODULE_LABELS: Record<string, StringKey> = {
  tasks: "p.plan_modules.module_tasks",
  time: "p.plan_modules.module_time",
  risks: "p.plan_modules.module_risks",
  financials: "p.plan_modules.module_financials",
  milestones: "p.plan_modules.module_milestones",
  governance: "p.plan_modules.module_governance",
  roadmap: "p.plan_modules.module_roadmap",
};

/** `list` with `key` turned on or off, in the order of `order` (the service's), without repeats. */
export function toggled(
  list: readonly string[],
  key: string,
  on: boolean,
  order: readonly string[],
): string[] {
  const set = new Set(list);
  if (on) set.add(key);
  else set.delete(key);
  const known = order.filter((k) => set.has(k));
  const unknown = [...set].filter((k) => !order.includes(k)).sort();
  return [...known, ...unknown];
}

/** Whether two module lists hold the same modules, in any order. */
export function sameModules(
  a: readonly string[],
  b: readonly string[],
): boolean {
  const x = new Set(a);
  const y = new Set(b);
  return x.size === y.size && [...x].every((k) => y.has(k));
}

/**
 * What saving would send. Choosing exactly the deployment's default while the plan follows the default sends nothing
 * (`null`): saving it as the plan's own would stop the plan following a later change to the default.
 */
export function modulesBody(
  chosen: readonly string[],
  current: { source: "plan" | "default"; modules: readonly string[] },
): { modules: string[] } | null {
  if (current.source === "default" && sameModules(chosen, current.modules))
    return null;
  return { modules: [...chosen] };
}
