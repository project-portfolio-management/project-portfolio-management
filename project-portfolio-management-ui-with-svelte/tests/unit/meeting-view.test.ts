// NC-1 slice 12 part (e) (NC-14, BR-3): a plan's meetings and the tasks made from their actions. The service's rules
// said first: what planning, recording as held and proposing tasks send (lines with nothing in them left out, a blank
// date not sent, nothing at all when no proposal is chosen), every problem named on its line, the passage looked for
// in the minutes as the service looks, the due window, each dropped reason's sentence and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { Meeting } from "../../src/lib/api/ppm-types";
import {
  ITEMS_MAX,
  MINUTES_MAX,
  PROPOSALS_MAX,
  TEXT_MAX,
  TITLE_MAX,
  allReady,
  droppedKey,
  dueInWindow,
  dueWindow,
  excerptFound,
  heldBody,
  linkOf,
  meetingRefusalKey,
  openActions,
  planBody,
  proposalFor,
  proposalsBody,
  squash,
  type ProposalForm,
} from "../../src/lib/meeting-view";

const TASK = "11111111-1111-4111-8111-111111111111";

describe("a link to one of the plan's records", () => {
  it("is read from the list's value, and none is none", () => {
    expect(linkOf(`task:${TASK}`)).toEqual({ kind: "task", pid: TASK });
    expect(linkOf("")).toBeUndefined();
    expect(linkOf("task:")).toBeUndefined();
    expect(linkOf(":x")).toBeUndefined();
  });
});

describe("planning a meeting", () => {
  it("sends the title and date trimmed, and the agenda lines with text, each with its record when chosen", () => {
    expect(
      planBody({
        title: "  Gate 2 review ",
        scheduledOn: "2026-11-24",
        agenda: [
          { text: " Scope ", link: `task:${TASK}` },
          { text: "", link: "" },
          { text: "Budget", link: "" },
        ],
      }),
    ).toEqual({
      body: {
        title: "Gate 2 review",
        scheduled_on: "2026-11-24",
        agenda: [
          { text: "Scope", link: { kind: "task", pid: TASK } },
          { text: "Budget" },
        ],
      },
    });
  });
  it("sends no agenda when there is none", () => {
    expect(
      planBody({ title: "Kick-off", scheduledOn: "2026-11-24", agenda: [] }),
    ).toEqual({
      body: { title: "Kick-off", scheduled_on: "2026-11-24" },
    });
  });
  it("names every problem, each agenda line by its place", () => {
    expect(
      planBody({
        title: "x".repeat(TITLE_MAX + 1),
        scheduledOn: "",
        agenda: [
          { text: "", link: `risk:${TASK}` },
          { text: "y".repeat(TEXT_MAX + 1), link: "" },
          { text: "fine", link: "" },
        ],
      }),
    ).toEqual({
      problems: [
        { key: "p.meetings.problem_title", line: 0 },
        { key: "p.meetings.problem_date", line: 0 },
        { key: "p.meetings.problem_item_text", line: 1 },
        { key: "p.meetings.problem_item_text", line: 2 },
      ],
    });
    expect(
      planBody({ title: "T", scheduledOn: "2026-11-24", agenda: [] }),
    ).toHaveProperty("body");
    const many = Array.from({ length: ITEMS_MAX + 1 }, () => ({
      text: "x",
      link: "",
    }));
    expect(
      planBody({ title: "T", scheduledOn: "2026-11-24", agenda: many }),
    ).toEqual({
      problems: [{ key: "p.meetings.problem_agenda_count", line: 0 }],
    });
  });
  it("counts characters, not bytes, as the service does", () => {
    expect(
      planBody({
        title: "é".repeat(TITLE_MAX),
        scheduledOn: "2026-11-24",
        agenda: [],
      }),
    ).toHaveProperty("body");
  });
});

