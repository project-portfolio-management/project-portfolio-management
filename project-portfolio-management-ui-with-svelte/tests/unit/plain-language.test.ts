// LT-12: plain-language labels, as a rule the sources are held to.
//
//  1. An acronym never stands alone in a heading, button, label, table header,
//     link or nav entry: it follows plain words ("Key measures (KPIs)") or is a
//     <Term> / <abbr> carrying its gloss.
//  2. Every <Term t="…"> has a gloss in the glossary.
//  3. The pages whose subject is jargon open with a plain sentence under the heading.
//
// This is a check on the English labels in the sources; it cannot judge whether a
// label is clear to a person, only that jargon is not left bare.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { ACRONYMS, GLOSSARY, glossFor } from "../../src/lib/glossary";

function walk(dir: string, match: (n: string) => boolean, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) walk(full, match, out);
    else if (match(name)) out.push(full);
  }
  return out;
}

const files = [
  ...walk("src/routes", (n) => n === "+page.svelte" || n === "+layout.svelte"),
  ...walk("src/lib/components", (n) => n.endsWith(".svelte")),
];

/** The visible label texts of a Svelte file, with glossed terms and parentheses removed. */
function labels(src: string): string[] {
  let s = src.replace(/<style[\s\S]*?<\/style>|<script[\s\S]*?<\/script>|<!--[\s\S]*?-->/g, "");
  s = s.replace(/<(abbr|Term)\b[^>]*?(\/>|>[\s\S]*?<\/\1>)/g, "");
  const out: string[] = [];
  for (const m of s.matchAll(/<(h1|h2|h3|h4|button|label|summary|caption|th|legend|a)\b[^>]*>([\s\S]*?)<\/\1>/g)) {
    const t = (m[2] ?? "").replace(/<[^>]+>|\{[^}]*\}/g, "").replace(/\s+/g, " ").trim();
    if (t) out.push(t);
  }
  for (const m of s.matchAll(/label: "([^"]+)"/g)) out.push(m[1] ?? "");
  return out.map((t) => t.replace(/\([^)]*\)/g, "").trim()).filter(Boolean);
}

const bare = (text: string, a: string) =>
  new RegExp(`(?<![\\w])${a.replace(/[()]/g, "\\$&")}(?![\\w])`).test(text);

describe("plain-language labels", () => {
  it("scans the pages", () => expect(files.length).toBeGreaterThan(60));

  for (const f of files) {
    it(`${f} leaves no acronym standing alone in a label`, () => {
      const found: string[] = [];
      for (const t of labels(readFileSync(f, "utf8")))
        for (const a of ACRONYMS) if (bare(t, a)) found.push(`${a} in “${t}”`);
      expect(found).toEqual([]);
    });
  }

  it("every <Term> used has a gloss", () => {
    const used = new Set<string>();
    for (const f of files)
      for (const m of readFileSync(f, "utf8").matchAll(/<Term\s+t="([^"]+)"/g)) used.add(m[1] ?? "");
    expect(used.size).toBeGreaterThan(3);
    for (const t of used) expect(glossFor(t), t).not.toBeNull();
  });

  it("each gloss is a sentence, not a restatement", () => {
    for (const [term, gloss] of Object.entries(GLOSSARY)) {
      expect(gloss.length, term).toBeGreaterThan(20);
      expect(gloss.endsWith("."), term).toBe(true);
    }
  });

  it("the scanner itself catches a bare acronym and lets the glossed forms through", () => {
    expect(labels("<h1>SPI</h1>").some((t) => bare(t, "SPI"))).toBe(true);
    expect(labels("<h1>Schedule index (SPI)</h1>").some((t) => bare(t, "SPI"))).toBe(false);
    expect(labels('<th><Term t="SPI" /></th>').some((t) => bare(t, "SPI"))).toBe(false);
    expect(labels('<th><abbr title="x">SPI</abbr></th>').some((t) => bare(t, "SPI"))).toBe(false);
    expect(labels('<button>Open KPIs</button>').some((t) => bare(t, "KPIs"))).toBe(true);
  });
});

describe("jargon pages open with a plain sentence", () => {
  const PAGES = [
    "plans/[pid]/earned-value", "plans/[pid]/critical-path", "plans/[pid]/schedule-risk",
    "plans/[pid]/cost-risk", "plans/[pid]/joint-risk", "plans/[pid]/leveling",
    "plans/[pid]/schedule-health", "plans/[pid]/raci", "plans/[pid]/flow", "plans/[pid]/work",
    "kpis", "okrs",
  ];
  for (const p of PAGES) {
    it(`${p} has a plain sentence under its heading`, () => {
      const src = readFileSync(`src/routes/${p}/+page.svelte`, "utf8");
      const m = src.match(/<\/h1>\s*<p class="lede">([^<]+)<\/p>/);
      expect(m, "a <p class=\"lede\"> right after the <h1>").not.toBeNull();
      expect((m?.[1] ?? "").length).toBeGreaterThan(30);
    });
  }
});
