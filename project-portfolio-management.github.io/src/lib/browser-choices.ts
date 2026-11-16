// The one store the site keeps the reader's choices in (#lib/choices.ts): localStorage, or first-party cookies when the
// site is built with VITE_CHOICE_STORAGE=cookie. While prerendering, and where the browser refuses localStorage, a
// store that keeps nothing past the page.
import { browser } from "$app/env";
import { choiceStore, memoryStore, type ChoiceStorage, type ChoiceStore } from "./choices.js";

/** Where this build keeps choices. */
export const CHOICE_STORAGE: ChoiceStorage = import.meta.env.VITE_CHOICE_STORAGE === "cookie" ? "cookie" : "localStorage";

let store: ChoiceStore | null = null;

/** The store, made once (with `cookie`, moving earlier localStorage choices into cookies). */
export function browserChoices(): ChoiceStore {
  if (store) return store;
  if (!browser) return memoryStore();
  let local: ChoiceStore | null = null;
  try {
    local = localStorage; // reading it throws where the browser blocks site data
  } catch {
    local = null;
  }
  store = choiceStore(CHOICE_STORAGE, local ?? memoryStore(), document, location.protocol === "https:");
  return store;
}
