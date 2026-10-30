#!/usr/bin/env node
// Copy the repository documents this site shows into src/content/docs.json, so the site builds on its own once
// `git subtree` has exported this directory (the exported repository has no ../README.md to read).
//
//   node scripts/sync-content.mjs           rewrite src/content/docs.json from the monorepo
//   node scripts/sync-content.mjs --check   exit 1 if src/content/docs.json differs from the monorepo files
//
// Run from the monorepo only (it reads the parent directory). The list of documents is here and nowhere else:
// src/lib/site.ts reads the paths back out of the JSON.
import { readFileSync, writeFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const SITE = join(dirname(fileURLToPath(import.meta.url)), "..");
const ROOT = join(SITE, "..");
const OUT = join(SITE, "src", "content", "docs.json");

// The documents, by repository path. The role briefs are every locales/<locale>/for/<role>/index.md.
const DOCS = [
  "index.md",
  "README.md",
  "INSTALL.md",
  "operations/wpm-bridge.md",
  "spec/index.md",
  "project-portfolio-management-skill/SKILL.md",
];

function briefs() {
  const out = [];
  for (const locale of readdirSync(join(ROOT, "locales")).sort()) {
    const forDir = join(ROOT, "locales", locale, "for");
    if (!existsSync(forDir)) continue;
    for (const role of readdirSync(forDir).sort()) {
      const p = `locales/${locale}/for/${role}/index.md`;
      if (existsSync(join(ROOT, p))) out.push(p);
    }
  }
  return out;
}

function render() {
  /** @type {Record<string, string>} */
  const files = {};
  for (const p of [...DOCS, ...briefs()]) files[p] = readFileSync(join(ROOT, p), "utf8");
  return (
    JSON.stringify(
      { generated: "by scripts/sync-content.mjs from the monorepo; do not edit by hand", files },
      null,
      2,
    ) + "\n"
  );
}

if (!existsSync(join(ROOT, "locales")) || !existsSync(join(ROOT, "spec", "index.md"))) {
  console.error("sync-content: run this inside the monorepo (the parent directory has no locales/ or spec/)");
  process.exit(2);
}
const text = render();
if (process.argv.includes("--check")) {
  const current = existsSync(OUT) ? readFileSync(OUT, "utf8") : "";
  if (current !== text) {
    console.error("src/content/docs.json is stale: run `pnpm run sync` (node scripts/sync-content.mjs)");
    process.exit(1);
  }
  console.log("src/content/docs.json is current");
} else {
  writeFileSync(OUT, text);
  console.log(`wrote src/content/docs.json (${Object.keys(JSON.parse(text).files).length} documents)`);
}
