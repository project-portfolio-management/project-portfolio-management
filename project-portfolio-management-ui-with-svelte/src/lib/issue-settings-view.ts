// The issue workflow form (OC-1): what a person is drafting, the first thing wrong with it, and the request body. The
// service decides; this only saves a round trip.

import type { IssueStatusCategory, NewIssueWorkflow } from "./api/ppm-types.js";

/** One status being drafted. `id` keeps its row when its key changes. */
export interface DraftState {
  id: number;
  key: string;
  label: string;
  category: IssueStatusCategory;
  terminal: boolean;
}

/** One move being drafted. `roles` is what was typed, comma-separated. */
export interface DraftMove {
  id: number;
  from: string;
  to: string;
  roles: string;
}

/** What the form holds. `initial` is the id of the status new issues start in. */
export interface WorkflowDraft {
  name: string;
  issueType: string;
  states: DraftState[];
  moves: DraftMove[];
  initial: number;
  nextId: number;
}

/** A form that starts from the shape every workflow needs: somewhere to start and somewhere to finish. */
export function emptyWorkflowDraft(): WorkflowDraft {
  return {
    name: "",
    issueType: "",
    states: [
      { id: 1, key: "open", label: "Open", category: "todo", terminal: false },
      {
        id: 2,
        key: "closed",
        label: "Closed",
        category: "done",
        terminal: true,
      },
    ],
    moves: [],
    initial: 1,
    nextId: 3,
  };
}

/** The first thing wrong with a draft, or `null`. */
export type WorkflowProblem =
  "name" | "no_done_status" | "repeated_key" | "no_start" | null;

export function workflowProblem(d: WorkflowDraft): WorkflowProblem {
  if (d.name.trim() === "") return "name";
  const keys = d.states.map((s) => s.key.trim());
  if (new Set(keys).size !== keys.length) return "repeated_key";
  if (!d.states.some((s) => s.category === "done")) return "no_done_status";
  if (!d.states.some((s) => s.id === d.initial)) return "no_start";
  return null;
}

/** The request body: a deployment-wide default. A plan's own workflow is registered through the API. */
export function workflowBody(d: WorkflowDraft): NewIssueWorkflow {
  return {
    name: d.name.trim(),
    applies_to: "issue",
    issue_type: d.issueType === "" ? null : d.issueType,
    is_default: true,
    states: d.states.map((s) => ({
      key: s.key.trim(),
      label: s.label.trim(),
      category: s.category,
      is_initial: s.id === d.initial,
      is_terminal: s.terminal,
    })),
    transitions: d.moves.map((m) => ({
      from: m.from,
      to: m.to,
      roles: m.roles
        .split(",")
        .map((r) => r.trim())
        .filter((r) => r !== ""),
    })),
  };
}
