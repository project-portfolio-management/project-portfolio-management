// The site's search. GitHub Pages has no server, so the index is a static file (`/search-index.json`, prerendered from
// the same documents as the pages) and this runs in the reader's browser. Safe for the browser: no document text here.

/** One searchable page: `lang` is its BCP 47 tag, `text` its words with the Markdown gone. */
export type SearchDoc = { route: string; title: string; lang: string; text: string };
export type SearchHit = { route: string; title: string; lang: string; excerpt: string };

const EXCERPT_BEFORE = 60;
const EXCERPT_LENGTH = 220;

/** The query in a search URL. Lily's SearchPicker writes `/search/?foo%20bar`: the whole query string, with no key. */
export function queryOf(search: string): string {
  const raw = search.replace(/^\?/, "");
  try {
    return decodeURIComponent(raw).trim();
  } catch {
    return raw.trim();
  }
}

/** The address of a search, in the form `queryOf` reads. */
export function searchUrl(action: string, query: string): string {
  return `${action}?${encodeURIComponent(query.trim())}`;
}

function excerptOf(text: string, at: number): string {
  const start = Math.max(0, at - EXCERPT_BEFORE);
  const piece = text.slice(start, start + EXCERPT_LENGTH).trim();
  return `${start > 0 ? "…" : ""}${piece}${start + EXCERPT_LENGTH < text.length ? "…" : ""}`;
}

/**
 * Pages holding every word of the query, ignoring case: pages with a word in the title first, then by how often the
 * words appear. A substring match, so it also works for Chinese and Hindi, which have no spaces or stems to index.
 */
export function search(docs: readonly SearchDoc[], query: string): SearchHit[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
  if (terms.length === 0) return [];
  const scored: { hit: SearchHit; title: boolean; count: number }[] = [];
  for (const doc of docs) {
    const title = doc.title.toLowerCase();
    const text = doc.text.toLowerCase();
    if (!terms.every((t) => title.includes(t) || text.includes(t))) continue;
    const at = terms.map((t) => text.indexOf(t)).filter((i) => i >= 0);
    scored.push({
      hit: {
        route: doc.route,
        title: doc.title,
        lang: doc.lang,
        excerpt: excerptOf(doc.text, at.length > 0 ? Math.min(...at) : 0),
      },
      title: terms.some((t) => title.includes(t)),
      count: terms.reduce((n, t) => n + text.split(t).length - 1, 0),
    });
  }
  scored.sort((a, b) => Number(b.title) - Number(a.title) || b.count - a.count);
  return scored.map((s) => s.hit);
}
