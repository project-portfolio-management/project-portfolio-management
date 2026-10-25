// What a page says when an action or a load fails (CQ-20): one rule instead of the same
// expression copied into every page.
//
// A server fault (5xx) or a network failure has nothing useful to say to the reader, and loco's
// text for it is English, so the page's own catalogue message is shown instead. A refusal the
// service explains (4xx) says what to change, in English. In English that sentence is shown as it
// is. In another language it is shown behind the catalogue's sentence for that kind of refusal
// (`p.error.status_409`, ...) so the reader gets their own language first and the service's
// specific reason after it.

import { ApiError } from "./api/client";
import { i18n, t, tDynamic, type StringKey } from "./i18n.svelte.js";

/** The reason to give for `err`: the service's own for a refusal in English, the catalogue's sentence for the
 *  kind of refusal (with the service's reason after it) in another language, `""` when there is nothing to say. */
export function errorReason(err: unknown): string {
  if (!(err instanceof Error)) return "";
  if (!(err instanceof ApiError)) return err.message;
  if (err.status >= 500) return t("p.error.server");
  const detail = err.message;
  if (i18n.locale.startsWith("en")) return detail;
  const generic = tDynamic(`p.error.status_${err.status}`);
  const kind = generic.startsWith("p.error.") ? t("p.error.client") : generic;
  return detail ? `${kind} (${detail})` : kind;
}

/** The text to show for `err`, falling back to the catalogue message `key`. */
export function errorText(
  err: unknown,
  key: StringKey,
  values?: Parameters<typeof t>[1],
): string {
  if (err instanceof ApiError && err.status >= 500) return t(key, values);
  return errorReason(err) || t(key, values);
}
