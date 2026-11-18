// Register completeness (EX-1; NC-1 slice 12 part d): the words and the order `/register-completeness` uses. Pure and
// tested.
//
// The service names each missing field by a key (`sro`, `partners`, …); the page names it in the catalogue's words,
// and a key it does not know is "another field", never the key itself. The service's `note` is English and is not used.
// A programme's state is a word and a shape, never colour alone.

import type {
  RegisterCompleteness,
  RegisterProgramme,
} from "./api/ppm-types.js";
import { t, type StringKey } from "./i18n.svelte.js";

/** The ten fields the service checks, in its order. */
export const REGISTER_FIELDS = [
  "sro",
  "partners",
  "stage",
  "baseline",
  "target_date",
  "budget",
  "funding_source",
  "confidence",
  "top_risks",
  "dependencies",
] as const;

export type RegisterField = (typeof REGISTER_FIELDS)[number];

const FIELD_KEYS: Record<RegisterField, StringKey> = {
  sro: "p.director_pack.rf_sro",
  partners: "p.director_pack.rf_partners",
  stage: "p.director_pack.rf_stage",
  baseline: "p.director_pack.rf_baseline",
  target_date: "p.director_pack.rf_target_date",
  budget: "p.director_pack.rf_budget",
  funding_source: "p.director_pack.rf_funding_source",
  confidence: "p.director_pack.rf_confidence",
  top_risks: "p.director_pack.rf_top_risks",
  dependencies: "p.director_pack.rf_dependencies",
};

const isField = (f: string): f is RegisterField =>
  (REGISTER_FIELDS as readonly string[]).includes(f);

/** A field's name in the reader's language; one the page does not know is "another field". */
export function fieldLabel(f: string): string {
  return isField(f)
    ? t(FIELD_KEYS[f])
    : t("p.register_completeness.field_other");
}

/** Several fields' names, joined; "none" when there are none. */
export function fieldList(fields: readonly string[]): string {
  return fields.length === 0
    ? t("p.register_completeness.none_listed")
    : fields.map(fieldLabel).join(", ");
}

/** The programmes as the page lists them: incomplete first, the least complete first, then by name. */
export function orderProgrammes(
  rows: readonly RegisterProgramme[],
): RegisterProgramme[] {
  return [...rows].sort(
    (a, b) =>
      Number(a.complete) - Number(b.complete) ||
      a.score_pct - b.score_pct ||
      a.name.localeCompare(b.name) ||
      a.plan_pid.localeCompare(b.plan_pid),
  );
}

/** A programme's state as a word and a shape. */
export function stateOf(p: Pick<RegisterProgramme, "complete">): {
  symbol: string;
  text: string;
} {
  return p.complete
    ? { symbol: "✓", text: t("p.register_completeness.state_complete") }
    : { symbol: "▲", text: t("p.register_completeness.state_incomplete") };
}

/** How many of the programmes are complete, out of how many. */
export function counts(r: Pick<RegisterCompleteness, "programmes">): {
  complete: number;
  total: number;
} {
  return {
    complete: r.programmes.filter((p) => p.complete).length,
    total: r.programmes.length,
  };
}

/** The sentence for the deployment's "no register, no money" switch. */
export function switchKey(enforced: boolean): StringKey {
  return enforced
    ? "p.register_completeness.switch_on"
    : "p.register_completeness.switch_off";
}
