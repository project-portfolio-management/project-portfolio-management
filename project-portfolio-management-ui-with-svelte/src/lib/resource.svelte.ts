// A loaded value and what is known about loading it (CQ-20): the `data`, `error`, `loading` triple
// that pages wrote by hand around `onMount`, with the failure text from `errorText`.
//
// `reload()` runs the loader again. When calls overlap, only the latest one is allowed to set the
// state, so a slow earlier answer cannot overwrite a newer one (the race CQ-17 fixed page by page).
// `data` keeps its last good value while a reload runs or fails; `error` is cleared when a load starts.

import { errorText } from "./error-text.js";
import type { StringKey } from "./i18n.svelte.js";

export interface Resource<T> {
  /** The last value loaded, `null` before the first success. */
  readonly data: T | null;
  /** The text to show for the last failed load, `null` when the last load worked or none has run. */
  readonly error: string | null;
  /** Whether a load is running. */
  readonly loading: boolean;
  /** Run the loader again; resolves when that call is finished (it never rejects). */
  reload(): Promise<void>;
}

/** Wrap `load`; `failKey` is the catalogue message used when the failure has nothing better to say. */
export function resource<T>(
  load: () => Promise<T>,
  failKey: StringKey,
): Resource<T> {
  let data = $state<T | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);
  let latest = 0;

  async function reload(): Promise<void> {
    const call = ++latest;
    loading = true;
    error = null;
    try {
      const value = await load();
      if (call === latest) data = value;
    } catch (err) {
      if (call === latest) error = errorText(err, failKey);
    } finally {
      if (call === latest) loading = false;
    }
  }

  return {
    get data() {
      return data;
    },
    get error() {
      return error;
    },
    get loading() {
      return loading;
    },
    reload,
  };
}
