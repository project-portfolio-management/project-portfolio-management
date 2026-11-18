// The named trade-off on a new ask (EX-13, NC-1 slice 11 part m): the service's rules (`controllers/governance.rs`),
// said before anything is sent. The service still decides. A proposal tagged as an external ask from the sponsor must
// name what it displaces (10 to 2000 characters) before it is approved; the trade-off can be named or changed until
// the proposal is decided.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { Proposal } from "./api/ppm.js";

/** The shortest and longest trade-off, in characters after trimming. */
export const TRADEOFF_MIN = 10;
export const TRADEOFF_MAX = 2000;
/** The statuses in which a trade-off can still be named or changed. */
export const UNDECIDED: readonly Proposal["status"][] = [
  "draft",
  "submitted",
  "in_review",
];

const chars = (s: string) => [...s.trim()].length;

/** Whether the proposal names a trade-off long enough to be approved. */
export function hasNamedTradeoff(
  p: Pick<Proposal, "external_ask" | "tradeoff">,
): boolean {
  return chars(p.tradeoff ?? "") >= TRADEOFF_MIN;
}

/** Whether the trade-off can be named or changed now: an external ask not yet decided. */
export function canNameTradeoff(
  p: Pick<Proposal, "external_ask" | "status">,
): boolean {
  return p.external_ask && UNDECIDED.includes(p.status);
}

/** What is wrong with a trade-off's text, or `null`. */
export function tradeoffProblem(text: string): StringKey | null {
  const n = chars(text);
  return n < TRADEOFF_MIN || n > TRADEOFF_MAX
    ? "p.proposals.problem_tradeoff"
    : null;
}

/** What is wrong with a new proposal, or `null`: a title, and a trade-off (if one is named now) of 10 to 2000
 * characters. A new external ask may leave its trade-off to be named later. */
export function newProposalProblem(form: {
  title: string;
  externalAsk: boolean;
  tradeoff: string;
}): StringKey | null {
  if (form.title.trim() === "") return "p.proposals.problem_title";
  if (form.externalAsk && form.tradeoff.trim() !== "")
    return tradeoffProblem(form.tradeoff);
  return null;
}

/** The external-ask fields of a new proposal's body: nothing for an ordinary one. */
export function newAskFields(
  externalAsk: boolean,
  tradeoff: string,
): { external_ask?: true; tradeoff?: string } {
  if (!externalAsk) return {};
  const text = tradeoff.trim();
  return text ? { external_ask: true, tradeoff: text } : { external_ask: true };
}

/** The body for `PUT …/tradeoff`, the problem, or `null` when it names what the proposal already names. */
export function tradeoffBody(
  p: Pick<Proposal, "external_ask" | "status" | "tradeoff">,
  text: string,
): { body: { tradeoff: string } } | { problem: StringKey } | null {
  if (!p.external_ask) return { problem: "p.proposals.problem_not_ask" };
  if (!UNDECIDED.includes(p.status))
    return { problem: "p.proposals.problem_decided" };
  const why = tradeoffProblem(text);
  if (why) return { problem: why };
  const t = text.trim();
  if ((p.tradeoff ?? "").trim() === t) return null;
  return { body: { tradeoff: t } };
}

/** Why the proposal cannot be approved yet, or `null`: an external ask needs a named trade-off. */
export function approveProblem(
  p: Pick<Proposal, "external_ask" | "tradeoff">,
): StringKey | null {
  return p.external_ask && !hasNamedTradeoff(p)
    ? "p.proposals.problem_approve"
    : null;
}

/** The sentence for a refusal of `action`, or `null` for the generic one. */
export function proposalRefusalKey(
  action: "create" | "submit" | "review" | "approve" | "reject" | "tradeoff",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.proposals.refused_gone";
  if (err.status !== 422) return null;
  if (action === "create") return "p.proposals.refused_create";
  if (action === "approve") return "p.proposals.refused_approve";
  if (action === "tradeoff") return "p.proposals.refused_tradeoff";
  return "p.proposals.refused_step";
}
