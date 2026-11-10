// The caller's notifications (NC-1 slice 4, PL-5, LT-6, RM-11): their inbox, their choices per kind and channel,
// and the destinations (sinks) they send copies to. Pure: what a choice is when nothing is stored, the request bodies,
// the checks the service would otherwise refuse, and the refusal sentence for each step.
//
// The rules mirror the service: `src/notification_prefs.rs` (no stored row means the default: on, instant; a digest
// only changes the in-app inbox, which is where the service reads it) and `src/sinks.rs` (six kinds; an email address
// for `email`, otherwise an https URL to a public host with no credentials; Telegram needs a chat id; at most ten per
// person; the destination is a secret and comes back masked). The service decides; this only saves a round trip and
// lets the page say which field is wrong, in the reader's language.

import type {
  InboxItem,
  NotificationChannel,
  NotificationDigest,
  NotificationPref,
  NotificationPrefBody,
  NotificationPrefs,
  NotificationSinkBody,
  SinkKind,
} from "./api/ppm-types.js";

/** The channels, in the order the page shows them. The in-app inbox comes first. */
export const CHANNELS: readonly NotificationChannel[] = [
  "in_app",
  "webhook",
  "chat",
  "email",
];

/** The sink kinds, in the order the add form offers them. */
export const SINK_KINDS: readonly SinkKind[] = [
  "email",
  "webhook",
  "slack",
  "mattermost",
  "discord",
  "telegram",
];

/** The chat kinds; each is named by its product, which the page puts inside a translated sentence. */
export const CHAT_NAMES: Readonly<Record<string, string>> = {
  slack: "Slack",
  mattermost: "Mattermost",
  discord: "Discord",
  telegram: "Telegram",
};

/** The service's cap on the destinations one person may hold (`sinks::MAX_PER_PERSON`). */
export const MAX_SINKS = 10;
/** The service's cap on a URL destination's length, in characters. */
export const MAX_URL_LENGTH = 2048;

/** What applies to one kind on one channel: the stored choice, or the default when nothing is stored. */
export interface Choice {
  enabled: boolean;
  digest: NotificationDigest;
  /** Whether a choice is stored (so "back to the default" has something to undo). */
  stored: boolean;
}

/** The choice in force for `kind` on `channel`. */
export function choiceFor(
  prefs: Pick<NotificationPrefs, "prefs" | "default">,
  kind: string,
  channel: NotificationChannel,
): Choice {
  const row: NotificationPref | undefined = prefs.prefs.find(
    (p) => p.kind === kind && p.channel === channel,
  );
  const digest =
    (row?.digest ?? prefs.default.digest) === "daily" ? "daily" : "instant";
  return {
    enabled: row ? row.enabled : prefs.default.enabled,
    digest,
    stored: row !== undefined,
  };
}

/** The channels a kind has a stored choice on, which "back to the default" resets. */
export function storedChannels(
  prefs: Pick<NotificationPrefs, "prefs">,
  kind: string,
): NotificationChannel[] {
  return CHANNELS.filter((c) =>
    prefs.prefs.some((p) => p.kind === kind && p.channel === c),
  );
}

/**
 * The body that switches `kind` on or off on `channel`, keeping its digest. A digest only means something in the
 * in-app inbox, so on any other channel it is always `instant`.
 */
export function toggleBody(
  prefs: Pick<NotificationPrefs, "prefs" | "default">,
  kind: string,
  channel: NotificationChannel,
  enabled: boolean,
): NotificationPrefBody {
  const digest =
    channel === "in_app" ? choiceFor(prefs, kind, channel).digest : "instant";
  return { kind, channel, enabled, digest };
}

/** The body that sets the in-app digest of `kind`, keeping whether it is on. */
export function digestBody(
  prefs: Pick<NotificationPrefs, "prefs" | "default">,
  kind: string,
  daily: boolean,
): NotificationPrefBody {
  return {
    kind,
    channel: "in_app",
    enabled: choiceFor(prefs, kind, "in_app").enabled,
    digest: daily ? "daily" : "instant",
  };
}

/** The add-destination form: every field a string. */
export interface SinkForm {
  kind: SinkKind;
  destination: string;
  chatId: string;
}

/** A form with nothing typed. */
export function emptySinkForm(): SinkForm {
  return { kind: "email", destination: "", chatId: "" };
}

/** The first thing wrong with the add form, or `null` when it can be sent. */
export type SinkProblem =
  | "too_many"
  | "destination_required"
  | "email_invalid"
  | "url_invalid"
  | "url_not_https"
  | "url_credentials"
  | "url_private"
  | "url_too_long"
  | "chat_id_required"
  | null;

/** As the service's `email_shaped`: one `@`, a dotted domain, no spaces, no characters a mail header splits on. */
export function emailShaped(s: string): boolean {
  const at = s.indexOf("@");
  if (at <= 0 || s.indexOf("@", at + 1) >= 0 || s.length > 254) return false;
  const domain = s.slice(at + 1);
  if (!domain.includes(".") || domain.startsWith(".") || domain.endsWith("."))
    return false;
  // eslint-disable-next-line no-control-regex
  return !/[\s\u0000-\u001f\u007f,;<>]/.test(s);
}

