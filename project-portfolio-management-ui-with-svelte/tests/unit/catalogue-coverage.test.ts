// HD-13: every page renders fully in Welsh. Two guards:
//  1. the Welsh page catalogue is complete, differs from the English except where a string is
//     genuinely the same in both (the allow-list below), and keeps every {placeholder};
//  2. no page or component template carries visible English outside the catalogue: any text node
//     or accessible-name attribute with words in it must come from `t()`, or be on the
//     allow-list of brand names and code.

import { describe, expect, it } from "vitest";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { parse } from "svelte/compiler";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import { PAGES_CY } from "../../src/lib/i18n-pages.cy";

/** English strings that are correct, unchanged, in Welsh (names, units, codes, symbols). */
const SAME_IN_WELSH = new Set([
  "CSV", "GBP", "PERT", "Gantt", "RACI — PPM", "PROJ-2026", "uuid", "capex", "opex",
  "person:…", "person:<uuid>", "worker:<uuid>", "CPI {value}", "Delta (b − a)", "Asana (allforio CSV)",
  // Same word in Welsh, or a name/abbreviation:
  "Excel", "TSV", "Agenda", "Cost", "cost", "CPI", "Person", "Band", "Cap", "label", "pts", "PPM", "ROI", "SPI", "SPI {ratioOrReason} ·",
  "· ROI {toFixed}%", "Cost — {title}: {costSkippedText}", "band", "person", "{word}: SPI {ratioText}, {gap}.",
  " · {points}pt", "{headcount} person",
  "Main X", "Gantt — Main X", "Plans — Main X", "Main X · Plans", "{name} — Main X", "{t} — Main X",
  "{v} — Main X", "Edit case — Main X", "New case — Main X", "Tour · Main X Plans",
]);
const SAME_PREFIX = /^(?:[A-Za-z ]+ — PPM|\{\w+\} — PPM|ROI|SPI|Main X)/;

/** Visible text in a template that is deliberately not translated (brand, code, symbols). */
const TEMPLATE_ALLOW = new Set([
  "PPM", "Kanban", "OpenDyslexic", "Jira", "Asana", "Microsoft Project", "Keycloak", "PERT", "RACI",
  "KPI", "OKR", "ROI", "FTE", "CSV", "GBP", "P50", "P80", "P90", "EV", "PV", "AC", "CPI", "SPI",
]);

const PH = /\{(\w+)\}/g;
const placeholders = (s: string): string[] => [...s.matchAll(PH)].map((m) => m[1]!).sort();

function walk(dir: string, out: string[] = []): string[] {
  for (const f of readdirSync(dir)) {
    const p = join(dir, f);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (p.endsWith(".svelte")) out.push(p);
  }
  return out;
}

/** The `.ts` source files under `dir`, the catalogues themselves left out. */
function walkTs(dir: string, out: string[] = []): string[] {
  for (const f of readdirSync(dir)) {
    const p = join(dir, f);
    if (statSync(p).isDirectory()) walkTs(p, out);
    else if (p.endsWith(".ts") && !/i18n/.test(f)) out.push(p);
  }
  return out;
}

describe("the Welsh page catalogue", () => {
  const keys = Object.keys(PAGES_EN) as (keyof typeof PAGES_EN)[];

  it("has every key, and no others", () => {
    expect(Object.keys(PAGES_CY).sort()).toEqual([...keys].sort());
  });

  it("keeps the placeholders of the English text", () => {
    const bad = keys.filter((k) => placeholders(PAGES_EN[k]).join() !== placeholders(PAGES_CY[k]).join());
    expect(bad).toEqual([]);
  });

  it("is translated: Welsh differs from English unless the string is on the allow-list", () => {
    const same = keys.filter(
      (k) =>
        PAGES_CY[k] === PAGES_EN[k] &&
        /[A-Za-z]{3,}/.test(PAGES_EN[k]) &&
        !SAME_IN_WELSH.has(PAGES_EN[k]) &&
        !SAME_PREFIX.test(PAGES_EN[k]),
    );
    expect(same).toEqual([]);
  });
});

describe("no visible English outside the catalogue", () => {
  const found: string[] = [];
  const word = /[A-Za-z]{3,}/;
  const visit = (n: any, file: string, inCode: boolean): void => {
    if (!n || typeof n !== "object") return;
    const code = inCode || (n.type === "RegularElement" && ["code", "kbd", "pre", "script", "style", "title"].includes(n.name));
    if (n.type === "Text" && !code) {
      const s = String(n.data).trim();
      if (word.test(s) && !TEMPLATE_ALLOW.has(s) && !/^[\p{P}\p{S}\s\d]*$/u.test(s)) found.push(`${file}: ${JSON.stringify(s.slice(0, 60))}`);
    }
    for (const a of n.attributes ?? []) {
      if (a.type !== "Attribute" || !/^(aria-label|title|placeholder|alt)$/.test(a.name) || !Array.isArray(a.value)) continue;
      for (const p of a.value) {
        if (p.type === "Text" && word.test(p.data) && !TEMPLATE_ALLOW.has(p.data.trim())) {
          found.push(`${file}: ${a.name}=${JSON.stringify(String(p.data).slice(0, 60))}`);
        }
      }
    }
    for (const k of Object.keys(n)) {
      if (k === "parent" || k === "attributes") continue;
      const v = n[k];
      if (Array.isArray(v)) v.forEach((c) => visit(c, file, code));
      else if (v && typeof v === "object" && v.type) visit(v, file, code);
    }
  };
  for (const file of walk(join(process.cwd(), "src"))) {
    const ast = parse(readFileSync(file, "utf8"), { modern: true });
    visit(ast.fragment, file.replace(process.cwd() + "/", ""), false);
  }
  it("finds no literal words in templates", () => {
    expect(found).toEqual([]);
  });
});

// CQ-23: a browser dialog's text is not a template node, so the walk above cannot see it. Its first
// argument must come from the catalogue (`t(...)`), never a literal.
describe("no literal English in browser dialogs", () => {
  it("passes every confirm, alert and prompt a catalogue string", () => {
    const literal = /\b(?:window\.)?(confirm|alert|prompt)\(\s*[`"']/;
    const found = walk(join(process.cwd(), "src"))
      .concat(walkTs(join(process.cwd(), "src")))
      .flatMap((file) =>
        readFileSync(file, "utf8")
          .split("\n")
          .map((line, i) => [line, i] as const)
          .filter(([line]) => literal.test(line))
          .map(([line, i]) => `${file.replace(process.cwd() + "/", "")}:${i + 1}: ${line.trim().slice(0, 80)}`),
      );
    expect(found).toEqual([]);
  });
});
