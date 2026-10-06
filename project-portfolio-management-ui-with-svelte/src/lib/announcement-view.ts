// The announcement feed: the words and symbols it shows. Pure and tested.
// Severity is a word and a shape, never colour alone.

import type { Announcement, AnnouncementSeverity } from "#lib/api/ppm.js";

export const SEVERITY_TEXT: Record<AnnouncementSeverity, string> = {
  info: "Info",
  important: "Important",
  urgent: "Urgent",
};

/** A decorative shape per severity (hide it from screen readers; the word is there). */
export const SEVERITY_SYMBOL: Record<AnnouncementSeverity, string> = {
  info: "○",
  important: "▲",
  urgent: "■",
};

/** How long ago, in words; a future time says "scheduled". */
export function ageText(iso: string, now: Date): string {
  const ms = now.getTime() - new Date(iso).getTime();
  if (Number.isNaN(ms)) return "";
  if (ms < 0) return "scheduled";
  const minutes = Math.floor(ms / 60_000);
  if (minutes < 1) return "just now";
  if (minutes < 60) return `${minutes} min ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours} h ago`;
  const days = Math.floor(hours / 24);
  return days === 1 ? "yesterday" : `${days} days ago`;
}

/** The first `max` characters of a body, cut at a word, with an ellipsis only if cut. */
export function excerpt(body: string, max: number): string {
  const text = body.replace(/\s+/g, " ").trim();
  if (text.length <= max) return text;
  const cut = text.slice(0, max);
  const at = cut.lastIndexOf(" ");
  return `${(at > max / 2 ? cut.slice(0, at) : cut).trimEnd()}…`;
}

/** The state of an announcement as a short label ("" when live). */
export function stateLabel(a: Pick<Announcement, "state">): string {
  return a.state === "live"
    ? ""
    : a.state === "scheduled"
      ? "Scheduled"
      : "Expired";
}
