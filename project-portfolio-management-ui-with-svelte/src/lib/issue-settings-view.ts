// The issue workflow form (OC-1, OC-4, OC-6): what a person is drafting, the first thing wrong with it, and the request
// bodies. The service decides; this only saves a round trip.

import type {
  FieldRule,
  IssuePermission,
  IssueState,
  IssueStatusCategory,
  IssueWorkflow,
  IssueWorkflowChange,
  Labels,
  NewIssueWorkflow,
} from "./api/ppm-types.js";

/** The permissions a role can carry, in the order the settings page lists them. */
export const ISSUE_PERMISSIONS: IssuePermission[] = [
  "issue_add",
  "issue_edit",
  "issue_move",
  "issue_delete",
];

/** The fields a status can make required or read-only. */
export const RULE_FIELDS = [
  "title",
  "description",
  "severity",
  "priority",
  "owner_ref",
  "due_date",
  "resolution",
] as const;

/** A field a rule can name. */
export type RuleField = (typeof RULE_FIELDS)[number];

/** One field rule being drafted. `roles` is what was typed, comma-separated. */
export interface DraftRule {
  id: number;
  field: string;
  rule: FieldRule["rule"];
  roles: string;
}

/** One status being drafted. `id` keeps its row when its key changes. `labels` is what was typed. */
export interface DraftState {
  id: number;
  key: string;
  label: string;
  labels: string;
  category: IssueStatusCategory;
  terminal: boolean;
  rules: DraftRule[];
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
      {
        id: 1,
        key: "open",
        label: "Open",
        labels: "",
        category: "todo",
        terminal: false,
        rules: [],
      },
      {
        id: 2,
        key: "closed",
        label: "Closed",
        labels: "",
        category: "done",
        terminal: true,
        rules: [],
      },
    ],
    moves: [],
    initial: 1,
    nextId: 3,
  };
}

/** The form for changing a registered workflow. */
export function draftFromWorkflow(w: {
  name: string;
  issue_type: string | null;
  workflow: IssueWorkflow;
}): WorkflowDraft {
  let id = 0;
  const next = () => (id += 1);
  const states: DraftState[] = w.workflow.states.map((s) => ({
    id: next(),
    key: s.key,
    label: s.label,
    labels: labelsText(s.labels),
    category: s.category,
    terminal: s.is_terminal,
    rules: (s.field_rules ?? []).map((r) => ({
      id: next(),
      field: r.field,
      rule: r.rule,
      roles: r.roles.join(", "),
    })),
  }));
  const moves: DraftMove[] = w.workflow.transitions.map((t) => ({
    id: next(),
    from: t.from,
    to: t.to,
    roles: t.roles.join(", "),
  }));
  const start = w.workflow.states.findIndex((s) => s.is_initial);
  return {
    name: w.name,
    issueType: w.issue_type ?? "",
    states,
    moves,
    initial: states[start]?.id ?? 0,
    nextId: id + 1,
  };
}

/** Translations as a person edits them: `cy=Risg; zh-cn=风险`. */
export function labelsText(labels: Labels | null | undefined): string {
  return Object.entries(labels ?? {})
    .map(([code, text]) => `${code}=${text}`)
    .join("; ");
}

/** Translations from what was typed, or `null` if a part is not `language=name`. Blank is no translations. */
export function parseLabels(text: string): Labels | null {
  const labels: Labels = {};
  for (const part of text.split(";")) {
    if (part.trim() === "") continue;
    const at = part.indexOf("=");
    if (at < 1) return null;
    const code = part.slice(0, at).trim().toLowerCase();
    const word = part.slice(at + 1).trim();
    if (word === "" || !/^[a-z]{2,3}(-[a-z0-9]{2,8})*$/.test(code)) return null;
    labels[code] = word;
  }
  return labels;
}

/** The first thing wrong with a draft, or `null`. */
export type WorkflowProblem =
  | "name"
  | "no_done_status"
  | "repeated_key"
  | "no_start"
  | "bad_translations"
  | null;

export function workflowProblem(d: WorkflowDraft): WorkflowProblem {
  if (d.name.trim() === "") return "name";
  const keys = d.states.map((s) => s.key.trim());
  if (new Set(keys).size !== keys.length) return "repeated_key";
  if (!d.states.some((s) => s.category === "done")) return "no_done_status";
  if (!d.states.some((s) => s.id === d.initial)) return "no_start";
  if (d.states.some((s) => parseLabels(s.labels) === null))
    return "bad_translations";
  return null;
}

const names = (typed: string): string[] =>
  typed
    .split(",")
    .map((r) => r.trim())
    .filter((r) => r !== "");

function statesOf(d: WorkflowDraft): IssueState[] {
  return d.states.map((s) => ({
    key: s.key.trim(),
    label: s.label.trim(),
    labels: parseLabels(s.labels) ?? {},
    category: s.category,
    is_initial: s.id === d.initial,
    is_terminal: s.terminal,
    field_rules: s.rules.map((r) => ({
      field: r.field,
      rule: r.rule,
      roles: names(r.roles),
    })),
  }));
}

function movesOf(d: WorkflowDraft) {
  return d.moves.map((m) => ({
    from: m.from,
    to: m.to,
    roles: names(m.roles),
  }));
}

/** The request body to register a workflow: a plan's own, or a deployment-wide default with no `plan`. */
export function workflowBody(
  d: WorkflowDraft,
  plan: string | null = null,
): NewIssueWorkflow {
  return {
    name: d.name.trim(),
    applies_to: "issue",
    issue_type: d.issueType === "" ? null : d.issueType,
    plan_pid: plan,
    is_default: plan === null,
    states: statesOf(d),
    transitions: movesOf(d),
  };
}

/** The request body to change a registered workflow. */
export function workflowChange(d: WorkflowDraft): IssueWorkflowChange {
  return { name: d.name.trim(), states: statesOf(d), transitions: movesOf(d) };
}
