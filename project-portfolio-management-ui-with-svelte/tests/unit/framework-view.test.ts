import { describe, expect, it } from "vitest";
import { formOf, frameworkBody, frameworkProblems, linkBody, linkProblems, summary, systemText, type FrameworkForm, type LinkForm } from "../../src/lib/framework-view";
import type { Framework } from "../../src/lib/api/ppm";

const form = (over: Partial<FrameworkForm> = {}): FrameworkForm => ({ ...formOf(null), name: "Standard", ...over });
const link = (over: Partial<LinkForm> = {}): LinkForm => ({ kind: "docs", url: "https://wiki.example.com", label: "Spec", system: "", external_id: "", ...over });

describe("frameworkProblems", () => {
  it("accepts a plain form and refuses a missing name", () => {
    expect(frameworkProblems("ways-of-working", form())).toEqual([]);
    expect(frameworkProblems("ways-of-working", form({ name: "  " }))).toHaveLength(1);
  });
  it("checks the numbers a family takes", () => {
    expect(frameworkProblems("ways-of-working", form({ cadence_days: "14", wip_limit: "5" }))).toEqual([]);
    expect(frameworkProblems("ways-of-working", form({ cadence_days: "0" }))).toHaveLength(1);
    expect(frameworkProblems("ways-of-working", form({ wip_limit: "1001" }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ review_cadence_days: "400" }))).toHaveLength(1);
  });
  it("wants each escalation level once, 1 to 9, with a body and a trigger", () => {
    const ok = { level: "1", body: "Sponsor", trigger: "Slip" };
    expect(frameworkProblems("governance-frameworks", form({ path: [ok, { ...ok, level: "2" }] }))).toEqual([]);
    expect(frameworkProblems("governance-frameworks", form({ path: [ok, ok] }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ path: [{ ...ok, level: "10" }] }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ path: [{ ...ok, trigger: "" }] }))).toHaveLength(1);
  });
});

describe("frameworkBody", () => {
  it("sends every list the form holds and keeps the description", () => {
    const existing = { description: "d" } as Framework;
    const body = frameworkBody(
      "ways-of-working",
      form({ cadence_days: "7", done: " Tested ", ceremonies: [{ name: " Standup ", cadence: "daily", purpose: "" }] }),
      existing,
      null,
    );
    expect(body).toMatchObject({ name: "Standard", cadence_days: 7, wip_limit: null, definition_of_done: "Tested", definition_of_ready: null, description: "d" });
    expect(body.ceremonies).toEqual([{ name: "Standup", cadence: "daily", purpose: null }]);
    expect(body).not.toHaveProperty("plan_pid");
  });
  it("owns a new record by the plan, with its path, rights and forums", () => {
    const body = frameworkBody(
      "governance-frameworks",
      form({
        path: [{ level: "2", body: " Board ", trigger: " Cost " }],
        rights: [{ decision: "Scope", decides: "Sponsor", consulted: "" }],
        forums: [{ name: "Steering", cadence: "monthly", purpose: "Review" }],
      }),
      null,
      "p1",
    );
    expect(body).toMatchObject({
      plan_pid: "p1",
      escalation_path: [{ level: 2, body: "Board", trigger: "Cost" }],
      decision_rights: [{ decision: "Scope", decides: "Sponsor", consulted: null }],
      forums: [{ name: "Steering", cadence: "monthly", purpose: "Review" }],
    });
  });
  it("round-trips a record through the form", () => {
    const rec = { name: "R", methodology: "scrum", ceremonies: [{ name: "Retro", cadence: "fortnightly", purpose: "Learn" }], definition_of_ready: "Ready" } as Framework;
    const body = frameworkBody("ways-of-working", formOf(rec), rec, null);
    expect(body).toMatchObject({ methodology: "scrum", definition_of_ready: "Ready", ceremonies: [{ name: "Retro", cadence: "fortnightly", purpose: "Learn" }] });
  });
});

