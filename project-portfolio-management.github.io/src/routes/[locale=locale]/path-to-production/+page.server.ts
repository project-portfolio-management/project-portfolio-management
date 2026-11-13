import type { EntryGenerator, PageServerLoad } from "./$types";
import { BRIEF_LOCALES, type BriefLocale } from "#lib/site.js";
import { readiness } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const entries: EntryGenerator = () => BRIEF_LOCALES.map((locale) => ({ locale }));

export const load: PageServerLoad = ({ params }) => {
  const locale = params.locale as BriefLocale;
  const path = `locales/${locale}/path-to-production/index.md`;
  return { ...readiness(path, href), path, locale, pathToProduction: true };
};
