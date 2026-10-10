// Tracker import (T-28n): the words for a preview and its report. Pure and tested.

import type { ImportUnmapped, TrackerImportReport } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export const SOURCE_LABEL = {
  get jira() { return t("l.import_view.jira_csv_export"); },
  get asana() { return t("l.import_view.asana_csv_export"); },
  get mspdi() { return t("l.import_view.microsoft_project_xml_file"); },
} as const;

export type ImportSource = keyof typeof SOURCE_LABEL;

/** What the file picker offers for each source. */
export const SOURCE_ACCEPT: Record<ImportSource, string> = {
  jira: ".csv,text/csv",
  asana: ".csv,text/csv",
  mspdi: ".xml,application/xml,text/xml",
};

/** The one thing people get wrong with Microsoft Project: a .mpp file is not the XML export. */
export function mspdiHint(): string {
  return t("l.import_view.mspdi_hint");
}

/** What the unmapped kinds are called to a person. */
const WHAT: Record<string, string> = {
  get column() { return t("l.import_view.columns_not_imported"); },
  get row() { return t("l.import_view.rows_skipped"); },
  get link() { return t("l.import_view.links_not_imported"); },
  get parent() { return t("l.import_view.parents_not_used"); },
  get date() { return t("l.import_view.dates_not_understood"); },
  get field() { return t("l.import_view.fields_shortened"); },
  get status() { return t("l.import_view.status_notes"); },
  get resource() { return t("l.import_view.resources_not_matched_to_people"); },
  get cost() { return t("l.import_view.costs_not_imported"); },
  get schedule() { return t("l.import_view.schedules_not_imported"); },
  get constraint() { return t("l.import_view.constraints_not_imported"); },
  get calendar() { return t("l.import_view.calendar_notes"); },
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

const taskCount = (n: number): string =>
  n === 1 ? t("l.import_view.task_one", { n }) : t("l.import_view.task_other", { n });
const dependencyCount = (n: number): string =>
  n === 1 ? t("l.import_view.dependency_one", { n }) : t("l.import_view.dependency_other", { n });

/** "Would create 3 tasks and update 2; 1 dependency." — or the same in the past tense. */
export function summaryText(r: TrackerImportReport): string {
  const verb = r.dry_run ? t("l.import_view.would_create") : t("l.import_view.created");
  const upd = r.dry_run ? t("l.import_view.word_update") : t("l.import_view.word_updated");
  const links = r.dry_run ? r.links_in_file : r.links_created;
  const tail =
    links > 0 ? `; ${dependencyCount(links)}` : "";
  return t("l.import_view.and", { verb, plural: taskCount(r.tasks_created), upd, tasks_updated: r.tasks_updated, tail });
}

/** The label of the one primary button: preview first, then the import itself. */
export function actionLabel(report: TrackerImportReport | null): string {
  if (!report) return t("l.import_view.preview_the_import");
  const n = report.tasks_created + report.tasks_updated;
  return t("l.import_view.import", { plural: taskCount(n) });
}
