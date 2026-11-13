import type { EntryGenerator, PageServerLoad } from "./$types";
import { PATH_LOCALES, type PathLocale } from "#lib/site.js";
import { readiness } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const entries: EntryGenerator = () => PATH_LOCALES.map((locale) => ({ locale }));

export const load: PageServerLoad = ({ params }) => {
  const locale = params.locale as PathLocale;
  const path = `locales/${locale}/path-to-production/index.md`;
  return { ...readiness(path, href), path, locale, pathToProduction: true };
};
