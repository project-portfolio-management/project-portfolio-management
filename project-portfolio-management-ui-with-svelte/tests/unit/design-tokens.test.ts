// RE-11: a colour comes from a design token, not a literal. In `src/`, outside `src/app.css` (where the tokens are
// defined), a `#hex`, `rgb()`/`rgba()` or `hsl()`/`hsla()` colour is refused, except as the fallback of a
// `var(--token, …)`. The files below held literals when the rule was added (2026-11-14), mostly the SVG charts; each
// may keep at most its count, and a lower count must be written here, so the list only shrinks. A new file with a
// literal fails.
//
// Contrast is not checked here: the tokens resolve through the active theme with `color-mix`, and axe's colour
// contrast rule measures the rendered pairs on every route (tests/e2e/a11y.spec.ts).

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

/** Literals per file when the rule was added; only ever lowered. */
const LEGACY: Record<string, number> = {
  "src/lib/components/CalendarMonth.svelte": 2,
  "src/lib/components/CapacityMeter.svelte": 6,
  "src/lib/components/CpmGantt.svelte": 7,
  "src/lib/components/CpmTable.svelte": 2,
  "src/lib/components/CumulativeFlow.svelte": 27,
  "src/lib/components/DataTable.svelte": 4,
  "src/lib/components/EvBars.svelte": 4,
  "src/lib/components/FeverChart.svelte": 9,
  "src/lib/components/GanttView.svelte": 1,
  "src/lib/components/NetworkDiagram.svelte": 3,
  "src/lib/components/RaciMatrix.svelte": 2,
  "src/lib/components/SCurve.svelte": 3,
  "src/routes/+layout.svelte": 4,
  "src/routes/balance/+page.svelte": 8,
  "src/routes/capacity-heatmap/+page.svelte": 4,
  "src/routes/capacity/+page.svelte": 2,
  "src/routes/dashboard/+page.svelte": 3,
  "src/routes/escalations/+page.svelte": 4,
  "src/routes/executive/+page.svelte": 4,
  "src/routes/lifecycle/+page.svelte": 1,
  "src/routes/pipeline/+page.svelte": 13,
  "src/routes/plans/[pid]/critical-chain/+page.svelte": 7,
  "src/routes/plans/[pid]/cutover/+page.svelte": 1,
  "src/routes/plans/[pid]/escalations/+page.svelte": 4,
  "src/routes/plans/[pid]/flow/+page.svelte": 2,
  "src/routes/plans/[pid]/governance/+page.svelte": 4,
  "src/routes/plans/[pid]/schedule/+page.svelte": 3,
  "src/routes/plans/[pid]/tolerances/+page.svelte": 5,
  "src/routes/prioritization/+page.svelte": 3,
  "src/routes/reviews/+page.svelte": 1,
  "src/routes/risk/+page.svelte": 2,
  "src/routes/scenarios/+page.svelte": 4,
  "src/routes/tolerances/+page.svelte": 5,
};

const FALLBACK =
  /var\(\s*--[\w-]+\s*,\s*(#[0-9a-fA-F]{3,8}|rgba?\([^)]*\)|hsla?\([^)]*\))\s*\)/g;
const LITERAL = /#[0-9a-fA-F]{3,8}\b|\brgba?\(|\bhsla?\(/g;

/** The colour literals in a source text, not counting `var()` fallbacks. */
export function literals(text: string): number {
  return (text.replace(FALLBACK, "var(--x)").match(LITERAL) ?? []).length;
}

function walk(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (/\.(svelte|ts|css)$/.test(name)) out.push(p);
  }
  return out;
}

describe("colours come from design tokens (RE-11)", () => {
  it("counts a literal but not a var() fallback", () => {
    expect(literals("color: #fff;")).toBe(1);
    expect(literals("fill: rgb(0 0 0); stroke: hsla(0, 0%, 0%, 1)")).toBe(2);
    expect(literals("color: var(--mxi-color-fg, #111827);")).toBe(0);
    expect(literals("color: var(--mxi-color-fg);")).toBe(0);
    expect(literals('href="#top"')).toBe(0);
  });

  it("no file has more literals than it had, and a new file has none", () => {
    const problems: string[] = [];
    for (const file of walk("src")) {
      if (file === join("src", "app.css")) continue;
      const n = literals(readFileSync(file, "utf8"));
      const allowed = LEGACY[file] ?? 0;
      if (n > allowed)
        problems.push(
          `${file}: ${n} colour literals, at most ${allowed}; use a --mxi-color-* token`,
        );
      if (n < allowed)
        problems.push(
          `${file}: ${n} colour literals, fewer than ${allowed}: lower its count in LEGACY`,
        );
    }
    for (const file of Object.keys(LEGACY))
      if (!walk("src").includes(file))
        problems.push(`${file} is gone: remove it from LEGACY`);
    expect(problems).toEqual([]);
  });
});