const IPV4 = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/;

/**
 * Whether a host is one the service never sends to, as its `destination_url_ok` decides: `localhost`, a name ending in
 * `.local`, `.internal` or `.localhost`, or an address literal that is loopback, private, link-local (the cloud
 * metadata address among them), carrier-grade NAT, unspecified, broadcast, multicast, documentation or unique-local.
 * A name that resolves to such an address is the service's to catch when it sends.
 */
export function privateHost(hostname: string): boolean {
  const host = hostname.toLowerCase().replace(/^\[|\]$/g, "");
  const tld = host.split(".").pop() ?? "";
  if (host === "localhost" || ["local", "internal", "localhost"].includes(tld))
    return true;
  const v4 = IPV4.exec(host);
  if (v4) {
    const [a, b, c] = [Number(v4[1]), Number(v4[2]), Number(v4[3])];
    return (
      a === 0 ||
      a === 10 ||
      a === 127 ||
      (a === 100 && b >= 64 && b <= 127) ||
      (a === 169 && b === 254) ||
      (a === 172 && b >= 16 && b <= 31) ||
      (a === 192 && b === 168) ||
      (a === 192 && b === 0 && c === 2) ||
      (a === 198 && b === 51 && c === 100) ||
      (a === 203 && b === 0 && c === 113) ||
      (a >= 224 && a <= 239) ||
      host === "255.255.255.255"
    );
  }
  if (host.includes(":")) {
    return (
      host === "::" ||
      host === "::1" ||
      /^f[cd][0-9a-f]{0,2}:/.test(host) ||
      /^fe[89ab][0-9a-f]?:/.test(host) ||
      host.startsWith("ff")
    );
  }
  return false;
}

/** The first thing wrong with the form, given how many destinations the person already holds. */
export function sinkProblem(f: SinkForm, held: number): SinkProblem {
  if (held >= MAX_SINKS) return "too_many";
  const dest = f.destination.trim();
  if (dest === "") return "destination_required";
  if (f.kind === "email") return emailShaped(dest) ? null : "email_invalid";
  if (dest.length > MAX_URL_LENGTH) return "url_too_long";
  let url: URL;
  try {
    url = new URL(dest);
  } catch {
    return "url_invalid";
  }
  if (url.protocol !== "https:") return "url_not_https";
  if (url.username !== "" || url.password !== "") return "url_credentials";
  if (privateHost(url.hostname)) return "url_private";
  if (f.kind === "telegram" && f.chatId.trim() === "")
    return "chat_id_required";
  return null;
}

/** The request body for the form. Only Telegram carries settings. */
export function sinkBody(f: SinkForm): NotificationSinkBody {
  const body: NotificationSinkBody = {
    kind: f.kind,
    destination: f.destination.trim(),
  };
  if (f.kind === "telegram") body.settings = { chat_id: f.chatId.trim() };
  return body;
}

/** The pids of an inbox line that still need marking read: all of them when any is unread (marking is idempotent). */
export function pidsToMark(item: Pick<InboxItem, "pids" | "unread">): string[] {
  return item.unread ? [...item.pids] : [];
}

/** The in-app path of a line's subject, when the page can name one: a plan. Anything else is shown without a link. */
export function subjectPath(
  item: Pick<InboxItem, "subject_kind" | "subject_pid">,
): string | null {
  return item.subject_kind === "plan" || item.subject_kind === "portfolio"
    ? `/plans/${item.subject_pid}`
    : null;
}

/** A step a person takes from this screen. */
export type NotificationStep =
  | "inbox"
  | "mark_read"
  | "prefs"
  | "set_pref"
  | "reset_pref"
  | "sinks"
  | "add_sink"
  | "remove_sink"
  | "deliveries";

/**
 * The catalogue sentence for a refusal of `step`, by HTTP status: 403 that the caller may not do it, 404 that the
 * notification or destination has gone (or is someone else's: the service answers the two alike), 422 what was not
 * accepted. These routes answer no 409. `null` for anything else, so the page uses the general rule.
 */
export function notificationRefusalKey(
  step: NotificationStep,
  status: number,
):
  | "p.notifications.refused_forbidden"
  | "p.notifications.refused_note_gone"
  | "p.notifications.refused_sink_gone"
  | "p.notifications.refused_not_addressable"
  | "p.notifications.refused_pref_invalid"
  | "p.notifications.refused_sink_invalid"
  | null {
  if (status === 403) return "p.notifications.refused_forbidden";
  if (status === 404) {
    if (step === "mark_read") return "p.notifications.refused_note_gone";
    if (step === "remove_sink" || step === "deliveries")
      return "p.notifications.refused_sink_gone";
    return null;
  }
  if (status === 422) {
    if (step === "set_pref" || step === "reset_pref")
      return "p.notifications.refused_pref_invalid";
    if (step === "add_sink") return "p.notifications.refused_sink_invalid";
    // Reading the inbox, the choices or the destinations: the sign-in identity is not a person the service can
    // address (or, for the inbox, the browser's time zone was refused).
    return "p.notifications.refused_not_addressable";
  }
  return null;
}
