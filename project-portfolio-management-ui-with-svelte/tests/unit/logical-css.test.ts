// WW-1.15: layout CSS uses logical properties, so the page mirrors under `dir="rtl"` (Arabic). A physical
// left/right in a stylesheet would stay on the same side in both directions. Positions computed for a chart's
// geometry (`style:left="{pct}%"` on a timeline bar) are not stylesheet rules and are not scanned: a timeline that
// runs left to right in every locale is a stated limit (UI spec, "Right-to-left").
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

const PHYSICAL: [string, RegExp][] = [
  [
    "a physical margin, padding or border side",
    /\b(margin|padding|border)-(left|right)\b/,
  ],
  ["text-align left or right", /\btext-align:\s*(left|right)\b/],
  ["float left or right", /\bfloat:\s*(left|right)\b/],
  ["clear left or right", /\bclear:\s*(left|right)\b/],
];

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) out.push(...walk(path));
    else if (/\.(svelte|css)$/.test(path)) out.push(path);
  }
  return out;
}

/** The stylesheet text of a file: a `.css` file whole, a component's `<style>` blocks. */
function styles(path: string, text: string): string {
  if (path.endsWith(".css")) return text;
  return [...text.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/g)]
    .map((m) => m[1] ?? "")
    .join("\n");
}

/** Physical-direction rules in a stylesheet, as "rule: line". */
export function physical(css: string): string[] {
  const found: string[] = [];
  for (const raw of css.split("\n")) {
    const line = raw.trim();
    if (line.startsWith("/*") || line.startsWith("*")) continue;
    for (const [name, re] of PHYSICAL)
      if (re.test(line)) found.push(`${name}: ${line}`);
  }
  return found;
}

describe("stylesheets use logical properties (WW-1.15)", () => {
  it("finds a physical property when it is there (the scan works)", () => {
    expect(physical("  margin-left: 1rem;")).toHaveLength(1);
    expect(physical("th { text-align: left; }")).toHaveLength(1);
    expect(physical("border-right-color: red;")).toHaveLength(1);
    expect(
      physical("margin-inline-start: 1rem; text-align: start;"),
    ).toHaveLength(0);
  });

  it("no stylesheet in src/ uses one", () => {
    const root = join(__dirname, "../../src");
    const found: string[] = [];
    for (const file of walk(root)) {
      const css = styles(file, readFileSync(file, "utf8"));
      for (const hit of physical(css))
        found.push(`${relative(root, file)}: ${hit}`);
    }
    expect(found).toEqual([]);
  });
});
