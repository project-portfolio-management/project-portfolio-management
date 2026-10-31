// Display preferences (LT-12): "simple mode" and a dyslexia-friendly font. Pure
// rules, so they are unit-testable without a component.
//
// Simple mode changes **prominence, not capability**: the advanced destinations
// stay one step away behind a "More" disclosure, and every route stays reachable
// by URL. It is on by default for a worker persona (the deployment's `view`
// attribute is `me`) and an opt-in for everyone else; an explicit choice, once
// made, always wins over that default.
//
// Preferences are kept on the device (so they work signed out) and, when
// signed in, as one saved view of the user's own (`PREFERENCES_ROUTE`), so they
// follow the person to another browser.

import { parseWeekStart, type WeekStart } from "./week-start.js";

/** The saved-view route that holds a user's display preferences. */
export const PREFERENCES_ROUTE = "/__preferences";
/** The saved-view name. */
export const PREFERENCES_NAME = "display";
/** The device-storage key. */
export const STORAGE_KEY = "mxi.display";

/** A person's display choices. A missing field means "not chosen yet". */
export interface Prefs {
  /** Simple mode: `undefined` until the person chooses. */
  simple?: boolean;
  /** The dyslexia-friendly font. */
  dyslexia?: boolean;
  /**
   * Sections of the top bar this person chose to hide, by path (`/okrs`). Absent until they hide one: a person who
   * never changed it follows the current default, so a section added later reaches them, and a choice is only ever
   * a section they themselves hid (BR-17).
   */
  hidden?: string[];
  /** The first day of the week in calendars (WW-1.4); absent means the page's locale's. */
  weekStart?: Exclude<WeekStart, "locale">;
}

/** The most sections one person can hide. */
export const MAX_HIDDEN = 100;
/** Paths that cannot be hidden: the way home. (The Display page, where a choice is undone, is not in the bar.) */
export const NEVER_HIDDEN: readonly string[] = ["/"];

/** A hidden-section list read back: only in-app paths, once each, within the cap; `undefined` when nothing is left. */
export function parseHidden(raw: unknown): string[] | undefined {
  if (!Array.isArray(raw)) return undefined;
  const seen = new Set<string>();
  for (const h of raw) {
    if (typeof h !== "string" || h.length > 100) continue;
    if (!/^\/(?![/\\])/.test(h) || NEVER_HIDDEN.includes(h)) continue;
    seen.add(h);
    if (seen.size === MAX_HIDDEN) break;
  }
  return seen.size > 0 ? [...seen] : undefined;
}

/** `prefs` with `href` hidden or shown again. Showing the last hidden section removes the field altogether. */
export function withHidden(prefs: Prefs, href: string, hide: boolean): Prefs {
  const now = new Set(prefs.hidden ?? []);
  if (hide) now.add(href);
  else now.delete(href);
  const next: Prefs = { ...prefs };
  const hidden = parseHidden([...now]);
  if (hidden) next.hidden = hidden;
  else delete next.hidden;
  return next;
}

/** The nav items minus what this person hid. Nothing else is touched, so a new item always shows. */
export function applyHidden<T extends { href: string }>(
  items: T[],
  prefs: Prefs,
): T[] {
  const hidden = new Set(prefs.hidden ?? []);
  if (hidden.size === 0) return items;
  return items.filter(
    (i) => !hidden.has(i.href) || NEVER_HIDDEN.includes(i.href),
  );
}

/**
 * Run async tasks one after another, in the order they were asked for, whatever each outcome (a failure does not stop
 * the ones behind it). Account writes go through one of these, so two quick changes never interleave their
 * list, delete and create steps and leave two copies or none.
 */
export function serialQueue(): <R>(task: () => Promise<R>) => Promise<R> {
  let tail: Promise<unknown> = Promise.resolve();
  return <R>(task: () => Promise<R>): Promise<R> => {
    const run = tail.then(task, task);
    tail = run.catch(() => undefined);
    return run;
  };
}

