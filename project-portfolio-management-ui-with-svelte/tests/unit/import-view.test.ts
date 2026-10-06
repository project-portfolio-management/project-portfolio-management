import { describe, expect, it } from "vitest";
import type { TrackerImportReport } from "../../src/lib/api/ppm";
import {
  actionLabel,
  summaryText,
  unmappedGroups,
} from "../../src/lib/import-view";
import { ApiClient } from "../../src/lib/api/client";

const report = (
  over: Partial<TrackerImportReport> = {},
): TrackerImportReport => ({
  dry_run: true,
  source: "jira",
  items_in_file: 5,
  tasks_created: 3,
  tasks_updated: 2,
  links_in_file: 1,
  links_created: 0,
  links_already_present: 0,
  unmapped: [],
  note: "",
  ...over,
});

describe("the preview in words", () => {
  it("speaks in the conditional before and the past after", () => {
    expect(summaryText(report())).toBe(
      "Would create 3 tasks and update 2; 1 dependency.",
    );
    expect(summaryText(report({ dry_run: false, links_created: 2 }))).toBe(
      "Created 3 tasks and updated 2; 2 dependencies.",
    );
    expect(
      summaryText(
        report({ tasks_created: 1, tasks_updated: 0, links_in_file: 0 }),
      ),
    ).toBe("Would create 1 task and update 0.");
  });
  it("one button: preview first, then the import itself", () => {
    expect(actionLabel(null)).toBe("Preview the import");
    expect(actionLabel(report())).toBe("Import 5 tasks");
    expect(actionLabel(report({ tasks_created: 1, tasks_updated: 0 }))).toBe(
      "Import 1 task",
    );
  });
});

describe("what was left out", () => {
  it("is grouped by kind in a fixed order with the key in front of each reason", () => {
    const g = unmappedGroups([
      { what: "link", key: "P-2", reason: "a link was skipped" },
      { what: "column", key: null, reason: "sprint: not imported" },
      { what: "link", key: "P-3", reason: "another" },
      { what: "weird", key: null, reason: "x" },
    ]);
    expect(g.map((x) => x.title)).toEqual([
      "Columns not imported",
      "Links not imported",
      "weird",
    ]);
    expect(g[1]?.reasons).toEqual(["P-2: a link was skipped", "P-3: another"]);
    expect(unmappedGroups([])).toEqual([]);
  });
});

describe("the client", () => {
  it("sends a raw body untouched, with the caller's content type", async () => {
    let seen: RequestInit | undefined;
    const client = new ApiClient({
      baseUrl: "http://x.example",
      fetch: (async (_u: string, init: RequestInit) => {
        seen = init;
        return new Response("{}", {
          status: 200,
          headers: { "content-type": "application/json" },
        });
      }) as unknown as typeof fetch,
    });
    await client.post("/api/x", {
      rawBody: "a,b\n1,2\n",
      headers: { "content-type": "text/csv" },
    });
    expect(seen?.body).toBe("a,b\n1,2\n");
    expect((seen?.headers as Record<string, string>)["content-type"]).toBe(
      "text/csv",
    );
  });
});
