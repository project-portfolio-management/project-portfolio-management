// Approval processes (PL-2): the pure rules the page applies before it asks the service, and the
// words for a run's state. The service checks everything again; this only saves a round trip and
// says what is wrong in the reader's own language.

import { t } from "./i18n.svelte.js";

export type StepKind = "approve" | "todo" | "notify";

export interface StepDraft {
  kind: StepKind;
  person_ref: string;
  title: string;
}

/** The most steps a process may have (the service's limit). */
export const MAX_STEPS = 20;

export const STEP_KINDS: readonly StepKind[] = ["approve", "todo", "notify"];

/** A new, empty step. */
export const blankStep = (): StepDraft => ({ kind: "approve", person_ref: "", title: "" });

/** Whether `ref` is a `person:` or `worker:` URN with something after the colon. */
export function isPersonRef(ref: string): boolean {
  const at = ref.indexOf(":");
  return at > 0 && ["person", "worker"].includes(ref.slice(0, at)) && ref.slice(at + 1).trim() !== "";
}

/** What is wrong with a draft definition, in words; empty when it can be sent. */
export function definitionProblems(name: string, steps: readonly StepDraft[]): string[] {
  const out: string[] = [];
  if (name.trim() === "" || [...name].length > 120) out.push(t("l.processes.name_problem"));
  if (steps.length === 0 || steps.length > MAX_STEPS) out.push(t("l.processes.steps_problem", { max: MAX_STEPS }));
  steps.forEach((s, i) => {
    if (s.title.trim() === "" || [...s.title].length > 200) out.push(t("l.processes.title_problem", { n: i + 1 }));
    if (!isPersonRef(s.person_ref.trim())) out.push(t("l.processes.person_problem", { n: i + 1 }));
  });
  return out;
}

/** The label for a step kind. */
export function kindLabel(kind: string): string {
  return kind === "approve"
    ? t("l.processes.kind_approve")
    : kind === "todo"
      ? t("l.processes.kind_todo")
      : kind === "notify"
        ? t("l.processes.kind_notify")
        : kind;
}

/** The label for a step's state in a run. */
export function stepStateLabel(state: string): string {
  const labels: Record<string, string> = {
    pending: t("l.processes.state_pending"),
    waiting: t("l.processes.state_waiting"),
    completed: t("l.processes.state_completed"),
    skipped: t("l.processes.state_skipped"),
    rejected: t("l.processes.state_rejected"),
    failed: t("l.processes.state_failed"),
  };
  return labels[state] ?? state;
}

/** The label for a run's status. */
export function runStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    running: t("l.processes.run_running"),
    completed: t("l.processes.run_completed"),
    rejected: t("l.processes.run_rejected"),
    cancelled: t("l.processes.run_cancelled"),
    failed: t("l.processes.run_failed"),
  };
  return labels[status] ?? status;
}

/** Whether a run can still be cancelled or resumed. */
export const isRunning = (status: string): boolean => status === "running";
