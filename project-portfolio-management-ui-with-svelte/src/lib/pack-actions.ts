// Pure helpers for drafting, reviewing, issuing and reissuing director packs (DP-1, DP-12, NC-1 slice 11): the
// audiences in the reader's language, the checks the service makes before anything is sent, which actions a pack's
// state allows, and the sentence for each refusal. The page does not know the caller's pack rights; the service
// decides, and a 403 says which right the action needs.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { DirectorPackMeta } from "./api/ppm-types.js";

/** The built-in audiences (`AUDIENCES` in the service's `src/director_pack.rs`), each with its catalogue key. */
export const PACK_AUDIENCES: ReadonlyArray<{ key: string; label: StringKey }> =
  [
    { key: "executive", label: "p.director_pack.aud_executive" },
    { key: "board", label: "p.director_pack.aud_board" },
    { key: "escalation_board", label: "p.director_pack.aud_escalation_board" },
    { key: "partner", label: "p.director_pack.aud_partner" },
  ];

/** A reissue needs a change note of at least this many characters (`MIN_NOTE`). */
export const NOTE_MIN = 10;
/** A cut-off may be at most this many days old (`draft` in `controllers/director_packs.rs`). */
export const CUT_OFF_MAX_DAYS = 400;

/** Every problem with a new draft, as the service checks them: the period, the audience, and the cut-off if given. */
export function draftProblems(
  form: { period: string; audience: string; cutOff: string },
  now: Date,
): StringKey[] {
  const out: StringKey[] = [];
  if (!/^\d{4}-(0[1-9]|1[0-2])$/.test(form.period))
    out.push("p.director_pack.problem_period");
  if (form.audience === "") out.push("p.director_pack.problem_audience");
  if (form.cutOff !== "") {
    const at = new Date(form.cutOff);
    if (Number.isNaN(at.getTime()))
      out.push("p.director_pack.problem_cut_off_bad");
    else if (at.getTime() > now.getTime())
      out.push("p.director_pack.problem_cut_off_future");
    else if (now.getTime() - at.getTime() > CUT_OFF_MAX_DAYS * 86_400_000)
      out.push("p.director_pack.problem_cut_off_old");
  }
  return out;
}

/** The problem with a reissue's change note, or `null`: at least ten characters once trimmed. */
export function noteProblem(note: string): StringKey | null {
  return [...note.trim()].length < NOTE_MIN
    ? "p.director_pack.problem_note"
    : null;
}

/** What a pack's state allows. A scheduled draft must be reviewed before it is issued; an issued pack is reissued. */
export function packActions(
  p: Pick<DirectorPackMeta, "status" | "origin" | "reviewed">,
): {
  review: boolean;
  issue: boolean;
  issueWaitsForReview: boolean;
  reissue: boolean;
} {
  const draft = p.status === "draft";
  const waits = draft && p.origin === "scheduled" && p.reviewed !== true;
  return {
    review: draft && p.reviewed !== true,
    issue: draft && !waits,
    issueWaitsForReview: waits,
    reissue: !draft,
  };
}

export type PackAction = "draft" | "review" | "issue" | "reissue";

/**
 * The sentence for a refusal of `action`: which right a 403 needs, and what a 409 means for that action; `null` for
 * anything else (the page then uses its general sentence).
 */
export function refusalKey(action: PackAction, err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 403)
    return action === "issue"
      ? "p.director_pack.refused_issue"
      : "p.director_pack.refused_edit";
  if (err.status === 409) {
    if (action === "draft") return "p.director_pack.conflict_draft";
    if (action === "review") return "p.director_pack.conflict_review";
    if (action === "issue") return "p.director_pack.conflict_issue";
    return "p.director_pack.conflict_reissue";
  }
  return null;
}
