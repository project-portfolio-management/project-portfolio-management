// What a page says when an action or a load fails (CQ-20): one rule instead of the same
// expression copied into every page.
//
// No page shows the service's own error text. It is English, unreviewed and sometimes carries
// internals, so the reader gets the catalogue's sentence for the kind of refusal instead
// (`p.error.status_409`, ...), in their language. A server fault (5xx) and a failure that is not
// an API refusal (a dropped connection) say nothing useful and fall back to the page's own message.
// `tests/unit/no-api-text.test.ts` fails if a route reads `err.message`.

import { ApiError } from "./api/client";
import { t, tDynamic, type StringKey } from "./i18n.svelte.js";

/** The catalogue's sentence for the kind of failure `err` is, `""` when it is not an API refusal. */
export function errorReason(err: unknown): string {
  if (!(err instanceof ApiError)) return "";
  if (err.status >= 500) return t("p.error.server");
  const generic = tDynamic(`p.error.status_${err.status}`);
  return generic.startsWith("p.error.") ? t("p.error.client") : generic;
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
