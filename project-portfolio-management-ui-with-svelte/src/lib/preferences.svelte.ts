// The display preferences store (LT-12): simple mode and the dyslexia-friendly
// font. The rules are in `./preferences.ts`; this holds the reactive state and
// the persistence. Choices are kept on the device always (so they work signed
// out) and, when signed in, as the user's own saved view (so they follow the
// person to another browser). Saving to the account is best effort: a failure
// is said, and the choice still holds on this device.

import { browser } from "$app/env";
import { PpmClient } from "#lib/api/ppm.js";
import {
  PREFERENCES_ROUTE,
  STORAGE_KEY,
  fromSavedViews,
  parsePrefs,
  serialQueue,
  toSavedView,
  withHidden,
  type Prefs,
} from "./preferences.js";

function readLocal(): Prefs {
  if (!browser || typeof localStorage === "undefined") return {};
  try {
    return parsePrefs(localStorage.getItem(STORAGE_KEY));
  } catch {
    return {};
  }
}

function writeLocal(prefs: Prefs): void {
  if (!browser || typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
  } catch {
    // Storage may be blocked; the choice then lasts for this page only.
  }
}

/** Where the last choice was kept, in words for the person. */
export type SaveStatus = "idle" | "device" | "account" | "account-failed";

let prefs = $state<Prefs>(readLocal());
let status = $state<SaveStatus>("idle");
let syncedFor: boolean | null = null;

// One account write at a time, each sending the choices as they are when its turn comes (BR-17).
const queue = serialQueue();

function pushToAccount(_asked: Prefs): Promise<boolean> {
  return queue(() => pushNow(prefs));
}

async function pushNow(next: Prefs): Promise<boolean> {
  const ppm = PpmClient.withFetch();
  try {
    const old = await ppm.listSavedViews(PREFERENCES_ROUTE);
    for (const v of old) await ppm.deleteSavedView(v.pid);
    await ppm.createSavedView(toSavedView(next));
    return true;
  } catch {
    return false;
  }
}

export const display = {
  /** The current choices (a missing field means "not chosen"). */
  get prefs(): Prefs {
    return prefs;
  },
  /** Whether the dyslexia-friendly font is on. */
  get dyslexia(): boolean {
    return prefs.dyslexia === true;
  },
  /** Where the last change was kept. */
  get status(): SaveStatus {
    return status;
  },

  /**
   * Change one or more choices; `undefined` clears a choice (back to the
   * default). Applies at once, then keeps it on the device and, if signed in,
   * on the account.
   */
  async set(
    patch: Partial<Record<keyof Prefs, boolean | undefined>>,
    signedIn: boolean,
  ): Promise<void> {
    const next: Prefs = parsePrefs({ ...prefs, ...patch });
    prefs = next;
    writeLocal(next);
    status = "device";
    if (signedIn)
      status = (await pushToAccount(next)) ? "account" : "account-failed";
  },

  /** Hide a section of the top bar, or show it again (BR-17). Nothing is stored for a section never hidden. */
  async setSectionHidden(
    href: string,
    hide: boolean,
    signedIn: boolean,
  ): Promise<void> {
    const next = withHidden(prefs, href, hide);
    prefs = next;
    writeLocal(next);
    status = "device";
    if (signedIn)
      status = (await pushToAccount(next)) ? "account" : "account-failed";
  },

  /** Show every section again. */
  async showAllSections(signedIn: boolean): Promise<void> {
    const next: Prefs = { ...prefs };
    delete next.hidden;
    prefs = next;
    writeLocal(next);
    status = "device";
    if (signedIn)
      status = (await pushToAccount(next)) ? "account" : "account-failed";
  },

  /**
   * When a person signs in, adopt what their account holds; if it holds
   * nothing but this device does, keep the device's choices on the account.
   * Runs once per sign-in state.
   */
  async sync(signedIn: boolean): Promise<void> {
    if (syncedFor === signedIn) return;
    syncedFor = signedIn;
    if (!signedIn) return;
    try {
      const found = fromSavedViews(
        await PpmClient.withFetch().listSavedViews(PREFERENCES_ROUTE),
      );
      if (found) {
        prefs = found.prefs;
        writeLocal(found.prefs);
      } else if (Object.keys(prefs).length > 0) {
        await pushToAccount(prefs);
      }
    } catch {
      // Not reachable: the device's choices stand.
    }
  },
};
