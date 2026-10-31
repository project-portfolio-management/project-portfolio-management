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
import { PAGES_AR } from "../../src/lib/i18n-pages.ar";
import { PAGES_DE } from "../../src/lib/i18n-pages.de";
import { PAGES_ES } from "../../src/lib/i18n-pages.es";
import { PAGES_FR } from "../../src/lib/i18n-pages.fr";
import { PAGES_HI } from "../../src/lib/i18n-pages.hi";
import { PAGES_ZH } from "../../src/lib/i18n-pages.zh";

/** English strings that are correct, unchanged, in Welsh (names, units, codes, symbols). */
const SAME_IN_WELSH = new Set([
  "CSV",
  "GBP",
  "EUR",
  "PERT",
  "Gantt",
  "RACI — PPM",
  "PROJ-2026",
  "uuid",
  "capex",
  "opex",
  "person:…",
  "person:<uuid>",
  "worker:<uuid>",
  "CPI {value}",
  "Delta (b − a)",
  "Asana (allforio CSV)",
  // Same word in Welsh, or a name/abbreviation:
  "Excel",
  "TSV",
  "Agenda",
  "Cost",
  "cost",
  "CPI",
  "Person",
  "Band",
  "Cap",
  "label",
  "pts",
  "PPM",
  "ROI",
  "SPI",
  "SPI {ratioOrReason} ·",
  "· ROI {toFixed}%",
  "Cost — {title}: {costSkippedText}",
  "band",
  "person",
  "{word}: SPI {ratioText}, {gap}.",
  " · {points}pt",
  "{headcount} person",
  "Main X",
  "Gantt — Main X",
  "Plans — Main X",
  "Main X · Plans",
  "{name} — Main X",
  "{t} — Main X",
  "{v} — Main X",
  "Edit case — Main X",
  "New case — Main X",
  "Tour · Main X Plans",
]);
const SAME_PREFIX = /^(?:[A-Za-z ]+ — PPM|\{\w+\} — PPM|ROI|SPI|Main X)/;

/** Visible text in a template that is deliberately not translated (brand, code, symbols). */
const TEMPLATE_ALLOW = new Set([
  "PPM",
  "Kanban",
  "OpenDyslexic",
  "Jira",
  "Asana",
  "Microsoft Project",
  "Keycloak",
  "PERT",
  "RACI",
  "KPI",
  "OKR",
  "ROI",
  "FTE",
  "CSV",
  "GBP",
  "P50",
  "P80",
  "P90",
  "EV",
  "PV",
  "AC",
  "CPI",
  "SPI",
]);

const PH = /\{(\w+)\}/g;
const placeholders = (s: string): string[] =>
  [...s.matchAll(PH)].map((m) => m[1]!).sort();

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
    const bad = keys.filter(
      (k) =>
        placeholders(PAGES_EN[k]).join() !== placeholders(PAGES_CY[k]).join(),
    );
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

// WW-1.1: the same three guards for every other locale's page catalogue (machine drafts, unreviewed; see
// governance/translation-reviews/). A string left identical to the English must be a name or code (the Welsh
// allow-list and `CODE` below). Arabic, Hindi and Chinese may leave nothing else. German, Spanish and French share
// many words with English (Status, Budget, Plan, Description, Total); those are counted, and the count in
// `SAME_LIMIT` may only fall, so a new English string copied across unchanged fails.
const OTHERS = {
  ar: PAGES_AR,
  de: PAGES_DE,
  es: PAGES_ES,
  fr: PAGES_FR,
  hi: PAGES_HI,
  zh: PAGES_ZH,
} as const;
/** Code and identifiers shown as written in every language. */
const CODE = new Set(["organization:…", "HTTP {status}", "bearer sub"]);
/** Upper bound on strings identical to English and not on the name list, per language. It may only fall. */
const SAME_LIMIT: Record<keyof typeof OTHERS, number> = {
  ar: 0,
  de: 114,
  es: 52,
  fr: 137,
  hi: 0,
  zh: 0,
};

