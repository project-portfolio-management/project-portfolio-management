// NC-1 slice 12 part (j), RM-9 and PL-3: many tasks at once on the board, checked first, nothing sent for no change;
// and the words for following the board live.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  BULK_MAX,
  bulkEditBody,
  bulkOutcome,
  bulkRefusalKey,
  deleteGone,
  deleteProblem,
  emptyBulkEditForm,
  transferBody,
  type BulkEditForm,
} from "../../src/lib/board-bulk-view";
import {
  LIVE_EVENTS,
  liveEventKey,
  stateAfterError,
} from "../../src/lib/live-board-view";

const PERSON = "person:0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";
const tasks = [
  { pid: "t1", assignee_ref: PERSON, due_date: "2026-12-01" },
  { pid: "t2", assignee_ref: null, due_date: null },
];
const form = (over: Partial<BulkEditForm> = {}): BulkEditForm => ({
  ...emptyBulkEditForm(),
  ...over,
});

describe("the bulk edit", () => {
  it("sends only the fields set or cleared, each id once", () => {
    expect(
      bulkEditBody(
        ["t1", "t2", "t1"],
        form({
          assignee: { mode: "set", value: ` ${PERSON} ` },
          due: { mode: "clear", value: "" },
        }),
        tasks,
      ),
    ).toEqual({
      body: {
        ids: ["t1", "t2"],
        set: { assignee_ref: PERSON, due_date: null },
      },
    });
    expect(
      bulkEditBody(
        ["t2"],
        form({ start: { mode: "set", value: "2026-12-07" } }),
        tasks,
      ),
    ).toEqual({ body: { ids: ["t2"], set: { start_date: "2026-12-07" } } });
  });
  it("sends nothing when every chosen task has the values already", () => {
    expect(
      bulkEditBody(
        ["t1"],
        form({
          assignee: { mode: "set", value: PERSON },
          due: { mode: "set", value: "2026-12-01" },
        }),
        tasks,
      ),
    ).toBeNull();
    expect(
      bulkEditBody(["t2"], form({ due: { mode: "clear", value: "" } }), tasks),
    ).toBeNull();
    // A start date is always a change: the board does not read it.
    expect(
      bulkEditBody(
        ["t2"],
        form({ start: { mode: "clear", value: "" } }),
        tasks,
      ),
    ).not.toBeNull();
  });
  it("says every problem at once", () => {
    expect(bulkEditBody([], form(), tasks)).toEqual({
      problems: ["p.bulk.problem_none", "p.bulk.problem_no_field"],
    });
    expect(
      bulkEditBody(
        ["t1"],
        form({
          assignee: { mode: "set", value: "Ada" },
          due: { mode: "set", value: "2026-02-30" },
          start: { mode: "set", value: "1969-12-31" },
        }),
        tasks,
      ),
    ).toEqual({
      problems: [
        "p.bulk.problem_assignee",
        "p.bulk.problem_due",
        "p.bulk.problem_start",
      ],
    });
    const many = Array.from({ length: BULK_MAX + 1 }, (_, i) => `t${i}`);
    expect(
      bulkEditBody(many, form({ due: { mode: "clear", value: "" } }), tasks),
    ).toEqual({ problems: ["p.bulk.problem_too_many"] });
  });
});

describe("copy and move", () => {
  it("keeps the assignee only for a copy that asks", () => {
    expect(transferBody("copy", ["t1"], "p2", "p1", true)).toEqual({
      body: { ids: ["t1"], to_plan_pid: "p2", keep_assignee: true },
    });
    expect(transferBody("copy", ["t1"], "p1", "p1", false)).toEqual({
      body: { ids: ["t1"], to_plan_pid: "p1" },
    });
    expect(transferBody("move", ["t1"], "p2", "p1", true)).toEqual({
      body: { ids: ["t1"], to_plan_pid: "p2" },
    });
  });
  it("refuses a move to the same plan, and no plan, first", () => {
    expect(transferBody("move", ["t1"], "p1", "p1", false)).toEqual({
      problems: ["p.bulk.problem_same_plan"],
    });
    expect(transferBody("copy", [], "", "p1", false)).toEqual({
      problems: ["p.bulk.problem_none", "p.bulk.problem_plan"],
    });
  });
});

describe("the outcome", () => {
  const result = {
    requested: 3,
    ok: 2,
    refused: 1,
    results: [
      { pid: "t1", result: "ok" as const, note: "service text" },
      { pid: "t2", result: "ok" as const, note: null },
      {
        pid: "abcdef0123456789",
        result: "refused" as const,
        note: "service text",
      },
    ],
  };
  it("names the refused tasks, never the service's reason, and counts the mapped ones of a move", () => {
    const title = (p: string) => (p === "t1" ? "Design" : undefined);
    expect(bulkOutcome("move", result, title)).toEqual({
      done: 2,
      requested: 3,
      refused: ["abcdef01"],
      mapped: 1,
    });
    expect(bulkOutcome("copy", result, title).mapped).toBe(0);
  });
});

describe("refusals", () => {
  const err = (status: number) => new ApiError(status, "service text", {});
  it("is a sentence by action and status", () => {
    expect(bulkRefusalKey("edit", err(422))).toBe("p.bulk.refused_edit");
    expect(bulkRefusalKey("move", err(422))).toBe("p.bulk.refused_transfer");
    expect(bulkRefusalKey("copy", err(404))).toBe("p.bulk.refused_plan_gone");
    expect(bulkRefusalKey("edit", err(403))).toBe("p.bulk.refused_forbidden");
    expect(bulkRefusalKey("edit", err(500))).toBeNull();
    expect(bulkRefusalKey("edit", new Error("x"))).toBeNull();
  });
  it("counts a task already deleted as gone", () => {
    expect(deleteGone(err(404))).toBe(true);
    expect(deleteGone(err(403))).toBe(false);
    expect(deleteProblem([])).toBe("p.bulk.problem_none");
    expect(deleteProblem(["t1"])).toBeNull();
  });
});

describe("following the board", () => {
  it("listens for every event the service sends, and words one it does not know", () => {
    expect(LIVE_EVENTS).toEqual([
      "task_created",
      "task_moved",
      "task_updated",
      "raci_updated",
      "comment_added",
      "resync",
    ]);
    expect(liveEventKey("task_moved")).toBe("p.live.event_task_moved");
    expect(liveEventKey("something_new")).toBe("p.live.event_other");
  });
  it("says stopped when the browser gives up, retrying when it reconnects", () => {
    expect(stateAfterError(2)).toBe("stopped");
    expect(stateAfterError(0)).toBe("retrying");
  });
});
