// The rules of the roadmap categories page (`/roadmap-categories`, RE-7; NC-1 slice 12 part l), kept out of the
// component so they are unit-tested: a category's name (1 to 100 characters) and its translations, written one to a
// line as `code: text` (a language code such as `cy` or `zh-cn`; each text 1 to 100 characters; at most 50), what a
// change sends (only what changed), and which name to show a reader.

export const NAME_MAX = 100;
export const TRANSLATIONS_MAX = 50;

/** A category as the service holds it. */
export interface RoadmapCategory {
  pid: string;
  name: string;
  translations: Record<string, string>;
  active: boolean;
}

/** Whether `s` is a language code the service takes: 2 or 3 lower-case letters, then `-` parts of 2 to 8. */
export function isLanguageCode(s: string): boolean {
  return s.length <= 35 && /^[a-z]{2,3}(-[a-z0-9]{2,8})*$/.test(s);
}

/** The translations as their text area holds them, one `code: text` to a line, by code. */
export const translationsText = (t: Record<string, string>): string =>
  Object.keys(t)
    .sort()
    .map((k) => `${k}: ${t[k]}`)
    .join("\n");

/** The translations a text area says, or every problem with them as catalogue keys. */
export function parseTranslations(
  text: string,
): { translations: Record<string, string> } | { problems: string[] } {
  const problems = new Set<string>();
  const out: Record<string, string> = {};
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l !== "");
  if (lines.length > TRANSLATIONS_MAX)
    problems.add("p.roadmap_categories.too_many");
  for (const line of lines) {
    const at = line.indexOf(":");
    const code = (at < 0 ? line : line.slice(0, at)).trim().toLowerCase();
    const value = at < 0 ? "" : line.slice(at + 1).trim();
    if (!isLanguageCode(code)) problems.add("p.roadmap_categories.code_bad");
    else if (value === "" || [...value].length > NAME_MAX)
      problems.add("p.roadmap_categories.text_bad");
    else if (code in out) problems.add("p.roadmap_categories.code_twice");
    else out[code] = value;
  }
  return problems.size > 0
    ? { problems: [...problems] }
    : { translations: out };
}

const sameMap = (a: Record<string, string>, b: Record<string, string>) =>
  JSON.stringify(Object.entries(a).sort()) ===
  JSON.stringify(Object.entries(b).sort());

/**
 * What adding (`c` null) or changing a category sends: the name and the whole set of translations, a change only what
 * changed; every problem first; `null` when a change changes nothing.
 */
export function categoryChange(
  name: string,
  translations: string,
  c: RoadmapCategory | null,
):
  | { body: { name?: string; translations?: Record<string, string> } }
  | { problems: string[] }
  | null {
  const problems: string[] = [];
  const n = name.trim();
  if (n === "" || [...n].length > NAME_MAX)
    problems.push("p.roadmap_categories.name_bad");
  const t = parseTranslations(translations);
  if ("problems" in t) problems.push(...t.problems);
  if (problems.length > 0 || "problems" in t) return { problems };
  if (!c) return { body: { name: n, translations: t.translations } };
  const body: { name?: string; translations?: Record<string, string> } = {};
  if (n !== c.name) body.name = n;
  if (!sameMap(t.translations, c.translations ?? {}))
    body.translations = t.translations;
  return Object.keys(body).length === 0 ? null : { body };
}

/** Whether another category already has this name, ignoring case (the service's 409, said first). */
export const nameTaken = (
  name: string,
  list: readonly RoadmapCategory[],
  except?: string,
) =>
  list.some(
    (c) =>
      c.pid !== except && c.name.toLowerCase() === name.trim().toLowerCase(),
  );

/** The name to show in `locale` (`cy-001`, `zh-cn`): its own translation, its language's, else the name itself. */
export function shownName(c: RoadmapCategory, locale: string): string {
  const lc = locale.toLowerCase();
  const t = c.translations ?? {};
  return t[lc] ?? t[lc.split("-")[0]!] ?? c.name;
}
