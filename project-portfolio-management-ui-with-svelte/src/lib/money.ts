// The one money formatter and parser (spec 02-scope §2.0, WW-1.3). Amounts are integer minor units;
// the exponent (0 for JPY, 2 for EUR, 3 for KWD, 4 for CLF) and the number format come from `Intl`,
// never from a hand-written `/100`. There is no default currency: a caller always passes the one the
// record carries.

import { i18n } from "./i18n.svelte.js";

/** The reader's locale: the one in the URL. */
export function currentLocale(): string {
  return i18n.locale;
}

/** Minor-unit exponent of `currency` (ISO 4217); 2 when the runtime does not know the code. */
export function exponentOf(currency: string, locale: string): number {
  try {
    return (
      new Intl.NumberFormat(locale, {
        style: "currency",
        currency,
      }).resolvedOptions().maximumFractionDigits ?? 2
    );
  } catch {
    return 2;
  }
}

/** `minor` units of `currency` in the reader's own number format. Always names the currency. */
export function money(
  minor: number | null,
  currency: string,
  locale: string = currentLocale(),
): string {
  if (minor === null) return "–";
  const e = exponentOf(currency, locale);
  try {
    return new Intl.NumberFormat(locale, {
      style: "currency",
      currency,
    }).format(minor / 10 ** e);
  } catch {
    return `${currency} ${minor}`;
  }
}

/** Money with the sign always shown ("+1,200.00", "-5"), for a change. */
export function signedMoney(
  minor: number,
  currency: string,
  locale: string = currentLocale(),
): string {
  const text = money(Math.abs(minor), currency, locale);
  return minor > 0 ? `+${text}` : minor < 0 ? `-${text}` : text;
}

/** Plain amount for a text input ("1234,50" in a decimal-comma locale), no symbol, no grouping. */
export function minorToInput(
  minor: number,
  currency: string,
  locale: string = currentLocale(),
): string {
  const e = exponentOf(currency, locale);
  try {
    return new Intl.NumberFormat(locale, {
      minimumFractionDigits: e,
      maximumFractionDigits: e,
      useGrouping: false,
    }).format(minor / 10 ** e);
  } catch {
    return String(minor / 10 ** e);
  }
}

function symbols(locale: string): {
  decimal: string;
  group: string;
  digits: string[];
} {
  let decimal = ".";
  let group = ",";
  const digits: string[] = [];
  try {
    const nf = new Intl.NumberFormat(locale);
    for (const p of nf.formatToParts(1234567.5)) {
      if (p.type === "decimal") decimal = p.value;
      if (p.type === "group") group = p.value;
    }
    for (let d = 0; d < 10; d++) digits.push(nf.format(d));
  } catch {
    for (let d = 0; d < 10; d++) digits.push(String(d));
  }
  return { decimal, group, digits };
}

/**
 * An amount typed in `locale`'s own format to a number of major units, or `null` when it is not a plain
 * number. The locale's decimal mark is the only decimal mark ("1,5" and "1 234,50" in a decimal-comma
 * locale, "1,234.50" in an English one); its group mark and any space are ignored.
 */
export function parseAmount(
  text: string,
  locale: string = currentLocale(),
): number | null {
  const { decimal, group, digits } = symbols(locale);
  let s = text.trim();
  if (s === "") return null;
  s = s.replace(/[\s\u00a0\u202f']/g, "");
  if (group !== decimal && group.trim() !== "") s = s.split(group).join("");
  s = s.split(decimal).join(".");
  s = [...s]
    .map((c) => {
      const i = digits.indexOf(c);
      return i >= 0 ? String(i) : c;
    })
    .join("");
  if (!/^-?(\d+\.?\d*|\.\d+)$/.test(s)) return null;
  const n = Number(s);
  return Number.isFinite(n) ? n : null;
}

/** An amount typed in `locale` to minor units of `currency`, or `null` when it is not a number. */
export function toMinor(
  text: string,
  currency: string,
  locale: string = currentLocale(),
): number | null {
  const n = parseAmount(text, locale);
  if (n === null) return null;
  const e = exponentOf(currency, locale);
  // "1.005e2" is exactly 100.5, where 1.005 * 100 is 100.49999…
  const scaled = Number(`${n}e${e}`);
  return Number.isFinite(scaled) ? Math.round(scaled) : null;
}

/** A currency code as typed: three letters, upper-cased; `null` when it is not three letters. */
export function currencyCode(text: string): string | null {
  const c = text.trim();
  return /^[A-Za-z]{3}$/.test(c) ? c.toUpperCase() : null;
}
