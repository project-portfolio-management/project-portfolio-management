// LT-12: one primary action per screen — the static half. A page may declare at
// most ONE unconditional primary button; a page whose next step changes (preview,
// then commit) says so with `class:primary={…}`, and the browser tests check what
// is actually on screen. A button repeated in a list is never primary: ten
// primaries are no primary.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

function pages(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) pages(full, out);
    else if (name === "+page.svelte") out.push(full);
  }
  return out;
}

/** Literal primary buttons (a class attribute naming `primary`), not CSS rules. */
function literalPrimaries(src: string): number {
  const markup = src.replace(/<style[\s\S]*?<\/style>/g, "");
  return (markup.match(/class="[^"]*\bprimary\b[^"]*"/g) ?? []).length;
}

/** Whether a literal primary sits inside an `{#each}` block (so it repeats). */
function primaryInsideEach(src: string): boolean {
  const markup = src.replace(/<style[\s\S]*?<\/style>/g, "");
  let depth = 0;
  for (const m of markup.matchAll(
    /\{#each\b|\{\/each\}|class="[^"]*\bprimary\b[^"]*"/g,
  )) {
    if (m[0].startsWith("{#each")) depth++;
    else if (m[0] === "{/each}") depth = Math.max(0, depth - 1);
    else if (depth > 0) return true;
  }
  return false;
}

describe("one primary action per screen (static)", () => {
  const files = pages("src/routes");
  it("finds the pages", () => expect(files.length).toBeGreaterThan(50));

  for (const f of files) {
    it(`${f.slice("src/routes".length)} declares at most one primary button, and none in a list`, () => {
      const src = readFileSync(f, "utf8");
      expect(
        literalPrimaries(src),
        "unconditional primary buttons",
      ).toBeLessThanOrEqual(1);
      expect(
        primaryInsideEach(src),
        "a primary button repeated by {#each}",
      ).toBe(false);
    });
  }
});
