// NFR-4a: every locale is named `<language>-<region>` (lowercase, hyphenated); a bare two-letter
// code is not a locale at all: no URL prefix, no directory, no file.

import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { LOCALES, localeOfPath } from "../../src/lib/locales";

/** The bare language codes of the supported locales (`en`, `cy`, `de`, …). */
const BARE = [...new Set(LOCALES.map((l) => l.split("-")[0] ?? l))];

const ROOTS = ["src", "static", "tests", "docs", "locales"];
const SKIP = new Set(["node_modules", ".svelte-kit", "build", "dist"]);

function walk(dir: string, out: { path: string; dir: boolean }[] = []) {
  let names: string[];
  try {
    names = readdirSync(dir);
  } catch {
    return out;
  }
  for (const n of names) {
    if (SKIP.has(n)) continue;
    const p = join(dir, n);
    const isDir = statSync(p).isDirectory();
    out.push({ path: p, dir: isDir });
    if (isDir) walk(p, out);
  }
  return out;
}

describe("locale names (NFR-4a)", () => {
  it("every supported locale is <language>-<region>", () => {
    for (const l of LOCALES) expect(l, l).toMatch(/^[a-z]{2,3}-[a-z0-9]{2,3}$/);
  });

  it("a two-letter code is never a supported locale and never a URL prefix", () => {
    for (const bare of BARE) {
      expect((LOCALES as readonly string[]).includes(bare), bare).toBe(false);
      expect(localeOfPath(`/${bare}/plans`), bare).toBeNull();
    }
  });

  it("no directory or file is named for a bare two-letter locale", () => {
    const bare = new Set(BARE);
    const found = ROOTS.flatMap((r) => walk(r))
      .filter((e) => {
        const stem = (e.path.split("/").pop() ?? "").replace(/\.[^.]+$/, "");
        const underLocales = /(^|\/)(locales?|i18n|lang|translations?)\//.test(
          e.path,
        );
        return bare.has(stem) && (e.dir || underLocales);
      })
      .map((e) => e.path);
    expect(found).toEqual([]);
  });

  it("any locale directory that exists is named <language>-<region>", () => {
    const bad = ROOTS.flatMap((r) => walk(r))
      .filter(
        (e) =>
          e.dir && /(^|\/)(locales?|i18n|lang|translations?)$/.test(e.path),
      )
      .flatMap((e) => readdirSync(e.path).map((n) => join(e.path, n)))
      .filter((p) => !/\/[a-z]{2,3}-[a-z0-9]{2,3}(\.[a-z]+)?$/.test(p));
    expect(bad).toEqual([]);
  });
});
