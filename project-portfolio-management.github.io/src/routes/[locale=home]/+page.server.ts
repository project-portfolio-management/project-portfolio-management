import type { EntryGenerator, PageServerLoad } from "./$types";
import { home } from "#lib/server/content.js";
import { href } from "#lib/href.js";
import { HOME_LOCALES, type BriefLocale } from "#lib/site.js";

export const entries: EntryGenerator = () => HOME_LOCALES.map((locale) => ({ locale }));

export const load: PageServerLoad = ({ params }) => {
  const locale = params.locale as BriefLocale;
  const content = home(locale, href);
  return { title: content.headline, home: content, locale };
};
