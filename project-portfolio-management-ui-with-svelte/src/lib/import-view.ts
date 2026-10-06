// Tracker import (T-28n): the words for a preview and its report. Pure and tested.

import type { ImportUnmapped, TrackerImportReport } from "#lib/api/ppm.js";

export const SOURCE_LABEL = {
  jira: "Jira (CSV export)",
  asana: "Asana (CSV export)",
  mspdi: "Microsoft Project (XML file)",
} as const;

export type ImportSource = keyof typeof SOURCE_LABEL;

/** What the file picker offers for each source. */
export const SOURCE_ACCEPT: Record<ImportSource, string> = {
  jira: ".csv,text/csv",
  asana: ".csv,text/csv",
  mspdi: ".xml,application/xml,text/xml",
};

/** The one thing people get wrong with Microsoft Project: a .mpp file is not the XML export. */
export const MSPDI_HINT =
  "In Microsoft Project use File → Save As → XML Format. A .mpp file cannot be read.";

/** What the unmapped kinds are called to a person. */
const WHAT: Record<string, string> = {
  column: "Columns not imported",
  row: "Rows skipped",
  link: "Links not imported",
  parent: "Parents not used",
  date: "Dates not understood",
  field: "Fields shortened",
  status: "Status notes",
  resource: "Resources not matched to people",
  cost: "Costs not imported",
  schedule: "Schedules not imported",
  constraint: "Constraints not imported",
  calendar: "Calendar notes",
};

/** The unmapped list grouped by kind, kinds in a fixed order, each with its reasons. */
export function unmappedGroups(
  list: readonly ImportUnmapped[],
): { title: string; reasons: string[] }[] {
  const order = Object.keys(WHAT);
  const groups = new Map<string, string[]>();
  for (const u of list) {
    const line = u.key ? `${u.key}: ${u.reason}` : u.reason;
    groups.set(u.what, [...(groups.get(u.what) ?? []), line]);
  }
  return [...groups.entries()]
    .sort(
      ([a], [b]) => (order.indexOf(a) + 1 || 99) - (order.indexOf(b) + 1 || 99),
    )
    .map(([what, reasons]) => ({ title: WHAT[what] ?? what, reasons }));
}

const plural = (n: number, one: string, many: string) =>
  `${n} ${n === 1 ? one : many}`;

/** "Would create 3 tasks and update 2; 1 dependency." — or the same in the past tense. */
export function summaryText(r: TrackerImportReport): string {
  const verb = r.dry_run ? "Would create" : "Created";
  const upd = r.dry_run ? "update" : "updated";
  const links = r.dry_run ? r.links_in_file : r.links_created;
  const tail =
    links > 0 ? `; ${plural(links, "dependency", "dependencies")}` : "";
  return `${verb} ${plural(r.tasks_created, "task", "tasks")} and ${upd} ${r.tasks_updated}${tail}.`;
}

/** The label of the one primary button: preview first, then the import itself. */
export function actionLabel(report: TrackerImportReport | null): string {
  if (!report) return "Preview the import";
  const n = report.tasks_created + report.tasks_updated;
  return `Import ${plural(n, "task", "tasks")}`;
}
