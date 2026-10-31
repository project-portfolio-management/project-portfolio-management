// The first day of the week (WW-1.4): the page's locale's by default, or the person's own choice on the Display page.
//
// A calendar's week starts where its reader's does: Monday in most of the world (and in the `-001` world locales, by
// CLDR), Sunday in the Americas, Japan, India and elsewhere, Saturday in parts of the Middle East and North Africa.
// The interface's calendars ask Lily's CalendarView, which reads the first day from a locale; a choice is passed as
// the Unicode `fw` extension (`en-001-u-fw-sun`). Browsers without `Intl.Locale` week information (Firefox, at the
// time of writing) fall back to a region table and do not see the extension: there the choice has no effect, a
// stated limit. Week numbers and the API's week buckets stay ISO 8601 (Monday first), a stated limit too.

/** A person's choice: the locale's day, or a fixed one. */
export type WeekStart = "locale" | "mon" | "sun" | "sat";

/** The choices, in the order the Display page lists them. */
export const WEEK_STARTS: readonly WeekStart[] = [
  "locale",
  "mon",
  "sun",
  "sat",
];

/** A stored value read back: one of {@link WEEK_STARTS}, else `undefined` (the default). */
export function parseWeekStart(raw: unknown): WeekStart | undefined {
  return typeof raw === "string" &&
    (WEEK_STARTS as readonly string[]).includes(raw) &&
    raw !== "locale"
    ? (raw as WeekStart)
    : undefined;
}

const DAY: Record<Exclude<WeekStart, "locale">, number> = {
  sun: 0,
  mon: 1,
  sat: 6,
};

/** Regions whose week starts on Sunday or Saturday by CLDR, for engines with no week information (a subset, checked against the CLDR data in Node 26 on 2026-10-31). */
const SUNDAY = new Set([
  "us",
  "ca",
  "mx",
  "br",
  "jp",
  "kr",
  "tw",
  "hk",
  "in",
  "il",
  "ph",
  "za",
  "sa",
]);
const SATURDAY = new Set([
  "af",
  "dz",
  "eg",
  "ir",
  "iq",
  "jo",
  "kw",
  "ly",
  "om",
  "qa",
  "sd",
  "sy",
]);

/** The first day of the week (0 = Sunday … 6 = Saturday) for `locale` under `choice`. */
export function firstDayOfWeek(locale: string, choice?: WeekStart): number {
  if (choice && choice !== "locale") return DAY[choice];
  try {
    const loc = new Intl.Locale(locale) as Intl.Locale & {
      getWeekInfo?: () => { firstDay: number };
      weekInfo?: { firstDay: number };
    };
    const info = loc.getWeekInfo?.() ?? loc.weekInfo;
    if (info && typeof info.firstDay === "number") return info.firstDay % 7;
  } catch {
    // A malformed tag: use the table.
  }
  const region = locale.toLowerCase().split("-")[1] ?? "";
  if (SUNDAY.has(region)) return 0;
  if (SATURDAY.has(region)) return 6;
  return 1;
}

/** The locale tag to hand a Lily calendar so it starts the week on the chosen day. */
export function calendarLocale(locale: string, choice?: WeekStart): string {
  if (!choice || choice === "locale") return locale;
  const base = locale.split("-u-")[0] ?? locale;
  return `${base}-u-fw-${choice}`;
}
