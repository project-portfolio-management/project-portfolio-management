// What the site remembers in the reader's browser, and where: only choices the reader makes in the pickers (theme,
// language, time zone, text size), never a guess and never an identifier, in localStorage (the default) or in
// first-party cookies (a build with VITE_THEME_STORAGE=cookie). PRIVACY.md is the notice that says so to readers.
//
// Lily's pickers, given a storage key, store every value they apply, including the one they guessed from the browser
// (its light/dark setting, its time zone, the normal text size), so after a first visit the browser's setting no
// longer counted and the reader had a stored value they never chose. The layout therefore gives the pickers no storage
// key and keeps the reader's choices here. The theme rules are the UI's (`src/lib/theme-choice.ts`, the same key), so
// a theme chosen in either is read the same way.
//
// No imports: `node --test tests/unit/` runs this file as TypeScript without a build.

/** Where the reader's own theme choice is kept. */
export const CHOICE_KEY = "ppm-theme-choice";

/** Where the theme picker kept every theme it applied, guessed or chosen, until 2026-11-16 on this site. */
export const LEGACY_KEY = "lily-theme";

/** Where the other choices are kept: the language of the site's own words, the time zone, the text size. */
export const LOCALE_KEY = "lily-locale";
export const TIME_ZONE_KEY = "lily-time-zone";
export const TEXT_SIZE_KEY = "lily-text-size";

/** Every name the site may store, for the cookie move and the privacy notice. */
export const STORED_KEYS = [CHOICE_KEY, LOCALE_KEY, TIME_ZONE_KEY, TEXT_SIZE_KEY] as const;

/** The theme the browser's setting asks for. */
export function systemTheme(prefersDark: boolean): "light" | "dark" {
  return prefersDark ? "dark" : "light";
}

/** The minimal part of `Storage` used here, so the rules can be tested without a browser. */
export interface ChoiceStore {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

/**
 * The reader's chosen theme, or "" to follow the browser. A value under the legacy key moves across once: `light` or
 * `dark` there was most likely the picker's own guess from the browser, so it is dropped; any other theme was picked.
 */
export function readChoice(store: ChoiceStore): string {
  try {
    const legacy = store.getItem(LEGACY_KEY);
    if (legacy !== null) {
      store.removeItem(LEGACY_KEY);
      if (legacy && legacy !== "light" && legacy !== "dark" && store.getItem(CHOICE_KEY) === null) {
        store.setItem(CHOICE_KEY, legacy);
      }
    }
    return store.getItem(CHOICE_KEY) ?? "";
  } catch {
    return "";
  }
}

/**
 * The choice to keep after the picker applies `slug` while the browser asks for `system`: a theme other than the
 * browser's is the reader's choice; the browser's own theme means "follow the browser" again, so nothing is kept.
 */
export function nextChoice(slug: string, system: string): string {
  return slug === system ? "" : slug;
}

/** Keep `choice`, or forget it when it is "". */
export function writeChoice(store: ChoiceStore, choice: string): void {
  writeValue(store, CHOICE_KEY, choice);
}

/** Read one stored value, "" when there is none or storage is refused. */
export function readValue(store: ChoiceStore, key: string): string {
  try {
    return store.getItem(key) ?? "";
  } catch {
    return "";
  }
}

/** Keep `value` under `key`, or forget it when it is "". A browser that refuses storage keeps nothing. */
export function writeValue(store: ChoiceStore, key: string, value: string): void {
  try {
    if (value) store.setItem(key, value);
    else store.removeItem(key);
  } catch {
    // The choice lasts until the page is left.
  }
}

/**
 * An `onChange` for a Lily picker that keeps only what the reader picks. The picker reports the value it starts with
 * (a stored choice, or its own guess) once as it mounts; that first report is not a choice and is not stored.
 */
export function keepPicked(store: ChoiceStore, key: string, also?: (value: string) => void): (value: string) => void {
  let started = false;
  return (value) => {
    also?.(value);
    if (!started) {
      started = true;
      return;
    }
    writeValue(store, key, value);
  };
}

/** Where the site keeps the reader's choices: the browser's localStorage (the default), or first-party cookies. */
export type ChoiceStorage = "localStorage" | "cookie";

/** A year: how long a cookie keeps a choice before the browser forgets it. */
export const COOKIE_MAX_AGE = 60 * 60 * 24 * 365;

/** The minimal part of `Document` the cookie store uses, so it can be tested without a browser. */
export interface CookieJar {
  cookie: string;
}

/**
 * Choices kept in cookies instead of localStorage: one cookie per choice, the path `/`, a year, `SameSite=Lax`, and
 * `Secure` on HTTPS. Choices only, never an identifier; GitHub Pages serves static files and reads no cookie.
 */
export function cookieStore(jar: CookieJar, secure: boolean): ChoiceStore {
  const tail = `; Path=/; SameSite=Lax${secure ? "; Secure" : ""}`;
  return {
    getItem(key) {
      for (const part of jar.cookie.split(";")) {
        const at = part.indexOf("=");
        if (at > 0 && part.slice(0, at).trim() === key) {
          try {
            return decodeURIComponent(part.slice(at + 1).trim());
          } catch {
            return null;
          }
        }
      }
      return null;
    },
    setItem(key, value) {
      jar.cookie = `${key}=${encodeURIComponent(value)}; Max-Age=${COOKIE_MAX_AGE}${tail}`;
    },
    removeItem(key) {
      jar.cookie = `${key}=; Max-Age=0${tail}`;
    },
  };
}

/** A store that keeps nothing past the page: used where the browser refuses localStorage altogether. */
export function memoryStore(): ChoiceStore {
  const held = new Map<string, string>();
  return {
    getItem: (key) => held.get(key) ?? null,
    setItem: (key, value) => void held.set(key, value),
    removeItem: (key) => void held.delete(key),
  };
}

/**
 * The store to keep choices in. With `cookie`, each choice already in localStorage (the theme under its new key or the
 * old one, and the others) moves into a cookie once and leaves localStorage, so a reader's picks survive the change.
 */
export function choiceStore(kind: ChoiceStorage, local: ChoiceStore, jar: CookieJar, secure: boolean): ChoiceStore {
  if (kind !== "cookie") return local;
  const cookies = cookieStore(jar, secure);
  try {
    readChoice(local); // the old theme key, moved or dropped first
    for (const key of STORED_KEYS) {
      const earlier = local.getItem(key);
      if (earlier && cookies.getItem(key) === null) cookies.setItem(key, earlier);
      local.removeItem(key);
    }
  } catch {
    // Storage refused: the cookies alone decide.
  }
  return cookies;
}
