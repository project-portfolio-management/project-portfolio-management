// The rules of an organization's names on `/organizations` (RE-7; NC-1 slice 12 part l), kept out of the component so
// they are unit-tested: its name (1 to 200 characters), short name (at most 50), short code (a letter or digit, then up
// to 19 letters, digits or hyphens; unique ignoring case), whether it is the one national organization, and its names
// in other languages, written one to a line as `code: name | short name` (the short name optional; at most 50
// languages). A change sends only what changed; every problem is said first.

import { isLanguageCode } from "./roadmap-category-view.js";

export const NAME_MAX = 200;
export const SHORT_NAME_MAX = 50;
export const LANGUAGES_MAX = 50;

/** An organization's names in one language. */
export interface OrgNames {
  name: string;
  short_name?: string | null;
}

/** The fields of an organization this form changes. */
export interface OrgNamed {
  pid: string;
  name: string;
  code?: string | null;
  national?: boolean;
  short_name?: string | null;
  translations?: Record<string, OrgNames>;
}

/** The form's fields. */
export interface OrgForm {
  name: string;
  short_name: string;
  code: string;
  national: boolean;
  translations: string;
}

/** What a change sends; an empty `code` or `short_name` clears it, `translations` replaces the set. */
export interface OrgChange {
  name?: string;
  short_name?: string;
  code?: string;
  national?: boolean;
  translations?: Record<string, OrgNames>;
}

/** Whether `code` is an organization code the service takes. */
export const isOrgCode = (code: string): boolean =>
  code.length <= 20 && /^[A-Za-z0-9][A-Za-z0-9-]*$/.test(code);

const len = (s: string) => [...s].length;

/** The names in other languages as their text area holds them, one language to a line, by code. */
export function namesText(t: Record<string, OrgNames> | undefined): string {
  return Object.keys(t ?? {})
    .sort()
    .map((k) => {
      const n = t![k]!;
      return n.short_name
        ? `${k}: ${n.name} | ${n.short_name}`
        : `${k}: ${n.name}`;
    })
    .join("\n");
}

/** The form for an organization. */
export const orgFormOf = (o: OrgNamed): OrgForm => ({
  name: o.name,
  short_name: o.short_name ?? "",
  code: o.code ?? "",
  national: o.national ?? false,
  translations: namesText(o.translations),
});

/** The names a text area says, or every problem with them as catalogue keys. */
export function parseNames(
  text: string,
): { translations: Record<string, OrgNames> } | { problems: string[] } {
  const problems = new Set<string>();
  const out: Record<string, OrgNames> = {};
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l !== "");
  if (lines.length > LANGUAGES_MAX) problems.add("p.organizations.tr_too_many");
  for (const line of lines) {
    const at = line.indexOf(":");
    const code = (at < 0 ? line : line.slice(0, at)).trim().toLowerCase();
    const rest = at < 0 ? "" : line.slice(at + 1);
    const bar = rest.indexOf("|");
    const name = (bar < 0 ? rest : rest.slice(0, bar)).trim();
    const short = bar < 0 ? "" : rest.slice(bar + 1).trim();
    if (!isLanguageCode(code)) problems.add("p.organizations.tr_code_bad");
    else if (name === "" || len(name) > NAME_MAX)
      problems.add("p.organizations.tr_name_bad");
    else if (len(short) > SHORT_NAME_MAX)
      problems.add("p.organizations.tr_short_bad");
    else if (code in out) problems.add("p.organizations.tr_code_twice");
    else out[code] = short ? { name, short_name: short } : { name };
  }
  return problems.size > 0
    ? { problems: [...problems] }
    : { translations: out };
}

const sameNames = (a: Record<string, OrgNames>, b: Record<string, OrgNames>) =>
  namesText(a) === namesText(b);

/**
 * What saving the form sends for `o`: only what changed, every problem first as catalogue keys (a code or a national
 * scope another organization in `list` holds among them); `null` when nothing changed.
 */
export function orgChange(
  form: OrgForm,
  o: OrgNamed,
  list: readonly OrgNamed[],
): { body: OrgChange } | { problems: string[] } | null {
  const problems: string[] = [];
  const name = form.name.trim();
  const short = form.short_name.trim();
  const code = form.code.trim();
  if (name === "" || len(name) > NAME_MAX)
    problems.push("p.organizations.name_bad");
  else if (
    list.some(
      (x) =>
        x.pid !== o.pid && x.name.trim().toLowerCase() === name.toLowerCase(),
    )
  )
    problems.push("p.organizations.name_taken");
  if (len(short) > SHORT_NAME_MAX) problems.push("p.organizations.short_bad");
  if (code !== "" && !isOrgCode(code))
    problems.push("p.organizations.code_bad");
  else if (
    code !== "" &&
    list.some(
      (x) =>
        x.pid !== o.pid && (x.code ?? "").toLowerCase() === code.toLowerCase(),
    )
  )
    problems.push("p.organizations.code_taken");
  if (
    form.national &&
    !o.national &&
    list.some((x) => x.pid !== o.pid && x.national)
  )
    problems.push("p.organizations.national_taken");
  const t = parseNames(form.translations);
  if ("problems" in t) problems.push(...t.problems);
  if (problems.length > 0 || "problems" in t) return { problems };
  const body: OrgChange = {};
  if (name !== o.name) body.name = name;
  if (short !== (o.short_name ?? "")) body.short_name = short;
  if (code !== (o.code ?? "")) body.code = code;
  if (form.national !== (o.national ?? false)) body.national = form.national;
  if (!sameNames(t.translations, o.translations ?? {}))
    body.translations = t.translations;
  return Object.keys(body).length === 0 ? null : { body };
}

/** The name to show in `locale` (`cy-001`, `zh-cn`): its own language's, else the stored name. */
export function shownOrgName(o: OrgNamed, locale: string): string {
  const lc = locale.toLowerCase();
  const t = o.translations ?? {};
  return (t[lc] ?? t[lc.split("-")[0]!])?.name ?? o.name;
}
