/**
 * Rules behind a plan's components page (`/plans/[pid]/components`, PL-11; NC-1 slice 12 part n). Pure; unit-tested.
 * The service's limits are said and checked before anything is sent; refusals are the catalogue's sentences.
 */

import type { ComponentBody } from "./api/ppm";

/** The longest name the service takes. */
export const NAME_MAX = 120;

/** A lead is a `person:` or `worker:` reference, held as such and never as a name. */
export function leadOk(s: string): boolean {
  return /^(person|worker):\S+$/.test(s.trim());
}

/** The body to send, or the catalogue key of the first problem. A blank lead clears it. */
export function componentBody(
  name: string,
  lead: string,
): { body: ComponentBody } | { problem: string } {
  const n = name.trim();
  if (!n) return { problem: "p.components.name_required" };
  if ([...n].length > NAME_MAX)
    return { problem: "p.components.name_too_long" };
  const l = lead.trim();
  if (l && !leadOk(l)) return { problem: "p.components.lead_bad" };
  return { body: { name: n, lead_ref: l || null } };
}

/** Whether a name is already another component's in the plan, ignoring case (the service refuses it). */
export function nameTaken(
  name: string,
  others: readonly { pid: string; name: string }[],
  self?: string,
): boolean {
  const n = name.trim().toLowerCase();
  return others.some((c) => c.pid !== self && c.name.toLowerCase() === n);
}

/** The catalogue key for a refusal, by action and status. */
export function refusalKey(
  action: "save" | "remove",
  status: number | undefined,
): string {
  if (action === "save" && status === 409) return "p.components.taken";
  if (action === "save" && status === 422) return "p.components.too_many";
  if (action === "remove" && status === 409)
    return "p.components.in_use_refused";
  return `p.components.could_not_${action}`;
}
