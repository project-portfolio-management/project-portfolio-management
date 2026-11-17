// Pure helpers for the strategy canvas pages (`/canvases`, `/plans/[pid]/canvases`, `/canvases/[pid]`; LT-1, NC-1
// slice 9): the names of the built-in templates and their boxes in the reader's language, items grouped by box, and
// the checks the service makes, so a refusal is said before anything is sent. Kept out of the components so the
// rules are unit-tested.

import type { StringKey } from "./i18n.svelte.js";
import type { CanvasItem, CanvasPromoteBody } from "./api/ppm-types.js";

/** The service's limits (`src/canvas.rs`): a canvas title, an item's text, and the items one canvas holds. */
export const CANVAS_TITLE_MAX = 200;
export const CANVAS_TEXT_MAX = 2000;
export const CANVAS_ITEMS_MAX = 500;

/** Each built-in template's catalogue key. A template the service adds later shows the service's own title. */
export const TEMPLATE_LABELS: Record<string, StringKey> = {
  lean: "p.canvas.template_lean",
  business_model: "p.canvas.template_business_model",
  swot: "p.canvas.template_swot",
  empathy: "p.canvas.template_empathy",
  logic_model: "p.canvas.template_logic_model",
  risk: "p.canvas.template_risk",
  goal: "p.canvas.template_goal",
};

/** Each box's catalogue key. One key per box name: "Channels" is the same box in the Lean and Business Model canvases. */
export const BOX_LABELS: Record<string, StringKey> = {
  problem: "p.canvas.box_problem",
  solution: "p.canvas.box_solution",
  key_metrics: "p.canvas.box_key_metrics",
  unique_value_proposition: "p.canvas.box_unique_value_proposition",
  unfair_advantage: "p.canvas.box_unfair_advantage",
  channels: "p.canvas.box_channels",
  customer_segments: "p.canvas.box_customer_segments",
  cost_structure: "p.canvas.box_cost_structure",
  revenue_streams: "p.canvas.box_revenue_streams",
  key_partners: "p.canvas.box_key_partners",
  key_activities: "p.canvas.box_key_activities",
  key_resources: "p.canvas.box_key_resources",
  value_propositions: "p.canvas.box_value_propositions",
  customer_relationships: "p.canvas.box_customer_relationships",
  strengths: "p.canvas.box_strengths",
  weaknesses: "p.canvas.box_weaknesses",
  opportunities: "p.canvas.box_opportunities",
  threats: "p.canvas.box_threats",
  says: "p.canvas.box_says",
  thinks: "p.canvas.box_thinks",
  does: "p.canvas.box_does",
  feels: "p.canvas.box_feels",
  pains: "p.canvas.box_pains",
  gains: "p.canvas.box_gains",
  inputs: "p.canvas.box_inputs",
  activities: "p.canvas.box_activities",
  outputs: "p.canvas.box_outputs",
  outcomes: "p.canvas.box_outcomes",
  impact: "p.canvas.box_impact",
  vulnerabilities: "p.canvas.box_vulnerabilities",
  existing_controls: "p.canvas.box_existing_controls",
  residual_risks: "p.canvas.box_residual_risks",
  goal: "p.canvas.box_goal",
  success_measures: "p.canvas.box_success_measures",
  actions: "p.canvas.box_actions",
  obstacles: "p.canvas.box_obstacles",
  resources: "p.canvas.box_resources",
};

/** A label: the catalogue's when it has one, else the service's own text (`fallback`), else the key. */
export function labelFor(
  labels: Record<string, StringKey>,
  key: string,
  fallback: string | undefined,
  t: (k: StringKey) => string,
): string {
  const k = labels[key];
  return k ? t(k) : fallback || key;
}

/** Items by box key, in the order the service gave them (position, then insertion). A box with none is absent. */
export function itemsByBox(
  items: readonly CanvasItem[],
): Map<string, CanvasItem[]> {
  const out = new Map<string, CanvasItem[]>();
  for (const item of items) {
    const list = out.get(item.box_key);
    if (list) list.push(item);
    else out.set(item.box_key, [item]);
  }
  return out;
}

/** Every problem with a new canvas, as the service checks them: a title that is blank or too long; a long description. */
export function canvasProblems(
  title: string,
  description: string,
): StringKey[] {
  const out: StringKey[] = [];
  const t = title.trim();
  if (t === "") out.push("p.canvas.problem_title_blank");
  else if ([...t].length > CANVAS_TITLE_MAX)
    out.push("p.canvas.problem_title_long");
  if ([...description.trim()].length > CANVAS_TEXT_MAX)
    out.push("p.canvas.problem_description_long");
  return out;
}

/** The problem with an item's text, or `null`: blank, or longer than the service keeps. */
export function itemProblem(body: string): StringKey | null {
  const b = body.trim();
  if (b === "") return "p.canvas.problem_text_blank";
  if ([...b].length > CANVAS_TEXT_MAX) return "p.canvas.problem_text_long";
  return null;
}

/** What an item may become. A risk needs a plan, so a free-standing canvas offers only an idea or an objective. */
export function promoteTargets(
  planPid: string | null,
): Array<CanvasPromoteBody["target"]> {
  return planPid ? ["idea", "objective", "risk"] : ["idea", "objective"];
}

/**
 * The body to send to promote an item, or the problem that stops it. A risk's probability and impact are 1 to 5 and
 * are never defaulted (the service refuses a risk without them); other targets send neither.
 */
export function promoteBody(
  target: CanvasPromoteBody["target"],
  probability: string,
  impact: string,
): { body: CanvasPromoteBody } | { problem: StringKey } {
  if (target !== "risk") return { body: { target } };
  const p = Number(probability);
  const i = Number(impact);
  const ok = (n: number) => Number.isInteger(n) && n >= 1 && n <= 5;
  if (probability === "" || impact === "" || !ok(p) || !ok(i))
    return { problem: "p.canvas.problem_risk_scores" };
  return { body: { target, probability: p, impact: i } };
}

/** The catalogue key naming what an item became. */
export function promotedLabel(
  kind: CanvasItem["promoted_to_kind"],
): StringKey | null {
  if (kind === "idea") return "p.canvas.became_idea";
  if (kind === "risk") return "p.canvas.became_risk";
  if (kind === "objective") return "p.canvas.became_objective";
  return null;
}
