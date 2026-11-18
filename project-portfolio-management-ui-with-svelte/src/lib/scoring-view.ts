// Ranking by WSJF and RICE (`/prioritization`, NC-1 slice 12 part g, NC-6): the model the queue ranks by, a plan's
// estimated inputs (`GET`/`PUT /api/plans/{pid}/scoring-inputs`) and each model's score with its derivation (`GET
// /api/plans/{pid}/score?model=`). The service's rules (`controllers/scoring.rs`, `domain/src/scoring_models.rs`), said
// before anything is sent. The service still decides.
//
// WSJF is (business value + time criticality + risk reduction or opportunity enablement) / job size, each a whole
// number from 1 to 20. RICE is reach x impact x confidence / effort: reach a whole number of people or events in a
// period, impact one of 0.25, 0.5, 1, 2 or 3, confidence a whole percentage from 0 to 100, effort person-months above
// zero. A score with any input missing is absent and names what it waits for, never a guess; a stated zero is not a
// missing input. Saving replaces every input: an empty box clears it. The service's formula, explanations and reasons
// are English and never shown; the catalogue names and explains each component and input by its name.

import { ApiError } from "./api/client.js";
import type {
  EstimateModel,
  RankModel,
  ScoringInputs,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** The models the queue ranks by, the Smart Score first (the service's default, left out of the query). */
export const MODELS: readonly RankModel[] = ["smart", "wsjf", "rice"];

/** Each model's name. */
export const MODEL_KEY: Record<RankModel, StringKey> = {
  smart: "p.scoring.model_smart",
  wsjf: "p.scoring.model_wsjf",
  rice: "p.scoring.model_rice",
};

/** Each estimated model's formula, in words. */
export const FORMULA_KEY: Record<EstimateModel, StringKey> = {
  wsjf: "p.scoring.formula_wsjf",
  rice: "p.scoring.formula_rice",
};

/** The lowest and highest value on a WSJF scale (`WSJF_MIN`, `WSJF_MAX`). */
export const WSJF_MIN = 1;
export const WSJF_MAX = 20;
/** The largest reach the service stores (a 32-bit whole number). */
export const REACH_MAX = 2_147_483_647;
/** The impact values RICE allows, smallest first (`IMPACTS`). */
export const IMPACTS = [0.25, 0.5, 1, 2, 3] as const;

/** Each impact value's word. */
export const IMPACT_KEY: Record<(typeof IMPACTS)[number], StringKey> = {
  0.25: "p.scoring.impact_minimal",
  0.5: "p.scoring.impact_low",
  1: "p.scoring.impact_medium",
  2: "p.scoring.impact_high",
  3: "p.scoring.impact_massive",
};

/** The inputs, WSJF's four then RICE's four. */
export type InputName = keyof ScoringInputs;
export const WSJF_INPUTS: readonly InputName[] = [
  "business_value",
  "time_criticality",
  "risk_reduction",
  "job_size",
];
export const RICE_INPUTS: readonly InputName[] = [
  "reach",
  "impact",
  "confidence_percent",
  "effort",
];
export const INPUTS: readonly InputName[] = [...WSJF_INPUTS, ...RICE_INPUTS];

/** Each input's name, as a form label and in a list of what a score waits for. */
export const INPUT_KEY: Record<InputName, StringKey> = {
  business_value: "p.scoring.input_business_value",
  time_criticality: "p.scoring.input_time_criticality",
  risk_reduction: "p.scoring.input_risk_reduction",
  job_size: "p.scoring.input_job_size",
  reach: "p.scoring.input_reach",
  impact: "p.scoring.input_impact",
  confidence_percent: "p.scoring.input_confidence",
  effort: "p.scoring.input_effort",
};

/** The name of an input the service named; one this page does not know is a general word. */
export function inputKey(name: string): StringKey {
  return (
    (INPUT_KEY as Record<string, StringKey>)[name] ?? "p.scoring.input_other"
  );
}

/** Why a score needs each input, in words (the service's `reason`, which is English, is never shown). */
const MISSING_KEY: Record<InputName, StringKey> = {
  business_value: "p.scoring.needs_delay_part",
  time_criticality: "p.scoring.needs_delay_part",
  risk_reduction: "p.scoring.needs_delay_part",
  job_size: "p.scoring.needs_divisor",
  reach: "p.scoring.needs_reach",
  impact: "p.scoring.needs_impact",
  confidence_percent: "p.scoring.needs_confidence",
  effort: "p.scoring.needs_divisor",
};

/** Why a score waits for an input; one this page does not know is a general sentence. */
export function missingKey(input: string): StringKey {
  return (
    (MISSING_KEY as Record<string, StringKey>)[input] ?? "p.scoring.needs_other"
  );
}

/** Each component the service returns, by name: its label and what it means. */
const COMPONENT: Record<string, { label: StringKey; means: StringKey }> = {
  business_value: {
    label: "p.scoring.input_business_value",
    means: "p.scoring.means_business_value",
  },
  time_criticality: {
    label: "p.scoring.input_time_criticality",
    means: "p.scoring.means_time_criticality",
  },
  risk_reduction: {
    label: "p.scoring.input_risk_reduction",
    means: "p.scoring.means_risk_reduction",
  },
  cost_of_delay: {
    label: "p.scoring.component_cost_of_delay",
    means: "p.scoring.means_cost_of_delay",
  },
  job_size: {
    label: "p.scoring.input_job_size",
    means: "p.scoring.means_job_size",
  },
  reach: { label: "p.scoring.input_reach", means: "p.scoring.means_reach" },
  impact: { label: "p.scoring.input_impact", means: "p.scoring.means_impact" },
  confidence: {
    label: "p.scoring.input_confidence",
    means: "p.scoring.means_confidence",
  },
  effort: { label: "p.scoring.input_effort", means: "p.scoring.means_effort" },
};

/** A component's label and meaning; one this page does not know is a general word and sentence. */
export function componentKeys(name: string): {
  label: StringKey;
  means: StringKey;
} {
  return (
    COMPONENT[name] ?? {
      label: "p.scoring.input_other",
      means: "p.scoring.means_other",
    }
  );
}

/** How a component's value is shown: confidence is a fraction of 1 (a percentage), impact a value with its word. */
export function componentShape(
  name: string,
  value: number | null,
):
  | { kind: "absent" }
  | { kind: "percent"; fraction: number }
  | { kind: "impact"; value: number; word: StringKey | null }
  | { kind: "number"; value: number } {
  if (value === null || value === undefined) return { kind: "absent" };
  if (name === "confidence") return { kind: "percent", fraction: value };
  if (name === "impact")
    return {
      kind: "impact",
      value,
      word: (IMPACT_KEY as Record<number, StringKey>)[value] ?? null,
    };
  return { kind: "number", value };
}

/** The inputs as typed: a number input's value (`undefined` or `null` when empty), the impact a choice. */
export type ScoringForm = Record<InputName, number | null | undefined>;

/** The form filled from what is stored. */
export function scoringFormOf(inputs: ScoringInputs | null): ScoringForm {
  const form = {} as ScoringForm;
  for (const name of INPUTS) form[name] = inputs?.[name] ?? null;
  return form;
}

/** A problem with one input. */
export interface InputProblem {
  input: InputName;
  key: StringKey;
}

const whole = (n: number) => Number.isSafeInteger(n);

/** What is wrong with one typed value, or `null`. */
function inputProblem(name: InputName, v: number): StringKey | null {
  if (!Number.isFinite(v)) return "p.scoring.problem_number";
  if (WSJF_INPUTS.includes(name))
    return whole(v) && v >= WSJF_MIN && v <= WSJF_MAX
      ? null
      : "p.scoring.problem_wsjf";
  if (name === "reach")
    return whole(v) && v >= 0 && v <= REACH_MAX
      ? null
      : "p.scoring.problem_reach";
  if (name === "impact")
    return (IMPACTS as readonly number[]).includes(v)
      ? null
      : "p.scoring.problem_impact";
  if (name === "confidence_percent")
    return whole(v) && v >= 0 && v <= 100
      ? null
      : "p.scoring.problem_confidence";
  return v > 0 ? null : "p.scoring.problem_effort";
}

/** The inputs to send, every problem at once, or `null` when they are what is stored (nothing is sent). The body
 * carries all eight, as the service replaces them all: an empty box is `null`, which clears it. */
export function scoringBody(
  form: ScoringForm,
  stored: ScoringInputs | null,
): { body: ScoringInputs } | { problems: InputProblem[] } | null {
  const problems: InputProblem[] = [];
  const body = {} as ScoringInputs;
  for (const name of INPUTS) {
    const raw = form[name];
    const v = raw === undefined || raw === null ? null : Number(raw);
    if (v !== null) {
      const key = inputProblem(name, v);
      if (key) problems.push({ input: name, key });
    }
    body[name] = v;
  }
  if (problems.length > 0) return { problems };
  const same = INPUTS.every((name) => body[name] === (stored?.[name] ?? null));
  return same ? null : { body };
}

/** The sentence for a refused save, or `null` for the generic one. Every rule is checked first, so a 422 says only
 * that an input is out of its range; the service's text is never shown. */
export function scoringRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.scoring.refused_gone";
  if (err.status === 422) return "p.scoring.refused_inputs";
  return null;
}
