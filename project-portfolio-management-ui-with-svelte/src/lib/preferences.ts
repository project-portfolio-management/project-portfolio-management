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
