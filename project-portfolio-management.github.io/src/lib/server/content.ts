// Server-only (prerender time): the repository documents and their Markdown, rendered to HTML.
import { Marked, type Tokens } from "marked";
import docs from "../../content/docs.json" with { type: "json" };
import { BRIEF_LOCALES, DOC_PAGES, REPO_URL, SITE_URL, langTag, routeOfRepoPath } from "#lib/site.js";
import type { SearchDoc } from "#lib/search.js";

const FILES: Record<string, string> = docs.files;

export function source(path: string): string {
  const text = FILES[path];
  if (text === undefined) throw new Error(`not in src/content/docs.json: ${path} (run pnpm run sync)`);
  return text;
}

function escapeHtml(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}

/** GitHub's heading anchor, as `scripts/check-links.py` in the monorepo computes it. */
export function slug(text: string): string {
  let h = text.trim().toLowerCase();
  h = h.replace(/`|\*|_(?=\W|$)/g, "");
  h = h.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1");
  h = h.replace(/[^\p{L}\p{N}_\- ]/gu, "");
  return h.replace(/ /g, "-");
}

/** Join a relative link onto the directory of `from` (a repository path); null if it climbs out of the repository. */
export function joinRepoPath(from: string, href: string): string | null {
  const parts = href.startsWith("/") ? [] : from.split("/").slice(0, -1);
  for (const seg of href.split("/")) {
    if (seg === "" || seg === ".") continue;
    if (seg === "..") {
      if (parts.length === 0) return null;
      parts.pop();
    } else parts.push(seg);
  }
  return parts.join("/");
}

/**
 * Where a link in the document at `from` should go on the site. A link to another document the site shows goes to
 * its page; any other repository path goes to the file on GitHub; external links and in-page anchors are kept.
 * `route` turns a site route ("/install/") into an href that honours the base path.
 */
export function rewriteHref(from: string, href: string, route: (r: string) => string): string {
  if (href.startsWith("#") || /^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(href) || href.startsWith("//")) return href;
  const [rawPath = "", frag] = href.split("#", 2);
  const target = rawPath === "" ? from : joinRepoPath(from, decodeURIComponent(rawPath));
  const hash = frag ? `#${frag}` : "";
  if (target === null) return `${REPO_URL}${hash}`;
  const page = routeOfRepoPath(target);
  if (page) return route(page) + hash;
  const kind = /\.[^/]+$/.test(target) ? "blob" : "tree";
  return `${REPO_URL}/${kind}/main/${target.split("/").map(encodeURIComponent).join("/")}${hash}`;
}

export type Rendered = { title: string; html: string };

/** Render one repository document. Raw HTML in the Markdown is shown as text, never passed through. */
export function render(path: string, route: (r: string) => string, body?: string): Rendered {
  let text = body ?? source(path);
  // The skill's YAML front matter is metadata for agents, not page text.
  text = text.replace(/^---\n[\s\S]*?\n---\n/, "");
  const seen = new Map<string, number>();
  let title = "";
  const md = new Marked({
    gfm: true,
    walkTokens(token) {
      if (token.type === "link") (token as Tokens.Link).href = rewriteHref(path, (token as Tokens.Link).href, route);
    },
    renderer: {
      html(token) {
        return escapeHtml(token.text);
      },
      heading(token) {
        const inner = this.parser.parseInline(token.tokens);
        const base = slug(token.text);
        const n = seen.get(base) ?? 0;
        seen.set(base, n + 1);
        const id = n === 0 ? base : `${base}-${n}`;
        if (token.depth === 1 && !title) title = token.text.replace(/[`*_]/g, "");
        return `<h${token.depth} id="${escapeHtml(id)}">${inner}</h${token.depth}>\n`;
      },
      code(token) {
        // tabindex: a block that scrolls sideways must be reachable from the keyboard (axe scrollable-region-focusable).
        const lang = token.lang ? ` class="language-${escapeHtml(token.lang.split(/\s/)[0] ?? "")}"` : "";
        return `<pre tabindex="0"><code${lang}>${escapeHtml(token.text)}</code></pre>\n`;
      },
      table(token) {
        // A wide table scrolls inside its own region instead of widening the page (phones, long paths).
        const head = token.header.map((c) => `<th>${this.parser.parseInline(c.tokens)}</th>`).join("");
        const body = token.rows
          .map((row) => `<tr>${row.map((c) => `<td>${this.parser.parseInline(c.tokens)}</td>`).join("")}</tr>`)
          .join("\n");
        return `<div class="table-scroll" tabindex="0"><table><thead><tr>${head}</tr></thead><tbody>${body}</tbody></table></div>\n`;
      },
    },
  });
  const html = md.parse(text, { async: false });
  return { title: title || path, html };
}

/** One item of a readiness list: `- **Title** — 40%. Text` in the Markdown. */
export type ReadinessItem = { title: string; percent: number; html: string };
export type ReadinessPart = { kind: "html"; html: string } | { kind: "items"; items: ReadinessItem[] };

// The title in bold, a spaced dash, a whole percentage from 0 to 100 and a full stop, then the text. Every locale writes
// the figure in ASCII digits so that one pattern reads them all.
const READINESS_ITEM = /^- \*\*([^*]+)\*\* — (\d{1,3})%\. (.+)$/;

/**
 * A path-to-production page: its Markdown, with each run of readiness items taken out as data (so the page can give
 * each one a slider) and everything else rendered as usual. A malformed figure fails the build.
 */
export function readiness(path: string, route: (r: string) => string): { title: string; parts: ReadinessPart[] } {
  const lines = source(path).split("\n");
  const parts: ReadinessPart[] = [];
  let text: string[] = [];
  let title = "";
  const flush = () => {
    if (text.join("").trim() === "") return;
    const r = render(path, route, text.join("\n"));
    if (!title && r.title !== path) title = r.title;
    parts.push({ kind: "html", html: r.html });
    text = [];
  };
  for (const line of lines) {
    const m = READINESS_ITEM.exec(line);
    if (!m) {
      if (/^- \*\*[^*]+\*\* — /.test(line)) throw new Error(`${path}: a readiness item needs " — NN%. ": ${line}`);
      text.push(line);
      continue;
    }
    const [, itemTitle = "", figure = "", body = ""] = m;
    const percent = Number(figure);
    if (percent > 100) throw new Error(`${path}: ${itemTitle} is over 100%`);
    flush();
    const html = render(path, route, body).html.trim().replace(/^<p>([\s\S]*)<\/p>$/, "$1");
    const last = parts.at(-1);
    const item = { title: itemTitle.trim(), percent, html };
    if (last?.kind === "items") last.items.push(item);
    else parts.push({ kind: "items", items: [item] });
  }
  flush();
  if (!parts.some((p) => p.kind === "items")) throw new Error(`${path}: no readiness items`);
  return { title: title || path, parts };
}

/** A brief's title (its H1) and summary (the italic line under it), for the list of briefs. */
export function briefSummary(path: string): { title: string; summary: string } {
  const text = source(path);
  const title = /^# (.+)$/m.exec(text)?.[1]?.trim() ?? path;
  const summary = /^\*([^*\n].*?)\*\s*$/m.exec(text)?.[1]?.trim() ?? "";
  return { title, summary };
}

export type Home = { headline: string; subheadline: string; intro: string; html: string; path: string };

/** A locale's home page: its heading, the italic line and the first paragraph make the hero, and the rest is the page. */
export function home(locale: string, route: (r: string) => string): Home {
  const path = `locales/${locale}/index.md`;
  const m = /^# (.+)\n\n\*(.+)\*\n\n(.+)\n\n([\s\S]*)$/.exec(source(path));
  if (!m) throw new Error(`${path} must open with a heading, an italic line and a paragraph`);
  const [, headline = "", subheadline = "", intro = "", rest = ""] = m;
  return { headline, subheadline, intro, html: render(path, route, rest).html, path };
}

const ENTITIES: Record<string, string> = { amp: "&", lt: "<", gt: ">", quot: '"' };

/** The words of rendered HTML, for the search index: this is shown as text, never as HTML. */
export function plainText(html: string): string {
  return html
    .replace(/<[^>]+>/g, " ")
    .replace(/&#(\d+);/g, (_, n: string) => String.fromCodePoint(Number(n)))
    .replace(/&(amp|lt|gt|quot);/g, (_, n: string) => ENTITIES[n] ?? "")
    .replace(/\s+/g, " ")
    .trim();
}

/** Every page made from a document: the documents and the role briefs (not the lists, which only link to the briefs). */
export function searchIndex(): SearchDoc[] {
  const documents = DOC_PAGES.map((d) => d.path);
  const briefs = Object.keys(FILES).filter((p) => p.startsWith("locales/") && !p.endsWith("/README.md"));
  return [...documents, ...briefs].flatMap((path) => {
    const route = routeOfRepoPath(path);
    if (!route) return [];
    const { title, html } = render(path, (r) => r);
    const locale = /^locales\/([^/]+)\//.exec(path)?.[1];
    return [{ route, title, lang: locale ? langTag(locale) : "en", text: plainText(html) }];
  });
}

/** Every page of the site, for the sitemaps. */
export function allRoutes(): string[] {
  const docs = DOC_PAGES.map((d) => `/${d.slug}/`);
  const repo = Object.keys(FILES)
    .map((p) => routeOfRepoPath(p))
    .filter((r): r is string => r !== null);
  const lists = BRIEF_LOCALES.map((l) => `/${l}/for/`);
  return [...new Set(["/", "/compare/", "/search/", ...docs, ...repo, ...lists])].sort();
}

function escapeXml(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

/** A sitemap of these routes. A slug may be in Welsh or Chinese, so each address is percent-encoded. */
export function sitemap(routes: string[]): Response {
  const urls = routes.map((r) => `  <url><loc>${escapeXml(SITE_URL + encodeURI(r))}</loc></url>`).join("\n");
  const xml = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;
  return new Response(xml, { headers: { "content-type": "application/xml" } });
}
