// "3 business days", "1 calendar day": a count with its unit, pluralized by the size of the
// number (NFR-11: a duration in days always says which kind). One place, so every page and
// helper words it the same way and Welsh needs it translated once.

import { t } from "./i18n.svelte.js";

/** `1 business day`, `3 business days`, `-1 business day` (plural by magnitude). */
export function businessDays(n: number): string {
  return Math.abs(n) === 1
    ? t("p.common.business_day_one", { n })
    : t("p.common.business_day_other", { n });
}

/** `1 calendar day`, `3 calendar days`. */
export function calendarDays(n: number): string {
  return Math.abs(n) === 1
    ? t("p.common.calendar_day_one", { n })
    : t("p.common.calendar_day_other", { n });
}
