// The rules of a product family's page (`/plans/[pid]/family`, RE-5, RE-7; NC-1 slice 12 part l), kept out of the
// component so they are unit-tested: a colour is `#rrggbb` (any case; stored lower case), its name in other languages
// follows the categories' rule, and a change sends only what changed. Removing the colour removes the translations too.

import { parseTranslations } from "#lib/roadmap-category-view.js";

/** How a colour reads under white and under black text (WCAG 2; AA is 4.5:1 for body text). */
export interface Contrast {
  with_white_text: number;
  with_black_text: number;
  white_text_meets_aa: boolean;
  black_text_meets_aa: boolean;
  better_text: "white" | "black";
}

/** `GET`/`PUT /api/plans/{pid}/roadmap-family`. */
export interface RoadmapFamilyView {
  plan_pid: string;
  name: string;
  colour: string | null;
  contrast: Contrast | null;
  translations: Record<string, string>;
  products: number;
}

/** The colour as stored, lower case, when `s` is `#rrggbb`; else `null`. */
export function colourOf(s: string): string | null {
  const t = s.trim();
  return /^#[0-9a-fA-F]{6}$/.test(t) ? t.toLowerCase() : null;
}

/**
 * What a change sends: the colour and, when they changed, the whole set of translations; every problem first as
 * catalogue keys; `null` when nothing changed. A family with no colour keeps no translations, so a blank colour with
 * translations is a problem; removing the colour is its own action.
 */
export function familyChange(
  colourText: string,
  translationsText: string,
  f: Pick<RoadmapFamilyView, "colour" | "translations">,
):
  | { body: { colour: string; translations?: Record<string, string> } }
  | { problems: string[] }
  | null {
  const problems: string[] = [];
  const colour = colourOf(colourText);
  if (!colour) problems.push("p.roadmap_family.colour_bad");
  const t = parseTranslations(translationsText);
  if ("problems" in t) problems.push(...t.problems);
  if (problems.length > 0 || !colour || "problems" in t) return { problems };
  const sameT =
    JSON.stringify(Object.entries(t.translations).sort()) ===
    JSON.stringify(Object.entries(f.translations ?? {}).sort());
  if (colour === f.colour && sameT) return null;
  return {
    body: sameT ? { colour } : { colour, translations: t.translations },
  };
}