/** The persona that gets simple mode by default: the worker's `view=me`. */
export const WORKER_VIEW = "me";

/** Whether simple mode is on, given the choices and the deployment's `view`. */
export function effectiveSimple(
  prefs: Prefs,
  view: string | null | undefined,
): boolean {
  if (typeof prefs.simple === "boolean") return prefs.simple;
  return view === WORKER_VIEW;
}

/** Parse stored or received preferences, ignoring anything that is not a boolean. */
export function parsePrefs(raw: unknown): Prefs {
  let value = raw;
  if (typeof raw === "string") {
    try {
      value = JSON.parse(raw);
    } catch {
      return {};
    }
  }
  if (typeof value !== "object" || value === null) return {};
  const v = value as Record<string, unknown>;
  const out: Prefs = {};
  if (typeof v.simple === "boolean") out.simple = v.simple;
  if (typeof v.dyslexia === "boolean") out.dyslexia = v.dyslexia;
  const hidden = parseHidden(v.hidden);
  if (hidden) out.hidden = hidden;
  const weekStart = parseWeekStart(v.weekStart);
  if (weekStart && weekStart !== "locale") out.weekStart = weekStart;
  return out;
}

/** The nav destinations simple mode keeps in front: what a worker needs day to day. */
export const SIMPLE_NAV_HREFS = [
  "/me",
  "/plans",
  "/board",
  "/calendar",
  "/tour",
] as const;

export interface NavLink {
  href: string;
  label: string;
}

/**
 * Split the nav into what stays in front and what moves behind "More". With
 * simple mode off everything is in front. Nothing is ever dropped: front plus
 * more is always the whole list, in its original order within each part.
 */
export function splitNav<T extends NavLink>(
  items: T[],
  simple: boolean,
): { front: T[]; more: T[] } {
  if (!simple) return { front: items, more: [] };
  const keep = new Set<string>(SIMPLE_NAV_HREFS);
  return {
    front: items.filter((i) => i.href === "/" || keep.has(i.href)),
    more: items.filter((i) => i.href !== "/" && !keep.has(i.href)),
  };
}

/** The plan workspace links simple mode keeps in front. */
export const SIMPLE_PLAN_LINKS = ["governance", "board", "schedule"] as const;

/** Split a plan's workspace links the same way. Nothing is dropped. */
export function splitPlanLinks<T extends string>(
  links: readonly T[],
  simple: boolean,
): { front: T[]; more: T[] } {
  if (!simple) return { front: [...links], more: [] };
  const keep = new Set<string>(SIMPLE_PLAN_LINKS);
  return {
    front: links.filter((l) => keep.has(l)),
    more: links.filter((l) => !keep.has(l)),
  };
}

/** The newest saved view holding display preferences, if the user has one. */
export function fromSavedViews(
  views: { route: string; name: string; filter: unknown }[],
): { prefs: Prefs } | null {
  const mine = views.filter(
    (v) => v.route === PREFERENCES_ROUTE && v.name === PREFERENCES_NAME,
  );
  const last = mine[mine.length - 1];
  return last ? { prefs: parsePrefs(last.filter) } : null;
}

/** The saved-view body that stores these preferences. */
export function toSavedView(prefs: Prefs): {
  route: string;
  name: string;
  filter: Prefs;
  sort: object;
  columns: string[];
} {
  return {
    route: PREFERENCES_ROUTE,
    name: PREFERENCES_NAME,
    filter: parsePrefs(prefs),
    sort: {},
    columns: [],
  };
}

/** The `<html>` attributes that express the preferences (CSS keys on these). */
export function htmlAttributes(
  simple: boolean,
  dyslexia: boolean,
): { "data-simple": string; "data-font": string } {
  return {
    "data-simple": simple ? "on" : "off",
    "data-font": dyslexia ? "dyslexia" : "default",
  };
}
