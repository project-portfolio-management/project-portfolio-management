// NC-1 slice 3 (LT-10): the task edit form's checks and body, the words for a link from each side, the checks before
// adding a link, and the refusal sentences. The rules mirror the service's `validate_task` and
// `src/task_relations.rs`; the service still decides.

import { describe, expect, it } from "vitest";
import type { Task, TaskRelation } from "../../src/lib/api/ppm-types";
import {
  RELATION_KINDS,
  formFromTask,
  isPersonRef,
  relationProblem,
  relationWordsKey,
  taskBody,
  taskProblem,
  taskRefusalKey,
} from "../../src/lib/task-edit-view";

const UUID = "0b9d7c1e-2f3a-4b5c-8d6e-7f8091a2b3c4";

const task = (over: Partial<Task> = {}): Task => ({
  pid: "t1",
  title: "Write the spec",
  description: null,
  status: "in_progress",
  assignee_ref: null,
  points: null,
  due_date: null,
  sprint_pid: "s1",
  created_at: "2026-11-01T00:00:00Z",
  status_changed_at: "2026-11-02T00:00:00Z",
  done_at: null,
  blocked_days: null,
  parent_task_pid: "t0",
  component_pid: "c1",
  ...over,
});

const rel = (over: Partial<TaskRelation> = {}): TaskRelation => ({
  pid: "r1",
  kind: "blocks",
  direction: "outgoing",
  other_task: {
    pid: "t2",
    title: "Build it",
    plan_pid: "p1",
    status: "todo",
    due_date: null,
  },
  ...over,
});

describe("the task edit form", () => {
  it("starts from the task, with empty boxes for what it lacks", () => {
    expect(formFromTask(task())).toEqual({
      title: "Write the spec",
      description: "",
      assignee: "",
      points: "",
      dueDate: "",
    });
    expect(
      formFromTask(
        task({
          points: 0,
          due_date: "2026-11-20",
          assignee_ref: `person:${UUID}`,
        }),
      ),
    ).toMatchObject({
      points: "0",
      dueDate: "2026-11-20",
      assignee: `person:${UUID}`,
    });
  });

  it("refuses what the service would", () => {
    const f = formFromTask(task());
    expect(taskProblem(f)).toBeNull();
    expect(taskProblem({ ...f, title: "  " })).toBe("title_required");
    expect(taskProblem({ ...f, title: "é".repeat(513) })).toBe(
      "title_too_long",
    ); // 1026 bytes
    expect(taskProblem({ ...f, title: "a".repeat(1024) })).toBeNull();
    expect(taskProblem({ ...f, assignee: "alice" })).toBe(
      "assignee_not_reference",
    );
    expect(taskProblem({ ...f, assignee: "person:alice" })).toBe(
      "assignee_not_reference",
    );
    expect(taskProblem({ ...f, assignee: `worker:${UUID}` })).toBeNull();
    expect(taskProblem({ ...f, points: "2.5" })).toBe("points_not_whole");
    expect(taskProblem({ ...f, points: "-1" })).toBe("points_not_whole");
    expect(taskProblem({ ...f, points: "101" })).toBe("points_out_of_range");
    expect(taskProblem({ ...f, points: "100" })).toBeNull();
    expect(taskProblem({ ...f, dueDate: "20/11/2026" })).toBe("due_not_date");
  });

  it("knows a person or worker reference", () => {
    expect(isPersonRef(`person:${UUID}`)).toBe(true);
    expect(isPersonRef(`team:${UUID}`)).toBe(false);
    expect(isPersonRef(UUID)).toBe(false);
  });

  it("sends every field, the hidden ones as the task has them, and never a status", () => {
    const body = taskBody(
      {
        title: " New title ",
        description: " ",
        assignee: "",
        points: " 5 ",
        dueDate: "2026-11-20",
      },
      task(),
    );
    expect(body).toEqual({
      title: "New title",
      description: null,
      assignee_ref: null,
      points: 5,
      due_date: "2026-11-20",
      component_pid: "c1",
      parent_task_pid: "t0",
      sprint_pid: "s1",
    });
    expect(Object.keys(body)).not.toContain("status");
    expect(
      taskBody(
        formFromTask(
          task({ component_pid: undefined, parent_task_pid: undefined }),
        ),
        task({ component_pid: undefined, parent_task_pid: undefined }),
      ),
    ).toMatchObject({
      component_pid: null,
      parent_task_pid: null,
    });
  });
});

