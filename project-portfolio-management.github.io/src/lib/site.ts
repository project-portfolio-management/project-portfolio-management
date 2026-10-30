// The site's pages and how a repository path maps to one. Safe for the browser: the document text itself is read
// only on the server side, during prerendering (`$lib/server/content.ts`).

export const REPO_URL = "https://github.com/project-portfolio-management/project-portfolio-management";

/** A page made from one repository document. `slug` is the route under the site root ("" is the home page). */
export type DocPage = { slug: string; path: string; nav: string };

export const DOC_PAGES: readonly DocPage[] = [
  { slug: "", path: "index.md", nav: "Home" },
  { slug: "readme", path: "README.md", nav: "README" },
  { slug: "install", path: "INSTALL.md", nav: "Install" },
  { slug: "wpm-bridge", path: "operations/wpm-bridge.md", nav: "WPM bridge" },
  { slug: "spec", path: "spec/index.md", nav: "Specification" },
  { slug: "skill", path: "project-portfolio-management-skill/SKILL.md", nav: "Agent skill" },
];

/** The locales that have role briefs (`locales/<locale>/for/`), in the repository's spelling. */
export const BRIEF_LOCALES = [
  "cy-001",
  "cy-gb",
  "en-001",
  "en-gb",
  "en-gb-oxendict",
  "en-us",
  "hi-001",
  "hi-in",
  "zh-001",
  "zh-cn",
] as const;
export type BriefLocale = (typeof BRIEF_LOCALES)[number];

/** The locale the navigation's "Role briefs" link opens (the one the repository's index.md links to). */
export const DEFAULT_BRIEF_LOCALE: BriefLocale = "en-gb-oxendict";

/** The roles that have a brief in every locale. */
export const ROLES = [
  "chief-executive-officer",
  "chief-information-officer",
  "chief-legal-officer",
  "chief-operating-officer",
  "chief-people-officer",
  "chief-risk-officer",
  "chief-security-officer",
  "chief-strategy-officer",
  "chief-technology-officer",
  "governance-risk-compliance",
  "independent-security-tester",
] as const;
export type Role = (typeof ROLES)[number];

export function isBriefLocale(s: string): s is BriefLocale {
  return (BRIEF_LOCALES as readonly string[]).includes(s);
}
export function isRole(s: string): s is Role {
  return (ROLES as readonly string[]).includes(s);
}

/** BCP 47 form for `lang` ("en-gb-oxendict" → "en-GB-oxendict", "zh-001" → "zh-001"). */
export function langTag(locale: string): string {
  const [lang, region, ...rest] = locale.split("-");
  if (!region) return lang ?? locale;
  const r = /^[a-z]{2}$/.test(region) ? region.toUpperCase() : region;
  return [lang, r, ...rest].join("-");
}

// Right-to-left base languages (the same list Lily's LocalePicker uses). No brief locale is right-to-left today; the
// layout follows `dir` all the same, so a brief in Arabic or Hebrew needs no layout change.
const RTL = new Set(["ar", "arc", "ckb", "dv", "fa", "he", "iw", "ji", "ks", "ku", "mzn", "ps", "sd", "ug", "ur", "yi"]);
export function isRtl(locale: string): boolean {
  return RTL.has(locale.split("-")[0] ?? "");
}

/** The locale named by a site pathname's first segment, if it is one. */
export function localeOfPath(pathname: string, base = ""): BriefLocale | null {
  const rest = pathname.startsWith(base) ? pathname.slice(base.length) : pathname;
  const first = rest.split("/").filter(Boolean)[0] ?? "";
  return isBriefLocale(first) ? first : null;
}

/** The site route ("/install/", "/cy-gb/for/chief-risk-officer/") of a repository path, or null if it has none. */
export function routeOfRepoPath(path: string): string | null {
  const clean = path.replace(/\/+$/, "");
  for (const d of DOC_PAGES) {
    const dir = d.path.replace(/\/?[^/]+$/, "");
    if (clean === d.path || (dir && clean === dir && /\/(index|README)\.md$/.test("/" + d.path)))
      return d.slug ? `/${d.slug}/` : "/";
  }
  const m = /^locales\/([^/]+)\/for(?:\/([^/]+))?(?:\/index\.md)?$/.exec(clean);
  if (m && m[1] && isBriefLocale(m[1])) {
    if (!m[2]) return `/${m[1]}/for/`;
    if (isRole(m[2])) return `/${m[1]}/for/${m[2]}/`;
  }
  return null;
}
