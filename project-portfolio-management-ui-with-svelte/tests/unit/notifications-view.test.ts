// NC-1 slice 4 (PL-5, LT-6, RM-11): what a notification choice is when nothing is stored, the bodies that change one,
// the checks on a new destination, and the refusal sentences. The rules mirror the service's
// `src/notification_prefs.rs` and `src/sinks.rs`; the service still decides.

import { describe, expect, it } from "vitest";
import type { NotificationPrefs } from "../../src/lib/api/ppm-types";
import {
  CHANNELS,
  MAX_SINKS,
  SINK_KINDS,
  choiceFor,
  digestBody,
  emailShaped,
  emptySinkForm,
  notificationRefusalKey,
  pidsToMark,
  privateHost,
  sinkBody,
  sinkProblem,
  storedChannels,
  subjectPath,
  toggleBody,
} from "../../src/lib/notifications-view";

const prefs = (rows: NotificationPrefs["prefs"] = []): NotificationPrefs => ({
  prefs: rows,
  known_kinds: ["comment", "task_assigned"],
  channels: ["in_app", "webhook", "chat", "email"],
  digests: ["instant", "daily"],
  default: { enabled: true, digest: "instant" },
});

const row = (
  kind: string,
  channel: string,
  enabled: boolean,
  digest = "instant",
) => ({
  pid: `${kind}-${channel}`,
  kind,
  channel,
  enabled,
  digest,
});

describe("a notification choice", () => {
  it("is the default when nothing is stored: on and instant", () => {
    expect(choiceFor(prefs(), "comment", "in_app")).toEqual({
      enabled: true,
      digest: "instant",
      stored: false,
    });
  });

  it("is the stored row when there is one, per kind and channel", () => {
    const p = prefs([
      row("comment", "in_app", true, "daily"),
      row("comment", "chat", false),
    ]);
    expect(choiceFor(p, "comment", "in_app")).toEqual({
      enabled: true,
      digest: "daily",
      stored: true,
    });
    expect(choiceFor(p, "comment", "chat").enabled).toBe(false);
    expect(choiceFor(p, "comment", "email")).toEqual({
      enabled: true,
      digest: "instant",
      stored: false,
    });
    expect(choiceFor(p, "task_assigned", "chat").stored).toBe(false);
  });

  it("names the channels a kind has a stored choice on, in the page's order", () => {
    const p = prefs([
      row("comment", "email", false),
      row("comment", "in_app", false),
    ]);
    expect(storedChannels(p, "comment")).toEqual(["in_app", "email"]);
    expect(storedChannels(p, "task_assigned")).toEqual([]);
  });

  it("switching on or off keeps the in-app digest, and sends instant on any other channel", () => {
    const p = prefs([row("comment", "in_app", true, "daily")]);
    expect(toggleBody(p, "comment", "in_app", false)).toEqual({
      kind: "comment",
      channel: "in_app",
      enabled: false,
      digest: "daily",
    });
    expect(toggleBody(p, "comment", "webhook", false)).toEqual({
      kind: "comment",
      channel: "webhook",
      enabled: false,
      digest: "instant",
    });
  });

  it("setting the digest keeps whether the kind is on in the inbox", () => {
    const p = prefs([row("comment", "in_app", false)]);
    expect(digestBody(p, "comment", true)).toEqual({
      kind: "comment",
      channel: "in_app",
      enabled: false,
      digest: "daily",
    });
    expect(digestBody(prefs(), "task_assigned", false).digest).toBe("instant");
  });

  it("offers the inbox first, then the three kinds of destination", () => {
    expect(CHANNELS).toEqual(["in_app", "webhook", "chat", "email"]);
  });
});

