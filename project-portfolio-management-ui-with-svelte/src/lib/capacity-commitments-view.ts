// FB-6 (completed) and FB-5: the capacity commitment form and the outcome of sending a programme's demand to the
// workforce service, as the service records it (entity spec §8.5a rules 5 and 6). Pure: the page renders what
// this returns.
//
// The form's figures are read in the reader's own number format (`parseAmount`, the one parser), so "1,5" is one
// and a half in a decimal-comma locale. The service owns every rule about what is sent: the page only says, in
// plain words, what the service recorded. The service's own `detail` text is English and is not shown.

import { parseAmount, currentLocale } from "./money.js";
import { fte, momentLabel, monthLabel } from "./capacity-heatmap-view.js";
import { t } from "./i18n.svelte.js";

/** The latest send of a programme's demand on a pool for a month, as the service records it. */
export interface PeerDemandSend {
  outcome: string;
  detail: string | null;
  /** Hundredths of a full-time equivalent sent; 0 is a withdrawal; null when nothing was sent. */
  fte_centi: number | null;
  peer_event_id: string;
  /** When it was recorded (on the list only). */
  at?: string;
}

/** One row of `GET /api/capacity-commitments`, and the answer of `PUT`. */
export interface CapacityCommitment {
  pid: string;
  pool_pid: string;
  plan_pid: string;
  organization: string | null;
  organization_pid?: string | null;
  kind: "supply" | "demand";
  /** `YYYY-MM`. */
  month: string;
  committed_fte: number;
  delivered_fte: number | null;
  note: string | null;
  /** Present on a demand commitment of a linked pool (null when nothing is recorded yet), or wherever a send is. */
  peer_demand_send?: PeerDemandSend | null;
}

/** The body of `PUT /api/capacity-commitments`. */
export interface CommitmentInput {
  pool_pid: string;
  plan_pid: string;
  organization?: string;
  kind: "supply" | "demand";
  month: string;
  committed_fte: number;
  delivered_fte?: number;
  note?: string;
}

/** The form as typed. */
export interface CommitmentForm {
  pool: string;
  plan: string;
  organization: string;
  kind: string;
  month: string;
  committed: string;
  delivered: string;
  note: string;
}

export type Problem =
  "pool" | "plan" | "kind" | "month" | "committed" | "delivered";

/** The most full-time equivalents the service takes for one commitment. */
export const FTE_MAX = 1000;

function figure(text: string, locale: string): number | null {
  const n = parseAmount(text, locale);
  return n === null || n < 0 || n > FTE_MAX ? null : n;
}

/** Check the form and build the body, or name every field that is wrong. Nothing is sent with a problem. */
export function readForm(
  f: CommitmentForm,
  locale: string = currentLocale(),
): { body: CommitmentInput | null; problems: Problem[] } {
  const problems: Problem[] = [];
  if (!f.pool) problems.push("pool");
  if (!f.plan) problems.push("plan");
  if (f.kind !== "supply" && f.kind !== "demand") problems.push("kind");
  if (!/^\d{4}-(0[1-9]|1[0-2])$/.test(f.month.trim())) problems.push("month");
  const committed = figure(f.committed, locale);
  if (committed === null) problems.push("committed");
  let delivered: number | null = null;
  if (f.delivered.trim() !== "") {
    delivered = figure(f.delivered, locale);
    if (delivered === null) problems.push("delivered");
  }
  if (problems.length > 0 || committed === null)
    return { body: null, problems };
  const body: CommitmentInput = {
    pool_pid: f.pool,
    plan_pid: f.plan,
    kind: f.kind as "supply" | "demand",
    month: f.month.trim(),
    committed_fte: committed,
  };
  const org = f.organization.trim();
  if (org) body.organization = org;
  if (delivered !== null) body.delivered_fte = delivered;
  const note = f.note.trim();
  if (note) body.note = note;
  return { body, problems };
}

/** A send's figure in words: the full-time equivalents, "a withdrawal" at zero, "nothing" when none was sent. */
export function sentFigure(
  send: PeerDemandSend,
  locale: string = currentLocale(),
): string {
  if (send.fte_centi === null) return t("p.commitments.sent_nothing");
  if (send.fte_centi === 0) return t("p.commitments.sent_withdrawal");
  return t("p.commitments.sent_fte", {
    fte: fte(send.fte_centi / 100, locale),
  });
}

/**
 * What happened to the demand sent to the workforce service, in plain words. `undefined` (the answer has no
 * `peer_demand_send`) means the commitment is not sent at all (supply, or a pool that is not linked); `null`
 * means the pool is linked and nothing is recorded yet.
 */
export function sendOutcome(
  send: PeerDemandSend | null | undefined,
  locale: string = currentLocale(),
): string {
  if (send === undefined) return t("p.commitments.out_not_sent_at_all");
  if (send === null) return t("p.commitments.out_none_yet");
  const zero = send.fte_centi === 0;
  const amount =
    send.fte_centi === null ? "" : fte(send.fte_centi / 100, locale);
  switch (send.outcome) {
    case "started":
      return zero
        ? t("p.commitments.out_started_zero")
        : t("p.commitments.out_started", { fte: amount });
    case "sent":
      return zero
        ? t("p.commitments.out_sent_zero")
        : t("p.commitments.out_sent", { fte: amount });
    case "duplicate":
      return t("p.commitments.out_duplicate");
    case "refused":
      return t("p.commitments.out_refused");
    case "unreachable":
      return t("p.commitments.out_unreachable");
    case "not_configured":
      return t("p.commitments.out_not_configured");
    case "not_sent_no_token":
      return t("p.commitments.out_no_token");
    case "not_sent_too_large":
      return t("p.commitments.out_too_large");
    case "not_sent_zero":
      return t("p.commitments.out_old_zero");
    default:
      return t("p.commitments.out_other", { outcome: send.outcome });
  }
}

/** One row of the recent sends. */
export interface SendRow {
  key: string;
  pool: string;
  programme: string;
  month: string;
  sent: string;
  outcome: string;
  at: string;
}

/**
 * The recent sends, newest first, at most `limit`: one per pool, programme and month (each organization's demand
 * on the same pool and month shares the one send of the programme's total).
 */
export function recentSends(
  rows: CapacityCommitment[],
  poolName: (pid: string) => string,
  planName: (pid: string) => string,
  limit = 20,
  locale: string = currentLocale(),
): SendRow[] {
  const seen = new Map<string, { c: CapacityCommitment; s: PeerDemandSend }>();
  for (const c of rows) {
    const s = c.peer_demand_send;
    if (c.kind !== "demand" || !s) continue;
    const key = `${c.pool_pid}|${c.plan_pid}|${c.month}`;
    if (!seen.has(key)) seen.set(key, { c, s });
  }
  return [...seen.entries()]
    .sort(([, a], [, b]) => (b.s.at ?? "").localeCompare(a.s.at ?? ""))
    .slice(0, limit)
    .map(([key, { c, s }]) => ({
      key,
      pool: poolName(c.pool_pid),
      programme: planName(c.plan_pid),
      month: monthLabel(c.month, locale),
      sent: sentFigure(s, locale),
      outcome: sendOutcome(s, locale),
      at: s.at ? momentLabel(s.at, locale) : "",
    }));
}
