// What a page says when an action or a load fails (CQ-20): one rule instead of the same
// expression copied into every page.
//
// A refusal the service explains (4xx) keeps the service's own reason, which says what to
// change. A server fault (5xx) or a network failure has nothing useful to say to the reader,
// and loco's text for it is English, so the page's own catalogue message is shown instead.

import { ApiError } from "./api/client";
import { t, type StringKey } from "./i18n.svelte.js";

/** The text to show for `err`, falling back to the catalogue message `key`. */
export function errorText(
  err: unknown,
  key: StringKey,
  values?: Parameters<typeof t>[1],
): string {
  if (err instanceof ApiError && err.status >= 500) return t(key, values);
  return err instanceof Error && err.message ? err.message : t(key, values);
}