describe("a link's words, from this task's side", () => {
  it("reads each kind both ways", () => {
    expect(relationWordsKey({ kind: "blocks", direction: "outgoing" })).toBe(
      "p.task.rel_blocks",
    );
    expect(relationWordsKey({ kind: "blocks", direction: "incoming" })).toBe(
      "p.task.rel_blocked_by",
    );
    expect(relationWordsKey({ kind: "precedes", direction: "incoming" })).toBe(
      "p.task.rel_follows",
    );
    expect(
      relationWordsKey({ kind: "duplicates", direction: "incoming" }),
    ).toBe("p.task.rel_duplicated_by");
    expect(
      relationWordsKey({ kind: "copied_from", direction: "incoming" }),
    ).toBe("p.task.rel_copied_to");
    expect(relationWordsKey({ kind: "related", direction: "incoming" })).toBe(
      "p.task.rel_related",
    );
  });

  it("offers the service's five kinds", () => {
    expect([...RELATION_KINDS].sort()).toEqual([
      "blocks",
      "copied_from",
      "duplicates",
      "precedes",
      "related",
    ]);
  });
});

describe("checks before adding a link", () => {
  const me = { pid: "t1", due_date: "2026-11-10" };
  it("needs another task", () => {
    expect(relationProblem("blocks", me, null, [])).toBe("choose_task");
    expect(
      relationProblem("blocks", me, { pid: "t1", due_date: null }, []),
    ).toBe("self");
    expect(
      relationProblem("blocks", me, { pid: "t2", due_date: null }, []),
    ).toBeNull();
  });

  it("refuses a second copy, and either way for related", () => {
    expect(
      relationProblem("blocks", me, { pid: "t2", due_date: null }, [rel()]),
    ).toBe("exists");
    expect(
      relationProblem("precedes", me, { pid: "t2", due_date: null }, [rel()]),
    ).toBeNull();
    expect(
      relationProblem("related", me, { pid: "t2", due_date: null }, [
        rel({ kind: "related", direction: "incoming" }),
      ]),
    ).toBe("exists");
  });

  it("refuses a two-task loop of blocks or precedes, not of duplicates", () => {
    expect(
      relationProblem("blocks", me, { pid: "t2", due_date: null }, [
        rel({ direction: "incoming" }),
      ]),
    ).toBe("cycle");
    expect(
      relationProblem("duplicates", me, { pid: "t2", due_date: null }, [
        rel({ kind: "duplicates", direction: "incoming" }),
      ]),
    ).toBeNull();
  });

  it("needs the later task due strictly after for precedes, when both are dated", () => {
    expect(
      relationProblem(
        "precedes",
        me,
        { pid: "t2", due_date: "2026-11-10" },
        [],
      ),
    ).toBe("precedes_dates");
    expect(
      relationProblem(
        "precedes",
        me,
        { pid: "t2", due_date: "2026-11-11" },
        [],
      ),
    ).toBeNull();
    expect(
      relationProblem("precedes", me, { pid: "t2", due_date: null }, []),
    ).toBeNull();
    expect(
      relationProblem("blocks", me, { pid: "t2", due_date: "2026-11-01" }, []),
    ).toBeNull();
  });
});

describe("refusals", () => {
  it("names each refusal in the catalogue's words", () => {
    expect(taskRefusalKey("edit", 403)).toBe("p.task.refused_forbidden");
    expect(taskRefusalKey("edit", 404)).toBe("p.task.refused_gone");
    expect(taskRefusalKey("remove_link", 404)).toBe("p.task.refused_link_gone");
    expect(taskRefusalKey("add_link", 409)).toBe("p.task.refused_link_exists");
    expect(taskRefusalKey("edit", 422)).toBe("p.task.refused_edit_invalid");
    expect(taskRefusalKey("add_link", 422)).toBe("p.task.refused_link_invalid");
    expect(taskRefusalKey("edit", 409)).toBeNull();
    expect(taskRefusalKey("edit", 500)).toBeNull();
  });
});
