// Comments, reactions, following and attachments on a subject (NC-1 slice 5: LT-4, LT-5, RM-3). Pure: no runes, no
// `$app/*`. The rules mirror the service so the page can say what is wrong before a round trip, in the reader's
// language: `src/discussion.rs` (a comment is not blank and at most 4000 characters; five reactions) and
// `src/artifacts.rs` (twelve content types, never HTML, SVG, archives or programs; not empty; at most 10 MiB by
// default). The service decides: its limit can be set lower or higher (`PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES`),
// it checks that the content is what the type says, and an optional malware scan has the last word.

import type { ReactionKind } from "./api/ppm-types.js";

/** The service's cap on one comment, in characters (`discussion::MAX_COMMENT_LEN`). */
export const MAX_COMMENT_CHARS = 4000;

/** The default cap on one file (`artifacts::DEFAULT_MAX_BYTES`); a deployment may set another. */
export const MAX_ATTACHMENT_BYTES = 10 * 1024 * 1024;

/** The reactions, in the order the page shows them, with the symbol drawn beside the words. */
export const REACTIONS: readonly { kind: ReactionKind; symbol: string }[] = [
  { kind: "thumbs_up", symbol: "👍" },
  { kind: "heart", symbol: "❤️" },
  { kind: "celebrate", symbol: "🎉" },
  { kind: "eyes", symbol: "👀" },
  { kind: "question", symbol: "❓" },
];

/** The content types the service accepts (`artifacts::ALLOWED_TYPES`), each with the word the page shows for it. */
export const ALLOWED_TYPES: Readonly<Record<string, FileTypeWord>> = {
  "application/pdf": "pdf",
  "image/png": "png",
  "image/jpeg": "jpeg",
  "image/gif": "gif",
  "image/webp": "webp",
  "text/plain": "text",
  "text/csv": "csv",
  "text/markdown": "markdown",
  "application/json": "json",
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
    "word",
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet": "excel",
  "application/vnd.openxmlformats-officedocument.presentationml.presentation":
    "powerpoint",
};

/** A short name for a file type, looked up in the catalogue as `p.discussion.type_<word>`. */
export type FileTypeWord =
  | "pdf"
  | "png"
  | "jpeg"
  | "gif"
  | "webp"
  | "text"
  | "csv"
  | "markdown"
  | "json"
  | "word"
  | "excel"
  | "powerpoint";

/**
 * The type for a name when the browser gives none (it often has no type for `.md`, and sometimes none for `.csv` or
 * `.json`). Only these text formats: a file the browser names some other type is sent as the browser named it, and a
 * name is never used to turn a refused type into an accepted one.
 */
const BY_EXTENSION: Readonly<Record<string, string>> = {
  md: "text/markdown",
  markdown: "text/markdown",
  csv: "text/csv",
  json: "application/json",
  txt: "text/plain",
};

/** The `accept` attribute of the file box: the twelve types, so the browser's picker offers those first. */
export const ACCEPT = [
  ...Object.keys(ALLOWED_TYPES),
  ...Object.keys(BY_EXTENSION).map((e) => `.${e}`),
].join(",");

/** What is known of a chosen file. */
export interface ChosenFile {
  name: string;
  size: number;
  /** The browser's type, `""` when it has none. */
  type: string;
}

/** The content type to send for `file`: the browser's, or for a text file it gave no type, the one its name says. */
export function uploadType(file: ChosenFile): string {
  const declared = file.type.split(";")[0]?.trim().toLowerCase() ?? "";
  if (declared !== "") return declared;
  const dot = file.name.lastIndexOf(".");
  const ext = dot < 0 ? "" : file.name.slice(dot + 1).toLowerCase();
  return BY_EXTENSION[ext] ?? "";
}

/** The first thing wrong with a file before it is sent, or `null`. */
export type FileProblem = "choose_file" | "empty" | "too_large" | "type" | null;

