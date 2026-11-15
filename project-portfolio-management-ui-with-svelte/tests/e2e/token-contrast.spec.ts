// RE-11, the contrast half: every text/background pair of the UI's own tokens (`src/app.css`, `--mxi-color-*`),
// in every theme the theme picker offers, against WCAG 2.2 AA (4.5:1 for text).
//
// The tokens resolve through the active Lily theme with `color-mix`, so only a browser knows the colours: each is
// painted to one canvas pixel and read back as sRGB, which settles `color-mix`, `oklch()` and fallbacks alike. axe
// (tests/e2e/a11y.spec.ts) measures the rendered pages, but only under the light and dark system preference; the
// other themes a person can pick were measured nowhere.
//
// Themes that fail a pair are listed in KNOWN with the pairs that fail, so the list can only shrink: a theme that
// passes a pair it is listed for fails the test until the entry is removed, and a new failure fails it too.
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

/**
 * Themes with pairs under 4.5:1 when this test was added (2026-11-15), each with those pairs, in PAIRS order. Only
 * ever shrinks. Nineteen of the 63; light and dark, the themes a visitor gets by default, pass every pair. The
 * contrast each measured is in tasks.md (RE-15).
 */
const KNOWN: Record<string, readonly string[]> = {
  abyss: ["--mxi-color-primary-fg on --mxi-color-primary"],
  aqua: ["--mxi-color-primary-fg on --mxi-color-primary"],
  black: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
  ],
  bumblebee: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-primary-fg on --mxi-color-primary",
  ],
  business: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
  ],
  cmyk: ["--mxi-color-link on --mxi-color-surface"],
  corporate: ["--mxi-color-primary-fg on --mxi-color-primary"],
  cupcake: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-primary-fg on --mxi-color-primary",
  ],
  emerald: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-primary-fg on --mxi-color-primary",
  ],
  garden: ["--mxi-color-primary-fg on --mxi-color-primary"],
  lemonade: [
    "--mxi-color-primary-fg on --mxi-color-primary",
    "--mxi-color-danger on --mxi-color-bg",
  ],
  nord: [
    "--mxi-color-primary-fg on --mxi-color-primary",
    "--mxi-color-success on --mxi-color-bg",
  ],
  pastel: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-primary-fg on --mxi-color-primary",
  ],
  retro: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-primary-fg on --mxi-color-primary",
    "--mxi-color-danger on --mxi-color-bg",
  ],
  silk: [
    "--mxi-color-danger on --mxi-color-bg",
    "--mxi-color-success on --mxi-color-bg",
  ],
  synthwave: ["--mxi-color-primary-fg on --mxi-color-primary"],
  valentine: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
    "--mxi-color-success on --mxi-color-bg",
  ],
  winter: [
    "--mxi-color-danger on --mxi-color-bg",
    "--mxi-color-success on --mxi-color-bg",
  ],
  wireframe: [
    "--mxi-color-link on --mxi-color-bg",
    "--mxi-color-link on --mxi-color-surface",
  ],
};

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
  for (const name of Object.keys(KNOWN)) expect(THEMES, name).toContain(name);
});

for (const theme of THEMES)
  test(`${theme}: every token text pair meets 4.5:1`, async ({ page }) => {
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
    const names = failing.map((f) => f.replace(/ [\d.]+:1$/, ""));
    expect(names, `${theme}: ${failing.join("; ")}`).toEqual([
      ...(KNOWN[theme] ?? []),
    ]);
  });
