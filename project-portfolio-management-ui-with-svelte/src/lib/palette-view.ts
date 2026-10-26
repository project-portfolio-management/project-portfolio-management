// Command palette (BR-14): the pure rules. No runes, no `$app/*`; unit-tested in `tests/unit/palette-view.test.ts`.
//
// The palette offers three kinds of row: a command (a fixed destination), a plan (found by name through the search
// API) and a recent item (a plan or command opened before). Ranking is by how the query meets the label: the whole
// label, then a word start, then anywhere in it, then letters in order. Ties keep the order given, so recents and
// commands come out in the order the caller chose.

/** What a row is. */
export type PaletteKind = "command" | "plan" | "recent";

/** One row in the palette. */
export interface PaletteItem {
  /** Stable id: `cmd:<name>` or `plan:<pid>`. */
  id: string;
  kind: PaletteKind;
  /** The text shown and searched. */
  label: string;
  /** An in-app path, without the locale prefix. */
  href: string;
}

/** The most recent items kept. */
export const MAX_RECENT = 5;
/** The most rows shown at once. */
export const MAX_ROWS = 12;
/** The device-storage key for recent items. */
export const RECENT_KEY = "mxi.palette.recent";

/**
 * How well `query` meets `label`: 0 is no match, higher is better. Case and accents are ignored.
 * 4 the whole label, 3 the label starts with it, 2 a word starts with it, 1 it occurs inside a word,
 * and the letters in order (not touching) score 0.5.
 */
export function score(query: string, label: string): number {
  const q = fold(query);
  const l = fold(label);
  if (q === "") return 1;
  if (l === q) return 4;
  if (l.startsWith(q)) return 3;
  if (l.split(/[^\p{L}\p{N}]+/u).some((w) => w.startsWith(q))) return 2;
  if (l.includes(q)) return 1;
  let i = 0;
  for (const c of l) if (c === q[i] && ++i === q.length) return 0.5;
  return 0;
}

function fold(s: string): string {
  return s.normalize("NFD").replace(/\p{M}/gu, "").toLowerCase().trim();
}

/**
 * Rank `items` for `query`: matches only, best score first, equal scores in the order given, and at most
 * {@link MAX_ROWS}. An empty query keeps every item in the order given.
 */
export function rank(query: string, items: PaletteItem[]): PaletteItem[] {
  return items
    .map((item, order) => ({ item, order, s: score(query, item.label) }))
    .filter((x) => x.s > 0)
    .sort((a, b) => b.s - a.s || a.order - b.order)
    .map((x) => x.item)
    .slice(0, MAX_ROWS);
}

/**
 * The rows to show: commands and plans together for a query, ranked; with no query, recents first and then the
 * commands. A plan or command in both lists appears once, as the first.
 */
export function rows(
  query: string,
  commands: PaletteItem[],
  plans: PaletteItem[],
  recent: PaletteItem[],
): PaletteItem[] {
  const pool = query.trim() === "" ? [...recent, ...commands] : [...commands, ...plans];
  const seen = new Set<string>();
  const unique = pool.filter((i) => (seen.has(i.id) ? false : (seen.add(i.id), true)));
  return rank(query, unique);
}

/** `item` put first in `list`, once, keeping at most {@link MAX_RECENT}. */
export function pushRecent(list: PaletteItem[], item: PaletteItem): PaletteItem[] {
  const as: PaletteItem = { ...item, kind: "recent" };
  return [as, ...list.filter((i) => i.id !== item.id)].slice(0, MAX_RECENT);
}

/** Recent items read back from storage text; anything malformed is dropped, never thrown. */
export function parseRecent(text: string | null): PaletteItem[] {
  if (!text) return [];
  try {
    const raw: unknown = JSON.parse(text);
    if (!Array.isArray(raw)) return [];
    return raw
      .filter(
        (r): r is PaletteItem =>
          typeof r === "object" &&
          r !== null &&
          typeof (r as PaletteItem).id === "string" &&
          typeof (r as PaletteItem).label === "string" &&
          typeof (r as PaletteItem).href === "string" &&
          // One leading slash and no second slash or backslash: `//host` and `/\host` both leave the site.
          /^\/(?![/\\])/.test((r as PaletteItem).href),
      )
      .map((r) => ({ id: r.id, kind: "recent" as const, label: r.label, href: r.href }))
      .slice(0, MAX_RECENT);
  } catch {
    return [];
  }
}

/** A plan found by name, as a row. */
export function planItem(p: { pid: string; name: string }): PaletteItem {
  return { id: `plan:${p.pid}`, kind: "plan", label: p.name, href: `/plans/${p.pid}` };
}

/** Whether a key event opens the palette: Ctrl or Cmd with K. */
export function opensPalette(e: { key: string; ctrlKey: boolean; metaKey: boolean; altKey: boolean }): boolean {
  return (e.ctrlKey || e.metaKey) && !e.altKey && e.key.toLowerCase() === "k";
}

/** The index after an arrow key, wrapping, among `count` rows; `-1` when there are none. */
export function move(current: number, key: "ArrowDown" | "ArrowUp", count: number): number {
  if (count <= 0) return -1;
  if (key === "ArrowDown") return current < 0 ? 0 : (current + 1) % count;
  return current <= 0 ? count - 1 : current - 1;
}