describe("recording a meeting as held", () => {
  it("sends the minutes trimmed, a date only when one is given, and the outcomes with something in them", () => {
    expect(
      heldBody({
        minutes: " We met. ",
        heldOn: "",
        outcomes: [
          { kind: "decision", text: " Go ", link: "" },
          { kind: "", text: "", link: "" },
          { kind: "action", text: "Order the racks", link: `task:${TASK}` },
        ],
      }),
    ).toEqual({
      body: {
        minutes: "We met.",
        outcomes: [
          { kind: "decision", text: "Go" },
          {
            kind: "action",
            text: "Order the racks",
            link: { kind: "task", pid: TASK },
          },
        ],
      },
    });
    expect(
      heldBody({ minutes: "m", heldOn: "2026-11-25", outcomes: [] }),
    ).toEqual({
      body: { minutes: "m", held_on: "2026-11-25" },
    });
  });
  it("names every problem: the minutes, an outcome with no kind or no text", () => {
    expect(
      heldBody({
        minutes: "x".repeat(MINUTES_MAX + 1),
        heldOn: "",
        outcomes: [
          { kind: "", text: "Go", link: "" },
          { kind: "note", text: "", link: "" },
        ],
      }),
    ).toEqual({
      problems: [
        { key: "p.meetings.problem_minutes", line: 0 },
        { key: "p.meetings.problem_outcome_kind", line: 1 },
        { key: "p.meetings.problem_item_text", line: 2 },
      ],
    });
    expect(heldBody({ minutes: "  ", heldOn: "", outcomes: [] })).toEqual({
      problems: [{ key: "p.meetings.problem_minutes", line: 0 }],
    });
  });
});

describe("a passage of the minutes", () => {
  const minutes = "We agreed.\nPriya  will ORDER the racks by the tenth.";
  it("is found across capitals and line breaks, as the service looks", () => {
    expect(squash("  A\n b  C ")).toBe("a b c");
    expect(excerptFound(minutes, "priya will order the racks")).toBe(true);
    expect(excerptFound(minutes, "Priya will order the servers")).toBe(false);
  });
  it("must be at least 8 characters to count as evidence", () => {
    expect(excerptFound(minutes, "agreed")).toBe(false);
    expect(excerptFound(minutes, "agreed.\n")).toBe(false);
    expect(excerptFound(minutes, "we agreed")).toBe(true);
  });
});

describe("the due window", () => {
  it("runs from a week before the meeting to a year after", () => {
    expect(dueWindow("2026-11-03")).toEqual({
      first: "2026-10-27",
      last: "2027-11-03",
    });
    expect(dueInWindow("2026-10-27", "2026-11-03")).toBe(true);
    expect(dueInWindow("2026-10-26", "2026-11-03")).toBe(false);
    expect(dueInWindow("2027-11-03", "2026-11-03")).toBe(true);
    expect(dueInWindow("2027-11-04", "2026-11-03")).toBe(false);
  });
});

const HELD: Meeting = {
  pid: "m1",
  title: "Gate 2 review",
  scheduled_on: "2026-11-03",
  status: "held",
  held_on: "2026-11-04",
  minutes: "We agreed.\nPriya will order the racks by the tenth.",
  agenda: [],
  outcomes: [
    { index: 0, kind: "decision", text: "Go", link: null, task_pid: null },
    {
      index: 1,
      kind: "action",
      text: "Order the racks",
      link: null,
      task_pid: null,
    },
    {
      index: 2,
      kind: "action",
      text: "Book the room",
      link: null,
      task_pid: "t9",
    },
  ],
};

describe("the actions a task may be proposed for", () => {
  it("are a held meeting's actions with no task yet", () => {
    expect(openActions(HELD).map((o) => o.index)).toEqual([1]);
    expect(openActions({ ...HELD, status: "planned" })).toEqual([]);
    expect(openActions({ ...HELD, minutes: null })).toEqual([]);
  });
  it("start from the action's text as the title, not chosen", () => {
    expect(proposalFor(HELD.outcomes[1]!)).toEqual({
      chosen: false,
      outcome: 1,
      title: "Order the racks",
      excerpt: "",
      assignee: "",
      due: "",
      points: "",
    });
  });
});