for (const [lang, pages] of Object.entries(OTHERS) as [
  keyof typeof OTHERS,
  Record<string, string>,
][]) {
  describe(`the ${lang} page catalogue`, () => {
    const keys = Object.keys(PAGES_EN) as (keyof typeof PAGES_EN)[];
    it("has every key, and no others", () => {
      expect(Object.keys(pages).sort()).toEqual([...keys].sort());
    });
    it("keeps the placeholders of the English text", () => {
      const bad = keys.filter(
        (k) =>
          placeholders(PAGES_EN[k]).join() !== placeholders(pages[k]!).join(),
      );
      expect(bad).toEqual([]);
    });
    it("is translated: few strings are left identical to the English", () => {
      const same = keys.filter(
        (k) =>
          pages[k] === PAGES_EN[k] &&
          /[A-Za-z]{3,}/.test(PAGES_EN[k]) &&
          !SAME_IN_WELSH.has(PAGES_EN[k]) &&
          !CODE.has(PAGES_EN[k]) &&
          !SAME_PREFIX.test(PAGES_EN[k]),
      );
      expect(same.length).toBeLessThanOrEqual(SAME_LIMIT[lang]);
    });
  });
}

describe("no visible English outside the catalogue", () => {
  const found: string[] = [];
  const word = /[A-Za-z]{3,}/;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- the Svelte AST is untyped here
  const visit = (n: any, file: string, inCode: boolean): void => {
    if (!n || typeof n !== "object") return;
    const code =
      inCode ||
      (n.type === "RegularElement" &&
        ["code", "kbd", "pre", "script", "style", "title"].includes(n.name));
    if (n.type === "Text" && !code) {
      const s = String(n.data).trim();
      if (
        word.test(s) &&
        !TEMPLATE_ALLOW.has(s) &&
        !/^[\p{P}\p{S}\s\d]*$/u.test(s)
      )
        found.push(`${file}: ${JSON.stringify(s.slice(0, 60))}`);
    }
    for (const a of n.attributes ?? []) {
      if (
        a.type !== "Attribute" ||
        !/^(aria-label|title|placeholder|alt)$/.test(a.name) ||
        !Array.isArray(a.value)
      )
        continue;
      for (const p of a.value) {
        if (
          p.type === "Text" &&
          word.test(p.data) &&
          !TEMPLATE_ALLOW.has(p.data.trim())
        ) {
          found.push(
            `${file}: ${a.name}=${JSON.stringify(String(p.data).slice(0, 60))}`,
          );
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
          .map(
            ([line, i]) =>
              `${file.replace(process.cwd() + "/", "")}:${i + 1}: ${line.trim().slice(0, 80)}`,
          ),
      );
    expect(found).toEqual([]);
  });
});

// CQ-23: a page key nothing refers to is dead text that translators still have to read.
describe("no unused page keys", () => {
  /** Groups whose keys are built or handed over as a set, so a search for the key cannot see them. */
  const SET_PREFIXES = [
    "p.director_pack.",
    "c.LilyGanttTable.",
    "c.LilyKanbanTable.",
    "c.CalendarMonth.",
  ];
  /** Unused when the census was added. It can only shrink: delete the key, or use it, and remove it here. */
  const KNOWN_UNUSED = new Set<string>([]);

  const sources = (dir: string, out: string[] = []): string[] => {
    for (const f of readdirSync(dir)) {
      const p = join(dir, f);
      if (statSync(p).isDirectory()) sources(p, out);
      else if (/\.(svelte|ts)$/.test(p) && !/i18n/.test(f))
        out.push(readFileSync(p, "utf8"));
    }
    return out;
  };
  const text = sources(join(process.cwd(), "src")).join("\n");
  // The text before a `${` in a quoted or template string: a key built at run time.
  const dynamic = [...text.matchAll(/[`"']([A-Za-z0-9_.]+)\$\{/g)]
    .map((m) => m[1]!)
    .filter((p) => p.length > 2);
  // A plural group's forms (`<base>_one`, `<base>_other`) are named by their base in a `tn("<base>", …)` call.
  const pluralBase = (k: string) =>
    /^(.+)_(?:zero|one|two|few|many|other)$/.exec(k)?.[1];
  const unused = Object.keys(PAGES_EN).filter(
    (k) =>
      !text.includes(k) &&
      !(pluralBase(k) && text.includes(`"${pluralBase(k)}"`)) &&
      !dynamic.some((p) => k.startsWith(p)) &&
      !SET_PREFIXES.some((p) => k.startsWith(p)),
  );

  it("finds no key nobody refers to, beyond the known list", () => {
    expect(unused.filter((k) => !KNOWN_UNUSED.has(k))).toEqual([]);
  });
  it("keeps the known list honest: every entry is still unused and still a key", () => {
    expect([...KNOWN_UNUSED].filter((k) => !unused.includes(k))).toEqual([]);
  });
});
