// Following a board's changes as they happen (`/plans/[pid]/board`, NC-1 slice 12 part j, PL-3): `GET
// /api/plans/{pid}/live`, a `text/event-stream` read through the proxy with the browser's `EventSource`. The service
// (`controllers/live.rs`, `src/live.rs`) sends `task_created`, `task_moved`, `task_updated`, `raci_updated` and
// `comment_added` with identifiers only, and `resync` when it cannot replay what a reconnecting reader missed; the
// browser reconnects by itself, sending the last event's id. Any event reads the board again (the answers carry no
// text to show). Following is asked for, never started on its own, and stops on leaving the page.

import type { StringKey } from "./i18n.svelte.js";

/** The events the service sends, each with the words for "the last change". */
export const LIVE_EVENT_KEY: Record<string, StringKey> = {
  task_created: "p.live.event_task_created",
  task_moved: "p.live.event_task_moved",
  task_updated: "p.live.event_task_updated",
  raci_updated: "p.live.event_raci_updated",
  comment_added: "p.live.event_comment_added",
  resync: "p.live.event_resync",
};

/** The event names to listen for (an `EventSource` hears a named event only when asked). */
export const LIVE_EVENTS: readonly string[] = Object.keys(LIVE_EVENT_KEY);

/** Where following stands. */
export type LiveState = "off" | "connecting" | "on" | "retrying" | "stopped";

/** The words for where following stands. */
export const LIVE_STATE_KEY: Record<LiveState, StringKey> = {
  off: "p.live.state_off",
  connecting: "p.live.state_connecting",
  on: "p.live.state_on",
  retrying: "p.live.state_retrying",
  stopped: "p.live.state_stopped",
};

/** The words for an event; one the page does not know is a general change. */
export function liveEventKey(kind: string): StringKey {
  return LIVE_EVENT_KEY[kind] ?? "p.live.event_other";
}

/**
 * Where following stands after the stream fails: the browser retries a dropped connection by itself (`CONNECTING`),
 * but gives up on a refusal such as a 404 (`CLOSED`), which is said as stopped.
 */
export function stateAfterError(readyState: number): LiveState {
  return readyState === 2 ? "stopped" : "retrying";
}
