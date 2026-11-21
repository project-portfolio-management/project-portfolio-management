// The UI follows the browser's light/dark setting (spec, Theming) through Lily's ThemePicker "Follow this device"
// option, whose value is "system" and which the picker keeps in `localStorage` key `lily-theme` (and a cookie). Two
// earlier schemes left values behind; `migrateThemeChoice` brings them across once, before the picker mounts.

/** Where the UI itself kept the person's own choice, 2026-11-15 to 2026-11-21. */
export const OLD_CHOICE_KEY = "ppm-theme-choice";

/** The picker's key: the choice it keeps ("system", or a theme slug). */
export const PICKER_KEY = "lily-theme";

/** The minimal part of `Storage` used here, so the rules can be tested without a browser. */
export interface ChoiceStore {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

/**
 * Move old values to the picker's key. A theme under `ppm-theme-choice` was picked by the person and becomes the
 * picker's choice; the old key goes. Before that, the picker kept every theme it applied, so `light` or `dark` under
 * `lily-theme` with no other choice was most likely its guess from the browser: it is dropped, and the picker starts on
 * "Follow this device". Any other theme there was picked and stays.
 */
export function migrateThemeChoice(store: ChoiceStore): void {
  try {
    const picked = store.getItem(OLD_CHOICE_KEY);
    if (picked !== null) {
      store.removeItem(OLD_CHOICE_KEY);
      if (picked) store.setItem(PICKER_KEY, picked);
      return;
    }
    const kept = store.getItem(PICKER_KEY);
    if (kept === "light" || kept === "dark") store.removeItem(PICKER_KEY);
  } catch {
    // A browser that refuses storage still follows its own setting.
  }
}
