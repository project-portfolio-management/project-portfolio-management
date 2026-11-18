// A sprint's ceremonies and its commitment (`/plans/[pid]/board`, NC-1 slice 12 part j, FR-29): `GET`/`POST
// /api/sprints/{pid}/ceremonies`, `GET /api/sprints/{pid}/commitment` and `POST /api/sprints/{pid}/commit`. The
// service's rules (`controllers/ceremony.rs`), said and checked before anything is sent; the service still decides.
//
// Four kinds: planning, daily, review and retrospective; a sprint holds one planning and one review (a second is a
// re-plan, which is a new sprint), any number of dailies and retrospectives. The facilitator is a reference, not a
// name. Every kind is counted, zero too: a sprint that never held a retrospective is a finding. The commitment is the
// sprint's tasks as they are when committed, taken once and never rewritten, so tasks added or removed later read as
// a change. A sprint with no tasks is not committed here (the service would record nothing and answer as if it were
// never committed, a service gap). Every refusal is one 422 `unprocessable` with English text.

import { ApiError } from "./api/client.js";
import type {
  CeremonyBody,
  CeremonyKind,
  SprintCeremonies,
  SprintCommitment,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { isPersonRef } from "./access-view.js";

/** The kinds, in a sprint's order. */
export const CEREMONY_KINDS: readonly CeremonyKind[] = [
  "planning",
  "daily",
  "review",
  "retrospective",
];

/** A kind in words. */
export const CEREMONY_KEY: Record<CeremonyKind, StringKey> = {
  planning: "p.ceremony.kind_planning",
  daily: "p.ceremony.kind_daily",
  review: "p.ceremony.kind_review",
  retrospective: "p.ceremony.kind_retrospective",
};

/** A kind in words; one the page does not know is a general word. */
export function ceremonyKey(kind: string): StringKey {
  return (
    (CEREMONY_KEY as Record<string, StringKey>)[kind] ?? "p.ceremony.kind_other"
  );
}

/** The kinds a sprint holds once. */
export const ONCE: ReadonlySet<string> = new Set(["planning", "review"]);

/** How many of each kind were held, every kind present (zero when the service leaves one out). */
export function heldCounts(
  c: Pick<SprintCeremonies, "held">,
): { kind: CeremonyKind; count: number }[] {
  return CEREMONY_KINDS.map((kind) => ({
    kind,
    count: c.held.find((h) => h.kind === kind)?.count ?? 0,
  }));
}

/** The ceremony form as typed. */
export interface CeremonyForm {
  kind: CeremonyKind | "";
  facilitator: string;
  summary: string;
}

export function emptyCeremonyForm(): CeremonyForm {
  return { kind: "", facilitator: "", summary: "" };
}

/** The body to send (a blank facilitator or summary not sent) or every problem at once. */
export function ceremonyBody(
  form: CeremonyForm,
  held: Pick<SprintCeremonies, "held"> | null,
): { body: CeremonyBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  if (!form.kind) problems.push("p.ceremony.problem_kind");
  else if (
    ONCE.has(form.kind) &&
    held &&
    (held.held.find((h) => h.kind === form.kind)?.count ?? 0) > 0
  )
    problems.push("p.ceremony.problem_once");
  const facilitator = form.facilitator.trim();
  if (facilitator && !isPersonRef(facilitator))
    problems.push("p.ceremony.problem_facilitator");
  if (problems.length > 0 || !form.kind) return { problems };
  const body: CeremonyBody = { kind: form.kind };
  if (facilitator) body.facilitator_ref = facilitator;
  const summary = form.summary.trim();
  if (summary) body.summary = summary;
  return { body };
}

/** Why the sprint cannot be committed now, or `null`. */
export function commitProblem(c: SprintCommitment | null): StringKey | null {
  if (!c) return "p.commitment.problem_unread";
  if (c.was_committed || c.committed > 0) return "p.commitment.problem_done";
  if (c.current === 0) return "p.commitment.problem_empty";
  return null;
}

/** The tasks added and removed since the commitment, named where the board knows them, else by the start of the id. */
export function scopeChange(
  c: SprintCommitment,
  titleOf: (pid: string) => string | undefined,
): { added: string[]; removed: string[] } {
  const name = (pid: string) => titleOf(pid) ?? pid.slice(0, 8);
  return {
    added: c.added_after_commitment.map(name),
    removed: c.removed_after_commitment.map(name),
  };
}

/** The catalogue sentence for a refused ceremony or commitment, or `null` for the general rule. */
export function sprintRefusalKey(
  action: "ceremony" | "commit",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.ceremony.refused_gone";
  if (err.status === 422)
    return action === "ceremony"
      ? "p.ceremony.refused_ceremony"
      : "p.commitment.refused_commit";
  if (err.status === 403) return "p.ceremony.refused_forbidden";
  return null;
}
