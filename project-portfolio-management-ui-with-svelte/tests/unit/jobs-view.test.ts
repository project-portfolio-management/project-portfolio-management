import { describe, expect, it } from "vitest";
import {
  STATUSES,
  cancellable,
  exportBody,
  formatOfName,
  orderJobs,
  refusalKey,
  rowsText,
  stateOf,
} from "../../src/lib/jobs-view";
import type { BulkJob } from "../../src/lib/api/ppm";

const job = (over: Partial<BulkJob> = {}): BulkJob => ({
  job_id: "j1",
  kind: "import",
  status: "completed",
  format: "jsonl",
  counts: {
    total: 3,
    created: 1,
    updated: 1,
    unchanged: 0,
    queued_for_review: 0,
    failed: 1,
  },
  reconciles: true,
  rows_omitted: 0,
  errors_url: null,
  review_url: null,
  download_url: null,
  expires_at: null,
  failure_reason: null,
  progress: 100,
  cancel_requested: false,
  cancelled: false,
  note: null,
  started_at: "2026-11-26T10:00:00Z",
  finished_at: "2026-11-26T10:01:00Z",
  ...over,
});

describe("jobs-view (T-8, NC-1 slice 12 part k)", () => {
  it("says every state as a word and a symbol, and an unknown one as unknown", () => {
    expect(STATUSES.map((s) => stateOf(s).symbol)).toEqual([
      "…",
      "▶",
      "✓",
      "◐",
      "✗",
      "■",
    ]);
    expect(stateOf("completed_with_errors").text).toBe("completed with errors");
    expect(stateOf("exploded")).toEqual({ symbol: "?", text: "unknown state" });
  });

  it("offers a cancel only where it can change something", () => {
    expect(cancellable(job({ status: "queued" }))).toBe(true);
    expect(cancellable(job({ status: "running" }))).toBe(true);
    expect(
      cancellable(job({ status: "running", cancel_requested: true })),
    ).toBe(false);
    for (const s of [
      "completed",
      "completed_with_errors",
      "failed",
      "cancelled",
    ] as const)
      expect(cancellable(job({ status: s }))).toBe(false);
  });

  it("puts every count in one sentence in the reader's numbers", () => {
    expect(
      rowsText(
        job({
          counts: {
            total: 12000,
            created: 10000,
            updated: 1000,
            unchanged: 500,
            queued_for_review: 400,
            failed: 100,
          },
        }),
      ),
    ).toBe(
      "12,000 rows: 10,000 created, 1,000 updated, 500 unchanged, 400 held for review, 100 failed.",
    );
  });

  it("lists a job not started first, then newest first, ties by id", () => {
    const order = orderJobs([
      job({ job_id: "old", started_at: "2026-11-01T00:00:00Z" }),
      job({ job_id: "b", started_at: "2026-11-20T00:00:00Z" }),
      job({ job_id: "queued", started_at: null, status: "queued" }),
      job({ job_id: "a", started_at: "2026-11-20T00:00:00Z" }),
    ]).map((j) => j.job_id);
    expect(order).toEqual(["queued", "a", "b", "old"]);
  });

  it("sends only what differs from the service's defaults", () => {
    expect(
      exportBody({ format: "jsonl", full: false, kind: " ", name: "" }),
    ).toEqual({});
    expect(
      exportBody({
        format: "csv",
        full: true,
        kind: " Project ",
        name: " apollo ",
      }),
    ).toEqual({
      format: "csv",
      masking_profile: "full",
      kind: "Project",
      name_like: "apollo",
    });
  });

  it("reads the format from a file's name", () => {
    expect(formatOfName("plans.CSV")).toBe("csv");
    expect(formatOfName("plans.jsonl")).toBe("jsonl");
    expect(formatOfName("plans")).toBe("jsonl");
  });

  it("has a sentence of its own for each refusal it knows", () => {
    expect(refusalKey("export", 403)).toBe("p.jobs.export_forbidden");
    expect(refusalKey("export", 422)).toBe("p.jobs.export_refused");
    expect(refusalKey("import", 413)).toBe("p.jobs.too_big");
    expect(refusalKey("import", 422)).toBe("p.jobs.import_refused");
    expect(refusalKey("cancel", 404)).toBe("p.jobs.gone");
    expect(refusalKey("cancel", 500)).toBe("p.jobs.could_not_cancel");
    expect(refusalKey("load", undefined)).toBe("p.jobs.could_not_load");
  });
});
