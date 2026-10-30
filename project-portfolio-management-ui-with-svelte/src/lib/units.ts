// "3 business days", "1 calendar day": a count with its unit, in the form the page's language uses for that count
// (`tn`, CLDR plural rules; NFR-11: a duration in days always says which kind). One place, so every page and
// helper words it the same way and Welsh needs it translated once.

import { tn } from "./i18n.svelte.js";

/** `1 business day`, `3 business days`, `-1 business day` (plural by magnitude). */
export function businessDays(n: number): string {
  return tn("p.common.business_day", n, { n });
}

/** `1 calendar day`, `3 calendar days`. */
export function calendarDays(n: number): string {
  return tn("p.common.calendar_day", n, { n });
}
