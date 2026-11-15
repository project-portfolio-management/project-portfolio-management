// RE-11, the contrast half: every text/background pair of the UI's own tokens (`src/app.css`, `--mxi-color-*`),
// in every theme the theme picker offers, against WCAG 2.2 AA (4.5:1 for text).
//
// The tokens resolve through the active Lily theme with `color-mix`, so only a browser knows the colours: each is
// painted to one canvas pixel and read back as sRGB, which settles `color-mix`, `oklch()` and fallbacks alike. axe
// (tests/e2e/a11y.spec.ts) measures the rendered pages, but only under the light and dark system preference; the
// other themes a person can pick were measured nowhere.
//
// The rule (UI spec, Theming; the owner's decision of 2026-11-15): `light` and `dark`, the themes a visitor gets by
// default, must pass every pair, and the test fails if either does not. Every other theme stays offered even where it
// does not pass, because a person who picks one has chosen it: its failing pairs are recorded as an annotation on
// its test (`contrast-below-aa`), shown in the report, and do not fail the run.
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "./test";
import { stubPlansApi } from "./fixtures";

/** The pairs the UI writes text in: [text token, background token]. */
const PAIRS: readonly [string, string][] = [
  ["--mxi-color-fg", "--mxi-color-bg"],
  ["--mxi-color-fg", "--mxi-color-surface"],
  ["--mxi-color-muted", "--mxi-color-bg"],
  ["--mxi-color-muted", "--mxi-color-surface"],
  ["--mxi-color-link", "--mxi-color-bg"],
  ["--mxi-color-link", "--mxi-color-surface"],
  ["--mxi-color-primary-fg", "--mxi-color-primary"],
  ["--mxi-color-danger", "--mxi-color-bg"],
  ["--mxi-color-success", "--mxi-color-bg"],
];

const AA = 4.5;

/** Every theme the picker offers: one stylesheet each in static/assets/themes. */
const THEMES = readdirSync(join(process.cwd(), "static/assets/themes"))
  .filter((f) => f.endsWith(".css"))
  .map((f) => f.replace(/\.css$/, ""))
  .sort();

/** The themes that must pass every pair: what a visitor gets with no choice of their own. */
const GATED: readonly string[] = ["light", "dark"];

function luminance([r, g, b]: number[]): number {
  const c = (v: number) => {
    const s = v / 255;
    return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * c(r) + 0.7152 * c(g) + 0.0722 * c(b);
}

function ratio(a: number[], b: number[]): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

test("the theme list is the picker's", () => {
  expect(THEMES.length).toBeGreaterThan(2);
  expect(THEMES).toContain("light");
  expect(THEMES).toContain("dark");
  for (const name of GATED) expect(THEMES, name).toContain(name);
});

for (const theme of THEMES)
  test(`${theme}: ${GATED.includes(theme) ? "every token text pair meets 4.5:1" : "token text pairs measured (not gated)"}`, async ({
    page,
  }) => {
    await stubPlansApi(page);
    await page.addInitScript(
      (t) => localStorage.setItem("lily-theme", t),
      theme,
    );
    await page.goto("/en-001");
    // The theme's stylesheet is loaded when its base colour is defined on the root.
    await expect
      .poll(() =>
        page.evaluate(() =>
          getComputedStyle(document.documentElement)
            .getPropertyValue("--color-base-100")
            .trim(),
        ),
      )
      .not.toBe("");
    const colours = await page.evaluate(
      (names) => {
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const ctx = canvas.getContext("2d", { willReadFrequently: true })!;
        const probe = document.createElement("div");
        document.body.append(probe);
        const out: Record<string, number[]> = {};
        for (const name of names) {
          probe.style.setProperty("color", `var(${name})`);
          const css = getComputedStyle(probe).color;
          ctx.clearRect(0, 0, 1, 1);
          ctx.fillStyle = "#000";
          ctx.fillStyle = css;
          ctx.fillRect(0, 0, 1, 1);
          out[name] = Array.from(ctx.getImageData(0, 0, 1, 1).data.slice(0, 3));
        }
        probe.remove();
        return out;
      },
      [...new Set(PAIRS.flat())],
    );
    const failing = PAIRS.filter(
      ([fg, bg]) => ratio(colours[fg], colours[bg]) < AA,
    ).map(
      ([fg, bg]) =>
        `${fg} on ${bg} ${ratio(colours[fg], colours[bg]).toFixed(2)}:1`,
    );
    if (GATED.includes(theme)) {
      expect(failing, `${theme} must meet 4.5:1 for every pair`).toEqual([]);
    } else if (failing.length > 0) {
      test.info().annotations.push({
        type: "contrast-below-aa",
        description: failing.join("; "),
      });
    }
  });
