import { t } from "./i18n.svelte.js";
// Plain-language glosses for the terms and acronyms the screens cannot avoid
// (LT-12). An acronym never stands alone in a heading, label, button or table
// header: it follows plain words ("Key measures (KPIs)") or is a `<Term>` that
// carries its gloss. Pure data, so the rule can be tested against the sources.

export const GLOSSARY: Record<string, string> = {
  get SPI() { return t("l.glossary.schedule_performance_index_work_done"); },
  get CPI() { return t("l.glossary.cost_performance_index_value_of"); },
  get "SPI(t)"() { return t("l.glossary.schedule_index_by_time_how"); },
  get EV() { return t("l.glossary.earned_value_the_budget_for"); },
  get PV() { return t("l.glossary.planned_value_the_budget_for"); },
  get AC() { return t("l.glossary.actual_cost_what_has_really"); },
  get EAC() { return t("l.glossary.estimate_at_completion_the_expected"); },
  get ETC() { return t("l.glossary.estimate_to_complete_what_is"); },
  get VAC() { return t("l.glossary.variance_at_completion_the_expected"); },
  get RAG() { return t("l.glossary.red_amber_or_green_a"); },
  get RACI() { return t("l.glossary.responsible_accountable_consulted_in"); },
  get PERT() { return t("l.glossary.a_way_of_weighting_the"); },
  get KPI() { return t("l.glossary.key_performance_indicator_a_measure"); },
  get KPIs() { return t("l.glossary.key_performance_indicators_measures_"); },
  get OKR() { return t("l.glossary.objective_and_key_results_a"); },
  get OKRs() { return t("l.glossary.objectives_and_key_results_goals"); },
};

/** Acronyms that must not stand alone in a heading, label, button or table header. */
export const ACRONYMS: readonly string[] = [
  "CPM",
  "RAG",
  "EV",
  "PV",
  "AC",
  "SPI",
  "CPI",
  "EAC",
  "TCPI",
  "ETC",
  "BAC",
  "VAC",
  "RACI",
  "WBS",
  "PERT",
  "ETag",
  "ABAC",
  "OKR",
  "OKRs",
  "KPI",
  "KPIs",
  "MSPDI",
  "URN",
  "SV",
  "CV",
  "WIP",
  "TBA",
  "PMO",
];

/** The gloss for a term, or null when there is none. */
export function glossFor(term: string): string | null {
  return GLOSSARY[term] ?? null;
}
