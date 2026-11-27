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

// ---- changing one (`PUT /api/announcements/{pid}`, NC-1 slice 12 part n) ----

/** An announcement as its edit form holds it; the times are `datetime-local` values in the reader's own zone. */
export interface AnnouncementForm {
  title: string;
  body: string;
  severity: AnnouncementSeverity;
  pinned: boolean;
  publish: string;
  expires: string;
}

const two = (n: number) => String(n).padStart(2, "0");

/** An instant as a `datetime-local` value in this browser's zone, to the minute; "" for none. */
export function localInput(iso: string | null | undefined): string {
  if (!iso) return "";
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return "";
  return `${d.getFullYear()}-${two(d.getMonth() + 1)}-${two(d.getDate())}T${two(d.getHours())}:${two(d.getMinutes())}`;
}

/** The form for a stored announcement. */
export function formOf(a: Announcement): AnnouncementForm {
  return {
    title: a.title,
    body: a.body,
    severity: a.severity,
    pinned: a.pinned,
    publish: localInput(a.publish_at),
    expires: localInput(a.expires_at),
  };
}

/** The same instant to the minute (the form shows minutes, so seconds a stored time has are not a change). */
const sameMinute = (
  a: string | null | undefined,
  b: string | null | undefined,
) => localInput(a) === localInput(b);

/**
 * What an edit sends, every problem with it as catalogue keys (the service's: a title up to 120 characters, a message
 * up to 4,000, an expiry after the publishing time), or `null` when it changes nothing. The service replaces the
 * whole announcement, and an absent expiry clears it, so the expiry is always sent; the publishing time is sent only
 * when it changed, so one set to the second stays as it was.
 */
export function announcementEdit(
  f: AnnouncementForm,
  a: Announcement,
):
  | {
      body: {
        title: string;
        body: string;
        severity: AnnouncementSeverity;
        pinned: boolean;
        publish_at?: string;
        expires_at: string | null;
      };
    }
  | { problems: string[] }
  | null {
  const problems: string[] = [];
  const title = f.title.trim();
  const body = f.body.trim();
  if (title === "" || [...title].length > 120)
    problems.push("p.announcements.title_bad");
  if (body === "" || [...body].length > 4000)
    problems.push("p.announcements.body_bad");
  const publish = f.publish ? new Date(f.publish) : new Date(a.publish_at);
  const expires = f.expires ? new Date(f.expires) : null;
  if (
    Number.isNaN(publish.getTime()) ||
    (expires !== null && Number.isNaN(expires.getTime()))
  )
    problems.push("p.announcements.time_bad");
  else if (expires !== null && expires.getTime() <= publish.getTime())
    problems.push("p.announcements.expires_bad");
  if (problems.length > 0) return { problems };
  const publishChanged = !sameMinute(publish.toISOString(), a.publish_at);
  const expiresIso = expires ? expires.toISOString() : null;
  const unchanged =
    title === a.title &&
    body === a.body &&
    f.severity === a.severity &&
    f.pinned === a.pinned &&
    !publishChanged &&
    (expiresIso === null
      ? !a.expires_at
      : sameMinute(expiresIso, a.expires_at));
  if (unchanged) return null;
  return {
    body: {
      title,
      body,
      severity: f.severity,
      pinned: f.pinned,
      ...(publishChanged ? { publish_at: publish.toISOString() } : {}),
      // Unchanged to the minute, the stored expiry is sent back as it is, seconds and all.
      expires_at:
        expiresIso === null
          ? null
          : sameMinute(expiresIso, a.expires_at)
            ? (a.expires_at ?? null)
            : expiresIso,
    },
  };
}