describe("proposing tasks", () => {
  const row = (over: Partial<ProposalForm> = {}): ProposalForm => ({
    ...proposalFor(HELD.outcomes[1]!),
    chosen: true,
    excerpt: "Priya will order the racks",
    ...over,
  });
  it("sends nothing when no proposal is chosen", () => {
    expect(proposalsBody(HELD, [row({ chosen: false })])).toBeNull();
  });
  it("sends what is given, trimmed, points as a number (a number input binds a number once typed in)", () => {
    expect(
      proposalsBody(HELD, [
        row({ assignee: " person:ana ", due: "2026-11-10", points: 3 }),
      ]),
    ).toEqual({
      body: {
        items: [
          {
            title: "Order the racks",
            excerpt: "Priya will order the racks",
            outcome: 1,
            assignee_ref: "person:ana",
            due_on: "2026-11-10",
            points: 3,
          },
        ],
      },
    });
    expect(proposalsBody(HELD, [row({ points: "0" })])).toMatchObject({
      body: { items: [{ points: 0 }] },
    });
  });
  it("names every problem by the outcome it is for, judging the due date against the day it was held", () => {
    expect(
      proposalsBody(HELD, [
        row({
          title: " ",
          excerpt: "not in there at all",
          due: "2026-10-27",
          points: 1.5,
        }),
      ]),
    ).toEqual({
      problems: [
        { key: "p.meetings.problem_task_title", outcome: 2 },
        { key: "p.meetings.problem_excerpt", outcome: 2 },
        { key: "p.meetings.problem_due", outcome: 2 },
        { key: "p.meetings.problem_points", outcome: 2 },
      ],
    });
    expect(proposalsBody(HELD, [row({ points: 1001 })])).toEqual({
      problems: [{ key: "p.meetings.problem_points", outcome: 2 }],
    });
    const many = Array.from({ length: PROPOSALS_MAX + 1 }, () => row());
    expect(proposalsBody(HELD, many)).toMatchObject({
      problems: [{ key: "p.meetings.problem_proposals_count", outcome: 0 }],
    });
  });
});

describe("the service's verdicts", () => {
  it("say each reason code in the catalogue's words, and an unknown one generally", () => {
    for (const r of [
      "title_invalid",
      "excerpt_not_in_minutes",
      "outcome_not_found",
      "outcome_not_an_action",
      "outcome_already_has_a_task",
      "due_outside_window",
      "points_invalid",
      "assignee_not_in_plan",
      "outcome_proposed_twice",
    ])
      expect(droppedKey(r)).toBe(`p.meetings.dropped_${r}`);
    expect(droppedKey("something_new")).toBe("p.meetings.dropped_other");
  });
  it("let the tasks be created only when every proposal is ready", () => {
    const ready = { index: 0, status: "ready" as const, assignee: null };
    const dropped = {
      index: 1,
      status: "dropped" as const,
      reason: "points_invalid",
    };
    expect(
      allReady({
        meeting: "m1",
        counts: { ready: 1, dropped: 0 },
        items: [ready],
      }),
    ).toBe(true);
    expect(
      allReady({
        meeting: "m1",
        counts: { ready: 1, dropped: 1 },
        items: [ready, dropped],
      }),
    ).toBe(false);
    expect(
      allReady({ meeting: "m1", counts: { ready: 0, dropped: 0 }, items: [] }),
    ).toBe(false);
  });
});

describe("a refusal", () => {
  const e = (s: number) => new ApiError(s, "service text");
  it("has a sentence per action and status", () => {
    expect(meetingRefusalKey("held", e(404))).toBe("p.meetings.refused_gone");
    expect(meetingRefusalKey("held", e(409))).toBe(
      "p.meetings.refused_settled",
    );
    expect(meetingRefusalKey("cancel", e(409))).toBe(
      "p.meetings.refused_settled",
    );
    expect(meetingRefusalKey("preview", e(409))).toBe(
      "p.meetings.refused_not_held",
    );
    expect(meetingRefusalKey("commit", e(409))).toBe(
      "p.meetings.refused_commit_state",
    );
    expect(meetingRefusalKey("plan", e(422))).toBe("p.meetings.refused_link");
    expect(meetingRefusalKey("held", e(422))).toBe("p.meetings.refused_link");
    expect(meetingRefusalKey("commit", e(422))).toBe(
      "p.meetings.refused_commit_invalid",
    );
    expect(meetingRefusalKey("preview", e(422))).toBe(
      "p.meetings.refused_proposals",
    );
  });
  it("leaves anything else to the general rule", () => {
    expect(meetingRefusalKey("plan", e(409))).toBeNull();
    expect(meetingRefusalKey("plan", e(500))).toBeNull();
    expect(meetingRefusalKey("plan", new Error("x"))).toBeNull();
  });
});
