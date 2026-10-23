// The announcement feed: the words and symbols it shows. Pure and tested.
// Severity is a word and a shape, never colour alone.

import type { Announcement, AnnouncementSeverity } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export const SEVERITY_TEXT: Record<AnnouncementSeverity, string> = {
  get info() {
    return t("p.announcements.info");
  },
  get important() {
    return t("p.announcements.important");
  },
  get urgent() {
    return t("p.announcements.urgent");
  },
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
  if (ms < 0) return t("l.announcement_view.scheduled");
  const minutes = Math.floor(ms / 60_000);
  if (minutes < 1) return t("l.announcement_view.just_now");
  if (minutes < 60) return t("l.announcement_view.min_ago", { minutes });
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return t("l.announcement_view.h_ago", { hours });
  const days = Math.floor(hours / 24);
  return days === 1
    ? t("l.announcement_view.yesterday")
    : t("l.announcement_view.calendar_days_ago", { days });
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
      ? t("l.announcement_view.scheduled")
      : t("l.announcement_view.expired");
}
