import { describe, expect, it } from "vitest";
import type {
  ControlledDocument,
  DocumentStatus,
  DocumentVersion,
} from "../../src/lib/api/ppm";
import {
  ACTION_LABEL,
  STATUS_SYMBOL,
  STATUS_TEXT,
  approverRef,
  forceText,
  inFlight,
  isEditable,
  nextAction,
  versionLine,
} from "../../src/lib/document-view";

const v = (
  n: number,
  status: DocumentStatus,
  over: Partial<DocumentVersion> = {},
): DocumentVersion => ({
  pid: `v${n}`,
  version: n,
  status,
  change_summary: null,
  approval_pid: null,
  effective_from: null,
  superseded_on: null,
  superseded_by: null,
  created_by: null,
  ...over,
});
const doc = (
  versions: DocumentVersion[],
  inForce: number | null = null,
): ControlledDocument => ({
  pid: "d",
  title: "Policy",
  plan_pid: null,
  created_by: null,
  in_force_today: inForce,
  versions,
});

describe("next action", () => {
  it("names exactly one step for each state", () => {
    expect(nextAction(doc([v(1, "draft")]))).toBe("send_for_review");
    expect(nextAction(doc([v(1, "in_review", { approval_pid: "a" })]))).toBe(
      "decide",
    );
    expect(nextAction(doc([v(1, "approved", { approval_pid: "a" })]))).toBe(
      "make_effective",
    );
    expect(
      nextAction(doc([v(1, "effective", { effective_from: "2026-10-06" })], 1)),
    ).toBe("start_next_draft");
    expect(nextAction(doc([]))).toBe("none");
  });
  it("follows the version in flight, not the one in force", () => {
    const d = doc(
      [v(1, "effective", { effective_from: "2026-10-01" }), v(2, "draft")],
      1,
    );
    expect(inFlight(d)?.version).toBe(2);
    expect(nextAction(d)).toBe("send_for_review");
  });
  it("every action has a label", () => {
    for (const a of [
      "send_for_review",
      "decide",
      "make_effective",
      "start_next_draft",
    ] as const)
      expect(ACTION_LABEL[a]).toBeTruthy();
  });
});

describe("words", () => {
  it("every status has a word and a distinct shape", () => {
    const s = Object.keys(STATUS_TEXT) as DocumentStatus[];
    expect(new Set(s.map((x) => STATUS_SYMBOL[x])).size).toBe(s.length);
  });
  it("says when a version was in force and what replaced it", () => {
    const all = [
      v(1, "superseded", { superseded_on: "2026-10-06", superseded_by: "v2" }),
      v(2, "effective", { effective_from: "2026-10-06" }),
    ];
    expect(versionLine(all[1]!, all)).toBe(
      "Version 2 — In force since 2026-10-06",
    );
    expect(versionLine(all[0]!, all)).toBe(
      "Version 1 — Replaced on 2026-10-06 by version 2",
    );
    expect(versionLine(v(3, "draft"))).toBe("Version 3 — Draft");
  });
  it("the list says which version is in force, or none", () => {
    expect(forceText(doc([v(1, "draft")]))).toBe("Nothing in force yet");
    expect(forceText(doc([v(2, "effective")], 2))).toBe("Version 2 in force");
  });
  it("only a draft is editable", () => {
    expect(isEditable(v(1, "draft"))).toBe(true);
    for (const s of [
      "in_review",
      "approved",
      "effective",
      "superseded",
    ] as const)
      expect(isEditable(v(1, s))).toBe(false);
  });
});

describe("approver", () => {
  const id = "3f2c6b7e-0000-4000-8000-0000000000b2";
  it("accepts a bare id or a person URN and nothing else", () => {
    expect(approverRef(id)).toBe(`person:${id}`);
    expect(approverRef(` person:${id} `)).toBe(`person:${id}`);
    expect(approverRef("someone")).toBeNull();
    expect(approverRef("")).toBeNull();
    expect(approverRef("org:" + id)).toBeNull();
  });
});