describe("numbers typed into number inputs", () => {
  it("arrive as numbers and are still checked and sent", () => {
    const typed = form({ cadence_days: 7 as unknown as string, wip_limit: 0 as unknown as string });
    expect(frameworkProblems("ways-of-working", typed)).toHaveLength(1);
    const ok = form({ cadence_days: 7 as unknown as string, wip_limit: null as unknown as string });
    expect(frameworkProblems("ways-of-working", ok)).toEqual([]);
    expect(frameworkBody("ways-of-working", ok, null, null)).toMatchObject({ cadence_days: 7, wip_limit: null });
    const path = form({ path: [{ level: 1 as unknown as string, body: "Sponsor", trigger: "Slip" }] });
    expect(frameworkProblems("governance-frameworks", path)).toEqual([]);
  });
});

describe("list rows", () => {
  it("need a name and a cadence, or the decision and who decides", () => {
    expect(frameworkProblems("ways-of-working", form({ ceremonies: [{ name: "Standup", cadence: "", purpose: "" }] }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ forums: [{ name: "", cadence: "monthly", purpose: "" }] }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ rights: [{ decision: "Scope", decides: "", consulted: "" }] }))).toHaveLength(1);
    expect(frameworkProblems("governance-frameworks", form({ rights: [{ decision: "Scope", decides: "Sponsor", consulted: "" }] }))).toEqual([]);
  });
});

describe("summary", () => {
  it("lists only what is set, and orders the escalation path", () => {
    const rows = summary("governance-frameworks", {
      decision_model: "steering_committee",
      escalation_path: [{ level: 2, body: "Board", trigger: "x" }, { level: 1, body: "Sponsor", trigger: "y" }],
    } as Framework);
    expect(rows.map((r) => r[1])).toEqual(["Steering committee", "1: Sponsor; 2: Board"]);
  });
});

describe("linkProblems", () => {
  it("accepts a plain link and one into a self-hosted Jira", () => {
    expect(linkProblems(link())).toEqual([]);
    expect(linkProblems(link({ kind: "tracker", system: "jira", url: "https://jira.internal.example/projects/P", external_id: "PPM" }))).toEqual([]);
  });
  it("refuses a script, a missing label, and a tracker without a system", () => {
    expect(linkProblems(link({ url: "javascript:alert(1)" }))).toHaveLength(1);
    expect(linkProblems(link({ label: " " }))).toHaveLength(1);
    expect(linkProblems(link({ kind: "tracker" }))).toHaveLength(1);
  });
  it("keeps a hosted system on its own domain, without being fooled by a look-alike", () => {
    expect(linkProblems(link({ kind: "tracker", system: "trello", url: "https://trello.com/b/x" }))).toEqual([]);
    expect(linkProblems(link({ kind: "tracker", system: "trello", url: "https://eviltrello.com/b/x" }))).toHaveLength(1);
    expect(linkProblems(link({ kind: "tracker", system: "trello", url: "https://trello.com.evil.example/b" }))).toHaveLength(1);
  });
  it("wants an id to have a system and no spaces", () => {
    expect(linkProblems(link({ external_id: "X-1" }))).toHaveLength(1);
    expect(linkProblems(link({ kind: "ticket", system: "linear", url: "https://linear.app/a", external_id: "has space" }))).toHaveLength(1);
  });
});

describe("linkBody and systemText", () => {
  it("sends only what is filled in", () => {
    expect(linkBody(link({ label: " Spec " }))).toEqual({ kind: "docs", url: "https://wiki.example.com", label: "Spec" });
    expect(linkBody(link({ kind: "ticket", system: "jira", external_id: "PPM-7" }))).toMatchObject({ system: "jira", external_id: "PPM-7" });
  });
  it("names the system and id", () => {
    expect(systemText({ system: "azure_devops", external_id: "12" })).toBe("Azure DevOps 12");
    expect(systemText({ system: null, external_id: null })).toBe("");
  });
});
