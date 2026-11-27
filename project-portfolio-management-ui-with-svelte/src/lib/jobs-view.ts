/**
 * Rules behind the bulk jobs page (`/jobs`, T-8; NC-1 slice 12 part k). Pure: no DOM, no fetch; unit-tested.
 * The service's English (`note`, `failure_reason`) is never shown: a state is a word and a symbol from the catalogue.
 */

import { t, tDynamic } from "./i18n.svelte.js";
import { formatInteger, formatPercent } from "./locale-format.js";
import type {
  BulkJob,
  BulkJobStatus,
  ReviewQueueItem,
  StartExportBody,
} from "./api/ppm";

/** Every state, in the order the filter offers them. */
export const STATUSES: readonly BulkJobStatus[] = [
  "queued",
  "running",
  "completed",
  "completed_with_errors",
  "failed",
  "cancelled",
];

const SYMBOL: Record<BulkJobStatus, string> = {
  queued: "…",
  running: "▶",
  completed: "✓",
  completed_with_errors: "◐",
  failed: "✗",
  cancelled: "■",
};

/** A state as a word and a symbol; an unknown state is said as unknown, never guessed. */
export function stateOf(status: string): { symbol: string; text: string } {
  return (STATUSES as readonly string[]).includes(status)
    ? {
        symbol: SYMBOL[status as BulkJobStatus],
        text: tDynamic(`p.jobs.status_${status}`),
      }
    : { symbol: "?", text: t("p.jobs.status_unknown") };
}

/** Whether cancelling could change anything: a queued job, or a running one not yet asked to stop. */
export function cancellable(j: BulkJob): boolean {
  return (
    j.status === "queued" || (j.status === "running" && !j.cancel_requested)
  );
}

/** The counts in one sentence, in the reader's numbers. */
export function rowsText(j: BulkJob): string {
  const c = j.counts;
  const n = (x: number) => formatInteger(x);
  return t("p.jobs.rows", {
    total: n(c.total),
    created: n(c.created),
    updated: n(c.updated),
    unchanged: n(c.unchanged),
    review: n(c.queued_for_review),
    failed: n(c.failed),
  });
}

/** Newest first by start (a queued job, not yet started, first of all), then by id for a stable order. */
export function orderJobs(jobs: readonly BulkJob[]): BulkJob[] {
  const at = (j: BulkJob) => j.started_at ?? "9999";
  return [...jobs].sort(
    (a, b) => at(b).localeCompare(at(a)) || a.job_id.localeCompare(b.job_id),
  );
}

/** The export form as the page holds it. */
export interface ExportForm {
  format: "jsonl" | "csv";
  full: boolean;
  kind: string;
  name: string;
}

/** The body to send: only what differs from the service's defaults, blank filters left out. */
export function exportBody(f: ExportForm): StartExportBody {
  const b: StartExportBody = {};
  if (f.format !== "jsonl") b.format = f.format;
  if (f.full) b.masking_profile = "full";
  if (f.kind.trim()) b.kind = f.kind.trim();
  if (f.name.trim()) b.name_like = f.name.trim();
  return b;
}

/** The format a file's name suggests (`.csv` is CSV; anything else JSON lines). */
export function formatOfName(name: string): "jsonl" | "csv" {
  return /\.csv$/i.test(name.trim()) ? "csv" : "jsonl";
}

/** The catalogue key for a refusal when starting or cancelling, by status. */
export function refusalKey(
  action: "export" | "import" | "cancel" | "load",
  status: number | undefined,
): string {
  if (action === "export" && status === 403) return "p.jobs.export_forbidden";
  if (action === "import" && status === 413) return "p.jobs.too_big";
  if ((action === "export" || action === "import") && status === 422)
    return `p.jobs.${action}_refused`;
  if (action === "cancel" && status === 404) return "p.jobs.gone";
  return `p.jobs.could_not_${action}`;
}

/** How alike a held row and a plan are, in the reader's words: the matcher's score as a percentage and its band. */
export function matchText(m: ReviewQueueItem["matches"][number]): string {
  const band = ["High", "Medium", "Low"].includes(m.confidence)
    ? tDynamic(`p.jobs.confidence_${m.confidence.toLowerCase()}`)
    : t("p.jobs.confidence_unknown");
  return t("p.jobs.review_match", {
    score: formatPercent(m.score),
    confidence: band,
  });
}

/** A held row's name for a question or a notice; a row with none is said to have none. */
export function heldName(i: ReviewQueueItem): string {
  const n = typeof i.candidate.name === "string" ? i.candidate.name.trim() : "";
  return n || t("p.jobs.review_unnamed");
}

/** The catalogue key for a refused decision on a held row. */
export function decisionRefusalKey(status: number | undefined): string {
  if (status === 409) return "p.jobs.review_already";
  if (status === 422) return "p.jobs.review_invalid";
  if (status === 404) return "p.jobs.review_gone";
  return "p.jobs.could_not_decide";
}
