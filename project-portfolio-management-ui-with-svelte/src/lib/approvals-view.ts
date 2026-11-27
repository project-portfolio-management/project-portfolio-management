// The rules of the approvals page (`/approvals`, PL-1; NC-1 slice 12 part n), kept out of the component so they are
// unit-tested: what a request and a decision send and every problem with them (the service's: an approver is a
// `person:` or `worker:` reference to a UUID, a note or comment up to 2,000 characters, a rejection says why), and
// which sentence a refusal is.

/** What a request can be about (the service's `discussion::SUBJECT_KINDS`). */
export const SUBJECT_KINDS = [
  "plan",
  "task",
  "risk",
  "proposal",
  "idea",
  "canvas_item",
  "budget_line",
  "scenario",
  "document_version",
] as const;
export type SubjectKind = (typeof SUBJECT_KINDS)[number];

/** The service's cap on a request's note and a decision's comment. */
export const MAX_TEXT = 2000;

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/** Whether `ref` is a `person:` or `worker:` reference to a UUID, as the service checks. */
export function isApproverRef(ref: string): boolean {
  const m = /^(person|worker):(.+)$/.exec(ref.trim());
  return m !== null && UUID.test(m[2]!);
}

/** `POST /api/approvals`. */
export interface ApprovalRequestBody {
  subject_kind: string;
  subject_pid: string;
  approver_ref: string;
  message?: string | null;
}

/** The body a request sends, or every problem with it as catalogue keys. */
export function requestBody(f: {
  subject_kind: string;
  subject_pid: string;
  approver_ref: string;
  message: string;
}): { body: ApprovalRequestBody } | { problems: string[] } {
  const problems: string[] = [];
  if (!UUID.test(f.subject_pid.trim()))
    problems.push("p.approvals.subject_bad");
  if (!isApproverRef(f.approver_ref)) problems.push("p.approvals.approver_bad");
  const message = f.message.trim();
  if ([...message].length > MAX_TEXT) problems.push("p.approvals.text_long");
  if (problems.length > 0) return { problems };
  return {
    body: {
      subject_kind: f.subject_kind,
      subject_pid: f.subject_pid.trim(),
      approver_ref: f.approver_ref.trim(),
      message: message === "" ? null : message,
    },
  };
}

/** The body a decision sends, or the key of its problem: a rejection says why, and a comment is at most 2,000 characters. */
export function decideBody(
  decision: "approved" | "rejected",
  comment: string,
):
  | { body: { decision: "approved" | "rejected"; comment?: string } }
  | { problem: string } {
  const c = comment.trim();
  if ([...c].length > MAX_TEXT) return { problem: "p.approvals.text_long" };
  if (decision === "rejected" && c === "")
    return { problem: "p.approvals.why_required" };
  return { body: c === "" ? { decision } : { decision, comment: c } };
}

/**
 * The sentence for a refused call: 403 is someone other than the approver deciding or the requester withdrawing, 409 a
 * request already decided or withdrawn (a decision is final), 404 a subject or request not there or not readable.
 */
export function refusalKey(
  status: number,
  action: "request" | "decide" | "withdraw",
): string | null {
  if (status === 409) return "p.approvals.already_settled";
  if (status === 404) return "p.approvals.not_found";
  if (status === 403)
    return action === "withdraw"
      ? "p.approvals.not_requester"
      : "p.approvals.not_approver";
  return null;
}