/** What the page refuses before sending: no file, an empty one, one over the default cap, or a type not on the list. */
export function fileProblem(file: ChosenFile | null): FileProblem {
  if (!file) return "choose_file";
  if (file.size === 0) return "empty";
  if (file.size > MAX_ATTACHMENT_BYTES) return "too_large";
  if (!(uploadType(file) in ALLOWED_TYPES)) return "type";
  return null;
}

/** The word for a stored type; a type the page does not know (stored before a list change) shows as itself. */
export function typeWord(contentType: string): FileTypeWord | null {
  return ALLOWED_TYPES[contentType.toLowerCase()] ?? null;
}

/** The first thing wrong with a comment's text, or `null`. */
export type CommentProblem = "blank" | "too_long" | null;

/** A comment the service would refuse: blank, or over 4000 characters (counted as the service counts, by code point). */
export function commentProblem(body: string): CommentProblem {
  if (body.trim() === "") return "blank";
  if ([...body].length > MAX_COMMENT_CHARS) return "too_long";
  return null;
}

/** Characters left in a comment box, by code point; negative when over. */
export function charsLeft(body: string): number {
  return MAX_COMMENT_CHARS - [...body].length;
}

/** A size in the reader's units: bytes, KB or MB (1024-based, as the service's limit is), one decimal above a KB. */
export function sizeParts(bytes: number): {
  unit: "b" | "kb" | "mb";
  value: number;
} {
  if (bytes < 1024) return { unit: "b", value: bytes };
  if (bytes < 1024 * 1024)
    return { unit: "kb", value: Math.round((bytes / 1024) * 10) / 10 };
  return { unit: "mb", value: Math.round((bytes / (1024 * 1024)) * 10) / 10 };
}

/** The count shown on a reaction: the service sends every kind, but an absent one is zero. */
export function reactionCount(
  counts: Partial<Record<ReactionKind, number>>,
  kind: ReactionKind,
): number {
  return counts[kind] ?? 0;
}

/** A step that can be refused. */
export type DiscussionStep =
  | "comment"
  | "edit_comment"
  | "delete_comment"
  | "read_history"
  | "react"
  | "watch"
  | "upload"
  | "remove_file";

/** The catalogue sentence for a refused step, or `null` to fall back to the generic text. */
export function discussionRefusalKey(
  step: DiscussionStep,
  status: number,
):
  | "p.discussion.refused_sign_in"
  | "p.discussion.refused_not_author"
  | "p.discussion.refused_not_uploader"
  | "p.discussion.refused_forbidden"
  | "p.discussion.refused_comment_gone"
  | "p.discussion.refused_file_gone"
  | "p.discussion.refused_gone"
  | "p.discussion.refused_comment_invalid"
  | "p.discussion.refused_file_invalid"
  | "p.discussion.refused_too_large"
  | "p.discussion.refused_scan_unavailable"
  | null {
  if (status === 401) return "p.discussion.refused_sign_in";
  if (status === 403) {
    if (step === "edit_comment" || step === "delete_comment")
      return "p.discussion.refused_not_author";
    if (step === "remove_file") return "p.discussion.refused_not_uploader";
    return "p.discussion.refused_forbidden";
  }
  if (status === 404) {
    if (
      step === "edit_comment" ||
      step === "delete_comment" ||
      step === "read_history"
    )
      return "p.discussion.refused_comment_gone";
    if (step === "remove_file") return "p.discussion.refused_file_gone";
    return "p.discussion.refused_gone";
  }
  // 413: the proxy in front of the service refused a body over its own limit.
  if (status === 413) return "p.discussion.refused_too_large";
  if (status === 422) {
    if (step === "upload") return "p.discussion.refused_file_invalid";
    if (step === "comment" || step === "edit_comment")
      return "p.discussion.refused_comment_invalid";
  }
  if (status === 503 && step === "upload")
    return "p.discussion.refused_scan_unavailable";
  return null;
}
