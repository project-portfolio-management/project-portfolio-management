import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

// CQ-21: a table's row is cast to the page's own type in one place, `rowOf<T>()` (src/lib/row-of.ts),
// not by a double cast at each use. The only other double cast is a regex match in pack-view.ts.
const ALLOWED = ["src/lib/row-of.ts", "src/lib/pack-view.ts"];

describe("no double casts outside the named exceptions", () => {
  const walk = (dir: string, out: string[] = []): string[] => {
    for (const f of readdirSync(dir)) {
      const p = join(dir, f);
      if (statSync(p).isDirectory()) walk(p, out);
      else if (/\.(svelte|ts)$/.test(p)) out.push(p);
    }
    return out;
  };
  it("finds `as unknown as` only where it is allowed", () => {
    const found = walk(join(process.cwd(), "src"))
      .map((p) => p.replace(process.cwd() + "/", ""))
      .filter((p) => readFileSync(p, "utf8").includes("as unknown as"))
      .filter((p) => !ALLOWED.includes(p));
    expect(found).toEqual([]);
  });
});
