// Plain-language glosses for the terms and acronyms the screens cannot avoid
// (LT-12). An acronym never stands alone in a heading, label, button or table
// header: it follows plain words ("Key measures (KPIs)") or is a `<Term>` that
// carries its gloss. Pure data, so the rule can be tested against the sources.

export const GLOSSARY: Record<string, string> = {
  SPI: "Schedule performance index: work done compared with work planned by now. Above 1.00 is ahead of plan.",
  CPI: "Cost performance index: value of the work done compared with what it cost. Above 1.00 is under budget.",
  "SPI(t)": "Schedule index by time: how far along the plan we are in time, which stays truthful even when a project finishes late.",
  EV: "Earned value: the budget for the work actually finished.",
  PV: "Planned value: the budget for the work that should be finished by now.",
  AC: "Actual cost: what has really been spent.",
  EAC: "Estimate at completion: the expected final cost.",
  ETC: "Estimate to complete: what is still expected to be spent.",
  VAC: "Variance at completion: the expected final cost against the budget.",
  RAG: "Red, amber or green: a status light.",
  RACI: "Responsible, Accountable, Consulted, Informed: who does the work, who signs it off, who is asked and who is told.",
  PERT: "A way of weighting the likely estimate more than the best and worst cases.",
  KPI: "Key performance indicator: a measure tracked over time against a target.",
  KPIs: "Key performance indicators: measures tracked over time against a target.",
  OKR: "Objective and key results: a goal with a few measurable results.",
  OKRs: "Objectives and key results: goals with a few measurable results.",
};

/** Acronyms that must not stand alone in a heading, label, button or table header. */
export const ACRONYMS: readonly string[] = [
  "CPM", "RAG", "EV", "PV", "AC", "SPI", "CPI", "EAC", "TCPI", "ETC", "BAC", "VAC",
  "RACI", "WBS", "PERT", "ETag", "ABAC", "OKR", "OKRs", "KPI", "KPIs", "MSPDI", "URN",
  "SV", "CV", "WIP", "TBA", "PMO",
];

/** The gloss for a term, or null when there is none. */
export function glossFor(term: string): string | null {
  return GLOSSARY[term] ?? null;
}