describe("a new destination", () => {
  const form = (over: Partial<ReturnType<typeof emptySinkForm>> = {}) => ({
    ...emptySinkForm(),
    ...over,
  });

  it("offers the service's six kinds", () => {
    expect([...SINK_KINDS].sort()).toEqual([
      "discord",
      "email",
      "mattermost",
      "slack",
      "telegram",
      "webhook",
    ]);
  });

  it("is refused when the person already holds ten", () => {
    expect(
      sinkProblem(form({ destination: "me@example.com" }), MAX_SINKS),
    ).toBe("too_many");
    expect(
      sinkProblem(form({ destination: "me@example.com" }), MAX_SINKS - 1),
    ).toBeNull();
  });

  it("needs an address, and for email an address shaped as the service wants", () => {
    expect(sinkProblem(form({ destination: "  " }), 0)).toBe(
      "destination_required",
    );
    expect(sinkProblem(form({ destination: "nobody" }), 0)).toBe(
      "email_invalid",
    );
    expect(emailShaped("a@example.com")).toBe(true);
    expect(emailShaped("a@b@example.com")).toBe(false);
    expect(emailShaped("a@localhost")).toBe(false);
    expect(emailShaped("a@.example.com")).toBe(false);
    expect(emailShaped("a b@example.com")).toBe(false);
    expect(emailShaped("a@example.com\nBcc: x@y.z")).toBe(false);
    expect(emailShaped("a@example.com,b@example.com")).toBe(false);
  });

  it("needs https to a public host with no credentials for a web destination", () => {
    const web = (destination: string, kind: "webhook" | "slack" = "webhook") =>
      sinkProblem(form({ kind, destination }), 0);
    expect(web("https://hooks.example.com/abc")).toBeNull();
    expect(web("not a url")).toBe("url_invalid");
    expect(web("http://hooks.example.com/abc")).toBe("url_not_https");
    expect(web("https://user:pw@hooks.example.com/")).toBe("url_credentials");
    expect(web("https://localhost/x")).toBe("url_private");
    expect(web("https://127.0.0.1/x")).toBe("url_private");
    expect(web("https://169.254.169.254/latest")).toBe("url_private");
    expect(web("https://10.1.2.3/x")).toBe("url_private");
    expect(web("https://printer.local/x")).toBe("url_private");
    expect(web("https://api.internal/x")).toBe("url_private");
    expect(web("https://[::1]/x")).toBe("url_private");
    expect(web(`https://example.com/${"a".repeat(2048)}`)).toBe("url_too_long");
  });

  it("knows which addresses are not public, as the service's rule does", () => {
    for (const h of [
      "0.0.0.0",
      "100.64.0.1",
      "172.16.0.1",
      "172.31.255.255",
      "192.168.1.1",
      "192.0.2.1",
      "198.51.100.7",
      "203.0.113.9",
      "224.0.0.1",
      "255.255.255.255",
      "fd00::1",
      "fe80::1",
      "::",
    ])
      expect(privateHost(h), h).toBe(true);
    for (const h of [
      "8.8.8.8",
      "172.32.0.1",
      "100.128.0.1",
      "hooks.slack.com",
      "2001:4860::8888",
      "intranet",
    ])
      expect(privateHost(h), h).toBe(false);
  });

  it("needs a chat for Telegram, and sends it as the only setting", () => {
    const tg = form({
      kind: "telegram",
      destination: "https://api.telegram.org/botX/sendMessage",
    });
    expect(sinkProblem(tg, 0)).toBe("chat_id_required");
    expect(sinkProblem({ ...tg, chatId: " 42 " }, 0)).toBeNull();
    expect(sinkBody({ ...tg, chatId: " 42 " })).toEqual({
      kind: "telegram",
      destination: "https://api.telegram.org/botX/sendMessage",
      settings: { chat_id: "42" },
    });
    expect(sinkBody(form({ destination: " me@example.com " }))).toEqual({
      kind: "email",
      destination: "me@example.com",
    });
  });
});

describe("an inbox line", () => {
  it("marks every notification it stands for when any is unread, and none when all are read", () => {
    expect(pidsToMark({ pids: ["a", "b"], unread: true })).toEqual(["a", "b"]);
    expect(pidsToMark({ pids: ["a"], unread: false })).toEqual([]);
  });

  it("links to a plan, and to nothing it cannot name", () => {
    expect(subjectPath({ subject_kind: "plan", subject_pid: "p1" })).toBe(
      "/plans/p1",
    );
    expect(subjectPath({ subject_kind: "portfolio", subject_pid: "p1" })).toBe(
      "/plans/p1",
    );
    expect(subjectPath({ subject_kind: "task", subject_pid: "t1" })).toBeNull();
  });
});

describe("refusals", () => {
  it("say which step was refused and why, by status", () => {
    expect(notificationRefusalKey("set_pref", 403)).toBe(
      "p.notifications.refused_forbidden",
    );
    expect(notificationRefusalKey("mark_read", 404)).toBe(
      "p.notifications.refused_note_gone",
    );
    expect(notificationRefusalKey("remove_sink", 404)).toBe(
      "p.notifications.refused_sink_gone",
    );
    expect(notificationRefusalKey("deliveries", 404)).toBe(
      "p.notifications.refused_sink_gone",
    );
    expect(notificationRefusalKey("set_pref", 422)).toBe(
      "p.notifications.refused_pref_invalid",
    );
    expect(notificationRefusalKey("reset_pref", 422)).toBe(
      "p.notifications.refused_pref_invalid",
    );
    expect(notificationRefusalKey("add_sink", 422)).toBe(
      "p.notifications.refused_sink_invalid",
    );
    expect(notificationRefusalKey("inbox", 422)).toBe(
      "p.notifications.refused_not_addressable",
    );
    expect(notificationRefusalKey("prefs", 422)).toBe(
      "p.notifications.refused_not_addressable",
    );
    expect(notificationRefusalKey("sinks", 422)).toBe(
      "p.notifications.refused_not_addressable",
    );
  });

  it("leave anything else to the general rule", () => {
    expect(notificationRefusalKey("inbox", 404)).toBeNull();
    expect(notificationRefusalKey("add_sink", 409)).toBeNull();
    expect(notificationRefusalKey("add_sink", 500)).toBeNull();
  });
});
