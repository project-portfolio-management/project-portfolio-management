// Labels every Lily widget takes for export (CSV, TSV, Excel), copy and search, in the page's language.
// Shared so a widget gains a control by being given these, with the words kept in one place.
import { t } from "#lib/i18n.svelte.js";

export function exportLabels() {
  return {
    export: t("p.table.export"),
    exportCsv: t("p.table.export_csv"),
    exportTsv: t("p.table.export_tsv"),
    exportExcel: t("p.table.export_excel"),
    exportAnnouncement: (_format: string, rows: number, file: string) => t("p.table.exported", { rows, file }),
    copy: t("p.table.copy"),
    copyAnnouncement: (rows: number) => t("p.table.copied", { rows }),
    copyFailed: t("p.table.copy_failed"),
  };
}

export function searchAnnouncement(matches: number, total: number): string {
  return t("p.table.matches", { matches, total });
}
