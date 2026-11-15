// The UI uses the browser's light/dark setting (spec, Theming). A theme the person picks in the theme picker wins over
// it and is remembered; nothing else is. Lily's ThemePicker, given a storage key, stores every theme it applies,
// including the one it guessed from the browser, so after a first visit the browser's setting no longer counted. The
// layout therefore gives the picker no storage key and keeps the person's choice here.

/** Where the person's own choice is kept. */
export const CHOICE_KEY = "ppm-theme-choice";

/** Where the picker kept every theme it applied, guessed or chosen, until 2026-11-15. */
export const LEGACY_KEY = "lily-theme";

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
 * The person's chosen theme, or "" to follow the browser. A value under the legacy key moves across once: `light` or
 * `dark` there was most likely the picker's own guess from the browser, so it is dropped; any other theme was picked.
 */
export function readChoice(store: ChoiceStore): string {
  try {
    const legacy = store.getItem(LEGACY_KEY);
    if (legacy !== null) {
      store.removeItem(LEGACY_KEY);
      if (
        legacy &&
        legacy !== "light" &&
        legacy !== "dark" &&
        store.getItem(CHOICE_KEY) === null
      ) {
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
 * browser's is the person's choice; the browser's own theme means "follow the browser" again, so nothing is kept.
 */
export function nextChoice(slug: string, system: string): string {
  return slug === system ? "" : slug;
}

/** Keep `choice`, or forget it when it is "". */
export function writeChoice(store: ChoiceStore, choice: string): void {
  try {
    if (choice) store.setItem(CHOICE_KEY, choice);
    else store.removeItem(CHOICE_KEY);
  } catch {
    // A browser that refuses storage still follows its own setting.
  }
}
